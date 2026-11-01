# A thread of slime between two surfaces stretches, thins, and snaps

Iteration 13 (2026-11-01). Item: the strings. Ranked first by the owner's run instruction (the 80s practical-effects look, "start your way through"), by the iteration 12 backlog
(`docs/loop-log.md`: item 3, "slime strings", the *Thing* and *Hellraiser* signature) and by what iteration 12 measured: the guts are bloodied now, but a gut that leaves a
wound **leaves it clean**: nothing joins the body and what fell out of it, where Ref D1 (*Day of the Dead*) is a *web* of taut slick strands between the abdomen and the hands, and
Ref T3 (*The Thing*, Palmer) is dozens of thin clear threads hanging from a ceiling, each ending in a bead. Runner-ups, kept for the log: the heap and the web as *more strands*
(a cost and a dial default, not a look), a glint (a `flesh.wesl` change and a 79-minute WebGL2 re-check, with 6 GB of disk), the wound interior and a real skin lip (the same shader
cost), a burst hit (*Scanners*: it moves digests). The strings are cosmetic, outside every digest, and need no shader.

## What was measured before any change

`capture_preset`, frames 124 to 140 on 0.8.6 (the guts hang from the torso and fall): there is **no connection** between the wound and any strand; a strand is a tube that is, one
tick, at the wound and, the next, a tube beside it. The viscera solver pins every fourth node to the wound (`Mesentery` anchors, `GoreDials::strands` of them) and those pins
tear at a strain of 0.35, so the link is a pin of 12 mm, never a thread. In the stills, Ref T3 measured off the frame (1 000 px wide, the face about 280 px across and taken as
15 cm, so a pixel is about 0.54 mm): the clear threads are 3 to 5 px, **about 2 to 3 mm across**, each ending in a bead at its lower end; Ref D1's strands are red, slick, taut and
a few millimetres across, many of them, running from the cut edge to the hands.

## Sources

- **Physics.** A viscous bridge between two surfaces pulled apart thins at the neck and pinches off.
  - Weickgenannt, Roisman & Tropea, *Pinch-off of a stretching viscous filament and drop transport*, New J. Phys. 17 (2015) `083059`, `doi:10.1088/1367-2630/17/8/083059`: "At high enough
    acceleration the evolution of the midpoint diameter of the bridge is universal; it depends neither on the acceleration nor on the liquid viscosity. Moreover, at high acceleration rates even the
    breakup time does not depend on the acceleration, but is determined solely by the liquid viscosity."
  - Liao, Franses & Basaran, *Deformation and breakup of a stretching liquid bridge covered with an insoluble surfactant monolayer*, Phys. Fluids 18 (2006) `022101`, `doi:10.1063/1.2166657`: "The bridge
    lengths are 3.15 mm initially and about 4–7 mm at breakup, which occurs after stretching for about 0.1–0.2 s, depending on the volume and viscosity of the liquid and the surface density of spread
    monolayers." (A Newtonian, millimetre bridge: the order of magnitude, not slime.)
  - Qian & Breuer, *The motion, stability and breakup of a stretching liquid bridge with a receding contact line*, J. Fluid Mech. (2011), `doi:10.1017/s0022112010004611`: "at a critical radius,
    the bridge becomes unstable ... The bridge subsequently pinches off, and a small droplet is left on the substrate." And Lister & Stone, *Capillary breakup of a viscous thread surrounded by another
    viscous fluid*, Phys. Fluids 10 (1998), `doi:10.1063/1.869799`: "both axial and radial scales decrease linearly with time to breakup".
  - The volume argument is this note's own: an incompressible cylinder of radius `r0` and length `L0` stretched to `L` has radius `r0 * sqrt(L0 / L)`. Only abstracts were read (`paper_search`, all four).
