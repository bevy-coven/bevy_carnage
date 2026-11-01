//! **The render mesh: an `n`-gon swept along the node polyline, rebuilt on the CPU.**
//!
//! Rendering a strand is the caller's job. This hands back a [`Mesh`] and **never spawns** — no
//! entity, no material, no asset handle. An eight-sided tube over the 25 nodes [`crate::viscera::spill`]
//! produces is 384 side triangles plus 16 of cap, which is cheap enough to regenerate every tick and
//! is why there is no GPU skinning path to keep in step with the solver.

use core::f32::consts::TAU;

use bevy::asset::RenderAssetUsages;
use bevy::math::Vec3;
use bevy::mesh::{Indices, Mesh, PrimitiveTopology};

use crate::viscera::frame::perpendicular_basis;
use crate::viscera::hash::{hash_pair, hash_unit};
use crate::viscera::look::{soak, soak_tint};
use crate::viscera::strand::Strand;

/// The fewest sides a tube can have and still enclose anything.
pub const MIN_SIDES: u32 = 3;
/// The most sides [`tube_mesh`] will build. Past this the silhouette stops changing.
pub const MAX_SIDES: u32 = 32;

/// **Build the tube for one strand.**
///
/// `sides` is clamped into `MIN_SIDES..=MAX_SIDES`. The result is a `TriangleList` carrying
/// `ATTRIBUTE_POSITION`, `ATTRIBUTE_NORMAL` and `ATTRIBUTE_UV_0` with `U32` indices, at
/// `RenderAssetUsages::default()` — main world *and* render world, because a caller regenerating the
/// mesh every tick has to be able to read back the one it just built.
///
/// Counts, exactly: `(nodes + 2) × (sides + 1) + 2` vertices — a duplicated seam vertex per ring so
/// the `u` coordinate runs 0→1 without wrapping, plus a fan centre and a rim copy for each end cap —
/// and `untorn_segments × sides × 2 + 2 × sides` triangles. Every emitted normal is unit length: the
/// side walls carry the radial, the caps carry the end tangent.
///
/// **A parted segment emits no side quads.** The tube is severed where the strand is severed, which
/// leaves the two new ends open: the caller is looking at a tear, and a tear is open.
///
/// The cross-section frame is parallel-transported. The first ring's reference is the world axis
/// *least aligned* with the first tangent (see [`crate::viscera::frame`]), and every ring after it re-projects
/// the previous ring's normal onto the new tangent's plane. Picking the reference from the tangent's
/// smallest component rather than always taking `Y` is what stops the frame flipping when a strand
/// happens to lie along an axis, and it makes the sweep a total function of the node positions — the
/// same strand gives the same mesh on every run.
///
/// **The radius is not constant.** A bowel bulges between its constrictions, so each ring's radius is the strand's by a
/// factor of two sines along its length ([`LUMP_MAJOR`], [`LUMP_MINOR`]), their phases from [`Strand::seed`], and a strand
/// laid by [`crate::viscera::spill`] has a seed of its own. Every factor averages `1`, so a strand's mass and its floor clamp
/// (`floor_y + radius`, in the solver) are the same; only what is drawn is lumpy. The seam, the counts and the unit normals
/// are as before. For the slump a strand takes where it lies on the floor, and the tint of the blood it lies in, see [`soaked_tube`].
pub fn tube_mesh(strand: &Strand, sides: u32) -> Mesh {
    sweep(strand, sides, None)
}

/// **The same tube, slumped where it lies on the floor and tinted by the blood it is soaked in.**
///
/// `floor_y` is the height of the surface the strand rests on (the solver's own, `ViscSettings::floor_y`). A ring whose centre is
/// within [`SLUMP_FADE`] radii of resting on it is squashed to [`SLUMP_SQUASH`] of its height and widened by the inverse square
/// root, so a loop on the floor is flatter than it is wide and its area is kept; its normals are the ellipse's. The mesh also
/// carries `ATTRIBUTE_COLOR`: [`soak`] says how soaked each node is and [`soak_tint`] what that multiplies the material's texture by.
///
/// Everything else, including the vertex count and the indices, is [`tube_mesh`]'s. **Cosmetic**: the strand's nodes, its digest and
/// every canvas are untouched, and a `NaN` `floor_y` slumps nothing and soaks only by the wound and the base.
pub fn soaked_tube(strand: &Strand, sides: u32, floor_y: f32) -> Mesh {
    sweep(strand, sides, Some(floor_y))
}

