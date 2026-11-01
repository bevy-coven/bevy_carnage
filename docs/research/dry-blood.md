# Blood dries dark and stays where it dried

Iteration 9 (2026-11-01). Every recorder in this repository ends within seconds of the last shot, and blood here takes
`dry_ticks` (1 800, 30 s at 60 Hz) to dry, so **no committed clip, frame or test has ever looked at dried blood on a surface
the flesh material draws**. Looked at, with the new `capture_skinned --dry 120` (blood dries in 120 ticks, so frame 359 is
fully dried), the preset's blood is wrong in two ways, each measured below before any change:

1. **Dried spatter is erased.** The substrate's soak takes up to 38 bytes of a texel's film (`absorbency`, 0.15 of 255) over the
   drying, from every texel, so a spatter drop's thin rim goes first and a drop a few texels across goes entirely. The wall of
   `capture_skinned --through --floor 512`, 6 s after the shots and again once dried: **9 242 px of blood in 176 drops, then 4 508 px
   in 94**. At the canvas level (`tests/zz` probe, since removed; a cone is what `paint_drop` writes under eight texels): after
   1 800 ticks a drop of radius 1.5 texels or less is gone, one of radius 2 keeps 1 of 11 texels over the draw edge, radius 3 keeps
   6 of 27, radius 4 keeps 19 of 47. At 512 texels over 3 m (5.9 mm a texel) that is **every drop a wound throws at a wall**.
2. **Dried blood is drawn the colour of fresh blood.** `flesh.wesl` composites the film from its *thickness* alone (`blood_over(base,
   depth_mm)`, the blood table by depth); the age drives only the clear coat's gloss (`wetness`). The CPU does age the colour (the
   canvas's albedo image dries from `(113, 33, 42)` to `(49, 21, 15)` for a thick film, `bloodstain::dry::appearance_with_fresh`), but
   the flesh material does not read that image, so the preset's blood never turns brown. Measured off the same frames, wet and
   dried: the floor pool `(119, 53, 59)` and `(124, 63, 68)`, the chest wound `(113, 43, 48)` and `(116, 45, 49)`, the wall's drops
   `(142, 61, 62)` and `(148, 65, 63)`: the same red to within the lighting.

A horror film's aftermath is the opposite of both: dried blood is **there** (a room the day after, Ref N) and it is **brown to black**
(Ref Q). This note fixes both in one item because either alone is wrong: an erased stain has no colour, and a persistent one drawn
fresh-red is paint.

## Sources

1. **Physics, the footprint.** Smith, Nicloux & Brutin, *A new forensic tool to date human blood pools*, Sci. Rep. 10 (2020),
   `doi:10.1038/s41598-020-65465-4` (in the corpus, `distill_search`, and already cited by `bloodstain::dry` for the rim front).
2. **Physics, the colour.** Bremmer, de Bruin, de Joode, Buma, van Leeuwen & Aalders, *Biphasic oxidation of oxy-hemoglobin in
   bloodstains*, PLoS ONE 6 (2011), `doi:10.1371/journal.pone.0021845` (in the corpus; the walk `bloodstain::dry` implements).
3. **Horror-film and forensic reference** (`## Reference set`): Ref Q (dried blood on a tile wall) and Ref N (a police photograph of
   a floor with dried blood, 1950s), plus the crate's own Ref 1 for the spatter's count.
