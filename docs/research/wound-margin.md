# A wound has a margin and a floor

Iteration 14 (2026-11-01). Items 6 and 7 of the owner's 80s-horror list: "wound-interior detail" ("muscle striation, pale fascia, marbled fat, sharp glints ... the biggest visible gap") and "a real skin
lip" ("a thin pale dermis edge over a yellow fat line. The current pink halo is wrong against every still"). **Why the shader route is not this pass's**: the detail finer than a canvas texel can only
come from `flesh.wesl`, and a change there must be re-checked in WebGL2 (`CLAUDE.md`, `tools/webgl2/check.sh`: a cold wasm build of Bevy, 38 to 79 minutes and several GB) on a disk with **4.3 GB free**. So the
pass takes what the canvas can carry: a wound the size of a hand or a chest (a texel is 3.9 mm on the 256 canvas of `capture_gore`, whose craters are 12 to 17 cm in radius, 30 to 44 texels) and every wound's margin.
Runner-ups, in the log: an opened chest with ribs (needs the canvas to know which way is up), a burst hit (a big composition), slime strings (parked), the sharper glint (shader).

## What was measured before any change

`capture_gore` frame 260, the crater (crop `170,90,300,300` of the 640 by 480 frame; reddish pixels, mask `R > 1.5 G and R > 1.5 B and R > 40`) against six wound-makeup close-ups (`## Reference set`):

| | reddish pixels, mean sRGB | red minus green | luminance mean | 5th pct | 95th pct | texture contrast, `hp3` | `hp6` |
|---|---|---|---|---|---|---|---|
| the crater, 0.8.7 | `(134, 48, 47)` | 86 | 66.5 | 34.5 | 109.9 | **0.057** | **0.081** |
| the six references | `(138..167, 49..75, 39..65)` | 86 to 109 | 67 to 94 | 16 to 27 | 134 to 165 | **0.29 to 0.37** | **0.32 to 0.43** |

