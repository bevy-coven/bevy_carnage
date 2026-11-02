# A film of blood per tissue: what a cut face shows through its blood

Iteration 19 (2026-11-02). The first item of iteration 18's backlog and the owner's "biggest visible gap" (idea 6, wound-interior detail): iteration 17 built a muscle strip that reads as meat
(`docs/research/cut-face.md`) and found that **the preset never shows it**, because every exposed tissue wears one film of blood, `CUT_FACE_FILM_MM` (1.0 mm) thick, which hides what it lies on. This note
records what was measured, five rounds of a per-tissue film in `flesh.wesl`, why none reached GREAT at the preset's cut faces, and the hypothesis for the next attempt. **Nothing here shipped in the shader:**
the attempt is parked on `loop/iter-19-film` (one commit, `src/flesh.wesl` only); `main`'s shader is 0.8.12's. What did ship is the tooling the attempt needed (`## Outcome`).

## What was measured before any change

- **What a film hides**, from `blood_lut()` itself (venous, the film over black and over white; a throwaway example, deleted): the share of what lies under the film that still shows, per channel, against the
  film's thickness. Red lingers where green and blue are gone:

  | film, mm | 0.02 | 0.035 | 0.05 | 0.08 | 0.10 | 0.14 | 0.20 | 0.30 | 0.60 |
  |---|---|---|---|---|---|---|---|---|---|
  | base's red showing | 0.75 | 0.68 | 0.61 | 0.52 | 0.46 | 0.37 | 0.28 | 0.17 | 0.05 |
  | base's green showing | 0.40 | 0.17 | 0.03 | 0 | 0 | 0 | 0 | 0 | 0 |
  | base's blue showing | 0.27 | 0.11 | 0.04 | 0 | 0 | 0 | 0 | 0 | 0 |

  So the tint is the blood's by 0.05 mm, and its lightness and darkness are the tissue's until 0.3 mm or more. The shader's own comment said "opaque past about 0.15 mm"; that was wrong for the red channel and
  is corrected in the parked shader.
- **The old film**, `blood_over(tissue, cut.x * mix(rim, 1, smoothstep(0.08, 0.5, depth_frac)))`, is 0.25 mm at a cap's rim and 1.0 mm from half the layer span down: so every exposed tissue is drawn at 0 to 5 %
  of its own red. On the strip test bed (`capture_flesh --film`, a limb tube with the strip along it, skin right, lean left; the 0.8.12 shader): lean `L*a*b*` (45.3, 39.2, 14.7), fat (48.4, 50.4, 20.8),
  skin (51.7, 37.1, 18.2); **ΔE fat-lean 13.1, fat-skin 13.9, lean-skin 7.6**, lean `L*` standard deviation 1.14. One pink-red tube.
- **In the preset**, `capture_preset --eye 0.30,0.42,0.62 --at 0.0,0.10,0.0` frame 300 (a new flag): the cut faces of the blocky body's pieces are flat wine-crimson sails with a thin pale rim. **Their faces are
  fat**, which a sweep of the constants showed (the marrow's share raised to 0.4 left the faces as they were, the fat's raised to 0.4 turned them dark, and all three at zero left them cream and olive): the pieces
  are small cells, so most of a face lies in the fat's band.
- **The craters are not affected**: a bleeding crater is filled by the wetmap's own film (`capture_skinned --eye 0.0,1.33,0.36 --at 0.0,1.3,0.0`, frames 126-340), which the cut film lies under; old and new
  differ in a fringe at the rim.

## Sources