4. **Technique.** Coenen, *DOOM Eternal - Graphics Study* (<https://simoncoenen.com/blog/programming/graphics/DoomEternalStudy>), on
   the decals the game keeps: "the decal count can go up to thousands of them". And this crate's own constraint from `CLAUDE.md`: the
   CPU canvas is the authority, the GPU is write-only, nothing in a shader may be needed by a hash.

## Craft principle

The model source for a stain's footprint, quoted from the corpus (`distill_search`, `10.1038_s41598-020-65465-4`):

> "The initial mass loss for water or blood pools is linear until it reaches approximately 60% of mass loss. At this stage, the pool is
> pinned to the surface and the liquid phase evaporates at the surface. The volume loss corresponds then to a decrease in height."

A drying pool or drop **loses height, not area**: its contact line is pinned, so what is left is a thinner film over the same
footprint. The substrate's soak in this crate took the *footprint* (a coverage byte of 0.04 or less is not drawn), which is the
opposite. And for the colour (`10.1371_journal.pone.0021845`, abstract):

> "Upon exiting the body, bloodstains transit from bright red to dark brown, which is attributed to oxidation of oxy-hemoglobin..."

## How shipping games do it

Games keep their blood. Coenen's study of *DOOM Eternal* (and *DOOM* 2016 before it): "decals are of great importance in the game.
In Doom 2016, this was already the case and the decal count can go up to thousands of them", culled into froxels so that a
thousand stains cost a lookup each. A decal does not fade because it is thin; it is removed by a budget, by the caller. That is
this crate's boundary exactly: a canvas never forgets blood, and a game that wants a stain gone sends a `GoreReset`. The drying
of a stain (gloss to matte, red to brown) is a *material* state, not a coverage one.

## Model adopted

Two changes, one for each defect, and no new dial.

**1. The soak takes the film and never the stain** (`wetmap::STAIN_FLOOR`, 33 of 255). `advance` still applies the schedule
`absorbed_by(age)` to every wet texel, but a texel never goes under `min(amount, STAIN_FLOOR)`: the substrate takes what is above a
stain's own depth and leaves the stain. 33 is **one cone step above the draw edge** (`DETAIL_EDGE` 10 of 255 plus `CONE_SLOPE`'s 23),
which is why it is exact for a drop: every texel of a cone is within one step of its neighbours, so levelling a cone's inside to 33
keeps the ring at the edge (values 10 to 33) as painted, moves the contour by under a tenth of a texel, and creates no difference of
25 or more for the spread pass to creep on. A pool's 255 still goes to 217 (its 15 % is untouched); only the thin things stop
dissolving. A texel already under the floor is not soaked at all, so a faint smear stays faint instead of vanishing: also the
physics (pigment mass does not leave).

