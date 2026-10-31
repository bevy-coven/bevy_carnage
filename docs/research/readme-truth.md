# The README, `docs/DEMOS.md` and the manifest say one thing

Iteration 3 (2026-10-31), issue #6. A non-visible item: no pixel changes, so there is no `## Reference set`;
the reference source is the shipping practice below. The files are `README.md` (which is also the crate-root
rustdoc: `src/lib.rs:1` is `#![doc = include_str!("../README.md")]`, so docs.rs shows it and its `rust` blocks
are doctests), `docs/DEMOS.md`, `Cargo.toml`, and the new `tests/readme.rs`.

## Sources

1. **Physics (the README's own physical claims, which this rewrite must not make false).** README line 49
   leans on Comiskey, Yarin & Attinger, *Theoretical and experimental investigation of forward spatter of
   blood from a gunshot*, Phys. Rev. Fluids 3, 063901 (2018), `doi:10.1103/physrevfluids.3.063901`, read
   through `distill_search` on 2026-10-31 (corpus id `10.1103_physrevfluids.3.063901`, indexed).
2. **Horror-film reference, replaced by a shipping practice** (the item changes no pixel): how a published
   Rust crate keeps the README a reader installs from equal to the manifest. `version-sync`
   (<https://docs.rs/version-sync/latest/version_sync/>, 0.9.5) is the ecosystem's tool for exactly this, and
   `bevy_hanabi`'s README, whose compatibility table this repository's `Cargo.toml:515-516` quotes as
   `| 0.19 | 0.19 |`, is the Bevy-ecosystem form of the same promise: one row per Bevy version.
3. **Technique (what a reader and a registry actually see).** crates.io's API (`/api/v1/crates/bevy_carnage`,
   read 2026-10-31): newest release **`0.5.18`**, 2026-10-12; `bevy_hanabi`'s newest is **`0.19.0`**,
   2026-06-27. `Cargo.toml`: `version = "0.7.0"`, `bevy = "0.20.0"`, `default = ["serde", "flesh"]`,
   `vfx = ["dep:bevy_hanabi", ...]` with `bevy_hanabi = "0.19"` (a Bevy 0.19 crate).

## Craft principle

The model source for a README that is a contract is `version-sync`'s own statement of what drifts when a
crate is released:

> "When making a release of a Rust project, you typically need to adjust some version numbers in your code
> and documentation. This crate gives you macros that covers some typical cases where version numbers need
> updating: TOML examples in the `README.md` files that show how to add a dependency on your crate. [...]
> A `Changelog.md` file that should at least mention the current version."

and the loop's own rule for why the README matters more than any other file (`.omp/commands/build-loop.md`,
"README, demos and docs stay true"): "A README is read before anything else, so a false line there costs more
than a false line anywhere else."

The physics a README line states is held to the same standard. Comiskey et al. give the two speeds the
README's line 49 quotes, and they give them as a **previous experiment's** measurement, not as this paper's:

> "For example, in a previous experiment where both forward and backward spatter were observed, at 0.45 ms
> after bullet impact, droplet velocity in the forward spatter case was 40 m/s, whereas for backward spatter
> it was only 8 m/s [15]"

so `FORWARD_SPATTER_SPEED = 40.0` and `BACK_SPATTER_SPEED = 8.0` (`src/bloodstain/droplet.rs:54,58`) are what
the paper says, and the README's "bracketed by their measured 40 m/s forward and 8 m/s back spatter" is
accurate if "their" is read as the authors' group, which `docs/bloodstain-citations.md:43` makes explicit for
the back-spatter figure (`doi:10.1103/physrevfluids.2.073906`). Nothing in this item touches that line.

## How shipping games do it

- **`version-sync`** (`assert_markdown_deps_updated!`, `assert_contains_regex!`): one integration test, in
  `tests/version-numbers.rs`, that reads `README.md` and `Cargo.toml` and fails `cargo test` when the install
  block names a version the manifest does not carry. "When you run `cargo test`, your version numbers will be
  checked automatically." The cost is a dev-dependency (`regex`, `toml`, `semver`).
- **`bevy_hanabi`**: a README compatibility table, one row per Bevy minor. This crate states the same fact in
  one line ("Requires **Bevy 0.20**") and the test makes that line unable to disagree with `bevy = "0.20.0"`.
- **This repository's `tests/leaf.rs`**: reads `Cargo.toml` with `std::fs` and plain string scanning, no
  dependency, scoped to `[dependencies]` on purpose. The new test follows it (one path per feature: no second
  convention for reading the manifest).

## Model adopted

Four facts a README states that the manifest also states, each held by one test in `tests/readme.rs`:

