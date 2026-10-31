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
