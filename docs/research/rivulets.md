# Blood runs in rivulets, not as a curtain

Iteration 10 (2026-11-01). Item: what a bleeding wound leaves running down a body. Ranked first from the loop's own backlog
(`docs/loop-log.md`, iteration 9: "the horror-film items") by looking at a state no earlier pass had measured against a reference:
the chest wound of `capture_skinned` at frame 330, close (`--eye 0.0,1.3,0.75 --at 0.0,1.25,0.0`). Runner-ups: the dried colour's
saturation and rim (a shader term, a 38-minute cold wasm build for the WebGL2 check); the wall spray's size spread (resolution-bound,
and the fix is a shader term too); `digest=` for a body that breaks.

## What was measured before any change

The three runs of a heavy wound on the mannequin's back read as **one painted curtain**: at frame 330 they are 100 px across
(about 12 cm; the torso is 285 px, 27 cm) and the gaps between them have closed. The canvas is the torso's own, 256 texels over a sheet
about 0.9 m on a side, so a texel is 3.5 mm and each run is about 8 texels wide, 28 long at the stamp, and 3 texels apart. Against the
references below every run is an order of magnitude too wide: a rivulet of blood on a vertical surface is 2 to 5 mm across.

Cause, in `preset::bleed` (`src/preset.rs`, the pulse of a wound): each of the `BLEED_WELLS` (3) beads is a stamp `bead * 2.6` long and
`bead * 0.6` wide, `bead` the wound's radius over the square root of three. The width follows the **wound's radius**, and no physical
property of a rivulet is in it. A run is the track a stamp lays plus the parcels the drip pass sheds down it, so a stamp 8 texels wide
runs as a band 8 texels wide, and three bands a bead apart touch.

## Sources

- **Physics.**
  - Schwartz, *"Viscous flows down an inclined plane: Instability and finger formation"*, Phys. Fluids A 1 (1989), `doi:10.1063/1.857466`:
    "Fingers, or rivulets, form that are similar to those observed experimentally. The instability that leads to fingering is lateral flow
    driven by surface tension with larger curvature at the finger troughs. A scaling law for the prediction of finger width is derived."
  - Le Grand, Daerr & Limat, *"Shape and motion of drops sliding down an inclined plane"*, J. Fluid Mech. 541 (2005),
    `doi:10.1017/s0022112005006105`: "the drops change their aspect ratio: they become longer and thinner, but maintain a constant,
    millimetre-scale height" and, faster still, "a tail breaks into smaller drops (pearling transition)".
  - Kim, Kim & Kang, *"Meandering instability of a rivulet"*, J. Fluid Mech. 498 (2004), `doi:10.1017/s0022112003006748`: "A rivulet is a
    narrow stream of liquid flowing down a solid surface. When its flow rate exceeds a certain limit, it tends to meander".
  - Blood's viscosity is the crate's own (`bloodstain::rheo`, Cho & Kensey 1991: 3.45 mPa s at high shear, 56 mPa s at none; density 1 060 kg/m^3).
  - Full text of the first two is not open access (`paper_download`: no copy); their abstracts are quoted from `paper_search`.
- **Horror-film and practical-effects references**: `## Reference set`.
- **Technique.** Vlachos, *"Rendering Wounds in Left 4 Dead 2"*, GDC 2010 (`https://alex.vlachos.com/graphics/Vlachos-GDC10-Left4Dead2Wounds.pdf`):
  "Use projected texture for rough edges and blood layer", and "We multiply the blood layer by a gradient to prevent the blood from spraying too
  far", over a list of what the first prototype lacked: "Lacked blood on the clothes and skin near the border of the wound". A shipping game
  authors the wound's blood as a static texture; this crate simulates it on a canvas the CPU owns, so the question here is what the simulation is
  told, not how a texture is projected.

## Craft principle

A rivulet's width is a property of the fluid, the surface and the flow, **not of the wound it comes from**, and it grows only as the quarter
power of the flow. That is the lubrication limit of a thin viscous rivulet on a vertical plane: a cross-section `h(x) = h0 (1 - (2x/w)^2)`
of width `w` and contact angle `theta = 4 h0 / w`, a local flux `g h^3 / (3 nu)`, so

```
Q = (g / 3 nu) * integral of h^3 dx = (16/105) (g / nu) h0^3 w = (1/420) (g theta^3 / nu) w^4        w = (420 nu Q / (g theta^3))^(1/4)
```

