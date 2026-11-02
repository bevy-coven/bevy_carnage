//! **The observable contracts, one test each.**
//!
//! The headline claim is that this wetmap is reproducible, so the first two tests are the claim and its
//! negation: the same scripted hits give the same digest, and **one hit moved by a single texel** gives
//! a different one. A digest test that only proved equality would pass on a function that returned a
//! constant.
//!
//! Everything else defends a rule the crate states in prose somewhere: mass is conserved, dry paint
//! does not move, a mesh without UVs is refused rather than silently painted, a hit lands where the
//! mesh's UVs say it should, the upload budget is a budget, and the plugin's system survives a missing
//! resource — which in Bevy 0.19 is a panic rather than a skip unless you ask for an `Option`.

use bevy::app::{App, TaskPoolPlugin};
use bevy::asset::{AssetPlugin, Assets};
use bevy::ecs::component::Component;
use bevy::image::{Image, ImagePlugin};
use bevy::shape::Sphere;
use bevy::math::{Vec2, Vec3};
use bevy::mesh::{Mesh, Meshable, PrimitiveTopology};
use bevy::transform::components::GlobalTransform;
use bevy_carnage::wetmap::{Smear, StainShape, WetCanvas, WetSettings, WetmapPlugin};

/// A round stain of a given size in metres, with no spines, so a test's expectations are about the
/// wetmap and not about `bloodstain`'s silhouette.
fn blob(major: f32) -> StainShape {
    StainShape { major, minor: major, spines: 0, satellites: 0, direction: [1.0, 0.0], seed: 11 }
}

fn scratch(size: u32) -> (Assets<Image>, WetCanvas) {
    let mut images = Assets::<Image>::default();
    let canvas = WetCanvas::new(&mut images, size, [0.78, 0.66, 0.60], 0.55);
    (images, canvas)
}

/// Total coverage on the canvas, exactly — the integer quantity `wetted_area` scales.
fn coverage(canvas: &WetCanvas) -> u32 {
    let mut sum = 0;
    for y in 0..canvas.size() {
        for x in 0..canvas.size() {
            sum += canvas.amount_at(x, y) as u32;
        }
    }
    sum
}

/// The scripted sequence both digest tests run: three hits at different ticks, ticked in between, so
/// the drip, spread, age and shade passes have all run more than once by the end.
fn scripted_run(nudge_texels: i32) -> u64 {
    let size = 64u32;
    let (_images, mut canvas) = scratch(size);
    let s = WetSettings::default();
    let nudge = nudge_texels as f32 / size as f32;
    let hits: [(f32, f32, u32); 3] =
        [(0.30 + nudge, 0.20, 0), (0.55, 0.35, 6), (0.32, 0.50, 14)];
    let mut next = 0;
    for tick in 0..30u32 {
        while next < hits.len() && hits[next].2 == tick {
            canvas.paint_uv(Vec2::new(hits[next].0, hits[next].1), &blob(0.09), tick);
            next += 1;
        }
        canvas.tick(tick, Vec2::new(0.0, 1.0), &s);
    }
    canvas.digest()
}

#[test]
fn two_identical_runs_agree_to_the_bit() {
    assert_eq!(scripted_run(0), scripted_run(0));
}

#[test]
fn one_hit_moved_by_a_single_texel_changes_the_digest() {
    let baseline = scripted_run(0);
    let nudged = scripted_run(1);
    assert_ne!(
        baseline, nudged,
        "moving a hit by one texel left the digest alone — the fold is not reading the buffer"
    );
}

/// **The edge dial, and the reason it is a dial.**
///
/// One painter, three settings, so the only thing that differs between the three runs is
/// `edge_samples`. At `1` the stamp must be the rasterisation the crate has always written — pinned
/// to the byte through the digest, because that is what keeps every frozen wetmap golden frozen. At
/// `4` the rim gains partial coverage and the total converges: point-sampling one coverage value per
/// texel is a quadrature of the silhouette with one tap, and the error it makes is the staircase.
#[test]
fn the_edge_dial_smooths_the_rim_and_one_sample_is_the_frozen_rasterisation() {
    let size = 128u32;
    let shape = blob(0.25);
    let paint = |samples: u32| -> (u64, u32, usize, usize) {
        let (_images, mut canvas) = scratch(size);
        let s = WetSettings { edge_samples: samples, ..Default::default() };
        canvas.paint_uv_with(Vec2::new(0.5, 0.5), &shape, 0, &s);
        let mut partial = 0;
        let mut wetted = 0;
        for y in 0..size {
            for x in 0..size {
                let a = canvas.amount_at(x, y);
                if a > 0 {
                    wetted += 1;
                }
                if a > 0 && a < 255 {
                    partial += 1;
                }
            }
        }
        (canvas.digest(), coverage(&canvas), partial, wetted)
    };

    let (digest_1, sum_1, partial_1, wetted_1) = paint(1);
    let (_, sum_4, partial_4, wetted_4) = paint(4);

    // The frozen rasterisation. `edge_samples = 1` is one tap, divisor one, rounding term zero — the
    // identity — so this digest is the one this canvas has always produced for this stamp.
    assert_eq!(
        digest_1, 0xa3bc_55f8_b50f_7c3d,
        "one sample per texel moved the shipped rasterisation"
    );

    // The disc's footprint is its area. `rasterise` puts the rim at `0.5 / 1.04` of the mask's
    // half-width for a spineless shape, so a `major = 0.25` stain on a 128-texel canvas is a disc of
    // radius `0.25 · 128 · 0.5 / 1.04 = 15.4` texels.
    let r = 0.25 * size as f32 * 0.5 / 1.04;
    let area = std::f32::consts::PI * r * r;
    for (samples, wetted) in [(1u32, wetted_1), (4, wetted_4)] {
        let err = (wetted as f32 - area).abs() / area;
        assert!(err < 0.05, "at {samples} samples the footprint was {wetted} texels, not ~{area:.0}");
    }

    // Anti-aliasing is more texels carrying a *share* of the edge, not more blood.
    assert!(
        partial_4 > partial_1,
        "four samples produced no more partial texels than one ({partial_4} vs {partial_1})"
    );
    // …and it is exactly that: a redistribution. The dial must not make a stain wetter, because the
    // coverage byte is a film depth and a smoother rim is not more blood.
    let drift = (sum_4 as f32 - sum_1 as f32).abs() / sum_1 as f32;
    assert!(drift < 0.01, "the edge dial moved {:.1} % of the coverage", 100.0 * drift);
}

