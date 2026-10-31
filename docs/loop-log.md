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

## Iteration 4 — 2026-10-31 — the preset's default throw lands where the spatter was measured (AG-143)

**Item and why it ranked first.** The bug recorded in this log since iteration 0 and first in the backlog of iterations 1
to 3: `preset::tests::a_wound_stains_the_floor_beneath_it` was red (`nothing landed on the floor`, `src/preset.rs`), the
one thing between this loop and its first green gate. The order is a bug issue, then a bug already recorded in the log,
then `consumer` issues, then other enhancement issues, so it beat #8 (an enhancement). Skipped, not blocked: #8,
`#[non_exhaustive]` on the error enums, the Bevy 0.19 citation sweep, the horror-film items. No `consumer` issue is open.
The acceptance was iteration 0's: green **with the test unchanged, a fix in `src/`**; it is.

**Diagnosis, measured before any change.** A probe in `throw_spray` (removed) printed the test's wound: an impact of radius
0.03 m at `(0, 0.9, 0.3)`, seven droplets launched at `13.6, 8.3, 39.6, 8.3, 27.8, 22.6, 36.6` m/s, landing at `z = 4.79,
4.58, 43.47, 1.83, ...` on a floor whose half-width is 1.5 m. Nothing lands on a three-metre floor because
`GoreDials::default()` is `BloodSettings::default()`, whose `spatter_speed_scale` is `1.0`: the paper's launch speeds
landed by `droplet::landing`, a drag-free parabola. A chest-height wound (area 0.02 m², 48 droplets) lands 1.50 to 54.99 m
out, median 4.83 m, 21 of 48 within 4 m. `BloodSettings`' own doc said "Expect to lower it" and every demo lowers it by
hand; the two-line inclusion did not. The assertion was right and the fixture is the demos' own.

