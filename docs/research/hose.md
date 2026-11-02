# The hose: an arterial wound throws an arc a heartbeat

Iteration 16 (2026-11-01). Item 10 of the owner's 80s-horror list: "A blood hose. A sustained, high-volume arterial jet that paints a wall in arcs (*Evil Dead II*). The CPU pulse and the arterial pattern class exist,
but the visible jet is vfx-only today." Why this and not the runner-ups (a wound-interior texture through `texel_at`; a stump with a protruding bone; a non-red fluid; a flayed body): **every part of it is already in the
crate and no caller reaches it.** `bloodstain::patterns::arterial_arc` ("one arc of large discrete droplets per systole, reach falling as the body loses pressure") has its generator, its tests and its documentation, and
**nothing but those tests calls it**; `GoreHit::arterial()` sets `so2` and the preset reads `so2` in exactly one place, `announce`, to put `PatternClass::ArterialSpurt` on the `Wounded` message a particle effect might
listen to (`vfx` cannot build on Bevy 0.20). A game that writes `.arterial()` gets a wound that bleeds exactly as a venous one does: wells on the skin, one drip a systole straight down. The one thing an artery does that
a vein does not, the reason a horror film has the shot, is missing. The runner-ups need either a shader change and the WebGL2 re-check this disk cannot hold (3 GB free), or a model the note's own sources do not yet carry.

## What was measured before any change

