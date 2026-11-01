# An exit wound is torn, not just wide

Iteration 10 (2026-11-01). Item: the crater a shot leaves where it comes out of a body. Ranked first, after the rivulet item turned out
to be bounded by the canvas (`docs/research/rivulets.md`, "Outcome"), because of a **promise the crate made and did not keep**:
`EXIT_SCALE`'s own doc, and `GoreHit::through`'s, say an exit is "larger and more ragged than the hole it went in by", and the crater an
exit was given was the entrance's, a round smoothstep disc, 1.6 times wider. Runner-ups: the dried colour's hue and rim (a shader term, a
38-minute cold wasm build to check); the wall spray's size spread (resolution-bound); `digest=` for a body that breaks.

## What was measured before any change

`capture_skinned --through --eye 0.0,1.3,-0.75 --at 0.0,1.25,0.0` (the exit side of the chest shot, frames 125 to 330): the exit is a round
crater with a red graze ring, three rounded lumps of blood on its rim and, later, three runs. It differs from the entrance only in being
1.6 times wider. The entrance and the exit of one shot are two circles.

## Sources

- **Forensic pathology** (the physics of what is being drawn).
  - Medscape, *Forensic Pathology of Firearm Wounds* (`https://emedicine.medscape.com/article/1975428-overview`): "Exit wounds ... may have a
    stellate configuration or, particularly in the event of a low-velocity exit wound, have a slit-like configuration"; "Entrance wounds
    typically present as circular defects with marginal abrasions, while exit wounds are often more irregular or slit-like."
  - *Gunshot Wound Patterns: A Narrative Review From a Forensic Medical Perspective* (PMC, `https://pmc.ncbi.nlm.nih.gov/articles/PMC12791185/`):
    "Usually, exit wounds are bigger, more asymmetrical, and everted, especially when bones are broken"; "Entry wounds are typically
    smaller, with well-defined margins and tissue invagination."
  - The crate's own source, Comiskey, Yarin & Attinger 2018 (`doi:10.1103/physrevfluids.3.063901`, in the corpus), and Radford 2010 (Otago
    thesis, in `paper_search`): "Forward spatter is ejected from the exit wound and travels in the same direction as the bullet", which is why
    an exit is where this crate throws its forward spatter, and why the exit is worth drawing.
  - The shored exit (an exit supported by something behind it shows a collar like an entrance's) is **not drawn**; named in `## Deviations`.
- **Horror-film and practical-effects references**: `## Reference set`.
- **Technique.** Vlachos, *"Rendering Wounds in Left 4 Dead 2"*, GDC 2010 (`https://alex.vlachos.com/graphics/Vlachos-GDC10-Left4Dead2Wounds.pdf`):
  "Use projected texture for rough edges and blood layer". A shipping game gives a wound its torn edge as the alpha of a texture projected on the
  body (the authored art of a shape); this crate has no texture to project and a CPU depth buffer the crater is written into, so the torn edge
  is written into it, from a hash of the place, and the flesh material reads it through the layers as it reads every crater.

## Craft principle

Where a projectile leaves, the skin is **burst outward from inside**, not cut. The margin is therefore irregular and the tear runs along the
lines of tension of the skin: radial splits from a small hub (stellate) or, at low energy, a single slit; the skin is everted and there is no
collar. Where it goes in the skin is pushed in and abraded: a small round defect with a ring. The picture the crate has to give a player at a
glance is the **pair**: a small clean hole, and, on the far side, a star.

Frances Glessner Lee's chest plates (about 1940, ceramic, for the Nutshell Studies; public domain on Wikimedia Commons) model exactly that with a
revolver's lead bullet: the entrance is a small round hole with a few specks round it, the exit a small hub with six or seven thin
splits of different lengths, bent and rippled, the longest about nine hub radii, a few short.

## How shipping games do it

A shipping game authors the exit's shape as a texture (Vlachos 2010: "rough edges" from a projected texture, over a pose-space ellipsoid that
culls the skin) and so every exit of that texture looks the same, the way Left 4 Dead 1's "5 variations only" did. Here the shape is a pure
function of a place on the canvas, so two exits are never one star. Doom Eternal, the crate's other cited game (`floor-spatter.md`), draws no
tearing on the body: its gore is a mesh swapped in.

## Model adopted

`FlayCanvas::paint_uv_torn` and `paint_world_torn` (`tear` in `[0, 1]`; `paint_uv` and `paint_world` are the same call at `0.0`, byte for byte,
so every entrance and every recorder that has no exit is what it was):

