//! **What a spilled strand looks like: a texture for the tube [`crate::viscera::tube_mesh`] sweeps.**
//!
//! Bowel is not the colour of the blood it lies in. Its outer coat is a pale, salmon-pink serosa,
//! glossy and a little translucent, with the mesentery's yellow fat along the side the strand was
//! tethered by and the straight vessels (vasa recta) branching off that side and round the wall, a
//! few to every centimetre or two. A strand drawn the red of the blood pool reads as red tubing in it,
//! which is what every spill in this crate looked like until 0.5.41.
//!
//! The image is laid out for the tube's own `UV_0`: `u` round the wall, with the mesenteric side at
//! `u = 0`, and `v` along the strand from the wound (`v = 0`) to its free end, over a strand of
//! [`crate::viscera::SPILL_SEGMENTS`] × [`crate::viscera::SPILL_REST_LEN`]. It repeats round the wall
//! and is sampled that way. The end that left the wound is bloodied, and patches along the rest are
//! smeared.
//!
//! Cosmetic: a pure function of nothing, built once by the caller and handed to whatever material it
//! likes, and outside every digest. The crate still never spawns and never writes an asset arena.

use bevy::asset::RenderAssetUsages;
use bevy::image::{Image, ImageAddressMode, ImageSampler, ImageSamplerDescriptor};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use std::sync::LazyLock;

use crate::bloodstain::spectral::{self, Film};
use crate::viscera::hash::{hash_pair, hash_unit};
use crate::viscera::strand::Strand;
use crate::viscera::tube::contact;

/// Texels round the wall. A 20 mm tube is 126 mm round, so 2 mm a texel.
const AROUND: u32 = 64;
/// Texels along the strand. 840 mm over 384, so 2.2 mm a texel.
const ALONG: u32 = 384;

/// The serosa, sRGB: a salmon pink. This crate's own number, chosen by eye against the blood colour
/// the wetmap draws; a paler `0.84, 0.60, 0.56` read as pink candy in the pool.
const SEROSA: [f32; 3] = [0.76, 0.48, 0.45];
/// The mesentery's fat along the tethered side, sRGB.
const FAT: [f32; 3] = [0.84, 0.68, 0.52];
/// A vessel in the wall, sRGB: venous, dark and a little purple.
const VESSEL: [f32; 3] = [0.50, 0.13, 0.20];
/// Blood smeared on the wall, sRGB, as the wetmap draws a thick venous film.
const BLOOD: [f32; 3] = [0.40, 0.05, 0.07];

/// Straight vessels along the strand, each leaving the mesenteric side for both sides of the wall.
const VESSELS: u32 = 56;

/// **How rough a spilled strand's wall is**: `0.15`, wet and bloodied bowel. Until 0.8.6 it was `0.3`, a soft streak on a clean pink wall; the
/// *Thing* stills (`docs/research/guts.md`, T1, T2) have a dark body under a hard, small glint, and a lumpy wall ([`crate::viscera::tube_mesh`])
/// breaks a lower roughness's highlight into several. The flesh shader draws a clear coat only from a wet canvas, which a strand has none of, so
/// this is the whole of its gloss. **Tuned by eye**, not measured.
pub const GUT_ROUGHNESS: f32 = 0.15;

/// **How soaked a node is at the least, on average**, `[0, 1]`: a spilled strand is slick with blood wherever it hangs, in patches ([`SOAK_PATCH`]).
/// **Tuned by eye** against `docs/research/guts.md` (Ref D1: even the far end of the pulled bowel is red), not measured.
pub const SOAK_BASE: f32 = 0.45;
/// **How much of [`SOAK_BASE`] swings along the strand**: the base is `SOAK_BASE * (1 + SOAK_PATCH * (2n - 1))`, `n` in `[0, 1]` a smooth noise of the
/// arc length on a wavelength of [`SOAK_PATCH_M`]. Blood on a wall is thick in one place and a smear in the next; a strand soaked to one tone along
/// its length read as red tubing. **Tuned**, `docs/research/guts.md` (Ref D1, T1).
pub const SOAK_PATCH: f32 = 0.5;
/// The wavelength of [`SOAK_PATCH`], metres of bowel: 12 cm, a hand's width.
pub const SOAK_PATCH_M: f32 = 0.12;
/// **How far along the bowel from the wound the blood that left it still coats the wall**, metres: the soak falls as `exp(-arc / reach)`. The
/// blood runs down a strand from where it came out; 0.30 m is a third of a spilled strand. **Tuned**, `docs/research/guts.md`.
pub const WOUND_REACH_M: f32 = 0.30;
/// **The thickness of the blood film a fully soaked node wears**, mm: 0.35, between a smear (0.05) and a drip (0.5), `spectral::Film`'s own
/// range. The film is `SOAK_FILM_MM * soak^3`: venous blood is opaque in green by about 0.04 mm, so on a linear scale nine tenths of the range would be the
/// same dark red, and a cube spends it where the colour changes. **Tuned** against Ref D1's dark pockets, `docs/research/guts.md`.
pub const SOAK_FILM_MM: f32 = 0.35;
/// How many entries the tint table holds from a bare node to a fully soaked one; between them it is interpolated.
pub const SOAK_STEPS: usize = 17;

