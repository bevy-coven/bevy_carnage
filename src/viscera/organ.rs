//! **An organ: a lump of dark, wet, heavy meat, built from a seed.**
//!
//! A heap of guts that is nothing but loops reads as rope. What the films have among their loops (Refs D2 and T1 of `docs/research/organs.md`) are *masses*: a dark wedge of
//! liver, a sac of stomach. [`organ_mesh`] builds one as a closed mesh, and [`organ_image`] the texture it wears; neither spawns anything, as [`crate::viscera::tube_mesh`]
//! does not: the caller chooses the material, the handle and the parent. Both are pure functions of their arguments.
//!
//! **The shape is Gray's liver** (*Anatomy of the Human Body*, 1918, "The Liver"): "its greatest transverse measurement is from 20 to 22.5 cm. Vertically, near its lateral or right
//! surface, it measures about 15 to 17.5 cm., while its greatest antero-posterior diameter ... is from 10 to 12.5 cm"; "its consistence is that of a soft solid; it is friable,
//! easily lacerated and highly vascular; its color is a dark reddish brown, and its specific gravity is 1.05". So: a wedge about 21 by 16 by 10 cm, thinning toward its left end,
//! dark reddish brown, soft enough to **lie flat where it lands**, which is why the mesh has a contact patch where it meets the floor. One wedge with a different seed and size stands
//! for a stomach and a spleen too (`docs/research/organs.md`, `## Deviations`). **Cosmetic**: outside every digest.

use core::f32::consts::{PI, TAU};

use bevy::asset::RenderAssetUsages;
use bevy::image::{Image, ImageAddressMode, ImageSampler, ImageSamplerDescriptor};
use bevy::math::Vec3;
use bevy::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::viscera::hash::{hash_pair, hash_unit};

/// **Half the extents of an organ at `size` 1**, metres, `(x, y, z)` with `y` up when it lies on the floor: `(0.105, 0.05, 0.08)`, which is Gray's liver, 21 cm across,
/// 16 cm deep and, flattened to lie on its posterior surface, 10 cm of its 10 to 12.5 thick.
pub const ORGAN_HALF_EXTENTS: Vec3 = Vec3::new(0.105, 0.05, 0.08);
/// **How thin the wedge is at its left end, as a fraction of its thickness at its right**: 0.55. **Tuned by eye** against Gray's "the thin edge toward the left".
pub const ORGAN_TAPER: f32 = 0.55;
/// **How much three octaves of noise lump the radius**, as fractions of it: 0.16, 0.08 and 0.04. **Tuned by eye**: no two organs are alike, and none is an ellipsoid.
pub const ORGAN_LUMP: [f32; 3] = [0.16, 0.08, 0.04];
/// **How much of the thickness is cut away below the middle to make the contact patch**, as a fraction of the half thickness: 0.35. A "soft solid" of specific gravity 1.05 lies
/// flat on the floor. **Tuned by eye.**
pub const ORGAN_FLAT: f32 = 0.35;

/// Latitude bands and longitude segments of the sphere an organ is displaced from.
const RINGS: usize = 20;
const SEGMENTS: usize = 32;

/// A smooth 3D value noise in `[-1, 1]` at `p`, a pure function of `seed`.
fn noise3(p: Vec3, seed: u32) -> f32 {
    let (cell, frac) = (p.floor(), p - p.floor());
    let corner = |dx: f32, dy: f32, dz: f32| {
        let h = hash_pair(hash_pair(hash_pair(seed, (cell.x + dx) as i32 as u32), (cell.y + dy) as i32 as u32), (cell.z + dz) as i32 as u32);
        hash_unit(h) * 2.0 - 1.0
    };
    let s = frac * frac * (Vec3::splat(3.0) - 2.0 * frac);
    let lerp = |a: f32, b: f32, t: f32| a + (b - a) * t;
    let x00 = lerp(corner(0.0, 0.0, 0.0), corner(1.0, 0.0, 0.0), s.x);
    let x10 = lerp(corner(0.0, 1.0, 0.0), corner(1.0, 1.0, 0.0), s.x);
    let x01 = lerp(corner(0.0, 0.0, 1.0), corner(1.0, 0.0, 1.0), s.x);
    let x11 = lerp(corner(0.0, 1.0, 1.0), corner(1.0, 1.0, 1.0), s.x);
    lerp(lerp(x00, x10, s.y), lerp(x01, x11, s.y), s.z)
}

