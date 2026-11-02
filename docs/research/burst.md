# The burst: a body that comes apart throws its blood in every direction

Iteration 18 (2026-11-02). Item 9 of the owner's 80s-horror list: "A burst hit. A radial fracture of a head or torso, a spherical droplet cloud, and blood stains on every nearby surface, plus viscera. This is
*Scanners*. It mostly composes parts we already have." **The parts are all here and the cloud is the one thing that is not.** A hit that reaches bone breaks a baked body into pieces and spills a torso's guts
(`take_hits`), and a wound's spray (`throw_spray`) is a cone about its normal: the percolation cone of `bloodstain::droplet`, 32 degrees half-angle. A game that wants a head to burst has the pieces and nothing
for the air round them: no spatter on the wall behind, none overhead and to the sides, none on the shoulders of what is left. Why this and not the runner-ups (iteration 17's backlog): *a film per tissue* in
`flesh.wesl` (the strip's muscle never reaches the preset's screen because a 1 mm film of blood sits over it; the fix is a shader change and its WebGL2 re-check wants a cold wasm build this disk cannot hold);
*curve the hose* (polish on a feature that works); *a second coordinate for a cap* (geometry that moves every recorder's digest, and is hidden by the same film); *a non-red fluid* (the wetmap holds `(amount, age)`
per texel with no room for which fluid, so it can only be one fluid a scene: more than one iteration).

## What was measured before any change

- `grep -n "spatter_cone_deg" src/`: the only spray the preset has is the cone, `32.0` degrees either side of a wound's normal. A game cannot ask for a sphere.
- `bench_carnage` on 0.8.11, three runs at load 3.8: `carnage_frame_ms` 0.09258, 0.09247, 0.09357 (median 0.09258), `carnage_p99_ms` 2.645, 2.604, 2.685 (2.645); digest `0x4366a8b64352c85a`.
- `capture_skinned` default `5f8eb5573aea933f` (`flay=e1c8e99a7953b57f dermis=90cb9d2114068ac2`), `--hose` `0c903766bf51f0f2`, `capture_preset` `04758d82ec10aa5b` (`flay=5ee1e742f1726133 dermis=056f49e5af3be51d`), each twice.
- The headless host's corpus (`catalog_recent`, limit 200): 142 events, none in a domain this crate reads (psychology and cyber-psychology titles); `system_status` healthy (18 673 documents), `scribe_health` reachable.

## Sources

- **Physics** (what leaves a destroyed volume, and how fast). Comiskey, Yarin & Attinger, "Theoretical and experimental investigation of forward spatter of blood from a gunshot", *Phys. Rev. Fluids* 3, 063901
  (2018), `doi:10.1103/PhysRevFluids.3.063901` (in the corpus; the model `bloodstain::droplet` reduces): "The chaotic disintegration of a blood layer located ahead and aside of the bullet is considered in the
  framework of percolation theory. The size distribution of blood drops is determined, which allows for the prediction of a blood spatter cloud being ejected from the rear side of the target", and "droplet
  trajectories are numerically predicted accounting for gravity and air drag". Their introduction names the assumption a reconstruction rests on: "a common assumption that has been in use since at least as early
  as the 1950s is called the straight-line trajectory assumption, and it forms the basis of a class of evaluation techniques known as the method of strings". That assumption is also the **test** here: stains of a
  burst point back to its centre. Forensic science, *Bloodstain Pattern Analysis: Principles* (`https://www.forensicsciencesimplified.org/blood/principles.html`): "Typically, forward spatter is a fine mist and
  back spatter is larger and fewer drops." "When blood is impacted, droplets are dispersed through the air. When these droplets strike a surface, the shape of the stain changes depending on the angle of impact,
  velocity, distance travelled and type of surface impacted." Ledo et al., "Defining Patterns and Behaviours of Forward Spatter Gunshot Misting" (*Forensic Sciences* 1(2), 2021; the poster,
  `https://www.csfs.ca/wp-content/uploads/2021/06/14-Bianca_LEDO_Defining-patterns-and-behaviours-of-gunshot-misting.pdf`): "Misting Pattern - Bloodstain pattern with a diameter less than 1mm that is created by
  a high amount of energy & velocity". **The corpus has no source on a head or torso bursting** (`distill_search` on temporary cavity, explosive exit and intracranial pressure returned the Comiskey paper and a
  skull-fracture paper that does not mention them): the cloud is built from the spatter model the crate already has, spread over a sphere, which is stated as a deviation below.
