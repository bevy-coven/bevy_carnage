# bevy_carnage — loop log

One entry per iteration of `.omp/commands/build-loop.md`, appended, never rewritten. The next
iteration's number is the last one here plus one. `BACKLOG.md` and `BACKLOG_ARCHIVE.md` hold the
pre-loop ticket history (`AG-001` … `AG-139`); the loop's backlog lives at the end of the newest entry
below.

## Iteration 0 — 2026-10-30 — loop setup

Written by the owner's session, not by a pass. Nothing under `src/`, `tests/`, `Cargo.toml` or CI
changed.

- **The command.** `.omp/commands/build-loop.md`, adapted from bevy_stigmergy's. Scratch is
  `/tmp/cN/`, the baseline worktree `~/cN_base` (built into `~/cN_base-target`). The supervisor sets
  `CARGO_TARGET_DIR`; the binaries are under `$CARGO_TARGET_DIR/release/examples/`.
- **The owner's direction, 2026-10-30: this loop learns its look from horror films.** Every pass that
  touches something visible gathers 4-6 practical-effects references (*The Thing*, *Evil Dead*,
  *Hellraiser*, *Re-Animator*, *Terrifier*, *The Fly*, *Saw*, FX-artist pages) with `web_search`, the golem
  MCP's `search_reference_images` and `read`, keeps them under `/tmp/cN/refs/` (never committed unless
  CC0/CC-BY with attribution), writes what each shows into the research note's `## Reference set`, and
  judges its captures beside them in a `magick montage`. The look is replicated procedurally.
- **The gate** (from `.github/workflows/ci.yml` and `CLAUDE.md`): `cargo build`, `cargo build --release`,
  `cargo test`, `cargo build --examples`; plus the reproducibility gate (each recorder's digest equal
  across two runs and to the baseline unless meant to move) and the performance gate (`bench_carnage`'s
  `carnage_frame_ms` and `carnage_p99_ms`, median of three, no regression over 5 %). Clippy, rustfmt and
  wasm are not gates (CI runs none; `BACKLOG.md` records 338 standing clippy warning lines).
  `--all-features` is not a gate: `vfx` cannot build until `bevy_hanabi` publishes for Bevy 0.20.
- **Version.** `Cargo.toml` on `main` says `0.7.0` (set in `f6f7bc7`, 2026-10-19). crates.io's newest is
  **`0.5.18`** (`cargo info bevy_carnage`). The repository has **no tags**. So the first pass that
  releases diffs from `f6f7bc7` per the Release block; everything from `0.5.19` to `0.7.0` reaches
  crates.io then. The version history before this loop is the comment block at the top of
  `Cargo.toml`; from here on it is `CHANGELOG.md` (created with an empty `## [Unreleased]`).
- **Issues.** `gh issue list --repo Ladvien/bevy_carnage --state all` is empty: none has ever been
  filed. bevy_emerge's family issue F12 (`#108`, destructible props and damage dials) is closed.

### Ranked backlog for iteration 1

No open issues, so the order is bugs recorded in the repository, then the horror-film reference set,
then measurable items from the code and docs.

1. **Bug, verify first: `preset::tests::a_wound_stains_the_floor_beneath_it`.** `Cargo.toml`'s `0.6.0`
   note says `cargo test` passes except this test, failing the same way on 0.5.64 under Bevy 0.19, and
   CI runs `cargo test`. Done when `cargo test` is green with the test unchanged (a fix in `src/`, not a
   loosened assertion), or, if it already passes, one log line quoting the run.
2. **Match a horror-film reference set: arterial spray.** A pulsed jet whose droplets streak, fan in a
   cone and land on a wall as elongated stains whose aspect ratio follows the impact angle
   (`capture_skinned --through`, `--guard`). Measure: stain count, smallest-to-largest size spread and
   elongation against angle, beside 4-6 wall-splatter stills; the bench digest moves deliberately or not
   at all.
3. **Match a horror-film reference set: pooling and drying.** A pool's radius over time, a crisp rim,
   the dark ring and brown-black crust as it dries, the highlight breaking up as it skins over
   (`capture_skinned --stroll`, `dry_timeline`, `drying`). Measure the sampled sRGB fresh and dry against
   the references and the radius curve against the rheology the pool model cites. (`pooling` and
   `capture_pooling` need `vfx`; see item 6.)
4. **Match a horror-film reference set: wound sheen.** A crater, a laceration and a cut face read wet:
   a specular highlight on the film, a darker lip, walls darker than the floor (`capture_skinned`,
   `capture_flesh`, `laceration_open`, `flaymap_peel`), beside wound-makeup close-ups. Shader-side; WebGL2
   re-checked after `flesh.wesl` changes.