#[test]
fn a_tick_conserves_every_byte_of_coverage() {
    // `absorbency = 0` isolates movement from the substrate's cut: the drip and spread passes are
    // each mass-conserving on their own, and this is what says so through the public surface.
    let s = WetSettings { absorbency: 0.0, ..Default::default() };
    let (_images, mut canvas) = scratch(64);
    canvas.paint_uv(Vec2::new(0.5, 0.25), &blob(0.12), 0);
    let before = coverage(&canvas);
    assert!(before > 0, "the stamp painted nothing");

    canvas.tick(0, Vec2::new(0.0, 1.0), &s);
    assert_eq!(before, coverage(&canvas), "one tick lost or invented coverage");

    // Five more, and the run still has not reached a border, so it is still exact.
    for tick in 1..6u32 {
        canvas.tick(tick, Vec2::new(0.0, 1.0), &s);
    }
    assert_eq!(before, coverage(&canvas), "a run of ticks leaked coverage");
}

#[test]
fn dry_paint_does_not_move() {
    // `dry_ticks = 1` means the paint is set after its first tick, so everything from tick 1 onwards
    // must be a fixed point.
    let s = WetSettings { dry_ticks: 1, ..Default::default() };
    let (_images, mut canvas) = scratch(32);
    canvas.paint_uv(Vec2::new(0.5, 0.3), &blob(0.15), 0);
    canvas.tick(0, Vec2::new(0.0, 1.0), &s);
    let settled = canvas.digest();
    let settled_coverage = coverage(&canvas);
    assert!(settled_coverage > 0, "nothing was left to be dry");

    for tick in 1..50u32 {
        canvas.tick(tick, Vec2::new(0.0, 1.0), &s);
    }
    assert_eq!(canvas.digest(), settled, "dry paint moved");
    assert_eq!(coverage(&canvas), settled_coverage, "dry paint soaked away");
}

#[test]
fn a_mesh_with_no_uvs_is_refused_and_a_miss_is_refused_too() {
    let (_images, mut canvas) = scratch(32);
    let xf = GlobalTransform::default();

    let mut bare = Mesh::new(PrimitiveTopology::TriangleList, bevy::asset::RenderAssetUsages::MAIN_WORLD);
    bare.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
    );
    bare.insert_indices(bevy::mesh::Indices::U32(vec![0, 1, 2]));
    assert!(
        canvas
            .paint_world(
                &bare,
                &xf,
                Vec3::new(0.2, 0.2, 1.0),
                Vec3::new(0.0, 0.0, -1.0),
                &blob(0.1),
                0
            )
            .is_none(),
        "a mesh with no UVs must be refused, not silently painted onto"
    );
    assert_eq!(coverage(&canvas), 0, "a refused mesh still painted something");

    let sphere = Sphere::new(0.5).mesh().uv(32, 18);
    assert!(
        canvas
            .paint_world(
                &sphere,
                &xf,
                Vec3::new(0.0, 5.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                &blob(0.1),
                0
            )
            .is_none(),
        "a ray pointing away from the mesh must miss"
    );
    assert_eq!(coverage(&canvas), 0, "a miss painted something");
}

#[test]
fn a_hit_on_a_uv_sphere_lands_where_the_spheres_uvs_say_it_should() {
    // Bevy's UV sphere puts its pole on +Z and its seam on +X: `u = atan2(y, x) / 2pi` and
    // `v = 0.5 - asin(z / r) / pi` (`bevy_mesh-0.19.0/src/primitives/dim3/sphere.rs:187-200`). So the
    // analytic answer is computable, and the assertion is against the geometry rather than against a
    // previously recorded number.
    //
    // The aim is deliberately off every axis: +X is exactly on the u seam and the equator is exactly
    // on a vertex ring, and a test that fired at either would be asserting which side of a shared
    // edge Moller-Trumbore happened to pick.
    let size = 128u32;
    let (_images, mut canvas) = scratch(size);
    let sphere = Sphere::new(0.5).mesh().uv(32, 18);
    let xf = GlobalTransform::default();

    let aim = Vec3::new(0.6, 0.7, 0.35).normalize();
    let expected_u = aim.y.atan2(aim.x) / std::f32::consts::TAU;
    let expected_v = 0.5 - aim.z.asin() / std::f32::consts::PI;

    // One texel of paint, so the wet centroid IS the stamp centre — and the UV the paint reports is
    // the spot it painted.
    let painted = canvas.paint_world(
        &sphere,
        &xf,
        aim * 5.0,
        -aim,
        &blob(1.0 / size as f32),
        0
    );
    let Some(painted) = painted else { panic!("the ray missed the sphere") };
    assert!(
        (painted.x - expected_u).abs() < 1.0 / size as f32 && (painted.y - expected_v).abs() < 1.0 / size as f32,
        "reported {painted:?}, expected ({expected_u}, {expected_v})"
    );

    let mut weight = 0f32;
    let mut cx = 0f32;
    let mut cy = 0f32;
    for y in 0..size {
        for x in 0..size {
            let a = canvas.amount_at(x, y) as f32;
            if a > 0.0 {
                weight += a;
                cx += a * x as f32;
                cy += a * y as f32;
            }
        }
    }
    assert!(weight > 0.0, "the hit reported success but painted nothing");
    let (cx, cy) = (cx / weight, cy / weight);
    let (want_x, want_y) = (expected_u * size as f32, expected_v * size as f32);
    // Three texels of tolerance: the mesh is a 32x18 facetted approximation of the sphere, so the hit
    // point is a chord rather than the analytic surface and its UV is off by up to half a facet.
    assert!(
        (cx - want_x).abs() <= 3.0 && (cy - want_y).abs() <= 3.0,
        "paint landed at ({cx:.1}, {cy:.1}) texels but the sphere's UVs put that hit at \
         ({want_x:.1}, {want_y:.1})"
    );
}

#[test]
fn wetted_area_rises_with_paint_and_falls_as_the_substrate_takes_its_cut() {
    // Gravity zero, so nothing runs off a border and the only thing changing the total is absorption.
    let s = WetSettings { dry_ticks: 120, ..Default::default() };
    let (_images, mut canvas) = scratch(32);
    assert_eq!(canvas.wetted_area(), 0.0, "a blank canvas is not dry");

    canvas.paint_uv(Vec2::new(0.5, 0.5), &blob(0.4), 0);
    let fresh = canvas.wetted_area();
    assert!(fresh > 0.0, "paint did not wet anything");

    for tick in 0..s.dry_ticks {
        canvas.tick(tick, Vec2::ZERO, &s);
    }
    let dried = canvas.wetted_area();
    assert!(dried < fresh, "the substrate absorbed nothing: {fresh} -> {dried}");
    assert!(dried > 0.0, "the whole stain soaked away — absorbency is being read per tick");
}

#[test]
fn flush_uploads_only_when_dirty() {
    let s = WetSettings::default();
    let (mut images, mut canvas) = scratch(16);
    assert!(!canvas.is_dirty(), "a fresh canvas claims to be out of date");
    assert!(!canvas.flush(&mut images), "a clean canvas uploaded anyway");

    canvas.paint_uv(Vec2::new(0.5, 0.5), &blob(0.2), 3);
    canvas.tick(3, Vec2::new(0.0, 1.0), &s);
    assert_eq!(canvas.dirty_since(), Some(3), "the dirty tick is not the tick it was painted on");
    assert!(canvas.flush(&mut images), "a painted canvas refused to upload");
    assert!(!canvas.is_dirty(), "flush did not clear the dirty flag");
    assert!(!canvas.flush(&mut images), "a freshly flushed canvas uploaded twice");
}