- `grep arterial_arc src/` finds the generator and tests; no caller. `grep so2 src/preset.rs` finds `announce` and the `GoreHit` builders: an arterial wound is a venous wound with a different message.
- `capture_skinned --hose` (this note's scene, added with it: an arterial hit to the back of the raised, waving left forearm, a wall a metre and a quarter behind the body), run on 0.8.9: `digest=0895c30c84933635
  flay=03f50f9d79290199`. Frames 30, 90, 150 and 300: **a few pinpricks of the hit's own spray on the wall beside the arm, a small pool on the floor, nothing else.** No arc, at any frame.
- `bench_carnage` on 0.8.9, four runs, load 3: `carnage_frame_ms` 0.09423, 0.09376, 0.09314, 0.09343 (median 0.09360), `carnage_p99_ms` 2.698, 2.672, 2.644, 2.672 (2.672); digest `0x4366a8b64352c85a`.

## Sources

- **Forensic science** (what an arterial pattern is). SWGSTAIN, *Recommended Terminology*, Forensic Science Communications 11(2), April 2009 (`https://theiai.org/docs/SWGSTAIN_Terminology.pdf`): "**Projected Pattern** A
  bloodstain pattern resulting from the ejection of a volume of blood under pressure." "**Directionality** The characteristic of a bloodstain that indicates the direction blood was moving at the time of deposition."
  "**Flow Pattern** A bloodstain pattern resulting from the movement of a volume of blood on a surface due to gravity or movement of the target." Forensic Pocket Guide, *Processing Bloodstains: Projected Stains*
  (`https://forensicpocketguide.com/processing-bloodstains-projected-stains/`): "Arterial spurt patterns occur when blood exits from a pressurized, breached arterial vessel, it exits as a column of blood and separates
  into drops as the victim moves around. This type of pattern may continue to form as long as the heart continues to beat. When the heart stops, the pressure drops and the blood is no longer forced out in a projected
  pattern." A teaching slide (`https://www.slideserve.com/aideen/blood`): "Blood exiting body under arterial pressure (120 mm Hg); corresponds to heart
  beating", "Large stains with downward flow on vertical surfaces", "Creates a wave-like appearance".
- **Physics** (how fast a jet leaves). Torricelli's law (`https://en.wikipedia.org/wiki/Torricelli's_law`): "The law states that the speed of efflux of a fluid through a sharp-edged hole in the wall of the tank filled to
  a height h above the hole is the same as the speed that a body would acquire in falling freely from a height h", that is `v = sqrt(2 P / rho)` for a pressure `P`. **My own arithmetic, not a quoted figure:** 120 mmHg is
  16.0 kPa and blood is about 1 060 kg/m³, so `v = sqrt(2 x 16 000 / 1 060) = 5.5 m/s`; a vessel's resistance and the hole's coefficient only lower it. The crate's `ARTERIAL_SPEED` is 8 m/s (see Deviations).
  **The corpus has nothing on arterial jets**: three `distill_search` queries returned the forward-spatter paper already cited (`10.1103/PhysRevFluids.3.063901`) and unrelated books, and `paper_search` found
  forensic textbooks and one Korean study of spurt against swing cast-off (`10.53051/ksfs.2024.25.2.5`) that I did not obtain; recorded under Pending index.
- **Technique** (how games do it). Epic forums, *Best way to make blood effects?*, post 6 (`https://forums.unrealengine.com/t/best-way-to-make-blood-effects/1562`): "The particles accelerate downward over their
  life AND scale vertically over their life, which gives the impression of fluidity (since a ball of liquid becomes elongates as it falls)." "These particles are set to collide, and they generate events on collision.
  Those events' dispatcher (OnParticleCollide) is bound to another event in BP, which spawns decals at the location of the impact". Garcia-Obledo on *Assassin's Creed 3 Remastered*, 80.lv
  (`https://80.lv/articles/001agt-behind-the-scenes-blood-in-assassins-creed`): "I ended up having 4 emitters: thick drops, smaller drops, core, and spray." **Both are forum and interview practice, not a talk or a
  paper**; both draw the jet as particles and the stain as a decal at the particle's collision. This crate cannot draw the particles (`vfx` does not build on Bevy 0.20) and draws the stain.

## Craft principle

An artery is a pump with a hole in it: **one bolus a heartbeat, fired along the hole's axis at the speed the pressure gives, dropping under gravity as it goes, landing where the parabola meets a wall or the floor.** What
a viewer reads in the frame after is *the pattern*, not the jet: a row of large stains along a curve, each drawn out the way the blood was travelling, each with its own run down the wall, the whole thing wave-shaped
because the pressure rises and falls and the victim moves. The jet in flight is a thin red line for a fifth of a second; the pattern is what stays.

## How shipping games do it

Particles and decals: a drop emitter that collides, and on collision spawns a decal of a random splatter texture, the drop stretched along its fall so it reads as a liquid (quotes above). The stain is a sticker
at the place the particle died; its size, its direction and its run are the artist's, and the pulse is a timer on the emitter. This crate's stains are not stickers: a droplet lands as the stain it makes
(`hard_stain`: from its size, its speed and the angle it struck at, drawn out along the way it was going), is wet blood on a canvas, and runs under the wetmap's own passes. What the crate lacks is only the *throw*.

## Model adopted

1. **The pulse is the crate's own.** A wound spurts on `bloodstain::bleed::pulses_on` (an integer modulo of the age, a phase per wound from its seed, 72 bpm at 60 Hz: one beat every 50 ticks) while
   `rheo::flows` says the blood still moves, and with the pressure `1 - age / pressure_decay_ticks` that `arterial_arc` already computes. No new stopping rule; no new dial.
2. **The droplets are `arterial_arc`'s**: `BloodSettings::arc_stains` (7) of them at `ARTERIAL_DROPLET_M`, spaced and jittered across `ARTERIAL_ARC_DEG` (22 degrees either way) in the plane of the wound's normal and
   the axis nearest up, at `ARTERIAL_SPEED x pressure x along`. The preset launches them at `spatter_speed_scale` 1.0, as `throw_spray` launches the ones a wall or a body catches: the 0.25 that stands in for air drag
   on a 40 m/s bullet spatter would make an 8 m/s jet a two-metre-an-second drip.
3. **The flight is ballistic**, `x(t) = p0 + v t + g t² / 2` at `BloodSettings::gravity`, because a jet at 8 m/s falls 0.2 m in the first metre (`throw_spray` casts straight "since a droplet at 8 to 40 m/s falls under a
   centimetre in the metre it travels", which is true there and false here). It is walked as a polyline of chords no longer than `HOSE_CHORD_M`; a chord is cast at the walls and floors the box of which it crosses, through
   the mesh index, and the first landing wins. A droplet that reaches the floor's plane with nothing in the way lands by the closed form (`crate::bloodstain::landing`), as a hit's floor spray does.
4. **The stain is `hard_stain`'s**, drawn by `paint_drop` at the surface's resolution, from the droplet **as it arrives** (its speed and direction at the landing time, not at the launch), so a drop that has fallen a
   metre strikes a wall steeper and faster than it left and an arc's far stains are drawn out downward. The wetmap does the rest: a wall's large stain runs.
5. **The wound's place and axis follow the body.** The place is the anchor the wound already keeps (`WoundSite::skin`, or `at_local` on a rigid mesh). **The axis is the anchor triangle's winding normal as the pose draws it
   now**, signed to agree with the wound's normal at the hit, so a waving forearm sweeps the arc across the wall and a turned head turns the jet. The wound remembers the triangle and the sign in a new component,
   `Spurts`, beside `Wounds`, because `WoundSite` is a public struct whose literal a field would break.
6. **Walls and floors only** (`Gore::wall`, `Gore::floor`, rigid): a jet through another body or a cloth is not cast (a body in the way stops `throw_spray`'s straight droplets; here it would need each body posed
   for each chord). Named under Deviations. A tier that draws no blood or no decals throws nothing.

## Parameters

Unchanged: everything `arterial_arc` and `BloodSettings` carry. New, each **this crate's own and tuned against the references**: `HOSE_CHORD_M` 0.2 m (a chord is shorter than the thinnest wall the broad phase
boxes, and a droplet crosses a metre in five), `HOSE_MAX_CHORDS` 24 (a flight past 4.8 m of arc is not walked), and a droplet scale for the stain's size if the first renders show it (the references' stains are
large, several centimetres, and a 5 mm droplet at 8 m/s stains less than that: [INFERENCE] about 2 cm): decided from the render and recorded here. No `GoreDials` or `BloodSettings` field is added.

