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

No model change. Three exact reductions, each proved against the thing it replaces, in the order of what they cost:

1. **`rasterise`: a bucketed bound on the rim.** The circle of angles is cut into 64 buckets. For each, the spines whose window
   (`arg >= -17`, the existing skip) can reach it, as a bit set, and an upper bound on `rim` there: `1 +` the sum over those spines
   of `reach * exp(-gap^2 / 2w^2)`, `gap` the least angular distance from the spine to the bucket, padded so float error cannot
   cross it. A sample outside the circle of the largest bound is zero without an `atan2`; one outside its bucket's bound is zero
   without a spine; the rest walk only the spines whose bit is set, in the same ascending order, with the same arithmetic. Satellites
   are tested on the squared distance first, with a margin the float error cannot cross. Bytes identical:
   `rasterise_writes_the_mask_its_definition_does` already holds `rasterise` to a by-definition oracle over 3 000 shapes and is
   widened to the ceiling of spines.
2. **`shade`: a table of `(level, age)`.** The albedo and roughness a `(fill level, age)` pair gets is a pure function of the
   canvas's settings, so a canvas keeps them in a small direct-mapped table and the one-slot memo stops thrashing along a rim.
   The age's wetness byte, a `u64` division a texel, is memoised by age. Identical bytes by construction; the same pure function.
3. **The passes' inner loops** (`spread`, `drip`, `advance`, the re-measure): row slices rather than a `get` and a bounds branch
   per neighbour, the lean's `across` hoisted out of the texel, the cumulative absorption by age tabulated, an empty texel with
   four empty neighbours skipped. The conservation arguments in `CLAUDE.md` (`wetmap`) do not change and the tests that defend
   them are the proof.

## Parameters

`RIM_BUCKETS = 64` (a design default: the bound's looseness against its tables' cost, to be settled by measurement in the log).
`BUCKET_PAD = 1e-3` rad, `WINDOW_PAD = 1e-3` relative and `RIM_PAD = 1e-4` relative: each three
orders over float error and below anything that changes a cull, so none moves a byte; the tests are what say so. No behaviour dial.

## Validation targets

- Every recorder variant's digest equal to the baseline's: `preset` `d4c0bcda72414525`, default `e19726fb05ac40f3`, `--through`
  `4da541caaf98793b`, `--guard` `a6d89deea27fa717`, `--shirt` `bfb01ebc2acae68b`, `--break` `6b95aab1742db9fa`, `--stroll`
  `8ebe761bcc8b4d83`, `--quiet` `f83a631a7426808d`, and `flay=` and `dermis=` unchanged (iteration 6's log). Frames pixel-identical.
- Every golden and `tests/wetmap.rs` untouched; the lib's 356 tests.
- `hit_tick_cost`: a heavy wound facing the wall from 25 to 29 ms toward the frame budget; the ticks after the hit down by the
  share the passes cost; no case slower.

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