- **Horror-film reference** (what the effect looks like, and what it is made of). Wikipedia, *Scanners* (`https://en.wikipedia.org/wiki/Scanners`), on Dick Smith's and Chris Walas's exploding head: "a gelatin-encased
  plaster skull packed with 'leftover burgers' as well as 'latex scraps, some wax, and just bits and bobs and a lot of stringy stuff that we figured would fly through the air a little better'", shot in the back of
  the head with a shotgun. So the film's burst is **chunks and strings with a fan of fine red droplets round them**, and the droplets alone are half of it; the other half is the pieces this crate's break already
  throws. The stills (Reference set) show the cloud as a fan **three or four head-widths across**, densest at the head, fine at its edge, bright fresh red. *Maniac* (1980), the shotgun scene: the aftermath is a
  car interior sprayed red from one point, the seat and the glass dark with it.
- **Technique** (how shipping games put a burst in the world). Dambuster's *Dead Island 2* FLESH system (Motionographer, `https://motionographer.com/2025/01/27/dead-island-2-a-stunning-visual-zombie-gore-feast/`):
  zombies have "separate layers of skin, muscle, fat, and bones" and blood splatters show "on the environment"; Gfinity's review (`https://www.gfinityesports.com/article/dead-island-2-review`): "Even with the
  slightest hit to the head using a plank of wood, a shower of blood covered the nearby area". *Doom Eternal* players' own measurement (Steam discussion `https://steamcommunity.com/app/782330/discussions/0/2247803885928042792/`):
  "the amount of blood and gore is maximized when you are closer to what you are killing", which is a budget by distance, not a physics. The shipping idea is a cast from the wound into the level, as this crate's
  spray is (`docs/research/spray-reach.md`); what none of the write-ups says is how the cloud is bounded, which here is a count and a flight time.

## Craft principle

**A burst is a spray with the cone removed.** The physics is one law, the percolation inversion: many small droplets leave fast, few large ones leave slow (`bloodstain::droplet`'s module docs, from Comiskey et
al.). A wound's cone aims that law along an axis; a burst aims it everywhere, at uniform solid angle. Everything else, what a droplet lands on, how it stains, what a body in the way catches, is what the crate
already does for a spray, a spurt and a hose. What is new is the **flight**: a cone's droplets are cast straight (`throw_spray`: "a droplet at 8-40 m/s falls under a centimetre in the metre it travels"), but
a burst's slow, large droplets are the ones that matter on the floor and on the body beneath, and they fall in a metre. The preset's `spatter_speed_scale` (0.25: 2 to 10 m/s) is the dial the floor spray flies at,
and at 5 m/s a droplet falls a metre and a half in two metres of flight under the shipped 18 m/s^2. So a burst flies **the parabola**, as the hose does (`hose_wall`, `flown`), and is caught by the first thing
its chords cross.

## How shipping games do it

By a cast from the wound into the level, bounded by a budget, and by decals. A burst there is a sprite effect with a stamped decal set where it lands. Here the stamp is a stain on the canvas of whatever it
landed on (a wall, a floor, a skin, a cloth), and which surface that is, is a cast through the mesh index: bodies catch it, and what is behind them is **shadowed**, which is the one thing a decal set does
not do and the first thing a forensic analyst looks for (a void in a spatter marks what stood in it).

## Model adopted