## Validation targets

- **The arc is an arc**: the stain a droplet leaves on a wall is `x(t)`, not the straight line. For a launch of `v` at angle `a` from a wall at distance `d`, the landing's height is `y0 + d tan a - g d² / (2 v² cos² a)`
  to within 2 cm (a unit test with a flat wall and one droplet). A straight cast would be 0.3 m off at 1.5 m.
- **One arc a beat, and the pressure falls**: no droplet lands on a tick the heart does not beat on; the reach of the beats falls over `pressure_decay_ticks`; after the clot nothing is thrown.
- **A vein does not spurt**, a tier that draws no blood throws nothing, `GoreReset` of the body ends it.
- **Reproducible**: two runs of `--hose` print one digest; **every existing digest equals 0.8.9's** (nothing else uses `.arterial()` but `carnage_web`); `bench_carnage` within 5 %.
- **Cost**: a pulse tick of one arterial wound against a room of floor and three walls, median and p99, in tens of microseconds, and nothing on a tick that is not a beat.
- **The render, against the references**: seven or more stains a beat on the wall, elongated along the way they travelled (length over width at least 1.5), with runs below them that are near vertical (six of the eight
  largest connected structures of F2 lie 76 to 108 degrees from the horizontal, measured by principal axis on a reddish mask; the other two are merged chains), stains of 2 to 8 cm, and a pattern that is a curve and
  not a line: successive beats at different heights or places as the arm swings and the pressure falls.

## What great looks like

Beside F2 and F3: **a wave of large, tall stains along an arc on the wall behind the victim, each with its run, and the pulses told apart by where the arm was and how hard the heart pushed**, a stipple of
small drops along the same curve where the column broke up, the whole thing wet and red and still running when the beat has gone. Beside M1 (the Black Knight): a long parabola, from a wound on a limb, that reaches the
wall. Seen from the distance of a player: *something is pumping blood out of that arm*.