5. **Match a horror-film reference set: fracture debris.** A broken body's pieces read as meat, fat and
   bone with their own blood, and the pieces' size spread looks torn rather than diced
   (`capture_skinned --break`, `explode`, `sever`), beside dismemberment and body-horror stills; every
   piece still closed and manifold by `isomesh`.
6. **`vfx` on Bevy 0.20.** Six examples (`carnage`, `capture_carnage`, `pooling`, `capture_pooling`,
   `ribbons`, `capture_ribbons`) and the GPU spray cannot build until `bevy_hanabi` supports Bevy 0.20.
   Check whether it has published; if so, bump it and make `cargo build --examples --features vfx` pass.
   Done when that command succeeds, or one log line names the newest hanabi and its Bevy requirement.
7. **`CLAUDE.md` says "Bevy 0.19 is pinned" in four rule lists** (and cites `bevy-0.19.1` and
   `bevy_pbr-0.19.0` paths) while the manifest is on `0.20.0`. Re-point every one to the 0.20 source,
   checking each cited line still says what it is cited for. Measure: `grep -c '0\.19' CLAUDE.md` falls to
   the historical mentions only.
8. **Release.** crates.io serves `0.5.18`; `main` is `0.7.0` with no tag. The first pass that changes
   the package and passes the gate publishes (6-hour rule; `cargo publish --dry-run` first, since the
   optional `bevy_hanabi 0.19` dependency targets Bevy 0.19).

## Iteration 1 — 2026-10-31 — `tools/qa.sh` finishes a round on a default build (#5)

Pass 7 (the driver's number; this is the log's iteration 1) worked on #5 for 5 h 48 m, committed two
research notes and left `tools/qa.sh`, `tests/qa_roster.rs` and the changelog uncommitted; the driver
parked them as `2ed4559` on `loop/iter-1` and `refs/wiggum/pass-7-*` ("never built, tested or gated").
Pass 8 adopted them on `loop/iter-1b` (`loop/iter-1` is the parked branch and was left alone): the two
research commits cherry-picked, the three files brought in with `git checkout 2ed4559 -- <paths>`, then
built, tested, run for real and finished.

**Item and why it ranked first.** Issue #5, the oldest open bug (filed 19:43:56; #6 and #7 are bugs a
second and three seconds younger, #8 an enhancement) and the one with a parked draft. The loop's own
QA command did not work on a default build: a full round died at `capture_carnage` (`vfx` cannot build
on Bevy 0.20), and `capture_skinned`, the recorder every blood measurement uses, was not nameable.
Runner-up: #7 (`fracture_skeleton` reads a strip as a list), the oldest bug that touches a caller's mesh;
it goes first next time.

**What changed** (no `src/`, `examples/`, `Cargo.toml` or shader line; `git diff main -- src examples
Cargo.toml Cargo.lock` is empty):

- The six `vfx` examples stay on the roster and run only when `QA_FEATURES` names `vfx` (which also adds it
  to the build step); otherwise each leaves one `missing.txt` line naming the feature, and the round goes on.
  A binary a default build should have made and did not still ends the round (`need_bin`). Never run a
  binary an earlier build left behind.
- `capture_skinned` and its eight measured variants (`_break _shirt _through _guard _spray _stroll _blows
  _quiet`, flags from `docs/DEMOS.md`) are recorder names; `skinned` is a windowed one (its keys `1`-`6`,
  `C`, `R` are its own `just_pressed` reads).