**What changed** (`6bfce57` research, `bfbb14a` code, `b8d1f46` docs): `GoreDials::default()` ships
`blood.spatter_speed_scale = GORE_SPATTER_SPEED_SCALE` (`0.25`, public in `preset`), the default wound now lands
0.59 to 5.11 m out, median 1.55 m, 47 of 48 within 4 m, the 0.25 to 4 m field Comiskey, Yarin & Attinger measured
their stains over although the droplets left at 8 to 40 m/s (air drag). `BloodSettings::default()` keeps `1.0`; no
bloodstain golden moved. **Found by measuring, not by reading: the scale must compress the flight to a floor and nothing
else.** The first version scaled the casts too and `capture_skinned --through` frame 195 showed the wall's stipple at
two thirds the size and nearly one size (median diameter 5.3 to 3.4 px on one side of the body, 4.9 to 3.7 on the
other; p90 over p10 of the diameters 6.65 to 3.92 and 4.55 to 1.52; Ref 1's is 8.1), because the stamp's floor of 1.5
texels clamps the small end. `throw_spray` now casts every droplet from the paper's launch speed
(`spatter_speed_scale: 1.0` for the cast) and lands the uncaught ones on a floor with the dial's scale; the wall is
pixel-identical to before. Two tests: `the_default_throw_lands_inside_the_field_the_spatter_was_measured_over`
(>= 95 % of 48 droplets in 0.25 to 4 m, farthest under 8 m; the paper's own settings put the median past 4 m and the
farthest past 40 m) and `the_flight_scale_leaves_a_wall_as_it_was` (**fails first**: it failed, with `left != right`,
against a build that casts at the scaled speed, and passes on the shipped one). The bug's own test is the third.

**Assumed decision (owner may override):** options (a) change `BloodSettings::default()` to `0.25`: breaks four frozen
goldens and makes the paper's constants lie; (b) change only the test's fixture (a 60 m floor): leaves every default
`GorePlugin` with an empty floor, and iteration 0 asked for a fix in `src/`; (c) the preset states the scale,
`BloodSettings` keeps the paper's number. Picked (c), and picked "flight only" over "all speeds" for the reason above.
No issue exists for it, so it is recorded here and in the commit body. A caller who authored the dial (`0.25`, or
`gore_tier`'s `0.12`) keeps its floor exactly and now gets walls, cloth and skin sized at the paper's speed.

**Sources** (`docs/research/spray-reach.md`): physics, Comiskey, Yarin & Attinger 2018
(`doi:10.1103/physrevfluids.3.063901`), quoted through `distill_search`: stains measured on "a butcher paper substrate
strip (0.41 × 4 m²) ... binned from 25–400 cm", droplets at "40 m/s ... forward ... 8 m/s [back]", and "the initially
slower, larger droplets fly further than the initially faster, smaller ones". Reference set, kept under `/tmp/c4/refs/`
and cited by URL only: Ref 1 `commons.wikimedia.org/wiki/File:Blood_Spatter_Texture_-_FREE.jpg` (CC BY 2.0, wall spray, fresh
stain sRGB `(161, 60, 46)`, 632 stains, p90/p10 of diameters 8.1); Ref 2 `.../File:2023_Blutstropfen_Wischspur.jpg` (CC0,
drops, `(172, 54, 30)`); Ref 3 `.../File:Calculating_the_area_of_origin_of_a_blood_splatter.jpg` (CC BY-SA 4.0); Ref 4
`filmhounds.co.uk/2023/04/17/gory-groovy-the-splatter-of-evil-dead/` (*Evil Dead*, press still, film `(150, 37, 30)`); Ref 5
`variety.com/2024/artisans/news/terrifier-3-kills-art-the-clown-1236177913/` (*Terrifier 3*, dried flecks `(56, 29, 21)`); Ref
6 `ilovesplatter.com/2013/04/05/evil-dead-2013/`. Technique: *Dead Island 2*'s FLESH system (Motionographer, AusGamers) and the
jittered-ray cone of the open-source Minecraft "gore" mod. `distill_search` for a game-gore technique source returned only
SIGGRAPH 2011 slide headings; the `catalog_recent` re-scan (limit 200) found nothing in a domain (the window is the bulk
embed of the SIGGRAPH course notes, already in `corpus.md`); `system_status` and `scribe_health` healthy.

**Critique and verdict** (`/tmp/c4/critique.md`, `cmp_top359.png`, `cmp_topthrough359.png`, `cmp_wall_zoom.png`, all removed
with `/tmp/c4`). Against Ref 1 and Ref 2: (1) the floor now takes 9 stains in the top-down view (`capture_skinned --eye
0,4.2,0.3 --at 0,0,0`, frame 359; none before), but each is a ragged blob of about 10 cm (p10 1.1 px, p90 23.6 px, max
25 px) where Refs 1 and 2 show round stains of a few millimetres to two centimetres in their hundreds: wrong size by about
5 times, wrong count by about 10; that is the sheet blob `stain_radius` (2 to 12 cm) the floor has always stamped, now
visible by default; (2) the blood in the frame is `(127, 63, 67)`, pinker and darker than Ref 1 `(161, 60, 46)` and Ref 4
`(150, 37, 30)`, which this item did not touch; (3) **a second round** built the floor on `hard_stain`, as walls and skin
have been since 0.5.62: 7 dots of about 2 cm (236 red pixels against 1,536), the right size and sparse, because a 3.5 cm
crater throws `pi r^2 * 2400 = 9` droplets. It changes every caller's floor look, which the bug did not ask for, and is no
more legible until the count is decided, so it is **parked, not shipped**: branch `loop/iter-4-floor-stains`, commit
`bffcbe8`. Verdict: **spray reach GREAT** against its validation targets (the bug's test green and unchanged, 47 of 48
droplets in the measured field, the wall pixel-identical, every golden untouched, every digest stable); **floor stain look
NOT YET**, unchanged by this item, backlog item 1.

**Numbers.**

- *Reproducibility* (`capture_preset` and `capture_skinned` variants, each twice, before and after, all pairs equal):
  `preset` `b7721708304df85f` to `8a10edd6f39c51d8`; default `ac9ee874ea09f208` to `9677d79de848daaa`; `--break`
  `1fdc02e938a7404a` to `baa76e5ee1e0f626`; `--shirt` `9bc536185df82588` to `2c86f87087d1e03a`; `--through`
  `0e4493510865f8b6` to `890512dd64b645ea`; `--guard` `5acf242d8eeb9360` to `555fc5022db246a1`; `--blows`
  `ac9ee874ea09f208` to `9677d79de848daaa` (the default again, issue #8's blind spot); `--stroll` `8ebe761bcc8b4d83`
  and `--quiet` `f83a631a7426808d` unchanged, as they take no hit. `--eye 0,4.2,0.3 --at 0,0,0` prints the digest it
  printed without it. The bench digest `0x4366a8b64352c85a` is unchanged. Moved on purpose, by a floor that now takes spray.
- *Pixels* (every fifth frame, before against after): `preset` differs from frame 35 in a 168 by 92 patch at its lower
  edge; `skinned`, `--through`, `--guard`, `--break` and `--shirt` are pixel-identical (their camera does not see the floor
  in front of the body). `docs/DEMOS.md` says which committed clips predate this and cannot be re-cut here (`vfx` does not
  build on Bevy 0.20, ImageMagick is absent).
- *Locks*: `hash_f32_is_frozen`, `the_spatter_model_is_frozen`, `the_stain_placement_is_frozen` and the rest of section 3's
  twelve pass untouched inside the 352 lib tests. `tests/wetmap.rs` (mass) 13 pass; the wetmap passes were not touched.
- *Bench* (`bench_carnage`, before and after alternated in the same minute, host load 8 to 10): `carnage_frame_ms` median of
  five stable runs 1.04537 before, 1.05201 after (+0.6 %); `carnage_p99_ms` 29.62 before, 29.06 after (-1.9 %). One
  earlier alternated set ran at load 10 to 11 and read 2.50 to 2.54 ms on both sides. The bench never calls the preset.
- *Gate*, in the loop's order, on the code that ships (`bfbb14a`; the docs commit changed no `src/` line): `cargo build`
  rc 0, 50 s; `cargo build --release` rc 0, 147 s; `cargo test --no-fail-fast` rc 0, 2322 s: **352 lib tests pass, 0 fail**
  (350 and the two new; the red one is green) and every integration binary passes (`cross_section_plugin` 2,
  `expressive_range` 3, `fault_modes` 7, `laceration_plugin` 4, `leaf` 3, `origin` 3, `qa_roster` 4, `readme` 4, `skeleton`
  13, `tear` 9, `viscera` 19, `wetmap` 13) and the doctests (8 passed, 5 ignored); `cargo build --examples` rc 0. After the
  docs commit: `readme`, `leaf`, `qa_roster` and the three locks re-run green. Not run: rustfmt (the repository is not
  rustfmt-clean, the lesson of iteration 2), `tools/qa.sh` (no sheets without ImageMagick).

**Found, not fixed:** `capture_skinned` and `capture_preset` never see the floor in front of the body, so their frames
cannot show the spray this item moves; `--eye 0,4.2,0.3 --at 0,0,0` does, and is how it was judged.

**Release: `0.7.0`, published** (https://crates.io/crates/bevy_carnage/0.7.0, tag `v0.7.0`, commit `a3afdf0` on `main`), the
first since `0.5.18`: the gate was green for the first time since iteration 0, and no tag existed so the 6-hour rule did not
apply. `cargo semver-checks check-release --baseline-version 0.5.18` could not run with the crate's all-features default (it
turns on `vfx`, which does not build on Bevy 0.20: `E0277` in the generated docs); with `--only-explicit-features --features
serde,flesh` it answered `0 checks: 0 pass, 260 skip ... no semver update required`, which is uninformative across a Bevy
bump, so the bump was read by hand: breaking against `0.5.18`, a MINOR bump, and the manifest's `0.7.0` (never published) is
above `0.6.0`, so `Cargo.toml` and `Cargo.lock` keep it and the `[Unreleased]` section became `[0.7.0]`. The release commit
also moved the README's install sentence, Status line and its "unreleased" bullet (`tests/readme.rs` green).
`cargo publish --dry-run` packaged 178 files, 1.5 MiB compressed, and verified; its one warning is `clipboard-win v5.4.1`
yanked in `Cargo.lock`, a transitive dependency of Bevy's windowing that dependents' lockfiles do not read, left alone. The
gate was re-run on the merged tree before the push (`cargo build` 16 s, `--release` 21 s, `cargo test --no-fail-fast` 1651 s,
`--examples` 5 s, all rc 0; 352 lib tests, every integration binary and the doctests pass).

**Cleanup.** Removed `/tmp/c4` (2.1 GB: the before and after recorder frames and binaries, the semver-checks target, the
reference images, a venv with Pillow and SciPy, the gate logs) and `target/package` in the clone's cache; `df`: 18 G free at
the end against 26 G at the start, the difference being the clone's own build cache in
`/Users/ladvien/target/bevy_carnage-loop`, which is kept. No worktree or `*-target` was made (`git worktree list` shows the
clone only; the baseline was the before-binaries copied out of the cache, not a worktree). Left, not mine and not deleted: the
branches `loop/iter-1` (parked, `2ed4559`, superseded by `loop/iter-1b`), `loop/iter-1b`, `loop/iter-2`, `loop/iter-3`
(merged), `refs/wiggum/pass-7-*`, `/tmp/c1`, `/tmp/c12-1` to `/tmp/c12-3`, `/tmp/c5` and `/tmp/c70.sh` (other loops'), and
`~/b2_base` (another loop's clone). Mine and kept on purpose: `loop/iter-4` (merged) and `loop/iter-4-floor-stains` (parked,
pushed).

### Ranked backlog for iteration 5

1. **The floor's stain look and the droplet count**, together. Parked: branch `loop/iter-4-floor-stains` (`bffcbe8`, floors on
   `hard_stain`, 7 dots of 2 cm), adopt it on a new branch. The count is the real gap: a wound throws `area * 2400`
   droplets, 9 for a bullet-sized crater, against hundreds of stains per 4 m strip in Comiskey et al. Research the count
   (and a stipple or mist population on canvases), then judge the top-down frame against Refs 1 and 2, and the colour
   `(127, 63, 67)` against `(161, 60, 46)`.
2. **#8: a fold for flaymap and dermis canvases** in `capture_preset` and `capture_skinned` (`DermisCanvas::digest`, `flay=`
   and `dermis=`), so the reproducibility gate sees craters, bruises and burns; `--blows` still prints the default digest.
3. **`#[non_exhaustive]` on `SkeletalRefusal` (and the other public error enums)**: no longer free. `0.7.0` is published
   with them exhaustive, so adding the attribute is now a breaking change (a MINOR bump pre-1.0). Do it with the next
   variant, or never.
4. **A Bevy 0.19 citation sweep** (`CLAUDE.md`, `docs/wetmap.md`, `docs/flaymap.md`, `tests/wetmap.rs`, `tests/viscera.rs`,
   examples): re-point each to the 0.20 source after checking the cited line.
5. **The horror-film items from iteration 0** (arterial spray, pooling and drying, wound sheen, fracture debris), each needing
   `magick` installed first. `vfx` on Bevy 0.20 is still blocked on `bevy_hanabi` (newest `0.19.0`).

## Iteration 5 — 2026-10-31 — the recorders' digest line folds craters, bruises and burns (AG-144, #8)

**Item and why it ranked first.** #8, the only open issue (an enhancement, no `consumer` label): `capture_preset` and
`capture_skinned` printed a digest that folded the wetmaps and a holed cloth, so a flaymap crater, a bruise or a burn
could come out differently from run to run and both runs printed the same line; `capture_skinned --blows` printed the
default's line (`9677d79de848daaa`). The loop's order is a bug, then `consumer` issues, then every other open
enhancement issue before any candidate of its own, so it beat the backlog's item 1 (the floor's stain look, parked on
`loop/iter-4-floor-stains`, not adopted: it is a visual candidate and #8 outranks it) and the horror-film items. Skipped,
not blocked: nothing else is filed. It is the instrument every later pass's reproducibility gate reads, which is why
it is worth a pass.

**What changed** (`1d33126` research, `3b7c415` corpus, `d46c5d4` code, `6205c97` docs): `DermisCanvas::digest()`
(FNV-1a over the bytes; `ClothHoles::digest` delegates to it). `examples/common/digest.rs` holds the sorted fold both
recorders used inline and a `SkinTrace`. Both recorders keep `digest=` bit for bit and append `flay=` and `dermis=`.
Test: `the_dermis_digest_sees_where_a_bruise_fell_and_how_a_burn_went` (a blow and a burn move the digest off a fresh
skin's, the same hit in a second app repeats it, a blow a hand's breadth away and a burn of another degree move it
again); it did not compile before (`DermisCanvas` had no `digest`; `src/preset.rs`'s only `fn digest` was
`ClothHoles`'s). Its first run failed on an assertion of mine, not the code: an iron held three times as long at 150 °C
prints the same dermis, because the burn's zones are a function of its *degree* (`Degree::Third` either way), so the
test now compares a third-degree burn (1 s at 150 °C) with a second-degree one (30 s at 55 °C).

**Found by running it, not by reading the issue: the proposed fold is blind where it was meant to see.** #8 says "a
sorted FNV fold over that component across the world, the same way the wet fold is done", read once as the clip ends.
Built that way, `capture_preset` and `capture_skinned --break` printed `flay=cbf29ce484222325 dermis=cbf29ce484222325`,
the FNV offset basis, the fold of nothing: the body breaks (frame 120 of the preset, 200 of `--break`), is despawned,
and no skin canvas is left at the end. A field that reads one constant whatever the craters were is the blind spot again
under another name (Bettner & Terrano: "there was always one more thing that slipped just under the radar"). The
fields are therefore **sampled every frame and chained** (`SkinTrace`): preset `flay=50aa35bf8f90ed3b
dermis=056f49e5af3be51d`, `--break` `flay=4407fce2151c2e83 dermis=a4e298cf20094206`.

**Assumed decision (owner may override)** (also on #8, label `assumed-decision`): options (a) one fold at the end, as the
issue proposes: blind for the two clips whose body breaks, measured; (b) sample every frame and chain; (c) fold once at a
per-clip frame before the break: a magic number per recorder. Picked (b): no per-clip constant, sees every canvas while
it exists, hashes about 0.6 MB a frame (an estimate from the canvas sizes: six skins at the shipped 128, 96 KiB each; the measurement is the recorder's wall time, 17.4 and 17.1 s before, 17.0 and 17.8 s after). The price: `--reset-at N` no longer
prints the quiet run's whole line, only its `digest=` (the fields carry the history before the reset), which the
recorder's header now says.

**Sources** (`docs/research/digest-fold.md`): physics, Stam, van Gemert, van Leeuwen & Aalders 2010
(`doi:10.1007/s11517-010-0647-5`), read in the corpus through `markdown_read`: "a forward difference method in space ...
For a time step of 0.1 h, a simulation of 400 h takes 1 min on a standard laptop computer" (the bruise is a function of a
step count), and Gowrishankar et al. 2004 (`doi:10.1186/1475-925x-3-42`, the burn; its step text did not extract, so
`DT_S` is cited from `src/bloodstain/burn.rs`). Horror-film reference: none, nothing visible moves (every frame of nine
variants is byte-identical). Technique: Bettner & Terrano, *1500 Archers on a 28.8*, GDC 2001
(`gamedeveloper.com/programming/1500-archers-on-a-28-8-network-programming-in-age-of-empires-and-beyond`) and Fiedler,
*Deterministic Lockstep* (`gafferongames.com/post/deterministic_lockstep/`: "you could take a checksum of your entire
physics state at the end of each frame and it would be identical"); no shipping game has published a determinism check
of its *gore*, so the lockstep checksum is cited as the nearest practice, not as a gore pipeline. Both are web pages, not
in the corpus: `docs/research/corpus.md` lists them. `catalog_recent` (limit 200) found nothing in a domain (the SIGGRAPH
course-note bulk embed and psychology, forest-ecology and MCP papers); `system_status` and `scribe_health` healthy.

**Critique and verdict** (`/tmp/c5/critique.md`, removed with `/tmp/c5`). Against the six validation targets, all
measured: (1) the test above; (2) acceptance: default `flay=86616c58c7041083 dermis=90cb9d2114068ac2`, `--blows`
`flay=86616c58c7041083 dermis=3168e11cd45a3ca3`, `digest=` equal; (3) every `digest=` after equals before, and every frame
(nine variants, 360 each, `md5`) is byte-identical; (4) **the gate can fail**: the shin blow of `capture_skinned` placed
5 mm further along moved `dermis=` (`90cb9d2114068ac2` to `5efe3f6c25b6764c`) and nothing else, a crater 9.0 to 9.1 mm deep
moved `flay=` (`86616c58c7041083` to `64517b18644072ab`) and nothing else, `digest=` `9677d79de848daaa` in both, then
reverted (a diff of the example shows no mutation left); (5) a breaking body is seen (above); (6) locks and cost below.
Flaws I will not hide: the fold is permutation-blind (two skins that swapped their bruises read the same: the price of not
keying on an `Entity`); `digest=` still reads the world once at the end, so for the two breaking clips it sees only the
floor's and the wall's wetmaps `[INFERENCE: the body is despawned; the WetCanvas count at the end was not measured]`; the
bruise's model state (`Bruising`), `Soaking`, `Tread` and `Wounds` are folded only as what they wrote into a canvas. Verdict:
**digest gate GREAT** against its targets; the look of nothing changed, so no capture is graded. A frame of `--blows`
(`--eye 0.32,0.42,0.8 --at 0.095,0.3,0.05`, frame 300) was read to confirm the recorder draws real frames: two brown
bruises on the shin, blood pools on the floor, as before.

**Numbers.**

- *Reproducibility* (nine variants, each twice, all pairs equal): `digest=` before and after, equal in all nine: `preset`
  `8a10edd6f39c51d8`, default `9677d79de848daaa`, `--break` `baa76e5ee1e0f626`, `--shirt` `2c86f87087d1e03a`, `--through`
  `890512dd64b645ea`, `--guard` `555fc5022db246a1`, `--stroll` `8ebe761bcc8b4d83`, `--blows` `9677d79de848daaa`, `--quiet`
  `f83a631a7426808d`. New `flay=` / `dermis=`: `preset` `50aa35bf8f90ed3b` / `056f49e5af3be51d`; default `86616c58c7041083` /
  `90cb9d2114068ac2`; `--break` `4407fce2151c2e83` / `a4e298cf20094206`; `--shirt` `86616c58c7041083` / `90cb9d2114068ac2` (its slash
  opens cloth and geometry, not a crater or a bruise); `--through` `066d165e0ab24b95` / `90cb9d2114068ac2`; `--guard` `0d1841c84aa56b1d` /
  `10bd4f5f69331f9d`; `--stroll` `4ba4265c49331f9d` / `10bd4f5f69331f9d`; `--blows` `86616c58c7041083` / `3168e11cd45a3ca3`;
  `--quiet` `4ba4265c49331f9d` / `10bd4f5f69331f9d`. The three clips with no bruise or burn share one `dermis=` and the two with
  no crater share one `flay=`, which is what a fold of an untouched canvas should do. `--reset-at 300` prints the quiet
  run's `digest=` (`f83a631a7426808d`); its `flay=` and `dermis=` carry the history before frame 300.
- *Locks and mass*: 353 lib tests pass (352 and the new one), 2 ignored, so `hash_f32_is_frozen`, the spatter, stain, pool,
  bruise, burn and wick goldens, `the_flesh_tables_are_frozen`, `the_tear_is_frozen`, `the_strips_are_frozen`,
  `the_scripted_wound_is_frozen` and `the_measured_rows_are_the_papers` pass untouched; `tests/wetmap.rs` 13 pass (the wetmap
  passes were not touched). No geometry was touched.
- *Bench* (`bench_carnage`, before and after alternated three times each at host load 4 to 5): `carnage_frame_ms` median
  0.50455 before, 0.50736 after (+0.6 %); `carnage_p99_ms` 14.137 before, 14.137 after; digest `0x4366a8b64352c85a` unchanged. At
  load 10 to 14 the bench refused four runs in a row ("NO STABLE PLATEAU", best-3 spread 6.26 % against 5.00 %): a reading about
  the host, not the code, which gained one method no hot path calls and an example-only module.
- *Gate*, in the loop's order, on `d46c5d4`: `cargo build` rc 0, 183 s; `cargo build --release` rc 0, 254 s;
  `cargo test --no-fail-fast` rc 0, 2423 s (lib 353 passed; `cross_section_plugin` 2, `expressive_range` 3, `fault_modes` 7,
  `laceration_plugin` 4, `leaf` 3, `origin` 3, `qa_roster` 4, `readme` 4, `skeleton` 13, `tear` 9, `viscera` 19, `wetmap` 13; doctests 8
  passed, 5 ignored); `cargo build --examples` rc 0, 40 s. After the DEMOS edit `readme`, `leaf` and `qa_roster` re-run green.
  Not run: rustfmt (the repository is not rustfmt-clean, iteration 2's lesson), `tools/qa.sh` (no sheets without ImageMagick),
  clippy (no standing gate).
- *Merged tree* (`a3e5bd3`, the merge of `loop/iter-5` into `main`, before the push): `cargo build` rc 0, 21 s; `cargo build --release`
  rc 0, 20 s; `cargo test --no-fail-fast` rc 0, 2298 s (lib 353 passed, 2 ignored, every integration binary and the doctests as
  above); `cargo build --examples` rc 0, 3 s. `main` pushed (`5bd8960..a3e5bd3`); #8 closed by that push (`Fixes #8`).

**Not released.** The package changed (`src/preset.rs` ships), but `v0.7.0` is under two hours old and loops release at most once
every six hours, so the change rides in the next release; it is in `CHANGELOG.md` under `## [Unreleased]`. It adds a public
method and nothing breaks, so the next bump is a PATCH.

**Owner's file.** #8 asks the loop's checklist to compare all three fields. `.omp/commands/build-loop.md` is the owner's to edit
(a never-edit file for the loop); the request is on #8 in one comment. This loop's own gate compares `digest=`, `flay=` and `dermis=` from here.

### Ranked backlog for iteration 6

1. **`digest=` itself, for a body that breaks.** The same blind spot, in the field that was supposed to be sound: in the two clips whose
   body breaks, the body's `WetCanvas`es are gone at the end `[INFERENCE, measure the count first]`, so blood on the body is in no
   digest for them. Add a per-frame wet trace beside `flay=` and `dermis=` (a new field, `wet=`, so no documented `digest=` moves), and
   audit what else the preset owns that no line folds (`Bruising`'s step counts, `Soaking`, `Tread`, `Wounds`).
2. **The floor's stain look and the droplet count**, carried from iteration 5's list: parked on `loop/iter-4-floor-stains` (`bffcbe8`).
   The count is the gap (9 droplets for a bullet-sized crater against hundreds of stains per 4 m strip in Comiskey et al.); judge the
   top-down frame against Refs 1 and 2 and the blood colour `(127, 63, 67)` against `(161, 60, 46)`.
3. **`#[non_exhaustive]` on `SkeletalRefusal` (and the other public error enums)**: breaking since `0.7.0`; do it with the next variant, or never.
4. **A Bevy 0.19 citation sweep** (`CLAUDE.md`, `docs/wetmap.md`, `docs/flaymap.md`, `tests/wetmap.rs`, `tests/viscera.rs`, examples).
5. **The horror-film items from iteration 0** (arterial spray, pooling and drying, wound sheen, fracture debris), each needing `magick`
   installed first; `vfx` on Bevy 0.20 is still blocked on `bevy_hanabi`.

**Cleanup.** No worktree or `*-target` was made (`git worktree list` shows the clone only). Removed `/tmp/c5` (the before and after
recorder digests and `md5` lists, the binaries copied out of the cache, the gate logs, the home-still catalog dump; 907 MB at its
peak; `df`: 181 G free at the end against 21 G at the start, the difference being other loops' cleanups, not mine). The clone's
own build cache, `/Users/ladvien/target/bevy_carnage-loop`, is kept. Left, not mine and not deleted: `loop/iter-1`, `loop/iter-1b`,
`loop/iter-2`, `loop/iter-3`, `loop/iter-4` (merged), `loop/iter-4-floor-stains` (parked, mine, kept on purpose), `refs/wiggum/pass-7-*`,
and other loops' `/tmp` directories.