## Reference set

Stills and photographs, copyrighted by their publishers, kept under `/tmp/c16/refs/` and **never committed**; this note cites their pages. The film stills are two: Bing's image results for *Evil Dead II*, *Kill Bill*,
*Dead Alive* and *Sleepy Hollow* were posters, aftermath and tearsheets, not a spurt in frame, and are not used.

| Ref | Source | Page | What it shows that the crate must reproduce |
|---|---|---|---|
| F2 | *Arterial Spurt Pattern* (forensic training photograph, on a teaching slide) | `https://www.slideserve.com/ashton-rojas/blood` | two arcs on a white wall of large oblong stains, each tilted along the arc and each with long runs below it, the whole wave-shaped; six of the eight largest connected structures lie 76 to 108 degrees from the horizontal (a principal-axis measure on a reddish mask; two are merged chains) |
| F3 | *Arterial Spurt* (BLOODY2.com photograph in a slide) | `https://www.slideserve.com/chipo/unit-5-blood` | thick continuous streaks where the column struck, near-vertical runs under each, and a stipple of small round drops along the curve where it broke up |
| F4 | the slide's text and diagram | `https://www.slideserve.com/aideen/blood` | "120 mm Hg; corresponds to heart beating", "Large stains with downward flow on vertical surfaces", "Creates a wave-like appearance": stains of rising and falling size along a row |
| M1 | *Monty Python and the Holy Grail* (1975), the Black Knight, on a meme page | `https://imgflip.com/m/fun?sort=latest&after=72eg4p` (image `https://i.imgflip.com/72efxg.jpg`) | a thin, continuous red parabola leaving the knight's shoulder stump and curving down: the jet in flight, where the crate draws only its landing |
| M2 | *Sweeney Todd*, a Curve, Leicester production still | `https://www.curveonline.co.uk/whats-on/shows/sweeney-todd/` (image `https://cdn.curveonline.co.uk/wordpress/uploads/2013/07/show_sweeney-todd_prd-06.jpg`) | a throat's jet that has already separated into separate drops along an arc, and the apron's stains, elongated along the throw |

## Deviations

- **`ARTERIAL_SPEED` is 8 m/s, and Torricelli at 120 mmHg gives 5.5.** The constant is the crate's, "sized against the reach it has to produce" under a shipped 18 m/s² gravity (`patterns.rs`), not a measurement; the hose
  keeps it, since the pattern the references show reaches a wall two to three metres away and 5.5 m/s under 18 m/s² carries 1.7 m at 45 degrees. A caller who wants the physical number sets `BloodSettings` as before.
- **Walls and floors only.** A jet that meets a body, a cloth or a skinned surface passes through it; `throw_spray`'s droplets are caught by them. A body across the jet is the next item.
- **The jet in flight is not drawn** (M1). The stain appears on the beat, at the time the heart pushed, not after the flight's two tenths of a second; with no particles to see in the air a delay would read as lag.
- **No new `GoreDials`, `BloodSettings` or `WoundSite` field**: the wound's spurt state is a new component, `Spurts`, so the change is additive.
- **`GoreHit::arterial()` now does what its name says**, which moves nothing that does not use it; `carnage_web` is the one example that does, and has no digest.
- **A cut face does not spurt.** The preset throws every piece of a broken body, and a piece with no heart behind it has no pressure: the body side of a stump spurts, and there is no body side yet.

## Pending index

- No home-still document on arterial jets, spurt patterns or the pressure at a breached artery (three `distill_search` queries, four `paper_search`); the Korean spurt-against-swing study
  (`10.53051/ksfs.2024.25.2.5`) was not obtained.
- The two film references are a gag and a stage production; no still of an arterial pattern painting a wall in a feature film was gathered.
- The technique sources are a forum answer and an interview.
