//! **Headless recorder for a skinned character** — the preset on a body that is never in its bind pose.
//!
//! ```sh
//! cargo run --release -p bevy_carnage --example capture_skinned -- --out frames-skinned
//! ```
//!
//! `--eye x,y,z` renders from another camera position, aimed at the chest or at `--at x,y,z`. Neither
//! changes a digest. `--floor N` dresses the floor at `N` texels a side instead of the dial's 256 (`GoreDials::floor_canvas`):
//! a three-metre slab at 512 is 6 mm a texel, where a drop of blood a few millimetres across is a stain and not a tint.
//! It changes the digest, as the canvas it folds is another size. `--dry N` makes blood dry in `N` ticks instead of 1 800 (30 s at 60 Hz)
//! (`WetSettings::dry_ticks`), so the clip shows what a wall and a floor look like once the blood on them has dried; it changes the digest.
//! `--break` makes the body breakable and takes a heavy shot to the chest at frame 200: it breaks where
//! its skeleton is, mid-walk with the arm raised, and its pieces fall to the floor. `--shirt` dresses
//! the body in a sleeveless shirt: the two chest shots hole it and the wound is seen through the hole,
//! with the blood the shirt soaks up around it. The holes are folded into the digest. `--through` puts
//! a wall behind the body and every shot goes through it: an exit wound opens on the far side and its
//! forward spatter lands on the wall and the floor behind, and runs down the wall. `--quiet` takes no
//! hits at all. `--reset-at N` sends a `GoreReset` for the body and every dressed surface on frame
//! `N`: once the canvases have uploaded, every frame after is the quiet run's, and so is `digest=`.
//! `--blows` adds a second blow to the left shin, lower down, at frame 60: two bruises on one limb,
//! both ageing. `--stroll` takes no hits: the body walks along a tiled floor instead of in place,
//! through a spill of blood painted on the floor ahead of it at frame 0, and every foot it puts down is
//! sent as a `GoreStep` — so it walks out of the spill leaving prints that fade, seen from above.
//! `--guard` stands the body on guard in front of the wall instead of walking, the right forearm held
//! across the chest, and takes one shot through it at frame 40: it goes in and out of the forearm and
//! on into the chest, and once the arm is lowered the wound it hid is seen, its forward spatter round it.
//! `--slow N` runs the clock at `1/N` speed — every frame a `60 N`-th of a second, so a fixed tick
//! every `N` frames — which is how the spray a hit throws, gone from a body-sized view within two
//! frames at full speed, is seen leaving it. It prints its own digest: the same ticks fall on other
//! frames.
//! `--smear` takes no hits and no body: a wall and a floor, a pool painted on the floor, and four contacts sent as `GoreSmear`s (`docs/research/smear.md`): at frame 30 a body, a third of a
//! metre wide and bleeding as it goes, is dragged out of the pool for a metre and a half; at 40 a hand soaked in blood is slid down the wall; at 130 a less soaked one, at a slant; at 200 a clean hand
//! is wiped sideways across the first one's streaks. The floor is dressed at 512 a side. It prints its own digest.
//! `--hose` stands the body left of a wall and turns it, and takes one arterial hit (`GoreHit::arterial`) at frame 20 to the back of the raised forearm, facing the wall (`docs/research/hose.md`): every heartbeat
//! the wound throws a bolus of large droplets that fly the parabola and land on the wall and the floor, each stain with a run below it and a stipple of fine drops beside it, the arc swept across the wall as the body
//! turns and shortening as the pressure falls. `--vein` is the same hit, venous: what 0.8.9 and earlier threw for an arterial one too. It prints its own digest.
//! `--burst` puts the body in a corner (a wall 0.45 m behind it, a wall at its left and a ceiling: `--wall-z Z` moves the first) with a camera that looks head to knees, and takes one heavy shot to the middle of
//! the head at frame 60 with a `GoreBurst` from its centre sent on the same frame (`docs/research/burst.md`): the cloud of stains lands over the next three quarters of a second. `--break` breaks the body with it
//! (the head leaves as the cloud lands) and `--no-cloud` sends the shot and not the burst, what 0.8.11 threw for it. It prints its own digest.
//!
//! Writes `frame0000.png …` and prints one digest line; two runs must print the same line. `digest=`
//! folds the wetmaps and the holed cloth as the clip ends; `flay=` and `dermis=` fold the craters and
//! the bruises and burns, which `digest=` cannot see, over every frame (a body that breaks is gone at the
//! end): `--blows` prints the default's `digest=` and its own `dermis=`.
//!
//! The subject is `common/mannequin.rs`: six skinned meshes on eleven joints, walking in place with
//! its left arm raised and waving. **Every shot is aimed at where the body is drawn at that frame**,
//! from the joint transforms the renderer last used, so a wound that opens somewhere else is the
//! preset casting at the bind pose — arms hanging at the sides — instead of at the pose.
//!
//! | frame | what |
//! |---|---|
//! | 0–29 | the body walks, dressed in the flesh material, the left arm up |
//! | 30 | a shot to the raised left forearm |
//! | 66 | a slash across the right thigh, mid-stride |
//! | 100 | a hot iron on the right forearm, which swings |
//! | 120, 150 | two shots to the chest, at one spot |
//! | 190 | a blow to the left shin |
//! | 192–359 | every wound rides the limb it is on, bleeding and dripping to the floor until it clots |

