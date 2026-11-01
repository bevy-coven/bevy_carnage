# What a wounded scene costs a frame: the stamp and the canvas tick

Iteration 7 (2026-10-31). Item: the cost of a heavy wound on a wall, and of every tick after it, **with every byte unchanged**.
Ranked first because a hit that spikes the frame is a stated failure of this crate and the loop's own measure
(`preset::tests::hit_tick_cost`, iteration 6's backlog item 1) read 28 ms for it. Runner-up: the few-texel drop's look in
`flesh.wesl` (a look, not a cost; it needs a WebGL2 re-check this host cannot make quickly).

## Diagnosis (measured before any change)

`cargo test --release --lib -- hit_tick_cost --ignored --nocapture`, big_mac, load 3.5 to 5, probes added in a scratch copy
(removed before the commit). A tick is `FixedUpdate` of the preset's chain in a room: a body, a floor, optionally a wall 0.9 m
behind the wound; 45 hits one tick apart; then 90 ticks with no hit.

- **The hit is not where the time goes.** Per-system spans (a marker system between each pair of the chain, ms a tick): a
  bullet on a floor: `take_hits` 0.58, `tick_canvases` 3.08, the rest under 0.3; a bullet facing the wall 1.60 and 4.30; a heavy
  wound (8 cm) facing the wall **12.5 and 18.3**. The canvases' own tick is the larger share everywhere.
- **It does not stop when the shooting does.** The 90 ticks after the last hit, no new blood: a bullet scene **3.4 to 4.8 ms a
  tick**, a heavy wound facing a wall **26 ms a tick**, for as long as the blood is wet (`dry_ticks`, hundreds of ticks). That is
  every frame of a gore-heavy scene, not one spike.
- **The stamp.** A heavy wound facing the wall throws 49 spined stamps a hit (mean 4.4 texels, but 22 and more for the largest) and
  `rasterise` takes **10.5 ms a hit at 430 ns a sample**; a stamp on the floor is 115 to 165 ns a sample. The sample cost is the
  spine loop: a droplet at wall speed has the Weber number of 20 000 and 24 spines (`SPINE_MAX`), every sample walks all 24 and
  calls `exp` for the four or five whose window it is in, and `atan2` for all of them; and `reach_bound`, the cull in front of it,
  is `1 + the sum of every spine's reach`, about 10, so it culls nothing.
- **The tick's passes** (ms a canvas tick, `floor2_wall_big`): `spread` 2.7, `shade` 3.0, `drip` 1.0, `advance` 0.5, the
  re-measure 0.43, `covered` 0.2, for 45 000 covered texels of which 15 000 hold blood: 175 ns a covered texel where a four-neighbour
  stencil is a few. `shade` memoises one `(level, age)` and a stamp's rim runs through all sixteen levels, so it recomputes
  `appearance_with_fresh` (and its `dry_ticks` power) at nearly every rim texel.

The `sample` tool could not attribute this (about 15 samples a second on this host), so the numbers are the crate's own
`Instant` probes.

## Sources

- **Physics** (what the bytes encode, and so what may not move): Comiskey, Yarin & Attinger 2018,
  `doi:10.1103/physrevfluids.3.063901` (read through `distill_search`); Knock & Davison 2007,
  `doi:10.1111/j.1556-4029.2007.00505.x` (the spine count, through `docs/bloodstain-citations.md`); the oxidation walk
  `shade` ages a texel along, Bremmer et al. 2012 and the biphasic study `doi:10.1371/journal.pone.0021845`.
- **Horror-film and forensic references** (the look this must not disturb): see `## Reference set`.
- **Technique**: Akenine-Moller, Haines & Hoffman, *Real-Time Rendering* 4th ed. (stem `book_real_time_rendering_4th_akenine`, p. 166);
  Haines & Akenine-Moller (eds.), *Ray Tracing Gems* (2019), ch. 22 (`doi:10.1007/978-1-4842-4427-2`); *Advances in Real-Time
  Rendering* SIGGRAPH 2010 (stem `s2010-advances-pdf-slides-5`); Geffroy, *Rendering of Doom Eternal*, SIGGRAPH 2020 (cited in
  `docs/research/floor-spatter.md`).

## Craft principle

The model is not touched, so the principle is the one that lets a cost fall without a byte moving: **spend a sample only where
the answer can differ.** *Ray Tracing Gems* ch. 22 states it for a renderer: "The core idea is to run the base case of TAA on
most pixels and then ... output a conservative segmentation mask identifying where TAA fails and why", so that "we can
adaptively employ 8x ray traced supersampling for 6% of the total image resolution at a cost of fewer than 0.5 rays per pixel.
Image quality is then comparable to at least 8x supersampling everywhere". The word that carries it is *conservative*: a mask
that may spend a sample it did not need and may never skip one it did. Here the mask is a bound, and a bound is exact: a
sample outside an upper bound on the stain's rim is zero in the original, so skipping it changes nothing.

The physics keeps its own claim on the answer: "Other than that, droplet motion is fully deterministic, and Figs. 7 and 9
represent the fully deterministic predictions" (Comiskey et al. 2018). A stain here is a pure function of its shape, and a
digest folds the canvas; a cost change that moved one byte would be a model change and would move every recorder.

## How shipping games do it

Doom Eternal draws its blood as mesh decals: "extremely fast and allows artists to go crazy with massive amounts of decals", and
pays for that by a cap, "the maximum amount of decals per mesh would theoretically be 255" (Geffroy 2020; Coenen's study). The
frame is protected by a number. This crate's canvases have no per-droplet draw and no such cap, so the protection has to be in the
work per droplet and per live texel, and the budget the caller sees is the tick. The SIGGRAPH 2010 hybrid anti-aliasing talk
puts the same trade as where the samples go: "the supersampling is required mostly only for distant objects, where the GPU fails
to approximate shading/mipmapping/visibility per pixel"; a stamp supersampled four ways in each texel is the near object that
needs it only at its rim, and skips it inside and outside.

## Model adopted

No model change. Five exact reductions, each held to the thing it replaces, in the order they were made:

1. **`rasterise`: a bucketed bound on the rim.** The circle of angles is cut into 32 buckets. For each, the spines whose window
   (`arg >= -17`, the existing skip) can reach it, as a bit set, and an upper bound on `rim` there: `1 +` the sum over those spines
   of `reach * exp(-gap^2 / 2w^2)`, `gap` the least angular distance from the spine (or its copy a turn either way) to the bucket,
   padded so float error cannot cross it. A texel outside the circle of the largest bound is zero without an `atan2`; one outside
   its bucket's bound is zero without a spine; the rest walk only the spines whose bit is set, in the same ascending order, with
   the same arithmetic. A texel outside a satellite's disc by a margin skips its square root. Bytes identical:
   `rasterise_writes_the_mask_its_definition_does` already held `rasterise` to a by-definition oracle over 3 000 shapes; a second
   test holds it there at the ceiling of 24 spines and 24 satellites and over 2 000 impacts `stain_shape` itself makes.
2. **`shade`: a table on `(level, age)`.** What a `(film level, age)` pair looks like is a pure function of the canvas's settings,
   so a canvas keeps it in a 1024-slot direct-mapped table, between ticks, and the one-slot memo that thrashed along every rim
   goes. The wetness byte, a `u64` division a texel, is memoised by age.
3. **`spread` and `advance`.** The flux across an edge is `round(k * (a_i - a_j))` of two bytes, one of 511 values, read from a
   table; the substrate's soak by age is a byte table of the drying span.
4. **Bare texels are skipped.** Most of what a tick covers holds no blood (three covered texels in four on a wall). A bare texel
   with four bare neighbours stays `(0, 0)` through `spread`; a bare texel whose `R` byte in the metallic-roughness image is
   already zero is already drawn bare (every write of that byte draws the whole texel) and `shade` leaves it.
5. **What a tick covers is the blood, not the tiles it fell in.** The live tiles kept the bounding rectangle of the blood in each
   32-texel tile, which for a spray of fine drops was most of the tile round each: 9 388 texels covered for 1 144 with blood
   (`a_spray_is_covered_by_its_own_blood_and_not_by_its_tiles`, which fails on 0.8.0 and passes now at under 3 240). A tick now
   re-measures the blood as a rectangle one row high for each stretch of a row that holds any (`SPAN_GAP`, within seven texels is
   one stretch), and covers those and the tiles painted since, each grown by the reach of one tick, swept by row. The passes are
   paid per texel covered, so this is the cost.
6. **The check that they are what they say**: `the_passes_are_what_their_definitions_say` copies the pre-table `spread` and
   `advance` into the test, writes `shade` by its definition (`appearance_with_fresh` asked afresh at every texel), and compares
   `wet` and every pixel of both images to them on every tick of four scripted runs (soak at 0 and 0.9, a short drying span, a
   spread at its ceiling); `the_live_tiles_change_nothing_but_the_cost` holds the covered tick to the whole-canvas one.

## Levers measured and not taken (each moves pixels, so each is its own item)

- **Where the stamp's time is.** After the exact reductions a spined stamp costs about 8 ns a sample for each spine whose window
  holds the texel: 24 spines 200 to 260 ns a sample, 12 spines 100, none 27 (`rasterise`, fine mask of 20 and 96 texels, best of 25
  batches); the satellites cost 15 ns. One heavy wound facing a wall is 49 stamps (`STAMPS by px`, one fresh hit): 16 of two texels
  (0.8 ms together), 7 of three, 5 of four, 20 of five (4.6 ms, the first of them 1.5 ms cold) and one of 24 texels (0.8 ms): the cost
  is the many small stamps, each 25 to 150 us, of which the table is 10 to 19, and not the one big one.
- **A sample budget by stamp size** (`edge_samples` 4 to 2 past a stamp of about 14 texels): coverage of a spined stamp, 4 samples
  against 3, 2 and 1 on a 512 canvas. Stamp of 26 texels (6 mm at 25 m/s): mean difference 0.35, 1.24 and 7.3 of 255 a texel,
  worst 4, 13 and 66; of 16: 0.74, 2.4 and 14.7, worst 9, 17 and 95; of 8 (3 mm at 12 m/s): 2.3, 8.2 and 30. A budget of 2 on
  stamps of 14 texels and over is invisible to a mean of 1 to 2.4 of 255 and saves three quarters of the work of the stamps it
  touches, which in a heavy hit is one in 49: it does not move the hit by itself; below 14 texels the difference is not
  invisible, and `tests/wetmap.rs` pins four samples at 32 texels.
- **Spines a canvas cannot resolve** are the most expensive part of a wall stamp and cannot be seen: at 5 mm a texel a
  stamp of five texels has spines under a texel wide. But the body is shrunk to make room for them (`extent`), so drawing none
  makes the body 2.5 times the area: not a free saving. A stamp's mass has to be kept by re-sizing, and judged against Ref 1.

## Parameters

`RIM_BUCKETS = 32`: a design default. At 16, 32 and 64 the cost of a big stamp was the same inside the noise of a shared host
(a fine mask of 96 texels: 2.5, 2.2 and 2.2 ms), and the table a stamp builds fell from 37 us at 64 to 10 to 19 at 32, which is
most of what a stamp of two texels costs. `BUCKET_PAD = 1e-3` rad, `WINDOW_PAD = 1e-3` relative and `RIM_PAD = 1e-4` relative:
each orders over float error and below anything that changes a cull, so none moves a byte; the tests are what say so.
`SHADE_SLOTS = 1024` (8 KiB a canvas that has shaded blood). No behaviour dial.

## Validation targets, and what was measured

- **Every recorder variant's digest equal to the baseline's, and every frame**: held. 18 runs (9 variants twice), digests equal
  pairwise and to 0.8.0's: `preset` `d4c0bcda72414525`, default `e19726fb05ac40f3`, `--through` `4da541caaf98793b`, `--guard`
  `a6d89deea27fa717`, `--shirt` `bfb01ebc2acae68b`, `--break` `6b95aab1742db9fa`, `--blows` `e19726fb05ac40f3`, `--stroll`
  `8ebe761bcc8b4d83`, `--quiet` `f83a631a7426808d`, with `flay=` and `dermis=` unchanged; and `cmp` on all 6 480 frames, none
  different. `canvas_digest`, `dry_timeline` and `stain_sweep` print byte-identical output; `bench_carnage`'s digest is
  `0x4366a8b64352c85a`, as before.
- **Every golden and `tests/wetmap.rs` untouched**: the gate's `cargo test --no-fail-fast` is green (lib 359 passed and three
  ignored in the dev profile, `tests/wetmap.rs` 17, `leaf` 3, `readme` 4, every other binary and the doctests), and the three new
  tests (`rasterise_writes_the_mask_its_definition_does_at_the_ceiling_and_for_real_impacts`,
  `the_passes_are_what_their_definitions_say`, `a_spray_is_covered_by_its_own_blood_and_not_by_its_tiles`) pass; the last fails
  on 0.8.0 (9 388 texels covered for 1 144 with blood, bound 3 240).
- **`hit_tick_cost`**, 8 runs alternated with 0.8.0 on a host at load 3 to 5 (a single run varies by two; the median of 8, ms,
  0.8.0 to now): hit tick of a bullet on a floor 4.51 to 3.04, facing a wall 6.86 to 4.82, a heavy wound 8.26 to 4.98, a heavy
  wound facing a wall 28.51 to 18.68, the same without the floor's mist 28.81 to 18.12; **the tick after the last hit** 4.80 to
  2.79, 9.39 to 5.66, 5.56 to 2.84, 26.52 to 15.35. No case slower. The whole run retires 30.47 billion instructions
  (`/usr/bin/time -l`, stable to 0.05 % between runs where wall time is not) before, and 18.03 billion now: 41 % fewer.
- **`bench_carnage`** does not call the preset or a canvas: 15.75 billion instructions before and 15.75 to 15.80 now.

### Critique and verdict

Frame 359 of `capture_skinned --through` beside Ref 1 (a wall) and Ref 7 (a floor fan), `compare-wall-spray.png` under `/tmp/c7`.
The look is exactly 0.8.0's. Against Ref 1 the wall stipple is dozens of near-round blobs of one narrow size band (about a
centimetre), almost no fine specks and no tails or rays; Ref 1's larger stains throw spines, tails and satellites, and run from a
millimetre to three centimetres. **The spines exist in the mask (24 at wall speed) and are under a texel wide at 5 mm a texel,
so they do not read: the most expensive part of a wall stamp is the part nobody can see.** Against Ref 7 the floor's drops are
finer than the wall's. None of this is moved by this item.