- **Horror-film and practical-effects references**: `## Reference set`.
- **Technique.** Zarkonnen, *My adventures in slime*, dev blog of *Biomechanoid Repair Shop*, 2026-05-11, `https://www.zarkonnen.com/biomech/slime` (only the first lines could be read: the page is cut): "So I wanted
  to draw a rope of slime between two points: it stretches, thins, and then breaks." And the general practice for a rope in a game: "a simplified physical representation, such as a short chain of capsules,
  particles, or rigid bodies connected by constraints, and the visible rope mesh is then drawn along that simulated path using a spline, cable mesh, or procedural tube" (`yelzkizi.org`, *Dynamic rope and
  tentacle simulation*). This crate already has the chain (`viscera::Strand`) and the tube (`tube_mesh`); a thread is the tube with no chain of its own, two endpoints and a curve.

## Craft principle

The practical-effects artist's slime is a **viscoelastic gel strung between two surfaces**: Bottin's kennel mass was "mayonnaise, melted bubble gum and plenty of K-Y Jelly" (Dead Northern, *The Thing*
retrospective), and a gel of that kind holds a string where water would not. Three things make a string read: it is **thin** and **bright** (a highlight along its whole length, T3), it **thickens at
both ends** where it wets the surface (the bead, T3; a meniscus, in the physics), and it **thins to a point and snaps** (Weickgenannt, Roisman & Tropea), the two halves falling away.

## How shipping games do it

A shipping game does not simulate the thread's surface: it draws a tube along a curve between the two attachment points and tapers it with the stretch ("it stretches, thins, and then breaks", the
Zarkonnen quote above). This crate's strands are tubes swept on the CPU already (`tube_mesh`), so the model is: **a `Thread` that knows its two ends only as numbers handed to it each tick**, a pure
function of them and of an integer age, and a mesh built from that. No texture, no binding, no shader branch.

## Model adopted

All of it cosmetic and outside every digest.

1. **A thread has a slack** `S` (metres): the distance at which it goes taut. It is slack for `L <= S`, drooping under gravity by `SAG_FRACTION * (S - L) + ...` (a quadratic curve whose middle sits
   below the chord); it straightens as `L` reaches `S`, and is taut and thinning past it.
2. **Thinning**: the neck's radius is `r0 * sqrt(S / L)` past `S` (the volume argument above), clamped at `NECK_MIN` of `r0`; the ends keep a bead of `BEAD` times `r0` over `BEAD_REACH` of the length.
3. **Snapping, once and for good**: the thread snaps when `L >= BREAK_STRETCH * S`, **or** when it has been taut for `BREAK_TICKS` ticks whatever the pull (Weickgenannt, Roisman & Tropea: the breakup
   time "does not depend on the acceleration"). A snapped thread stays snapped: a snap is never undone, as a tear is not (`CLAUDE.md`, "Tearing is monotone").
4. **Recoil**: for `RECOIL_TICKS` ticks after a snap each half shortens toward its own end, tapering to a point at the break, then the thread is gone (`Thread::gone`) and its owner despawns it.
5. **Where the preset draws them**: when a torso hit spills guts, each strand gets `THREADS_PER_STRAND` threads from the wound (a point within the wound's radius, hashed from the strand's own seed) to
   nodes of that strand, with a slack hashed in `SLACK_MIN_M..SLACK_MAX_M`; the thread's two ends are re-read from the strand every tick. A strand that is despawned takes its threads with it.
6. **The material** is a plain `StandardMaterial`: the colour of a thin venous film over a pale substrate (`bloodstain::spectral`, no colour authored), `AlphaMode::Blend`, roughness 0.05.

## Parameters