/// **What a renderer samples, all four channels of it.**
///
/// The metallic-roughness image is no longer two channels and two constants: **R is the coverage
/// byte and A is wetness**, `round(255 · (1 − age / dry_ticks))` and `0` where there is no blood —
/// which is why an untouched texel here is `[0, 140, 0, 0]` rather than the `[0, 140, 0, 255]` this
/// test pinned before the data channels existed. G is still roughness and B is still the dielectric
/// zero. No digest moved with it: `digest()` folds the buffer, never the pixels.
#[test]
fn what_reaches_the_gpu_is_blood_over_the_base_surface() {
    // **A test may read `Assets<Image>`; the library may not.** This is the only place in the crate
    // that looks at an uploaded pixel, and it exists because "the canvas is composited on the CPU so
    // there is nothing to blend in WGSL" is a claim about what a renderer will actually sample. A
    // green build proves the handles exist; this proves they carry blood.
    let s = WetSettings::default();
    let size = 32u32;
    let mut images = Assets::<Image>::default();
    let mut canvas = WetCanvas::new(&mut images, size, [0.78, 0.66, 0.60], 0.55);
    let albedo = canvas.albedo();
    let roughness = canvas.roughness();

    let at = |image: Option<&Image>, x: u32, y: u32| -> [u8; 4] {
        let data = image.and_then(|i| i.data.as_ref()).expect("the canvas built its images");
        let i = ((y * size + x) * 4) as usize;
        [data[i], data[i + 1], data[i + 2], data[i + 3]]
    };

    let base_albedo = at(images.get(&albedo), size / 2, size / 2);
    let base_rough = at(images.get(&roughness), size / 2, size / 2);
    assert_eq!(base_albedo, [199, 168, 153, 255], "the base surface is not the sRGB it was given");
    // G is roughness, B is metallic (`bevy_pbr-0.19.0/src/pbr_material.rs:153-154`), and blood is a
    // dielectric, so B stays 0. R is the coverage byte and A is wetness — this crate's own two data
    // channels, both zero on a texel with no blood on it.
    assert_eq!(base_rough, [0, 140, 0, 0], "the base roughness is not in the green channel");

    canvas.paint_uv(Vec2::new(0.5, 0.5), &blob(0.25), 0);
    canvas.tick(0, Vec2::new(0.0, 1.0), &s);
    assert!(canvas.flush(&mut images));

    let wet_albedo = at(images.get(&albedo), size / 2, size / 2);
    let wet_rough = at(images.get(&roughness), size / 2, size / 2);
    let base_chroma = base_albedo[0] as i32 - base_albedo[1] as i32;
    let wet_chroma = wet_albedo[0] as i32 - wet_albedo[1] as i32;
    assert!(
        wet_chroma > base_chroma,
        "the uploaded albedo did not get redder: {base_albedo:?} -> {wet_albedo:?}"
    );
    assert!(
        wet_rough[1] < base_rough[1],
        "fresh blood must be glossier than the dry surface: {base_rough:?} -> {wet_rough:?}"
    );
    assert_eq!(wet_rough[2], 0, "blood stopped being a dielectric");
    // **The data channels carry the buffer, not an inference from the colour.** A freshly stamped
    // texel at full coverage reports its amount in R and full wetness in A; one tick of age has not
    // moved it off 255 at the shipped 1800-tick drying span.
    assert_eq!(wet_rough[0], canvas.amount_at(size / 2, size / 2), "R is not the coverage byte");
    assert_eq!(wet_rough[3], 255, "fresh blood is not reported as wet");

    // Dry it, and the wetness channel is what changed.
    for tick in 0..s.dry_ticks {
        canvas.tick(tick, Vec2::ZERO, &s);
    }
    assert!(canvas.flush(&mut images));
    let dry_rough = at(images.get(&roughness), size / 2, size / 2);
    assert_eq!(dry_rough[3], 0, "a set texel still reports as wet: {dry_rough:?}");
    assert!(dry_rough[1] > wet_rough[1], "dry blood is not rougher than fresh: {dry_rough:?}");
}

/// Which canvas is which, for the budget test.
#[derive(Component)]
struct Slot(u32);

fn headless_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        TaskPoolPlugin::default(),
        AssetPlugin::default(),
        ImagePlugin::default(),
        WetmapPlugin,
    ));
    app
}

#[test]
fn the_plugin_uploads_at_most_the_budget_oldest_dirty_first() {
    let mut app = headless_app();
    let s = WetSettings::default();
    let budget = s.max_canvas_updates_per_tick;
    let total = budget + 2;

    app.world_mut().resource_scope(|world, mut images: bevy::ecs::change_detection::Mut<Assets<Image>>| {
        for slot in 0..total {
            let mut canvas = WetCanvas::new(&mut images, 16, [0.8, 0.7, 0.6], 0.5);
            // Painted on tick `slot`, so `dirty_since` orders them and the sort has real work to do.
            canvas.paint_uv(Vec2::new(0.5, 0.5), &blob(0.2), slot);
            canvas.tick(slot, Vec2::new(0.0, 1.0), &s);
            assert_eq!(canvas.dirty_since(), Some(slot));
            world.spawn((canvas, Slot(slot)));
        }
    });

    app.update();

    let mut still_dirty: Vec<u32> = app
        .world_mut()
        .query::<(&WetCanvas, &Slot)>()
        .iter(app.world())
        .filter(|(canvas, _)| canvas.is_dirty())
        .map(|(_, slot)| slot.0)
        .collect();
    still_dirty.sort_unstable();
    assert_eq!(
        still_dirty,
        (budget..total).collect::<Vec<_>>(),
        "the budget uploaded the wrong canvases — oldest dirty must go first"
    );

    // A second frame clears the remainder, and a third has nothing to do.
    app.update();
    let dirty_after = app
        .world_mut()
        .query::<&WetCanvas>()
        .iter(app.world())
        .filter(|canvas| canvas.is_dirty())
        .count();
    assert_eq!(dirty_after, 0, "the backlog never drained");
}

#[test]
fn the_upload_system_survives_a_missing_settings_resource() {
    // Bevy 0.19 PANICS a system with a missing `Res<T>` rather than skipping it, so the system takes
    // an `Option` even though the plugin inits the resource. This is that guard, checked.
    let mut app = headless_app();
    assert!(app.world().get_resource::<WetSettings>().is_some(), "the plugin did not init its dials");
    app.world_mut().remove_resource::<WetSettings>();
    app.update();
    app.update();
}

