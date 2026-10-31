# Reading list, per domain

The papers the crate's models rest on, grouped by the domains the loop re-scans the home-still corpus
for (`.omp/commands/build-loop.md`, section 1). This is an index, not a second citations table: each
row names where the paper is already cited, and that place says what the code takes from it.
`docs/bloodstain-citations.md` records, for the `bloodstain` papers, whether each one resolves in the
local corpus; a row added here for a new source says the same in its own words.

**Re-scans.** Iteration 1 (2026-10-30) read `catalog_recent` at limit 200: every one of the 200 events
was an embed or download of an LLM, speech, mesh-generation or agent paper, and none falls in a
domain below. Nothing was added. Iteration 1's second attempt (2026-10-30, 22:05 local) read it again at
limit 200: the window is the bulk embed of the SIGGRAPH *Advances in Real-Time Rendering* course notes,
2006 to 2025 (stems `s2006-advances-materials` … `s2025-advances-materials-12`), and `sig2024_*` and
`sig2026_*` papers (geometry processing, differentiable rendering, cloth appearance). The course notes
are the shipping-game technique source this loop cites, so they are listed under *Real-time gore* below;
`distill_search` answered `gpu busy` this pass, so which stem holds which talk is unchecked. Nothing blood-, wound- or fracture-specific
arrived, and the `sig*` papers are not listed: none is about a domain this crate models.
Iteration 2 (2026-10-31) read it at limit 200 against the same window (the SIGGRAPH course-note bulk embed
of 02:05 to 02:22 UTC, then spinal-deformity, chronic-pain and relationship-psychology papers up to 07:21):
nothing is blood-, wound-, fracture- or gore-specific. `tetgen_si2015_wias2602` and
`transvoxel_dissertation_lengyel2010` (mesh generation) are the nearest and model nothing this crate does.
`distill_search` answered `gpu busy: 4508 MB free < 5000 MB required to load bge-m3` for both queries, so the
corpus was not searched this pass; `paper_search` found the one paper below.

## Blood rheology and haemodynamics

| DOI | Paper | Cited in |
|---|---|---|
| `10.1103/physrevfluids.9.023305` | Steinik et al., *Inertial and shear-thinning effects in the capillary rise of a non-Newtonian fluid* | `docs/bloodstain-citations.md`, `src/bloodstain/wick.rs` |
| `10.1016/j.forsciint.2016.08.005` | Laan et al., *Morphology of drying blood pools* | `docs/bloodstain-citations.md` (`dry`) |
| `10.1038/s41598-020-65465-4` | Smith, Nicloux & Brutin, *A new forensic tool to date human blood pools* | `docs/bloodstain-citations.md` (`dry`) |
| `10.1016/j.forsciint.2011.07.027` | Bremmer et al., *Forensic quest for age determination of bloodstains* | `docs/bloodstain-citations.md` (`dry::SRGB_*`) |

## Droplet ballistics, impact and bloodstain pattern analysis

| DOI | Paper | Cited in |
|---|---|---|
| `10.1103/PhysRevFluids.3.063901` | Comiskey, Yarin & Attinger, *Forward spatter of blood from a gunshot* | `docs/bloodstain-citations.md`, `src/bloodstain/droplet.rs` |
| `10.1103/physrevfluids.2.073906` | Comiskey, Yarin & Attinger, *Hydrodynamics of back spatter* | `docs/bloodstain-citations.md` |
| `10.1520/jfs2003224` | Hulse-Smith, Mehdizadeh & Attinger, *Deducing drop size and impact velocity from circular bloodstains* | `docs/bloodstain-citations.md`, `src/bloodstain/stain.rs` |
| `10.1111/j.1556-4029.2007.00505.x` | Knock & Davison, *Predicting the position of the source of blood stains* | `docs/bloodstain-citations.md`, `src/bloodstain/stain.rs` |
| `10.1016/j.forsciint.2011.12.002` | Adam, *Fundamental studies of bloodstain formation* | `docs/bloodstain-citations.md` |
| `10.1016/j.forsciint.2019.109934` | Adam, *Release of blood droplets from weapon tips* | `docs/bloodstain-citations.md` |
| `10.1111/1556-4029.13855` | Williams et al., *Blood drop release from swinging objects* | `docs/bloodstain-citations.md` (`patterns::cast_off`) |
| `10.1007/s00414-010-0498-5` | Donaldson et al., *Expirated bloodstain pattern formation* | `docs/bloodstain-citations.md` |
| Mundo, Sommerfeld & Tropea (1995) | The deposition/splash boundary `K = We^0.5 Re^0.25` | `docs/bloodstain-citations.md` (`SPLASH_K_CRIT`) |

## Wound and skin biomechanics

| DOI | Paper | Cited in |
|---|---|---|
| `10.1016/j.jmbbm.2011.08.016` | Ní Annaidh et al., *Characterization of the anisotropic mechanical properties of excised human skin* | `src/laceration/curve.rs` |
| `10.1145/566570.566579` | O'Brien, Bargteil & Hodgins, *Graphical modeling and animation of ductile fracture* | `src/laceration/curve.rs`, `src/laceration/tear.rs` |
| `10.48550/arXiv.2209.08531` | Kamarianakis et al., *Progressive tearing and cutting of soft-bodies in high-performance virtual reality* | `src/laceration/tear.rs` |
| `10.4172/2155-6156.1000216` | Akkus et al., skin and subcutaneous adipose thickness by ultrasound | `src/cross_section/layers.rs` |
| `10.1371/journal.pone.0086637` | Derraik et al., skin thickness by site, age, sex and BMI | `src/cross_section/layers.rs` |
| `10.1177/1742271X14554678` | Abe, Loenneke & Thiebaud, ultrasound muscle thickness of the lower extremity | `src/cross_section/layers.rs` |
| `10.5603/fm.a2017.0114` | Dimitrova et al., facial soft-tissue thicknesses | `src/cross_section/layers.rs` |
| `10.1007/s11517-010-0647-5` | Stam et al., *3D finite compartment modeling of formation and healing of bruises* | `docs/bloodstain-citations.md` (`bruise`) |
| `10.1186/1475-925x-3-42` | Gowrishankar et al., *Transport lattice models of heat transport in skin* | `docs/bloodstain-citations.md` (`burn`) |
| `10.1002/bjs.18004016413` | Jackson, the three zones of a burn | `src/preset.rs` |

