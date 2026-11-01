# Organs: a heap of guts has lumps in it

Iteration 13 (2026-11-01). Item: the lumps a spill is missing. Ranked first after iteration 13's own first pick (slime strings, `docs/research/slime-strings.md`) was built, measured and **parked**: a thread is
a sixth of a pixel at the preset's camera and the strands it joins fall in a third of a second. Iteration 12's backlog item 1 is "the heap and the web (D1, T1): more, thinner strands that overlap, **organ blobs**
(stomach, liver) as lumpy closed meshes"; what survives the scale test is the lumps, because a liver is a fifth of a metre across, thirty pixels at the preset's camera. Runner-ups: more strands (a dial default,
tried below), the wound interior and skin lip (a `flesh.wesl` change, a 79-minute WebGL2 re-check and a disk with 6 GB free), a burst hit (moves digests), slime strings (parked, `loop/iter-13`).

## What was measured before any change

`capture_preset` on 0.8.6, frames 150 to 360: what a torso hit that reached muscle leaves is **loops and nothing else**: three to five bowel strands 40 mm across and 0.84 m long in a pool, under and between the pieces of
the body. Every lump in the heap is a piece of the body. Against Ref D2 (*Day of the Dead*, the opened torso) the cavity is **a mass**: reddish pixels of the cavity (box `330,260,330,170`, mask `R > 1.5 G and R > 1.5 B and
R > 40`, 49 582 pixels) mean sRGB `(127, 38, 40)`, red minus green 89, luminance 56.9, 5th percentile 15.8, 98th percentile 133, with dark masses (box `530,300,110,90`, 9 512 pixels) `(123, 52, 52)`, luminance 66.8; and
against Ref T1 (the kennel) a pile of wet lumps. There is nothing in the crate's spill that is a lump: `viscera` has strands and no other shape.

## Sources

- **Physics (anatomy).** Gray, *Anatomy of the Human Body* (1918, public domain), "XI. Splanchnology, 2i. The Liver" (`http://ascenttrust.com/Archive/Manual/250.html`): "In the male it weighs from 1.4 to 1.6 kilogm.,
  in the female from 1.2 to 1.4 kilogm. ... Its greatest transverse measurement is from 20 to 22.5 cm. Vertically, near its lateral or right surface, it measures about 15 to 17.5 cm., while its greatest
  antero-posterior diameter ... is from 10 to 12.5 cm. ... Its consistence is that of a soft solid; it is friable, easily lacerated and highly vascular; its color is a dark reddish brown, and its specific gravity is
  1.05." `distill_search` found nothing on organ size or tissue optics (two queries; the first page was off-topic), so this is a web source, recorded under `## Pending index`.
- **Horror-film and practical-effects references**: `## Reference set`.
- **Technique.** Evans-Lawes and Ridge on *Dead Island 2*'s FLESH system (Kotaku, 2023-05-08, `https://kotaku.com/ama-reddit-dead-island-2-weapons-flesh-system-ps5-xbox-1850415887`), quoted in
  `docs/research/guts.md`: "Zombies are built out of a surface mesh and a set of 'viscera' meshes", "individual pieces that can be destroyed, or broken", "filling the resulting hole with destructible guts". A
  shipping game's guts are **meshes of different shapes**, not one tube; the crate's `tube_mesh` is one of them and an organ is the other.

## Craft principle

A heap of guts reads as guts when it is **not all one shape**: the loops are ropes, and among them are masses (liver, stomach, spleen) that are dark, wet and heavy. Gray gives the liver its numbers: a **wedge
about 21 cm wide, 16 cm deep and 11 cm thick, 1.5 kg, dark reddish brown, soft** and of specific gravity 1.05, which is to say that it **falls like a stone, does not bounce, and flattens where it lands**.

## How shipping games do it

*Dead Island 2*'s viscera are individual meshes (the Kotaku quote above). This crate's are tubes swept on the CPU, so an organ is the same kind of thing: **a mesh built from a seed**, one function (`organ_mesh`) and a
texture (`organ_image`), handed to a material the caller owns, and spilled by the preset beside the strands. It is thrown with the pieces' own ballistic integrator (`Flying`): no new physics.

## Model adopted

All of it cosmetic and outside every digest.