**Practice (what an artist does, in order).** M. Cramer & Associates, "Prosthetic Wound Makeup: A Step-by-Step Guide" (`https://www.mcramer.com/articles/how-to-color-prosthetic-makeup-wounds/`): the steps
are "Step 2: Apply Base Color", "Step 3: Create Depth and Dimension", "Step 4: Color the Wound" ("For fresh wounds, use vibrant colors like bright reds, pinks, and purples to mimic fresh blood and
inflammation."), "Step 5: Add Texture and Detail" ("use a small brush and a dark red or purple color to paint fine lines, simulating veins"), and last, "Step 6: Apply Blood and Ooze" ("To add realism to the
wound, apply liquid or gel blood. ... To make the blood or ooze appear wet and shiny, add a touch of gloss."). Smooth-On, "How To Make a Quick Silicone Wound Prosthetic"
(`https://www.smooth-on.com/tutorials/quick-fake-wound-prosthetic/`): "Use more red makeup to color the interior of the wound appliance." A gelatine exposed-muscle product page (Chantal Duxfield, read through a
search snippet, the page now answers 404): "Add the supplied fake blood to the middle of the wound after painting to elevate your gore factor!" **The tissue is painted first, and the blood is added last and in
the middle.**

**Physics.** Bosschaart, Edelman, Aalders et al. 2014, "A literature review and novel theoretical approach on the optical properties of whole blood", *Lasers in Medical Science* 29:453
(`doi:10.1007/s10103-013-1446-7`, in the corpus): "In part I ('Methods' and 'Results' sections) of this article, we focus on the absorption coefficient of whole blood. We compile an average μa". It is the
table `crate::bloodstain::spectral` is built on and `flesh.rs` already cites; the table above is that model asked what a film of each thickness hides.

**Technique.** Dambuster's *Dead Island 2* FLESH (Motionographer, `https://motionographer.com/2025/01/27/dead-island-2-a-stunning-visual-zombie-gore-feast/`): zombies have "separate layers of skin, muscle,
fat, and bones" and blood splatters show "on the environment": the layers are real geometry and materials there, which is what the strip and the band uniforms are here.

## Craft principle

Paint the tissue, then put blood on it, and on a cut put it in the middle (Cramer, step 6; the gelatine page). A film is a *thin* thing that tints; only a deep one hides. So a cut tissue should be drawn
as itself under a veil of blood whose thickness is **its own** (a vessel-rich lean redder than a fat), and blood should be deep only where it has collected.

## Reference set

Photographs and stills, kept under `/tmp/c19/refs/` only, cited by URL, deleted at cleanup; none is in the repository.

1. *Hellraiser* (1987), the skinless Frank (`https://static0.srcdn.com/wordpress/wp-content/uploads/2020/11/Hellraiser-1987-Frank-Without-Skin.jpg`): a lean that is **wet red with specular highlights**,
   darker in the recesses (the neck), the muscle's structure visible under the gloss. Not a film of blood: the tissue, wet.
2. ForensicFX Studios, "Deep Fatty Tissue Wound" (image `http://forensicfxstudios.com/cdn/shop/files/FORENSICFX_STUDIOS_23_1200x1200.png`; a prosthetic, not a film): **yellow-orange fat lobules,
   rounded beads 3-6 mm, ringing a red lean floor** under a thin red veil, a ragged pale skin lip outside. The fat is yellow, not red.
3. A painted slash on a cheek (`https://www.etsy.com/listing/1781416100/silicone-gash-prosthetic`; a prosthetic): **dark thick blood in the middle of the cut** with gloss, thinner and paler at the margins.
4. *The Thing* (1982), the defibrillator scene (`https://static0.srcdn.com/wordpress/wp-content/uploads/2024/03/the-defibrillator-scene-in-the-thing.jpg`): a torn chest, the cavity **dark wet red-brown** with
   lit ridges, the skin edge pale.
5. The shank photographs of `docs/research/cut-face.md` (its reference set, 1-6): raw lean, fat and fascia as tissue, for the strip's own colour.

## Model adopted (and tried)

`flesh.wesl`, the exposed-tissue block: `film_mm = cut.x * share`, with `share` per tissue (`TISSUE_FILM_FAT`, `_MUSCLE`, `_MARROW`, none on cortex), the crater's graze as before, and blood pooled only in a
crater's hollow (`POOL_SHARE` of `cut.x` from `POOL_FROM` to `POOL_TO` of the layer span, canvas mode only); shine by film thickness. A **mottle** (a value noise in two octaves over the cut face's planar
`UV_0`, 9 mm patches, 0.45 to 1.55 of the share) breaks up the coat. The final numbers: fat 0.035, lean 0.14, marrow 0.04 (mm, with `cut.x` 1.0).

## Rounds

Five, each baked, rendered on three beds (the strip tube, the heap of pieces, a crater) and looked at. The verdicts are mine and each was NOT YET.

1. **One thin film per tissue, no pattern** (fat 0.06, lean 0.05): the strip reads (lean red with fibres, fat, skin), but the heap's faces, which are fat, are **flat coral**: a thin red veil over cream is
   salmon, uniform and bright. "Candy red" again (0.5.38), now by the other route.
