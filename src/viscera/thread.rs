//! **A thread of slime between two points: whole, then thinning, then snapped.**
//!
//! A [`Thread`] has no body of its own. It is two endpoints the caller hands it every tick, a rest length, and an integer age: its shape is a function of
//! those and nothing else, so it cannot swing, wrap or catch on anything, and it costs one small tube a tick. That is what a thread of slime is in a game
//! (a rope drawn "between two points: it stretches, thins, and then breaks", Zarkonnen, 2026) and what keeps it outside every digest: nothing here is read
//! by the solver, and a [`Thread`]'s state is a function of the positions it was shown, in order.
//!
//! # The model, and what it is made of
//!
//! * **Whole, then thinning.** A liquid thread has no slack: it is a bridge, straight between its two ends, and the volume it holds sets its radius. An incompressible
//!   cylinder of radius `r0` and length `R` (its **rest length**) stretched to `L` has radius `r0 * sqrt(R / L)` (this crate's own volume argument, not a paper's). A
//!   thread no longer than `R` is `r0` across; past it the neck thins, down to [`NECK_MIN`] of `r0`, and the ends keep a bead ([`BEAD`]) where the liquid wets the surface.
//! * **Snapping.** The thread snaps when `L >= BREAK_STRETCH * R`, or once it has been stretched past `R` for [`BREAK_TICKS`] ticks **whatever the pull**: Weickgenannt,
//!   Roisman & Tropea, *Pinch-off of a stretching viscous filament and drop transport*, New J. Phys. 17 (2015) `083059`, `doi:10.1088/1367-2630/17/8/083059`:
//!   "at high acceleration rates even the breakup time does not depend on the acceleration, but is determined solely by the liquid viscosity". **A snap is never
//!   undone**, as a tear is not (`CLAUDE.md`).
//! * **Recoil.** For [`RECOIL_TICKS`] ticks after a snap each half shortens toward its own end, tapering to a point at the break; then the thread is
//!   [`gone`](Thread::gone).
//!
//! Every number is tuned by eye against `docs/research/slime-strings.md` and says so at its constant.
//!
//! **Cosmetic, and a tick count, not a clock**: nothing reads one, and a thread's age is the number of times it was ticked.

use core::f32::consts::{PI, TAU};

use bevy::asset::RenderAssetUsages;
use bevy::math::Vec3;
use bevy::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::prelude::Component;

use crate::viscera::frame::perpendicular_basis;
use crate::viscera::hash::{hash_pair, hash_unit};

/// **How much wider a thread is at either end than it is at its unstretched radius**: 1.8, the bead where the liquid wets the surface (Ref T3 of
/// `docs/research/slime-strings.md`: each clear thread ends in one). **Tuned by eye.**
pub const BEAD: f32 = 1.8;
/// **How much of the thread's length each bead takes**, as a fraction of it: 0.08. **Tuned by eye.**
pub const BEAD_REACH: f32 = 0.08;
/// **The thinnest a neck gets, as a fraction of the thread's radius**: 0.2, where it pinches off. **Tuned by eye.**
pub const NECK_MIN: f32 = 0.2;
/// **How far past its rest length a thread stretches before it snaps**, as a multiple of it: 2. Liao, Franses & Basaran, *Deformation and breakup of a
/// stretching liquid bridge covered with an insoluble surfactant monolayer*, Phys. Fluids 18 (2006) `022101`, `doi:10.1063/1.2166657`, measured bridges that are
/// "3.15 mm initially and about 4–7 mm at breakup", 1.3 to 2.2 times as long; a gel holds longer, so 2 is the shortest we take. **Tuned.**
pub const BREAK_STRETCH: f32 = 2.0;
/// **How many ticks a thread may be stretched before it snaps whatever the pull**: 40, two thirds of a second at 60 Hz. The same paper's bridges break "after stretching
/// for about 0.1–0.2 s"; a slime is a gel and breaks later. **Tuned by eye**, in ticks, as every game time in this crate is.
pub const BREAK_TICKS: u32 = 40;
/// **How many ticks the two halves of a snapped thread take to recoil**: 10. **Tuned by eye.**
pub const RECOIL_TICKS: u32 = 10;