## Tissue optics and subsurface scattering

| DOI | Paper | Cited in |
|---|---|---|
| `10.1007/s10103-013-1446-7` | Bosschaart et al., *Optical properties of whole blood* | `docs/bloodstain-citations.md`, `src/flesh.rs` |
| `10.1145/383259.383319` | Jensen, Marschner, Levoy & Hanrahan, *A practical model for subsurface light transport* | `src/flesh.rs` |
| Penner, *Pre-Integrated Skin Shading* (SIGGRAPH 2011 course, no DOI) | The `(N·L, 1/r)` lookup `flesh::sss_lut` bakes | `src/flesh.rs` |

## Skinning and skeletal breakage

| DOI | Paper | Cited in |
|---|---|---|
| `10.1145/1230100.1230107` | Kavan, Collins, Žára & O'Sullivan, *Skinning with dual quaternions* (not in the corpus: `paper_download` found no open copy, see `docs/research/skeleton.md` `## Pending index`) | `docs/research/skeleton.md` |
| glTF 2.0 Specification, "Skinned Mesh Primitive Attributes", "Skins" and "Primitive topology" (Khronos, no DOI) | The per-vertex matrix `Σ W_i · M_Ji · B_Ji`; a triangle list's vertex count `MUST` be divisible by three | `docs/research/skeleton.md`, `src/skeleton.rs` |

## Fracture mechanics and fracture simulation

| DOI | Paper | Cited in |
|---|---|---|
| `10.1145/3549540` | Sellán et al., *Breaking Good: Fracture Modes for Realtime Destruction* | `docs/bloodstain-citations.md`, `src/fracture_modes/` |
| `10.1063/1.329934` | Grady, *Local inertial effects in dynamic fragmentation* | `docs/bloodstain-citations.md` |
| `10.1098/rspa.1947.0042` | Mott, *Fragmentation of shell cases* | `docs/bloodstain-citations.md` |
| `10.1016/j.cmpb.2022.106980` | Parra-Cabrera et al., *Fracture pattern projection on 3D bone models* | `docs/bloodstain-citations.md` |
| `10.3233/BME-1991-1102` | Miyasaka et al., *Bending and torsion fractures in long bones* | `docs/bloodstain-citations.md` |
| `10.1016/j.forsciint.2021.110899` | Isa et al., *Investigating reverse butterfly fractures* | `docs/bloodstain-citations.md` |
| `10.1016/j.forsciint.2016.04.035` | Cohen et al., *Impact velocity and bone fracture pattern* | `docs/bloodstain-citations.md` |
| `10.3390/jimaging11060187` | *Biomechanics of spiral fractures: periosteal effects via DIC* | `docs/bloodstain-citations.md` |
| `10.1111/cgf.13326` | Deul et al., *Direct position-based solver for stiff rods* | `src/viscera/solver.rs` |
| `10.1145/1399504.1360662` | Bergou et al., *Discrete elastic rods* | `src/viscera/solver.rs` |

## Real-time gore, game feel, decals and texture-space damage

| DOI | Paper | Cited in |
|---|---|---|
| `10.1109/tg.2021.3072241` | Pichlmair & Johansen, *Designing Game Feel: A Survey* | `docs/bloodstain-citations.md` |
| `10.1016/j.entcom.2020.100359` | Kao, *The effects of juiciness in an action RPG* | `docs/bloodstain-citations.md` |
| `10.1080/02699931.2010.496997` | Oum, Lieberman & Aylward, *A feel for disgust* | `docs/bloodstain-citations.md`, `src/flaymap/canvas.rs` |
| `10.1080/10867651.1997.10487468` | Möller & Trumbore, *Fast, minimum storage ray-triangle intersection* | `src/flaymap/uv.rs` |
| Kihl, SIGGRAPH 2010 *Advances in Real-Time Rendering* (no DOI) | Frostbite 2's GPU damage mask | `CLAUDE.md` (`flaymap`) |
| SIGGRAPH *Advances in Real-Time Rendering in Games* course notes, 2006-2025 (no DOI; stems `s2006-advances-materials` … `s2025-advances-materials-12`, embedded 2026-10-30) | The shipping-game technique source: the per-year material, shading and destruction notes. Which stem holds Kihl's 2010 talk is unchecked | the `## How shipping games do it` of each research note |

## Game testing and visual QA

| DOI | Paper | Cited in |
|---|---|---|
| `10.7603/s40601-013-0010-4` | Ostrowski & Aroudj, *Automated Regression Testing within Video Game Development* (stem `s40601-013-0010-4`, embedded 2026-10-30) | `docs/research/qa-harness.md` |
| `10.48550/arXiv.2309.11077` | Rahman, *Weak Supervision for Label Efficient Visual Bug Detection* | `docs/research/qa-harness.md` |

## Practical makeup effects

None yet. The horror-film reference sets the loop gathers are web pages and stills, cited by URL in
each system's research note under `## Reference set`, not papers.
