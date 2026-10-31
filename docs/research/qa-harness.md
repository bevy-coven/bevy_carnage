# The QA harness (`tools/qa.sh`)

Iteration 1, issue #5. `tools/qa.sh` is the one command that renders every recorder, terminal example
and autopiloted windowed example, and the loop's sections 2 and 7 measure through it. Since 0.6.0
(Bevy 0.20) it has not finished a default round: six of its examples need `vfx`, which cannot build
on Bevy 0.20, so `need_bin` ends the round at the first of them; and `capture_skinned`, the recorder
for blood on a skinned body, is not on its roster at all, so naming it is refused.

## Sources

1. **Physics.** Hulse-Smith, Mehdizadeh & Attinger, *Deducing drop size and impact velocity from
   circular bloodstains*, J. Forensic Sci. 50(1), `doi:10.1520/jfs2003224` — the stain law
   `capture_skinned --through` and `--guard` paint onto the wall behind the body
   (`src/bloodstain/stain.rs`, `minor / major = sin θ`). The variants this item puts on the roster are
   the recorders the loop's spray, pool and wound measurements are taken from.
2. **Reference.** None. The item changes no pixel. Per the loop command, a non-visible item cites a
   shipping pipeline in its place: Bevy's own example runner, below.
3. **Technique.**
   - Bevy, `tools/example-showcase/src/main.rs` (`bevyengine/bevy`, `main`),
     <https://github.com/bevyengine/bevy/blob/main/tools/example-showcase/src/main.rs> — the engine's
     tool that runs every example in CI and screenshots it.
   - Ostrowski & Aroudj, *Automated Regression Testing within Video Game Development*, 2013,
     `doi:10.7603/s40601-013-0010-4`.
   - Rahman, *Weak Supervision for Label Efficient Visual Bug Detection*, 2023,
     `doi:10.48550/arXiv.2309.11077` (in the home-still corpus, embedded).

## Craft principle

An automated regression round is only worth having if it runs to the end without a person. Ostrowski
& Aroudj's abstract:

> "Automated testing reduces the costs of software development. We propose a testing model that is
> specifically designed for the creation and execution of fully automated regression tests within
> video game development without the need of test isolation."

and Rahman's, on why the visual half cannot be left to hand checks:

> "As video games evolve into expansive, detailed worlds, visual quality becomes essential, yet
> increasingly challenging. Traditional testing methods, limited by resources, face difficulties in
> addressing the plethora of potential bugs."

So a round that stops at its fifth recorder is not a slower round, it is no round: no terminal
example, no windowed example, no `digests.txt`. And a round that cannot be pointed at the one recorder
that shows blood on a skinned body does not test what a game ships.

## How shipping games do it

Bevy's example runner treats an example's extra features as data read from the manifest, and either
builds the example with them or leaves it out on purpose. It never dies on one it was never going to
build. From `tools/example-showcase/src/main.rs`:

```rust
        #[arg(long)]
        /// Only run examples that don't need extra features
        only_default_features: bool,
```

```rust
                    .filter(|example| {
                        !only_default_features || example.required_features.is_empty()
                    })
```

```rust
                let required_features = if to_run.required_features.is_empty() {
                    vec![]
                } else {
                    vec!["--features".to_string(), to_run.required_features.join(",")]
                };
```

and the list itself comes from `parse_examples`, which reads `Cargo.toml` and takes each `[[example]]`'s
`required-features`.

## Model adopted

- **The roster stays complete.** Every example the crate ships that renders or prints a digest is on
  one of the three lists. The six `vfx` names stay on them, so the day `bevy_hanabi` publishes for
  Bevy 0.20 they run again with their key lists intact.
- **A gate is stated once.** `gated_by <example>` names the non-default feature an example needs (`vfx`
  for `carnage`, `capture_carnage`, `pooling`, `capture_pooling`, `ribbons`, `capture_ribbons`), from
  each example's `required-features` in `Cargo.toml`.
