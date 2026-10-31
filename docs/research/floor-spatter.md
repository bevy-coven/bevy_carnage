# Floor spatter: what a wound throws reads as drops on the floor, not nine pieces of jam

Iteration 6 (2026-10-31). The gap recorded since iteration 4 and carried at the head of two backlogs: a default
`GorePlugin` floor under a shot body takes about nine ragged blobs of 10 cm each (`capture_skinned --eye
0,4.2,0.3 --at 0,0,0`, frame 359: 22 components, 7 of them at least 3 px across, one of them 62 px). A visible
item. The code is the floor half of `throw_spray` in `src/preset.rs` and `GoreDials::default`. The parked first
draft is branch `loop/iter-4-floor-stains` (`bffcbe8`).

## Diagnosis (measured before any change)

Two separate causes, one picture:

1. **The floor stamps a sheet's blob.** Walls and skins have taken one `hard_stain` per droplet since 0.5.62, sized,
   stretched and pointed by how the droplet arrived. A floor still stamps `stain.radius * 2.2` square, the
   *placement* scale (2 to 12 cm a side, `stain_radius`) for a droplet 0.8 to 6 mm across. The note on
   `Stain::radius` says it is "not the drawn silhouette".
2. **Too few droplets.** `droplet_count = area * droplets_per_m2 * severity`, `2400` per m². A bullet crater of 3.5 cm
   radius is 3.85e-3 m², so **9 droplets**; a chest wound of 0.02 m² is 48. The floor takes the ones nothing above
   caught. A shot throws a few blobs where a film's blood bag throws a fan.

The first was tried alone last iteration (branch `loop/iter-4-floor-stains`): seven dots of 2 cm, the right size and
sparse, "no more legible until the count is decided".

## Sources

