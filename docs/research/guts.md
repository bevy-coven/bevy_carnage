# Guts that read as guts: bloodied, lumpy, slumped, glossy

Iteration 12 (2026-11-01). Item: what a spilled strand of bowel looks like. Ranked first by the owner's run instruction (replicate 80s
practical-effects gore, "things like *The Thing*"), and by the measurement below: of the seventeen ideas drawn from the reference stills, this is the one that is
CPU-only, moves no digest, needs no shader (the WebGL2 cache is gone: a cold wasm build is about 79 minutes), and is visible in the hero `preset.gif`.
Runner-ups, kept for the log: slime strings (a *Thing* and *Hellraiser* look, needs a thread primitive), the wound interior and a real skin lip
(the biggest gap, but a `flesh.wesl` change or a re-bless of `the_strips_are_frozen`), a burst hit (*Scanners*).

## What was measured before any change

`capture_preset`, frame 130 (guts hanging from the torso) and 200 (lying in the pool), on 0.8.5, `/tmp/c12/before/preset/`:

- The strands are **clean salmon-pink tubes**. Reddish pixels of the hanging strands at frame 130 (box `320,232,110,95`, the pool excluded, a mask of
  `R > 1.35 G and R > 1.35 B`): mean sRGB `(126, 68, 66)`, red minus green 58, saturation 0.48, luminance 80.5. A strand pixel in the pile at frame 200 is
  `(171, 102, 99)`, luminance 115; the pool they lie in (box `240,312,50,18`) is `(123, 51, 56)`, luminance 67: the guts are paler than the blood they lie on,
  which a real spill never is.
- Every strand has **one radius**, `SPILL_RADIUS` 20 mm, so each is a smooth pole, and where it lies on the floor it is a round tube
  resting on a line. Real bowel is lumpy and slumps flat under its own weight.
- The texture (`look::gut_image`) carries blood on the wound end and in patches, but the material multiplies nothing by how much blood the strand
  is actually in: a strand lying in the middle of the pool is as clean as one hanging in the air.
- The wall is `GUT_ROUGHNESS` 0.3 and has no coat (the flesh shader draws a clear coat only from a wet canvas, which a strand does not have, and overwrites
  `StandardMaterial::clearcoat` every fragment): the highlight is one soft streak, where the *Thing* stills have sharp white glints.

## Sources

- **Physics.**
  - Bowel anatomy, which sets which side the mesentery and its fat are on and why a wall is pale where it is clean: from the mesenteric-entry
    study in the corpus (`10.1007_s00423-024-03318-8`, *The mesenteric entry site as a potential weak point in gastrointestinal anastomoses*,
    Langenbecks Arch. Surg. 2024): "Schematic cross-section of the small intestine. a = Tunica mucosa; b = Tunica submucosa; c = Tunica
    muscularis; d = Tunica serosa; e = Mesentery". The outer coat a viewer sees is the serosa; `look::gut_image` already puts the mesentery's fat at `u = 0`.
  - Blood over a pale substrate: the crate's own `bloodstain::spectral` (Kubelka–Munk over Bosschaart et al., *A literature review and novel
    theoretical approach on the optical properties of whole blood*, Lasers Med. Sci. 29 (2014), `doi:10.1007/s10103-013-1446-7`). No blood colour is
    authored here: a strand's tint is the colour that a film of venous blood of the thickness it carries gives over the same pale substrate, divided by
    the substrate's own, which is what a texture multiplied by a vertex colour computes.
  - Blood's own drying and wetting (`docs/research/drying-front.md`, Smith, Nicloux & Brutin 2020): a film this thin on tissue is the oldest blood on
    the strand, so it dries first; this note does not age it (recorded under `## Deviations`).
- **Horror-film and practical-effects references**: `## Reference set`.
- **Technique.**
  - Evans-Lawes, technical art director of *Dead Island 2*, on its FLESH system, Reddit AMA as reported by Kotaku (2023-05-08,
    `https://kotaku.com/ama-reddit-dead-island-2-weapons-flesh-system-ps5-xbox-1850415887`): "When I was doing the skin/fat/muscle blending I looked at some
    real pictures of surgery because you actually get a much clearer look at the structures and colors that way, but in terms of the end results we wanted
    to achieve, I mainly looked at practical effects in movies such as *RoboCop*, *Starship Troopers*, *Day of The Dead* (and many more)." And, on the guts:
    "the other was filling the resulting hole with destructible guts".
  - Wet surfaces: Lagarde, *Water drop 3b, physically based wet surfaces* (2013), quoted in iteration 11: "Simply interpolating between dry and wet BRDF
    parameters without changing the lighting model do the effect". And `10.1007_s00371-020-01963-w` (Visual Computer 37, 2021): "the presence of liquid
    on the surface alters significantly their BRDF, which in turn results in considerable changes in their visual appearance".
  - Glossy coat, **not used**: glTF `KHR_materials_clearcoat` is what `StandardMaterial::clearcoat` implements, but `flesh.wesl` overwrites `clearcoat` every fragment from its
    own wet canvas, so the first plan (a coat on the gut material) could not work without a shader change; see `## Model adopted`.