- **The round builds what it was asked to.** The build step is `--examples` plus
  `--features $QA_FEATURES` when that is set. A gated example runs only when its feature is in
  `QA_FEATURES`; otherwise the round writes one `missing.txt` line naming the feature and goes on (a dry
  run prints the same as a `# skip` line).
- **A missing binary the build should have made still ends the round.** `need_bin` keeps its `die` for
  every example that is not gated: that is a build that lost an example, the failure `need_bin` exists
  to catch.
- **Gate on the feature list, not on the binary's presence.** Cargo skips an example whose features are
  unmet and never deletes the binary an earlier build left, so a present `capture_carnage` in a target
  directory says nothing about this build. Skipping only "while the binary is absent" would run a stale
  binary and file its digest as today's.
- **`capture_skinned` is a recorder name, and so is each variant the loop measures with**, each mapped
  to `capture_skinned` with the flags `docs/DEMOS.md`'s "Regenerating these" recipe uses:
  `capture_skinned_break`, `_shirt`, `_through`, `_guard`, `_spray` (`--guard --slow 4`), `_stroll`,
  `_blows` (with the shin framing) and `_quiet`.
- **`skinned` is a windowed name.** It is registered through `common::autopilot`, and the script
  promises every autopiloted windowed example. Its key list is its own `just_pressed` reads: `1`–`6`,
  `C`, `R`.
- **The test is the dry plan.** `tests/qa_roster.rs` runs the script (`QA_DRY=1`, and a real
  recorders-only round against an empty binary directory) and holds what it plans and reports to
  `Cargo.toml` and `examples/`.

## Parameters

| Name | Value | Status |
|---|---|---|
| `QA_FEATURES` | empty | Design default: a default round builds the default features, as a consumer gets them |
| `gated_by` | `vfx` for six examples | From `Cargo.toml`'s `required-features`; held to it by `tests/qa_roster.rs` |
| `bench_carnage` | off every list | Design default: it is timed by the loop's bench, three runs before and after, and refuses a noisy reading; a QA round is not a timing run |

## Validation targets

1. A recorders-only round against an empty `QA_BIN`, named `capture_carnage`: exits 0, and
   `missing.txt` holds one line naming `capture_carnage` and `vfx`. Today: exit 1, `no built binary`.
2. The same round named `capture_holes`: still exits non-zero with `no built binary`.
3. A dry default round plans every example under `examples/` except `bench_carnage` whose
   `required-features` the build line provides, plans none it does not, and prints a `# skip` line for
   each one it leaves out. Today it plans the six `vfx` examples under a build line without `vfx`, and
   neither `capture_skinned` nor `skinned`.
4. Each `capture_skinned` variant run through the script prints the same digest twice, and the same
   digest as the binary run by hand with the recipe's flags.
5. `skinned` under the autopilot writes every screenshot its script asks for.

## What great looks like

`FONT=… tools/qa.sh <out>` on a default build runs to its summary line on any host the crate builds
on, renders every surface it can build, and says in `missing.txt` exactly which ones it could not and
which feature would bring them back. Every recorder the loop measures a look with is a name it accepts.
Nothing it reports comes from a binary this build did not make.

## Deviations

None from a source. `QA_FEATURES` and keeping `bench_carnage` off the roster are design defaults (see
Parameters).

## Pending index

- `distill_search` could not be used this pass: the MCP bridge answered `Embedding error: gpu busy:
  4640 MB free < 5000 MB required to load bge-m3`, and the CLI (`hs distill search`) answered
  `401 Unauthorized: missing bearer token`. Sources were found with `hs paper search` / `hs paper get`
  and the papers already quoted in the repository.
- Ostrowski & Aroudj 2013 (`10.7603/s40601-013-0010-4`) was not in the home-still corpus on the first
  attempt (quoted from its abstract as `hs paper get` returns it); the stem `s40601-013-0010-4` was
  embedded at 02:08 UTC on 2026-10-31, so a later pass can quote the paper itself.
- The second attempt's `distill_search` answered `500 gpu busy: 3288 MB free < 5000 MB required to load
  bge-m3` and the CLI `401 Unauthorized`, as before; `catalog_recent` and `hs status` work.