1. **Physics.** Comiskey, Yarin & Attinger, *Theoretical and experimental investigation of forward spatter of blood
   from a gunshot*, Phys. Rev. Fluids 3, 063901 (2018), `doi:10.1103/physrevfluids.3.063901`, in the corpus (stem
   `10.1103_physrevfluids.3.063901`, read with `markdown_read` this pass). What it supplies: that a gunshot makes a
   *population* of drops, not a handful (percolation: "drops of different sizes comprised of a different number of
   elementary droplets of size a0"), that the measured quantity on the floor is *the number and area of individual
   stains* binned by distance, and the stain-area law this crate already uses (`A = pi l_w^2 xi^2 / (4 sin alpha)`,
   the impact angle's ellipse). The paper's figure of the stain count against distance is an image the corpus
   does not extract (the document store answered 504 from this host), so **the count is not read off it**; see
   `## Deviations`.
2. **Horror-film and forensic reference** (`## Reference set`): Ref 7, a US Army moulage photograph (public domain)
   with a dripped floor and a gel wound, is the new one this pass; Refs 1, 2, 4, 5 are iteration 4's; measured here.
   Practical-effects craft: Tom Savini's rule that fresh blood is bright and dried blood dark brown ("blood takes 24
   hours to turn dark brown"), Dick Smith's formula adding green so the colour stays dark and not "candy apple
   bright red" (RNZ interview with Savini, `rnz.co.nz/national/programmes/afternoons/audio/2018762272/`), and
   Damien Leone on *Terrifier 2*: powdered milk "to kill the translucency", fat for "chunky" blood
   (`slashfilm.com/1440753/terrifier-2-grisly-death-scenes-involved-some-unlikely-gross-ingredients/`).
3. **Technique.** Doom Eternal's mesh decals (Geffroy, *Rendering of Doom Eternal*, SIGGRAPH 2020,
   `advances.realtimerendering.com/s2020/RenderingDoomEternal.pdf`; Coenen's graphics study,
   `simoncoenen.com/blog/programming/graphics/DoomEternalStudy`): a decal is a projected draw, so a mesh is capped
   at 255 of them. Left 4 Dead 2 caps overlapping decals with `r_decal_cover_count` and masks identical
   textures by variation. This crate's canvases are the opposite trade: a droplet is a stamp into a CPU byte buffer,
   so there is no per-droplet draw, no cap but the canvas, and a hit pays for the stamps it makes.

## Craft principle

The model source, quoted. A gunshot does not throw a handful of drops; the fragmenting blood leaves as a spread of
sizes, many small and few large, and the paper's own unit of observation is the stain count and the stain area on
the floor:

> "The model predicts the number and area of individual stains, as well as the stain distribution as a function of
> distance from the region of origin." (abstract)

> "In the subcritical case only finite clusters can exist, which corresponds to drops of different sizes comprised of
> a different number of elementary droplets of size a0." (Sec. III)

> "The blood spatter was produced on a butcher paper substrate strip (0.41 x 4 m^2) located on the floor behind the
> target. The substrate was then digitally scanned at 600 dpi and analyzed for the number of stains and their areas as
> a function of the distance traveled from the rear side of the target." (Sec. II)

So a floor under a wound is a field of *individual stains*, each with its own area, and a model that lands nine of them
for a shot has answered a question the paper does not ask. The film's artists answer it the same way: a pressurised
rig or a blood bag throws a fan, and the audience reads the floor and walls round the victim (see
`spray-reach.md`, "What was learned about the effect").

## How shipping games do it

Doom Eternal, quoted: a mesh decal is rendered into an ID buffer and projected at shading time. "This is extremely
fast and allows artists to go crazy with massive amounts of decals", but "Because the IDs are rendered to an 8-bit
texture, the maximum amount of decals per mesh would theoretically be 255." A game that wants a floor spattered with
hundreds of drops per shot, for every shot, over a whole fight, hits that 255 within three shots. Texture-space
damage has no such number: this crate's canvas is a byte buffer and a stamp is a few dozen texels written once. The
price is the one `Gore::canvas` already names: a stamp cannot be finer than its texel, so the floor at 12 mm a texel
cannot draw a 2 mm drop at its own size (`shape_on`'s floor of 1.5 texels). What it can draw is how many there are,
where they fall, which way each is drawn out and how big the big ones are.

## Model adopted

Three changes, no new physics, in the preset (the one module that owns a look) and one new canvas primitive.

1. **A floor stamps each droplet as its own physical stain.** The closed-form landing (`crate::bloodstain::landing`, flown
   at the dial's `spatter_speed_scale`) stays the placement. The stain's size, stretch and slant are `hard_stain`'s from
   the speed and angle the droplet arrives at, as a wall or a skin has had since 0.5.62. The floor's UV comes from a cast
   down on the landing through the mesh's index (`crate::bvh`), not from the canvas walking every triangle once per drop.
2. **The floor's mist.** A wound throws `floor_mist` (`9`) fine droplets at the floors for each droplet it throws at
   everything: the same stream (`droplet(wound, i)` is a pure function of the wound and the ordinal, so ordinals `0..9`
   are what they were and the mist is `9..90`), each cast at the same surfaces and caught by the first it reaches, stamped
   only on a floor. **A wall, a skin and a cloth take the primaries alone**, so what they take and what stamping it
   costs are what they were. This replaced a first design that raised `droplets_per_m2` for every surface: measured, a
   stamp of four texels or more costs 0.2 to 0.4 ms (`exp` and `atan2` per sample, spines and satellites at four samples
   a texel), a wall at 512 casts up to 128 droplets a wound, half of them that size, and `hit_tick_cost` already reads
   28 ms for a heavy wound facing a wall at the leaf's count.
3. **A droplet under four texels across is a speck** (`WetCanvas::paint_speck`), not a stamp. Measured on the floor at 12 mm
   a texel: a stamp of two texels peaks at 73 of 255 (all four of its texels lie off the profile's centre), one of three at
   243, a slanted one of one at 18, and a stain smaller than the stamp's floor of a texel and a half is drawn as a whole
   one; a two-texel stamp with its spines costs 45 us. A speck is the blood of an ellipse (its area, in bytes) placed to a
   fraction of a texel, so the blood a canvas cannot resolve reads as the tint it would be and a size spread survives at any
   resolution; it costs about 2 us. **It is spread over a soft round footprint, not laid as a hard disc**: a first version
   laid the area hard-edged, and the floor read as rotated squares (below, `## Deviations`). The flesh shader reads a canvas
   through a cubic B-spline and draws the stain where that crosses 4 % coverage, and a disc on a texel grid is a square after
   that; a `(1 - rho^2)^3` bump, normalised to the area, over `SPECK_GROW` (2) times the ellipse's axes and at least
   `SPECK_FLOOR` (1.2) texels, comes out round (`/tmp/c6/speckshape.py` read the bump through the B-spline and the edge, for
   ellipses of 0.2 to 1.4 texels). From four texels across a stain is a stamp as before.

## Parameters

| Parameter | Value | Provenance |
|---|---|---|
| `GoreDials::floor_mist` / `GORE_FLOOR_MIST` | `9` | **Tuned, and says so.** No paper gives a droplet count per wound area at game scale; the paper's stain counts are in a figure the corpus does not extract. Set so a bullet-sized crater (9 droplets at the leaf's count) leaves a field of stains on the shipped floor: 25 stains in the preset's test, 4 with no mist; the reference photograph's fan has 111 in 45 cm. A design default. |
| `BloodSettings::droplets_per_m2` | `2400` (unchanged) | The leaf's own; the walls', skins' and cloths' count. |
| `max_droplets_per_wound` | `512` (unchanged) | Bounds the mist too: primaries plus mist never exceed it. |
| `FINE_TEXELS` | `4` | **Tuned, and says so.** Where a spine on a stain is a texel wide (the longest spine is about 0.6 of the body's width, and the mask holds the whole silhouette). |
| `SPECK_MAX_SEMI` | `2` texels | Four texels across, `FINE_TEXELS`: a stain wider than that is a stamp's. |
| `SPECK_GROW`, `SPECK_FLOOR` | `2`, `1.2` texels | **Tuned, and say so.** The footprint's reach past the ellipse, and its least; the least is past 0.71, the farthest a point is from the nearest texel's centre, so a speck always lands. Chosen by reading the bump through the B-spline and the edge. |
| floor stain shape | `hard_stain` | The wall's and skin's, unchanged. |

## Validation targets

Set before the code, and graded in the log against what was measured.

1. **Count.** A 3.5 cm impact at chest height on the shipped floor (256 texels, 3 m) leaves at least 15 separate stains and
   at least four times as many as with the mist at nothing (`a_shot_leaves_a_fan_of_drops_on_the_floor_not_a_few_blobs`).
   *Measured: 4 without, 25 with.* About half the droplets land past the slab's far edge: the count is geometry-bound.
2. **Drops, not sheets.** The widest stain at most 12 cm across. *Measured: 7 cm.* On a 12 mm texel a stain's apparent
   size is the flesh shader's crisp-edge threshold on a B-spline of its few texels (about 2.6 texels across for a
   saturated one), so the smallest drop reads 3 cm; at 512 texels, 1.6 cm.
3. **The wall and the skin are untouched** (`a_wall_and_a_skin_take_no_mist`: the same shot, mist at nothing and at its
   default, leaves both canvases' digests equal) and **nothing passes through a wall** (`the_mist_does_not_pass_through_a_wall`:
   373 texels wetted behind the wall's place without it, none with it). In the recorders `capture_skinned --through`'s frames
   differ from 0.7.0's only in a 20 by 5 pixel patch at the bottom edge, and not on the wall.
4. **Cost.** `hit_tick_cost`, median of 45 hits, alternated with the tick as it was, five times each (a body, a floor of two
   triangles and of twenty thousand, a wall 0.9 m behind the wound): no hit tick more than 1.5 ms over the old one, and the
   20 000-triangle floor cheaper. *Measured: see the log.*
5. **Locks.** `BloodSettings::default()` untouched: every bloodstain golden and `the_spatter_model_is_frozen` pass; `bench_carnage`'s
   digest `0x4366a8b64352c85a` unmoved (it does not call the preset).
6. **Determinism.** Every recorder variant prints one digest twice; the ones with a hit and a floor move on purpose,
   `--stroll`, `--quiet` and `--guard` keep theirs.
7. **Look**, against Refs 1 and 7 (`## Reference set`), in the log's critique: a fan of separate stains of varied size along the throw,
   none a sheet; the albedo the canvas writes for a saturated texel (`(113, 33, 42)` venous, `(138, 24, 41)` arterial at 2 mm, from
   `spectral::srgb`) against the references' darkest fifth `(107, 15, 33)`, `(136, 29, 20)`. The *lit* frame's colour is the scene's
   and is not a target here.

## Deviations

- **A new public field.** `GoreDials::floor_mist` (default `9`) changes what every default `GorePlugin` floor takes and breaks a
  struct literal that names every field. Marked a design default; pre-1.0, so the next release is a MINOR.
- **Not read off the paper.** The count is tuned, not derived: the paper gives stain counts per 25 cm bin in a figure this host
  could not extract. Recorded under `## Pending index`.
- **Not done: the size/speed inversion.** The model still makes the smallest droplets the fastest and lands them farthest (the
  paper's drag makes small drops fall short); fixing it moves four frozen goldens and is the owner's (iteration 4's note). The
  mist, drawn from the same stream, spreads out to the same 0.6 to 5 m field.
- **Not done: the mist on a wall, a skin or a cloth.** A wall in Ref 1 carries 220 stains; the preset's carries nine (a bullet) to a
  few dozen. Stamping 90 droplets on a wall costs 0.2 to 0.4 ms each at 512 texels, so it needs the large stamp to get cheaper
  first (the rasteriser's per-sample `exp` and `atan2`), or a budget of detailed stamps a throw. The next item.
- **Not done: sub-texel colour.** A speck of a tenth of a texel adds a tenth of full coverage, which the flesh shader draws as a
  thin film (pink at the edge, darker in the middle); the *lit* frame is paler than the albedo (`(122, 55, 60)` against the
  texel's `(113, 33, 42)`), the scene's ambient and specular.
- **A found flaw, not fixed: a heavy wound facing a wall costs 28 ms** (`hit_tick_cost`, `floor2_wall_big`, the same before and
  after this item): 48 spined stamps of 0.2 to 0.6 ms each on a 512 wall. A hit that spikes the frame; logged for the next pass.

## Pending index

- Comiskey et al. 2018, Fig. 7 (stains against distance) and Fig. 9 (mean stain area against distance): the figures,
  not the text, hold the counts. `doc_link` returned a link the host could not fetch (HTTP 504 through the cloud
  gateway, the LAN address unreachable from here). A future pass with the PDF can replace the tuned count with the
  measured order of magnitude.
- Stauffer & Aharony, *Introduction to Percolation Theory* (the cluster-size exponent behind "many small, few large"):
  a textbook, not in the corpus.
- Geffroy, *Rendering of Doom Eternal*, SIGGRAPH 2020 and Coenen's study: web pages, cited by URL.

## What great looks like

A shot into a body in the middle of a three-metre room leaves a fan on the floor in front of the wound: a hundred or so
separate drops, each its own size and its own outline, the nearest dense and small, a few larger ones drawn out along
the throw with their tails pointing downrange, none a disc of one size and none a jam-jar blob (Refs 1 and 7). It is
dark red where the drop is thick, brighter where it is thin, and no two drops touch except where a bigger one has
landed on a smaller one. It must read as spatter from the camera's distance (a few pixels a drop) and as drops at a
metre. The floor's cost is a few dozen microseconds per drop.

## Reference set

Kept under `/tmp/c6/refs/`, never committed. Measured with `/tmp/c6/stains.py` (components of pixels with
`R - max(G, B) > 100`, equivalent diameters of each, in the image's own pixels).

1. **Ref 1**: "Blood Spatter Texture - FREE", jimmy brown, CC BY 2.0,
   <https://commons.wikimedia.org/wiki/File:Blood_Spatter_Texture_-_FREE.jpg>. Spray on a wall: 220 stains in the frame,
   145 of them at least 3 px across, equivalent diameter p10 1.6 px, p50 4.4, p90 16.4, max 55 (**p90 / p10 10.3**),
   round with short tails and satellites. Mean `(162, 53, 41)`, darkest fifth `(136, 29, 20)`.
2. **Ref 2**: "2023 Blutstropfen Wischspur", Georgenhausen, CC0,
   <https://commons.wikimedia.org/wiki/File:2023_Blutstropfen_Wischspur.jpg>. Six drops on a board plus a wipe, 24
   components; round, crisp-edged. Mean `(160, 31, 16)`, darkest fifth `(137, 5, 2)`.
3. **Ref 3**: "Calculating the area of origin of a blood splatter", Ashley Do, CC BY-SA 4.0,
   <https://commons.wikimedia.org/wiki/File:Calculating_the_area_of_origin_of_a_blood_splatter.jpg>. The tail of each stain
   points back at the wound.
4. **Ref 4**: *Evil Dead* (2023), FILMHOUNDS, <https://filmhounds.co.uk/2023/04/17/gory-groovy-the-splatter-of-evil-dead/>,
   press still, reference only. Blood filming a face: mean `(123, 28, 25)`, darkest fifth `(86, 8, 13)`: dark, saturated,
   wet highlights. Fresh blood on screen is **not bright**: thick film reads near-black crimson.
5. **Ref 5**: *Terrifier 3* (2024), Variety, <https://variety.com/2024/artisans/news/terrifier-3-kills-art-the-clown-1236177913/>,
   press still, reference only. Flecks and drops on white fur and an axe head: a spread of sizes (p90 / p10 24), the older
   ones brown-black, darkest fifth `(84, 38, 21)`.
6. **Ref 6**: *Evil Dead* (2013), I Love Splatter, <https://ilovesplatter.com/2013/04/05/evil-dead-2013/>, press still,
   reference only. Blood across a face and collar, matte dark crimson.
7. **Ref 7** (new): "Blood, guts and gore galore, Moulage crew brings horror to life", Spc. Anthony Zane, US Army,
   public domain, <https://commons.wikimedia.org/wiki/File:Blood,_guts_and_gore_galore,_Moulage_crew_brings_horror_to_life_140723-A-IX584-092.jpg>.
   **The floor reference**: a drip and spatter field on a grey floor, 111 stains in the fan crop (470 x 512 px, about 45
   cm), diameters p10 2.3 px, p50 5.9, p90 17.4, max 92 (**p90 / p10 7.7**): a dense core of small round drops, a thinning
   scatter, one cluster where drops merged, every outline crisp. Mean `(140, 63, 63)`, darkest fifth `(107, 15, 33)`.
   The same photograph's wound is a raised gel with torn edges, a specular highlight on its surface and a bright
   `(193, 87, 90)` mean where it is thin: for the wound-sheen item, not this one.

## What was learned about the effect, for the artist's eye

Savini's rule is that blood the audience reads as fresh is bright and thin, and as old, dark brown; Dick Smith's formula
adds green to keep a film dark so it does not read "candy apple bright red". Leone puts powdered milk in to remove
translucency and fat in for body: a film that is opaque and has body reads as blood, a thin translucent one as paint.
Ref 7's drops are opaque at every size and their darkest part is near `(107, 15, 33)`.
