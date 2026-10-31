# Digest fold: the recorders' reproducibility line sees craters, bruises and burns

Iteration 5 (2026-10-31), issue #8 (`capture_preset` / `capture_skinned` digests fold only wetmaps:
flaymap craters, bruises and burns are invisible to the reproducibility gate; found by the owner's review
pass, 2026-10-30). Not a visible item: no pixel and no existing `digest=` moves. It is the gate the visible
items of every later pass are judged by, so it ranks first by the order the loop is bound to: no bug issue
is open and no `consumer` issue is open, and #8 is the oldest enhancement issue.

## The blind spot, measured on the code (not inferred)

- `examples/capture_preset.rs` folds `WetCanvas::digest` and nothing else; `examples/capture_skinned.rs` folds
  `WetCanvas::digest` and, for a cloth that was holed, `ClothHoles::digest`.
- `FlayCanvas::digest` exists (`src/flaymap/canvas.rs`, FNV-1a over the depth buffer) and every dressed skin
  carries a `FlayCanvas`; neither recorder reads it. A crater's depth and place are invisible to the gate.
- `DermisCanvas` (`src/preset.rs`) holds every bruise and every burn and had no `digest` at all; only the
  cloth-hole wrapper hashed one. `docs/DEMOS.md` recorded the blind spot as a fact (`--blows` "prints the
  default digest, `9677d79de848daaa`", because the second bruise lives only in the dermis), and the
  changelog entries for 0.5.43 and 0.5.44 offer "Dermis only; every recorder digest holds" as evidence that
  nothing moved: a sentence that, with the dermis not folded, cannot fail.
- Before this change `capture_skinned` and `capture_skinned --blows` print identical lines
  (`ac9ee874ea09f208` in the 0.5.x clips, `9677d79de848daaa` now), measured this pass in
  `/tmp/c5/before/digests.txt`.

## Sources

1. **Physics** (the models whose CPU state is being folded): Stam, van Gemert, van Leeuwen & Aalders,
   *3D finite compartment modeling of formation and healing of bruises*, Med Biol Eng Comput 2010,
   `doi:10.1007/s11517-010-0647-5`, in the local corpus (`markdown_read` of stem
   `10.1007_s11517-010-0647-5` this pass). It is the `bloodstain::bruise` model: a bruise's colour on the
   dermis canvas is a function of its step count. And Gowrishankar, Stewart, Martin & Weaver, *Transport
   lattice models of heat transport in skin with spatially heterogeneous, temperature-dependent perfusion*,
   BioMed Eng OnLine 3:42 (2004), `doi:10.1186/1475-925x-3-42`, the `bloodstain::burn` model
   (`docs/bloodstain-citations.md` lists every constant taken from each). Both are explicit schemes at a fixed
   step, which is what makes a reproducible state possible: nothing in either needs a clock.
2. **Horror-film reference.** None: this item changes no frame (the claim is measured, `md5` of every frame
   of every recorder variant before and after, `/tmp/c5/`). The visible references that bear on what the
   folded buffers hold are the ones `docs/research/spray-reach.md` gathered; this note adds none.
3. **Technique** (how a shipping game keeps a simulation honest): Bettner & Terrano, *1500 Archers on a
   28.8: Network Programming in Age of Empires and Beyond*, Gamasutra / GDC 2001,
   `https://www.gamedeveloper.com/programming/1500-archers-on-a-28-8-network-programming-in-age-of-empires-and-beyond`;
   and Fiedler, *Deterministic Lockstep*, Gaffer On Games,
   `https://gafferongames.com/post/deterministic_lockstep/`. Neither is in the corpus (web pages with no
   DOI); both were fetched and quoted this pass. No shipping game has written up a determinism check of its
   *gore*; the nearest published practice is the lockstep game's per-tick checksum of its simulation state,
   which is the same idea: compare a fold of the state authority across two runs.

## Craft principle

The models folded here are deterministic by construction and the crate's claim is that the whole
product is. A claim that is checked only where it was thought of is a claim about the places it was
thought of. The physics half: Stam et al. run their bruise as "a forward difference method in space", and
state: "For a time step of 0.1 h, a simulation of 400 h takes 1 min on a standard laptop computer", so the
state after `n` steps is a pure function of the inputs and `n`, and the crate steps by an integer count
(`CLAUDE.md`, "Ticks, not seconds ... And the one exception"). `bloodstain::burn` is the same at `0.02 s`
(`src/bloodstain/burn.rs`, `DT_S`). What a bruise *looks like* is that state written into a texel buffer by
`DermisCanvas::draw_bruises`, so the texel buffer is where a run-dependent bruise would show first; it is
the CPU authority and the GPU copy is write-only (`CLAUDE.md`, `wetmap` and `flaymap`: "`digest()` folds the
buffer, never the images, and that is not an implementation detail: it is the claim").

## How shipping games do it

Bettner & Terrano on what a checksum of a simulation is for, and where it fails: "As much as we
check-summed the world, the objects, the pathfinding, targeting and every other system -- it seemed that
there was always one more thing that slipped just under the radar." Their lesson is the one this item
applies: the set of things folded has to be audited against the set of things the simulation owns, because
the bug lives in whatever the checksum left out; "very subtle differences would multiply over time" and
"what showed up as a checksum difference as different food amounts had a cause that was sometimes puzzling
to trace back". Fiedler gives the standard the fold is held to: "Exact down to the bit-level. So exact,
you could take a checksum of your entire physics state at the end of each frame and it would be identical."

Applied here: the recorder line is this crate's per-run checksum of "the whole simulation", and three of the
four canvases a dressed skin owns (wet, flay, dermis, and a cloth's holes) were never in it, or were in it
only when something else happened to move.

## Model adopted

- `DermisCanvas::digest()`: FNV-1a (`0xcbf29ce484222325`, prime `0x100000001b3`) over the RGBA8 buffer,
  row-major, in the form `ClothHoles::digest` already had (which now delegates to it: one path). Over the
  buffer, not the image. The bruise-free `under` copy is not folded separately: it is what the buffer is
  rebuilt from at every redraw, so a difference in it is a difference in the buffer.
- The recorders keep `digest=` bit-for-bit (the same sorted XOR-then-multiply fold, now `digest::fold` in
  `examples/common/digest.rs`, read once as the clip ends) and append `flay=<hex>` (every
  `FlayCanvas::digest`) and `dermis=<hex>` (every `DermisCanvas::digest`). Each field is the sorted fold of
  its canvases, **sampled once every frame and chained** (`digest::SkinTrace`, `(acc ^ frame) * prime`), not
  read once at the end as the issue proposed: see "What the first build found" below. The documented
  `digest=` values in `README.md` and `docs/DEMOS.md` therefore hold, and `tools/qa.sh`'s `DIGEST_RE`
  (`digest|^[a-z_]+: frames=...`) already matches the longer line.
- A cloth's holes stay in `digest=` (when holed) and are not repeated in `dermis=`, which is the
  `DermisCanvas` *component* only: the cloth's canvas is not on the skin.

## Parameters

None tuned. The two FNV constants are the FNV-1a 64-bit offset basis and prime, the ones every other digest
in the crate uses (`wetmap`, `flaymap`, `ClothHoles`, the recorders).

## Validation targets

1. **Fails before, passes after.** The unit test `the_dermis_digest_sees_where_a_bruise_fell_and_how_a_burn_went`
   (`src/preset.rs`) does not compile before the change (`DermisCanvas::digest` did not exist) and passes
   after: a blow and a burn each move a limb's dermis digest off a fresh one's; the same hit in a second app
   gives the same number; a blow a hand's breadth away, and an iron that burned to another degree (150 °C for
   1 s is third degree, 55 °C for 30 s second), give other numbers. An iron held three times as long at
   150 °C does **not** (the first run of the test found it): the degree decides which zones the dermis shows,
   and both are third degree, so the buffer is the same one and the digest rightly agrees.
2. **The acceptance on the recorder.** `capture_skinned` and `capture_skinned --blows`: `digest=` equal
   (`9677d79de848daaa`), `dermis=` different, two runs of each print the same line.
3. **No documented digest moves.** Every recorder variant's `digest=` after equals its `digest=` before
   (`preset` `8a10edd6f39c51d8`, the eight skinned variants as `docs/loop-log.md` iteration 4 recorded them,
   re-measured this pass), and every frame is byte-identical (`md5` per frame).
4. **The gate can fail.** A throwaway perturbation of the script moves `dermis=` and leaves `digest=`: the
   shin blow of `capture_skinned` placed 5 mm further along the limb; and one of a crater's depth (9.0 mm to
   9.1 mm) moves `flay=`. Reverted after; the measured lines are in the log.
5. **A breaking body is seen.** `capture_preset` and `capture_skinned --break` print a `flay=` and a `dermis=`
   that are not `cbf29ce484222325` (the empty fold, which is what a fold of the world as the clip ends reads,
   because the body has broken and no skin canvas is left).
6. **Locks and cost.** Every golden in the loop's section 3 passes untouched; `bench_carnage`'s digest
   `0x4366a8b64352c85a` and its timing are unchanged (the library gains one method no hot path calls).

## What the first build found

The issue proposes "each a sorted FNV fold over that component across the world, the same way the wet fold is
done", read once as the clip ends. Built that way and run, `capture_preset` printed
`flay=cbf29ce484222325 dermis=cbf29ce484222325` and so did `capture_skinned --break`: the FNV offset basis, the
fold of nothing. In both clips the body breaks (frame 120 of the preset, 200 of `--break`), is despawned, and
its pieces carry no `FlayCanvas` or `DermisCanvas`; at the end of the clip no skin canvas is left to fold.
A field that reads one constant whatever the craters and bruises were is the blind spot again under another
name, and it is exactly Bettner & Terrano's "one more thing that slipped just under the radar". The fix is
to sample the canvases while they exist: once a frame, chained. That hashes about 0.6 MB a frame (an estimate:
six skins at the shipped 128, a 32 KiB depth buffer and a 64 KiB dermis each) in a recorder that also writes a
PNG a frame, and measured no change in wall time (17.4 and 17.1 s before, 17.0 and 17.8 s after). `digest=` should have the same property
for the body's wetmaps once the body is gone (`[INFERENCE]`: the body is despawned, so its `WetCanvas` goes
with it; the count of `WetCanvas` at the end of the clip was not measured this pass). It is not changed here
because every documented value of it would move, and is recorded in the log as the next item.

