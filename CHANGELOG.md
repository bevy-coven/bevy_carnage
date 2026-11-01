# Changelog

All notable changes to this crate are recorded here. `0.6.0` and `0.7.0` (Bevy 0.20, `fracture_skeleton`) are
described in the comment block at the top of `Cargo.toml`; they and every entry below were published together as
`0.7.0`, the first release since `0.5.18`, which is a Bevy 0.19 crate.

## [Unreleased]

### Changed

- **A drop is drawn at the size it is: round, crisp, and not diffused into a diamond** (iteration 8, AG-147,
  `docs/research/drop-edge.md`). On a default `GorePlugin` floor and wall the stains of a wound's spray read as rotated squares
  and rounded diamonds with a pale middle, all about one size, and iteration 6 and 7 put it down to the floor's texel and the
  flesh shader's edge. It was the canvas's own data. A speck (`paint_speck`) laid the stain's area as a blur twice its size, which a
  shader draws where it crosses 4 % of full (`WET_EDGE`, `DETAIL_EDGE`): two to four texels across for a drop of one, on a grid a
  rounded square, and a stamp or a speck then *spread*: the pass rounds `spread_rate / 4 · difference` to a byte from a difference of
  25, so a wet stain of five texels went from a centre of 255 to a plateau of 77 and the L1 ball of a four-neighbourhood, with every
  edge at 45°. The new `WetCanvas::paint_cone` writes the stain's own edge instead, after Green's distance fields ("Improved
  alpha-tested magnification...", Valve, SIGGRAPH 2007): the canvas under a drop is `DETAIL_EDGE − CONE_SLOPE · d` for `d`
  the signed distance from the edge, which the cubic B-spline the shader reads through reproduces exactly, so the 4 % line is the
  ellipse. Measured: within 1.5 % of the stain's area from a radius of 1.5 texels at any offset in the texel; round to a fifth of
  the radius; **with the substrate soaking nothing, 600 ticks leave every byte as painted** (its slope is under the 25 the spread
  rounds to a byte). `paint_drop` uses it for every droplet a floor, a wall or a skin takes under **eight** texels across
  (`FINE_TEXELS` was four, and a wall and a skin never took a speck: every droplet there was a spined stamp); a cloth keeps its
  blob. A drop is drawn no smaller than `CONE_MIN_SEMI` (0.7 texel), and no spine is drawn on a stain under eight texels, where it
  was under two texels wide. **The look**: a shot into a room leaves a fine spatter of round drops of the sizes the model
  throws, where it left a field of rounded squares two to three times too big. **Cost**, `hit_tick_cost`, median of six runs
  alternated with 0.8.1: a heavy wound facing a wall 18.8 to 9.3 ms on its hit tick and 15.5 to 12.1 on every tick after, a wound
  facing a wall 4.9 to 2.7 and 5.7 to 3.7, a bullet on a floor 3.0 to 2.7; no case slower; the whole run retires 25 % fewer
  instructions (18.0 to 13.6 billion). **Every digest that has a drop under eight texels moves, on purpose**: `capture_skinned`'s
  default, `--break`, `--shirt`, `--through`, `--guard` and `--blows`, and `capture_preset`; `--stroll` and `--quiet`, `bench_carnage`
  (`0x4366a8b64352c85a`) and every golden are 0.8.1's. The drying still takes a thin film: a cone of a texel's radius soaks away
  by the time it is dry, as faint spatter always has (`absorbency`), and a larger one shrinks from the rim. A cone is not a
  mass: its bytes do not sum to the stain's area.

### Added

- `WetCanvas::paint_cone(uv, semi, direction, tick)` and `wetmap::{CONE_SLOPE, CONE_MIN_SEMI, CONE_MAX_SEMI}`: see Changed. Six
  contract tests in `tests/wetmap.rs` (`a_cone_is_drawn_at_the_size_it_was_given_and_round`, `a_cone_is_an_ellipse_along_its_direction`,
  `a_cone_does_not_creep_while_it_dries`, and the reproducibility, least-size and refusal ones). `paint_speck` and its
  tests are kept, for a caller who wants a stain's blood as a mass; the preset no longer calls it.

### Fixed

- The test helper `preset::tests::stains_on` counted a stain at 32 of 255, a level only a drop's blood *diffused over a plateau several
  times its own area* reached; a drop of its own size has an edge at the draw edge (`DETAIL_EDGE`, 10 of 255) and a middle a little
  above it, and was missed. It counts at the draw edge now; what `a_shot_leaves_a_fan_of_drops_on_the_floor_not_a_few_blobs` asks of
  the count (at least 15 stains, four times the mist-free count, none wider than 12 cm) is unchanged. Assumed decision (owner may
  override): the assertion's *thresholds* stand; its *measure* followed the model it measured.

## [0.8.1] - 2026-10-31

### Changed

- **A wounded scene costs the caller about two fifths less a tick, with every byte unchanged** (iteration 7, AG-146,
  `docs/research/wet-tick-cost.md`). A heavy wound facing a wall took 28 ms of one tick, and the blood it left kept the scene at
  26 ms a tick for as long as it was wet: the canvases' own passes were the larger share of every hit, and the stamp the rest.
  Five reductions, each exact. `bloodstain::stain::rasterise` tabulates, per angular bucket, which spines can reach it and an
  upper bound on the rim there, so a texel outside the stain skips its spines and the rest walk only the spines that can reach
  them, in the same order (`rasterise_writes_the_mask_its_definition_does`, widened to the ceiling of 24 spines and 24
  satellites and to the impacts `stain_shape` makes, holds every byte to the mask by definition). `WetCanvas`'s shade pass keeps a
  table on `(level, age)` where one remembered pair thrashed along every rim; its spread reads the flux from a table of the 511
  differences and its soak is tabulated by age; both skip the bare texels whose surroundings are bare, which is most of what a
  tick covers; and **a tick now covers a spray's own blood and not the tiles it fell in**: the live tiles kept each tile's
  bounding rectangle of blood, which for a spray of fine drops was 9 388 texels covered for 1 144 with blood
  (`a_spray_is_covered_by_its_own_blood_and_not_by_its_tiles` fails on 0.8.0), and the passes are paid per texel covered. The
  passes are held to their definitions on every tick (`the_passes_are_what_their_definitions_say`: `wet` and every pixel of both
  images, four scripted runs). Median hit tick of 45 hits one tick apart, alternated with 0.8.0 eight times on a loaded host:
  a bullet on a floor 4.5 to 3.0 ms, facing a wall 6.9 to 4.8, a heavy wound 8.3 to 5.0, a heavy wound facing a wall 28.5 to
  18.7; and the tick after the last hit, with nothing new thrown, 4.8 to 2.8, 9.4 to 5.7, 5.6 to 2.8 and 26.5 to 15.4. The whole
  of `hit_tick_cost` retires 41 % fewer instructions (30.5 to 18.0 billion). **No digest moves**: `bench_carnage`
  `0x4366a8b64352c85a`, every recorder variant's `digest=`, `flay=` and `dermis=`, and every one of the 6 480 frames the
  recorders write are 0.8.0's, byte for byte. A canvas holds up to 8 KiB more once it has shaded blood and a byte an age of its
  drying span (`WetCanvas::heap_bytes` counts both).

### Added

- `preset::tests::hit_tick_cost` also prints `wet_tick_ms_median_*`, the median of the 90 ticks after the last hit with nothing
  new thrown: every frame of a scene while its blood is wet, which until now nothing measured.

## [0.8.0] - 2026-10-31

### Changed

- **A floor takes what a wound throws as drops, not as blobs** (iteration 6, `docs/research/floor-spatter.md`). A
  default `GorePlugin` floor under a bullet-sized shot took about nine ragged blobs of ten centimetres, each the
  *placement* radius of a droplet (2 to 12 cm) stamped as a sheet's stain: at 12 mm a texel, a few pieces of jam.
  Now each droplet a floor takes is its own stain, as a wall's and a skin's have been since 0.5.62: `hard_stain`'s size
  and slant from the speed it lands at, found on the floor by a cast through the mesh's index instead of the canvas
  walking every triangle once per droplet, so a hit on a floor of twenty thousand triangles costs 4 ms where it cost 9.
  A stain under four texels across, which a canvas cannot give spines, is a `WetCanvas::paint_speck`. **A wound also
  throws `GoreDials::floor_mist` fine droplets at the floors for each droplet it throws at everything** (`9`, so a
  crater's 9 droplets are 90): the same stream, cast at the same surfaces and caught by the first they reach (a wall
  across the room keeps the floor behind it dry), stamped only on a floor. A shot into a body in a three-metre room
  left 4 stains on the floor (the new stains, no mist) and now leaves 25 (`a_shot_leaves_a_fan_of_drops_on_the_floor_not_a_few_blobs`);
  **a wall, a skin and a cloth take exactly what they took**, pixel for pixel in `capture_skinned --through`. Every
  recorder digest whose clip has a hit and a floor moves on purpose (`capture_preset`, and `capture_skinned`'s default,
  `--through`, `--shirt`, `--break`, `--blows`), and `--stroll`, `--quiet` and `--guard` keep theirs.
  **Breaking, pre-1.0 so the next release is a MINOR**: `GoreDials` has a new public field, `floor_mist`; a literal
  that names every field adds `..default()`. A caller who wants the old throw states `floor_mist: 0.0`.

- The README follows the bevy-coven org's documentation guide (250 lines at most; it was 646): the org banner, badges, a Status and a Why line, an 18-line quick start compiled as a doctest, a still image, and "What you can do" bullets. Every section it carried moved unchanged into `docs/guide/` and `docs/gallery.md`, and the guide's full quick start is still compiled as doctests. Image and file links are absolute URLs, so docs.rs shows them.

### Added

- `WetCanvas::paint_speck(uv, semi, direction, tick)` and `wetmap::{SPECK_MAX_SEMI, SPECK_GROW, SPECK_FLOOR}`: a droplet's
  stain too small for a stamp, the blood of an ellipse of semi-axes in texels laid over a soft round footprint, placed to a
  fraction of a texel, so the bytes laid sum to its area (`a_speck_lays_its_area_whatever_its_size_and_where_it_sits`) and it
  is round about where it is centred (`a_speck_is_symmetric_about_where_it_is_centred`). A stamp of two texels peaked at 73
  of 255, one of three at 243 and a slanted one of one at 18 (measured on the floor at 12 mm a texel), and a stain smaller
  than the stamp's floor was drawn as a whole texel; a speck of an eighth of a texel adds an eighth of full coverage. A
  hard-edged disc on a grid is a square after the cubic B-spline a shader reads the canvas through, so the area is spread
  over a bump twice the ellipse's size (`SPECK_GROW`, tuned). Only multiplies, adds and compares on `f32`, no transcendental: the bytes are the same everywhere.
- `GoreDials::floor_mist` and `preset::GORE_FLOOR_MIST` (`9`): see Changed.
- `capture_skinned --floor N` dresses the floor at `N` texels a side instead of the dial's 256, to see spatter at 6 mm a
  texel (`--floor 512`). An ignored measurement test, `preset::tests::hit_tick_cost`, prints what one hit costs the tick it
  lands on in a room with a floor of two triangles and of twenty thousand and a wall (`cargo test --release --lib --
  hit_tick_cost --ignored --nocapture`).
- `DermisCanvas::digest()` (#8): FNV-1a over the bruise-and-burn canvas's bytes, the form `ClothHoles::digest`
  already had (which now delegates to it). `capture_preset` and `capture_skinned` append `flay=<hex>` (every
  `FlayCanvas::digest`, the craters) and `dermis=<hex>` (every `DermisCanvas::digest`, the bruises and burns) to
  their digest line, each a sorted fold over the world **sampled at every frame of the clip**: at the end of
  `capture_preset` and of `capture_skinned --break` the body has broken and no skin canvas is left, and a
  fold of the world as it then stands read `cbf29ce484222325`, the empty fold, whatever the craters had been.
  **No `digest=` value moves and no frame changes**: until now neither a crater's depth nor a bruise's place
  nor a burn's zones was in the reproducibility line, so `capture_skinned --blows` printed the default's line
  and a pass could have made any of them run-dependent unseen. Research: `docs/research/digest-fold.md`.

## [0.7.0] - 2026-10-31

### Changed

- `GoreDials::default()` throws at game scale: its `blood.spatter_speed_scale` is `0.25`
  (`preset::GORE_SPATTER_SPEED_SCALE`), where it inherited `BloodSettings::default()`'s `1.0`. The paper's
  8 to 40 m/s launch speeds, landed by this crate's drag-free parabola, put a droplet from a chest-height
  wound 1.5 to 55 m out (median 4.8 m, 21 of 48 within 4 m), so a three-metre floor under a default
  `GorePlugin` took almost none of what a wound threw, and `preset::tests::a_wound_stains_the_floor_beneath_it`
  was red from `0.5.64` to `0.7.0`. At `0.25` the same wound lands 0.6 to 5.1 m out (median 1.5 m, 47 of 48
  within 4 m), the 0.25 to 4 m field Comiskey, Yarin & Attinger measured their stains over.
  **The scale now compresses the flight to a floor and nothing else**: a droplet a wall, a cloth or a body
  part catches is cast and sized at the paper's launch speed whatever the dial says. Scaled too, a wall's
  stipple came out two thirds the size and nearly one size (`capture_skinned --through`). So a default
  `GorePlugin` stains its walls exactly as it did and its floors where the spatter was measured; a caller who
  authored the dial (`0.25`, or `gore_tier`'s `0.12`) keeps its floors and gets walls, cloth and skin stained at
  the paper's speed. **This moves every recorder digest whose clip has a hit and a floor under the default
  dials** (`capture_preset`, and `capture_skinned` except `--stroll` and `--quiet`); `BloodSettings::default()`
  is unchanged and so is every bloodstain golden. One that wants the old floor throw sets
  `spatter_speed_scale: 1.0`. Research: `docs/research/spray-reach.md`.

### Fixed

- `fracture_skeleton` refuses what it cannot cut exactly, by name (#7). A part that is not a
  `TriangleList` (a `TriangleStrip` or a `LineList`, which Bevy's glTF loader produces for
  `Mode::TriangleStrip` and `Mode::Lines`) or whose index count is not a multiple of three is
  `SkeletalRefusal::NotTriangleList { part }`, where it was cut as a list with the remainder dropped without
  a word; an index past the vertex buffer is `BadIndex { part, triangle }`, where it was reported as
  `SplitTriangle` ("the body is not rigid"); and a vertex with weight on a joint outside its dominant joint's
  group, or on a joint no group holds, is `SplitVertex { part, vertex }`, where the module's own "a
  soft-skinned body is refused" was not kept and the piece jumped at the break. A group's own joints may
  still share a vertex. Three new `SkeletalRefusal` variants: `fracture_skeleton` is in no published
  release, so nothing a consumer holds breaks. `read_part` reads the indices with `try_indices_option`
  rather than the accessor that `expect`s on a mesh extracted to the render world. Research:
  `docs/research/skeleton.md`.
- `tools/qa.sh` finishes a round on a default build (#5, `AG-140`). The six `vfx` examples (`carnage`,
  `capture_carnage`, `pooling`, `capture_pooling`, `ribbons`, `capture_ribbons`) stay on its roster but
  run only when `QA_FEATURES` names `vfx`, which also adds it to the build step; otherwise each gets one
  `missing.txt` line naming the feature and the round goes on, where it used to end at the first of them
  with "no built binary". A binary the default build should have made and did not still ends the round.
  A `vfx` binary an earlier build left behind is not run in a round that did not build `vfx`.
- `tools/qa.sh` renders the skinned body: `capture_skinned` is a recorder name, and so is each variant
  `docs/DEMOS.md`'s recipe records (`capture_skinned_break`, `_shirt`, `_through`, `_guard`, `_spray`,
  `_stroll`, `_blows`, `_quiet`), with the same flags; the autopiloted `skinned` example is on the
  windowed list with a key list taken from its own key reads.
- `tools/qa.sh` gives each windowed run a watchdog budget that grows with its script, `max(180, 60 +
  frames/4)` s, where it was a flat 180 s that reported a slow, busy host as a missing example
  (`skinned` and `sever` hit it). `QA_TIMEOUT` is still one flat limit for every run.
- `README.md` and `docs/DEMOS.md` say what `Cargo.toml` says (#6). The README is the crate-root rustdoc, so
  docs.rs and crates.io showed it: "Requires Bevy 0.19" over `bevy = "0.20.0"`, an install line of
  `bevy_carnage = "0.5"` over `version = "0.7.0"`, `vfx` ticked as a default feature when `default` is
  `["serde", "flesh"]` (and `vfx` cannot be enabled on Bevy 0.20 at all until `bevy_hanabi` publishes for
  it), a banner and a Contributing section calling this repository a read-only mirror of
  `foundation_vs_slop`, and a Status paragraph naming `0.5.64`. All now state the manifest's own values
  and the arrangement `CLAUDE.md` records (this repository is the source of truth; the monorepo's copy is
  frozen at `0.5.64` on Bevy 0.19). `docs/DEMOS.md` no longer says both features are on by default. The
  README gains a `0.6.0` / `0.7.0` section and puts `fracture_skeleton` and `SkeletalRefusal` under
  "Skinned meshes". `Cargo.toml`'s comment about the excluded GIFs no longer says they are served from a
  mirror. Research: `docs/research/readme-truth.md`.

### Added

- `tests/qa_roster.rs`: runs `tools/qa.sh` dry and for real against an empty binary directory, and
  holds what it plans and reports to `Cargo.toml` and `examples/`.
- `tests/readme.rs`: four tests that read `README.md` and `Cargo.toml` and fail when they disagree on the
  Bevy version (`Requires **Bevy X.Y**` against `[dependencies] bevy`), the install line's `bevy_carnage =
  "A.B"` against `[package] version`, the features the table ticks against `default = [..]`, and the version
  the Status paragraph says is in the repository against `[package] version`. A release that moves one of
  those numbers fails `cargo test` until the README moves with it.