/// How many segments a whole thread is drawn in, and each half of a snapped one.
const SEGMENTS: usize = 12;

/// **A viscous thread between two points.** See the module docs. Its endpoints are not stored: they are an argument to [`tick`](Self::tick) and
/// [`mesh`](Self::mesh), because they are somebody else's state (a wound, a strand's node).
#[derive(Component, Clone, Debug)]
pub struct Thread {
    /// The length at which the thread is at its full radius, metres.
    rest: f32,
    /// The radius of the thread at its rest length, metres.
    radius: f32,
    /// Ticks spent stretched past the rest length, whether or not in a row.
    stretched_ticks: u32,
    /// Ticks since the snap, or `None` while whole.
    since_snap: Option<u32>,
}

impl Thread {
    /// A thread whose rest length is `rest_m` and whose radius there is `radius_m`. A rest length below a millimetre is a millimetre; a negative or non-finite
    /// radius is zero (and draws nothing).
    pub fn new(rest_m: f32, radius_m: f32) -> Self {
        let rest = if rest_m.is_finite() { rest_m.max(1.0e-3) } else { 1.0e-3 };
        let radius = if radius_m.is_finite() { radius_m.max(0.0) } else { 0.0 };
        Self { rest, radius, stretched_ticks: 0, since_snap: None }
    }

    /// The length at which the thread is at its full radius, metres.
    #[inline]
    pub fn rest(&self) -> f32 {
        self.rest
    }

    /// The radius of the thread at its rest length, metres.
    #[inline]
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// Whether the thread has snapped. Never goes back.
    #[inline]
    pub fn snapped(&self) -> bool {
        self.since_snap.is_some()
    }

    /// Whether the thread has snapped and both halves have finished recoiling: [`mesh`](Self::mesh) draws nothing from now on.
    #[inline]
    pub fn gone(&self) -> bool {
        self.since_snap.is_some_and(|k| k >= RECOIL_TICKS)
    }

    /// **Advance one tick**, the thread's ends at `a` and `b`. A snapped thread only ages; a whole one counts a tick stretched if `|b - a|` is past its rest length, and
    /// snaps at [`BREAK_STRETCH`] times it or after [`BREAK_TICKS`] stretched ticks. Ends that are not finite change nothing: a thread is never snapped by a bad position.
    pub fn tick(&mut self, a: Vec3, b: Vec3) {
        if let Some(k) = self.since_snap {
            self.since_snap = Some(k.saturating_add(1));
            return;
        }
        let length = (b - a).length();
        if !length.is_finite() {
            return;
        }
        if length > self.rest {
            self.stretched_ticks = self.stretched_ticks.saturating_add(1);
        }
        if length >= BREAK_STRETCH * self.rest || self.stretched_ticks >= BREAK_TICKS {
            self.since_snap = Some(0);
        }
    }

    /// **The thread drawn between `a` and `b`**: a tube of `sides` sides (clamped to 3 to 16), thin at the neck and beaded at the ends, or, once snapped, the two halves
    /// recoiling. `None` when the thread is [`gone`](Self::gone), when its radius is zero, or when `a` and `b` are a millimetre or less apart or not finite: a thread that is
    /// not there is not drawn, and a degenerate tube is not built.
    ///
    /// Carries `ATTRIBUTE_POSITION`, `ATTRIBUTE_NORMAL` (unit) and `ATTRIBUTE_UV_0` (`u` round, `v` from `a` to `b`), `U32` indices, and **no end caps**: the ends sit inside
    /// the surfaces the thread is joined to.
    pub fn mesh(&self, a: Vec3, b: Vec3, sides: u32) -> Option<Mesh> {
        if self.gone() || self.radius <= 0.0 {
            return None;
        }
        let length = (b - a).length();
        if !length.is_finite() || length < 1.0e-3 {
            return None;
        }
        let sides = sides.clamp(3, 16) as usize;
        let points: Vec<Vec3> = (0..=SEGMENTS).map(|i| a.lerp(b, i as f32 / SEGMENTS as f32)).collect();
        // The neck thins as the thread stretches; once snapped it is as thin as it gets.
        let neck = if self.snapped() { NECK_MIN } else { (self.rest / length).sqrt().clamp(NECK_MIN, 1.0) };
        let radii: Vec<f32> = (0..=SEGMENTS).map(|i| self.radius * profile(i as f32 / SEGMENTS as f32, neck)).collect();

        let mut builder = Builder::default();
        match self.since_snap {
            None => builder.tube(&points, &radii, sides),
            Some(k) => {
                // Each half shortens toward its own end as it recoils, and tapers to a point at the break.
                let kept = 1.0 - k as f32 / RECOIL_TICKS as f32;
                let half = (kept * 0.5 * SEGMENTS as f32).floor() as usize;
                if half >= 1 {
                    builder.tube(points.get(..=half)?, &tapered(radii.get(..=half)?), sides);
                    let from = SEGMENTS - half;
                    let (mut p, mut r) = (points.get(from..)?.to_vec(), radii.get(from..)?.to_vec());
                    p.reverse();
                    r.reverse();
                    builder.tube(&p, &tapered(&r), sides);
                }
            }
        }
        builder.finish()
    }
}