/// **How strongly the two sines that lump a strand's radius swing it**, as a fraction of the radius: 22 % on a period of
/// [`LUMP_MAJOR_NODES`] nodes (about 19 cm of a spilled strand, a loop's bulge between two constrictions) and [`LUMP_MINOR`], 9 % on
/// [`LUMP_MINOR_NODES`] (8 cm, the haustra). **Tuned by eye** against the reference set in `docs/research/guts.md` (R1, D3), not measured: 14 % and 6 % (the first
/// round) read as a smooth pipe from the preset's camera; at these the silhouette shows two scales of bulge without reading as a string of sausages.
pub const LUMP_MAJOR: f32 = 0.22;
/// The period of [`LUMP_MAJOR`], in nodes.
pub const LUMP_MAJOR_NODES: f32 = 5.5;
/// See [`LUMP_MAJOR`].
pub const LUMP_MINOR: f32 = 0.09;
/// The period of [`LUMP_MINOR`], in nodes.
pub const LUMP_MINOR_NODES: f32 = 2.3;
/// **How far above resting on the floor a ring is still slumped**, in radii: contact is `1` for a centre at `floor_y + radius` and falls
/// smoothly to `0` this much higher. **Tuned**, `docs/research/guts.md`.
pub const SLUMP_FADE: f32 = 1.4;
/// **How tall a ring lying on the floor is**, as a fraction of its radius-high circle: 0.72. **Tuned** against R1 and D3: a loop of bowel
/// under its own weight is a flattened oval, not a pipe.
pub const SLUMP_SQUASH: f32 = 0.72;

/// The factor a strand's radius is drawn at, at node `i`: `1` plus the two sines, their phases `seed`'s.
pub(crate) fn lump(seed: u32, i: usize) -> f32 {
    let phase = |salt: u32| hash_unit(hash_pair(seed, salt));
    let at = i as f32;
    1.0 + LUMP_MAJOR * (TAU * (at / LUMP_MAJOR_NODES + phase(0xA11)) ).sin() + LUMP_MINOR * (TAU * (at / LUMP_MINOR_NODES + phase(0xB22))).sin()
}

/// How much a node `height_in_radii` radii above the floor is lying on it: `1` at one radius (resting), `0` at `1 + SLUMP_FADE` or above.
/// A non-finite height is not in contact.
pub(crate) fn contact(height_in_radii: f32) -> f32 {
    if !height_in_radii.is_finite() {
        return 0.0;
    }
    let t = ((height_in_radii - 1.0) / SLUMP_FADE).clamp(0.0, 1.0);
    1.0 - t * t * (3.0 - 2.0 * t)
}

/// The sweep both public builders share: `floor_y` is `Some` for the slumped, tinted tube.
fn sweep(strand: &Strand, sides: u32, floor_y: Option<f32>) -> Mesh {
    let sides = sides.clamp(MIN_SIDES, MAX_SIDES) as usize;
    let ring = sides + 1; // the seam vertex is duplicated so UVs do not wrap
    let nodes = strand.nodes();
    let radius = strand.radius();
    let node_count = nodes.len();

    let vertex_cap = (node_count + 2) * ring;
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(vertex_cap);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertex_cap);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(vertex_cap);
    let mut indices: Vec<u32> = Vec::with_capacity(node_count * sides * 6 + sides * 6);

    // `Strand::new` clamps to at least one segment, so a shorter polyline can only arrive from a
    // future edit. It yields an empty mesh rather than a panic.
    if node_count < 2 {
        return finish(positions, normals, uvs, indices);
    }

    // The radius each ring is drawn at, as a factor of the strand's.
    let scale: Vec<f32> = (0..node_count).map(|i| lump(strand.seed(), i)).collect();
    let scale_at = |i: usize| scale.get(i).copied().unwrap_or(1.0);

    // --- the swept rings ------------------------------------------------------------------------
    let mut tangent = tangent_at(nodes, 0);
    let (mut normal, mut binormal) = perpendicular_basis(tangent);
    let span = (node_count - 1) as f32;

    for (i, centre) in nodes.iter().enumerate() {
        if i > 0 {
            let next = tangent_at(nodes, i);
            // Parallel transport by projection: keep whatever of the previous normal is already
            // perpendicular to the new tangent. A 180° reversal between adjacent nodes is the only
            // case with no answer, and there the previous frame is kept rather than guessed at.
            let projected = normal - next * normal.dot(next);
            normal = projected.try_normalize().unwrap_or(normal);
            binormal = next.cross(normal);
            tangent = next;
        }
        let v = i as f32 / span;
        let ring_radius = radius * scale_at(i);
        // Slumped where it lies on the floor: `squash` of the height, widened to keep the area.
        let (squash, widen) = match floor_y {
            Some(floor) if radius > 0.0 => {
                let squash = 1.0 - (1.0 - SLUMP_SQUASH) * contact((centre.y - floor) / radius);
                (squash, 1.0 / squash.sqrt())
            }
            _ => (1.0, 1.0),
        };
        // How fast the radius grows along the strand, so the wall's normal leans the way the wall does.
        let (before, after) = (i.saturating_sub(1), (i + 1).min(node_count - 1));
        let run = match (nodes.get(before), nodes.get(after)) {
            (Some(a), Some(b)) => (*b - *a).length(),
            _ => 0.0,
        };
        let slope = if run > 1.0e-6 { radius * (scale_at(after) - scale_at(before)) / run } else { 0.0 };
        for j in 0..ring {
            let (sin_t, cos_t) = (TAU * (j as f32) / (sides as f32)).sin_cos();
            let radial = normal * cos_t + binormal * sin_t;
            let offset = Vec3::new(radial.x * widen, radial.y * squash, radial.z * widen) * ring_radius;
            positions.push((*centre + offset).to_array());
            // The ellipse's normal, leaned back by the slope: a plain radial when neither applies.
            let leaned = Vec3::new(radial.x / widen, radial.y / squash, radial.z / widen) - tangent * slope;
            normals.push(leaned.try_normalize().unwrap_or(radial).to_array());
            uvs.push([j as f32 / sides as f32, v]);
        }
    }
    // `tangent` now holds the last node's tangent; the loop above walked every node.
    let last_tangent = tangent;

    for seg in 0..node_count - 1 {
        if strand.segment_torn(seg) {
            continue;
        }
        let base = (seg * ring) as u32;
        let next = base + ring as u32;
        for j in 0..sides as u32 {
            let (a, b, c, d) = (base + j, base + j + 1, next + j, next + j + 1);
            // Counter-clockwise seen from outside, which is wgpu's front face.
            indices.extend_from_slice(&[a, b, c, b, d, c]);
        }
    }

    // --- the two end caps -----------------------------------------------------------------------
    // Snapshot the rim positions before appending, so the cap can carry its own flat normals without
    // aliasing the side wall's vertices.
    let first_rim: Vec<[f32; 3]> = positions.iter().take(ring).copied().collect();
    let last_rim: Vec<[f32; 3]> =
        positions.iter().skip((node_count - 1) * ring).take(ring).copied().collect();
    let first_tangent = tangent_at(nodes, 0);

    if let Some(&start) = nodes.first() {
        // The rings wind counter-clockwise about `+tangent`, so the cap facing `-tangent` reverses.
        push_cap(
            &mut positions,
            &mut normals,
            &mut uvs,
            &mut indices,
            start,
            -first_tangent,
            &first_rim,
            true,
        );
    }
    if let Some(&end) = nodes.last() {
        push_cap(
            &mut positions,
            &mut normals,
            &mut uvs,
            &mut indices,
            end,
            last_tangent,
            &last_rim,
            false,
        );
    }

    let vertex_count = positions.len();
    let mut mesh = finish(positions, normals, uvs, indices);
    if let Some(floor) = floor_y {
        // One tint per ring, the end caps the tint of the ring they close: the layout is `node_count` rings, then the two fans.
        let tints: Vec<[f32; 4]> = soak(strand, floor).into_iter().map(soak_tint).collect();
        let tint_at = |i: usize| tints.get(i).copied().unwrap_or([1.0; 4]);
        let mut colours: Vec<[f32; 4]> = Vec::with_capacity(vertex_count);
        for i in 0..node_count {
            colours.extend(core::iter::repeat_n(tint_at(i), ring));
        }
        // Each fan is one centre vertex and one copy of the rim: `1 + ring`.
        colours.extend(core::iter::repeat_n(tint_at(0), 1 + ring));
        colours.extend(core::iter::repeat_n(tint_at(node_count - 1), 1 + ring));
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colours);
    }
    mesh
}