The colour is already inside the references' range; **the interior is five times too smooth**. "Texture contrast" is the standard deviation, over the reddish pixels, of the luminance minus its Gaussian blur
at 3 % (`hp3`) and at 6 % (`hp6`) of the wound's diameter, over the mean luminance (`/tmp` script, not shipped): the same measure on every picture, scaled to its wound, so a 90 mm prosthetic and a 300 mm crater compare.
The deep darks (5th percentile 34.5 against 16 to 27) and the pale and glinting highs (95th 110 against 134 to 165) are missing for the same reason: there is nothing in the floor to be dark or bright.
The crater's colour comes from depth alone (`FlayCanvas::shade`: the layer table at the texel's depth, torn by two octaves of value noise of at most 3 mm at 24 mm and 9.6 mm), and the torso's table is
2.2 mm of skin, **16 mm of fat**, 7.3 mm of muscle, 2 mm of cortex and 4 mm of marrow, so a smooth bowl 30 mm deep shows a **pale, orange ring 57 % of the radius wide** (the model: depth under the skin plus the fat,
18.2 mm, out to 0.572 of the radius, because the bowl's rim has no slope), then a red floor with no features. The pink halo of item 7 is that ring.

## Sources

- **Forensic pathology** (the margin), already read for `docs/research/torn-exits.md`: *Gunshot Wound Patterns: A Narrative Review From a Forensic Medical Perspective* (PMC, `https://pmc.ncbi.nlm.nih.gov/articles/PMC12791185/`):
  "Entry wounds are typically smaller, with well-defined margins and tissue invagination"; Medscape, *Forensic Pathology of Firearm Wounds* (`https://emedicine.medscape.com/article/1975428-overview`): "Entrance
  wounds typically present as circular defects with marginal abrasions". **A wound has an edge**: the skin is cut or pushed in along a line, not eased into a bowl; the 0.8.7 crater's rim has zero slope
  (`smoothstep`'s derivative at its end), which is why the skin and fat bands are so wide.
- **The tissue** (the floor), the crate's own anatomy, `src/cross_section/strip.rs`: "adipose tissue is lobules a millimetre or two across held in a fibrous septal net; skeletal muscle in cross-section is fascicles
  — bundles of fibres — wrapped in a pale perimysium". At a flaymap texel (3.9 mm on the 256 canvas) those are finer than the texel, so `texel_mean` averages them away, and the floor is a gradient. A torn floor
  is lumpy at a larger scale, the lumps of the references: 4 to 8 mm nodules of cream fat and torn fascia between dark red crevices. This is **not** a measured anatomy: the scale and the contrast are matched to the
  references and said so under `## Parameters`.
- **Horror-film and practical-effects references**: `## Reference set`.
- **Technique.** Vlachos, *"Rendering Wounds in Left 4 Dead 2"*, GDC 2010 (`https://alex.vlachos.com/graphics/Vlachos-GDC10-Left4Dead2Wounds.pdf`): "Use projected texture for rough edges and blood layer". A shipping game
  authors the wound's edge and floor as a texture; this crate's floor is a CPU depth buffer shaded on the CPU, so the lumps are shaded into the albedo, roughness and normal images it already uploads, from a hash of
  the texel's place, and no texture is projected and no binding is added.

## Craft principle

A wound is read by its **edge** (a thin, pale, raised skin lip with a defined line, not a ring eased into the skin), its **floor** (knobbly: cream nodules of fat and fascia in dark red crevices, deeper
and darker toward a pit), and its **wetness** (glints on the nodules). A smooth gradient is a bruise or a burn, not a wound. The 0.8.7 crater has the colours and not the structure.

## How shipping games do it

Left 4 Dead 2's wounds (Vlachos 2010: "Use projected texture for rough edges and blood layer") put the structure in authored art: an edge mask and a flesh texture projected over a pose-space ellipsoid that culls
the skin. The structure is the artist's, so every wound of a texture looks alike (the "5 variations only" of Left 4 Dead 1, `torn-exits.md`). Here the structure is **generated from the place** (a hash of the
texel's millimetre position and the seed) so that no two wounds are alike and a wound is a pure function of the hits that made it.

## Model adopted

1. **The crater's profile** (`build_mask`, `build_torn_mask`'s crater): the falloff `w(t) = (1 - B) S(min(t / K, 1)) + B S(t)` with `t = 1 - d / r`, `S` the smoothstep, `K = WALL` the share of the radius the wall
   takes and `B = PIT` the share of the depth that keeps deepening to the centre. `K = 1, B = 1` is the 0.8.7 bowl. **A model change**: the depth buffer, so `the_scripted_wound_is_frozen` and every recorder's `flay=`
   move, deliberately and together; nothing else in a digest does (the wet and dermis canvases, `Strand`, the bench).
2. **The floor's lumps** (`FlayCanvas::shade`, outside every digest): a Worley field at `LUMP_CELL_MM` over the peeled texels, faded in with the depth (`LUMP_FADE_MM`) so the lip stays smooth. Its cell's interior
   is a nodule (a little paler and warmer, a little higher), its seams are crevices (darker, lower); it moves **the lookup depth** (so the fat and the muscle show in patches), **the albedo** (the light and
   the dark), **the roughness** (nodules wetter) and **the normal** (the lump's own height added to the depth's slope), so the shader's lighting sees them.
3. **The lip** (`shade`, outside every digest; **not** `strip.rs`, so `the_strips_are_frozen` is untouched): the first `LIP_MM` (3 mm, varied along the rim by the slow noise that tears the bands) of depth is
   the skin's cut edge, pushed toward a colour paler than the skin round it (`LIP_COLOUR`) and raised 1.8 mm in the normal image; the wall puts that band a texel or two wide.
4. **The rim** (`build_mask`): a crater of eight texels' radius or more has its rim moved by three harmonics of its angle (3rd, 5th, 8th), 3.5 % of the radius each at 32 texels, from a hash of the place; a
   bullet's hole stays round. `paint_uv_torn`'s crater takes the new profile and keeps its own two harmonics.

## Parameters

All **tuned by eye and by `hp3`/`hp6`** against the reference set, none measured: `WALL` 0.45, `PIT` 0.30, `RIM_RAG` 0.035 from 8 to 32 texels, `LUMP_CELL_MM` 16 (a second lattice 2.3 times coarser, a swell at 55 mm, a
warp at 30 mm, patches at 40 mm), `LUMP_RELIEF_MM` 5.0, `LUMP_LOOK_MM` 3.5, `LUMP_DARK` 0.6, `LUMP_LIGHT` 1.1, `LUMP_PALE` 0.3, `LUMP_WET` 0.6, `LUMP_FADE_MM` 4, `LIP_MM` 3, `LIP_PALE` 0.75, `LIP_RELIEF_MM` 1.8. Each
says so in its doc comment. **A diagonal weave appeared at 12 mm cells with a second octave at 6 mm (1.5 texels); one octave at 16 mm removed it** (the cause, the octave or the cell size, was not isolated).

## Validation targets

- `capture_gore` frame 260, the crater: `hp3 >= 0.25` and `hp6 >= 0.30` (the references' range is 0.29 to 0.37 and 0.32 to 0.43); reddish pixels within the references' range in red minus green (86 to 109) and
  luminance (67 to 94), the 5th percentile at most 27 and the 95th at least 125.
- The pale ring: its width, measured on the **model** (the share of the radius where the depth is under the skin plus the fat) at least a third narrower than 0.8.7's, and still a visible ring (at least one texel).
- Every digest that does not fold the depth buffer equals 0.8.7's: `bench_carnage` (`0x4366a8b64352c85a`), `capture_entrails` (`0x7d73a34dd4befb08`), `capture_flesh` (`d3150b10f27bcdd5`); the wet and dermis
  digests of `capture_preset` and every `capture_skinned` variant; and the flay digest of a recorder two runs of one build agree on.
- `shade` costs no more than twice what it did per peeled texel (the bench does not run it: a throwaway test).
- **No lumps on intact skin and none on the first millimetre** (a test), no non-finite byte, the same bytes twice, and the lump field periodic at no seam (the canvas has none: UV 0 and 1 are different places).

## What great looks like

Beside W2 and W5: a wound whose floor is knobbly (nodules and crevices, not a gradient), with deep darks and glints; a thin, defined, pale lip with a line; the fat a yellow nodule here and there and not a ring.
At a glance, from the distance of a player, it reads as torn meat and not as a target.

## Reference set

Product and tutorial photographs of wound makeup by effects artists (prosthetic appliances and silicone wounds sold for film and stage); copyrighted by the sellers, kept under `/tmp/c14/refs/`, never committed; this
note cites their pages. Reddish-pixel statistics as in `## What was measured before any change`.

| Ref | Source | Page | What it shows that the crate must reproduce |
|---|---|---|---|
| W1 | JessFX *Realistic Close Range Gunshot Entry Wound* | `https://www.treasurehouseofmakeup.co.uk/jessfx-prosthetic-close-entry-wound.html` | a glossy dark pit with a point glint, a thick pink-red rolled lip, blood smeared round it; `(149, 51, 43)`, `hp3` 0.286 |
| W2 | *Large Gory Gunshot Exit Wound Prosthetic 2* | `https://www.etsy.com/listing/1254209884/large-gory-gunshot-exit-wound-prosthetic` | a ragged, rolled pale skin edge round a floor of cream and red nodules the size of peas, a dark pit; `(138, 49, 39)`, 0.340 |
| W3 | *Trauma Prosthetic A* | `https://www.etsy.com/listing/1202966633/trauma-prosthetic-a-encapsulated` | torn flaps of pale skin with a sharp edge over a dark knobbly floor; `(152, 66, 55)`, 0.372 |
| W4 | *Deep Fatty Tissue Leg Wound* | `https://www.etsy.com/listing/752440362/deep-fatty-tissue-leg-wound-silicone` | yellow-orange fat nodules among red; `(167, 75, 65)`, 0.348 |
| W5 | *Gunshot Exit Wound 3* | `https://www.etsy.com/listing/1704072920/gunshot-exit-wound-3-prosthetic` | a rolled glossy ring round a dark pit with a hard glint; `(161, 52, 40)`, 0.311 |
| W6 | *Trauma Prosthetic B / Deep Laceration* | `https://www.etsy.com/listing/1667544396/trauma-prosthetic-b-deep-laceration` | a laceration with a lifted lip and a knobbly red floor; `(159, 68, 44)`, 0.340 |
| D2 | *Day of the Dead* (1985), the opened torso | `https://www.imdb.com/title/tt0088993/mediaviewer/rm947079168` | a cavity of wet red masses over pale ribs (`docs/research/guts.md`) |

## Deviations

- **The depth profile changes** (`WALL`, `PIT`) and **a big crater's rim is out of round** (`RIM_RAG`): a deliberate model change and not a "fix", named in the changelog and the log, re-blessing `the_scripted_wound_is_frozen`
  in the same commit. `capture_flesh`'s digest moves with it (it folds the flay canvas's), which this note's targets did not foresee.
- Small wounds are as smooth as a canvas lets them be: a bullet's crater is a few texels across and its floor has few lumps (the preset's entry wound shows them as many small glints); a wound's floor finer than a texel is what
  `flesh.wesl` would give it (a WebGL2 re-check, blocked by the disk): **the next item once the disk allows it**.