- **New this pass:** a windowed run's watchdog budget is `max(180, 60 + frames/4)` s instead of a flat
  180 s. The first real runs of the parked draft showed the flat limit kills a slow, busy host's runs and
  reports them as missing: `skinned` (1620 frames) 22/25 and 16/25 shots, `sever` (2660 frames) 28/41, at a
  load average of 6 to 22 (this loop's host shares its cores). `QA_TIMEOUT` is still one flat limit.
- `tests/qa_roster.rs`, 4 tests, drives the real script (dry, and a real recorders-only round against an
  empty binary directory) and holds what it plans to `Cargo.toml` and `examples/`. On main's `qa.sh`: the
  default-round test and the gated-example test fail (the round plans `capture_carnage` under a build line
  without `vfx`; a round named `capture_carnage` dies "no built binary"); the budget test fails
  ("bullet_holes: 180 s for 1880 frames"). All four pass after.

**Sources** (`docs/research/qa-harness.md`, `docs/research/corpus.md`): physics, Hulse-Smith et al. 2005
(`doi:10.1520/jfs2003224`, the stain law the `--through`/`--guard` variants paint); reference, none (no
pixel changed; a non-visible item cites a shipping pipeline), Bevy's `tools/example-showcase`
(`only_default_features`, `required_features` from `Cargo.toml`); technique, Ostrowski & Aroudj 2013
(`doi:10.7603/s40601-013-0010-4`) and Rahman 2023 (`doi:10.48550/arXiv.2309.11077`). Distill answered
`gpu busy` and the CLI `401`, so both were quoted from abstracts; Ostrowski & Aroudj was embedded at 02:08
UTC on 31 October and a later pass can quote the paper. The corpus re-scan found the bulk embed of the
SIGGRAPH *Advances in Real-Time Rendering* course notes (2006-2025), listed in `corpus.md`.

**Critique and verdict** (`/tmp/c1/critique.md`, since removed): `tools/qa.sh` on a default build **GREAT**,
`tests/qa_roster.rs` **GREAT**. Flaws kept on the record: **there is no `magick` on this host**, so the
`sheets` phase was never run by this pass (it is unchanged code, and every new name goes through the same
two sheet functions, but it is not exercised); `QA_FEATURES=vfx` is unexercised because `vfx` cannot build;
a real hang in `sever` is now sampled after 725 s rather than 180 s.

**Numbers.**

- *Full default round, no names* (`QA_PHASES="build recorders terminal windowed"`): before, dies at
  `capture_carnage` with no `warnings.txt` or `digests.txt`; after, runs to its summary, "5 warning lines,
  29 digest lines, 10 missing/failed lines" in 53 m 19 s: six `skipped: needs the vfx feature` lines and the
  two timeouts above. After the budget change `qa.sh sever skinned` gave 0 missing lines, 41/41 and 25/25
  shots in 7 m 27 s at load above 20.
- *Recorder digests*, each name run twice through the script (qa1, qa2), a third time in the full round, and
  three by hand: `capture_skinned` and `_blows` `ac9ee874ea09f208` (equal, as `docs/DEMOS.md` says and
  #8 reports), `_break` `1fdc02e938a7404a`, `_guard` `5acf242d8eeb9360`, `_quiet` `f83a631a7426808d`, `_shirt`
  `9bc536185df82588`, `_spray` `17816e329c9f194d` (ticks=91), `_stroll` `8ebe761bcc8b4d83`, `_through`
  `0e4493510865f8b6`; `capture_preset` `b7721708304df85f`. All equal across runs and to the by-hand runs
  (`--through`, `--blows` with the shin framing, `capture_preset`). `diff qa1/digests.txt qa2/digests.txt`
  identical. Nothing meant to move; nothing moved. Terminal examples in the full round: `rod_determinism`
  run 1 = run 2 `cea7d2e3a3eb8b5c`, `canvas_digest` run 1 = run 2 `0x435fc9b310d712ca`.
- *Bench* (`bench_carnage`, host at load 15-22): one of three reported, `carnage_frame_ms=1.95392`,
  `carnage_total_ms=1172.350`, `carnage_p99_ms=52.98812`, digest `0x4366a8b64352c85a` "unchanged"; the other
  two refused "NO STABLE PLATEAU" (best-3 spread 26.83 % on the first run). That is the host. No source
  line differs from main, so before = after by construction; the baseline worktree was not built (same tree,
  30 GB, a volume at 97 %).
- *Gate*, in the order the loop gives, on a host that was compiling Bevy for four loops at once: `cargo
  build` 45 m 54 s, `cargo build --release` 112 m 47 s, `cargo build --examples` 5 s after the tests had
  built them. `cargo test`: **349 lib tests pass and 1 fails, `preset::tests::a_wound_stains_the_floor_beneath_it`
  ("nothing landed on the floor", `src/preset.rs:6019`)**, which `Cargo.toml`'s 0.6.0 note records as failing
  the same way on 0.5.64, and is not caused by this pass (no source differs). `cargo test` stops there, so the
  integration tests were run with `--no-fail-fast`: all pass (`leaf` 3, `wetmap` 13, `skeleton` 7, `tear` 9,
  `viscera` 19, `qa_roster` 4, ...), and so do the doctests (8 passed, 5 ignored). The locks that live in the
  lib are among the 349; none is the failure.
- *Not run:* nothing for a visible surface. No frame changed, so there is no reference sheet.

**Release: none.** The gate is not green (the pre-existing failure above), so the Release block's "gate
green" is not met, and a publish verify on this host would be a build of that size again. The first release
still carries `0.5.19` to `0.7.0` plus everything since; `0.7.0` has no tag.

**Issues.** #5 fixed (`Fixes #5` in the two commits; the owner's earlier `assumed-decision` comment on it
holds). Open and untouched: #6 (README/`docs/DEMOS.md` contradict the manifest), #7 (`fracture_skeleton`
read holes), #8 (digests fold only wetmaps). Not blocked, only younger.

