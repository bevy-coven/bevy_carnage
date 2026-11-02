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

1. **The pulse is the crate's own.** A wound spurts on `bloodstain::bleed::pulses_on` (an integer modulo of the age, a phase per wound from its seed, `spurt_bpm` 96 at 60 Hz: one beat every 38 ticks) while
   `rheo::flows` says the blood still moves (full for `spurt_ticks` 210, tapering to nothing at `clot_ticks` 360: six seconds, nine or ten beats), and with the pressure `1 - age / pressure_decay_ticks` (600) that `arterial_arc` already computes. No new stopping rule; no new dial.
2. **The droplets are `arterial_arc`'s**: `BloodSettings::arc_stains` (7) of them at `ARTERIAL_DROPLET_M`, spaced and jittered across `ARTERIAL_ARC_DEG` (22 degrees either way) in the plane of the wound's normal and
   the axis nearest up, at `ARTERIAL_SPEED x pressure x along`. The preset launches them at `spatter_speed_scale` 1.0, as `throw_spray` launches the ones a wall or a body catches: the 0.25 that stands in for air drag
   on a 40 m/s bullet spatter would make an 8 m/s jet a two-metre-an-second drip.
3. **The flight is ballistic**, `x(t) = p0 + v t - g t² / 2` at `BloodSettings::gravity`, because a jet at 8 m/s falls 0.2 m in the first metre (`throw_spray` casts straight "since a droplet at 8 to 40 m/s falls under a
   centimetre in the metre it travels", which is true there and false here). It is walked as a polyline of chords no longer than `HOSE_CHORD_M` down to the lowest floor's plane; a chord is cast at the walls whose box it crosses, through
   the mesh index `take_hits` keeps (now a resource both systems share), and the first landing wins. A droplet that reaches a floor's plane with nothing in the way lands where that parabola crosses it, and the floor's own cast down finds the texel.
4. **The stain is `hard_stain`'s**, drawn by `paint_drop` at the surface's resolution, from the droplet **as it arrives** (its speed and direction at the landing time, not at the launch), so a drop that has fallen a
   metre strikes a wall steeper and faster than it left. **A large stain on a wall is given its run**: a stamp `HOSE_RUN_WIDTH_M` wide and `HOSE_RUN_STAINS` stain-lengths long (a half to one and a half of that by the droplet's seed),
   drawn down from the stain's lower edge, because the wetmap's drip pass sheds a texel's excess one texel a tick and a byte of coverage cannot hold the volume a long run is fed from; the bleed's wells are drawn the same way. **Beside each
   large droplet fall `HOSE_SATELLITES` fine ones**, 0.12 to 0.30 of its diameter, within a few degrees of its direction: "it exits as a column of blood and separates into drops".
5. **The wound's place and axis follow the body.** The place is the anchor the wound already keeps (`WoundSite::skin`, or `at_local` on a rigid mesh). **The axis is the anchor triangle's winding normal as the pose draws it
   now**, signed to agree with the wound's normal at the hit, so a waving forearm sweeps the arc across the wall and a turned head turns the jet. The wound remembers the triangle and the sign in a new component,
   `Spurts`, beside `Wounds`, because `WoundSite` is a public struct whose literal a field would break.
6. **Walls and floors only** (`Gore::wall`, `Gore::floor`, rigid): a jet through another body or a cloth is not cast (a body in the way stops `throw_spray`'s straight droplets; here it would need each body posed
   for each chord). Named under Deviations. A tier that draws no blood or no decals throws nothing.

## Parameters

Unchanged: everything `arterial_arc` and `BloodSettings` carry. New, each **this crate's own and tuned against the references**: `HOSE_CHORD_M` 0.2 m (a chord is shorter than the thinnest wall the broad phase
boxes, and a droplet crosses a metre in five), `HOSE_MAX_CHORDS` 24 (a flight past 4.8 m of arc is not walked), `HOSE_MAX_FLIGHT_S` 1.5, **`HOSE_DROP_SCALE` 3.5** (the stain is drawn for a droplet 3.5 times the diameter
`arterial_arc` throws: at 1.0 the first render was a column of 12 mm dots, polka dots, against stains the size of a palm in F2 and F3; 3.5 is 17 mm, and the stains several centimetres across [INFERENCE: read off the wall's width in pixels]), `HOSE_RUN_STAINS` 3, `HOSE_RUN_MAX_M` 0.5, `HOSE_RUN_WIDTH_M` 0.006 (`docs/research/rivulets.md`),
`HOSE_SATELLITES` 3 and `HOSE_SATELLITE_CONE` 0.1. No `GoreDials`, `BloodSettings` or `WoundSite` field is added.

## Validation targets

- **The arc is an arc**: the stain a droplet leaves on a wall is `x(t)`, not the straight line. For a launch of `v` at angle `a` from a wall at distance `d`, the landing's height is `y0 + d tan a - g d² / (2 v² cos² a)`
  to within 2 cm (a unit test with a flat wall and one droplet). A straight cast would be 0.3 m off at 1.5 m.
- **One arc a beat, and the pressure falls**: no droplet lands on a tick the heart does not beat on; the reach of the beats falls over `pressure_decay_ticks`; after the clot nothing is thrown.
- **A vein does not spurt**, a tier that draws no blood throws nothing, `GoreReset` of the body ends it.
- **Reproducible**: two runs of `--hose` print one digest; **every existing digest equals 0.8.9's** (nothing else uses `.arterial()` but `carnage_web`); `bench_carnage` within 5 %.
- **Cost**: a beat tick of one arterial wound against a room of a floor and three walls, against a tick that is not a beat: about a tenth of a millisecond more, and nothing on a tick that is not a beat (the system does not run: it is `run_if(any_with_component::<Spurts>)`).
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
- **A droplet is drawn 3.5 times the diameter `arterial_arc` gives it** (`HOSE_DROP_SCALE`): the generator's `ARTERIAL_DROPLET_M` is 5 mm "large and discrete", and at that size a stain is a 12 mm dot where the references' are the size of a palm ([INFERENCE] a large
  artery's bolus of ten millilitres a beat, split seven ways, is drops of a centimetre and a half). The generator is untouched and its goldens with it; the scale is the preset's.
- **A run is drawn, not simulated** (above): the canvas has no volume to run on.
- **The wound's pressure is the crate's `1 - age / pressure_decay_ticks`**, and the wound clots at `clot_ticks` (six seconds), so the last beat is at 0.4 of the pressure and the hose is about nine beats long.

## Outcome

Six render rounds of `capture_skinned --hose` (the victim stands left of a wall, turns about her axis, and takes one arterial hit at frame 20 to the back of the waving forearm), each changing one thing.
R1: the droplets as `arterial_arc` makes them, a column of 12 mm dots down the wall beside the arm. R2: 3.5 times the size: a column of blobs that merged into a curtain, and the same column every beat ([INFERENCE] because the arm waves
in a plane the wound's normal lies in, so its axis did not move and the jet always pointed at the wall's middle). R3: the body turns, and each beat's fan lands in its own column across the wall (a trace of where each beat landed showed it: tick
61 at `u` 0.50, 99 at 0.49, 137 at 0.56, 175 at 0.79, and from tick 213 the jet pointing along the wall and out over the floor's edge). R4: a run under each stain; the camera moved to see them. R5: three fine drops beside
each large one, the victim moved left so the wall to her right is open, and the set's sheet removed. R6 is the clip and the comparison below. A trace line printed from `spurt` was removed before the commit.

**Measures**, `smear_stat`'s mask (reddish pixels) and principal axis, on the wall region of frames 250 and 340, against F2 and F3 by the same script: the largest connected structures lie at **88, 89, 90, 93 degrees (frame 250) and 90, 91, 92
(frame 340) from the horizontal**, length over width 7.8, 4.2, 8.7, 3.7 and 8.5, 3.0, 3.2; F3's at 80, 83, 94 (9.0, 3.3, 2.1), F2's at 77, 77, 83, 97, 105, 108. Ours are within 3 degrees of vertical, the references' within 10 (F3) and 18 (F2). Mean mask colour sRGB `(123, 60, 60)`
against F3's `(141, 71, 42)` and F2's `(141, 94, 80)`: the same red to green ratio as F3 (2.05 against 1.99), darker (luma 79 against 89 and 106), and F3's is more orange (blue 42 against ours 60); the references are photographs of stains of unstated age
under a flash, and the crate's colour is the spectral model's (`bloodstain::spectral`) at its age: not tuned here.

**Critique, against F2, F3 and M2.** Against F3 (BLOODY2): the thick streak that narrows into a run that ends in a bead, the near-vertical runs, and the stipple of small drops beside the streaks are there, and a close-up of the middle column
(`docs/media/hose.png`, frame 340) reads as that image's right-hand streaks. Against F2 (the training photograph): **missed, the arcs**: F2's stains lie along curves across the wall, dozens of them, close, and ours are separate columns a beat
apart because each beat's droplets leave on one tick and the fan is vertical (`arterial_arc`'s arc plane); a body that turns fast enough sweeps a curve, and the mannequin's turn is slow. Against M2 (Sweeney Todd): the throat's
jet is visible in the air as a line of drops and the crate draws only where it lands. **Missed, the gloss**: the stains are a flat maroon on a matte wall, as the stains of the crate's other demos are under their flat ambient and two suns. **Missed, the density**:
nine beats of seven stains are sparse next to F2; a game that wants more raises `BloodSettings::arc_stains`. A stain's edge is soft and lobed where F2's are scalloped smooth.

**Cost**, on a throwaway test (removed): one wound beating in a room of a 6 m floor and three walls, 400 ticks of the whole schedule: a beat tick **215 / 212 / 251 microseconds** median against **109 / 106 / 142** for a tick that is not a beat (p99 beat 269 / 245 / 367, quiet 127 / 129 / 374; three runs, load 5 to 9): about
0.1 ms a beat, on the 28 shots (7 large and 21 fine) each walking up to 24 chords against three walls. `bench_carnage` (no wound in it spurts; the plugin now runs one more system, behind a condition that is false, and `take_hits` reads the mesh index as a resource and not a local): before, six runs at
load 3: `carnage_frame_ms` median 0.09360, `carnage_p99_ms` 2.672; after, at rest (load 5.2 to 5.6): 0.09437 and 2.676 (**+0.8 %, +0.1 %**). Four runs straight after a recorder sweep (load 6.5) read 0.0978 to 0.0990 and 2.80 to 2.91; the
same binary a minute later read 0.0930 to 0.0963. Digest `0x4366a8b64352c85a` unchanged.

**Reproducibility.** Every existing digest equals 0.8.9's in two runs (preset, every skinned variant, entrails, flesh, and `capture_gore`'s four); `--hose` prints `digest=0c903766bf51f0f2` twice, `--hose --vein` (the same hit, venous: the 0.8.9 behaviour) `38099dce0d339ebc`.

**Mutations the tests caught:** no gravity in the flight (`a_jet_lands_where_its_parabola_does_and_not_where_a_straight_line_would`); the axis kept as the wound opened (`a_jet_follows_the_body_as_it_turns`).

**Verdict: GREAT** for the mechanism and the stain vocabulary: an arterial wound throws a ballistic arc a heartbeat, at the walls and the floor, from where it is and the way it faces now, with its stains' runs and stipple, ending at the clot, for no
cost on a tick without a beat. **NOT YET**, and named for the next pass that touches it: the curve of F2 (a jet that sweeps within a beat), the gloss on a wet wall (`flesh.wesl`, a WebGL2 re-check), and a jet that meets a body or a cloth.

## Pending index

- No home-still document on arterial jets, spurt patterns or the pressure at a breached artery (three `distill_search` queries, four `paper_search`); the Korean spurt-against-swing study
  (`10.53051/ksfs.2024.25.2.5`) was not obtained.
- The two film references are a gag and a stage production; no still of an arterial pattern painting a wall in a feature film was gathered.
- The technique sources are a forum answer and an interview.
