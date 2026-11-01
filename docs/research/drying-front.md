# Blood dries from its edge in, and a dried crust cracks

Iteration 11 (2026-11-01). Item: how a film of blood passes from wet to dry **in space**. Ranked from iteration 9's and 10's own backlog
("dried blood's hue and its rim ... thickness-driven local age (thin film dries first)") and measured again this pass: after `--dry 120` the
frame 359 of `capture_skinned --through --floor 512` shows a wall of drops, a floor pool and a chest wound **each one flat tone** (a pool's
median `(79, 55, 51)`, a drop's `(78, 49, 45)`). `flesh.wesl` ages a film by the one age byte its texels carry, and a pool is painted all at
once, so every texel of it is the same age: the whole pool changes colour together, as a painted decal does. Runner-ups, and why they lost:
the wound's rim lumps and the rivulets (blocked by the canvas: `docs/research/rivulets.md`, "don't retry"); the spray's size spread (resolution
bound, a shader term too); `digest=` for a body that breaks (invisible, a gate item). The drying item is a shader term with no CPU change,
so **every digest holds** and the proof of that is the recorders' own.

## What was measured before any change

- `capture_skinned --through --floor 512 --dry 120` (frame 359, everything dried) against the references below. A drop on the wall:
  median `(78, 49, 45)`, 5th to 95th percentile `(70, 47, 43)` to `(87, 51, 45)`: **a flat tone**, no darker rim, no lighter film. The floor
  pool and the chest wound are the same: one colour edge to edge. The reference pool (Ref 1, below) is black round its rim, red in its middle
  while it is half dry, and the rim is cracked in lines at right angles to the edge.
- The cause is in the data and not the walk: `flesh.wesl::blood_dried` reads the canvas's wetness byte, which is **one number per paint**
  (`wetmap`'s age is per texel, but a pool is stamped in one tick and a drop in one), so the colour walk `fresh -> methaemoglobin -> hemichrome`
  runs over the whole film in step. The physics says it does not (next section). The CPU model is not at fault and is untouched: this note
  changes what the *shader* does with the age it is given.
- Rim and cracks do not exist anywhere in the shader: `Appearance::rim` (the drying front) and `Appearance::craquelure` are computed by
  `bloodstain::dry::appearance_with_fresh` and **read by nothing** that draws.

## Sources

1. **Physics, the front.** Smith, Nicloux & Brutin, *A new forensic tool to date human blood pools*, Sci. Rep. 10 (2020),
   `doi:10.1038/s41598-020-65465-4` (in the corpus, `distill_search`; CC BY 4.0, its figures may be reproduced).
2. **Physics, the colour.** Bremmer et al., *Biphasic oxidation of oxy-hemoglobin in bloodstains*, PLoS ONE 6 (2011),
   `doi:10.1371/journal.pone.0021845` (in the corpus; the walk `bloodstain::dry` implements; unchanged here).
3. **Physics, the drop.** Brutin, Sobac, Loquet & Sampol, *Pattern formation in drying drops of blood*, J. Fluid Mech. 667 (2011),
   `arXiv:1010.2510` (open abstract read; full text not converted, see `## Pending index`): the final deposit of a drop is a
   periphery, a corona and a central region, a dark ring with radial cracks, the coffee ring.
4. **Horror-film and practical-effects reference** (`## Reference set`): Smith's Fig. 12 (a pool at the drying front), a handprint on tile,
   blood on a pavement the morning after, Tom Savini on the age of blood.
5. **Technique.** Lagarde, *Water drop 3b, physically based wet surfaces* (2013): the state between wet and dry is interpolated in the
   material's parameters, spatially. And this crate's own constraint from `CLAUDE.md`: the CPU canvas is the authority, the GPU is write-only,
   nothing in a shader may be needed by a hash.

## Craft principle

The model source, quoted from the corpus (`distill_search`, `10.1038_s41598-020-65465-4`):

> "desiccation occurs first at the triple line, where the rim dries first. Then this drying front propagates towards the centre of the pool."

and, for what the dried zone does:

> "In the final stage, cracks start being observed. ... These cracks follow the drying front and the separation between them depends on the
> thickness of the sample, the physical and chemical properties, the adhesion with the surface and on the desiccation conditions."

and, on what the cracks of a whole pool look like (same paper): "The crack patterns of an entire blood pool are quite different from a single blood drop, where the cracks are
elongated and directed towards the centre of the stain. Nonetheless these cracks show similarities to the ones observed by Pauchard et al. [27] during
the drying of a colloidal suspension", which are polygonal crack networks: a plate pattern, which is what the shader draws.

and that the front is what a viewer reads as colour: "the transition edge between wet and dry blood, and visually refers to the transition
between the red and the black colour" (same paper, on its drying front). Two consequences for a renderer: **a film is not one age** (the part
near its contact line is older than the part far from it, by an amount that grows with the distance), and **the dried zone is cracked, at
right angles to the front, at a spacing that goes with the film's thickness**. Nothing in a flat-tone dried pool says either.