#[test]
fn the_shipped_dials_are_the_contract() {
    let s = WetSettings::default();
    assert_eq!(s.drip_rate, 0.35);
    assert_eq!(s.spread_rate, 0.08);
    assert_eq!(s.dry_ticks, 1800);
    assert_eq!(s.absorbency, 0.15);
    assert_eq!(s.max_canvas_updates_per_tick, 4);
    assert_eq!(s.humidity, 0.4);
}

/// **A speck lays the blood it is, and no more.** A canvas that cannot show a 3 mm stain should show the tint
/// 3 mm of blood is in a 12 mm texel, not a 12 mm dot: the bytes a speck deposits sum to its area, to within the
/// rounding of each texel, whatever its size, its axis ratio and where in the texel it sits. The stamp it replaces under
/// four texels is nothing like that: a stain of a sixteenth of a texel is drawn as a whole one.
#[test]
fn a_speck_lays_its_area_whatever_its_size_and_where_it_sits() {
    use std::f32::consts::PI;
    for (semi, direction) in [([0.25f32, 0.25f32], Vec2::X), ([0.5, 0.25], Vec2::new(1.0, 1.0)), ([1.0, 1.0], Vec2::X), ([1.5, 0.6], Vec2::new(0.3, -1.0)), ([2.0, 1.4], Vec2::Y)] {
        let want = PI * semi[0] * semi[1] * 255.0;
        for (sx, sy) in [(0.5, 0.5), (0.33, 0.71), (0.0, 0.0), (0.98, 0.12)] {
            let (_images, mut canvas) = scratch(64);
            // Centres on a texel's corner and on a texel's middle among them: the total must not care.
            canvas.paint_speck(Vec2::new((20.0 + sx) / 64.0, (20.0 + sy) / 64.0), semi, direction, 1);
            let got = coverage(&canvas) as f32;
            assert!((got - want).abs() <= 0.1 * want + 14.0, "a speck of {semi:?} at ({sx}, {sy}) laid {got} of {want}");
        }
    }
}

/// A speck is round about where it is centred, and is placed to a fraction of a texel: centred on the corner of four
/// texels it lays the same on each of them, centred in one it lays the same on either side and above and below, and
/// the texel it is centred in holds the most. Moving it half a texel moves coverage across and not the total.
#[test]
fn a_speck_is_symmetric_about_where_it_is_centred() {
    let size = 64u32;
    let (_images, mut corner) = scratch(size);
    corner.paint_speck(Vec2::new(20.0 / 64.0, 20.0 / 64.0), [0.5, 0.5], Vec2::X, 1);
    let four = [(19, 19), (20, 19), (19, 20), (20, 20)].map(|(x, y)| corner.amount_at(x, y));
    assert!(four[0] > 0 && four.iter().max().unwrap() - four.iter().min().unwrap() <= 1, "a speck on a corner was not laid evenly on its four texels: {four:?}");
    let (_images, mut middle) = scratch(size);
    middle.paint_speck(Vec2::new(20.5 / 64.0, 20.5 / 64.0), [0.5, 0.5], Vec2::X, 1);
    let (left, right) = (middle.amount_at(19, 20), middle.amount_at(21, 20));
    let (above, below) = (middle.amount_at(20, 19), middle.amount_at(20, 21));
    assert!(left > 0 && left.abs_diff(right) <= 1 && above.abs_diff(below) <= 1 && left.abs_diff(above) <= 1, "a speck centred in a texel was not round about it: {left} {right} {above} {below}");
    assert!(middle.amount_at(20, 20) > left, "the texel a speck is centred in does not hold the most");
    assert!((coverage(&corner) as i32 - coverage(&middle) as i32).abs() <= 14, "moving a speck half a texel changed what it laid");
}

/// A speck is a pure function of its arguments: two canvases painted alike hold the same bytes, and one painted
/// a twentieth of a texel off does not.
#[test]
fn a_speck_is_reproducible_and_is_placed_finely() {
    let paint = |dx: f32| {
        let (_images, mut canvas) = scratch(64);
        for i in 0..12 {
            let f = i as f32;
            canvas.paint_speck(Vec2::new((10.0 + 3.7 * f + dx) / 64.0, (12.0 + 2.9 * f) / 64.0), [0.3 + 0.1 * f, 0.2 + 0.05 * f], Vec2::new(f.cos(), f.sin()), 5);
        }
        canvas.digest()
    };
    assert_eq!(paint(0.0), paint(0.0), "a speck depends on something other than its arguments");
    assert_ne!(paint(0.0), paint(0.05), "a speck is placed on a whole texel");
}

/// Nonsense is refused and writes nothing, a speck at the canvas's edge is clipped and never wrapped to the far
/// side, and one wider than the largest a speck draws is clamped rather than drawn at its size.
#[test]
fn a_speck_refuses_nonsense_and_is_clipped_at_the_edge() {
    let (_images, mut canvas) = scratch(32);
    let centre = Vec2::new(0.5, 0.5);
    for (uv, semi, direction) in [
        (Vec2::new(f32::NAN, 0.5), [1.0, 1.0], Vec2::X),
        (centre, [f32::NAN, 1.0], Vec2::X),
        (centre, [1.0, f32::INFINITY], Vec2::X),
        (centre, [0.0, 0.5], Vec2::X),
        (centre, [1.0, -0.5], Vec2::X),
        (centre, [1.0, 1.0], Vec2::new(f32::NAN, 0.0)),
    ] {
        canvas.paint_speck(uv, semi, direction, 1);
    }
    assert_eq!(coverage(&canvas), 0, "a refused speck wrote");
    assert!(!canvas.is_dirty(), "a refused speck dirtied the canvas");
    // On the corner: a quarter of it is on the canvas, and none wraps to the far corner.
    canvas.paint_speck(Vec2::ZERO, [1.5, 1.5], Vec2::X, 1);
    let got = coverage(&canvas) as f32;
    let quarter = std::f32::consts::PI * 2.25 * 255.0 / 4.0;
    assert!((got - quarter).abs() <= 0.15 * quarter, "a speck on the corner laid {got}, a quarter of it is {quarter}");
    assert_eq!(canvas.amount_at(31, 31) + canvas.amount_at(31, 0) + canvas.amount_at(0, 31), 0, "a speck wrapped to the far side");
    // Far past the largest a speck draws: clamped to it, not drawn at a hundred texels.
    let (_images, mut huge) = scratch(64);
    huge.paint_speck(centre, [100.0, 100.0], Vec2::X, 1);
    let max = std::f32::consts::PI * bevy_carnage::wetmap::SPECK_MAX_SEMI.powi(2) * 255.0;
    assert!(coverage(&huge) as f32 <= 1.1 * max + 14.0, "a speck of a hundred texels laid {} (the largest is {max})", coverage(&huge));
}