- Exactness: **GREAT** (digests, frames, the mask and the passes by definition).
- Cost, as delivered: **GREAT** for what it is: the same bytes in 0.6 of the ticks and 0.59 of the instructions, with a test that
  fails on 0.8.0 for the one structural cause (the tiles). As an absolute, **NOT YET**: a heavy wound facing a wall is still
  about 19 ms of its hit tick and 15 ms of every tick after, and what is left is not exact: 49 spined stamps at 25 to 150 us, and
  the passes' per-texel cost. The changes that would move it to GREAT each move pixels and are their own items: stamp the
  spines a canvas can resolve and keep the body's mass; spend the sample budget by stamp size; or stamp each droplet at the tick it
  arrives (0.9 m at 8 to 40 m/s is one to seven ticks), which spreads the spike without changing a stamp.
- The wall stipple's look beside Ref 1: **NOT YET**, unchanged (backlog).

## Deviations

None to a default. The tick's passes keep their documented order and their snapshot discipline.

## Pending index

Nothing unindexed. The oxidation walk's two papers are in the corpus; no new source was needed.

## What great looks like

A heavy wound on a wall is one frame like any other (a few milliseconds), the blood beside it is the same spined stain, and the
scene after it ticks for the cost of the blood that is actually wet.

## Reference set

The pixels do not change, so the references are the ones the look was judged against; cited by URL, kept under
`/tmp/c7/refs/`, never committed:

1. `https://commons.wikimedia.org/wiki/File:Blood_Spatter_Texture_-_FREE.jpg` (CC BY 2.0): a wall, 220 stains, spines on the larger.
2. `https://commons.wikimedia.org/wiki/File:Blood,_guts_and_gore_galore,_Moulage_crew_brings_horror_to_life_140723-A-IX584-092.jpg`
   (US Army, public domain): a fan of 111 stains in 45 cm.
3. *Evil Dead* (2013), KNB EFX, methylcellulose blood for a "realistic splatter":
   `https://filmhounds.co.uk/2023/04/17/gory-groovy-the-splatter-of-evil-dead/`.
