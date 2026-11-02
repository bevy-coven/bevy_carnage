# A cut face is a wet surface, not a plane: relief and glints on the preset's caps

Iteration 19 (2026-11-02), the item that came out of `docs/research/film-per-tissue.md`. That note's two attempts to make a cut face show its tissue through its blood (a thin film per tissue, then a face
that drains) failed to reach GREAT, and between them established what the broken pieces of 0.8.12 lack: not a colour, **a surface**. Every cap the preset draws is a flat polygon of one tone (wine-crimson
under its film), lit by two suns and an environment, so it takes one shade and no highlight: the "flat red sail". The *Hellraiser* reference reads as meat because it is **lit wet**, all highlights; the
*The Thing* cavity because its ridges catch the light. This note records the model that gives a cap relief, measured against the 0.8.12 frame it replaces.

## What was measured before any change

`capture_preset --eye 0.30,0.42,0.62 --at 0.0,0.10,0.0`, frame 300 (the new flag from the first note), 800 x 500, the 0.8.12 shader, 68 000 pixels of cut faces (the pixels that are red in that frame): within a face
the luminance has a standard deviation of **7.65** (all of it between faces, from their different slopes), the local variation (mean absolute difference of neighbouring pixels) is **0.61**, no pixel of a face
is a highlight. The same measurement in `capture_skinned --break --eye 0.28,0.38,0.30 --at -0.08,0.14,-0.05` frame 359: a flat sail.

## Sources

**Practice.** M. Cramer & Associates, "Prosthetic Wound Makeup: A Step-by-Step Guide" (`https://www.mcramer.com/articles/how-to-color-prosthetic-makeup-wounds/`), step 6: "To make the blood or ooze
appear wet and shiny, add a touch of gloss." A wound is made to read as wet by a **gloss**, laid over colour that does not change.

**Physics / technique.** Mikkelsen, "Bump Mapping Unparametrized Surfaces on the GPU" (2010), the method `flesh.wesl`'s `lifted_normal` already uses for a blood film: a perturbed normal from a height's derivative
in screen space, with no tangents. A specular highlight is the mirror image of a light in a surface whose normal sweeps through it, so a surface whose normal is one value has one highlight or none. The preset's
film already does this for blood on skin (`FILM_RELIEF`, whose comment in `flesh.wesl` says "a pool's unevenness breaks a light's mirror image into a glint").

**Shipping games.** Dambuster's *Dead Island 2* FLESH (Motionographer, `https://motionographer.com/2025/01/27/dead-island-2-a-stunning-visual-zombie-gore-feast/`): zombies have "separate layers of skin, muscle,
fat, and bones": the interior is a *surface with layers*, not a coloured decal. (Quoted from the iteration 18 note; the page was not re-read.)

## Craft principle

Wet is a property of the highlights, not of the colour (Cramer: "add a touch of gloss"). A cap needs a surface that catches a light somewhere on every face, and the cut muscle's own folds are the right size to
do it.

## Reference set

Kept under `/tmp/c19/refs/` only, cited by URL, deleted at cleanup; see `film-per-tissue.md` for the full list. The ones this note rests on:

1. *Hellraiser* (1987), skinless Frank (`https://static0.srcdn.com/wordpress/wp-content/uploads/2020/11/Hellraiser-1987-Frank-Without-Skin.jpg`): the lean is **highlights on a wet red**, bright specular glints
   on the ridges of the face and neck, darker in the recesses.
2. *The Thing* (1982), the defibrillator scene (`https://static0.srcdn.com/wordpress/wp-content/uploads/2024/03/the-defibrillator-scene-in-the-thing.jpg`): a torn chest, wet dark red, ridges lit.
3. The painted slash on a cheek (`https://www.etsy.com/listing/1781416100/silicone-gash-prosthetic`; a prosthetic): glossy dark blood, a highlight where it pools.

## Model adopted

In `flesh.wesl`, for `FleshMode::Cap` with a cut film (`FleshParams::cut.x > 0`, which is what the preset's caps wear): a **height** that is a value noise in two octaves over the face's planar `UV_0` (mesh
units, stable as a piece moves), `CAP_BUMP_MM` (16 mm) across, `CAP_RELIEF_MM` (4 mm) high; its slope by central differences in the noise's lattice, carried to the screen by the UV's derivatives and lifted
into a normal by `lifted_normal` (tilt capped at 25 degrees). The tissue's normal takes that slope and the **clear coat's normal takes `COAT_BUMP` (3) times it**, the liquid's surface tilting further than the
folds it lies on; the coat is `CAP_COAT` (1.8) times a crater wall's. **No colour changes**: the film, the strip and the LUT are as they were. Eight value-noise evaluations a fragment on a cut face and none
elsewhere; no binding, no texture, no uniform: the WebGL2 budget is untouched.

## Parameters

| name | value | status |
|---|---|---|
| `CAP_BUMP_MM` | 16 | this crate's own: a belly's size at the strip's scale (12 mm), a little over |
| `CAP_RELIEF_MM` | 4 | this crate's own; swept (below) |
| `COAT_BUMP` | 3 | this crate's own; swept |
| `CAP_COAT` | 1.8 | this crate's own; swept |
| tilt cap | 25 degrees | `FILM_MAX_TILT_COS`, the film's own |

## Rounds

Four, each rendered on the heap of pieces and looked at beside the 0.8.12 frame.

1. 2 mm over 14, coat as it was: a **faint mottle** in the diffuse shading, no highlight.
2. 3 over 10, 4 over 16, 3 over 7: **a creased membrane**, **broad wet folds**, **crumpled foil**. 4 over 16 kept.
3. The coat 2 and 3 times a crater wall's, the normals alike: **no visible change**. A strong coat on a normal that never sweeps through a light has nothing to reflect.
4. The coat's normal at 3 times the tissue's slope, coat 1 and 2.5: glints on the folds, **faint** at 1 and **glittering** at 2.5. 3 and 1.8 kept.

## Validation targets

1. The faces carry texture the 0.8.12 frame has none of: local variation within the cut faces at least 2 x.
2. Highlights appear on the faces: at least 1 % of face pixels brighter than they were by 15 or more in luminance.
3. **The colour does not move**: the mean colour of the face pixels within 3 of 255 per channel of the 0.8.12 frame's.
4. Nothing else moves: every recorder's digest is 0.8.12's (the shader is not hashed), `bench_carnage` within noise.
5. WebGL2: `tools/webgl2/check.sh` reports OK.
6. Look: below.

## What great looks like

A broken body's pieces, at the camera that shows a piece a few tens of pixels across, whose faces **glisten**: a soft undulation in the shading and bright glints along the folds, the colour unchanged, so that a
face reads as wet raw flesh and not as a flat plate, and nothing of it reads as glitter or as foil.

## Deviations

- No source times or sizes the folds of a freshly cut muscle surface; the sizes are this crate's own, chosen against the strip's belly size and swept.
- The relief is **procedural noise**, not the strip's structure: the strip's bellies are 12 mm and its septa sub-millimetre, below what a pixel at 5 mm shows, so a height from the strip would be noise at
  the camera's scale anyway and would cost samples; the value noise is the same thing without them.
- Caps only. A crater's walls are lifted by the film's own relief (`FILM_RELIEF`) where there is blood on them, and left alone where there is not; that is another note.

## Pending index

- Not in the corpus: a measurement of the specular reflectance of fresh cut muscle (a gloss meter on a wet surface). The *Hellraiser* still is the only evidence of the highlights' brightness.