/// **What a reader draws.** The blood of a canvas as the flesh shader and the detailed images read it: each texel's
/// byte over 255 through the cubic B-spline, and drawn where that crosses `DETAIL_EDGE`. Sampled every eighth of a texel
/// over the `side`-texel square from `(x0, y0)`; returns the drawn area in texel squares, its centroid, and its
/// *roundness*, the spread of its radius about the centroid (largest minus smallest of 64 rays, over their mean).
struct Drawn {
    area: f32,
    centroid: Vec2,
    roundness: f32,
    /// The second moments `(xx, yy, xy)` about the centroid, texel squares.
    moments: (f32, f32, f32),
}

fn drawn(canvas: &WetCanvas, x0: i32, y0: i32, side: i32) -> Drawn {
    fn weight(x: f32) -> f32 {
        let x = x.abs();
        if x < 1.0 {
            (4.0 - 6.0 * x * x + 3.0 * x * x * x) / 6.0
        } else if x < 2.0 {
            let t = 2.0 - x;
            t * t * t / 6.0
        } else {
            0.0
        }
    }
    const K: i32 = 8;
    let field = |px: f32, py: f32| {
        let (ix, iy) = (px.floor() as i32, py.floor() as i32);
        let mut sum = 0.0;
        for j in iy - 2..=iy + 2 {
            for i in ix - 2..=ix + 2 {
                if i < 0 || j < 0 {
                    continue;
                }
                let a = canvas.amount_at(i as u32, j as u32) as f32 / 255.0;
                sum += a * weight(px - (i as f32 + 0.5)) * weight(py - (j as f32 + 0.5));
            }
        }
        sum
    };
    let n = side * K;
    let mut mask = vec![false; (n * n) as usize];
    let (mut count, mut sx, mut sy) = (0u32, 0.0f32, 0.0f32);
    for j in 0..n {
        for i in 0..n {
            let (px, py) = (x0 as f32 + (i as f32 + 0.5) / K as f32, y0 as f32 + (j as f32 + 0.5) / K as f32);
            if field(px, py) > bevy_carnage::wetmap::DETAIL_EDGE {
                mask[(j * n + i) as usize] = true;
                count += 1;
                sx += px;
                sy += py;
            }
        }
    }
    if count == 0 {
        return Drawn { area: 0.0, centroid: Vec2::ZERO, roundness: 0.0, moments: (0.0, 0.0, 0.0) };
    }
    let c = Vec2::new(sx / count as f32, sy / count as f32);
    let (mut mxx, mut myy, mut mxy) = (0.0f32, 0.0f32, 0.0f32);
    for j in 0..n {
        for i in 0..n {
            if mask[(j * n + i) as usize] {
                let (dx, dy) = (x0 as f32 + (i as f32 + 0.5) / K as f32 - c.x, y0 as f32 + (j as f32 + 0.5) / K as f32 - c.y);
                mxx += dx * dx;
                myy += dy * dy;
                mxy += dx * dy;
            }
        }
    }
    let k = count as f32;
    let (mut lo, mut hi, mut mean) = (f32::MAX, 0.0f32, 0.0f32);
    for ray in 0..64 {
        let t = ray as f32 / 64.0 * std::f32::consts::TAU;
        let (dx, dy) = (t.cos(), t.sin());
        let mut r = 0.0;
        let mut step = 0.0;
        while step < side as f32 / 2.0 {
            let (px, py) = (c.x + dx * step - x0 as f32, c.y + dy * step - y0 as f32);
            let (i, j) = ((px * K as f32) as i32, (py * K as f32) as i32);
            if i >= 0 && j >= 0 && i < n && j < n && mask[(j * n + i) as usize] {
                r = step;
            }
            step += 1.0 / 16.0;
        }
        lo = lo.min(r);
        hi = hi.max(r);
        mean += r / 64.0;
    }
    Drawn { area: count as f32 / (K * K) as f32, centroid: c, roundness: (hi - lo) / mean, moments: (mxx / k, myy / k, mxy / k) }
}

/// **A cone is drawn at the size it was given, round, where it was put.** A reader that draws at `DETAIL_EDGE` — the
/// flesh shader's edge, and the detailed images' — must draw the ellipse a cone was painted for: its area within 5 %
/// and its centre within a tenth of a texel, and no more than a fifth of its radius out of round, at every offset of the
/// texel from a radius of a texel and a half to the largest. At a texel (two across) the canvas holds four to nine texels of the
/// drop and the spline's smoothing of a rim two texels round is what the drawn area reads: within 15 % and a quarter out of round.
/// The speck this replaces for the preset drew a drop of two texels across at four and a half, on a square.
#[test]
fn a_cone_is_drawn_at_the_size_it_was_given_and_round() {
    use std::f32::consts::PI;
    for r in [1.0f32, 1.5, 2.0, 2.5, 3.0, 4.0] {
        let (area_tol, round_tol) = if r < 1.5 { (0.15, 0.25) } else { (0.05, 0.2) };
        for (sx, sy) in [(0.5, 0.5), (0.0, 0.0), (0.33, 0.71), (0.9, 0.2), (0.5, 0.0)] {
            let (_images, mut canvas) = scratch(64);
            canvas.paint_cone(Vec2::new((30.0 + sx) / 64.0, (30.0 + sy) / 64.0), [r, r], Vec2::X, 1);
            let d = drawn(&canvas, 20, 20, 20);
            let want = PI * r * r;
            assert!((d.area - want).abs() <= area_tol * want, "a cone of radius {r} at ({sx}, {sy}) was drawn {} texel squares, not {want}", d.area);
            assert!(d.centroid.distance(Vec2::new(30.0 + sx, 30.0 + sy)) <= 0.1, "a cone of radius {r} at ({sx}, {sy}) was drawn centred on {:?}", d.centroid);
            assert!(d.roundness <= round_tol, "a cone of radius {r} at ({sx}, {sy}) was drawn {} out of round", d.roundness);
        }
    }
}

/// A cone follows its ellipse: the long axis along the direction it was given, and the area of an ellipse, not a circle
/// of either radius.
#[test]
fn a_cone_is_an_ellipse_along_its_direction() {
    use std::f32::consts::PI;
    for (direction, angle) in [(Vec2::X, 0.0f32), (Vec2::new(1.0, 1.0), PI / 4.0), (Vec2::new(0.0, 1.0), PI / 2.0), (Vec2::new(-1.0, 0.5), (0.5f32).atan2(-1.0))] {
        let (_images, mut canvas) = scratch(64);
        canvas.paint_cone(Vec2::new(30.3 / 64.0, 30.6 / 64.0), [3.0, 1.5], direction, 1);
        let d = drawn(&canvas, 20, 20, 20);
        let want = PI * 3.0 * 1.5;
        assert!((d.area - want).abs() <= 0.1 * want, "an ellipse of 3 by 1.5 along {direction:?} was drawn {} texel squares, not {want}", d.area);
        // The principal axis of the drawn blood, from its second moments, and the ratio of its two spreads.
        let (mxx, myy, mxy) = d.moments;
        let axis = 0.5 * (2.0 * mxy).atan2(mxx - myy);
        let off = ((axis - angle + PI / 2.0).rem_euclid(PI) - PI / 2.0).abs();
        assert!(off <= 0.09, "the drawn long axis is {axis} rad, not {angle} ({direction:?})");
        let half = ((mxx - myy) * 0.5).hypot(mxy);
        let (big, small) = (0.5 * (mxx + myy) + half, 0.5 * (mxx + myy) - half);
        let ratio = (big / small).sqrt();
        assert!((ratio - 2.0).abs() <= 0.3, "the drawn axes are in the ratio {ratio}, not 2 ({direction:?})");
    }
}