- **No ribs, no flaps**: an opened chest with a rib layer needs the canvas to know which way is up, which no part of the crate does today.

## Outcome

Thirteen renders of `capture_gore` (the crater) and two of the preset's own close-ups (`capture_skinned --through`, entry and exit; each also at 0.8.7), judged on the pixel measures above and by eye beside W1 to W6.
Rounds, in the order they changed the answer: the steeper wall alone (K 0.45, B 0.3): the ring halves, the floor still smooth (`hp3` 0.064); Worley lumps at 9 mm with a second octave: 0.096, then 0.147 and
0.206 as the contrast and relief rose, and **a diagonal weave** (12 mm cells); one octave at 16 mm: 0.248, a regular raspberry field; a warped lattice, two sizes of nodule and a swell: more organic, 0.20; lumps
stop at the bone and the lip added: 0.238; the lip's width varied along the rim; colour (lighter crevices, brighter tops, less pale tint) last, because the blood film over a fresh wound hides the albedo of the
tissue and the lit relief is what the eye reads.

| | crater, 0.8.7 | crater, now | the six references |
|---|---|---|---|
| reddish pixels, mean sRGB | `(134, 48, 47)` | `(122, 42, 43)` | `(138..167, 49..75, 39..65)` |
| red minus green | 85.7 | **80.0** | 86 to 109 |
| luminance mean; 5th; 95th percentile | 66.5; 34.5; 109.9 | **58.9**; 26.1; **105.5** | 67 to 94; 16 to 27; 134 to 165 |
| texture contrast `hp3`; `hp6` | 0.057; 0.081 | **0.236; 0.266** | 0.29 to 0.37; 0.32 to 0.43 |
| the pale ring, share of the radius (model, 30 mm deep) | 0.572 | **0.310** (skin alone 0.166 to 0.087) | |

