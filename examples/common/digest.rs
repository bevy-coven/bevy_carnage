//! **The digest line the two preset recorders print**, in one place.
//!
//! `capture_preset` and `capture_skinned` end on one line that two runs of a build must agree on and
//! that every pass compares against the baseline's. Its `digest=` field folds the wetmaps (and, for
//! the skinned body, the cloth that was holed); it has not changed since it was first documented, and
//! the digests the README and `docs/DEMOS.md` quote are values of it. The CPU state that field cannot
//! see is folded into two fields beside it (issue #8):
//!
//! - `flay=` — every [`FlayCanvas`]'s depth buffer: the craters, how deep each went and where;
//! - `dermis=` — every [`DermisCanvas`]: the bruises and burns under the skin.
//!
//! A pass that makes a crater's depth, a bruise's place or a burn's zones differ from run to run, or
//! from the baseline, moves one of them and not `digest=`.

use bevy::prelude::*;
use bevy_carnage::flaymap::FlayCanvas;
use bevy_carnage::preset::DermisCanvas;

/// **FNV-1a over a set of per-canvas digests**, folded in ascending order.
///
/// Sorted first: XOR-then-multiply is order-dependent, and the order a query hands canvases back in is
/// not a total order across two `App`s, which is exactly the comparison the line exists for. Two
/// canvases that swapped contents between entities fold to the same number, which is the cost of not
/// keying on an `Entity`, an id that is allocated rather than authored.
pub fn fold(mut values: Vec<u64>) -> u64 {
    values.sort_unstable();
    values.into_iter().fold(0xcbf2_9ce4_8422_2325, |h, v| (h ^ v).wrapping_mul(0x0000_0100_0000_01b3))
}

/// **` flay=<hex> dermis=<hex>`**, the two fields the `digest=` field leaves out, with the leading
/// space, for appending to the recorder's line.
pub fn skin_fields(world: &mut World) -> String {
    let flay = fold(world.query::<&FlayCanvas>().iter(world).map(FlayCanvas::digest).collect());
    let dermis = fold(world.query::<&DermisCanvas>().iter(world).map(DermisCanvas::digest).collect());
    format!(" flay={flay:016x} dermis={dermis:016x}")
}