(the coefficient is derived here, from the section above, not copied from a paper; Schwartz 1989 derives a scaling law of the same form for the
width of a finger). With `nu` between blood's high-shear 3.3e-6 m^2/s and its zero-shear 5.3e-5, `theta` of 1 radian and a flow `Q` of 0.1 to 1 ml/s,
**`w` is 1.9 to 6.9 mm**; at a contact angle of 30 degrees it is up to 11 mm. The rivulet is thin, and a flow ten times larger makes it only
1.8 times wider. That is the property the crate's runs lack: they are as wide as the wound.

## How shipping games do it

Shipping games author the run as art and project it (Vlachos 2010 above: "Use projected texture for rough edges and blood layer", with a
gradient on it so "the blood" does not spray too far), which is why their runs are always the width an artist drew. Left 4 Dead 2 cannot show a
run that has not been authored and cannot make it longer for a longer bleed; this crate's runs are a simulation of the flow, and so the
widths they are told must be physical. Doom Eternal, the crate's other cited game (`docs/research/floor-spatter.md`), caps its decals and
keeps them: nothing of the run's shape is simulated.

## Model adopted

One change, in how a pulse is painted (`preset::bleed`), and none in the canvas passes:

1. **Width.** `rivulet_width(bead)`: `RIVULET_WIDTH_M` (4 mm, at the flow of a 2 cm wound) times the square root of the bead over its reference,
   because the flow a bead carries goes as its area and the width as the quarter power of the flow. Clamped to 0.5 to 3 times the reference.
   **Never under `RIVULET_MIN_TEXELS` (3) across at the canvas's own resolution**: a stamp of two texels peaks at 73 of 255 on the canvas
   (measured, iteration 6) and the drip pass lets a texel go only above 89, so a thinner stamp would not run at all. The canvas, not the
   physics, sets the thinnest run it can draw, and a finer canvas draws a finer one.
2. **Variety.** Width 0.75 to 1.25, length 0.7 to 1.3 and the lean below, by `hash_f32(seed, k)`: the seed is the wound's, the place a well
   is. The wells stay put for the wound's life (the run is only long when it is fed over the track the last pulse left wet), so does the variety.
3. **Where the blood leaves the rim and where it goes.** A well on a hole's rim leaves it along the outward direction and bends to downhill: its
   stamp is laid along downhill plus a share (`RIVULET_LEAN`, 0.3) of the outward direction across it, so a fan of rivulets opens from the lip
   and each turns down, not three clones side by side.

What is **not** here and is named: **meandering** (Kim et al. 2004). The drip pass moves every line of the canvas as a translation (`DripStep::across`
is a function of the line alone), which is the whole conservation argument (at most one parcel arrives at a texel). A rivulet that wanders
sideways by itself needs a per-texel bijection of a row (or a rule for two parcels reaching one texel without a byte lost), and is its own
item with its own proof.

## Parameters