/// **A cone is a drop, not a film: it does not creep.** Blood moves on a canvas while it is wet: the spread pass rounds
/// `spread_rate / 4 · difference` to a byte, and at the shipped rate a difference of 25 moves one. A cone's neighbouring
/// texels differ by 23 at most, so nothing leaves it. With the substrate soaking nothing (`absorbency` 0) six hundred ticks,
/// under both gravities, leave every byte of a cone as it was painted, up to the largest that holds no more than the drip pass
/// leaves in a texel (`drip_rate`, 0.35 of 255: a drop thicker than that sheds the excess downhill, which is a drop beginning
/// to run). With the shipped substrate the drying thins a cone down to `STAIN_FLOOR` and no lower, which keeps its footprint
/// (`a_dried_drop_keeps_its_footprint`); it never grows or moves it. (The speck this replaces went
/// from 4.5 texel squares of blood on a bump of 255 to a plateau of 77 and a diamond, over the first hundred ticks.)
#[test]
fn a_cone_does_not_creep_while_it_dries() {
    let tick_for = |canvas: &mut WetCanvas, s: &WetSettings, ticks: u32| {
        for tick in 1..=ticks {
            let gravity = if tick < ticks / 2 { Vec2::new(0.0, 1.0) } else { Vec2::new(1.0, 0.0) };
            canvas.tick(tick, gravity, s);
        }
    };
    let bytes = |canvas: &WetCanvas| (0..64u32).flat_map(|y| (0..64u32).map(move |x| (x, y))).map(|(x, y)| canvas.amount_at(x, y)).collect::<Vec<u8>>();
    let rinsed = WetSettings { absorbency: 0.0, ..WetSettings::default() };
    for r in [1.0f32, 1.5, 2.0, 3.0] {
        let (_images, mut canvas) = scratch(64);
        canvas.paint_cone(Vec2::new(30.4 / 64.0, 30.6 / 64.0), [r, r], Vec2::X, 0);
        let painted = bytes(&canvas);
        assert!(painted.iter().any(|&b| b > 0), "the premise: a cone of radius {r} painted nothing");
        tick_for(&mut canvas, &rinsed, 600);
        assert_eq!(painted, bytes(&canvas), "a cone of radius {r} crept: blood moved on a canvas whose substrate takes none");
    }
    for r in [1.5f32, 2.0, 3.0, 4.0] {
        let (_images, mut canvas) = scratch(64);
        canvas.paint_cone(Vec2::new(30.4 / 64.0, 30.6 / 64.0), [r, r], Vec2::X, 0);
        let before = drawn(&canvas, 16, 16, 28);
        tick_for(&mut canvas, &WetSettings::default(), 600);
        let after = drawn(&canvas, 16, 16, 28);
        assert!(after.area <= 1.02 * before.area, "a cone of radius {r} grew from {} to {} texel squares while it dried", before.area, after.area);
        assert!(after.area >= 0.3 * before.area, "a cone of radius {r} was soaked from {} to {} texel squares in a third of its drying", before.area, after.area);
        assert!(after.centroid.distance(before.centroid) <= 0.15, "a cone of radius {r} moved from {:?} to {:?}", before.centroid, after.centroid);
    }
}

/// **A drying stain loses height and keeps its footprint.** Smith, Nicloux & Brutin (2020, `doi:10.1038/s41598-020-65465-4`): "the
/// pool is pinned to the surface and the liquid phase evaporates at the surface. The volume loss corresponds then to a decrease
/// in height." The substrate's soak took 38 bytes from every texel over the drying, which is the whole of a spatter drop's film:
/// on 0.8.2 a cone of radius 1.5 texels was gone once dry, one of radius 2 a tenth of what it was and one of 3 a fifth, and a
/// wall's spatter was half its area a drying later. Now the substrate takes only the film above a stain's own depth, so a drop of
/// any size is drawn at the size it was painted once it has dried, where it was painted.
#[test]
fn a_dried_drop_keeps_its_footprint() {
    let s = WetSettings::default();
    for r in [0.8f32, 1.0, 1.5, 2.0, 3.0, 4.0] {
        let (_images, mut canvas) = scratch(64);
        canvas.paint_cone(Vec2::new(30.4 / 64.0, 30.6 / 64.0), [r, r], Vec2::X, 0);
        let before = drawn(&canvas, 16, 16, 28);
        assert!(before.area > 0.0, "the premise: a cone of radius {r} painted nothing that is drawn");
        for tick in 1..=s.dry_ticks + 60 {
            canvas.tick(tick, Vec2::new(0.0, 1.0), &s);
        }
        let after = drawn(&canvas, 16, 16, 28);
        assert!(after.area >= 0.9 * before.area, "a cone of radius {r} dried from {} to {} texel squares: the substrate took the stain", before.area, after.area);
        assert!(after.area <= 1.02 * before.area, "a cone of radius {r} grew from {} to {} texel squares while it dried", before.area, after.area);
        assert!(after.centroid.distance(before.centroid) <= 0.15, "a cone of radius {r} moved from {:?} to {:?} while it dried", before.centroid, after.centroid);
    }
}