/// A smooth noise in `[0, 1]` of `x` on a unit lattice, a pure function of `seed`.
fn patch(seed: u32, x: f32) -> f32 {
    let (cell, frac) = (x.floor(), x - x.floor());
    let corner = |k: f32| hash_unit(hash_pair(seed ^ 0x50AC, k as i64 as u32));
    let t = frac * frac * (3.0 - 2.0 * frac);
    corner(cell) + (corner(cell + 1.0) - corner(cell)) * t
}

/// **How soaked each node of `strand` is in blood**, `[0, 1]`, one per node, for a strand resting on a floor at `floor_y`.
///
/// `max` of three terms: the patchy base ([`SOAK_BASE`], [`SOAK_PATCH`]), the coat the wound left (`exp(-arc / WOUND_REACH_M)`, `arc` the length of bowel from
/// the wound end, which is node `0` **and the first node after any tear**: a torn end is itself a wound), and contact with the floor (the strand lies in the
/// pool the wound made; four fifths of a full soak at the least). Cosmetic and a pure function of the nodes and [`Strand::seed`]: nothing here reads a canvas,
/// a clock or an asset.
pub fn soak(strand: &Strand, floor_y: f32) -> Vec<f32> {
    let nodes = strand.nodes();
    let radius = strand.radius();
    let mut out = Vec::with_capacity(nodes.len());
    let mut arc = 0.0_f32;
    // How far along the bowel from node 0, tears and all: where the patches fall does not restart at a tear.
    let mut along = 0.0_f32;
    let mut before: Option<bevy::math::Vec3> = None;
    for (i, node) in nodes.iter().enumerate() {
        if let Some(prev) = before {
            let step = (*node - prev).length();
            along += step;
            // The wound end restarts the arc: the node after a tear is a new wound, and a strand that was never torn walks on.
            arc = if i > 0 && strand.segment_torn(i - 1) { 0.0 } else { arc + step };
        }
        before = Some(*node);
        let n = patch(strand.seed(), along / SOAK_PATCH_M);
        let base = SOAK_BASE * (1.0 + SOAK_PATCH * (2.0 * n - 1.0));
        let coat = (-arc / WOUND_REACH_M).exp();
        let lying = if radius > 0.0 { contact((node.y - floor_y) / radius) * (0.8 + 0.2 * n) } else { 0.0 };
        out.push(base.max(coat).max(lying).clamp(0.0, 1.0));
    }
    out
}

/// **What a node soaked `soak` multiplies the material's texture by**, linear RGBA, alpha `1`.
///
/// The colour of a venous film of `SOAK_FILM_MM * soak^3` over a mid-grey substrate, `bloodstain::spectral`'s Kubelka–Munk over Bosschaart's tables,
/// divided by the substrate alone and per channel: what a vertex colour must multiply a texture by to be that film over it. No blood colour is
/// authored here. A table of [`SOAK_STEPS`] entries, built once; a `soak` between two is interpolated, outside `[0, 1]` clamped, and `NaN` is `0`.
pub fn soak_tint(soak: f32) -> [f32; 4] {
    let table = &*TINT_TABLE;
    let at = if soak.is_finite() { soak.clamp(0.0, 1.0) } else { 0.0 } * (SOAK_STEPS - 1) as f32;
    let (lo, frac) = (at.floor() as usize, at.fract());
    let hi = (lo + 1).min(SOAK_STEPS - 1);
    let pick = |k: usize| table.get(k).copied().unwrap_or([1.0; 3]);
    let (a, b) = (pick(lo), pick(hi));
    [a[0] + (b[0] - a[0]) * frac, a[1] + (b[1] - a[1]) * frac, a[2] + (b[2] - a[2]) * frac, 1.0]
}