use std::time::Duration;

use bevy::mesh::skinning::SkinnedMeshInverseBindposes;
use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;
use bevy_carnage::preset::{ClothHoles, Gore, GoreBurst, GoreClock, GoreDials, GorePlugin, GoreReset, GoreSmear, GoreStep, HitKind, Sole};
use bevy_carnage::wetmap::{StainShape, WetCanvas, WetSettings};

mod common;
use common::digest;
use common::mannequin::{self, Bone, Mannequin, Part};
use common::preset_scene;
use common::recorder::Recorder;

const FRAMES: u32 = 360;

fn main() {
    let out = common::arg("--out").unwrap_or_else(|| "frames-skinned".to_string());
    // `--eye x,y,z` looks at the body from somewhere else, and `--at x,y,z` at another point than
    // the chest: the digest depends on neither, so they are how a render is checked from an angle,
    // or as close, as the committed clip does not use.
    let point = |name: &str| {
        common::arg(name)
            .map(|e| e.split(',').filter_map(|v| v.trim().parse::<f32>().ok()).collect::<Vec<_>>())
            .and_then(|v| (v.len() == 3).then(|| Vec3::new(v[0], v[1], v[2])))
    };
    let stroll = flag("--stroll");
    let smear = flag("--smear");
    let guard = flag("--guard");
    let hose = flag("--hose");
    let burst = flag("--burst");
    // A stroll is seen from above and to the side, the whole path in frame and the prints facing up at
    // it; a guard from in front and to the left, close on the chest and the forearm across it.
    let at = point("--at").unwrap_or(if stroll {
        Vec3::new(0.05, 0.5, 0.05)
    } else if smear {
        Vec3::new(0.25, 0.75, -1.0)
    } else if burst {
        Vec3::new(0.0, 1.45, -0.4)
    } else if hose {
        Vec3::new(0.0, 1.05, -0.6)
    } else if guard {
        Vec3::new(-0.03, 1.24, 0.06)
    } else {
        Vec3::new(0.12, 0.93, 0.1)
    });
    let camera = match point("--eye") {
        Some(eye) => Transform::from_translation(eye).looking_at(at, Vec3::Y),
        None if stroll => Transform::from_xyz(-2.9, 2.75, 0.15).looking_at(at, Vec3::Y),
        None if smear => Transform::from_xyz(-0.9, 1.9, 1.5).looking_at(at, Vec3::Y),
        None if burst => Transform::from_xyz(1.0, 1.6, 2.5).looking_at(at, Vec3::Y),
        None if hose => Transform::from_xyz(0.5, 1.6, 3.0).looking_at(at, Vec3::Y),
        None if guard => Transform::from_xyz(-0.5, 1.46, 1.5).looking_at(at, Vec3::Y),
        None => mannequin::view(),
    };
    let slow = common::arg("--slow").and_then(|n| n.parse::<u32>().ok()).unwrap_or(1).max(1);
    let Some(mut rec) = Recorder::new_with(800, 500, camera, &out, |app| {
        // **The clock is the frame, not the wall**, for the reason `capture_preset` gives: one sixtieth
        // per update and a 60 Hz fixed step is one tick per frame, and the animation reads the same
        // clock, so frame 62 of one run is frame 62 of the next. With `vfx`, every wound throws its
        // burst of blood in the air; the digest cannot see it.
        #[cfg(feature = "vfx")]
        app.add_plugins(bevy_carnage::CarnageVfxPlugin);
        if let Some(floor_canvas) = common::arg("--floor").and_then(|n| n.parse::<u32>().ok()).or(flag("--smear").then_some(512)) {
            app.insert_resource(GoreDials { floor_canvas, ..default() });
        }
        // **The fluid the blood is** (`--fluid`), and how long it takes to dry (`--dry`): both are the wetmap's own dials.
        let (dry_ticks, fluid) = (common::arg("--dry").and_then(|n| n.parse::<u32>().ok()), common::fluid());
        if dry_ticks.is_some() || fluid != bevy_carnage::Fluid::Blood {
            app.insert_resource(WetSettings { dry_ticks: dry_ticks.unwrap_or(WetSettings::default().dry_ticks), fluid, ..default() });
        }
        app.add_plugins(GorePlugin)
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / (60.0 * slow as f64))))
            .insert_resource(Time::<Fixed>::from_hz(60.0))
            .add_systems(Startup, setup);
        if flag("--stroll") {
            app.add_systems(Update, mannequin::stroll_animate);
        } else if flag("--guard") {
            app.add_systems(Update, mannequin::guard_animate);
        } else {
            app.add_systems(Update, mannequin::animate);
        }
        if flag("--hose") {
            app.add_systems(Update, hose_turn);
        }
    }) else {
        return;
    };
    {
        let world = rec.world();
        let mut cam = world.query_filtered::<Entity, With<Camera3d>>();
        if let Some(entity) = cam.iter(world).next() {
            let light = preset_scene::camera_light(&mut world.resource_mut::<Assets<Image>>());
            world.entity_mut(entity).insert(light);
        }
    }
    rec.warm_up(6);
    if hose {
        // The sheet the set hangs in the spray would stand between the wall and the lens; the hose throws at the wall.
        let world = rec.world();
        let sheets: Vec<Entity> = world.query_filtered::<Entity, With<preset_scene::Sheet>>().iter(world).collect();
        for sheet in sheets {
            world.despawn(sheet);
        }
    }
    let breaking = breaking();
    let through = flag("--through") || guard;
    let shirt = shirted();
    // `--quiet` takes no hits; `--reset-at N` heals the body and wipes every surface on frame `N`,
    // after which a run is the quiet one again, frame for frame.
    let quiet = flag("--quiet");
    let blows = flag("--blows");
    let reset_at = common::arg("--reset-at").and_then(|n| n.parse::<u32>().ok());
    let mut aim = bevy::ecs::system::SystemState::<mannequin::Aim>::new(rec.world());
    if stroll || smear {
        spill(rec.world());
    }
    // The contacts of `--smear`: a body, a soaked hand, a less soaked one, a clean one.
    let contacts: Vec<Entity> = (0..4).map(|_| rec.world().spawn_empty().id()).collect();
    // The last step a `GoreStep` was sent for.
    let mut planted: Option<i64> = None;
    let mut skin = digest::SkinTrace::default();
    for frame in 0..FRAMES {
        {
            let world = rec.world();
            if stroll {
                let t = world.resource::<Time>().elapsed_secs();
                let mut legs = world.query::<(Entity, &Part)>();
                let feet: Vec<(Entity, Part)> = legs.iter(world).map(|(e, p)| (e, *p)).collect();
                for (k, limb, at, heading) in mannequin::touchdowns(planted, t) {
                    planted = Some(k);
                    // Each foot is its leg's mesh, so the blood it carries is kept on the leg.
                    if let Some((foot, _)) = feet.iter().find(|(_, p)| p.0 == limb) {
                        world.write_message(GoreStep::new(*foot, at, heading).with_sole(Sole { length_m: 0.28, width_m: 0.09, tread: 3 }));
                    }
                }
            }
            if smear {
                smear_script(world, frame, &contacts);
            }
            let vein = flag("--vein");
            // `--no-cloud` sends the shot and not the burst: what 0.8.11 and earlier did for the same scene.
            let burst_at = if burst && frame == 60 && !flag("--no-cloud") { aim.get(world).ok().and_then(|aim| aim.on_bone(Bone::Head, -0.16)) } else { None };
            let hit = aim.get(world).ok().and_then(|aim| match frame {
                _ if quiet || stroll || smear => None,
                // **The burst**: a heavy shot to the middle of the head, which opens it (and, with `--break`, breaks the body), and the cloud of droplets
                // the head throws in every direction as it goes, a `GoreBurst` sent on the same frame.
                60 if burst => aim.shot(Bone::Head, -0.16, 0.07, if breaking { 60.0 } else { 45.0 }),
                _ if burst => None,
                // **The hose**: one arterial hit to the back of the raised forearm, which is waving, so what the
                // wound throws sweeps the wall behind it as the arm swings.
                20 if hose => aim.shot_from_behind(Bone::ArmLowL, 0.13, 0.025, 7.0).map(|h| if vein { h } else { h.arterial() }),
                _ if hose => None,
                40 if guard => aim.shot(Bone::ArmLowR, 0.16, 0.014, 7.0),
                _ if guard => None,
                60 if blows => aim.blow(Bone::LegLowL, 0.32, 0.04),
                30 => aim.shot(Bone::ArmLowL, 0.13, 0.03, 7.0),
                66 => aim.slash(Bone::LegUpR, 0.20, Vec3::new(0.8, -0.6, 0.0).normalize(), 0.07, 6.0),
                100 => aim.iron(Bone::ArmLowR, 0.10, 0.035),
                120 | 150 => aim.shot(Bone::Spine, -0.22, 0.035, 9.0),
                190 => aim.blow(Bone::LegLowL, 0.12, 0.04),
                200 if breaking => aim.shot(Bone::Spine, -0.22, 0.035, 60.0),
                200 if shirt => aim.slash(Bone::Spine, -0.05, Vec3::new(1.0, -0.35, 0.0).normalize(), 0.09, 6.0),
                _ => None,
            });
            if let Some(hit) = hit {
                let hit = if through && matches!(hit.kind, HitKind::Impact { .. }) { hit.through() } else { hit };
                world.write_message(hit);
            }
            if let Some(at) = burst_at {
                world.write_message(GoreBurst::new(at));
            }
            if reset_at == Some(frame) {
                let mut surfaces = world.query_filtered::<Entity, Or<(With<Mannequin>, With<Gore>)>>();
                let all: Vec<Entity> = surfaces.iter(world).collect();
                for entity in all {
                    world.write_message(GoreReset::new(entity));
                }
            }
        }
        rec.shoot();
        skin.sample(rec.world());
    }
    let world = rec.world();
    // `digest=` is the wetmaps as the clip ends, as it has always been, and the cloth that was holed;
    // `flay=` and `dermis=` are the craters and the bruises and burns it cannot see, folded every frame
    // because a breaking body is gone at the end (`digest::SkinTrace`). `digest::fold` sorts.
    let mut values: Vec<u64> = world.query::<&WetCanvas>().iter(world).map(|w| w.digest()).collect();
    // Only a cloth that was holed, so a clip in which nothing was holed keeps the digest it had.
    let mut holes = world.query::<&ClothHoles>();
    values.extend(holes.iter(world).filter(|h| h.is_holed()).map(ClothHoles::digest));
    let digest = digest::fold(values);
    let tick = world.resource::<GoreClock>().0;
    let frames = rec.finish();
    println!("skinned: frames={frames} ticks={tick} digest={digest:016x}{}", skin.fields());
}
/// Whether `--break` was given: the mannequin is made breakable and takes a hit to bone at frame 200.
fn breaking() -> bool {
    std::env::args().any(|a| a == "--break")
}

