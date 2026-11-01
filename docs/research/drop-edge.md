# A drop is drawn at the size it is: round, crisp, and not diffused into a diamond

Iteration 8 (2026-10-31). The gap recorded since iteration 4 and carried at the head of three backlogs: on a default
`GorePlugin` floor and wall the stains of a wound's spray read as *rotated squares and rounded diamonds with a pale
jelly-bean glint*, all about one size, where a droplet of blood on a floor is a round, opaque, dark disc and the spray
is "a field of drops from a pin-head to a fingernail" (Ref 7). Iteration 6 named the cause "the floor's texel" and
iteration 7 "the flesh shader's edge on a stain of a few texels". Measured here, it is neither alone: it is **the
canvas's own data** (a speck laid as an area, and a stamp, both of which the spread pass then diffuses), read at a
threshold that cannot give a small thing its size. The fix is on the CPU, in the canvas, and leaves the shader as it is.

## Diagnosis (measured before any change)

`capture_skinned --eye 0.0,1.5,3.1 --at 0,0,1.3 --floor 512`, frame 359, with a scratch dump of the floor's CPU canvas
(`amount_at` of every texel; a scratch edit, reverted):

1. **The floor's drops are diamonds with a plateau.** 56 connected components of blood on the floor, 39 of them between 32 and 62
   texels. One of them, as the CPU holds it 240 ticks after the shot that threw it:

   ```text
    0  0  0  0  5  0  0  0
    0  0  0 12 28 22  3  0
    0  0 12 36 51 46 27  3
    0  5 28 51 75 70 48 25  2         sum / 255 = 4.4 texels of blood, peak 77 of 255
    0  1 24 48 72 77 54 30  8         painted as a bump whose centre was 255 (a Python re-run of `paint_speck`
    0  0  5 29 51 53 35 12  0         at that size gives 255, 145, 71, 7 along a row), moved by the spread pass
    0  0  0  5 27 29 12  0  0         until no two neighbouring texels differ by 25 or more
    0  0  0  0  4  7  0  0  0
   ```

   The mass is what was painted (`paint_speck` conserves the area to within each texel's rounding); the **shape** is the
   spread pass's. The spread is `round(spread_rate / 4 · (a_i − a_j))` on the four neighbours, and at the shipped `0.08` that is
   a whole byte only from a difference of 25, so a wet speck diffuses into the L1 ball of a four-neighbourhood walk,
   flat-topped, with every edge at 45°, and stops where the differences fall under 25: the numbers along the middle row above
   step by 26, 23, 16, 7. That is the diamond. **[INFERENCE]** The flat top, 0.3 of full, is the pale middle seen in the frames: a
   thin film is pale in the film model (`spectral`), and its relief catches the light; not separately measured.
2. **A speck's size is not its own.** `paint_speck` lays the stain's area as a soft bump over twice its size, which the
   shader reads through a cubic B-spline and draws where it crosses `WET_EDGE` (4 %). Simulated (`/tmp/c8/py/run1.py`,
   twelve offsets of each size): a drop of 1.2 texels across is drawn 3.0 across, one of 2.0 is drawn 4.1, one of 3.0 is
   drawn 5.6, with a roundness (largest minus smallest radius over the mean) of 0.24, 0.11 and 0.05. The contour of a faint
   wide bump at a fixed low threshold is about the same size whatever is under it.
3. **A wall and a skin never took a speck.** Every droplet there is `paint_uv_with`, a stamp with 24 spines and satellites
   rasterised at four samples a texel, for a stain of 2 to 8 texels. Its spines are under a texel wide, so they do not read.
   The wall canvas of `capture_skinned --through`, dumped the same way: 180 components of 12 to 124 texels (median 50) with
   peaks of 83 of 255, the same plateau and the same diamond. Iteration 7 measured what the stamps cost; this is their look.

## Sources

1. **Physics.** Comiskey, Yarin & Attinger, *Theoretical and experimental investigation of forward spatter of blood from a
   gunshot*, Phys. Rev. Fluids 3, 063901 (2018), `doi:10.1103/physrevfluids.3.063901` (in the corpus). What it supplies:
   the unit of a spatter is **an individual stain with an area of its own**, "the longest size of the drop stain
   ΔX = ξ l / sin α", "A_i = π l² ξ² / (4 sin α)", and the stain is *as it landed*: nothing in the paper has a stain diffusing
   over the seconds after it lands. A drop's contact line is pinned by the surface; a canvas that lets it creep is
   modelling a film, and a drop is not one.
2. **Horror-film and forensic reference** (`## Reference set`): Ref 1 and Ref 7, measured; the films by URL.
3. **Technique.** Green, "Improved Alpha-Tested Magnification for Vector Textures and Special Effects", SIGGRAPH 2007
   Advances in Real-Time Rendering course (Valve; shipped in *Team Fortress 2*; in the corpus as
   `s2007-advances-course-notes-0-25-mb` and `s2007-advances-pdf-slides-0-9mb`), and Sigg & Hadwiger, "Fast third-order texture
   filtering", *GPU Gems 2* ch. 20, which this crate's shader already reads canvases through.

## Craft principle

The model source for what a *low-resolution texture can be made to say about a sharp edge*, quoted from the corpus
(`distill_search`, `s2007-advances-pdf-slides-0-9mb`):

> "Artist-drawn or rendered alpha channels contain 'coverage' values. ... Textures are reconstructed by the GPU using
> bilinear interpolation. 'Coverage' is not a linear function for interesting cases. ... Thresholding other edge types
> using interpolated low-resolution coverage will produce artifacts."

> "The distance between a point and an infinite line segment is a 2d linear function. ... If we store the signed distance
> between the sample location and the shape, bilinear filtering will produce a piece-wise linear approximation of the
> original shape boundary. This produces 'infinite resolution' linear edges."

This crate stored a *coverage* (the stain's area, as a blur) for a thing smaller than its texels and thresholded it at a fixed
low level: the artefacts are those, a size that does not follow the shape and a contour that follows the grid. The cure
is the paper's: store a quantity the reader reconstructs *exactly*, and let the threshold the reader already applies
(`WET_EDGE`, `DETAIL_EDGE`, 4 %) fall on the shape.

## How shipping games do it

*Team Fortress 2* shipped Green's technique for signs, decals and HUD: a 64 by 64 distance field read through the hardware's
bilinear filter gives crisp curved edges at any magnification, "fast, easy to implement, works on all graphics hardware".
The slide on its limit is the one that matters here: "corners will get chopped or rounded off as lower resolutions are used",
which for a *drop* is a virtue, since a drop has none. Doom Eternal's mesh decals (Geffroy, SIGGRAPH 2020; cited in
`floor-spatter.md`) project a pre-drawn stain sprite and so are as sharp as the sprite; the canvas here has no sprite, so the
distance is written, per drop, by the CPU, which is what keeps it reproducible.

## Model adopted

One new primitive and one change of which drops use it. No shader, no `WET_EDGE`, no dial changes.

**`WetCanvas::paint_cone(uv, semi, direction, tick)`**: the canvas under a droplet's stain is a *cone of blood*, `DETAIL_EDGE −
CONE_SLOPE · d` for `d` the signed distance of each texel's centre from the stain's edge (negative inside), and nothing
more than `DETAIL_EDGE / CONE_SLOPE` (0.44 texel) outside it. The stain's edge is then, exactly, where a reader that draws
at `DETAIL_EDGE` draws it: a linear field is reproduced by the cubic B-spline, so the contour sits on the ellipse, measured
(`/tmp/c8/py/cone.py`, `calib.py`; the integration test repeats it) within 1.5 % of the stain's area from a radius of 1.5 texels at every
offset in the texel, and within 11 % at a radius of one. The distance is first order, `(ρ − 1) · ρ / |∇ρ|` for the ellipse's `ρ`:
exact on a circle, close at the edge of an ellipse, which is where it is read.

Why it holds still: the slope, `0.09` per texel, is 23 of 255 between neighbouring texels, under the 25 the spread pass
rounds to a byte at `spread_rate 0.08`; its peak (0.04 + 0.09 r) is under the drip pass's threshold (89 of 255) to a radius of 3.3.
With the substrate soaking nothing, six hundred ticks leave every byte of a cone as painted (`a_cone_does_not_creep_while_it_dries`).
With the shipped substrate the drying takes a cone's rim as it takes any film under `absorbency`'s 38 bytes: a drop shrinks as it
dries and a cone of a texel's radius soaks away, which is the model's own ("faint spatter soaks away to nothing"), not a creep.

Which drops: `FINE_TEXELS` goes from 4 to **8**. A stain under eight texels across is a cone, and a spine on a stain that
size is under two texels, which no canvas of its resolution shows and the spread erases. A floor took specks under four
texels already; **a wall and a skin now take their droplets through the same `paint_drop`** (a cloth keeps its blob: its
weave and `Soaking` read it). Stamps, with their spines and satellites, are for the stains big enough to carry them.

## Parameters

| Parameter | Value | Provenance |
|---|---|---|
| `CONE_SLOPE` | `0.09` per texel | **Tuned, and says so.** Under the 25-byte step the spread pass rounds to a flux at the shipped `spread_rate` (23 of 255 between neighbouring texels); above, the cone creeps. The measured fit at 0.12 is no better (drawn-to-true area 0.98 to 1.03 from a radius of 1.2) and is not stable. |
| `CONE_MIN_SEMI` | `0.7` texel | **Tuned.** Under a radius of about 0.6 the smoothed peak of a cone never reaches 4 %: a smaller drop would not be drawn at all. |
| `CONE_MAX_SEMI` | `4` texels (eight across) | The stamp takes over (`FINE_TEXELS`). |
| `CONE_LIFT`, `CONE_LIFT_UNTIL` | `0.15 · (1.85 − r)²` for `r` under 1.85 | **A fit**, to the drawn radius over 16 offsets at each of nine radii from 0.7 to 2.4 (`calib.py`): within 0.03 texel of the stain's own. |
| `FINE_TEXELS` | 4 → 8 | **Tuned.** |

## Validation targets

Set before the code; graded in the log.

1. **Size.** The area a reader draws at `DETAIL_EDGE` is the stain's own, `π a b`: within 5 % from a radius of 1.5 texels and
   within 15 % at one, at every offset (`a_cone_is_drawn_at_the_size_it_was_given_and_round`).
2. **Round.** The drawn contour's radius varies by under 20 % of its mean about its own centroid (a quarter at a radius of one).
3. **It holds still.** With the substrate soaking nothing, 600 ticks leave every byte of a cone as painted; with the shipped
   substrate a cone never grows (`a_cone_does_not_creep_while_it_dries`).
4. **No locks move but the recorders'.** `hash_f32`, the spatter and stain goldens, `the_flesh_tables_are_frozen`: untouched.
   `bench_carnage` (it never calls the preset): its digest `0x4366a8b64352c85a` unmoved. The recorders' digests move
   where a drop under eight texels lands, and nowhere else (`--stroll`, `--quiet` equal to 0.8.1's).
5. **Cost.** `hit_tick_cost`: no case slower; a heavy wound facing a wall, 19 ms before, far lower.
6. **Look**, against Refs 1 and 7, in the log's critique: no rotated squares, drops round, sizes from a few millimetres up, no pale
   glint on a small drop.

## Deviations

- **A new public primitive, three constants and an exported meaning.** `WetCanvas::paint_cone`, `wetmap::{CONE_SLOPE, CONE_MIN_SEMI,
  CONE_MAX_SEMI}`. Additive. `paint_speck` and its tests stay: it is the area-conserving primitive, and a caller who wants
  the stain's blood as a tint rather than its outline still has it. The preset no longer calls it.
- **A cone is not a mass.** Its bytes do not sum to the stain's area; the area is the area inside its edge. A caller that
  sums `amount_at` over a floor to estimate how much blood landed gets less from a cone than from a speck or a stamp.
  Marked a design default.
- **Every recorder's digest with a wall, a skin or a floor that took a drop under eight texels moves**, on purpose: the bytes
  are different. The look changes with them; the model above the canvas (where a droplet lands and how big its stain is) does
  not.
- **The wall and the skin lose the stamp's spines on a stain under eight texels.** They were under two texels wide and did not
  read (iteration 7's finding); a drop reads as an ellipse along its throw. A *tail* (the exclamation mark of an oblique
  impact) is the next lever and is not built here.

## Pending index

- Comiskey et al. 2018, Figs. 7 and 9 (stains and stain area against distance): the figures are not in the markdown;
  carried from `floor-spatter.md`.
- Hulse-Smith, Mehdizadeh & Attinger 2005 (`doi:10.1520/jfs2003224`, stain diameter against drop diameter and impact velocity):
  cited in `corpus.md`; not re-read this pass.

## What great looks like

A shot into a wall or a floor leaves a field of separate, **round, dark, opaque drops**, from a few millimetres to a few
centimetres, each at the size its stain is, none a square, a diamond or a jelly bean, none with a pale glint in its middle,
the oblique ones drawn out along the throw. At two metres they read as a fine, dense spatter; at a metre as drops. Ref 7 is the
floor, Ref 1 the wall.

## Reference set

Kept under `/tmp/c8/refs/`, never committed. Measured with `/tmp/c8/stains.py` (components of pixels with `R − max(G, B) > T`,
equivalent diameters in the image's own pixels).

1. **Ref 1**: "Blood Spatter Texture - FREE", jimmy brown, CC BY 2.0,
   <https://commons.wikimedia.org/wiki/File:Blood_Spatter_Texture_-_FREE.jpg>. A wall: 231 stains in the frame, 151 at
   least 3 px, equivalent diameter p10 1.6 px, p50 4.1, p90 15.3, max 55 (p90 / p10 9.6). Drops round with short tails; a few
   large stains with satellites; a fine mist between. Mean `(163, 53, 41)`.
2. **Ref 7**: "Blood, guts and gore galore, Moulage crew brings horror to life", Spc. Anthony Zane, US Army, public domain,
   <https://commons.wikimedia.org/wiki/File:Blood,_guts_and_gore_galore,_Moulage_crew_brings_horror_to_life_140723-A-IX584-092.jpg>.
   The floor: a drip and spatter field on a grey floor, round opaque drops from 1 px to 90 px, one cluster merged; the gel wound
   above it is the wound-sheen item.
3. **Ref 8**: the same photograph series, `...140723-A-IX584-057.jpg`, public domain: moulage gel appliances laid out on a table, each a
   translucent gel piece with dark glossy blood in a cut. Wound makeup, for the wound-sheen item; not used for the drops.
4. *Evil Dead* (2013), KNB's methylcellulose blood, <https://filmhounds.co.uk/2023/04/17/gory-groovy-the-splatter-of-evil-dead/>,
   press page (read as text, not as an image, this pass; iteration 6 measured its still): the craft source for thrown blood being
   thick enough to keep a drop's shape.
5. *Terrifier 3* (2024), Variety, <https://variety.com/2024/artisans/news/terrifier-3-kills-art-the-clown-1236177913/>, press,
   reference only, measured in iteration 6: flecks and drops on white fur, a spread of sizes (p90 / p10 24).