/// **The surface of an organ in the direction `dir`** (a unit vector), metres, before it is set on the floor: the wedge's ellipsoid, thinned toward `-x`, lumped, and cut flat below.
fn surface(dir: Vec3, seed: u32, size: f32) -> Vec3 {
    let lump = 1.0
        + ORGAN_LUMP[0] * noise3(dir * 1.7 + Vec3::splat(11.0), seed)
        + ORGAN_LUMP[1] * noise3(dir * 3.6 + Vec3::splat(37.0), seed ^ 0x51ED)
        + ORGAN_LUMP[2] * noise3(dir * 7.5 + Vec3::splat(71.0), seed ^ 0xA7C3);
    // The thin edge is to the left (`-x`): a smooth step from the taper at the left end to full thickness two thirds of the way across.
    let t = ((dir.x + 1.0) / 1.6).clamp(0.0, 1.0);
    let thick = ORGAN_TAPER + (1.0 - ORGAN_TAPER) * t * t * (3.0 - 2.0 * t);
    let h = ORGAN_HALF_EXTENTS * size;
    let p = Vec3::new(dir.x * h.x * lump, dir.y * h.y * lump * thick, dir.z * h.z * lump * thick);
    Vec3::new(p.x, p.y.max(-ORGAN_FLAT * h.y), p.z)
}