/// Whether `--shirt` was given: the mannequin wears a shirt.
fn shirted() -> bool {
    flag("--shirt")
}

/// **A spill on the floor in the stroll's path**: three overlapping stains, half a metre across in
/// all, painted wet on the slab's canvas where the body's second and third steps land.
fn spill(world: &mut World) {
    let tick = world.resource::<GoreClock>().0;
    let mut slab = world.query_filtered::<(Entity, &Mesh3d, &GlobalTransform), With<preset_scene::Slab>>();
    let Some((entity, handle, xf)) = slab.iter(world).next().map(|(e, m, x)| (e, m.0.clone(), *x)) else { return };
    let Some(mesh) = world.resource::<Assets<Mesh>>().get(&handle).cloned() else { return };
    let Some(mut wet) = world.get_mut::<WetCanvas>(entity) else { return };
    // The slab is 3 m to a UV, so a stain's axes are its metres over three.
    for (x, z, across, along, seed) in [(0.0, -0.86, 0.50, 0.44, 11), (0.13, -0.72, 0.26, 0.24, 12), (-0.12, -1.02, 0.30, 0.27, 13)] {
        let shape = StainShape { major: across / 3.0, minor: along / 3.0, spines: 0, satellites: 0, direction: [1.0, 0.0], seed };
        let _ = wet.paint_world(&mesh, &xf, Vec3::new(x, 1.0, z), Vec3::NEG_Y, &shape, tick);
    }
}