/// The tint of each of [`SOAK_STEPS`] soaks, from bare to fully soaked: a venous film's linear colour over the grey substrate, over the substrate's.
static TINT_TABLE: LazyLock<[[f32; 3]; SOAK_STEPS]> = LazyLock::new(|| {
    let base = spectral::linear_srgb(&Film::venous(0.0));
    let mut t = [[1.0_f32; 3]; SOAK_STEPS];
    for (k, entry) in t.iter_mut().enumerate() {
        let x = k as f32 / (SOAK_STEPS - 1) as f32;
        let film = spectral::linear_srgb(&Film::venous(SOAK_FILM_MM * x * x * x));
        for c in 0..3 {
            let (f, b) = (film.get(c).copied().unwrap_or(0.0), base.get(c).copied().unwrap_or(1.0));
            if let Some(e) = entry.get_mut(c) {
                *e = if b > 1.0e-6 { (f / b).clamp(0.0, 1.0) } else { 1.0 };
            }
        }
    }
    t
});

/// **The texture a spilled strand wears**: `64 × 384` `Rgba8UnormSrgb`, repeating round the wall,
/// for the tube's `UV_0`. Hand it to a material's base colour with a white factor.
///
/// The same bytes every call, so a caller builds it once and shares the handle.
pub fn gut_image() -> Image {
    let mut data = Vec::with_capacity((AROUND * ALONG * 4) as usize);
    for y in 0..ALONG {
        let v = (y as f32 + 0.5) / ALONG as f32;
        for x in 0..AROUND {
            let u = (x as f32 + 0.5) / AROUND as f32;
            // Distance round the wall from the mesenteric side, `0..=0.5`.
            let side = (u - u.round()).abs();
            let mut c = SEROSA;
            // A soft mottle, so the wall is not a flat fill.
            let shade = 0.94 + 0.10 * noise(x, y, 8, 12, 0x6E7A);
            c = c.map(|ch| ch * shade);
            // The mesentery's fat, a thin band along the side the strand hung from.
            c = mix(c, FAT, 0.8 * (1.0 - smooth(0.03, 0.06, side)));
            // The straight vessels, round the wall from that side, each its own length.
            let mut vessel = 0.0f32;
            let k0 = (v * VESSELS as f32).floor() as i64;
            for k in k0 - 1..=k0 + 1 {
                if k < 0 || k >= VESSELS as i64 {
                    continue;
                }
                let h = |salt: u32| hash_unit(hash_pair(0x7A53_11D4 ^ salt, k as u32));
                let centre = (k as f32 + 0.5 + 0.6 * (h(1) - 0.5)) / VESSELS as f32;
                let wiggle = 0.003 * (core::f32::consts::TAU * (3.0 * u + h(2))).sin();
                let reach = 0.18 + 0.22 * h(3);
                let d = (v - centre - wiggle).abs() * ALONG as f32;
                let width = (1.3 * (1.0 - 0.6 * side / reach)).max(0.4);
                let line = 1.0 - smooth(0.5 * width, width, d);
                vessel = vessel.max(line * (1.0 - smooth(0.6 * reach, reach, side)) * smooth(0.04, 0.16, side));
            }
            // Faint: drawn at full strength, every vessel's root lined up along the fat and read as a
            // row of stitches.
            c = mix(c, VESSEL, 0.35 * vessel);
            // Blood: a wash over all of it, heavy at the end that left the wound, and smeared along
            // the strand in patches four times as long as they are wide — a strand drags through the
            // pool, it is not spotted.
            let smear = smooth(0.48, 0.78, noise(x, y, 16, 64, 0xB100));
            let wound_end = 1.0 - smooth(0.04, 0.22, v);
            c = mix(c, BLOOD, (0.15 + 0.75 * smear).max(0.9 * wound_end));
            data.extend(c.map(byte));
            data.push(255);
        }
    }
    let mut image = Image::new(
        Extent3d { width: AROUND, height: ALONG, depth_or_array_layers: 1 },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        ..ImageSamplerDescriptor::linear()
    });
    image
}