/// The tangent at node `i`: a central difference in the interior, one-sided at the ends.
///
/// A coincident pair has no direction; `Vec3::Y` stands in so the frame stays orthonormal, which is
/// the only property the sweep needs from it.
fn tangent_at(nodes: &[Vec3], i: usize) -> Vec3 {
    let n = nodes.len();
    let (a, b) = if i == 0 {
        (nodes.first().copied(), nodes.get(1).copied())
    } else if i + 1 >= n {
        (nodes.get(n.saturating_sub(2)).copied(), nodes.last().copied())
    } else {
        (nodes.get(i - 1).copied(), nodes.get(i + 1).copied())
    };
    match (a, b) {
        (Some(a), Some(b)) => (b - a).try_normalize().unwrap_or(Vec3::Y),
        _ => Vec3::Y,
    }
}

/// A flat fan closing one end of the tube: a centre vertex plus its own copy of the rim, so the cap
/// carries the end tangent as its normal rather than the side wall's radials.
fn push_cap(
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
    centre: Vec3,
    outward: Vec3,
    rim: &[[f32; 3]],
    reverse: bool,
) {
    let ring = rim.len();
    let sides = ring.saturating_sub(1);
    if sides < MIN_SIDES as usize {
        return;
    }
    let base = positions.len() as u32;
    let normal = outward.to_array();

    positions.push(centre.to_array());
    normals.push(normal);
    uvs.push([0.5, 0.5]);
    for (j, p) in rim.iter().enumerate() {
        let (sin_t, cos_t) = (TAU * (j as f32) / (sides as f32)).sin_cos();
        positions.push(*p);
        normals.push(normal);
        uvs.push([0.5 + 0.5 * cos_t, 0.5 + 0.5 * sin_t]);
    }

    for j in 0..sides as u32 {
        let a = base + 1 + j;
        let b = base + 2 + j; // the duplicated seam vertex closes the fan without a modulo
        if reverse {
            indices.extend_from_slice(&[base, b, a]);
        } else {
            indices.extend_from_slice(&[base, a, b]);
        }
    }
}

/// Assemble the attribute arrays into a mesh.
fn finish(
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}