/// **The contacts of `--smear`**, one `GoreSmear` a tick for each stroke that is under way: `(carrier, first frame, last frame, from, to, the surface's normal, width, load at the first frame, load
/// every frame)`, loads in square metres of full coverage. A body bleeding as it is dragged brings blood on every tick; a hand brings what it was dipped in once.
fn smear_script(world: &mut World, frame: u32, contacts: &[Entity]) {
    let strokes = [
        (0, 30, 124, Vec3::new(-0.10, 0.0, -0.86), Vec3::new(1.30, 0.0, -0.86), Vec3::Y, 0.34, 0.0, 0.0004),
        (1, 40, 90, Vec3::new(-0.70, 1.55, -1.2), Vec3::new(-0.70, 0.95, -1.2), Vec3::Z, 0.09, 0.014, 0.0),
        (2, 130, 175, Vec3::new(0.0, 1.5, -1.2), Vec3::new(0.15, 0.8, -1.2), Vec3::Z, 0.09, 0.008, 0.0),
        (3, 200, 240, Vec3::new(-1.0, 1.2, -1.2), Vec3::new(-0.3, 1.2, -1.2), Vec3::Z, 0.09, 0.0, 0.0),
    ];
    for (who, first, last, from, to, normal, width, first_load, load) in strokes {
        if frame < first || frame >= last {
            continue;
        }
        let span = (last - first) as f32;
        let at = |f: u32| from + (to - from) * ((f - first) as f32 / span);
        let Some(carrier) = contacts.get(who) else { continue };
        let brought = load + if frame == first { first_load } else { 0.0 };
        world.write_message(GoreSmear::new(*carrier, at(frame), at(frame + 1), normal).width(width).loaded(brought));
    }
}

