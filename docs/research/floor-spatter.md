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

Two changes, no new physics, both in the preset, which is the one module that owns a look.

1. **A floor stamps each droplet as its own physical stain.** The closed-form landing of `stains` stays the
   placement (it is what puts a drop where the throw says). The stain's size, stretch, spines and satellites are
   `hard_stain`'s, from the speed and slant the droplet arrives with (`impact_at_plane`'s relation), exactly as a wall
   or a skin has had since 0.5.62. The floor's UV comes from a cast down on the landing point through the mesh's
   hierarchy (`crate::bvh`), not from the canvas walking every triangle of the floor once per drop.
2. **`GoreDials::default()` states the preset's droplet density**, as iteration 4 made it state the preset's flight
   scale: `blood.droplets_per_m2 = GORE_DROPLETS_PER_M2`. `BloodSettings::default()` keeps `2400`: its constants are
   the paper's and the bloodstain goldens pin them. The ceiling `max_droplets_per_wound = 512` stays, and bounds the
   floor pass; the number of droplets cast at walls and bodies (`SPRAY_CAST`, `WALL_CAST`) stays, so their cost is
   bounded as it was.

## Parameters

| Parameter | Value | Provenance |
|---|---|---|
| `GORE_DROPLETS_PER_M2` | to be set against the validation targets below | **Tuned, and says so.** No paper gives a droplet count per wound area at game scale; the paper's stain counts are in a figure the corpus does not extract. Set so one bullet-sized crater leaves a fan of about a hundred stains on the shipped floor, against 111 drops in Ref 7's fan and 220 in Ref 1's wall. A design default. |
| `BloodSettings::droplets_per_m2` | `2400` (unchanged) | The leaf's own. |
| `max_droplets_per_wound` | `512` (unchanged) | Bounds the floor pass and a consumer's effect capacity. |
| floor stain shape | `hard_stain` | The wall's and skin's, unchanged. |

## Validation targets

1. **Count.** A 3.5 cm impact at chest height on the shipped floor (256 texels, 3 m) leaves at least 60 separate stains
   (components of `amount > 32`) in the preset's own test, where it leaves under 15 now.
2. **Drops, not sheets.** The largest stain's width is at most 15 cm and the median at most 4 cm; the wetted area is
   under 5 % of the floor's fan. Ref 7: p90 / p10 of the equivalent diameters 7.7; Ref 1: 10.3. The floor at 12 mm a
   texel cannot reach those spreads (its smallest stamp is 18 mm), so the target on the floor is p90 / p10 of at
   least 2, and at 512 texels at least 3.
3. **Cost.** One hit tick (`hit_tick_cost`, an ignored measurement test, median of 45 hits) on a floor of two
   triangles and one of twenty thousand, with and without a wall behind: no more than 1 ms over the tick as it is
   now, and the 20 000-triangle floor no more than 1 ms over the 2-triangle one (the cast goes through the index).
4. **Bounded.** No wound stamps more than `max_droplets_per_wound` stains on a floor.
5. **Locks.** `BloodSettings::default()` untouched: every bloodstain golden passes without edit, `bench_carnage`'s
   digest `0x4366a8b64352c85a` and its timing unmoved (it does not call the preset).
6. **Determinism.** Every recorder variant prints one digest twice; the ones with a hit and a floor move on purpose,
   `--stroll` and `--quiet` keep theirs.
7. **Look**, measured off the top-down capture against Refs 1 and 7 (`## Reference set`): at least 60 components,
   p90 / p10 at least 2, the blood's darkest fifth redder than `(110, 40, 40)`, a fan along the throw direction.

## Deviations

- **A changed default, marked a design default.** `GoreDials::default().blood.droplets_per_m2` `2400 -> GORE_DROPLETS_PER_M2`.
  It moves every default-dials recorder digest whose clip has a hit; it changes how many droplets a wall, a body part
  and a cloth are cast at (the first `SPRAY_CAST` / `WALL_CAST` of the wound's own stream: the droplets that were
  already there are the same droplets). A caller who authored `droplets_per_m2` keeps it.
- **Not read off the paper.** The count is tuned, not derived: the paper gives stain counts per 25 cm bin in a figure
  this host could not extract. Recorded under `## Pending index`.
- **Not done: the size/speed inversion.** The model still makes the smallest droplets the fastest and lands them
  farthest (the paper's drag makes small drops fall short); fixing it moves four frozen goldens and is the owner's
  (iteration 4's note). With more droplets the fan is wider than the paper's near field at the small end.
- **Not done: sub-texel coverage.** A stain under the floor's texel is stamped at 1.5 texels, saturated.

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
