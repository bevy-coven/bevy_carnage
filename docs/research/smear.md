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
   not move"), added to the carry; (b) **it deposits**: up to a target coverage set by the carry (`frac = sqrt(carry / full)`, the target `PEAK * frac * ink * feather`, so the deposit falls to nothing as the
   load does), a streak field and a feathered edge, **raising** the texel to the target with what the carry holds, never above full coverage, never from nothing. **Every byte lifted is a byte carried and every
   byte deposited is a byte spent**: `sum(amount) + carry` is the same before and after, to the byte (a test over random paths). A texel it raises is fresh blood, age 0.
2. **The streaks** are a value noise stretched along the path: `GRAIN` (7 mm, a fingertip's ridge) across and twenty of them along, so a line runs 14 cm and a path crosses a few. A line inks where the noise is
   under what the contact presses (`ink = smoothstep((pressed - streak) * 6 + 0.5)`, `pressed = frac² × (1 − 0.4 × edge)`, as the sole's tread does in `print_uv`): **the lines drop out with the load itself, not its
   root**, so the tail breaks into voids while the amounts are still falling, and the edge, where the contact presses less, is ragged.
3. **The bead**: the head of a stroke (`Smear::bead`, set on a stroke's first segment). Each line of the contact **touches down at its own distance from the start**, up to `3 GRAIN`, a hash of its lane, so the
   leading edge is ragged and some lines run longer into the mark than their neighbours: the thick rounded head of each finger line in the references, and not a rectangle.
4. **The preset**: a `GoreSmear` message (`carrier`, `from`, `to`, `normal`, `width_m`, `load_m2`) and a `Smeared { load_m2 }` the carrier keeps, as `GoreStep` and `Tread` do (the stroke's seed, a place, and
   the tick it was last heard from are private: two ticks of silence end a stroke and the next one touches down with a head). The surface under each end is cast for along `-normal` (a floor's `+Y`, a wall's),
   within `2 × SURFACE_STAND_OFF`; a path whose ends are farther apart on the canvas than on the surface (a UV seam) is refused; the contact's path on its canvas is `smear_uv`'s, and the carrier's load is
   carried in **square metres of full coverage** (a film of about a tenth of a millimetre: a millilitre is `0.01 m²`), converted to bytes by the canvas's texel area, so a hand moving from a wall to a floor
   carries the same blood. Per-tick segments, like steps. A tier that draws no blood smears none.

## Parameters

All **tuned by eye and against the reference measures** unless a source names them: `LIFT` 0.35, the peak target coverage 0.85, `SMEAR_FULL_M2` 0.016 m² (a hand's width and thirty centimetres of film at three-fifths
coverage: about a millilitre and a half), `SMEAR_GRAIN_M` 7 mm, the streak's length 20 grains, the bead 3 grains, the feather over the outer 15 % of the width, `SMEAR_WIDTH_M` 0.09 m (a hand), the load cap
`SMEAR_LOAD_MAX_M2` 0.1 m² (six and a half full loads: a body bleeding as it is dragged), `SMEAR_GAP_TICKS` 2. Each says so in its doc comment. (The first draft of this note said 0.004 m² for a full load: a
tenth of what a hand lays, found when the first render's smear ran out after a hand's length.)

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

Educational and stock photographs of smears; copyrighted by their publishers, kept under `/tmp/c15/refs/`, never committed; this note cites their pages. **No film still of a smear was gathered** (Bing's image
results for the films were posters and trailers): the gap is named under `## Pending index`. Statistics are `smear_stat`'s (reddish mask, axis by PCA, gradient orientation coherence, coverage by thirds).
**Three stock handprint images were also downloaded and dropped**: the page of one says it is AI-generated ("generated-ai-fake") and the other two read as the same, so they are evidence of nothing; S2's provenance
(a stock photograph of a trail and a shoe print) is not stated either, and it is used for the structure measures only.

| Ref | Source | Page | What it shows that the crate must reproduce |
|---|---|---|---|
| S1 | Forensic Pocket Guide, *Transfer stains: swipe pattern* | `https://forensicpocketguide.com/processing-bloodstains-transfer-stains/` | four finger-lines of blood, each with a thick rounded head and a tail that thins to a feathered dry-brush streak, parallel and unequal in length; coherence 0.612, streaks along the axis, coverage by thirds 0.236, 0.162, 0.136 |
| S2 | Dreamstime, *Smear of blood leaving a trail on a floor* (stock) | `https://www.dreamstime.com/smear-blood-leaving-trai-floor-disturbing-trailing-shoe-print-crime-scene-image398733220` | a pool at the head and a tail of about eight fine parallel streaks that run out into voids, on a tiled floor beside a print; L/W 5.1, coherence 0.56, coverage 0.604, 0.338, 0.023 |
| S3 | SlideServe, *Lecture 26, introduction to bloodstain pattern analysis, swipe patterns* | `https://www.slideserve.com/eamon/lecture-26-introduction-to-bloodstain-pattern-analysis-bpa-continued` | the same swipe with the SWGSTAIN vocabulary; coherence 0.573 |

## Deviations

- **A different model from a decal strip**, on purpose, with a citation: the budget is the crate's own `transfer` kernel and SWGSTAIN's definition of a transfer stain.
- `bloodstain::patterns::transfer` (in millilitres) is **not called**: the canvas's own bytes are the budget, because the canvas is where a wipe's lifted blood comes from. The two agree in kind and not in
  unit; a mapping from `transfer_rate` to the cap is a design decision the crate does not take.
- **Rigid surfaces only** (a `Gore::floor`, a `Gore::wall`): a skinned body is posed on the GPU and the preset has no cheap way to walk its UVs along a path. A hand dragged down a character is a later item.
- **A hand's print** (palm, five fingers) is not drawn: a smear is the *drag*; a handprint is a `Print` with a different outline, an item of its own.
- **A canvas is treated as isotropic.** `smear_uv` works in UV space, and `Gore::density()` is UV per metre in `u`. A surface whose UVs are stretched (the demo wall is 3 m by 2.4 m on one `0..1` square, 5.9 mm a
  texel across and 4.7 mm down) lays a slanted stroke a few per cent off in width and a texel's area is taken from `u`, so the load a stroke keeps is off by about a fifth there. [INFERENCE] A level's UVs, in metres, usually are not stretched.
- **No sag, no drip of the smear itself.** Blood on a wall runs under the wetmap's own passes only when a texel holds more than its threshold; a smear is film (`SMEAR_PEAK` 0.85 of full coverage at most) and
  does not run. A heavy hand on a wall that should sag a little does not.

## Outcome

Five render rounds (`capture_skinned --smear`, seen from the left and above, clear of the sheet) and a sixth tried and reverted. Measures are `smear_stat`'s, on frame 245, the crop of each mark:

| Mark | coherence (>= 0.45) | streak alignment (>= 0.95) | length / width (>= 1.5) | coverage by thirds | last / first (<= 0.6) |
|---|---|---|---|---|---|
| soaked hand down the wall | 0.57 | 1.00 | 5.0 | 0.982, 0.938, 0.529 | **0.54** |
| less soaked hand, slanted | 0.46 | 1.00 | 6.8 | 0.861, 0.419, 0.488 | **0.57** |
| body dragged out of the pool | 0.51 | 1.00 | 4.1 | 0.570, 0.542, 0.434 | 0.76 |

(S1: 0.612, 1.0, thirds 0.236 / 0.162 / 0.136; S2: 0.56, L/W 5.1, thirds 0.604 / 0.338 / 0.023.)

**What each round showed.** R1: the marks were solid bars with square heads that ended all at once, because a deposit of `0.55 + 0.45 frac` of the peak never fell to nothing; and the camera was behind the sheet.
R2: the deposit falls with the load (`PEAK * frac`): the end is gradual. R3: **a square head and 12 mm lines made a slab**; 7 mm lines and a ragged touch-down per line made a head that reads as fingers.
R4 (tried, reverted): `frac` to the three-halves made the tail thinner, but a contact with a little blood then laid almost nothing and kept most of it (5 778 of 8 000 bytes left), which is a hand that does not
mark and does not run out. R5: the lines drop out with `frac²`, not its root, so the voids come while the amounts are still falling: last / first 0.66 and 0.62 became 0.54 and 0.57.

**Critique, against S1 and S2.** The drag out of the pool is what S2 is: a pool grooved by the body's width, a fan of fine parallel lines leaving it, broken into dashes toward the end. The lightly loaded hand is
what S1 is: a ragged head, lines that part and run out in dots. **Missed:** (1) the soaked hand's first two thirds are a solid slab, not striated lines (coverage 0.98 and 0.94): in the renders the film reads as opaque
from roughly a third of full coverage (seen, not measured) and the lines sit at 0.4 to 0.85: the references show lines throughout a heavy head; (2) the dragged body's trail fades 0.76, not 0.6, because that body is fed every tick
(`0.0004` m² each) and a bleeding body does not run dry in a metre and a half; the number is the demo's choice, and a drained one would fade; (3) the second hand's coherence, 0.46, is 0.01 over the floor
of the target; (4) no film still was compared (named under Pending index).

**Cost.** `smear_uv` on a 512 canvas, median / p99 over 500 to 2 000 calls, load 5: a hand's tick (1.2 cm by 9 cm) 1 / 2 microseconds; a body's (5 cm by 34 cm) 16 / 43; a one-shot 25 cm by 9 cm 41 / 66.
`bench_carnage` (nothing smears in it; the plugin now runs one more system, with an empty reader): `carnage_frame_ms` 0.09507, 0.09540, 0.09796 against 0.09337, 0.09307, 0.09241
before (median +2.5 %, load 5.7 against 5), `carnage_p99_ms` median 2.709 against 2.684 (+0.9 %); digest `0x4366a8b64352c85a` unchanged.

**Reproducibility.** Every existing digest equals 0.8.8's in two runs (preset, skinned with every variant, entrails, flesh); `capture_gore`'s four module digests are equal in two runs and this change touches none of those modules; `--smear` prints `digest=b3f7357ae81ff12c` twice.

**Mutations the tests caught:** `SMEAR_LIFT = 0` (`a_wipe_drags_a_pool_along_its_path`); the carry not decremented on a deposit (three tests: conservation, the fade, the wipe); the tier gate ignored (`a_smear_moves_a_carriers_blood_onto_the_floor_and_a_tier_that_draws_none_smears_nothing`).

**Verdict: GREAT** for the kernel, the preset and the drag trail, and for a hand that is not soaked: the marks read as what they are and every measure but the trail's fade is on target. **NOT YET**, and named for
the next pass that touches it: a soaked hand's head as a striated mass instead of a slab (probably `flesh.wesl`, which means a WebGL2 re-check; not tried).

## Pending index

- No home-still document on swipes, wipes or contact transfer (three `distill_search` queries); SWGSTAIN is a web PDF, not in the corpus.
- No film still of a smear was gathered; the references are staged and forensic photographs.
- The technique sources are forum answers, not a talk or a paper.
