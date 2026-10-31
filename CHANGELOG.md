# Changelog

All notable changes to this crate are recorded here. `0.6.0` and `0.7.0` (Bevy 0.20, `fracture_skeleton`) are
described in the comment block at the top of `Cargo.toml`; they and every entry below were published together as
`0.7.0`, the first release since `0.5.18`, which is a Bevy 0.19 crate.

## [Unreleased]

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