## Craft principle

A practical-effects artist makes guts read as guts by three things the stills below show, and a modeller's clean pipe has none of them:

1. **They are soaked.** Savini's were real animal guts and a slaughterhouse's blood: "Pig's blood and guts were obtained from a local slaughterhouse for use in the scene"
   (Screen Rant, *Day of the Dead* behind-the-scenes facts). The stills show the bowel red, darkest where it came out and where it lies; the pale serosa shows only
   where the blood is thin. Bottin's kennel mass was "mayonnaise, melted bubble gum and plenty of K-Y Jelly" (Dead Northern, *The Thing* retrospective): the **slime** is
   what makes it shine.
2. **They are not a constant radius.** Real intestine is lumpy, and a loop lying on the floor spreads (general anatomy, not measured here; R1 and R2 show it).
3. **They shine in points, not in a smear.** The *Thing* stills (split-face, Palmer) have a clear wet coat over a darker body: sharp white glints.

## How shipping games do it

*Dead Island 2* (Dambuster) builds a zombie "out of a surface mesh and a set of 'viscera' meshes", "individual pieces that can be destroyed, or broken", and fills "the resulting
hole with destructible guts" (Kotaku, quoted above), after looking at *Day of the Dead*. This crate's guts are tubes swept on the CPU, so the model is: **the tube is the same
mesh it was, with a per-vertex tint and a per-ring radius**, and a lower roughness. Nothing here is a texture, a new binding, or a shader branch.

## Model adopted

All of it cosmetic, outside every digest; a pure function of the strand's own nodes, its seed, and the floor height.

1. **A radius profile.** `r_i = r * (1 + 0.22 sin(2 pi (i / 5.5 + a)) + 0.09 sin(2 pi (i / 2.3 + b)))`, `a` and `b` from a seed that `spill` stores on the
   strand (`Strand::with_seed`), so no two strands lump alike and the same spill lumps the same way on every run. A strand built by hand has seed `0`.
2. **A slump.** A ring whose centre is within `SLUMP_FADE` (1.4) radii of resting on the floor is squashed to `SLUMP_SQUASH` (0.72) of its height and widened by
   the inverse square root, so the area is kept; the normals are those of the ellipse, leaned back by the radius's slope.
3. **A soak per node**, in `[0, 1]`: `max(base, exp(-arc / WOUND_REACH_M), contact * (0.8 + 0.2 n))`, `arc` the length from the wound end (node 0 and the node
   after any tear), `contact` a smooth step on the node's height above the floor (1 resting on it, 0 a radius and 1.4 above), `n` a smooth noise of the length
   along the bowel on a 12 cm wavelength, and `base = SOAK_BASE * (1 + SOAK_PATCH * (2n - 1))`: a spilled strand is slick everywhere and patchy.
