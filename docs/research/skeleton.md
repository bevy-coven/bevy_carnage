# `fracture_skeleton`: what it reads, and what it must refuse

Iteration 2 (2026-10-31), issue #7. A non-visible item: no pixel changes, so there is no `## Reference set`;
the reference source is the shipping-game pipelines below. The code is `src/skeleton.rs`
(`read_part`), the tests are `tests/skeleton.rs`.

## Sources

1. **Physics (the maths of the thing being refused).** The glTF 2.0 specification, section "Skins": the
   per-vertex matrix is a weighted sum over the joints that influence it. And Kavan, Collins, Žára &
   O'Sullivan, *Skinning with dual quaternions* (`doi:10.1145/1230100.1230107`, I3D 2007), for linear
   blend skinning being the standard the body is drawn with.
2. **Horror-film reference, replaced by a shipping pipeline** (the item changes no pixel): Bethesda's
   Creation Engine dismemberment partitions (`BSDismemberSkinInstance`, GECK wiki) and Raven's GHOUL
   (*Soldier of Fortune*, 2000): both break a skinned body along its bones, both at *triangle*
   granularity.
3. **Technique (what the loader hands this function).** The glTF specification's "Primitive topology"
   rules and `bevy_gltf-0.20.1/src/loader/gltf_ext/mesh.rs:24-27`, which maps `Mode::Lines` to
   `LineList` and `Mode::TriangleStrip` to `TriangleStrip` and rejects only fans.

## Craft principle

The glTF specification defines the body's drawn position of a vertex (`W_i` the weight,
`M_j` the joint's global matrix, `B_j` its inverse bind matrix):

> "Then the effective transformation matrix of the current vertex is computed as a weighted linear sum of
> the global transformation matrices of the joints premultiplied with the corresponding inverse bind
> matrices. `Σ_{i=0}^{N-1} W_i · M_{J_i} · B_{J_i}`"
> (glTF 2.0, "Skins", <https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html>)

and calls the scheme by name: "**linear blend skinning**: A skinning method that computes a per-vertex
transformation matrix as a linear weighted sum of transformation matrices of the designated nodes."
Kavan et al. record that it is what games ship: "The standard solution, linear blend skinning, has some
serious drawbacks that require artist intervention. [...] linear blend skinning is still the number one
choice for the majority of developers."

`fracture_skeleton` promises that "a piece drawn with its joint's `joint_global · inverse_bind` matrix
lands exactly where the whole body drew it" (`src/skeleton.rs:23-24`). From the sum above that holds for a
vertex `v` exactly when every joint carrying weight on it is one the piece's group declares to move as
one (so the sum collapses to `(Σ W_i) · M_a · B_a`, and the weights sum to one). A vertex weighted
0.6 / 0.4 across two groups is drawn at `0.6·M0·B0·v + 0.4·M1·B1·v`; its piece, carried by one anchor,
draws it at `M0·B0·v`. The two agree only at the bind pose, so **the piece jumps at the instant of the
break** and the gap grows with the pose. That is a property of the sum, not of a bad asset, and nothing
a refusal threshold can paper over: so the condition is exact, with no epsilon.

What the function may read is a triangle list. The glTF specification: "**Triangle Lists** The number of
the primitive's vertices **MUST** be divisible by three. Each consecutive set of three vertices defines a
single triangle primitive", and for a strip "One triangle primitive is defined by each vertex and the two
vertices that follow it" (`P_i = { v_i, v_{i+(1+i%2)}, v_{i+(2-i%2)} }`). The same index buffer
`[0, 1, 2, 3]` is two triangles (0-1-2 and 2-1-3) as a strip and one triangle plus a dropped index as a
list, which is what `indices.chunks_exact(3)` does to it.

## How shipping games do it

- **Creation Engine** (Skyrim, Fallout 3, New Vegas, Fallout 4): a skin carries a partition list; each
  partition is a body part with a flag for whether it is visible when whole or only once dismembered
  (gore caps). Its modding documentation states the rule this crate's `SplitTriangle` enforces:
  "Polygons cannot be assigned to multiple dismemberment partitions, as it crashes the game if a model
  with them is loaded." (`BSDismemberSkinInstance`, GECK wiki,
  <https://geckwiki.com/index.php?title=BSDismemberSkinInstance>; read through a search result's quoted
  text, the site answering a direct fetch with a Cloudflare 403.) A shipping engine refuses the ambiguous
  triangle by name at build time rather than guessing where it goes.
- **GHOUL** (*Soldier of Fortune*, Raven, 2000): "character models are based on body parts that can each
  independently sustain damage ([[Graphic violence|gore]] zones). There are 26 zones in total"
  (Wikipedia, "Soldier of Fortune (video game)", wikitext read 2026-10-31). The body is authored as
  zones, so there is nothing to cut at run time.