**2. The flesh material ages the film's colour** from the wetness byte it already reads. The CPU walk
(`bloodstain::dry::appearance_with_fresh`: fresh to methaemoglobin at a third of the drying, to hemichrome at the end, each scaled
by the film's own red so a thin film stays lighter) is baked **at its two knots** into new rows of the flesh tables
(`flesh::aged_blood_lut`, below the blood table, rows `BLOOD_ROW0 + 4 ..`), and the shader interpolates between the stops by
`1 - wetness`, in encoded sRGB, exactly as `appearance_with_fresh` does, then decodes. The age is read from the **unsmoothed** wet map,
averaged over the four by four texels the spline reads, **by the spline's own weights and counting only the texels that have blood
in them** (`R > 0`; a dried texel has blood and no wetness): the smoothed wetness falls to zero at the rim of any drop (the
bare texels beside it carry none), which would draw a brown ring on every fresh one, and the youngest-of-nine that was tried first drew
the texel boundaries between two ages as hard lines. One spectral model, as for the fresh table;
no texture, sampler, uniform or binding is added, so the WebGL2 limits `CLAUDE.md` keeps are as they were.

## Parameters

| Parameter | Value | Provenance |
|---|---|---|
| `STAIN_FLOOR` | 33 of 255 | **Tuned, and says so.** `DETAIL_EDGE` (10) + `CONE_SLOPE` (23): the least a texel holds and still keeps a cone's rim where it was drawn. |
| aged stops | `t = 0.35` and `1.0` | Not new: the knots of `appearance_with_fresh` (Bremmer 2011/2012: oxidation is mostly the early, fast phase). |
| age read | spline-weighted mean of the `A` byte over texels with blood, 4 by 4 | **Design default.** The smoothed byte would ring every fresh drop; the youngest of nine was blocky. |
| new rows | 8, `BLOOD_ROW0 + 4 ..= + 11` | Own layout; `lut_digest` (subsurface and blood tables) is unchanged, the new rows have a golden of their own. |

## Validation targets

1. **A drop keeps its footprint.** The area a reader draws at `DETAIL_EDGE` (the `drawn` measure in `tests/wetmap.rs`) of a cone of
   radius 1 to 4 texels after a full drying with the shipped substrate: within 10 % of what was painted (0.8.2: 0 at radius 1.5, a
   tenth at radius 2). Fails on 0.8.2.
2. **A pool is still soaked.** A full-coverage texel ends at `255 - 38`; `wetted_area` falls as `tests/wetmap.rs` already asserts.
3. **Mass.** The drip and spread passes conserve every byte (`tests/wetmap.rs`), untouched; the soak's own loss is bounded by the schedule
   and by the floor, and a texel under the floor loses nothing.
4. **Nothing else moves.** `the_flesh_tables_are_frozen`, `hash_f32_is_frozen` and the spatter, stain, pool, bruise, burn, wick, tear,
   strip and scripted-wound goldens pass untouched; `bench_carnage`'s digest is unmoved (it never calls a canvas).
5. **The colour walk is the CPU's.** A baked stop equals `appearance_with_fresh` at its age, to the byte, for every row and column; the
   aged rows have a golden.
6. **Reproducibility.** Every recorder digest moves where blood soaks (on purpose), twice-equal; `--quiet` and `--stroll` keep theirs
   if no soaking blood is in them.
7. **Look**, against Refs Q and N: wall drops that survive the drying, at the count they had; a dried pool and wound dark brown and
   matte, thin edges lighter than thick cores; no brown ring on a fresh drop.
8. **WebGL2**: `flesh.wesl` re-checked in a browser (headless Chrome over SwiftShader, the `carnage_web` example with `webgl2`).
9. **Cost**: `hit_tick_cost` and `bench_carnage` no slower; the shader's 16 extra reads (and 8 extra table taps, 4 to 8 a blood pixel) are for blood pixels only.

## Deviations

- **An old behaviour is gone, deliberately.** "Faint spatter soaks away to nothing" was the design (`WetSettings::absorbency`'s doc).
  It is replaced by "a stain keeps its footprint". There is no setting that restores it (one path per feature); a caller that
  wants a stain gone resets the canvas.
- **The substrate no longer takes thin films at all.** A texel under 33 of 255 is never soaked. A caller's film of 5 of 255 (a smear)
  persisted forever where it used to vanish; that is the pinned-footprint physics, not a bug.
- **Every recorder digest where blood soaks moves** (the bytes are different); the flesh tables' golden does not.
- **The shader reads the wet map unsmoothed (`textureLoad`), 16 times a blood pixel.** Marked a design default.
- **The cut-film (`blood_over` over exposed tissue) is not aged**: it has no age (a crater's graze is wet with its own blood by
  construction).

## Pending index

- Brutin, Sobac, Loquet & Sampol, *Pattern formation in drying drops of blood*, J. Fluid Mech. 667 (2011) (the coffee-ring, the pinned
  contact line of a *drop*, and the final deposit's diameter): cited from memory by the forensic literature, **not in the corpus**
  and not read this pass; the pool paper above is the quote.
- Stauffer & Aharony (size spread of spatter, iteration 8's backlog item 1): still not in the corpus; unrelated to this item.

## What great looks like

A room the day after the fight. On the wall behind the body the same field of drops the shot threw, all of them, at the sizes they
landed, but **dark red-brown to black and matte** where they are thick and a lighter amber where they are thin (Ref Q); the pool
on the floor has gone the same dark brown, with a rim, and no gloss; the wound on the body is a dried dark crust, not a wet
one. A fresh drop beside an old one is visibly a different colour, and no fresh drop has a brown ring.

## Reference set

Kept under `/tmp/c9/refs/`, never committed.

1. **Ref Q**: "Bloody Handprint from Good-Luck Sacrifice - Shop Wall - Qazvin - Northwestern Iran", CC BY-SA 2.0,
   <https://commons.wikimedia.org/wiki/File:Bloody_Handprint_from_Good-Luck_Sacrifice_-_Shop_Wall_-_Qazvin_-_Northwestern_Iran_(7414179988).jpg>.
   Dried blood on a white tile wall. Measured over the 268 181 blood pixels: darkest 5 % `(88, 9, 3)`, lower quartile `(113, 35, 12)`,
   median `(134, 70, 32)`, lightest 5 % `(171, 135, 89)`. Thick is near black-red; thin is amber; the edges of a deposit are darker
   than its middle (the coffee ring); the footprint of every finger is intact.
2. **Ref N**: Oslo police, a 1950s photograph of a crime scene, public domain,
   <https://commons.wikimedia.org/wiki/File:Bloodstains_in_crime_scene_in_1950s_photograph_of_Norway,_from-_Aasted-2_(cropped).jpg>.
   Dark matte drops and stains and a run of footprints on a floor, all kept: nothing in the picture has faded for being thin.
3. **Ref 1**: "Blood Spatter Texture - FREE", CC BY 2.0, <https://commons.wikimedia.org/wiki/File:Blood_Spatter_Texture_-_FREE.jpg>
   (iteration 8's wall reference): 232 stains of at least 3 px in the frame. The count a wall's spatter must still have once dried.
4. *Evil Dead* (2013), KNB's methylcellulose blood, <https://filmhounds.co.uk/2023/04/17/gory-groovy-the-splatter-of-evil-dead/>:
   press page; the craft source for blood that stays on the set's walls for the whole shoot ("the blood spatter and damage to walls
   and sets had to look progressively worse throughout filming", so Alvarez shot in order).