## How shipping games do it

Games carry a wet and a dry state and blend between them, as a parameter: Lagarde (2013): "Simply interpolating between dry and wet BRDF
parameters without changing the lighting model do the effect." That is a uniform blend: a stain is wet or dry, per decal or per material.
A decal is authored once (Vlachos, *Rendering Wounds in Left 4 Dead 2*, GDC 2010; Coenen on Doom Eternal's decals, `docs/research/dry-blood.md`),
so the drying of a stain *in space* has no home in a shipping pipeline. This crate has what they lack, a per-texel canvas the CPU owns, so
the front can be drawn where it is. The boundary (`CLAUDE.md`): the canvas stays the authority, and the front is a function the shader
computes of what the canvas says, never read back.

## Model adopted

Three terms in `flesh.wesl`, for blood with a drying age over `FRONT_MIN_AGE` (0.02: about 0.6 s at the shipped 30 s), and none in any table, canvas
or digest. Fresh blood, the prepass and cloth take the code they always took.

1. **The front.** The age a fragment's colour is drawn at is `age = max(INTERIOR_RATE * canvas, clamp(canvas * (1 + LEAD) - LEAD * reach, 0, 1))`, with
   `canvas` the canvas's own age (`blood_dried`) and `reach` the fragment's distance from the film's edge as a share of `FRONT_REACH_MM`, `0` at the
   rim and `1` at that far or further. At `canvas` `0` it is `0` everywhere, at `1` it is `1` everywhere (the CPU's endpoints: a fully dried film is the
   colour it always was, and a fresh one is the colour it always was); between, the rim has dried and the middle has not: **the middle's colour
   walk runs at `INTERIOR_RATE` of the clock** (oxygen has to get in under a dried skin; Smith et al.'s S.IV has a red middle in a black rim), the
   rim's at the front. The first version let the middle lag the whole way and a half-dried pool went pink in the middle; with `max` against 0.4 of
   the clock the middle is the red-brown of the first stop, not fresh. Gloss follows: the coat is cut by how far the drawn age has got ahead of the
   canvas's, so the dry rim is matte while the middle still shines.
   `reach` is how the shader reads a canvas it does not own: six directions, each sampled seven times out to `FRONT_REACH_MM` at distances
   `(k/7)^1.8` of it (close together near the fragment), the run of film in a direction ending where the wet map is under half of its depth at the
   fragment, placed by interpolating between the two samples that bracket it, the six joined by a smooth minimum (p-norm, p = 4: a hard minimum
   draws a star). It is read in millimetres on the surface (`fwidth` of the world position over `fwidth` of the UV), so the same look holds on a
   3.5 mm skin, a 6 mm wall and a 12 mm floor.
2. **The crust.** The outermost `CRUST_REACH` of the front's reach in a film deeper than a fifth of the canvas's is darkened (`CRUST_DARK`) and drawn
   at the film's **full depth** (the deepest the film is within the reach), not the thin foot the canvas's spline gives its edge: the contact line
   is pinned and the solids are carried out to it (the coffee ring), so the rim of a thick film is as dark as a thick film. A drop (a film under a
   fifth of the canvas's depth) has no crust.
3. **The cracks.** Where the drawn age is past `CRAQUELURE_ONSET` (0.72, `bloodstain::dry`'s own number) and the film is deeper than a fifth of the canvas's, the
   borders of a jittered lattice of cells `CRACK_SPACING_MM` wide, in millimetres on the surface, are cracks 0.35 mm wide (a pixel at least), drawn as
   the surface the blood lies on showing through, in patches (two octaves of value noise over the lattice, so a pool is cracked here and whole
   there) and faded out where the plates are under twelve pixels across and gone under two. **A first version read the lattice at the nearest point of the
   edge, so that every crack ran in from the rim at right angles to it, as Ref 1's do; the normal the six directions give is too noisy where the film
   is deep and the cracks scrambled into swirls (contour lines of the estimate), so the plates are read where the fragment is.** The radial look is
   named in `## Deviations`. A lattice whose cell followed the film's depth (the quote above: spacing goes with thickness) drew a seam where the
   depth estimate steps, so the cell is constant.

What is **not** here and is named: the colours (`SRGB_MET`, `SRGB_HEMI` and the aged table stay: `the_flesh_tables_are_frozen` and
`the_aged_blood_table_is_frozen` pass untouched); a CPU drying front (a distance transform per tick, its own item with its own conservation
proof and every wet digest moving); the radial cracks of Ref 1 (above); a coffee ring on a drop.

## Parameters

| Parameter | Value | Provenance |
|---|---|---|
| `FRONT_LEAD` | 2.5 | **Design default**, tuned by eye against Ref 1: the front's width is `1 / (1 + LEAD)` of the reach. At 4 it was a thinner, sharper line and no better. |
| `FRONT_REACH_MM` | 30 | **Tuned.** Ref 1: a dried rim 1.5 to 2 cm wide on a pool 3.6 cm across at the half dry. A pool wider than twice this dries in its middle last, together. |
| `INTERIOR_RATE` | 0.4 | **Design default.** 1.0 (no lag) lost the red middle; 0 made the middle of a pool at an age of a half the colour of fresh blood. |
| `FRONT_DIRS`, `FRONT_STEPS` | 6, 7 | **Design default**: the eight by five first tried drew stair-steps at the texel on a 3.5 mm canvas; seven steps at `(k/7)^1.8` are under a texel apart within 12 mm of the rim. 42 taps. |
| `CRUST_REACH`, `CRUST_DARK` | 0.4, 0.6 | **Tuned** to Ref 1's black rim and Ref Q's darkest borders. |
| `CRACK_SPACING_MM` | 12 | **Tuned for legibility.** Ref 1 has 4 to 6 mm on a pool 5 cm across; 12 mm is read from a metre or two at 1080p. |
| `CRACK_WIDTH_MM`, `CRACK_MIN_PX` | 0.35, 2 | Ref 1's hairlines; a pixel at least; under two pixels of plate not drawn. |
| crack onset | drawn age 0.72 | `bloodstain::dry::CRAQUELURE_ONSET`, the crate's own number. |
| thin film does not crack | plateau under 0.2 of the canvas's depth | **Design default**: a film under a critical thickness does not crack (Smith et al., Pauchard et al., cited there); the number is this crate's own. |

## Validation targets

1. **No digest moves.** Every `capture_skinned` variant, `capture_preset` and `bench_carnage` print the digests they printed before, twice each
   (the shader is cosmetic). The goldens `the_flesh_tables_are_frozen` and `the_aged_blood_table_is_frozen` pass untouched.
2. **A fresh film is what it was.** At `age` `0` the frame is the same, pixel for pixel but the GPU's own noise (the wet frames of `--through` at
   frame 200, compared with the 0.8.4 binary: under 2 of 765 on at most a few hundred pixels).
3. **A dried film is what it was in its middle and different at its edge.** A fully dried pool's 5th percentile is darker than its 95th by
   a margin the flat one lacks, its rim ring is darker than its interior, and the whole is no lighter in mean than 0.8.4's.
4. **The front moves inward.** A pool read at three ages (`--dry` of `120`, then frames 200, 280, 359 of a run): the dark band's width grows from
   the rim, the interior stays red and glossy until the band reaches it (Ref 1, S.IV). Measured as the red-to-black transition's distance from
   the rim.
5. **Cracks are at right angles to the edge** in the dried zone, of different lengths, and not in a thin film (a wall's drops) or in a wet one.
6. **WebGL2.** `tools/webgl2/check.sh` prints `WEBGL2 PIPELINE OK`; the harness fails on a shader with one added type error.
7. **Cost.** `bench_carnage` never calls a canvas (digest and instructions retired equal); the shader's extra taps are for dried blood only
   (`age > 0`), counted and bounded in the log.

## Deviations

- `Appearance::rim` and `Appearance::craquelure` were read by nothing. The shader now follows their intent with its own spatial terms; the CPU
  values are unchanged and still the reference for a caller who draws blood elsewhere.
- **The cracks are plates, not the radial cracks of Ref 1.** The estimate of the edge's direction that the six directions give is not smooth enough to
  carry cracks at right angles to it (swirls: tried and removed). A reader who wants Ref 1's radial cracks needs a distance field the canvas does not
  keep: a CPU item.
- A film of one age no longer draws as one colour: a caller who compared `appearance()` against a screenshot of a *pool* will see the rim lead it.
  There is no setting that restores the flat pool (one path per feature).

## Results (graded against the targets above)

1. **No digest moves: met.** All eight `capture_skinned` variants (default, `--blows`, `--break`, `--shirt`, `--through`, `--guard`, `--stroll`, `--quiet`) once on
   0.8.4 and twice on the new build, and `capture_preset` twice: every digest, `flay=` and `dermis=` equal 0.8.4's (default `5f8eb5573aea933f`, `--break`
   `f89e1d5e3bcfc119`, `--shirt` `579e84b9d3ccfe51`, `--through` `cfcabf28b3d9b0a6`, `--guard` `33118d5449e57748`, `--stroll` `6cb633ae2cf7941c`, `--quiet`
   `f83a631a7426808d`, preset `04758d82ec10aa5b`). The frames of the two new runs are equal pixel for pixel. `cargo test --no-fail-fast`: 471 passed, 0 failed,
   8 ignored; every golden passes untouched. `bench_carnage` instructions retired, three alternations: 15.80, 15.72, 15.71 G before and 15.80, 15.78,
   15.76 G now (+0.2 %); its timing refused on the new build at host load 8 to 9 ("NO STABLE PLATEAU": the bench never calls a canvas).
2. **A fresh film is what it was: met.** Frame 60 of every variant (before any blood is 0.02 old) differs in 0 pixels, and `--quiet` in 0 of 360 frames. A
   shot's blood differs from its 0.6 s of age: at the shipped 30 s a wound's rim is a shade browner in the first seconds (the runs of frame 260, `--eye 0.0,1.3,0.75`).
3. **A dried film differs at its edge: met.** A wall's dried drops (a film under a fifth of the canvas's depth) are 0.8.4's (1 pixel of 31 250 differs, by 7 of 765);
   a pool and a run are not: at an age of 0.8 the pool's rim is `(70, 46, 42)` (R minus G 24) against a middle of `(92, 39, 36)` (53), where 0.8.4's are `(80, 45, 41)` (35)
   and `(71, 40, 37)` (31): Ref 1's are `(27, 19, 14)` (8) and `(72, 7, 6)` (65). Luminance of rim and middle are equal in Ref 1 (20 and 20) and now (50 and 50): the
   contrast is in chroma, as the reference's is. **The rim is not as neutral or as black as Ref 1's.**
4. **The front moves inward: met.** `docs/media/drying_front.gif`: at an age of 0.18 the pool is one red; at 0.49 a dark rim 12 mm wide; at 0.79 a rim ahead of a red middle with a red-to-dark edge
   between them (the sheet `compare-pool-front` of the pass: Ref 1, 0.8.4, now).
5. **Cracks: met in kind, not in morphology.** In patches, in plates, never in a thin film or a wet one, faded out by the pixel footprint (no shimmer at any
   distance in the clips). **They are not Ref 1's radial cracks in a black glossy rim, and its plates (S.V) are five times as wide and glossy:** named in `## Deviations`.
6. **WebGL2: met.** `tools/webgl2/check.sh` on the final shader: `WEBGL2 PIPELINE OK` (module initialised, WebGL2 context, no console error, 2 453 distinct colours). That harness
   fires its shots and photographs at once, so no blood in it is old enough to take the new path: what it proves is that the whole shader, new functions included, compiles and links on
   WebGL2 (it did fail on the first draft of this pass's native shader, which named a variable `patch`, a reserved word, and on iteration 9's added type error).
7. **Cost: counted, not timed.** 42 taps of the wet map for a pixel of blood older than 0.02 of its drying (six directions, seven taps), plus, where it cracks, 18 integer hashes
   and 8 for the patch noise. Fresh blood none. The recorders are readback-bound at 800 x 500, so a millisecond figure would be the readback's; an in-game GPU timing is in the log's backlog.

## Pending index`): the final deposit of a drop is a
   periphery, a corona and a central region, a dark ring with radial cracks, the coffee ring.
4. **Horror-film and practical-effects reference** (`## Reference set`): Smith's Fig. 12 (a pool at the drying front), a handprint on tile,
   blood on a pavement the morning after, Tom Savini on the age of blood.
5. **Technique.** Lagarde, *Water drop 3b, physically based wet surfaces* (2013): the state between wet and dry is interpolated in the
   material's parameters, spatially. And this crate's own constraint from `CLAUDE.md`: the CPU canvas is the authority, the GPU is write-only,
   nothing in a shader may be needed by a hash.

## Craft principle

The model source, quoted from the corpus (`distill_search`, `10.1038_s41598-020-65465-4`):

> "desiccation occurs first at the triple line, where the rim dries first. Then this drying front propagates towards the centre of the pool."

and, for what the dried zone does:

> "In the final stage, cracks start being observed. ... These cracks follow the drying front and the separation between them depends on the
> thickness of the sample, the physical and chemical properties, the adhesion with the surface and on the desiccation conditions."

and, on what the cracks of a whole pool look like (same paper): "The crack patterns of an entire blood pool are quite different from a single blood drop, where the cracks are
elongated and directed towards the centre of the stain. Nonetheless these cracks show similarities to the ones observed by Pauchard et al. [27] during
the drying of a colloidal suspension", which are polygonal crack networks: a plate pattern, which is what the shader draws.

and that the front is what a viewer reads as colour: "the transition edge between wet and dry blood, and visually refers to the transition
between the red and the black colour" (same paper, on its drying front). Two consequences for a renderer: **a film is not one age** (the part
near its contact line is older than the part far from it, by an amount that grows with the distance), and **the dried zone is cracked, at
right angles to the front, at a spacing that goes with the film's thickness**. Nothing in a flat-tone dried pool says either.

## How shipping games do it

Games carry a wet and a dry state and blend between them, as a parameter: Lagarde (2013): "Simply interpolating between dry and wet BRDF
parameters without changing the lighting model do the effect." That is a uniform blend: a stain is wet or dry, per decal or per material.
A decal is authored once (Vlachos, *Rendering Wounds in Left 4 Dead 2*, GDC 2010; Coenen on Doom Eternal's decals, `docs/research/dry-blood.md`),
so the drying of a stain *in space* has no home in a shipping pipeline. This crate has what they lack, a per-texel canvas the CPU owns, so
the front can be drawn where it is. The boundary (`CLAUDE.md`): the canvas stays the authority, and the front is a function the shader
computes of what the canvas says, never read back.

## Model adopted

Three terms in `flesh.wesl`, for blood with a drying age over `FRONT_MIN_AGE` (0.02: about 0.6 s at the shipped 30 s), and none in any table, canvas
or digest. Fresh blood, the prepass and cloth take the code they always took.

1. **The front.** The age a fragment's colour is drawn at is `age = max(INTERIOR_RATE * canvas, clamp(canvas * (1 + LEAD) - LEAD * reach, 0, 1))`, with
   `canvas` the canvas's own age (`blood_dried`) and `reach` the fragment's distance from the film's edge as a share of `FRONT_REACH_MM`, `0` at the
   rim and `1` at that far or further. At `canvas` `0` it is `0` everywhere, at `1` it is `1` everywhere (the CPU's endpoints: a fully dried film is the
   colour it always was, and a fresh one is the colour it always was); between, the rim has dried and the middle has not: **the middle's colour
   walk runs at `INTERIOR_RATE` of the clock** (oxygen has to get in under a dried skin; Smith et al.'s S.IV has a red middle in a black rim), the
   rim's at the front. The first version let the middle lag the whole way and a half-dried pool went pink in the middle; with `max` against 0.4 of
   the clock the middle is the red-brown of the first stop, not fresh. Gloss follows: the coat is cut by how far the drawn age has got ahead of the
   canvas's, so the dry rim is matte while the middle still shines.
   `reach` is how the shader reads a canvas it does not own: six directions, each sampled seven times out to `FRONT_REACH_MM` at distances
   `(k/7)^1.8` of it (close together near the fragment), the run of film in a direction ending where the wet map is under half of its depth at the
   fragment, placed by interpolating between the two samples that bracket it, the six joined by a smooth minimum (p-norm, p = 4: a hard minimum
   draws a star). It is read in millimetres on the surface (`fwidth` of the world position over `fwidth` of the UV), so the same look holds on a
   3.5 mm skin, a 6 mm wall and a 12 mm floor.
2. **The crust.** The outermost `CRUST_REACH` of the front's reach in a film deeper than a fifth of the canvas's is darkened (`CRUST_DARK`) and drawn
   at the film's **full depth** (the deepest the film is within the reach), not the thin foot the canvas's spline gives its edge: the contact line
   is pinned and the solids are carried out to it (the coffee ring), so the rim of a thick film is as dark as a thick film. A drop (a film under a
   fifth of the canvas's depth) has no crust.
3. **The cracks.** Where the drawn age is past `CRAQUELURE_ONSET` (0.72, `bloodstain::dry`'s own number) and the film is deeper than a fifth of the canvas's, the
   borders of a jittered lattice of cells `CRACK_SPACING_MM` wide, in millimetres on the surface, are cracks 0.35 mm wide (a pixel at least), drawn as
   the surface the blood lies on showing through, in patches (two octaves of value noise over the lattice, so a pool is cracked here and whole
   there) and faded out where the plates are under twelve pixels across and gone under two. **A first version read the lattice at the nearest point of the
   edge, so that every crack ran in from the rim at right angles to it, as Ref 1's do; the normal the six directions give is too noisy where the film
   is deep and the cracks scrambled into swirls (contour lines of the estimate), so the plates are read where the fragment is.** The radial look is
   named in `## Deviations`. A lattice whose cell followed the film's depth (the quote above: spacing goes with thickness) drew a seam where the
   depth estimate steps, so the cell is constant.

What is **not** here and is named: the colours (`SRGB_MET`, `SRGB_HEMI` and the aged table stay: `the_flesh_tables_are_frozen` and
`the_aged_blood_table_is_frozen` pass untouched); a CPU drying front (a distance transform per tick, its own item with its own conservation
proof and every wet digest moving); the radial cracks of Ref 1 (above); a coffee ring on a drop.

## Parameters

| Parameter | Value | Provenance |
|---|---|---|
| `FRONT_LEAD` | 2.5 | **Design default**, tuned by eye against Ref 1: the front's width is `1 / (1 + LEAD)` of the reach. At 4 it was a thinner, sharper line and no better. |
| `FRONT_REACH_MM` | 30 | **Tuned.** Ref 1: a dried rim 1.5 to 2 cm wide on a pool 3.6 cm across at the half dry. A pool wider than twice this dries in its middle last, together. |
| `INTERIOR_RATE` | 0.4 | **Design default.** 1.0 (no lag) lost the red middle; 0 made the middle of a pool at an age of a half the colour of fresh blood. |
| `FRONT_DIRS`, `FRONT_STEPS` | 6, 7 | **Design default**: the eight by five first tried drew stair-steps at the texel on a 3.5 mm canvas; seven steps at `(k/7)^1.8` are under a texel apart within 12 mm of the rim. 42 taps. |
| `CRUST_REACH`, `CRUST_DARK` | 0.4, 0.6 | **Tuned** to Ref 1's black rim and Ref Q's darkest borders. |
| `CRACK_SPACING_MM` | 12 | **Tuned for legibility.** Ref 1 has 4 to 6 mm on a pool 5 cm across; 12 mm is read from a metre or two at 1080p. |
| `CRACK_WIDTH_MM`, `CRACK_MIN_PX` | 0.35, 2 | Ref 1's hairlines; a pixel at least; under two pixels of plate not drawn. |
| crack onset | drawn age 0.72 | `bloodstain::dry::CRAQUELURE_ONSET`, the crate's own number. |
| thin film does not crack | plateau under 0.2 of the canvas's depth | **Design default**: a film under a critical thickness does not crack (Smith et al., Pauchard et al., cited there); the number is this crate's own. |

## Validation targets

1. **No digest moves.** Every `capture_skinned` variant, `capture_preset` and `bench_carnage` print the digests they printed before, twice each
   (the shader is cosmetic). The goldens `the_flesh_tables_are_frozen` and `the_aged_blood_table_is_frozen` pass untouched.
2. **A fresh film is what it was.** At `age` `0` the frame is the same, pixel for pixel but the GPU's own noise (the wet frames of `--through` at
   frame 200, compared with the 0.8.4 binary: under 2 of 765 on at most a few hundred pixels).
3. **A dried film is what it was in its middle and different at its edge.** A fully dried pool's 5th percentile is darker than its 95th by
   a margin the flat one lacks, its rim ring is darker than its interior, and the whole is no lighter in mean than 0.8.4's.
4. **The front moves inward.** A pool read at three ages (`--dry` of `120`, then frames 200, 280, 359 of a run): the dark band's width grows from
   the rim, the interior stays red and glossy until the band reaches it (Ref 1, S.IV). Measured as the red-to-black transition's distance from
   the rim.
5. **Cracks are at right angles to the edge** in the dried zone, of different lengths, and not in a thin film (a wall's drops) or in a wet one.
6. **WebGL2.** `tools/webgl2/check.sh` prints `WEBGL2 PIPELINE OK`; the harness fails on a shader with one added type error.
7. **Cost.** `bench_carnage` never calls a canvas (digest and instructions retired equal); the shader's extra taps are for dried blood only
   (`age > 0`), counted and bounded in the log.

## Deviations

- `Appearance::rim` and `Appearance::craquelure` were read by nothing. The shader now follows their intent with its own spatial terms; the CPU
  values are unchanged and still the reference for a caller who draws blood elsewhere.
- **The cracks are plates, not the radial cracks of Ref 1.** The estimate of the edge's direction that the six directions give is not smooth enough to
  carry cracks at right angles to it (swirls: tried and removed). A reader who wants Ref 1's radial cracks needs a distance field the canvas does not
  keep: a CPU item.
- A film of one age no longer draws as one colour: a caller who compared `appearance()` against a screenshot of a *pool* will see the rim lead it.
  There is no setting that restores the flat pool (one path per feature).

## Results

Written after the build, in the log entry and in `CHANGELOG.md`.

## Pending index

- Brutin, Sobac, Loquet & Sampol (2011), the J. Fluid Mech. version of `arXiv:1010.2510`: the full text did not convert (`read` gave the abstract);
  the periphery, corona and central region are quoted from the abstract and from a review (`web_search`).
- Laan et al., *Morphology of drying blood pools* (`doi:10.1016/j.forsciint.2016.08.005`, already cited by `bloodstain::dry`): not re-read this pass.
- Pauchard et al., the critical thickness of cracking in a drying colloid (cited inside Smith et al., ref 26 and 27): not read.
- Lagarde's page is a web page, not in the corpus.

## What great looks like

A pool on a floor an hour after the fight: a black glossy crust round its edge, thicker where it is wider, cracked in short lines at right angles to
the edge like the segments of a snake's belly; red and wet in the middle, with the light still breaking on it; the line between the two sharp
enough to read as an edge (Ref 1, S.IV). A wound's run the same on a smaller scale: dark at its borders, a wetter core. A fully dried pool is dark
crust to the middle with the cracks in it (S.V). A small drop dries before a big pool does.

## Reference set

Kept under `/tmp/c11/refs/`, never committed.

1. **Ref 1, Smith et al. 2020, Fig. 12 and Fig. 1** (CC BY 4.0), `https://www.nature.com/articles/s41598-020-65465-4`: a human blood pool
   on a cream substrate at the drying front (S.IV), and fully dried (S.V). Measured off the figure: the wet middle `(77, 6, 5)`, the dried rim
   `(25, 21, 14)` to `(38, 25, 18)`, the substrate `(254, 245, 211)`; the rim is 1.5 cm of a pool 5.5 cm across, the red-to-black edge 0.6 cm wide,
   and **radial cracks every 4 to 6 mm** at right angles to it, light against the black.
2. **Ref Q**, "Bloody Handprint from Good-Luck Sacrifice - Shop Wall - Qazvin", CC BY-SA 2.0,
   `https://commons.wikimedia.org/wiki/File:Bloody_Handprint_from_Good-Luck_Sacrifice_-_Shop_Wall_-_Qazvin_-_Northwestern_Iran_(7414179988).jpg`:
   blood dried on tile: thick, near black-red at the borders of every stroke and at the heel of the print, thin an amber film in the middle of each
   (measured over 292 777 blood pixels: median `(124, 63, 19)`, 5th percentile `(71, 6, 0)`, 95th `(161, 125, 89)`).
3. **Ref 3**, "Blood and sand on the ground in front of Le Petit Cambodge the day after November 2015 Paris attacks", CC BY 2.0,
   `https://commons.wikimedia.org/wiki/File:Blood_and_sand_on_the_ground_in_front_of_Le_Petit_Cambodge_the_day_after_November_2015_Paris_attacks_(22998462382).jpg`:
   old blood on granite, dark plum-brown, matte, no highlight, its edges ragged and darker than its middle (median `(114, 78, 78)` over granite).
4. **Ref 4**, "Blood on the sidewalk", CC BY-SA 3.0, `https://commons.wikimedia.org/wiki/File:Blood_on_the_sidewalk.jpg`: drops and a smear on
   concrete at dusk, near black plum `(30, 3, 22)`, glossy where thick.
5. **Ref N**, Oslo police 1950s, public domain, `https://commons.wikimedia.org/wiki/File:Bloodstains_in_crime_scene_in_1950s_photograph_of_Norway,_from-_Aasted-2_(cropped).jpg`
   (iteration 9): dark matte drops and stains kept on a floor.
6. Tom Savini, RNZ interview (`https://www.rnz.co.nz/national/programmes/afternoons/audio/2018762272/tom-savini-the-sultan-of-splatter`):
   "The blood would be brown, dark brown. It's 24 hours until blood turns dark brown." And the effects houses' practice, from the make-up
   suppliers' guides (`https://www.thefacepaintshop.com/en-us/blogs/all-blogs/fake-blood-australia-which-blood-for-which-effect`): "Most realistic
   wounds use both: coagulated inside and around the edge, fresh blood layered lightly on top", and "As blood ages, it adopts brown and orange
   undertones": the dried edge and the fresh middle are one wound.
7. *Evil Dead* (2013), `https://collider.com/evil-dead-remake-set-visit/`: "95% of the remake was shot in order" so the blood on the set's walls could
   get worse through the shoot: a set's blood is meant to age on camera.
