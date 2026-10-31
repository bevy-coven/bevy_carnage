# Spray reach: what a wound throws lands where the measured spatter does

Iteration 4 (2026-10-31). The bug recorded in `docs/loop-log.md` since iteration 0:
`preset::tests::a_wound_stains_the_floor_beneath_it` (`src/preset.rs`) is red, and `Cargo.toml`'s `0.6.0` note
says it has been since `0.5.64`. A visible item: it moves where blood lands on a floor under a default
`GorePlugin`. The code is `throw_spray` (the floor pass, closed form) and `GoreDials::default`.

## Diagnosis (measured, not inferred)

A probe in `throw_spray` (removed) printed, for the test's wound (an impact of radius 0.03 m at
`(0, 0.9, 0.3)`, normal `(-0.04, 0.19, 0.98)`, seven droplets): launch speeds
`13.6, 8.3, 39.6, 8.3, 27.8, 22.6, 36.6` m/s and landing points `z = 4.79, 4.58, 43.47, 1.83, ...` m on a
floor whose half-width is 1.5 m. **No droplet lands on a three-metre floor under the wound.** The assertion
is right and the fixture is the demos' own: `examples/common/preset_scene.rs:138-141` frames its camera because "the
spatter a torso shot throws lands on the floor in front of it", on the same three-metre slab. The cause is the shipped default:
`GoreDials::default()` is `BloodSettings::default()`, whose `spatter_speed_scale` is `1.0`, the paper's launch
speeds with no air in the way (`droplet::landing` is a vacuum parabola; the drag dial is "a look control on the
particles rather than a second integrator here", `src/bloodstain/stain.rs:409`). `BloodSettings`' own doc says
"Expect to lower it ... The reference demos set 0.25", and `carnage`, `capture_carnage`, `gore_tier` and
`bench_carnage` do; the two-line inclusion does not, so the preset's floor gets blood only from a few
droplets that happen to land in range and never from the slow ones that should.

## Sources

1. **Physics.** Comiskey, Yarin & Attinger, *Theoretical and experimental investigation of forward spatter of
   blood from a gunshot*, Phys. Rev. Fluids 3, 063901 (2018), `doi:10.1103/physrevfluids.3.063901`, indexed
   (quoted through `distill_search` this pass). Its measurement window and its drag are the two facts below.
   Also Comiskey et al. 2016 (`doi:10.1103/physrevfluids.2.073906`, back spatter), already in
   `docs/bloodstain-citations.md`.
2. **Horror-film and forensic reference** (`## Reference set`): three CC-licensed forensic photographs and
   diagrams from Wikimedia Commons, and three stills from *Evil Dead* and *Terrifier 3*.
3. **Technique.** How a shipping game puts the spray in the world: Dambuster's *Dead Island 2* FLESH system
   ("reacting in real-time by showing scars and blood splatters on the environment", Motionographer;
   "You're not just spawning in a decal or texture", AusGamers interview) and the open-source Minecraft
   mod "gore" (spray is "a set of jittered rays cast from the wound in a cone along the blow, clipped against
   block collision shapes"). The shipping idea is the same as this crate's: the spray is cast from the wound
   into the level, and what it reaches is stained.

## Craft principle

Comiskey et al. measured forward spatter on butcher paper laid on the floor behind the target:

> "The blood spatter was produced on a butcher paper substrate strip (0.41 × 4 m²) located on the floor
> behind the target. ... The parameters of interest were binned from 25–400 cm in 25-cm-wide strips."

so every stain they report lies between 0.25 m and 4 m of the target, **although** the same paper quotes the
launch speeds this crate uses: "at 0.45 ms after bullet impact, droplet velocity in the forward spatter case
was 40 m/s, whereas for backward spatter it was only 8 m/s". The two reconcile through air drag, which the
paper integrates ("droplet trajectories are numerically predicted accounting for gravity and air drag") and
which decides how far a drop flies:

> "the initially slower, larger droplets fly further than the initially faster, smaller ones ... the smaller
> drops have a lower inertia with respect to the effect of air drag and as such the trajectories of smaller
> drops are more affected by drag and fall shorter than those of larger drops."

A launch speed is therefore not a landing distance. At `1.0` this crate's closed form (no drag) puts a 40 m/s
droplet from chest height at 12 m to 43 m, ten times the measured field. At `0.25` the same droplet leaves
at 10 m/s and lands about 3 to 6 m out, and a back-spatter droplet (8 m/s, scaled 2 m/s) lands within a metre:
the measured 0.25 m to 4 m field.

## How shipping games do it

The spray is a cone of rays from the wound, cut against the level, and what a ray reaches is stained: a
floor in front of the victim and the walls and props round it, never a patch 40 m away. *Dead Island 2*'s
FLESH puts the "blood splatters on the environment" of each blow around the zombie it hit. This crate
already does it that way for a wall, a cloth and a body part (a straight cast from the wound); only the
floor is closed form, and the closed form is where the unscaled launch speed overshoots every level.

## Model adopted

No new model. `GoreDials::default()` states the preset's game scale: its `blood.spatter_speed_scale` is
`0.25` (`GORE_SPATTER_SPEED_SCALE`), the number every demo already authors by hand. `BloodSettings::default()`
keeps `1.0`: its constants are the paper's measurements and `the_spatter_model_is_frozen` pins them; the
preset is the one module that owns a look.

## Parameters

| Parameter | Value | Provenance |
|---|---|---|
| `GoreDials::blood.spatter_speed_scale` | `0.25` | **Tuned, and says so.** Chosen so that the unscaled 8 to 40 m/s lands in Comiskey et al.'s measured 0.25 to 4 m field once drag is left out; it is the number `docs/DEMOS.md:327` and `BACKLOG.md:739` record the demos setting by hand. A design default, not a measurement. |
| `BloodSettings::spatter_speed_scale` | `1.0` (unchanged) | The paper's launch speeds. |

## Validation targets

1. `a_wound_stains_the_floor_beneath_it` passes **unchanged** (the loop's acceptance since iteration 0).
2. A standard chest-height wound (`h = 0.9 m`, the test's) on a default `GoreDials`: the share of its droplets
   that land within 4 m of it rises from `1 of 7` to most of them; the farthest falls from 43 m to under 12 m.
3. `BloodSettings::default()` is untouched: `the_spatter_model_is_frozen`, `the_stain_placement_is_frozen`
   and every other bloodstain golden pass without edit.
4. A caller who sets `GoreDials { blood: BloodSettings { spatter_speed_scale: 1.0, .. }, .. }` gets the old
   throw, bit for bit (a test).
5. The recorders that dress a floor under the default dials (`capture_preset`, `capture_skinned`) print a new
   `digest` (a floor now takes spray), twice equal; `bench_carnage` (which authors its own scale) does not move.

## Deviations

- **A changed default, marked as a design default.** `GoreDials::default().blood.spatter_speed_scale`
  `1.0 -> 0.25`. Reason above; citation Comiskey et al. 2018. It moves every default-dials recorder digest
  that has a floor; the log and the changelog say so.
- **Not done, and recorded:** the model's size and speed are inverted against the paper's measured *range*
  (the paper's larger, slower drops fly farther; `droplet.rs` makes the smallest the fastest and the vacuum
  lands the fastest farthest). Fixing it needs a drag integrator in `landing`, which moves four frozen
  goldens, so it is a decision for the owner and not this pass's.

## Pending index

Nothing to index: the physics source is indexed. The technique sources are web pages (Motionographer, AusGamers,
Kotaku, a GitHub README), cited by URL, not papers. `distill_search` for a game-gore technique source returned
only SIGGRAPH 2011 "Dissolve / Gloop / Water" slide headings, which hold nothing on decals.

## What great looks like

A hit on a body in the middle of a three-metre room stipples the floor in a fan in front of the wound: a
dense core of fine drops a metre out, thinning to a few large ones at three, every drop a different size, none
of them a uniform disc, the whole fan one band along the throw direction rather than a sheet (Ref 1). The fan
must not stop at the slab's edge and must not be absent. Fresh blood on it is a saturated red, near sRGB
`(161, 60, 46)` on a neutral surface (Ref 1), darker `(150, 37, 30)` where it films thick (Ref 4), and a dried fleck
is brown-black `(56, 29, 21)` (Ref 5).

## Reference set

Kept under `/tmp/c4/refs/`, never committed (the CC ones could be, with attribution; none is needed).

1. **Ref 1**: `ref1_commons_spatter_texture_ccby2.jpg`, "Blood Spatter Texture - FREE", jimmy brown, CC BY 2.0,
   <https://commons.wikimedia.org/wiki/File:Blood_Spatter_Texture_-_FREE.jpg>. Spray on a wall as one diagonal
   band: about a hundred stains from a pixel to a thumb across, round with short tails, a dense core and a
   thinning edge. Fresh stain mean sRGB `(161, 60, 46)` over the 16,192 pixels with `R > 140, G < 90, B < 90`.
2. **Ref 2**: `ref2_commons_drop_wipe_cc0.jpg`, "2023 Blutstropfen Wischspur", Georgenhausen, CC0,
   <https://commons.wikimedia.org/wiki/File:2023_Blutstropfen_Wischspur.jpg>. Six drops fallen on a board, a few
   millimetres across, round, crisp-edged and each its own size, one wiped smear. Mean sRGB `(172, 54, 30)`.
3. **Ref 3**: `ref3_commons_area_of_origin_ccbysa4.jpg`, "Calculating the area of origin of a blood splatter",
   Ashley Do, CC BY-SA 4.0,
   <https://commons.wikimedia.org/wiki/File:Calculating_the_area_of_origin_of_a_blood_splatter.jpg>. The tail of
   each stain points back at the wound (the diagram the crate's `area_of_origin` follows).
4. **Ref 4**: `ref4_filmhounds_evildead2013.jpg`, *Evil Dead* (the article's still of the franchise's blood-soaked
   finale), <https://filmhounds.co.uk/2023/04/17/gory-groovy-the-splatter-of-evil-dead/>, a press still, copyright
   its owner, reference only. Fresh blood films the face: a dark crimson, mean `(150, 37, 30)`, with wet
   highlights and runs; the film is darker than a drop's colour because it is thick.
5. **Ref 5**: `ref5_variety_terrifier3.jpg`, *Terrifier 3* (2024), Variety,
   <https://variety.com/2024/artisans/news/terrifier-3-kills-art-the-clown-1236177913/>, press still, reference
   only. A stipple of small drops on white fur trim: a few millimetres each, a spread of sizes, and the
   older ones brown-black, mean `(56, 29, 21)`.
6. **Ref 6**: `ref6_ilovesplatter_evildead2013.jpg`, *Evil Dead* (2013), I Love Splatter,
   <https://ilovesplatter.com/2013/04/05/evil-dead-2013/>, press still, reference only. Blood across a
   face and collar, matte-dark crimson, mean `(148, 65, 58)` where thin.

## What was learned about the effect, for the artist's eye

*The Thing* (1982) and *Evil Dead* (2013) are shot with pressurised rigs, pump sprayers and blood bags (Bloody
Disgusting, Fangoria and FILMHOUNDS pages in the search log), which throw blood **metres** in a fan; the
floor and walls round the victim are what the audience reads. That is the target for the floor: a fan out to
about three metres, which is what the default has to deliver without the caller authoring a speed scale.