/// **The soak is bounded on both sides**: it takes a pool's film and stops at a stain's depth, and it takes nothing from a
/// stain already under it. With drip and spread off, so the soak is the only thing moving blood, a pool of 255 ends at
/// `255 − 38` (the schedule's whole share: untouched by the floor), no texel loses more than the schedule gives or goes under
/// `min(what it had, STAIN_FLOOR)`, none gains, and a faint cone whose peak is under the floor is **byte for byte** as painted.
#[test]
fn the_substrate_takes_a_pools_film_and_nothing_under_a_stains_depth() {
    use bevy_carnage::wetmap::STAIN_FLOOR;
    let s = WetSettings { spread_rate: 0.0, drip_rate: 1.0, ..WetSettings::default() };
    let bytes = |c: &WetCanvas| (0..64u32).flat_map(|y| (0..64u32).map(move |x| (x, y))).map(|(x, y)| c.amount_at(x, y)).collect::<Vec<u8>>();
    let dry = |c: &mut WetCanvas| {
        for tick in 1..=s.dry_ticks + 60 {
            c.tick(tick, Vec2::new(0.0, 1.0), &s);
        }
    };

    let (_images, mut pool) = scratch(64);
    pool.paint_uv(Vec2::new(0.5, 0.5), &blob(0.4), 0);
    let fresh = bytes(&pool);
    assert_eq!(fresh.iter().max(), Some(&255), "the premise: the pool has full texels");
    dry(&mut pool);
    let dried = bytes(&pool);
    assert_eq!(dried.iter().max(), Some(&(255 - 38)), "a pool's full texels lose the schedule's whole 15 %");
    for (i, (f, d)) in fresh.iter().zip(&dried).enumerate() {
        assert!(d <= f, "texel {i} gained blood while the substrate soaked: {f} to {d}");
        assert!(f - d <= 38, "texel {i} lost {} of {f}: more than the schedule gives", f - d);
        assert!(*d >= (*f).min(STAIN_FLOOR), "texel {i} went from {f} to {d}, under the stain's own depth {STAIN_FLOOR}");
        assert_eq!(*f > 0, *d > 0, "texel {i} was {f} and is {d}: the footprint changed");
    }

    let (_images, mut faint) = scratch(64);
    faint.paint_cone(Vec2::new(30.4 / 64.0, 30.6 / 64.0), [0.8, 0.8], Vec2::X, 0);
    let painted = bytes(&faint);
    assert!(painted.iter().any(|&b| b > 0) && painted.iter().all(|&b| b <= STAIN_FLOOR), "the premise: a cone of 0.8 peaks under the floor");
    dry(&mut faint);
    assert_eq!(painted, bytes(&faint), "the substrate soaked a film that was already under a stain's depth");
}

/// A cone is a pure function of its arguments, and is placed to a fraction of a texel: two canvases painted alike hold
/// the same bytes, and one painted a twentieth of a texel off does not.
#[test]
fn a_cone_is_reproducible_and_is_placed_finely() {
    let paint = |dx: f32| {
        let (_images, mut canvas) = scratch(64);
        for i in 0..12 {
            let f = i as f32;
            canvas.paint_cone(Vec2::new((10.0 + 3.7 * f + dx) / 64.0, (12.0 + 2.9 * f) / 64.0), [0.8 + 0.2 * f, 0.7 + 0.1 * f], Vec2::new(f.cos(), f.sin()), 5);
        }
        canvas.digest()
    };
    assert_eq!(paint(0.0), paint(0.0), "a cone depends on something other than its arguments");
    assert_ne!(paint(0.0), paint(0.05), "a cone is placed on a whole texel");
}

/// **A drop too small to draw is drawn at the smallest that can be**, not left out: a stain of a hundredth of a texel
/// is a drop of `CONE_MIN_SEMI`, so a floor that took a hundred of them shows a hundred.
#[test]
fn a_cone_too_small_to_draw_is_drawn_at_the_least() {
    use bevy_carnage::wetmap::CONE_MIN_SEMI;
    for r in [0.01f32, 0.2, 0.5] {
        let (_images, mut canvas) = scratch(64);
        canvas.paint_cone(Vec2::new(30.5 / 64.0, 30.5 / 64.0), [r, r], Vec2::X, 1);
        let d = drawn(&canvas, 20, 20, 20);
        let least = std::f32::consts::PI * CONE_MIN_SEMI * CONE_MIN_SEMI;
        assert!(d.area >= 0.7 * least && d.area <= 1.3 * least, "a cone of radius {r} was drawn {} texel squares; the least is {least}", d.area);
    }
}

/// Nonsense is refused and writes nothing, a cone at the canvas's edge is clipped and never wrapped to the far side, and one
/// wider than the largest a cone draws is clamped to it.
#[test]
fn a_cone_refuses_nonsense_and_is_clipped_at_the_edge() {
    let (_images, mut canvas) = scratch(32);
    let centre = Vec2::new(0.5, 0.5);
    for (uv, semi, direction) in [
        (Vec2::new(f32::NAN, 0.5), [1.0, 1.0], Vec2::X),
        (centre, [f32::NAN, 1.0], Vec2::X),
        (centre, [1.0, f32::INFINITY], Vec2::X),
        (centre, [0.0, 0.5], Vec2::X),
        (centre, [1.0, -0.5], Vec2::X),
        (centre, [1.0, 1.0], Vec2::new(f32::NAN, 0.0)),
    ] {
        canvas.paint_cone(uv, semi, direction, 1);
    }
    assert_eq!(coverage(&canvas), 0, "a refused cone wrote");
    assert!(!canvas.is_dirty(), "a refused cone dirtied the canvas");
    // On the corner: a quarter of it is on the canvas, and none wraps to the far corner.
    canvas.paint_cone(Vec2::ZERO, [2.0, 2.0], Vec2::X, 1);
    assert!(canvas.amount_at(0, 0) > 0, "a cone on the corner left it bare");
    assert_eq!(canvas.amount_at(31, 31) + canvas.amount_at(31, 0) + canvas.amount_at(0, 31), 0, "a cone wrapped to the far side");
    // Far past the largest a cone draws: clamped to it, not drawn at a hundred texels.
    let (_images, mut huge) = scratch(64);
    huge.paint_cone(centre, [100.0, 100.0], Vec2::X, 1);
    let d = drawn(&huge, 12, 12, 40);
    let max = std::f32::consts::PI * bevy_carnage::wetmap::CONE_MAX_SEMI.powi(2);
    assert!(d.area <= 1.1 * max, "a cone of a hundred texels was drawn {} texel squares (the largest is {max})", d.area);
}

/// A smear down the middle of a canvas, left to right at `y`, `width` wide, with the grain of a finger on a 256 canvas (3 texels).
fn stroke(y: f32, x0: f32, x1: f32, width: f32, seed: u32, bead: bool) -> Smear {
    Smear { from: Vec2::new(x0, y), to: Vec2::new(x1, y), width, grain: 3.0 / 256.0, seed, bead }
}

/// **A smear moves blood and makes none.** Over strokes of every direction and length across a canvas that holds a pool, a half-dry pool and bare floor,
/// `sum(amount) + carry` is the same after each stroke as before it: a byte lifted is carried and a byte deposited is spent.
#[test]
fn a_smear_conserves_every_byte() {
    let s = WetSettings::default();
    let (_images, mut canvas) = scratch(128);
    canvas.paint_uv(Vec2::new(0.35, 0.4), &blob(0.2), 0);
    canvas.paint_uv(Vec2::new(0.75, 0.7), &blob(0.12), 0);
    let mut carry = 90_000u32;
    let strokes = [
        (0.10, 0.40, 0.90, 0.40),
        (0.50, 0.10, 0.45, 0.85),
        (0.90, 0.90, 0.20, 0.55),
        (0.30, 0.70, 0.30, 0.30),
        (0.05, 0.05, 0.95, 0.95),
        (0.60, 0.20, 0.62, 0.21),
        (0.80, 0.30, 0.10, 0.31),
    ];
    for (k, &(ax, ay, bx, by)) in strokes.iter().enumerate() {
        let before = u64::from(coverage(&canvas)) + u64::from(carry);
        let smear = Smear { from: Vec2::new(ax, ay), to: Vec2::new(bx, by), width: 0.07, grain: 3.0 / 128.0, seed: 5 + k as u32, bead: k % 2 == 0 };
        carry = canvas.smear_uv(&smear, carry, 120_000, k as u32, &s);
        let after = u64::from(coverage(&canvas)) + u64::from(carry);
        assert_eq!(before, after, "stroke {k} made or lost {} bytes of blood", after as i64 - before as i64);
    }
}