/// **An organ as a closed mesh**, `size` times Gray's liver (clamped to `0.05..=4`), its lumps the `seed`'s, its lowest point at `y = 0` so that a floor clamp sits it on the floor.
///
/// A UV sphere of 20 bands by 32 segments displaced by [`ORGAN_HALF_EXTENTS`], [`ORGAN_TAPER`], [`ORGAN_LUMP`] and [`ORGAN_FLAT`]. Carries `ATTRIBUTE_POSITION`, `ATTRIBUTE_NORMAL` (unit,
/// the displaced surface's by central differences of the displacement, so **there is no seam and no flat shading on the flattened patch**) and `ATTRIBUTE_UV_0` (`u` round `x`/`z`, `v` top to
/// bottom, for [`organ_image`]), with `U32` indices wound counter-clockwise from outside. `(20 + 1) * (32 + 1)` vertices and `2 * 32 * 19` triangles.
pub fn organ_mesh(seed: u32, size: f32) -> Mesh {
    let size = if size.is_finite() { size.clamp(0.05, 4.0) } else { 1.0 };
    let mut positions: Vec<Vec3> = Vec::with_capacity((RINGS + 1) * (SEGMENTS + 1));
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity((RINGS + 1) * (SEGMENTS + 1));
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity((RINGS + 1) * (SEGMENTS + 1));
    let eps = 0.02;
    for i in 0..=RINGS {
        let theta = PI * i as f32 / RINGS as f32;
        let (sin_t, cos_t) = theta.sin_cos();
        for j in 0..=SEGMENTS {
            let phi = TAU * j as f32 / SEGMENTS as f32;
            let (sin_p, cos_p) = phi.sin_cos();
            let dir = Vec3::new(sin_t * cos_p, cos_t, sin_t * sin_p);
            positions.push(surface(dir, seed, size));
            // The normal from the surface itself: two tangents of the sphere, the displaced surface at either side of each, the cross of the differences.
            let helper = if dir.y.abs() > 0.9 { Vec3::X } else { Vec3::Y };
            let t1 = dir.cross(helper).normalize_or_zero();
            let t2 = dir.cross(t1);
            let d1 = surface((dir + t1 * eps).normalize(), seed, size) - surface((dir - t1 * eps).normalize(), seed, size);
            let d2 = surface((dir + t2 * eps).normalize(), seed, size) - surface((dir - t2 * eps).normalize(), seed, size);
            let mut n = d1.cross(d2).normalize_or_zero();
            // Outward: the sphere's own direction is outward of the displaced surface too, to a lump.
            if n.dot(dir) < 0.0 {
                n = -n;
            }
            if n == Vec3::ZERO {
                n = dir;
            }
            normals.push(n.to_array());
            uvs.push([j as f32 / SEGMENTS as f32, i as f32 / RINGS as f32]);
        }
    }
    // The lowest point is the floor.
    let floor = positions.iter().map(|p| p.y).fold(f32::MAX, f32::min);
    let positions: Vec<[f32; 3]> = positions.into_iter().map(|p| (p - Vec3::new(0.0, floor, 0.0)).to_array()).collect();

    let mut indices: Vec<u32> = Vec::with_capacity(2 * SEGMENTS * (RINGS - 1) * 3);
    let ring = (SEGMENTS + 1) as u32;
    for i in 0..RINGS as u32 {
        for j in 0..SEGMENTS as u32 {
            let (a, b, c, d) = (i * ring + j, i * ring + j + 1, (i + 1) * ring + j, (i + 1) * ring + j + 1);
            // The pole bands are triangles, not quads: the degenerate half is left out.
            // Counter-clockwise seen from outside, which is wgpu's front face: for this parametrisation `(b - a) x (c - a)` points out.
            if i != 0 {
                indices.extend_from_slice(&[a, b, c]);
            }
            if i != RINGS as u32 - 1 {
                indices.extend_from_slice(&[b, d, c]);
            }
        }
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// Texels round an organ and down it.
const IMAGE_W: u32 = 128;
const IMAGE_H: u32 = 64;

/// **The dark reddish brown of Gray's liver**, sRGB: `(0.54, 0.02, 0.025)`. The colour is Gray's ("a dark reddish brown"); the numbers are **tuned by eye** against Ref D2 (`docs/research/organs.md`): the
/// first try, `(0.30, 0.075, 0.07)`, came out of the flesh shader at red minus green 49 against D2's 89, `(0.38, ..)` at 58 and `(0.46, ..)` at 66, because the shader's grey veil lifts every channel and the texture must be redder to end up red.
const LIVER: [f32; 3] = [0.54, 0.02, 0.025];
/// The pale fibrous capsule that shows through in patches, sRGB. **Tuned by eye.**
const CAPSULE: [f32; 3] = [0.55, 0.25, 0.22];
/// Blood smeared over it, sRGB, as the wetmap draws a thick venous film. **Tuned by eye.**
const SMEAR: [f32; 3] = [0.70, 0.02, 0.03];
/// The deep, nearly black red of the parts of an organ that are in shadow of themselves (D2's 5th-percentile luminance is 16), sRGB. **Tuned by eye.**
const DEEP: [f32; 3] = [0.14, 0.01, 0.015];

/// A periodic value noise in `[0, 1)` at texel `(x, y)`: `cu` texels per cell across (a divisor of `IMAGE_W`, so it wraps) and `cv` down.
fn tile_noise(x: u32, y: u32, cu: u32, cv: u32, salt: u32) -> f32 {
    let period = IMAGE_W / cu;
    let (gx, gy) = (x / cu, y / cv);
    let (fx, fy) = ((x % cu) as f32 / cu as f32, (y % cv) as f32 / cv as f32);
    let corner = |i: u32, j: u32| hash_unit(hash_pair(salt ^ (i % period).wrapping_mul(0x9E37_79B9), j));
    let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
    let top = corner(gx, gy) + (corner(gx + 1, gy) - corner(gx, gy)) * sx;
    let bottom = corner(gx, gy + 1) + (corner(gx + 1, gy + 1) - corner(gx, gy + 1)) * sx;
    top + (bottom - top) * sy
}

fn smooth(a: f32, b: f32, t: f32) -> f32 {
    let t = ((t - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

/// **The texture an organ wears**: `128 x 64` `Rgba8UnormSrgb`, repeating round the organ (`u`), for [`organ_mesh`]'s `UV_0`. A dark reddish brown mottled at the scale of a lobule, with
/// pale fibrous patches of capsule and blood smeared over it in streaks along `v`, every channel a seeded noise. The same bytes every call, so a caller builds it once and shares the handle.
pub fn organ_image() -> Image {
    let mut data = Vec::with_capacity((IMAGE_W * IMAGE_H * 4) as usize);
    for y in 0..IMAGE_H {
        for x in 0..IMAGE_W {
            // A lobule is a few millimetres: a fine mottle in both directions.
            let mottle = 0.80 + 0.40 * tile_noise(x, y, 4, 4, 0x0E9A);
            let mut c = LIVER.map(|ch| ch * mottle);
            // The capsule shows through in patches, elongated down the organ.
            let capsule = smooth(0.66, 0.84, tile_noise(x, y, 16, 8, 0xCA95));
            c = mix(c, CAPSULE, 0.40 * capsule);
            // Blood: streaks, longer down the organ than across it.
            let smear = smooth(0.50, 0.75, tile_noise(x, y, 8, 32, 0xB10D));
            c = mix(c, SMEAR, 0.80 * smear);
            // Deep patches, blotchy, over the rest.
            let deep = smooth(0.55, 0.80, tile_noise(x, y, 8, 8, 0xDEE9));
            c = mix(c, DEEP, 0.70 * deep);
            data.extend(c.map(|ch| (ch.clamp(0.0, 1.0) * 255.0).round() as u8));
            data.push(255);
        }
    }
    let mut image = Image::new(
        Extent3d { width: IMAGE_W, height: IMAGE_H, depth_or_array_layers: 1 },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor { address_mode_u: ImageAddressMode::Repeat, ..ImageSamplerDescriptor::linear() });
    image
}

#[cfg(test)]
mod tests {
    use super::*;

    fn positions(mesh: &Mesh) -> Vec<Vec3> {
        mesh.attribute(Mesh::ATTRIBUTE_POSITION).and_then(|a| a.as_float3()).map(|v| v.iter().map(|p| Vec3::from(*p)).collect()).unwrap_or_default()
    }

    /// The signed volume of a closed mesh by the divergence theorem: positive when the triangles wind counter-clockwise seen from outside.
    fn volume(mesh: &Mesh) -> f32 {
        let p = positions(mesh);
        let Some(indices) = mesh.indices() else { return 0.0 };
        let idx: Vec<usize> = indices.iter().collect();
        idx.chunks(3).map(|t| p[t[0]].dot(p[t[1]].cross(p[t[2]])) / 6.0).sum()
    }

    /// **An organ is a closed solid wound outward, set on the floor, with unit normals.**
    #[test]
    fn an_organ_is_a_closed_outward_solid_on_the_floor() {
        for seed in [1_u32, 7, 0xC0FFEE] {
            let mesh = organ_mesh(seed, 1.0);
            let p = positions(&mesh);
            assert_eq!(p.len(), (RINGS + 1) * (SEGMENTS + 1));
            assert!(p.iter().all(|v| v.is_finite()));
            let low = p.iter().map(|v| v.y).fold(f32::MAX, f32::min);
            assert!(low.abs() < 1.0e-5, "the lowest point is the floor, not {low}");
            let v = volume(&mesh);
            let ellipsoid = 4.0 / 3.0 * PI * ORGAN_HALF_EXTENTS.x * ORGAN_HALF_EXTENTS.y * ORGAN_HALF_EXTENTS.z;
            assert!(v > 0.35 * ellipsoid && v < 1.2 * ellipsoid, "an organ of seed {seed} holds {v} m^3 where its ellipsoid holds {ellipsoid}");
            let Some(normals) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL).and_then(|a| a.as_float3()) else { panic!("an organ carries normals") };
            assert!(normals.iter().all(|n| ((n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt() - 1.0).abs() < 1.0e-4));
        }
    }

    /// **Gray's proportions survive the lumps**: wider than it is thick by at least 1.6, deeper than thick, and a flat patch on the floor where it lies.
    #[test]
    fn an_organ_is_a_flat_wedge_with_a_contact_patch() {
        let mesh = organ_mesh(3, 1.0);
        let p = positions(&mesh);
        let extent = |f: fn(&Vec3) -> f32| p.iter().map(f).fold(f32::MIN, f32::max) - p.iter().map(f).fold(f32::MAX, f32::min);
        let (w, t, d) = (extent(|v| v.x), extent(|v| v.y), extent(|v| v.z));
        assert!(w >= 1.6 * t, "wider than thick: {w} against {t}");
        assert!(d > t, "deeper than thick: {d} against {t}");
        let on_floor = p.iter().filter(|v| v.y < 1.0e-5).count();
        assert!(on_floor as f32 > 0.03 * p.len() as f32, "a contact patch: {on_floor} of {} vertices are on the floor", p.len());
    }

    /// **A seed is a shape**: the same seed the same mesh, another seed another, and a size scales it.
    #[test]
    fn a_seed_is_a_shape_and_a_size_scales_it() {
        let (a, b, c) = (positions(&organ_mesh(5, 1.0)), positions(&organ_mesh(5, 1.0)), positions(&organ_mesh(6, 1.0)));
        assert_eq!(a, b);
        assert_ne!(a, c);
        let big = positions(&organ_mesh(5, 2.0));
        let width = |p: &[Vec3]| p.iter().map(|v| v.x).fold(f32::MIN, f32::max) - p.iter().map(|v| v.x).fold(f32::MAX, f32::min);
        assert!((width(&big) / width(&a) - 2.0).abs() < 1.0e-3, "twice the size is twice the width");
    }

    /// **The texture wraps round the organ without a seam**, as the gut's does.
    #[test]
    fn the_organ_texture_wraps_without_a_seam() {
        let image = organ_image();
        let px = image.data.unwrap_or_default();
        let w = IMAGE_W as usize;
        assert_eq!(px.len(), w * IMAGE_H as usize * 4);
        let step = |row: usize, a: usize, b: usize| -> f32 { (0..3).map(|c| (px[(row * w + a) * 4 + c] as f32 - px[(row * w + b) * 4 + c] as f32).abs()).sum() };
        let rows = IMAGE_H as usize;
        let seam = (0..rows).map(|r| step(r, w - 1, 0)).sum::<f32>() / rows as f32;
        let inner = (0..rows).map(|r| (1..w).map(|x| step(r, x - 1, x)).sum::<f32>() / (w - 1) as f32).sum::<f32>() / rows as f32;
        assert!(seam <= 1.5 * inner, "the seam steps {seam} on average where neighbours step {inner}");
    }
}