/// The radii of a half, which run from its attached end to the break, with the last tapered to a tenth of itself.
fn tapered(radii: &[f32]) -> Vec<f32> {
    let last = radii.len().saturating_sub(1).max(1) as f32;
    radii.iter().enumerate().map(|(i, r)| r * (1.0 - 0.9 * (i as f32 / last).powi(3))).collect()
}

/// **The thread's radius at `t` along it, as a multiple of its radius at rest**: a neck of `neck` in the middle and a [`BEAD`] at either end.
fn profile(t: f32, neck: f32) -> f32 {
    let sin = (PI * t).sin();
    let waist = 1.0 - (1.0 - neck) * sin * sin;
    let end = t.min(1.0 - t) / BEAD_REACH;
    waist * (1.0 + (BEAD - 1.0) * (-end * end).exp())
}

/// Where a thread joins a strand to the wound it came from, and what it is made of.
#[derive(Clone, Debug)]
pub struct ThreadSite {
    /// The thread.
    pub thread: Thread,
    /// The node of the strand it is joined to.
    pub node: u32,
    /// Where it is joined to the wound, world space.
    pub anchor: Vec3,
}

/// **How many metres either side of the wound's centre a thread may be joined**: 15 mm. **Tuned by eye.**
pub const SITE_SPREAD_M: f32 = 0.015;
/// **The shortest and longest rest length a spilled thread is given**, metres: 0.08 and 0.22. Short, because the wound end is fixed in the world where the strand spilled and the body it was
/// on may fall away from it: a thread that snaps within a handful of ticks of the strand falling is gone before that shows. **Tuned by eye** (`docs/research/slime-strings.md`).
pub const REST_RANGE_M: (f32, f32) = (0.08, 0.22);

/// **`count` threads from the wound at `from` to the first nodes of a strand**, each a pure function of `seed` (the strand's own, [`crate::viscera::Strand::seed`])
/// and its index: the node is every second one from the wound end (0, 2, 4, ...), because those are the ones that are still near the wound when the strand is laid,
/// the anchor is a point within [`SITE_SPREAD_M`] of `from`, and the rest length is in [`REST_RANGE_M`]. `nodes` is the strand's node count, so a node past its end is
/// never named.
///
/// Seeds are places, never histories: nothing here carries a counter between calls.
pub fn spill_threads(from: Vec3, seed: u32, nodes: usize, count: u32, radius_m: f32) -> Vec<ThreadSite> {
    (0..count)
        .filter_map(|k| {
            let node = (2 * k) as usize;
            if node >= nodes {
                return None;
            }
            let h = |salt: u32| hash_unit(hash_pair(seed ^ 0x7EAD_5A17, k * 4 + salt));
            let azimuth = TAU * h(0);
            let reach = SITE_SPREAD_M * h(1).sqrt();
            let (sin, cos) = azimuth.sin_cos();
            let anchor = from + Vec3::new(cos * reach, (h(3) - 0.5) * SITE_SPREAD_M, sin * reach);
            let rest = REST_RANGE_M.0 + (REST_RANGE_M.1 - REST_RANGE_M.0) * h(2);
            Some(ThreadSite { thread: Thread::new(rest, radius_m), node: node as u32, anchor })
        })
        .collect()
}