/// **Nothing to move, nothing moves.** A contact with no blood on a bare canvas changes nothing and returns 0; and **dry paint does not move**: a contact with no blood across a dried pool lifts none
/// of it, so the canvas, its digest and its coverage are what they were.
#[test]
fn a_clean_contact_changes_nothing_and_dry_blood_is_not_lifted() {
    let (_images, mut bare) = scratch(128);
    let blank = bare.digest();
    let s = WetSettings { dry_ticks: 1, ..Default::default() };
    assert_eq!(bare.smear_uv(&stroke(0.5, 0.1, 0.9, 0.1, 3, true), 0, 6000, 0, &s), 0);
    assert_eq!(bare.digest(), blank, "a clean contact on a bare canvas painted something");
    assert!(!bare.is_dirty(), "and asked for an upload");

    let (_images, mut crust) = scratch(128);
    crust.paint_uv(Vec2::new(0.5, 0.5), &blob(0.2), 0);
    for tick in 0..4u32 {
        crust.tick(tick, Vec2::new(0.0, 1.0), &s);
    }
    let (settled, held) = (crust.digest(), coverage(&crust));
    assert!(held > 0, "nothing was left to be dry");
    assert_eq!(crust.smear_uv(&stroke(0.5, 0.1, 0.9, 0.1, 3, false), 0, 6000, 9, &s), 0);
    assert_eq!(crust.digest(), settled, "a clean contact lifted dry blood");
    assert_eq!(coverage(&crust), held);
}

/// **A smear is heavy at its head and fades along the path, and a contact with less blood lays a lighter mark.** A soaked contact slid across a bare canvas lays more blood in the first third of its
/// path than the second, and the second than the third; one with a fiftieth of the blood lays a mark a fraction as heavy, and keeps what it did not lay.
#[test]
fn a_smear_is_heavy_at_its_head_and_fades_along_the_path() {
    let s = WetSettings::default();
    let thirds = |canvas: &WetCanvas, x0: u32, x1: u32| -> [u32; 3] {
        let mut sums = [0u32; 3];
        for y in 0..canvas.size() {
            for x in x0..x1 {
                sums[(((x - x0) * 3) / (x1 - x0)) as usize] += u32::from(canvas.amount_at(x, y));
            }
        }
        sums
    };
    let (_images, mut canvas) = scratch(256);
    let left = canvas.smear_uv(&stroke(0.5, 0.1, 0.9, 0.06, 4, true), 300_000, 400_000, 0, &s);
    let [a, b, c] = thirds(&canvas, 26, 230);
    assert!(a > b && b > c, "the thirds of the smear are {a}, {b}, {c}: not a fade from the head");
    assert!(c * 10 < a * 6, "the tail ({c}) is more than 0.6 of the head ({a})");
    assert_eq!(u64::from(coverage(&canvas)) + u64::from(left), 300_000, "the blood laid and the blood left are not the blood there was");

    let window = |c: &WetCanvas, x0: u32, x1: u32| -> u32 { (x0..x1).map(|x| (0..256).map(|y| u32::from(c.amount_at(x, y))).sum::<u32>()).sum() };
    let (_images, mut thin) = scratch(256);
    let kept = thin.smear_uv(&stroke(0.5, 0.1, 0.9, 0.06, 4, true), 8_000, 400_000, 0, &s);
    assert_eq!(u64::from(coverage(&thin)) + u64::from(kept), 8_000, "a thin contact made or lost blood");
    let (head, soaked) = (window(&thin, 26, 82), window(&canvas, 26, 82));
    assert!(head * 5 < soaked, "a contact with a fiftieth of the blood laid {head} bytes in its first 56 columns; the soaked one laid {soaked}");
}

/// **A wipe drags a pool.** A contact with no blood drawn through a pool takes some of it (grooves in the pool) and lays it down beyond: blood appears past the pool's far edge where there was none.
#[test]
fn a_wipe_drags_a_pool_along_its_path() {
    let s = WetSettings::default();
    let (_images, mut canvas) = scratch(256);
    canvas.paint_uv(Vec2::new(0.3, 0.5), &blob(0.16), 0);
    let beyond = |c: &WetCanvas| -> u32 { (126..200).map(|x| (0..256).map(|y| u32::from(c.amount_at(x, y))).sum::<u32>()).sum() };
    let pool_before = coverage(&canvas);
    assert_eq!(beyond(&canvas), 0, "the premise: no blood between x = 126 and 200");
    let left = canvas.smear_uv(&stroke(0.5, 0.2, 0.9, 0.05, 6, false), 0, 60_000, 1, &s);
    assert!(beyond(&canvas) > 0, "a wipe through a pool carried none of it on past the pool's far edge");
    assert_eq!(u64::from(coverage(&canvas)) + u64::from(left), u64::from(pool_before), "a wipe made or lost blood");
}

/// **Streaks run along the motion, and a smear is reproducible.** The marks of a horizontal stroke change less from one texel to the next along it than across it, and the same stroke on two
/// canvases leaves the same digest; another seed leaves another.
#[test]
fn a_smears_streaks_run_along_the_motion_and_its_digest_is_reproducible() {
    let s = WetSettings::default();
    let run = |seed: u32| {
        let (_images, mut canvas) = scratch(256);
        canvas.smear_uv(&stroke(0.5, 0.1, 0.9, 0.08, seed, true), 400_000, 400_000, 0, &s);
        canvas
    };
    let (a, again, other) = (run(8), run(8), run(9));
    assert_eq!(a.digest(), again.digest(), "one stroke, two canvases, two digests");
    assert_ne!(a.digest(), other.digest(), "another seed laid the same marks");
    let (mut along, mut across, mut n) = (0u64, 0u64, 0u64);
    for y in 118..138 {
        for x in 60..200 {
            let here = i32::from(a.amount_at(x, y));
            along += (here - i32::from(a.amount_at(x + 1, y))).unsigned_abs() as u64;
            across += (here - i32::from(a.amount_at(x, y + 1))).unsigned_abs() as u64;
            n += 1;
        }
    }
    assert!(n > 0 && across > 0, "no marks to measure");
    assert!(along * 10 < across * 6, "the marks change {along} along the motion and {across} across it: they are not lines");
}