1. **`bloodstain::patterns::burst_spatter(at, count, salt, settings) -> Vec<Droplet>`**, a pure function. Directions uniform on the sphere (`z = 1 - 2u`, azimuth `2 pi v`); the size fraction `t = u^BURST_SIZE_BIAS`
   (this crate's own, 2.0: more small drops than large, which is the "fine mist with larger drops among it" of the sources), diameter `lerp(min, max, t)` and speed `lerp(forward, back, t) x spatter_speed_scale`:
   **the same law as `droplet()`**, asserted by a test over the pair. Seeded from the place (`site_seed(at, Impact)`) and `salt`; no clock.
2. **`GoreBurst`** (`#[non_exhaustive]`, a `Message`): `GoreBurst::new(at)` with `.with_radius(m)` (the source's radius; droplets leave at `at + dir x radius`, default 0.12) and `.with_droplets(n)` (default 960, capped at
   twice `max_droplets_per_wound`). A game that wants a body to come apart sends it **with** the `GoreHit` that breaks the body: the hit is the wound and the break, the burst is the air round them.
3. **`throw_burst`**, in `take_hits` after the hits of the tick: each droplet flies `p0 + v t - g t^2 / 2` in chords of `HOSE_CHORD_M` (0.2 m) until `fall_time` to the lowest floor or `HOSE_MAX_FLIGHT_S`, cast
   at **every dressed surface** through the mesh index (`nearest_landing`; a skinned body is posed once a batch, as a hit's spray poses it), and the nearest landing within the chord wins. **A back face within
   `2 x radius` of the source does not catch** (a droplet leaving the head it burst out of does not stain its own inside); a cloth's hole lets it through. The landing is stained by `hard_stain` from the speed and
   slant it arrives with and drawn by `paint_drop`, as a spray's is; a cloth takes the blob and a `Soaking` landing.
4. **Policy and cost.** A `GorePolicy` that draws no blood or no decals throws nothing. A tick with no burst pays nothing beyond the reader.

## Parameters

| name | value | status |
|---|---|---|
| `BURST_DROPLETS` (default count) | 960 (planned 320, then 480: see `## Outcome`) | this crate's own: nine tenths of `BURST_BUDGET` (2) x `max_droplets_per_wound`, the leaf's budget, so a burst is two wounds' worth |
| `BURST_SIZE_BIAS` | 2.0 | this crate's own: mean size fraction 1/3, mean diameter 2.5 mm against `droplet()`'s 3.4 mm |
| `BURST_RADIUS_M` (default) | 0.12 m | this crate's own: a head's radius |
| shell (`2 x radius`) | back faces within it do not catch | this crate's own |
| speeds, diameters | `droplet()`'s: 8-40 m/s paper span x `spatter_speed_scale`, 0.8-6 mm | Comiskey et al. 2018, through `bloodstain::droplet` |
| chord, max flight | `HOSE_CHORD_M`, `HOSE_MAX_FLIGHT_S` | the hose's own |

## Validation targets

1. **Pure function:** directions uniform on the sphere (the mean vector under 0.1 of the count, each octant within 30 % of the mean), the size-speed law of `droplet()` (every droplet on its line), a larger fraction
   of small drops than `droplet()`'s, same inputs same bits, another `salt` another cloud, zero count and non-finite place give nothing.
2. **Convergence** (the straight-line assumption turned round): the stains of a burst on an open wall, each traced back along its own flight's start direction, **centre on the burst's projection on the wall to
   within 15 cm** (the centroid of their positions).
3. **Shadow:** a body between the burst and the wall leaves a **void**: the wall's stain count in the body's shadow is under a quarter of an equal area outside it. A droplet that leaves the body it burst from
   is caught by the next part of it that stands in the way and stains it (the shoulders), not the wall behind.
4. **Spectrum on the wall:** stains from under 2 mm to over 6 mm across; **elongated stains point away from the centre** (the cosine of the angle between a stain's long axis and the radius from the burst's
   projection, over stains with an axis ratio of 1.3 or more, above 0.9 for four in five).
5. **Reproducible:** `capture_skinned --burst` twice, one digest; every existing recorder's digest the baseline's; `bench_carnage`'s digest unchanged and its median within 1 %.
6. **Cost:** one burst of 320 droplets in the scene of `--burst` (a floor, a wall and a body) under 3 ms of the schedule once; none on the ticks without.
7. Look: below.

## What great looks like

A wall three metres behind a standing body, hit by a head burst: a dense core of stains in a roughly circular fan of about a metre and a half, the stains at its edge fine and elongated away from the centre, a
few large round drops that fell short onto the floor and the shoulders; **a clean silhouette of the body in the spatter** where the body caught it; the floor round the feet freckled. Fresh bright arterial-venous
red (the canvases' own), no ring, no grid, no sphere of dots. Against the *Scanners* stills the fan is the right shape and the right size for the head, and the drops are wet, not paint.

## Reference set

Photographs and stills, kept under `/tmp/c18/refs/` only, cited by URL, deleted at cleanup; none is in the repository. Several results of the "exploding head" searches are AI-generated or illustrated (the fire-and-smoke
heads, the red vector heads) and were not used.

1. *Scanners* (1981), the exploding head: stills at `https://en.wikipedia.org/wiki/Scanners` and `https://www.monstersofmakeup.com/2021/02/26/dick-smith-a-guide-to-his-greatest-makeup-hits-part-1-of-2/`: a fan
   of red droplets and chunks, three to four head-widths across, densest at the head.
2. *Scanners*, the lecture-hall frame (`https://www.virtual-history.com/movie/film/27947/scanners/photographs`): the same effect with the audience for scale.
3. *Maniac* (1980), the car shotgun scene (`https://malevolentdark.com/maniac-1980-horror-with-street-cred/`): the aftermath, a car interior sprayed from the driver's seat.
4. Forensic radial back and forward spatter diagrams and photographs (`https://www.mdpi.com/2673-6756/1/2/9`, "radial pattern, thicker in the middle with drops on lines radiating outward", the misting panels on
   butcher paper): the pattern's structure.
5. The gunshot-misting poster above: a dense mist of stains under a millimetre, in a roughly circular field.

## Deviations

- **A burst is spatter spread over a sphere**, and the corpus has no source on a body bursting; the *Scanners* effect is a constructed one, not a physical one. The size-speed law and the spans are Comiskey's; the
  isotropy, the size bias and the count are this crate's own.
- **No chunks, no strings, no airborne mist.** The film's burst is half pieces; the crate's break throws the pieces, and what hangs in the air is a particle effect (`vfx` does not build on Bevy 0.20). This is the
  stains the burst leaves.
- **Walls, floors, skins and cloths are all caught**, and there is no drag: the speed dial stands in for it, as everywhere in the preset.
- **A burst is not a wound.** It opens no crater and breaks nothing: the hit that does is the game's to send alongside, which is also how the guts spill and the pieces fly.

## Pending index

- The Ledo et al. poster was read from its PDF (a few lines); the full paper is `doi` unknown to me and was not obtained (MDPI refused the fetch). Wikipedia's *Scanners* page was read for the construction of the
  effect. None is in the corpus.
- **To obtain:** a source on explosive cranial injuries (DiMaio and DiMaio, *Gunshot Wounds*, a book the crate already cites for exits) and any measurement of the droplet size distribution of a burst rather than a
  bullet.

## Outcome

Shipped in 0.8.12. **What was built** is the plan's three parts, plus one the plan did not have (landings are deferred by their flight time) and two numbers that moved.

**Rounds (four).**

1. Cast at once, drawn at the generator's own size (0.8-6 mm), a wall 1.2 m behind the head: 190 of 480 droplets landed on the 3 m slab, a sparse scatter of specks, dust at the camera's 5 mm a pixel. Not the effect.
2. `BURST_DROP_SCALE` 3.0 (the hose's reason: gouts, not a bullet's mist, a cloth's blob unscaled), the wall 0.45 m behind the head and a left wall and ceiling (`spawn_corner`): a head burst reads. But drawing 190
   stains on the tick they were thrown cost 9.4 ms (about 50 us a stain, up to 350 us on a skin), a frame the player would see.
3. **Deferral**: each landing queued on its surface (`InFlight`) and drawn on `tick + ceil(t x hz)`, the fine fast drops first and the large slow ones up to three quarters of a second later, at most `BURST_LANDINGS_PER_TICK` (24)
   a surface a tick. The budget is per surface and not global so that no query order can change a result. Worst tick 1.7 ms (480 droplets).
4. Density (the *Maniac* car interior is drenched; round 3's 480 droplets were a sparse spray beside it): `BURST_BUDGET` 2, the default 960 (`BURST_DROPLETS`), 630-740 stains landing over 44 ticks (0.2-2.5 ms a tick, 14-22 ms in all).

**Critique, written before the verdict** (a side-by-side of the *Scanners* stills over 0.8.11's frame and this release's, same shot, frame 359, made under `/tmp` and not kept: the stills are copyrighted).

- What is right: a dense core of stains round the head on the wall behind it, thinning to fine specks at about a metre and a half, the right size and shape for the head; a clean void where the torso stood
  between the burst and the wall (the shoulders and neck stained instead); the corner, the ceiling and the sheet beside it take their share; large stains fell short and low, fine ones went far and high; and the cloud
  arrives over three quarters of a second (the ceiling and the fast fine drops first), where the 0.8.11 frame has the head and nothing else.
- What is not: **it is the residue of a cloud and not the cloud**: *Scanners*' frame is airborne, orange and bright, three head-widths of ligaments and chunks; here nothing hangs in the air, and there are no strings. The
  stains are the canvases' dark crimson, a mean sRGB of `(116, 49, 50)` over the red pixels of frame 359's wall, body and sheet (a mask of `r > 90, r > g + 45, r > b + 35`), against `(158, 59, 36)` over the *Scanners*
  still's. **Coverage:** 8.7 % of that crop is stained (0.14 % with `--no-cloud`), against 20 % of the *Scanners* still's frame; *Maniac*'s dark stains missed the mask, so **it is not measured**, and by eye it is drenched where
  this is a spray. `.with_droplets` to the cap gets nearer, and the game's dials move it. Most stains are round gouts, because a droplet arriving near the normal leaves a round mark, and few are elongated; **I did not
  measure that** (target 4's second half). The ceiling reads dark in the demo, which has no light facing down [INFERENCE: not tested with one].

**Verdict: GREAT for the scope claimed**: the room round a burst body is stained, by the one size-speed law the crate already has, with the body shadowing what is behind it, arriving over time at a bounded cost a tick.
It is **not** claimed to be the film's airborne cloud: that is a particle effect and `vfx` does not build on Bevy 0.20, and the chunks are the break's.

**Numbers against the targets.**

| target | measured |
|---|---|
| 1 pure function | directions: each octant within 30 % of the mean and the mean vector under 0.1 (`a_burst_goes_every_way_at_once`); every droplet on `droplet()`'s line and a smaller mean diameter (`a_burst_flies_on_the_spatter_law_with_more_fine_drops`); same bits, another salt another cloud, count 0 and a NaN place empty (`a_burst_is_a_place_and_a_salt`) |
| 2 convergence | **replaced.** At the shipped `spatter_speed_scale` (0.25) a droplet leaves at 2-10 m/s, so over the metre to a wall it falls from 9 cm to more than a metre (`0.5 x 18 x t^2`, `t` = 0.1 to 0.5 s): the cloud's centre of mass on a wall is more than 10 cm lower under gravity 18 than under none (`a_burst_falls_as_it_flies`). The stains do not centre on the burst's projection, and the test is that they fall, which the straight-line reading would deny |
| 3 shadow | the wall behind a body 1.2 m across took **0.050** of what an open wall took (target under 0.25, the test's bound; the box took 0.056 m2); a sphere the burst starts inside takes nothing and lets at least 4/5 of the cloud through (that test's bound: `a_burst_leaves_the_body_it_burst_from_by_its_inside`) |
| 4 spectrum | 177 stains on a wall 1 m from the burst, 2 to 29 texels across (a texel is 5.9 mm there; median 5, p10 2, p90 16), drawn at 3 times the generator's 0.8-6 mm. **Elongation: not measured** |
| 5 reproducible | `capture_skinned --burst` `a3d602d1a8574a36` (flay `d2c76ae8b606a8a1`, dermis `10bd4f5f69331f9d`), `--burst --break` `d57e6842ae3de1af`, `--burst --no-cloud` `9bb7d6b61b621f75`, each twice equal; all 15 other recorder variants equal to 0.8.11's, twice each; `bench_carnage` digest `0x4366a8b64352c85a` unchanged, 0.09258 to 0.09314 ms a frame (+0.6 %, medians of three, load about 4-5), p99 2.645 to 2.674 ms |
| 6 cost | the planned "320 droplets under 3 ms once" was wrong in kind: the cast tick is 0.5-0.8 ms for 960 (a test room), and the painting is spread: 0.2-2.5 ms a tick over 44 ticks in `--burst`. A tick with no burst and nothing in the air pays the reader |

**Five mutations were each caught** and restored: `BURST_SHELL` 0 (the inside test), a skin that lets the burst through (the shadow test), gravity 0 in the throw (the fall test), `BURST_SIZE_BIAS` 1.0 (the fine-drops test),
and a hemisphere in place of a sphere (the isotropy test).

**Deviations from the plan.** Default count 320 became 960 and the cap went from `max_droplets_per_wound` to twice it (`BURST_BUDGET`); the landings are deferred (`InFlight`, `land_bursts`); the builders are `with_radius`
and `with_droplets`; the drop scale is 3.0; the demo is a corner (a wall 0.45 m behind the head, a left wall, a ceiling) because a wall 1.2 m off gave round 1's sparse scatter.

**Found, not fixed:** the preset's cut-film hides the muscle strip (iteration 17's finding, still the item above a burst in value); a burst does not curve; a cloud in the air; the droplets' colour is a scene dial the
canvas does not carry (no fluid id).
