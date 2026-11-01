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

Two terms in `flesh.wesl`, for blood with a drying age over zero, and none in any table, canvas or digest:

1. **The front.** The age a fragment's colour and gloss are drawn at is `local = clamp(age * (1 + LEAD) - LEAD * reach, 0, 1)`, with `age`
   the canvas's `blood_dried` and `reach` the fragment's distance from the film's edge as a share of `FRONT_REACH_MM`, `0` at the rim and
   `1` at that far or further. `local >= age` at the rim and `local <= age` inside: at `age` `0` it is `0` everywhere, at `age` `1` it is
   `1` everywhere (the CPU's endpoints, so a fully dried film is the colour it always was and a fresh one is the colour it always was), and between them the
   rim has dried before the middle. Gloss follows: a texel's coat is cut by how far `local` has got ahead of the canvas's own wetness, so the
   dry rim is matte while the middle still shines.
   The distance is taken the way `crater_light` already reads a canvas: eight directions, each out to `FRONT_STEPS` texel distances, the first
   at which the film is under half its own depth, interpolated.
2. **The cracks.** Where `local` is past `CRAQUELURE_ONSET` (0.72, `bloodstain::dry`'s own number) and the film is thick enough to crack, lines
   at right angles to the edge, spaced `CRACK_SPACING_MM` at the film's plateau and proportionally wider in a thicker film, each with its own
   length (hashed), drawn as a thin line where the substrate shows. A fade by the pixel footprint keeps a crack that is under two pixels wide from
   shimmering.

What is **not** here and is named: the colours (`SRGB_MET`, `SRGB_HEMI` and the aged table stay: `the_flesh_tables_are_frozen` and
`the_aged_blood_table_is_frozen` pass untouched); the coffee ring's thickening of the rim itself (a canvas change, which would move every wet digest);
a CPU drying front (a distance transform per tick, its own item with its own conservation proof).

## Parameters

| Parameter | Value | Provenance |
|---|---|---|
| `FRONT_LEAD` | 3 | **Design default.** The front's width is `1 / LEAD` of the reach: Ref 1's red-to-black edge is 0.6 cm of a 2 cm band. |
| `FRONT_REACH_MM` | 30 | **Tuned.** Ref 1: a black rim 1.5 to 2 cm wide on a pool 3.6 cm across at the half dry. A pool wider than twice this dries in its middle last, together. |
| `FRONT_STEPS` | 1, 2, 4, 8, 16 texels | **Design default**, the geometric series `crater_light` uses. |
| `CRACK_SPACING_MM` | 4 | **Tuned** to Ref 1 (about 40 cracks round a perimeter of about 20 cm, read off the figure). Spacing grows with the film's thickness (the quote above: not a measured law of this crate's own). |
| crack onset | `local` 0.72 | `bloodstain::dry::CRAQUELURE_ONSET`, the crate's own number. |
| thin film does not crack | amount under 0.25 of the canvas depth | **Design default**: a film below a critical thickness does not crack (Smith et al., Pauchard et al. cited there); the number is this crate's own. |

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