1. The Bevy line, `Requires **Bevy X.Y**`, equals the `X.Y` of `bevy = { version = "X.Y.Z" ... }` under
   `[dependencies]` (the issue's check 1).
2. The install block's `bevy_carnage = "A.B"` equals the `A.B` of `[package] version` (check 2).
3. The features the README's table marks on (✅) equal `default = [...]` (check 3).
4. The Status paragraph's "`X.Y.Z` in the repository" equals `[package] version` exactly. **Added by this
   pass**, not in the issue: the issue's own table lists `README.md:413` ("`0.5.64` in the repository") as a
   false line, and it was the one that went stale first. The same drift class, the same file pair.

Prose the tests cannot hold is rewritten once, by hand, against the manifest and `CLAUDE.md`:

- The banner (`README.md:18`) and the Contributing paragraph (`README.md:596-599`) said this repository is a
  read-only mirror of `foundation_vs_slop` and that PRs against it cannot be merged. `CLAUDE.md:17`: "Since
  2026-10-19 (the owner's call, Sanctuary decision 0241) this repository is the source of truth on Bevy 0.20.
  Work is done here and pushed here. `foundation_vs_slop`'s `crates/bevy_carnage` is a frozen Bevy 0.19 copy
  (0.5.64), never mirrored over this repo again. Releases to crates.io are cut from here."
- `vfx` is off by default and cannot build on Bevy 0.20 until `bevy_hanabi` publishes for it (`Cargo.toml:8-9`,
  and crates.io's newest `bevy_hanabi` is `0.19.0`). Said in the features table and in `docs/DEMOS.md`.
- The Status paragraph and a changelog section for `0.6.0` and `0.7.0`, written from `Cargo.toml`'s own header
  comment and `CHANGELOG.md`; `fracture_skeleton` goes in the "Skinned meshes" API section, where the other
  skinned-body items are.

`CLAUDE.md`'s "Bevy 0.19 is pinned" lines (backlog item 7 of iteration 0) are a separate file for agents, not
shipped to a reader; the issue names README and `docs/DEMOS.md` only, so they are left to their own item.

## Parameters

| where | was | becomes |
|---|---|---|
| `README.md:18` banner | "This repository is a mirror; `foundation_vs_slop` is the source of truth" | this repository is the source of truth; `foundation_vs_slop` holds a frozen Bevy 0.19 copy (0.5.64) |
| `README.md:59` | `bevy_carnage = "0.5"` | `bevy_carnage = "0.7"`, with one sentence that crates.io's newest is `0.5.18` until the release |
| `README.md:62` | Requires **Bevy 0.19** | Requires **Bevy 0.20** |
| `README.md:69` | `vfx` default ✅, "Turn it off for a headless build" | `vfx` default `—`, blocked on `bevy_hanabi` for Bevy 0.20 |
| `README.md:413-417` | "`0.5.64` in the repository ... developed ... in `foundation_vs_slop`" | `0.7.0` in the repository, developed here |
| `README.md` (new) | nothing on 0.6.0 / 0.7.0 | a "What `0.6.0` and `0.7.0` changed" section above the `0.5.19` one |
| `README.md:596-599` | "read-only history mirror" | where PRs and issues go: here |
| `docs/DEMOS.md:4-6,13-15` | "both features are on by default" | `flesh` is on by default; `vfx` is off and cannot build on Bevy 0.20 |

## Validation targets

- On `main` today the four checks fail with these values: Bevy `0.19` vs `0.20`; install `0.5` vs `0.7`;
  README features `{serde, flesh, vfx}` vs `{serde, flesh}`; Status `0.5.64` vs `0.7.0`. All four pass after.
- `cargo test --doc` still passes with the same counts (8 passed, 5 ignored): the README is the crate-root
  doc, and no `rust` block changes except where `fracture_skeleton` is added, and that one is compiled.
- `grep -n -i 'mirror\|"0.5"\|Bevy 0.19' README.md docs/DEMOS.md` leaves only history entries (the `0.5.x`
  sections and `0.2.0` rows) and a sentence that says the Bevy 0.19 copy is frozen.
- No `src/` line changes, so no digest and no bench moves; the gate still runs in full.

## What great looks like

A reader who copies the install block, reads "Requires Bevy 0.20", turns on the default features and finds
`vfx` "off, blocked on `bevy_hanabi`" gets exactly what the manifest gives them; a contributor who reads the
banner opens the issue or the PR in this repository; and the next release that moves a number the README
quotes fails `cargo test` until the README moves with it.

## Deviations

- Not asked for by the issue: the fourth test (Status version) and the Contributing paragraph and the
  `Cargo.toml:402-403` comment ("served from the mirror"), which carry the same false claim as line 18.
  Design defaults, named here and in the commit.
- The install line says `0.7` although crates.io has no `0.7` yet. The issue asks for it; the sentence under
  the block says so. **The release pass must edit that sentence and the Status line in the `release:` commit**
  (the Status test turns red if the version moves and the line does not).

## Pending index

Nothing: Comiskey et al. 2018 is indexed. `version-sync` and the crates.io figures are web pages, quoted above
and not in the corpus. Found while checking line 49, not fixed here: `src/bloodstain/droplet.rs:51,55` cite the
two speeds as "(ibid., §IV)"; the paper states them in its section III (the viscoelastic-tail discussion, just
before §IV opens), citing its reference [15], so the pointer should say that.