The preset's own close-ups (`capture_skinned --through --eye 0,1.3,±0.75 --at 0,1.25,0`, frames 200 and 250): the entry's `hp3`/`hp6` go 0.082/0.121 to 0.157/0.240 and the exit's 0.155/0.211 to 0.205/0.267.

**Against the targets written before the code:** `hp3 >= 0.25` and `hp6 >= 0.30`: **missed, narrowly** (0.236 and 0.266); red minus green 86 to 109 and luminance 67 to 94: **missed** (80.0 and 58.9, against the old
crater's 85.7 and 66.5: the bowl's bright orange fat ring of the 0.8.7 crater lifted that mean, and the pale lip and the crevices now take it down); 5th percentile at most 27: met (26.1); 95th at least 125: **missed**
(105.5); the pale ring a third narrower: **met** (46 % narrower); every digest that does not fold the depth buffer: **met** (bench, entrails, every recorder's `digest=`, the wet and dermis digests), and
`capture_flesh`'s moved, as above; `shade` at most twice the cost: **met** (+19 to +29 %); no lumps on intact skin, bone smooth, the lip paler: **met** (a test each, each mutation-checked).

## Critique

Against **W1** (JessFX, a glossy dark pit with a point glint, a thick pink-red rolled lip): the entry wound of the preset's close-up now has a floor that is not a bowl and glints in many places (the
0.8.7 floor had one soft highlight), and a pale lip along its upper edge; it has no thick, rolled lip and no single dark pit. Against **W2** (a ragged rolled skin edge round a floor of cream and red nodules): the
crater's margin is ragged and its lip pale and defined, the floor is knobbly; **the nodules are all red**, because under the blood film the cream tint hardly shows (the lumps took the crater's red minus green
from 85.7 to 76.9 and the last round, less pale tint and lighter crevices, got 3 of it back). Against **W3** (torn flaps): none, the margin is a ragged oval. Against **W4** (yellow fat nodules): the fat does not
show in patches at the preset's depths. Against **W5** (a glossy rolled ring round a dark pit with a hard glint): the lip is there, the pit is not. Against **D2**: no ribs.
The floor of the big crater reads as **raw mince** at a glance: even, matte, regular in its nodules; the same algorithm on the preset's own scene (its sun, its canvas, its blood) reads as wet torn meat. The lumps
have no big dark pit and no cream cluster, and the 95th percentile is 105 against 134 to 165: **what is missing is the highlights**, which are the shader's and the light's.

## Verdict

**The margin (profile, ragged rim, lip): GREAT** against W2 and W3 at a glance, a defined pale edge round a ragged oval where there was a pink target. **The floor's lumps: GREAT at the preset's close-up, NOT YET on
`capture_gore`'s** (the targets above are the evidence: `hp3` 0.236 against 0.25, colour 6 under, highlights 20 % under) and **NOT YET for the variety of colour**, which the film hides. Named in the changelog, the
guide and here; the changes that would move them are a dark pit, cream nodule clusters that show through a thinner film, a thicker rolled lip, and the glints of `flesh.wesl`.

## Pending index

- No home-still document on wound-edge morphology or on the appearance of a torn wound floor was found (three `distill_search` queries: gunshot entrance wound margin, cut skin lip, tissue layers): the
  margin rests on two web sources (PMC, Medscape) and the floor on the references.
- Vlachos 2010 is a web PDF, not in the corpus.