/// Tubes accumulated into one mesh.
#[derive(Default)]
struct Builder {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

impl Builder {
    /// Sweep a circle of `sides` sides along `points`, its radius `radii[i]` at point `i`. Fewer than two points, or no radii, add nothing. The frame is
    /// parallel-transported, as `tube_mesh`'s is, so a thread never twists.
    fn tube(&mut self, points: &[Vec3], radii: &[f32], sides: usize) {
        let n = points.len().min(radii.len());
        if n < 2 {
            return;
        }
        let ring = sides + 1;
        let base = self.positions.len() as u32;
        let tangent_at = |i: usize| -> Vec3 {
            let (lo, hi) = (i.saturating_sub(1), (i + 1).min(n - 1));
            match (points.get(lo), points.get(hi)) {
                (Some(p), Some(q)) => (*q - *p).try_normalize().unwrap_or(Vec3::Y),
                _ => Vec3::Y,
            }
        };
        let mut tangent = tangent_at(0);
        let (mut normal, mut binormal) = perpendicular_basis(tangent);
        for i in 0..n {
            if i > 0 {
                let next = tangent_at(i);
                normal = (normal - next * normal.dot(next)).try_normalize().unwrap_or(normal);
                binormal = next.cross(normal);
                tangent = next;
            }
            let (lo, hi) = (i.saturating_sub(1), (i + 1).min(n - 1));
            let (centre, radius) = match (points.get(i), radii.get(i)) {
                (Some(c), Some(r)) => (*c, *r),
                _ => continue,
            };
            // The wall leans the way the radius changes along the thread.
            let slope = match (radii.get(lo), radii.get(hi), points.get(lo), points.get(hi)) {
                (Some(r0), Some(r1), Some(p0), Some(p1)) => {
                    let run = (*p1 - *p0).length();
                    if run > 1.0e-6 { (r1 - r0) / run } else { 0.0 }
                }
                _ => 0.0,
            };
            for j in 0..ring {
                let (sin, cos) = (TAU * j as f32 / sides as f32).sin_cos();
                let radial = normal * cos + binormal * sin;
                self.positions.push((centre + radial * radius).to_array());
                self.normals.push((radial - tangent * slope).try_normalize().unwrap_or(radial).to_array());
                self.uvs.push([j as f32 / sides as f32, i as f32 / (n - 1) as f32]);
            }
        }
        for seg in 0..n - 1 {
            let (lo, hi) = (base + (seg * ring) as u32, base + ((seg + 1) * ring) as u32);
            for j in 0..sides as u32 {
                let (p, q, r, s) = (lo + j, lo + j + 1, hi + j, hi + j + 1);
                // Counter-clockwise seen from outside, which is wgpu's front face.
                self.indices.extend_from_slice(&[p, q, r, q, s, r]);
            }
        }
    }

    fn finish(self) -> Option<Mesh> {
        if self.positions.is_empty() {
            return None;
        }
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
        mesh.insert_indices(Indices::U32(self.indices));
        Some(mesh)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The profile is the volume argument and the bead.** Its neck is `neck` at the middle and its ends are `BEAD` wide.
    #[test]
    fn the_profile_has_a_neck_and_a_bead() {
        assert!((profile(0.5, 0.5) - 0.5).abs() < 1.0e-3);
        assert!((profile(0.0, 0.5) - BEAD).abs() < 1.0e-3 && (profile(1.0, 0.5) - BEAD).abs() < 1.0e-3);
        assert!((profile(0.5, 1.0) - 1.0).abs() < 1.0e-3, "an unstretched thread is as wide in the middle as it is");
    }

    /// **A half tapers to a tenth of itself at the break, and the neck-to-bead ratio is as the volume argument says.**
    #[test]
    fn a_recoiling_half_tapers_to_a_point() {
        let r = tapered(&[1.0, 1.0, 1.0, 1.0, 1.0]);
        assert!((r[0] - 1.0).abs() < 1.0e-6, "the attached end keeps its radius");
        assert!((r[4] - 0.1).abs() < 1.0e-6, "the break is a tenth of the radius");
        assert!(r.windows(2).all(|w| w[1] <= w[0]), "a half never widens toward the break");
    }
}