2. **A value-noise mottle** (0.3 to 1.7 of the share): orange patches on red, *salami*. The topology matches reference 2 (yellow-orange lumps on red) and the contrast is wrong.
3. **A cellular net** (Worley cells with red septa round cream lobules, the anatomy): *cracked paint*. FAIL.
4. **Hard-edged lumps of bare fat on red**: *camouflage*.
5. **Soft mottle, shares fat 0.035 / lean 0.14**: the strip reads best of all (below), the heap's faces are peach and orange-red patches. Soft, organic, and **too bright and too orange** beside references 1 and 4, which
   are dark, wet and red, and without the beads of reference 2.

## Critique (against the reference set), written before the verdict

- Against **1** (*Hellraiser*): the lean is right in hue where it is thin, but there are no specular highlights on a cap: a flat plane under the studio's key light shows none, and the coat is `0.3 x 0.75`. The
  reference is wet because it is *lit* wet.
- Against **2** (fatty wound): the fat is the right colour range now (peach to orange), but it is a noise and the reference is beads: a lobule needs relief (a normal), which a film model does not give.
- Against **3**, **4**: **the heap's faces are too bright.** A freshly broken body's cut faces are bloody (reference 4 is the cavity of a torn chest, dark), and the 1 mm film, which looks flat, was *right in
  value*. A thin film shows the tissue and loses the blood; a thick one the reverse; **one static film cannot be both** and the preset has no per-face blood to carry the difference.
- The strip bed, after: lean `L*a*b*` (49.3, 43.8, 20.4), fat (64.4, 39.6, 31.4), skin (72.6, 12.6, 11.6); **ΔE fat-lean 19.2, fat-skin 34.5, lean-skin 40.0** against 13.1, 13.9 and 7.6; lean `L*`
  standard deviation 1.75 against 1.14. The tissues are legible on the strip: that part works.

**Verdict: NOT YET** at the preset's cut faces, after five rounds. The rule says stop and revert to the last GREAT state; the shader is 0.8.12's, and the attempt is on `loop/iter-19-film`.

## Hypothesis for the next attempt

1. **The film's thickness should be a function of time since the cut**, not a constant: a face freshly broken is bloody (the old look, which was right in value), and it drains and clots over seconds, so that
   the tissue comes out of the blood. `FleshParams::cut.z` (unused) can carry a bleed-out share; the preset sets it per piece on a schedule in whole steps, quantised so a piece costs a material write a
   second and not a tick. This is the reading of Cramer's order that the static film lacks: blood last, *and less of it as it dries*.
2. **Fat lobules are relief, not colour**: a bump normal from the same cellular function (the `crack_cells` already in the shader) with the film's thickness in the grooves would give reference 2's beads
   without the paint look of rounds 3 and 4.
3. **The scale of the strip is a beef shank's** (12 mm bellies); at a game camera's 5 mm a pixel a belly is 2 pixels. A human thigh's bellies are 40-60 mm (the Visible Human section in `cut-face.md`'s reference 6).
   That is a strip change (a golden re-bless), and it is where the cap's structure would come from at a distance.

## Validation targets that were set, and what was measured

| target | measured, final parked shader |
|---|---|
| tissue legible on the strip bed: ΔE fat-lean at least 18, skin to either at least 30 | 19.2, 34.5 and 40.0: met |
| the lean's structure shows: `L*` standard deviation over the lean at least 1.5x the old film's | 1.75 against 1.14, 1.5x: met |
| no candy red: the fat's `a*` below the lean's | 39.6 against 43.8: met |
| the heap's faces read as a fresh cut beside references 1 and 4 at a glance | **not met**: too bright, too orange |
| WebGL2 pipeline | **OK** on big, Firefox on Xvfb with Mesa llvmpipe, WebGL 2.0: build 26 s warm, 3 min 24 s cold (not the 38-79 minutes recorded for the Mac) |
| cost | none expected: arithmetic in the fragment shader, no binding; not measured (the shader is parked) |

## Outcome

**Shipped (tooling, no look change).** (1) `tools/webgl2/check.sh` drives **Firefox on an Xvfb display** when there is no Chrome or no Node 22 (`WEBGL2_DRIVER=firefox` forces it), the AG-109 route as a script:
on big it ran the whole check in 72 s. It sends no input, so `CLICKS` is ignored there. (2) `capture_preset --eye x,y,z --at x,y,z`, so a cut face is seen at the pieces' own scale. (3)
`capture_flesh --film`, the limb strip under the preset's film of blood: the test bed for any change to the film (the 0.8.12 recorder showed the strip bare and so could not).