4. **The tint** is the venous film of `SOAK_FILM_MM * soak^3` over a `0.5` grey substrate, `spectral::linear_srgb`, divided by the substrate alone, per channel, from a
   table of `SOAK_STEPS` entries built once. The cube is a measured fix: on a linear scale the first round (`soak * 0.35 mm`) put 90 percent of the range at one
   dark red, because venous blood is opaque in green by 0.04 mm (the table's green is 0.131 at 0.035 mm and 0.031 at 0.07 mm). It is written as
   `Mesh::ATTRIBUTE_COLOR` and the material multiplies it into `gut_image`.
5. **A lower roughness**, `GUT_ROUGHNESS` 0.3 to 0.15. (The first plan was a clear coat on the material; the flesh shader overwrites `clearcoat` from its own wet
   canvas every fragment, so it cannot be set from a `StandardMaterial`, and a shader change is a 79-minute WebGL2 re-check. Roughness 0.10 measured the same
   highlight as 0.15 at the preset's camera, so 0.15 stays.)

The public additions are `Strand::with_seed`/`seed`, `viscera::soak`, `soak_tint`, `soaked_tube` and their constants; `tube_mesh` keeps its signature and gets
lumps (not the slump or the tint, which need the floor).

## Parameters

All **tuned by eye against the reference set** (this note), none measured: `SOAK_BASE` 0.45, `SOAK_PATCH` 0.5, `SOAK_PATCH_M` 0.12, `WOUND_REACH_M` 0.30,
`SOAK_FILM_MM` 0.35, `SLUMP_FADE` 1.4, `SLUMP_SQUASH` 0.72, the lump amplitudes 0.22 and 0.09 (0.14 and 0.06 in the first rounds) and periods 5.5 and 2.3 nodes, `GUT_ROUGHNESS` 0.15. Each says so in its
doc comment.

## Validation targets

- Red minus green of the hanging strands' reddish pixels at frame 130 (box `320,232,110,95`): from 58 to **at least 80** (Ref D1's mid-tone is 102 by the same mask).
- The strand in the pool is **not paler than the pool**: a strand pixel's luminance is no more than the pool's plus 25 (it is 48 above now: 115 against 67).
- The silhouette of a strand has **two scales of lump** in its radius: a tube's radius varies by at least 12 percent along its length (`tests/viscera.rs`).
- A strand on the floor is wider than it is tall by at least 1.2 (a test).
- `Strand::digest`, `entrails=`, `digest=` of every recorder, `the_flesh_tables_are_frozen` and every other lock are unchanged.
- Cost: `carnage_frame_ms` within 5 percent; the retube per tick is the same loop with two more sines per ring and one table lookup.

## What great looks like

Day of the Dead's guts are **saturated red and dark where they are deep, with a wet highlight where a strand's top turns to the light**, and they lie in a heap
that overlaps itself. The kennel and Palmer-thing stills add that the shine is a **hard, small, white glint** over a dark body. A frame is GREAT if, covering
the pool, the strands are the colour of the pool or darker; if each has a visible lump; if the highlight is a point; and if a strand lying on the floor reads as
flattened against it.

## Reference set

Stills are copyrighted film frames, kept under `/tmp/c12/refs/` for judging and never committed; this note cites their source pages.

| Ref | Film / source | Page | What it shows that the crate must reproduce |
|---|---|---|---|
| D1 | *Day of the Dead* (1985), Savini | `https://www.truemythmedia.com/true-myth-media/reviews/dayofthedead` | guts hauled from an abdomen: saturated red; reddish pixels of the gut region (mask `R > 1.5 G and R > 1.5 B and R > 60`, 37 925 pixels) mean sRGB `(138, 36, 31)`, red minus green 102, hue 2.9 degrees, saturation 0.77, luminance 57.6, 5th percentile 16.5, 98th percentile 136.8; strands stretched taut and slick, hard glints |
| D2 | *Day of the Dead* (1985) | `https://www.imdb.com/title/tt0088993/mediaviewer/rm947079168` | the opened torso: bright wet red meat over pale ribs, blood pooled on the table |
| D3 | *Day of the Dead* (1985) | `https://happyotter666.blogspot.com/2014/05/day-of-dead-1985.html` | the same subject at a distance: a **heap**, coils overlapping, one saturation |
| T1 | *The Thing* (1982), kennel | `https://en.wikipedia.org/wiki/File:TheThing(1982)-KennelSFX.jpg` (fair use) | red wet strands piled on straw: dark body, bright specular, thick |
| T2 | *The Thing* (1982), split-face | `https://thething.fandom.com/wiki/Split_Face` | a dark wet mass with sharp white glints |
| T3 | *The Thing* (1982), Palmer | `https://screenrant.com/best-sci-fi-movie-practical-special-effects/` | clear slime in strings over wet flesh (the next pass's item) |
| H1 | *Hellraiser* (1987), Frank | `https://makeupeffectsarchive.blogspot.com/2025/07/hellraiser-1987-part-1-frank-cottons.html` | dark red-purple wet tissue under a clear sheet of slime |
| R1 | pig small intestines, Wikimedia Commons, CC0 | `https://commons.wikimedia.org/wiki/File:Pig_small_intestines.jpg` | the real material clean: pale pink, translucent, lumpy, rope-like in a heap |
| R2 | Czech pig-killing day, CC BY 2.0 | `https://commons.wikimedia.org/wiki/File:Czech_traditional_pig-killing_Day._The_intestines_are_not_used._The_long_brown_bit_is_the_spleen.jpg` | fresh bowel in a tub: pale loops with blood on them |

## Deviations

- The crate's `look.rs` said the strand was "a salmon-pink serosa ... glossy and a little translucent" and chose it "by eye against the blood colour the wetmap
  draws". That is the *clean* serosa; the guts of a *spill* are not clean (R2 and D1). The texture is **kept** (it is the substrate); what changes is what
  multiplies it. Design default.
- The soak does not age: a strand's blood does not dry in this pass. The wet maps dry on the canvases, not on a strand, and a strand has no canvas; a strand
  that dried would need a clock, and the viscera module has none (`CLAUDE.md`). Recorded as the next item.
- The tint is a multiplier, so the strand can only darken from the texture's pink: the bright arterial red of D1 is reached by the film being thick and the
  coat reflecting, not by adding red.

## Pending index

- Bosschaart 2014 full text: the crate's tables are from it; it is not in the home-still corpus as a converted document.
- Lagarde 2013 and the Visual Computer 2020 wet-absorbent paper: abstracts only (paywalled / blog).
- No home-still paper on **intestinal serosa optical properties** was found (`distill_search`: nothing relevant on the first page); the colour here is
  therefore matched to the reference set, not derived.
