# Smears: a hand, a body, a sleeve drawn through blood

Iteration 15 (2026-11-01). Item 14 of the owner's 80s-horror list: "Smears. Drag a hand across wet canvases. The footprint lift and print kernels exist." Why this and not the next wound item: the backlog's
first three items need `flesh.wesl` (a WebGL2 re-check on a disk with 4.6 GB free), a canvas that knows which way is up, or a thinner film in the crevices; a smear needs none of them, is CPU-only, moves no
existing digest, and is the most common sight in a horror frame after a wound: the bloody hand slid down a wall, the body dragged out of the pool, the sleeve wiped through a splash. The crate has footprints
(`GoreStep`: a sole lifts blood from a floor and prints it, fainter at every step) and a kernel for a smear (`bloodstain::patterns::transfer`: "a dragged body runs out of blood", in millilitres) that **nothing
calls**. Runner-ups, in the log: a wound floor finer than a texel (shader, blocked by the disk), an opened chest with ribs (the canvas needs an up), a burst hit (a big composition), cream nodules under a thinner
film, slime strings (parked).

## What was measured before any change

Nothing in the crate paints a smear. A sole walking through a pool prints soles; a wound's bleed runs downhill; a hand, a sleeve or a body dragged through a pool or down a wall leaves **nothing but the
blood that was already there**, and blood a body carries out of a pool is lost. `grep transfer(` finds the kernel and its tests and no caller. The 0.8.8 baseline (`bench_carnage`, median of three):
`carnage_frame_ms` 0.09307, `carnage_p99_ms` 2.684, digest `0x4366a8b64352c85a`.

## Sources

- **Forensic science** (what a smear is), SWGSTAIN, *Recommended Terminology*, Forensic Science Communications 11(2), April 2009 (`https://theiai.org/docs/SWGSTAIN_Terminology.pdf`; the FBI's reprint
  `https://www2.fbi.gov/hq/lab/fsc/backissu/april2009/standards/2009_04_standards01.htm`), read in full for this note: "**Swipe Pattern** A bloodstain pattern resulting from the transfer of blood from a
  blood-bearing surface onto another surface, with characteristics that indicate relative motion between the two surfaces." "**Wipe Pattern** An altered bloodstain pattern resulting from an object moving through a
  preexisting wet bloodstain." "**Transfer Stain** A bloodstain resulting from contact between a blood-bearing surface and another surface." "**Directionality** The characteristic of a bloodstain that indicates
  the direction blood was moving at the time of deposition." "**Void** An absence of blood in an otherwise continuous bloodstain or bloodstain pattern." Two mechanisms and one picture: a **swipe** moves blood
  *from the object to the surface* (a bloody hand on a wall), a **wipe** moves blood *that is already on the surface* (a sleeve through a pool), and both are read by their direction and their voids.
  `distill_search` found nothing on swipes or wipes in the corpus (three queries: swipe and wipe, transfer stain, drag direction): it holds cast-off (`10.1111_1556-4029.13855`) and forward spatter
  (`10.1103_PhysRevFluids.3.063901`), not contact patterns. **A gap, not a model**: the physics of the streaks (finger lines, voids) is read off the references, not derived.
- **The crate's own kernel**: `bloodstain::patterns::transfer(contact, tangent, load_ml, settings)`: "`BloodSettings::transfer_rate` millilitres leave the load per contact, so a dragged body runs out of blood
  — the smear fades along the drag and then stops, which is the whole reason the budget is conserved rather than assumed infinite." The same conserved budget, here in canvas bytes.
