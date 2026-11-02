//! **Headless recorder for the preset** — the two-line inclusion, scripted.
//!
//! ```sh
//! cargo run --release -p bevy_carnage --example capture_preset -- --out frames-preset
//! ```
//!
//! `--eye x,y,z` renders from another camera position, aimed at `--at x,y,z` (default the body's middle, low): neither changes a
//! digest. It is how a cut face is seen close, at the pieces' own scale, which the default view is too far to show.
//!
//! Writes `frame0000.png …` and prints one digest line; two runs must print the same line. `digest=`
//! folds the wetmaps as the clip ends; `flay=` and `dermis=` fold the craters and the bruises and burns,
//! which `digest=` cannot see, over every frame (the body breaks at frame 120 and is gone at the end).
//!
//! | frames | what |
//! |---|---|
//! | 0–29 | the body stands, dressed in the flesh material |
//! | 30 | a blow to the left arm (a bruise under the skin), a hot iron on the right thigh (a burn), a slash across the left thigh |
//! | 60–119 | a shot every 30 frames on the torso, at one spot: peel, blood on the body, the sheet and the slab, a bleed that runs and dries; the bruise ages in time-lapse |
//! | 120– | the third shot shows cortex and hands off — the body parts along its fracture modes, caps banded, and the pieces fall |
//!
//! Every visual is the preset's: `GorePlugin`, a `Gore` per surface, a `GoreHit` per shot.

use std::time::Duration;

use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;
use bevy_carnage::preset::{GoreClock, GorePlugin};
use bevy_carnage::{GorePolicy, GoreTier};
use bevy_carnage::wetmap::WetCanvas;

mod common;
use common::digest;
use common::preset_scene::{self, Part};
use common::recorder::Recorder;

const FRAMES: u32 = 360;

fn main() {
    let out = common::arg("--out").unwrap_or_else(|| "frames-preset".to_string());
    // On the windowed demos' sweep, near straight ahead: the body, the sheet beside it and the floor
    // spatter in front all in frame. From the old (1.1, 1.5, 2.9) the sheet stood in front of the
    // right arm and the scalded thigh for the whole clip (R1-13).
    let point = |flag: &str| {
        common::arg(flag)
            .map(|s| s.split(',').filter_map(|p| p.trim().parse::<f32>().ok()).collect::<Vec<_>>())
            .and_then(|v| (v.len() == 3).then(|| Vec3::new(v[0], v[1], v[2])))
    };
    let camera = match point("--eye") {
        Some(eye) => Transform::from_translation(eye).looking_at(point("--at").unwrap_or(Vec3::new(0.15, 0.55, 0.3)), Vec3::Y),
        None => common::preset_scene::view(-0.08),
    };
    // `--tier 0|1|2|3` (or `stylised|blood|gore|gross`) replays the same script under that
    // `GorePolicy`. Absent, the shipped policy is in force, which is what the committed clip shows.
    let tier = common::arg("--tier").map(|t| {
        preset_scene::tier_named(&t).unwrap_or_else(|| {
            eprintln!("capture_preset: unknown tier {t:?}; use 0-3 or stylised, blood, gore, gross");
            std::process::exit(2)
        })
    });
    let Some(mut rec) = Recorder::new_with(800, 500, camera, &out, |app| {
        // **The clock is the frame, not the wall.** The preset runs on `FixedUpdate`; with the default
        // real-time strategy the number of fixed ticks per recorded frame depends on how long the
        // screenshot took, and the digest would depend on the machine. One sixtieth per update, and
        // a 60 Hz fixed step, is exactly one tick per frame.
        if let Some(tier) = tier {
            // Before the plugin, which `init_resource`s the shipped one and so leaves this alone.
            app.insert_resource(GorePolicy::for_tier(tier));
            // The game's half of a tier that draws no blood: a spark where it would have been. With
            // `vfx` the crate throws that spark itself, as the substitute palette's burst.
            if tier == GoreTier::Stylised && cfg!(not(feature = "vfx")) {
                app.add_systems(Update, preset_scene::sparks);
            }
        }
        // **Blood in the air**: the burst `CarnageVfxPlugin` throws from every wound the preset
        // announces, in the substitute palette at a tier that draws no blood. The digest cannot see it.
        #[cfg(feature = "vfx")]
        app.add_plugins(bevy_carnage::CarnageVfxPlugin);
        app.add_plugins(GorePlugin)
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
            .insert_resource(Time::<Fixed>::from_hz(60.0))
            .add_systems(Startup, setup);
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

    // Frame 0 is captured on the update that first populates the scene, so it comes back blank —
    // and a GIF's still preview is its first frame. The flesh material's pipeline is specialised on
    // its first draw and the cache compiles it off-thread, so this needs more frames than the
    // family clip's two: measured, frame 0 was still blank at two and stable at six.
    rec.warm_up(6);
    let mut skin = digest::SkinTrace::default();
    // `--frames N` runs the clip N frames (default 360) and `--stride K` writes every Kth: the cut faces drain over the twelve seconds after
    // the body breaks at frame 120, which the default clip is too short to see. The digest folds every frame, written or not; at the defaults it is what it was.
    let frames: u32 = common::arg("--frames").and_then(|s| s.parse().ok()).unwrap_or(FRAMES);
    let stride: u32 = common::arg("--stride").and_then(|s| s.parse().ok()).filter(|k| *k > 0).unwrap_or(1);
    for frame in 0..frames {
        {
            let world = rec.world();
            let part = |world: &mut World, i: usize| {
                let mut parts = world.query::<(Entity, &Part)>();
                parts.iter(world).find(|(_, p)| p.0 == i).map(|(e, _)| e)
            };
            if frame == 30 {
                if let Some(arm) = part(world, 2) {
                    world.write_message(preset_scene::blow(arm));
                }
                if let Some(leg) = part(world, 5) {
                    world.write_message(preset_scene::scald(leg));
                }
                if let Some(leg) = part(world, 4) {
                    world.write_message(preset_scene::slash(leg));
                }
                if let Some(torso) = part(world, 0) {
                    world.write_message(preset_scene::graze(torso));
                }
            }
            if frame >= 60 && (frame - 60) % 30 == 0 {
                let n = (frame - 60) / 30;
                if let Some(torso) = part(world, 0) {
                    world.write_message(preset_scene::shot(torso, n));
                }
            }
        }
        // Every `stride`th frame is written; the others are only drawn, so a long clip costs the disk what a short one does.
        if frame % stride == 0 {
            rec.shoot();
        } else {
            rec.step();
        }
        skin.sample(rec.world());
    }
    let world = rec.world();
    // `digest=` is the wetmaps as the clip ends, as it has always been; `flay=` and `dermis=` are the
    // craters and the bruises and burns it cannot see, folded every frame because the body is gone at
    // the end (`digest::SkinTrace`). Each is sorted before folding, for the reason `digest::fold` gives.
    let digest = digest::fold(world.query::<&WetCanvas>().iter(world).map(|w| w.digest()).collect());
    let tick = world.resource::<GoreClock>().0;
    let frames = rec.finish();
    println!("preset: frames={frames} ticks={tick} digest={digest:016x}{}", skin.fields());
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    preset_scene::spawn(&mut commands, &mut meshes, &mut materials, &mut images);
}