/// Value noise in `[0, 1)` at texel `(x, y)`, on a lattice `cu` texels round the wall and `cv` along
/// it, periodic round the wall so the seam at `u = 0` does not show. `cu` divides the texels round.
fn noise(x: u32, y: u32, cu: u32, cv: u32, salt: u32) -> f32 {
    let period = AROUND / cu;
    let (gx, gy) = (x / cu, y / cv);
    let (fx, fy) = ((x % cu) as f32 / cu as f32, (y % cv) as f32 / cv as f32);
    let corner = |i: u32, j: u32| hash_unit(hash_pair(salt ^ (i % period).wrapping_mul(0x9E37_79B9), j));
    let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
    let top = corner(gx, gy) + (corner(gx + 1, gy) - corner(gx, gy)) * sx;
    let bottom = corner(gx, gy + 1) + (corner(gx + 1, gy + 1) - corner(gx, gy + 1)) * sx;
    top + (bottom - top) * sy
}

/// The cubic smoothstep from `a` to `b`.
fn smooth(a: f32, b: f32, t: f32) -> f32 {
    let t = ((t - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// `a` toward `b` by `t`.
fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

/// A `[0, 1]` channel as a byte.
fn byte(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0).round() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The texture wraps round the tube without a seam.** The wall's first and last columns meet at
    /// `u = 0`, and the step between them must be no larger than the steps between neighbouring
    /// columns anywhere else; the mottle and the smears are periodic round the wall for exactly that.
    #[test]
    fn the_gut_texture_wraps_round_the_wall_without_a_seam() {
        let image = gut_image();
        let px = image.data.unwrap_or_default();
        let w = AROUND as usize;
        assert_eq!(px.len(), w * ALONG as usize * 4);
        let step = |row: usize, a: usize, b: usize| -> f32 {
            let (i, j) = ((row * w + a) * 4, (row * w + b) * 4);
            (0..3).map(|c| (px[i + c] as f32 - px[j + c] as f32).abs()).sum()
        };
        let rows = ALONG as usize;
        let seam = (0..rows).map(|r| step(r, w - 1, 0)).sum::<f32>() / rows as f32;
        let inner = (0..rows).map(|r| (1..w).map(|x| step(r, x - 1, x)).sum::<f32>() / (w - 1) as f32).sum::<f32>() / rows as f32;
        assert!(seam <= 1.5 * inner, "the seam steps {seam} on average where neighbours step {inner}");
    }

    /// **A soaked node is redder and darker than a bare one, and more soak never lightens it.** The tint is a venous film over a grey substrate divided by
    /// the substrate, so a bare node is white (it multiplies nothing) and a soaked one passes red and almost no green or blue. Red and the luminance never
    /// rise with the soak; blue alone is allowed to (it creeps up by a hundredth as the film saturates, which is what the spectral tables do).
    #[test]
    fn the_tint_of_blood_only_darkens_and_reddens() {
        assert_eq!(soak_tint(0.0), [1.0, 1.0, 1.0, 1.0], "a bare node multiplies the texture by nothing");
        let luminance = |t: [f32; 4]| 0.2126 * t[0] + 0.7152 * t[1] + 0.0722 * t[2];
        let mut last = soak_tint(0.0);
        for k in 1..=100 {
            let t = soak_tint(k as f32 / 100.0);
            assert!(t[0] <= last[0] + 1.0e-6, "red lightened at soak {}", k as f32 / 100.0);
            assert!(luminance(t) <= luminance(last) + 1.0e-6, "the tint lightened at soak {}", k as f32 / 100.0);
            last = t;
        }
        let full = soak_tint(1.0);
        assert!(full[0] > 3.0 * full[1] && full[0] > 3.0 * full[2], "a soaked node passes red and little else: {full:?}");
        assert_eq!(soak_tint(f32::NAN), soak_tint(0.0), "a node with no soak is bare, not a guess");
        assert_eq!(soak_tint(7.0), soak_tint(1.0));
        assert_eq!(soak_tint(-1.0), soak_tint(0.0));
    }

    /// **The soak's patches are a smooth function of where along the bowel they are.** A step between neighbouring nodes larger than a fifth of the
    /// patchy range would be a visible seam in a coat.
    #[test]
    fn the_patches_of_a_coat_are_smooth_along_the_bowel() {
        let mut last = patch(7, 0.0);
        for k in 1..=2000 {
            let now = patch(7, k as f32 * 0.01);
            assert!((now - last).abs() < 0.1, "the patch noise jumped {} in one hundredth of a cell", (now - last).abs());
            assert!((0.0..=1.0).contains(&now));
            last = now;
        }
    }
}