`RIVULET_WIDTH_M = 0.004` and `RIVULET_BEAD_REF_M = 0.012`: **tuned**, from the table above (4 mm is inside the 1.9 to 6.9 mm the derivation gives
and the 1.5 to 5 mm the references show). `RIVULET_MIN_TEXELS = 3.0`: a **measured resolution floor** (iteration 6's peak of a stamp of two texels).
`RIVULET_LEAN = 0.3`, the variety ranges: **design defaults**, chosen by eye against the references.

## Validation targets

1. Three rivulets below a heavy wound, **separated**: the gap between neighbours at least their own width at 3 s after the first pulse, at the shipped canvas.
2. Each rivulet no wider than 7 texels (a quarter of the old 28) drawn at 3 s on a 256 canvas, and narrower on a finer one.
3. The rivulets differ: longest over shortest at least 1.3.
4. The hit costs no more than 0.8.x did (`hit_tick_cost`): stamps are smaller.
5. Every golden untouched (`the_flesh_tables_are_frozen`, the wetmap conservation tests, `tests/wetmap.rs`); the recorders' digests move **on purpose**
   where a wound bleeds and nowhere else (`--quiet`'s `f83a631a7426808d` holds).
6. A frame beside Ref 1 (Public domain moulage) and Ref 2 (a glass pane): the runs read as separate threads, not a drape.

## Deviations

`bead * 0.6` (the stamp's width) and the equal lengths of the three wells were design defaults of 0.5.35 (`AG-108`); the run's width now follows
`rivulet_width`, a derived and tuned number, and the length varies. Marked here as the change.

## Pending index

Schwartz 1989, Le Grand et al. 2005 and Kim et al. 2004 (publisher PDFs, not open access); Towell & Rothfeld 1966, *Hydrodynamics of rivulet flow*,
and Duffy & Moffatt 1995, *Flow of a viscous trickle on a slowly varying incline*, the sources a full treatment of the width would read; Attinger et al.,
*Fluid dynamics topics in bloodstain pattern analysis*, `doi:10.1016/j.forsciint.2013.04.018` (no open copy). None is in the corpus.

## What great looks like

Beside Ref 1 and Ref 2: a wound whose blood leaves its lower lip as a few distinct glossy threads of different lengths, each a few millimetres
across at a canvas fine enough, each ending in a darker bead, fanning out from the lip and turning downhill, the skin clean between them; not a drape.
It is the look of a pane of glass with fake blood run down it (Ref 2), at the wound of a moulage (Ref 1).

## Reference set

Kept under `/tmp/c10/refs/`, never committed.

1. `https://commons.wikimedia.org/wiki/File:The_art_behind_Army_medical_training_140722-A-MD393-099.jpg` (US Army, Pvt. Travis Terreo; public domain):
   a moulage wound on a mannequin, chunky gel clots with glossy highlights, thin runs from the lip, a dark char round it. The wound's runs are narrow
   threads against a wide mat at the lip.
2. `https://commons.wikimedia.org/wiki/File:Blutaktion_von_Extinction_Rebellion_an_der_CDU-Zentrale,_Berlin,_17.08.2021_(51549841587).jpg` (CC BY 2.0):
   fake blood on a glass pane, dozens of near-parallel runs 1 to 2 px wide at 1.3 mm a px (**1.5 to 3 mm**), spaced 40 to 60 px (**5 to 8 cm**) apart,
   of very different lengths, each ending in a bead; a few wide at the head and thin below.
3. `https://commons.wikimedia.org/wiki/File:Blood,_guts_and_gore_galore,_Moulage_crew_brings_horror_to_life_140723-A-IX584-057.jpg` (US Army, Spc. Anthony Zane; public domain):
   prosthetic wounds with a lipped slit and a dark clot in the middle: a wound has an edge, and the blood is held in it.
4. `https://commons.wikimedia.org/wiki/File:Healthcare_Simulation_Moulage_Deep_Gash_Wound_Cut_on_Wrist.jpg` (CC BY-SA 4.0): a cut wrist, a wet dark lip, a stipple of
   blood round it, no drape.
5. `https://commons.wikimedia.org/wiki/File:Bloodywoman.jpg` (Mikamatto, CC BY 2.0): a thin streak across a forehead 3 to 4 mm wide and a glossy drop on the lip.
6. Tom Savini, as retold at `https://frombeautytoblood.substack.com/p/tom-savini-made-gore-mean-something`: "A wound has to read true, not just wet."

## Outcome (iteration 10): not shipped, and why

The change was built (`rivulet_width`, per-well variety, an outward lean; `preset::bleed`), captured on the real renderer and **reverted**, because the
canvas bounds what a run can be and the item could not reach GREAT inside it. Measured with the real `WetCanvas` (a throwaway integration test that
painted the wells' stamps and ticked the canvas, read through the shader's cubic B-spline at its 4 % edge; removed):

- **A thin stamp widens.** The spread pass lets a byte go only from a difference of 25 (`round(0.02 x d)`), so a film stops creeping when its slope
  is under 25 of 255 a texel. A stamp of three texels at full coverage (the least that runs: a stamp of two peaks at 73, under the drip threshold of 89)
  is six or seven texels wide after 400 ticks. At the mannequin's 3.5 mm a texel that is 2 cm, not 4 mm.
- **The wells are too close for threads.** The wells sit on the wound's rim, `radius x density` from its centre, fanned 50 degrees either side of
  downhill: for a 3.5 cm hole on that canvas, 5 texels apart. Threads of six texels at five apart are one sheet, which is what the three runs were. Fanned
  to 69 degrees (9 texels apart) they still merge as they widen, and the result was a wide skirt with one thin finger, worse than a curtain.
- **A rivulet that wanders needs a different drip pass.** `DripStep::across` is a function of the line alone, so every texel of a line steps alike and
  at most one parcel arrives at a texel: that is the conservation argument of the pass. A per-texel sideways step is a bijection of a row, or a rule for
  two parcels reaching one texel without a byte lost or a texel over 255. Named, not built.

What would move it: a canvas twice as fine along the run (the physical width needs it: 4 mm is a texel and a third at 3.5 mm), a drip pass that conserves
under a per-texel sideways step, and wells spaced by the wound's circumference and not its area. The derivation above stands and is the model for it.