## Deviations

- **The command file's checklist is the owner's to edit** (`.omp/commands/build-loop.md`, "Teach the loop's
  checklist to compare all three fields"). This pass does not edit it; it says so on #8 and the loop's own
  gate compares all three fields from here (`docs/loop-log.md`).
- **`flay=` and `dermis=` are sampled every frame, where #8 proposes one fold at the end.** The end fold is
  the empty fold for both clips that were run before the fix (the preset and `--break`; the run was stopped at
  the third). The issue's own acceptance (`--blows` against the default, `digest=` held) is met either way.
- **`dermis=` folds the `DermisCanvas` component, not a cloth's.** Recorded above; a cloth's holes keep the
  `digest=` slot they always had so that no documented digest moves.
- A shipping game's gore determinism check is not published; the lockstep checksum is the nearest source and
  is cited as such, not as a gore pipeline.

## Pending index

Not in the corpus, cited by URL: Bettner & Terrano 2001 and Fiedler's *Deterministic Lockstep* (web pages,
no DOI, fetched and quoted this pass). `distill_search` was healthy and answered for Stam et al.; the
Gowrishankar paper's time-step text did not extract legibly (`docs/bloodstain-citations.md` already says
which of its numbers resolve), so its fixed step is cited from `src/bloodstain/burn.rs`.

## What great looks like

A pass that makes a crater's depth, a bruise's place or a burn's zones run-dependent turns a recorder's
line red in a field named for what moved, and a pass that did not mean to touch them leaves all three
fields as the baseline printed. Nothing in a frame changes: this is the gate, not the picture.