- **Consequence for this function.** Both ship a rigid partition and refuse what does not partition. A
  triangle across two groups is refused (`SplitTriangle`, 0.7.0). The two holes the issue reports are
  the same refusal one level down: a *vertex* across two groups (`SplitVertex`), and an input that is
  not a triangle list at all (`NotTriangleList`) or names a vertex that does not exist (`BadIndex`).

## Model adopted

`read_part` checks, in this order, and the first failure is the answer:

1. topology is `TriangleList` and the index count is a multiple of three, else
   `NotTriangleList { part }`. Checked first: it is one fact about the mesh, and every later check assumes it.
2. positions, joints and weights are present (`NotSkinned`); indices are present and readable
   (`NotIndexed`).
3. every index is below the vertex count, else `BadIndex { part, triangle }` naming the first triangle
   that holds one. Before the group checks, so a missing vertex is never reported as a rigidity failure.
4. every vertex's dominant joint is grouped (`UngroupedJoint`), as before.
5. every vertex with positive, finite weight on a joint outside its dominant joint's group is refused,
   `SplitVertex { part, vertex }`. A weight on a joint that **no** group holds counts as outside: the vertex
   is then partly carried by something that belongs to no piece. Weights that are zero, negative or not finite
   are skipped, as `Rig::dominant_joint` skips them. Vertices are checked in index order, so the first
   offender reported is the same on every run.
6. a triangle whose three vertices are in different groups is `SplitTriangle`, as before.

Nothing in the output changes for an input that passes: the cut is the same bits (`two_cuts_of_one_body_are_the_same_bits`).

## Parameters

None. Check 5 has no tolerance on purpose: a tolerance would be a number with no source, and the maths
above is exact. glTF's own epsilon (`2·10^-7` times the count of non-zero weights) is for whether a
vertex's weights *sum* to one, which this function does not read.

## Validation targets

- The three tests in issue #7 fail before the change and pass after (`a_strip_is_refused_not_read_as_a_list`,
  `an_out_of_range_index_is_not_called_a_split_triangle`, `a_vertex_weighted_across_two_groups_is_refused`),
  plus a line list, an index count that is not a multiple of three, and a weight on a joint no group holds.
- All seven existing tests in `tests/skeleton.rs` pass unchanged; `a_triangle_across_two_groups_is_refused_not_guessed`
  still names `triangle: 24`.
- A vertex weighted across two joints **of one group** is still cut (the caller said they move as one).
- `tests/leaf.rs` (no panicking call in `src/`) passes; no recorder digest and no bench digest moves
  (`fracture_skeleton` has no caller in `examples/`).

## Deviations

- `SkeletalRefusal` gains three variants. It is public and not `#[non_exhaustive]`, so this is a
  semver-breaking change against any release that has it; `fracture_skeleton` is in no published version
  (crates.io has `0.5.18`, this is `0.7.0`), so against what consumers can hold it is additive.
- `NotTriangleList` also covers an index count that is not a multiple of three. The issue does not list
  it. It is the same hole (`chunks_exact(3)` drops the remainder without a word, "Nothing is repaired or
  guessed", `src/skeleton.rs:81`) and the glTF specification makes such a list invalid.
- `mesh.indices()` becomes `mesh.try_indices_option()`: the plain accessor `expect`s on a mesh whose data
  was extracted to the render world (`bevy_mesh-0.20.1/src/mesh.rs:809-811`), which `CLAUDE.md` forbids in
  library code. Reached only after `Rig::of` has read the same mesh, so it is a rule fix, not a new path.

## What great looks like

A caller who passes the wrong mesh gets a refusal that names the true cause and the offending part,
triangle or vertex, and never a piece that was cut from the wrong geometry: no silently dropped triangle,
no "the body is not rigid" for a vertex that is out of range, no limb that jumps at the break.

## Pending index

- `10.1145/1230100.1230107` (Kavan et al. 2007): `paper_download` found no open copy (Unpaywall: no PDF
  location; the TCD repository `hdl.handle.net/2262/18079` answers 403; CORE needs a key). Quoted here from
  its abstract as `paper_search` returned it. Add the PDF to the inbox to index it.
- `distill_search` answered `gpu busy: 4508 MB free < 5000 MB required to load bge-m3`, so the corpus
  itself was not searched this pass; the three sources above are public specifications and a published abstract.
- The Creation Engine quote is from a search result's text of the GECK page; a later pass with a browser
  can read the page in full and confirm the quoted sentence's context.