1. **The hub** is the crater `paint_uv` makes, its rim moved in and out by two low harmonics of the angle (`0.10` and `0.08` of the radius at
   most, phases from the place's hash), so it is not a circle.
2. **The splits.** `5 + 3h` (five to seven), each at its own angle spread round the circle with a jitter, from half a radius out to a tip of
   `1.05 + TEAR_REACH (1.3) x tear x (0.15 + 0.85 g^2)` radii, `g` a hash, so a few are short and most long; `TEAR_SPLIT_WIDTH` (2.0) texels wide
   at the root, tapering to a point; bent off the straight by up to a quarter of their length and rippled by a texel. Laid over the crater by `max`
   at `TEAR_SPLIT_DEPTH` (0.5) of its depth, so a split opens the skin and shows what is under it and does not bore into the muscle.
3. **The seed is a place**: `hash_f32` of the centre texel, so the same hit in the same place tears the same way on every run and a hit a
   texel over tears differently. The angles go through `libm`; the weights are bytes.
4. The preset's exit wound calls `paint_world_torn` with `EXIT_TEAR` (1.0). The entrance, a way back in after the first, and every other hit are
   `paint_world` as before.

## Parameters

`TEAR_REACH = 1.3`, `TEAR_SPLIT_DEPTH = 0.5`, `TEAR_SPLIT_WIDTH = 1.6`, the harmonics' amplitudes, the bend and the ripple, `EXIT_TEAR = 1.0`: **design
defaults**, tuned by eye against the plate and the moulage, each named in its doc comment. The count (five to seven) and the long-and-short
spread of lengths are read off the exit plate.

## Validation targets

1. An entrance is round: its reach in each of 36 directions within 0.3 radii of the rest. An exit is a star: the farthest split at least two of the
   entrance's radii, the margin never under 0.7 of the rim between splits, and four to seven splits past 1.4 radii. (Tests.)
2. A split is no deeper than `TEAR_SPLIT_DEPTH` of the crater at the hub, and the hub is as deep as a plain crater's. (Test.)
3. `tear` of zero or NaN is `paint_uv` byte for byte; the same hit twice tears the same; a hit a texel over tears differently. (Tests.)
4. **Every recorder digest** the item did not mean to move holds: the default, `--break`, `--shirt`, `--blows`, `--stroll`, `--quiet` and
   `capture_preset` (no exit), and in `--through` and `--guard` only `flay=` moves; the wet `digest=` holds, because no wet byte changes.
5. A hit costs no more than before (the mask is built once per exit wound).
6. A frame beside Ref 1 (the exit plate) and Ref 2 (the entrance plate), and Ref 3 (a moulage exit): the pair reads at a glance as a small round
   hole and a torn star.

## Deviations

`EXIT_SCALE` (1.6, "this crate's own") is unchanged; what changes is that the exit's margin is no longer a copy of the entrance's. **The shored exit**
(supported by clothing, a wall or bone behind it, which gives an abrasion collar like an entrance's, Medscape above) is not drawn: the crate does
not know what is behind a body. A caller who knows can send the exit as a plain `paint_world` hit.

## Pending index

DiMaio & DiMaio, *Gunshot Wounds* (cited by `EXIT_SCALE`; a book, not in the corpus); Medscape's page and the PMC review are web sources read
through `web_search` (the PMC review is open access and could be indexed); Glessner Lee's plates are an image reference, not text.

## What great looks like

Beside the two plates: on the near side a round hole in the skin with a red collar; on the far side a small dark hub, its margin out of round,
with five to seven thin splits of different lengths running from it, some short, most long, each bent and rippled, tapering to a point, raw red
against the skin; and the blood of the wound running from its lower lip over them.

## Reference set

Kept under `/tmp/c10/refs/`, never committed.

1. `https://commons.wikimedia.org/wiki/File:Experimental_gunshot_exit_wound.jpg` (public domain; derivative of Frances Glessner Lee's chest plates,
   about 1940): the exit of a revolver's lead bullet on a ceramic plate: a small hub and seven thin splits, bent and rippled, from two to nine hub
   radii long.
2. `https://commons.wikimedia.org/wiki/File:Experimental_gunshot_entrance_wound_-_dist_to_target_25_inch.jpg` (public domain): the entrance of the same
   shot at 25 inches: one small round hole and a scatter of specks.
3. `https://commons.wikimedia.org/wiki/File:The_art_behind_Army_medical_training_140722-A-MD393-099.jpg` (US Army, Pvt. Travis Terreo; public
   domain): a moulage wound on a mannequin, ragged torn margin, chunky gel clots, a dark char round it; the wound's edge is not a circle.
4. `https://commons.wikimedia.org/wiki/File:Blood,_guts_and_gore_galore,_Moulage_crew_brings_horror_to_life_140723-A-IX584-057.jpg` (US Army, Spc.
   Anthony Zane; public domain): prosthetic wounds with a lipped, torn slit, each different.
5. Tom Savini, as retold at `https://frombeautytoblood.substack.com/p/tom-savini-made-gore-mean-something`: "A wound has to read true, not just wet."