1. **The shape** is a unit sphere displaced to a **wedge**: half-extents `(0.105, 0.05, 0.08)` m times a `size` (the liver's 21 cm, a flattened 10 cm of its 11 and 16 cm of its depth; Gray), tapering to 0.55 of its
   thickness toward its left end, with three octaves of seeded value noise on the radius (`ORGAN_LUMP` 0.16, 0.08, 0.04 of it) so that no two are alike, and **flattened to a contact patch** below `-ORGAN_FLAT` of
   its height (it lies on the floor, Gray's "soft solid"). The mesh's lowest point is at `y = 0`, so the preset's floor clamp sits it on the floor. Normals are the displaced surface's, found by finite differences
   of the displacement and so **seamless**.
2. **The texture** (`organ_image`, 128 x 64, wraps round `u`): the dark reddish brown Gray gives, mottled at the scale of a lobule, paler fibrous patches (the capsule) and blood smeared in streaks, every channel
   a seeded noise: the same recipe as `gut_image`.
3. **The material** is the gut's `FleshMaterial` recipe (muscle profile, `FleshMode::Cap`) with the organ texture; roughness 0.35 (a wet capsule: a broad highlight, not the guts' streak).
4. **The spill**: a torso hit that reaches muscle also throws `ORGANS_PER_SPILL` (2) organs, of sizes in `0.75..1.15`, from the wound with a velocity along its normal and a little up, in the strands' own seed;
   each is a `Flying` and an `Organ`, with the same viscera-policy gate as the guts.

## Parameters

All **tuned by eye** against the reference set unless a source names it: the half-extents (Gray), `ORGAN_LUMP`, `ORGAN_FLAT` 0.35, the taper 0.55, the roughness 0.35, `ORGANS_PER_SPILL` 2, the throw speed 0.8 to 1.6 m/s.
Each says so in its doc comment.

## Validation targets

- `organ_mesh`: counts as documented, **every normal unit**, every position finite, lowest point at `y = 0` within `1e-5`, signed volume positive (outward winding), and the volume within 40 percent of the wedge's
  ellipsoid (a lumpy flattened ellipsoid is not an ellipsoid); two seeds give two meshes, one seed the same mesh; the extents are Gray's within the lumps (width at least 1.6 times the thickness).
- `organ_image`: right size, **seamless round `u`** (the same test as `gut_image`'s).
- A preset hit that spills guts spills `ORGANS_PER_SPILL` organs; at a tier that refuses viscera, none; after 200 ticks every organ is at rest on the floor.
- Every recorder's digest equals 0.8.6's (`capture_skinned` x8, `capture_preset`, `capture_entrails`), twice; `bench_carnage`'s digest and frame time are 0.8.6's within noise.
- Against Ref D2 and T1: **among the loops of a pile there are two dark lumps**, reddish pixels of an organ within 15 of D2's mean `(127, 38, 40)` in red minus green (`R - G` 75 to 105) and a luminance between 35 and 75;
  the lumps are visibly not tubes at the preset's camera (more than 12 px across at frame 200).

## What great looks like

D2's cavity is **wet, saturated red and dark where it is deep, in lumps that overlap**; T1's pile is **lumps and loops, none of one shape**. A frame is GREAT if a viewer sees *a liver* among the loops: a dark wedge, glossy,
flattened on the floor, lying in the pool's edge and not floating over it, not a sphere and not a cube, and if the loops still read as loops.

## Reference set

Stills are copyrighted film frames, kept under `/tmp/c13/refs/` for judging and never committed; this note cites their source pages.

| Ref | Film / source | Page | What it shows that the crate must reproduce |
|---|---|---|---|
| D2 | *Day of the Dead* (1985), the opened torso | `https://www.imdb.com/title/tt0088993/mediaviewer/rm947079168` | a cavity of wet red masses over pale ribs: reddish pixels mean `(127, 38, 40)`, red minus green 89, luminance 56.9, 5th percentile 15.8, 98th percentile 133; dark masses `(123, 52, 52)` |
| D1 | *Day of the Dead* (1985), the gut pull | `https://www.truemythmedia.com/true-myth-media/reviews/dayofthedead` | loops and masses together in one bright red heap (`(138, 36, 31)`) |
| T1 | *The Thing* (1982), kennel (Wikipedia, fair use) | `https://en.wikipedia.org/wiki/File:TheThing(1982)-KennelSFX.jpg` | a pile of wet red lumps and strands on straw |

## Deviations

- Gray's liver is **the** organ; a stomach, a spleen and the rest are other shapes (a sac, a bean). One wedge with a different seed and size stands for all of them, and the note says so: **a design default**.
  The taper and the lumps give a seed its own look; they do not give a stomach its sac.
- An organ is **a rigid lump** thrown and left: it does not drip, tear, soak the floor, or roll. Its texture carries the blood it would wear.
- `GoreDials` gets **no** `organs` dial (a public field added to a struct callers build literally is a breaking change); `GoreDials::strands` at 0 or the viscera policy switch the organs off with the guts, and
  `ORGANS_PER_SPILL` is a constant.

## Pending index

- Gray (1918) is a web page, not in the corpus; no home-still document on organ size or colour was found (two `distill_search` queries).
- Bosschaart et al. (`10.1007_s10103-013-1446-7`) **is** in the corpus (found by the second query): iteration 12's `docs/research/guts.md` says it is not, and is corrected in this pass.