- **Horror-film and practical-effects references**: `## Reference set`.
- **Technique, how shipping games do it.** Practitioners' answers, the nearest thing to a published method (no GDC talk on drag trails was found): the Unreal forum's "Basically, you have to spawn decals
  on the ground that use the same direction your character is going to" (`https://forums.unrealengine.com/t/how-would-you-do-a-continuous-blood-trail-appearing-progressively-in-blueprints/238611`) and
  polycount's "If I were doing this, I'd model out a simple mesh that curves and bends as you see fit. The UVs would stay straight, however"
  (`https://polycount.com/discussion/107896/creating-a-blood-trail-in-udk`): a trail is **a decal or a strip laid along the path**, the same image at every point, with no memory of how much blood there was.
  *Hitman: Blood Money* (2006) let a player drag a bloody body and leave a trail the guards found (`https://screenrant.com/hitman-3-clean-bloodstains-how-why-no-method/`: "dragging a bloody body would create a
  blood trail that guards would also discover"): the only shipping use found where the trail mattered.

## Craft principle

A smear is **a conserved load laid along a path**. The blood a hand carries is finite (SWGSTAIN's transfer stain: from one surface to another), so the mark is **heavy where the contact first touched, fades along
the path and ends** (the crate's `transfer` kernel says so, and the references show it: a bead at the head of each finger line and a feathered tail); it is **in lines parallel to the motion**, the fingers, the
fibres of a cloth, the bristles of whatever dragged, and as the load runs low the lines break up into **voids** (dry brush); and a contact that moves through blood already there **drags it**: a wipe leaves
grooves in the pool and carries what it took on down the path. A decal of one smear image, repeated, does none of these: it is the same at the end as at the start and creates blood from nothing.

## How shipping games do it

The forum answers above are the usual method: **a decal (or a strip of mesh with the UVs straight) per stretch of path, oriented along the motion**. It is cheap and reads as a trail at a distance. It has
no budget (the trail is as heavy at ten metres as at one), no lift (a body dragged through a pool takes nothing out of it and leaves nothing in it), no streaks parallel to the motion (the strip's one
image has its own grain), and no voids. Here the canvas is the blood: a smear **moves bytes** of it, so the same machinery that makes a pool spread and a crust dry makes a smear drip, spread and dry.

## Model adopted

1. **`WetCanvas::smear_uv(&Smear, carry, full, tick, settings) -> carry`** (wetmap, UV space, integer bytes). A contact `width` wide moves from `from` to `to`. The texels under the swept rectangle are visited in
   **one canonical order**: by how far along the path, then across, then index (a sort of integer keys). At each texel (a) **it lifts**: `LIFT` of the wet blood there is taken (dry blood is not: "dry paint does
   not move"), added to the carry; (b) **it deposits**: up to a target coverage set by the carry (`frac = sqrt(carry / full)`), a streak field and a feathered edge, **raising** the texel to the target with
   what the carry holds, never above full coverage, never from nothing. **Every byte lifted is a byte carried and every byte deposited is a byte spent**: `sum(amount) + carry` is the same before and after, to the
   byte (a test over random paths). A texel it raises is fresh blood, age 0.
2. **The streaks** are a value noise stretched along the path: `GRAIN` (12 mm, a finger) across and twenty of them along, so a line runs a hand's length and a path crosses a few. Where the noise is under the load the
   contact inks; as the load falls fewer lines ink and the rest are voids, as the sole's tread does in `print_uv` (`ink = smoothstep((pressed - streak) * 6 + 0.5)`), so the tail is feathered.
3. **The bead**: the first `3 GRAIN` of a stroke, `Smear::bead`, inks regardless of the streaks, to a share of the load: the head of each finger line of the references.
4. **The preset**: a `GoreSmear` message (`carrier`, `from`, `to`, `normal`, `width_m`, `load_m2`) and a `Smeared { load_m2 }` the carrier keeps, as `GoreStep` and `Tread` do. The surface under each end is cast
   for along `-normal` (a floor's `+Y`, a wall's), the contact's path on its canvas is `smear_uv`'s, and the carrier's load is carried in **square metres of full coverage** (a film of about a tenth of a
   millimetre: a millilitre is `0.01 m²`), converted to bytes by the canvas's texel area, so a hand moving from a wall to a floor carries the same blood. Per-frame segments, like steps.

## Parameters

All **tuned by eye and against the reference measures** unless a source names them: `LIFT` 0.35, the peak target coverage 0.85, `full` as `SMEAR_FULL_M2` 0.004 m² (a hand's film), `GRAIN` 12 mm, the streak's length
20 grains, the bead 3 grains, the feather over the outer 15 % of the width, `GoreSmear`'s default width 0.09 m (a hand), the load cap 0.05 m² (a litre-ish body). Each says so in its doc comment.

## Validation targets

- **Conservation, to the byte** (`tests/wetmap.rs`): over random strokes on a canvas holding wet pools, dry crusts and nothing, `sum(amount) + carry` before equals after; dry crust texels are neither lifted
  from nor re-aged unless raised; a clean contact (carry 0) on a clean canvas changes nothing and returns 0.
- **Determinism**: the same inputs the same canvas digest, twice.
- **Directionality**: the heavier end of the mark is where the stroke began (more bytes in the first third than the last), and a stroke through a pool leaves grooves (texels under the pool's amount) and a
  heavier head beyond them.
- **The render, measured as the references are** (`smear_stat`: reddish mask; axis by PCA; gradient orientation coherence; coverage of the oriented band by thirds): coherence at least **0.45** (the references:
  0.56 to 0.61), the gradients across the axis (streak alignment at least **0.95**; theirs 0.99 to 1.0), and the last third's coverage at most **0.6** of the first's (theirs 0.04 to 0.58), the length at least 1.5
  times the width.
- **Every existing digest equals 0.8.8's** (`bench_carnage` `0x4366a8b64352c85a`, every `capture_skinned` variant, `capture_preset`, `capture_entrails`, `capture_flesh`), twice each; a new recorder variant
  has its own digest, equal in two runs. `bench_carnage` within 5 %.
- **Cost**: a hand's smear (80 mm by 250 mm on a 256 canvas, about 1300 texels) in the tens of microseconds; per-frame segments no more than a step's.

## What great looks like

Beside S1 and S2: **a heavy head and a feathered tail in parallel lines**, the lines broken into voids toward the end, dragged blood in grooves through a pool, the edge irregular; a drag trail that starts as a
smeared pool and thins out over a body's length and stops. Seen from the distance of a player, it reads as *a hand dragged down the wall* and *something was pulled out of that pool*.

## Reference set

Stock and educational photographs of staged and forensic smears (prop blood and swipe demonstrations); copyrighted by their publishers, kept under `/tmp/c15/refs/`, never committed; this note cites their pages.
**No film still of a smear was gathered** (Bing's image results for the films were posters and trailers): the gap is named under `## Pending index`. Statistics are `smear_stat`'s.

| Ref | Source | Page | What it shows that the crate must reproduce |
|---|---|---|---|
| S1 | Forensic Pocket Guide, *Transfer stains: swipe pattern* | `https://forensicpocketguide.com/processing-bloodstains-transfer-stains/` | four finger-lines of blood, each with a thick rounded head and a tail that thins to a feathered dry-brush streak, parallel and unequal in length; coherence 0.612, streaks along the axis, coverage by thirds 0.236, 0.162, 0.136 |
| S2 | Dreamstime, *Smear of blood leaving a trail on a floor* | `https://www.dreamstime.com/smear-blood-leaving-trai-floor-disturbing-trailing-sh` | a pool at the head and a tail of about eight fine parallel streaks that run out into voids, on a tiled floor beside a print; L/W 5.1, coherence 0.56, coverage 0.604, 0.338, 0.023 |
| S3 | SlideServe, *Bloodstain pattern analysis, swipe patterns* | `https://www.slideserve.com/eamon/lecture-26-introduction-to-bloodstain-pattern-analysis` | the same swipe with the SWGSTAIN vocabulary; coherence 0.573 |
| H1 | Dreamstime, *Bloody handprint on a wall* | `https://www.dreamstime.com/bloody-handprint-wall-large-fresh-bloody-handprint-ol` | a fresh palm and four fingers laid on a wall, a heavy edge and streaks dragged down from each finger, runs under the heel |
| H2 | Dreamstime, *Fake closeup smeared bloody handprints on a shabby white wall* | `https://www.dreamstime.com/closeup-smeared-bloody-handprints` | prints slid down a wall: a stack of finger streaks per print, fading, one over another |
| H3 | Adobe Stock, *Fake bloodied handprints streaked across a grungy wall* | `https://stock.adobe.com/images/fake-bloodied-handprints-streaked-across-a-grungy` | prints with drips and smears, wet and dried together |

## Deviations

- **A different model from a decal strip**, on purpose, with a citation: the budget is the crate's own `transfer` kernel and SWGSTAIN's definition of a transfer stain.
- `bloodstain::patterns::transfer` (in millilitres) is **not called**: the canvas's own bytes are the budget, because the canvas is where a wipe's lifted blood comes from. The two agree in kind and not in
  unit; a mapping from `transfer_rate` to the cap is a design decision the crate does not take.
- **Rigid surfaces only** (a `Gore::floor`, a `Gore::wall`): a skinned body is posed on the GPU and the preset has no cheap way to walk its UVs along a path. A hand dragged down a character is a later item.
- **A hand's print** (palm, five fingers) is not drawn: a smear is the *drag*; a handprint is a `Print` with a different outline, an item of its own.

## Pending index

- No home-still document on swipes, wipes or contact transfer (three `distill_search` queries); SWGSTAIN is a web PDF, not in the corpus.
- No film still of a smear was gathered; the references are staged and forensic photographs.
- The technique sources are forum answers, not a talk or a paper.