**What the loop learned about its host.** Release builds of this crate cost 45 to 113 minutes when other
loops are compiling, and each of `cargo build` and `cargo build --release` (declared features only) is a
separate Bevy compile: start the gate's builds first and do the reading while they run. Pass 7's 5 h 48 m
was spent in that. `magick` is absent on big_mac: a visible item needs it installed (or `sips`/PIL) for
the montage and pixel sampling the loop command asks for.

**Cleanup.** Removed `/tmp/c1/before`, `/tmp/c1/after` (4.8 GB of recorder frames and logs; `df` 28 G to
32 G free) and the two script copies. No baseline worktree or `*-target` was made. Left, not mine: the
parked branch `loop/iter-1` (`2ed4559`, now superseded by this pass, safe to delete with the owner's
word), `refs/wiggum/pass-7-*`, and pass 7's `/tmp/c1/dois.txt`, `qa_new.sh`, `skinned_direct.sh`.
`git worktree list`: the clone only.

### Ranked backlog for iteration 2

1. **Bug: `preset::tests::a_wound_stains_the_floor_beneath_it`** (`src/preset.rs:6019`, "nothing landed on
   the floor", an impact at `(0, 0.9, 2.0)` toward `-Z` onto a `Gore::floor()` plane). Red `cargo test` is
   the Release block's blocker. Fix in `src/`, not the assertion.
2. **#7, bug: `fracture_skeleton`'s `read_part`**: refuse a non-`TriangleList`, name an out-of-range index
   (`BadIndex`), refuse a soft-skinned vertex (`SplitVertex`); the three tests in the issue. A breaking
   change (new `SkeletalRefusal` variants): MINOR.
3. **#6, bug: README and `docs/DEMOS.md`** say `0.5`, Bevy 0.19, `vfx` on by default and "this repository
   is a mirror"; add `tests/readme.rs`. Also loop-log item 7 (`CLAUDE.md`'s "Bevy 0.19 is pinned").
4. **#8: a fold for flaymap and dermis canvases** in `capture_preset` and `capture_skinned`
   (`DermisCanvas::digest`, `flay=` and `dermis=` fields), so the reproducibility gate sees craters,
   bruises and burns. The loop command's checklist should then compare all three fields: the owner's file.
5. **The horror-film items from iteration 0** (arterial spray, pooling and drying, wound sheen, fracture
   debris), each needing `magick` (or a substitute) installed first, and the **Release** once item 1 is
   green. `vfx` on Bevy 0.20 is still blocked on `bevy_hanabi`.

## Iteration 2 — 2026-10-31 — `fracture_skeleton` refuses a strip, a bad index and a soft-skinned vertex by name (#7)

**Item and why it ranked first.** Issue #7, a bug: `read_part` in `src/skeleton.rs` cut a `TriangleStrip` or
`LineList` as a triangle list, called an out-of-range index `SplitTriangle` ("the body is not rigid"), and
accepted a vertex weighted across two groups although the module says a soft-skinned body is refused. It beat
#6 (README contradicts the manifest; filed two seconds earlier, so strictly the older bug) because #7 returns
a wrong *result* on a caller's mesh (section 4, rank 1) where #6 is wrong prose, iteration 1's log had named
it "first next time", and #6 is a text edit plus a test that the next pass can do without a build of this
size. #8 is an enhancement. Skipped, not blocked: #6, #8, and the red `preset` test (below).

**Measured before the change** (a throwaway test against the old `src/`, removed): a `TriangleStrip`
`[0,1,2,3]` came back `Ok` with one triangle (index 3 and a triangle dropped); a `LineList` of six indices `Ok`
with two triangles nobody authored; a `TriangleList` of four indices `Ok` with the fourth dropped; index 7 of 3
vertices `Err(SplitTriangle { part: 0, triangle: 0 })`; a 0.6 / 0.4 vertex `Ok`. After: `NotTriangleList`,
`NotTriangleList`, `NotTriangleList`, `BadIndex { part: 0, triangle: 0 }`, `SplitVertex { part: 0, vertex: 0 }`.

**What changed** (`782da1a`, `3baeb0b`): three `SkeletalRefusal` variants, `NotTriangleList { part }`,
`BadIndex { part, triangle }` and `SplitVertex { part, vertex }`; `read_part` checks topology first, then the
index count (a multiple of three: glTF says a list's vertex count `MUST` be divisible by three), then per vertex
that every joint with positive finite weight is in the dominant joint's group, then per triangle `BadIndex`
before `SplitTriangle`; `Rig::carrying_joints` (crate-private) reads the same weights `dominant_joint` does;
`mesh.indices()` became `try_indices_option()` (the accessor `expect`s on an extracted mesh, `CLAUDE.md`
forbids it). Eight new tests in `tests/skeleton.rs` (13 total; the seven old ones pass unchanged, including
`triangle: 24`). **No owner decision was needed:** the issue offered "refuse, or correct the module doc" and
recommended refusing; refusing it is. Two additions the issue did not list, both the same hole: the index count
not a multiple of three, and `try_indices_option`; both are under `## Deviations` in the note.

**Sources** (`docs/research/skeleton.md`, `docs/research/corpus.md`): physics, the glTF 2.0 "Skins" equation
`Σ W_i · M_Ji · B_Ji` (so one anchor's matrix cannot draw a mixed vertex where the body did; the piece jumps at
the break) and Kavan et al. 2007 (`doi:10.1145/1230100.1230107`, abstract only, no open copy); reference (a
non-visible item, so a shipping pipeline), Creation Engine `BSDismemberSkinInstance` partitions ("Polygons
cannot be assigned to multiple dismemberment partitions", GECK wiki, read through a search result's text, the
site answers a direct fetch with a Cloudflare 403) and Raven's GHOUL zones (Wikipedia wikitext); technique, the
glTF "Primitive topology" rules and `bevy_gltf-0.20.1/src/loader/gltf_ext/mesh.rs:24-27`. `distill_search`
answered `gpu busy: 4508 MB free < 5000 MB required to load bge-m3`, so the corpus was not searched; the
`catalog_recent` re-scan (limit 200) found nothing in a domain; `paper_download` for Kavan found no copy.

**Critique and verdict** (no pixel changed, so no frame and no reference sheet). Flaws I looked for against the
note's `## What great looks like` ("a refusal that names the true cause and the offending part, triangle or
vertex, never a piece cut from the wrong geometry"): (1) the vertex check has **no tolerance** on purpose, so a
rigid vertex exported with residue weight `1e-9` on another group is refused, and the caller cannot override it;
that is the exactness the module promises, and it names the vertex so the weights can be cleaned, but a game's
asset pipeline may meet it; (2) `SkeletalRefusal` is not `#[non_exhaustive]`, so the next variant is a breaking
change again; `fracture_skeleton` is in no published release, so marking it now would cost nothing, which is
item 3 of the backlog below; (3) a vertex no triangle uses still has to be rigid (the existing rule for an
ungrouped joint, kept). Verdict: `fracture_skeleton`'s input refusals **GREAT** against the issue's acceptance
and the three failing-first inputs; flaws 1 and 2 are on the record.

**Numbers.**

- *Reproducibility* (recorders, each twice): `capture_preset` `b7721708304df85f` before and after, both runs
  equal; `capture_skinned` `ac9ee874ea09f208` before and after, both runs equal (`frames=360 ticks=365`). The
  other skinned variants were not run: `fracture_skeleton` has no caller in `examples/`, and the changed
  `Rig::carrying_joints` is called only from it. The bench's digest `0x4366a8b64352c85a` "unchanged" on both
  stable runs and equal to iteration 1's.
- *Locks*: all twelve of section 3's (`hash_f32_is_frozen` ... `the_measured_rows_are_the_papers`) ran and
  passed untouched, as part of `cargo test`'s 349 passing lib tests. `tests/wetmap.rs` (mass conservation) 13
  pass; `tests/leaf.rs` 3 pass (no panicking call in `src/`).
- *Bench* (`bench_carnage`; the host refused some runs, so no median of three): before, all three runs refused
  "NO STABLE PLATEAU" (best-3 spread 40.31 % on the first, host load 13 to 15), so there is no before number from
  this pass; after, load 7.7, two of three stable: `carnage_frame_ms` 1.48808 and 1.50466, `carnage_p99_ms`
  39.36500 and 42.49383, `carnage_total_ms` 892.849 and 902.796, the third refused (spread 8.21 %). Iteration 1
  recorded 1.95392 ms and 52.98812 ms p99 at load 15 to 22 on the same code path. The bench never calls
  `fracture_skeleton` or `Rig::carrying_joints`, so no change to its cost is claimed or implied; the figures
  are the host's, not the change's. No baseline worktree was built (30 GB, a volume at 97 %).
- *Gate*, in the loop's order, on the committed tree: `cargo build` rc 0 in 143 s; `cargo build --release` rc 0
  in 395 s; `cargo build --release --examples` rc 0 in 1330 s; `cargo test --no-fail-fast` rc 101 in 3009 s:
  **349 lib tests pass and 1 fails, `preset::tests::a_wound_stains_the_floor_beneath_it` ("nothing landed on
  the floor", `src/preset.rs:6019`)**, the same count and the same test as iteration 1, on a tree whose
  `src/preset.rs` this pass did not touch; every integration binary passes (`skeleton` 13, `leaf` 3, `wetmap`
  13, `viscera` 19, `tear` 9, `qa_roster` 4, `fault_modes` 7, `laceration_plugin` 4, `cross_section_plugin` 2,
  `expressive_range` 3, `origin` 3) and so do the doctests (8 passed, 5 ignored); `cargo build --examples` rc 0.
  No new compiler warning from `src/skeleton.rs`, `src/skin.rs` or `tests/skeleton.rs`.
- *Not run:* rustfmt. The repository is not rustfmt-clean (28 diffs on `src/skeleton.rs` and
  `tests/skeleton.rs` as they stood at `main`, no `rustfmt.toml`), so `cargo fmt` would restyle unrelated lines;
  the new code follows the files' own long-line style instead.

**Release: none.** The gate is not green for the one pre-existing reason above, and the Release block asks for a
green gate. `main` is `0.7.0`, crates.io has `0.5.18`, no tag exists. This pass's change adds enum variants to
a public item that no published release contains.

**Issues.** #7 fixed (`Fixes #7` in `782da1a`). #6 and #8 stay open, not blocked.

**Cleanup.** Removed `/tmp/c2` (457 MB: the before/after bench and recorder output, the glTF spec, the GECK,
Nexus and GitHub pages I fetched, `fmt/` and `gate.sh`). No worktree or `*-target` was made; `git
worktree list` shows the clone only. `df`: 29 G free at the end, 32 G at the start (the gate's builds in
`/Users/ladvien/target/bevy_carnage-loop`, which is the clone's own cache and is kept). Left, not mine and not
deleted: the parked branch `loop/iter-1` (`2ed4559`, superseded by `loop/iter-1b`), `refs/wiggum/pass-7-*`, pass
7's `/tmp/c1/dois.txt`, `qa_new.sh` and `skinned_direct.sh`, and `/tmp/c12-*`, `/tmp/c3`, `/tmp/c4`, `/tmp/c5`,
`/tmp/c70.sh`, which belong to other loops.

### Ranked backlog for iteration 3

1. **Bug: `preset::tests::a_wound_stains_the_floor_beneath_it`** (`src/preset.rs:6019`), still red, still the
   Release block's blocker (an impact at `(0, 0.9, 2.0)` toward `-Z` onto a `Gore::floor()` plane lands nothing).
   Fix in `src/`, not the assertion. Then **Release** (crates.io `0.5.18`, `main` `0.7.0`, no tag).
2. **#6, bug: README and `docs/DEMOS.md`** say `0.5`, Bevy 0.19, `vfx` on by default and "this repository is a
   mirror"; add `tests/readme.rs` (the three checks in the issue). Also `CLAUDE.md`'s "Bevy 0.19 is pinned".
   `fracture_skeleton` and its refusals are in no README section either: put them in the one the issue names.
3. **`#[non_exhaustive]` on `SkeletalRefusal` (and the other public error enums) before the first release that
   has them**: free today, a MINOR bump for every new variant after. An API decision, so record it as an
   assumed one on the issue.
4. **#8: a fold for flaymap and dermis canvases** in `capture_preset` and `capture_skinned`
   (`DermisCanvas::digest`, `flay=` and `dermis=` fields). The loop command's checklist should then compare all
   three fields: the owner's file.
5. **The horror-film items from iteration 0** (arterial spray, pooling and drying, wound sheen, fracture
   debris), each needing `magick` installed first. `vfx` on Bevy 0.20 is still blocked on `bevy_hanabi`.

## Iteration 3 — 2026-10-31 — README and `docs/DEMOS.md` say what `Cargo.toml` says (#6)

**Item and why it ranked first.** Issue #6, the oldest open bug issue that was still reproducible (filed 19:43:57;
#7 is fixed, #8 is an enhancement and ranks behind any bug). The command's order is a bug issue first, then a bug
recorded in the log, so it beat the red `preset::tests::a_wound_stains_the_floor_beneath_it`, which iterations 1
and 2 both ranked first in their backlogs and which is now the top item for iteration 4. The README is also the
crate-root rustdoc (`src/lib.rs:1`), so docs.rs and crates.io show it: four false lines were the first thing a
reader met. Skipped, not blocked: #8, the red `preset` test.

**Measured before the change** (the new `tests/readme.rs` against `main`'s README; all four failed):
Bevy `0.19` vs `0.20`; install `"0.5"` vs `"0.7"` (`0.7.0`); features ticked `["flesh", "serde", "vfx"]` vs
`default = ["flesh", "serde"]`; Status `0.5.64` vs `0.7.0`. After: 4 passed.

**What changed** (`a6ff43f`, `7fb1974`; no `src/`, `examples/` or `Cargo.lock` line, `git diff main -- src examples
Cargo.lock` is empty): `README.md` banner and Contributing paragraph say this repository is the source of truth and
`foundation_vs_slop` holds a frozen Bevy 0.19 copy (`0.5.64`), as `CLAUDE.md:17` does; install line `"0.7"` with one
sentence that crates.io's newest is `0.5.18` (a Bevy 0.19 crate) and the git form to use until then; "Requires Bevy
0.20"; `vfx` is `—` in the feature table and its feature bullet says it cannot be enabled on Bevy 0.20 (the locked
graph under `--features vfx` holds `bevy_hanabi v0.19.0 -> bevy v0.19.1` beside `bevy v0.20.0`, `bevy_ecs` at both
versions; not compiled, to keep a second Bevy out of a volume at 98 %); Status says `0.7.0` and has a "What `0.6.0`
and `0.7.0` changed" section; `fracture_skeleton`, `JointGroup`, `JointLink`, `SkeletalFracture`, `SkeletalPiece`
and `SkeletalRefusal` are under "Skinned meshes", and "It fractures the bind pose" points at them. `docs/DEMOS.md`
says `flesh` is on by default and `vfx` is off and cannot build. `Cargo.toml`'s GIF-exclusion comment no longer says
"mirror". `tests/readme.rs`, four tests, `std::fs` and string scanning like `tests/leaf.rs`, no dependency.

**Assumed decision:** none needed; the issue prescribed the fix. Two additions beyond it, both in
`docs/research/readme-truth.md` under Deviations: the fourth test (the Status version, exact to the patch, so a
release commit must move the README line or `cargo test` goes red) and the Contributing and `Cargo.toml` edits, which
carried the banner's false claim.

**Sources** (`docs/research/readme-truth.md`, `docs/research/corpus.md`): physics, Comiskey, Yarin & Attinger 2018
(`doi:10.1103/physrevfluids.3.063901`), quoted through `distill_search` (distill answered this pass): the 40 m/s
forward and 8 m/s back speeds the README quotes are a *previous experiment's* ("at 0.45 ms after bullet impact ...
40 m/s, whereas for backward spatter it was only 8 m/s [15]"), which `src/bloodstain/droplet.rs:51,55` cite as
"(ibid., §IV)" and the paper gives in its section III; reference (a non-visible item, so a shipping practice),
`version-sync` and `bevy_hanabi`'s per-Bevy-version README table; technique, crates.io's API (`bevy_carnage` newest
`0.5.18` 2026-10-12, `bevy_hanabi` newest `0.19.0` 2026-06-27). The corpus re-scan (`catalog_recent` limit 200)
found nothing in a domain; `system_status` healthy.

**Critique and verdict** (`/tmp/c3/critique.md`, since removed; no pixel changed, so no frame or sheet): README,
`docs/DEMOS.md` and `tests/readme.rs` against #6's acceptance **GREAT**. Flaws on the record: the install-line
sentence "until `0.7` is published" turns false at the release and no test holds it (the Status test going red is
the prompt); the README still carries drift #6 did not list ("Ten" / "Fourteen" browser examples, `cargo clippy ...
# 12 warnings pre-exist` against BACKLOG's 338 lines, "the seven sibling crates re-exported whole"); `docs/DEMOS.md`
is held by no test; the consumer line "used by Sanctuary and bevy_emerge's family integration" rests on `CLAUDE.md`
and the loop command, not on a look at either repository.

**Numbers.**

- *Recorders and bench:* not run. No `src/` or `examples/` line differs from `main`, so every digest and the bench
  are `main`'s by construction (iteration 2's: `capture_preset` `b7721708304df85f`, `capture_skinned`
  `ac9ee874ea09f208`, bench `0x4366a8b64352c85a`). No baseline worktree was built.
- *Gate*, in the loop's order, on the committed tree: `cargo build` 55 s, `cargo build --release` 28 s (both ok, warm
  cache); `cargo test --no-fail-fast`: **349 lib tests pass and 1 fails, `preset::tests::a_wound_stains_the_floor_beneath_it`
  (`src/preset.rs:6019`)**, the same count and test as iterations 1 and 2 (lib tests 170 s); every integration binary
  passes (`readme` 4 new, `skeleton` 13, `viscera` 19, `wetmap` 13, `tear` 9, `fault_modes` 7, `laceration_plugin` 4,
  `qa_roster` 4, `leaf` 3, `origin` 3, `expressive_range` 3, `cross_section_plugin` 2) and so do the doctests (8 passed,
  5 ignored: the README is the crate doc and no `rust` block in it changed); `cargo build --examples` ok. `cargo doc
  --no-deps` ok in 5 m 40 s, 52 warnings, none at `src/lib.rs:1` (the README). Whole second job 29 min at load 5 to 8.
- *Locks:* the twelve of section 3 are inside the 349 passing lib tests, untouched.

**Release: none.** The gate is not green for the one pre-existing reason above; `main` is `0.7.0`, crates.io has
`0.5.18`, no tag exists. **The release pass must edit the README's install sentence and Status line in the `release:`
commit** (`tests/readme.rs` makes the Status line a red test if it forgets).

**Issues.** #6 fixed (`Fixes #6` in `a6ff43f`). #8 open, not blocked, only younger than a bug.

**Found, not fixed:** `src/bloodstain/droplet.rs:51,55` cite the spatter speeds as "(ibid., §IV)"; the paper has them
in section III, quoting its reference [15] (`docs/research/readme-truth.md`, Pending index).

**Cleanup.** Removed `/tmp/c3` (my six files: the gate logs, `doc.log` and the critique, 84 KB; `df` 27 G free at the
end against 29 G at the start, the difference being the gate's builds in `/Users/ladvien/target/bevy_carnage-loop`,
the clone's own cache, which is kept). No baseline worktree or `*-target` was made; `git worktree list` shows the
clone only. Left, not mine and not deleted: the parked branch `loop/iter-1` (`2ed4559`, superseded by
`loop/iter-1b`), `refs/wiggum/pass-7-*`, pass 7's `/tmp/c1/dois.txt`, `qa_new.sh` and `skinned_direct.sh`, `/tmp/c12-*`,
`/tmp/c4`, `/tmp/c5` and `/tmp/c70.sh` (other loops'), and `~/b2_base` (1.6 MB, another loop's clone with
uncommitted `examples/two_balloons.rs`, `src/balloon.rs` and `tests/zz_probe.rs`; not removable, not idle-proven).

### Ranked backlog for iteration 4

1. **Bug: `preset::tests::a_wound_stains_the_floor_beneath_it`** (`src/preset.rs:6019`, "nothing landed on the floor",
   an impact at `(0, 0.9, 2.0)` toward `-Z` onto a `Gore::floor()` plane), red since 0.5.64 on Bevy 0.19, and the one
   thing between this loop and its first release. Fix in `src/`, not the assertion; if it is a real miss of the floor
   (a wound that throws nothing at a floor below it) it is a consumer-visible bug and `0.5.18` shipped it. Then
   **Release** (crates.io `0.5.18`, `main` `0.7.0`, no tag; the 6-hour rule; the install sentence and Status line).
2. **#8: a fold for flaymap and dermis canvases** in `capture_preset` and `capture_skinned` (`DermisCanvas::digest`,
   `flay=` and `dermis=` fields), so the reproducibility gate sees craters, bruises and burns. The loop command's
   checklist should then compare all three fields: the owner's file.
3. **`#[non_exhaustive]` on `SkeletalRefusal` (and the other public error enums) before the first release that has
   them**: free today, a MINOR bump for every new variant after. An API decision: record it as an assumed one.
4. **A Bevy 0.19 citation sweep.** `CLAUDE.md` still says "Bevy 0.19 is pinned" in four rule lists and cites
   `bevy-0.19.1/` and `bevy_pbr-0.19.0/src/pbr_material.rs:153-154,157-163`; so do `docs/wetmap.md`, `docs/flaymap.md`,
   `tests/wetmap.rs`, `tests/viscera.rs` and several examples' comments. Re-point each to the 0.20 source after checking
   the cited line still says what it is cited for; fix the README drift in the critique above in the same pass.
5. **The horror-film items from iteration 0** (arterial spray, pooling and drying, wound sheen, fracture debris), each
   needing `magick` installed first (absent on big_mac). `vfx` on Bevy 0.20 is still blocked on `bevy_hanabi`
   (newest `0.19.0`).