**Parked.** Two branches, neither merged, neither deleted. `loop/iter-19-film`: the static per-tissue film (the shader only). `loop/iter-19-drain` (`2959bbe`): the second attempt below, the shader,
`FleshParams::with_drained`, the preset's registry and system, their tests and the recorders' flags. To adopt either, `git checkout <branch> -- <paths>`; start from the hypotheses at the end of the second attempt.

## The second attempt: a cut face drains (hypothesis 1, built)

**The mechanism, built and tested** (`loop/iter-19-drain`). `FleshParams::cut.z`, the share a face has drained, `0` as shipped, `with_drained(share)`; the shader's exposed-tissue block draws the old film at
`0` (byte for byte, the same arithmetic) and the per-tissue film at `1`, blended **by the log of the film's thickness** (a linear blend spent the last third of a drain doing all its work: a film of 0.4 mm
hides what lies under it about as much as one of 1 mm). The schedule, `preset::cut_face_drained(age, blood)`: `0` until `clot_ticks` (the time `CutFaceBleed` pours until), a smooth rise to `1` over as long
again, written to the pieces' shared cap material in sixteen steps (`CUT_FACE_DRAIN_STEPS`) by `drain_cut_faces`, from a `CutFaces` registry the break fills; two tests (the schedule's shape; two bodies broken
at different ticks, one cap material each, drained at different times, the registry empty when done). **Cost** none measured on a tick without a break (a resource and a system that does not run while the
registry is empty); sixteen material writes a body over twelve seconds. `capture_preset --frames N --stride K`, `capture_flesh --drained S`.

**The sixth to tenth looks of the film, all of the drained state** (the fresh state is 0.8.12's, and was never touched), rendered on the heap (`capture_preset --eye 0.30,0.42,0.62 --at 0.0,0.10,0.0 --frames 900
--stride 30`, frame 870) and the strip bed (`capture_flesh --film --drained 1`):

| drained state | fat / lean film, mm | what it looked like |
|---|---|---|
| thin, tissue shows | 0.012 / 0.12 | the strip bed reads best of all (lean red with fibres, cream fat, pale skin) and the heap's fat faces are **cheese** |
| thin, redder fat | 0.022 / 0.12, 0.035 / 0.14 | **salmon**, then **coral** |
| clotted: the dried-blood table at 0.4 of its age | 0.016 / 0.12 | **brick**: flat, the aged table hides the tissue |
| clotted at 0.12 and 0.2 | 0.016-0.03 / 0.12 | **terracotta** |
| bloody but textured | 0.10 / 0.25 | crimson with the tissue's tone in it: **the best, and a modest change**: the flat wine plates of 0.8.12 become a mottled red |

**Critique (against the reference set), written before the verdict.** The mechanism does what it was built to: a face just broken is what it was, and then the film thins on the clock. What it thins *to* is the
trouble. Every state thin enough to show the strip reads as **food** beside references 1 and 4 (the cheese, the salmon, the terracotta), because the strip's tissue tones were tuned to retail beef and fat under a studio
light (`cut-face.md`), where the horror references are **dark, wet and bloody** with their tissue in relief and highlights. The state that keeps the blood (the last row) is not food and is a modest change on
flat plates at a camera that shows a piece a few tens of pixels across. The change is real, small and not worth a mechanism, a resource and a uniform on its own.

**Verdict: NOT YET**, and the fifth failed round of the second design; stopped by the rule, reverted on `main` and parked, with these as the next hypotheses:

1. **Wetness is relief and specular, not colour.** Reference 1 reads as meat because it is lit wet: a bump from the strip's own structure (its fibres and septa as a height) would break a face's mirror into
   glints with no change of colour. That is the version of the cap with a normal map, one more sample in `flesh.wesl`, and it would work at the fresh state too.
2. **Clotted blood in the hollows and tissue on the ridges**, not a film of one thickness over a plane: a height (the same bump) and a film that pools in its low places.
3. The strip's scale (hypothesis 3 above).

## Pending index

- Not in the corpus: a measured reflectance of a *fresh* cut surface of muscle and fat under a thin blood film (the nearest, Bosschaart's whole-blood spectra and the beef measurements of
  `cut-face.md`, are for the two things separately). To obtain by `paper_search`: "reflectance spectra of bloodied tissue", "optical properties of freshly excised muscle and adipose tissue".
- The Chantal Duxfield page was read through a snippet and now answers 404; Cramer and Smooth-On were read in full.
