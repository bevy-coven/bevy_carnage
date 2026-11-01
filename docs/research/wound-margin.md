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
2.2 mm of skin, **16 mm of fat**, 7.3 mm of muscle, 2 mm of cortex and 4 mm of marrow, so a smooth bowl 30 mm deep shows a **pale, orange ring a third of the radius wide** (skin 7 % and fat 51 % of the span, spread
over the bowl's flat rim by `smoothstep`), then a red floor with no features. The pink halo of item 7 is that ring.

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
3. **The lip**: the outermost peeled texels are the skin band, and the skin band is narrower by 1; the dermis is retuned in `strip.rs` if the thin ring wants to be paler (it ringed every wound in white at 0.5.9 only
   because it was wide). If so `the_strips_are_frozen` is re-blessed in the same commit with the reason.

## Parameters

All **tuned by eye and by `hp3`/`hp6`** against the reference set, none measured: `WALL` 0.45 and `PIT` 0.30 (start; the experiment of this note's first round), `LUMP_CELL_MM` 9 and a second octave at 4.5, the
lump's amplitudes (depth 2.5 mm, colour 0.35 down and 0.12 up, roughness 0.2), `LUMP_FADE_MM` 4. Each says so in its doc comment.

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

- **The depth profile changes** (`WALL`, `PIT`): a deliberate model change and not a "fix", named in the changelog and the log, re-blessing `the_scripted_wound_is_frozen` in the same commit.
- Small wounds stay smooth: a wound under about six texels across has no room for a lump, and the lumps fade with the wound's size. The shader is what would give a 20 mm wound a floor (`flesh.wesl`, a WebGL2
  re-check): **the next item once the disk allows it**.
- **No ribs, no flaps**: an opened chest with a rib layer needs the canvas to know which way is up, which no part of the crate does today.

## Pending index

- No home-still document on wound-edge morphology or on the appearance of a torn wound floor was found (three `distill_search` queries: gunshot entrance wound margin, cut skin lip, tissue layers): the
  margin rests on two web sources (PMC, Medscape) and the floor on the references.
- Vlachos 2010 is a web PDF, not in the corpus.