/// **The victim of `--hose` stands at the left of the wall and turns**, about her own vertical axis, three quarters of a radian either way over seven seconds: a wound that faces
/// the wall throws along a line the wall's own height spans, and the turn is what sweeps that line across it, to the open wall at her right. The preset follows the turn: the
/// wound's axis is read off the triangle it opened on.
fn hose_turn(time: Res<Time>, mut root: Query<&mut Transform, With<Mannequin>>) {
    let yaw = 0.75 * (time.elapsed_secs() * core::f32::consts::TAU / 7.0).sin();
    for mut transform in &mut root {
        transform.rotation = Quat::from_rotation_y(yaw);
        transform.translation = Vec3::new(-0.7, 0.0, 0.0);
    }
}

/// Whether a bare flag was given.
fn flag(name: &str) -> bool {
    std::env::args().any(|a| a == name)
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut bindposes: ResMut<Assets<SkinnedMeshInverseBindposes>>,
) {
    if flag("--smear") {
        // A wall and a floor, and nothing else in the frame: the contacts are the subject.
        preset_scene::spawn_set(&mut commands, &mut meshes, &mut materials);
        preset_scene::spawn_wall(&mut commands, &mut meshes, &mut materials);
        return;
    }
    let root = mannequin::spawn(&mut commands, &mut meshes, &mut materials, &mut images, &mut bindposes, shirted());
    if breaking() {
        commands.entity(root).insert(mannequin::breakable());
    }
    // **A stroll is on a tiled floor**, as a level's is: its UVs repeat the tiles, so it is dressed
    // `Gore::projected`, and at 512 a side, 5.9 mm a texel on the 3 m slab, where 256 ships — a print's
    // heel, waist and forefoot are a few centimetres each, and at 11.7 mm a texel the smoothing the
    // shader reads the canvas with rounds them into two drops. A showcase's choice, as the mannequin's
    // 256-texel skin is.
    if flag("--stroll") {
        let slab = preset_scene::tiled_slab(&mut meshes, &mut materials, &mut images);
        preset_scene::spawn_set_on(&mut commands, &mut meshes, &mut materials, slab);
    } else {
        preset_scene::spawn_set(&mut commands, &mut meshes, &mut materials);
    }
    // `--wall-z` stands the wall nearer the body (or further): a burst is a different pattern a hand's breadth from a wall than a metre from it, and `--burst` stands it at `-0.45`, close behind the head.
    let z = common::arg("--wall-z").and_then(|z| z.parse::<f32>().ok()).unwrap_or(if flag("--burst") { -0.45 } else { -1.2 });
    if flag("--through") || flag("--guard") || flag("--hose") || flag("--burst") {
        preset_scene::spawn_wall_at(&mut commands, &mut meshes, &mut materials, z);
    }
    if flag("--burst") {
        preset_scene::spawn_corner(&mut commands, &mut meshes, &mut materials, z);
    }
}