All **tuned by eye** against the reference set (this note), none measured, each saying so in its doc comment: `THREAD_RADIUS_M` 4 mm (Ref T3 is 1 to 1.5 mm in radius, but a thread under a pixel shimmers at the
preset's camera, where a gut 40 mm across is about 5.5 px, so a pixel is about 7 mm; a game with a closer camera sets it), `BEAD` 1.8, `BEAD_REACH` 0.08, `NECK_MIN` 0.2, `BREAK_STRETCH` 2.0 (Liao et al.'s Newtonian bridge breaks at 1.3 to 2.2 times
its length; a gel holds longer, and 2 is the lower), `BREAK_TICKS` 40 (0.67 s at 60 Hz; Liao et al.'s 0.1 to 0.2 s is a Newtonian bridge, and a slime breaks later), `RECOIL_TICKS` 10, `SAG_FRACTION` 0.6,
`SLACK_MIN_M` 0.12 and `SLACK_MAX_M` 0.40, `THREADS_PER_STRAND` 3.

## Validation targets

- A thread slack (`L < S`) is not snapped, its neck is `r0`, and it hangs below the chord; at `L = 1.5 S` the neck is `r0 * sqrt(1 / 1.5)` within 2 percent.
- A thread that goes past `BREAK_STRETCH * S` is snapped at once; one held at `1.2 S` is snapped at tick `BREAK_TICKS` and not before; a snapped thread whose ends come back together stays snapped.
- A snapped thread is gone within `RECOIL_TICKS` ticks and `mesh` then returns `None`; a mesh never carries a `NaN`; unit normals; a zero-length thread draws nothing rather than a degenerate mesh.
- A preset hit that spills guts spills `strands * THREADS_PER_STRAND` threads and, at a tier that refuses viscera, none; the threads are all gone within `BREAK_TICKS + RECOIL_TICKS + 5` ticks of the strands
  having fallen.
- Every recorder's digest equals 0.8.6's (`capture_skinned` x8, `capture_preset`, `capture_entrails`), twice; `bench_carnage`'s digest and frame time are 0.8.6's within noise.
- Against Ref T3 and D1: **a string is visible between the wound and a gut that has fallen from it for at least ten frames of `capture_preset`**, thin, light against the wound's dark and ending in a bead.

## What great looks like

T3's threads are **thin, bright, many, vertical, each ending in a bead**; D1's strands are **taut, slick, red, and join two things that are moving apart**. A frame is GREAT if a viewer who has not been told
sees *something stringy* joining the wound and the gut, if the string is lighter than what it hangs in front of (it catches the light), if it is thinner at the middle than at the ends, and if it breaks
cleanly without a pop. The threads must be a few, not a veil: thirty threads read as hair.

## Reference set

Stills are copyrighted film frames, kept under `/tmp/c13/refs/` for judging and never committed; this note cites their source pages.

| Ref | Film / source | Page | What it shows that the crate must reproduce |
|---|---|---|---|
| T3 | *The Thing* (1982), Palmer-thing | `https://collider.com/movies-practical-effects-best-ranked/` | dozens of clear threads hanging from a ceiling, 3 to 5 px of a 1 000 px frame (about 2 to 3 mm), bright against dark, each with a bead at its lower end, a thick red tentacle beside them |
| D1 | *Day of the Dead* (1985), Savini | `https://www.truemythmedia.com/true-myth-media/reviews/dayofthedead` | taut slick red strands between the cut edge and the hands: a web, not a veil, each running straight when taut |
| H1 | *Hellraiser* (1987), Frank | `https://makeupeffectsarchive.blogspot.com/2025/07/hellraiser-1987-part-1-frank-cottons.html` | clear slime in sheets and drapes over dark red tissue, with sharp specular breaks |
| K1 | *The Thing* kennel, Wikipedia (fair use) | `https://en.wikipedia.org/wiki/File:TheThing(1982)-KennelSFX.jpg` | red wet strands piled on straw |

## Deviations

- A real mucus or gel thread is viscoelastic and its breakup is not the Newtonian bridge's (the sources above are Newtonian bridges and a surfactant bridge, not slime); the thread here **takes their
  shape and their one clean fact, a breakup time that does not depend on the pull**, and the numbers are tuned. Design default.
- A thread is **not a simulation**: its curve is a function of its two ends and its age, with no mass of its own, so it cannot swing, wrap or catch on anything. That is what keeps it cheap and outside the digest.
- `GoreDials` gets **no** `threads` dial: adding a public field to a struct callers build literally is a breaking change, and the strand count (`GoreDials::strands`) and the viscera policy already
  switch the guts, and so the threads, off.

## Pending index

- The four physics papers are quoted from their abstracts (`paper_search`): `paper_download` was not tried, and `distill_search` for "filament thinning" and "bio-fluid filament breakup" found nothing relevant.
- *My adventures in slime* could be read only to its first lines.
- No home-still paper on the extensional rheology of **mucus or K-Y-type gels**; the breakup numbers are tuned, not derived.
