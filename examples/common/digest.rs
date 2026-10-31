//! **The digest line the two preset recorders print**, in one place.
//!
//! `capture_preset` and `capture_skinned` end on one line that two runs of a build must agree on and
//! that every pass compares against the baseline's. Its `digest=` field folds the wetmaps (and, for
//! the skinned body, the cloth that was holed) as they stand when the clip ends; it has not changed
//! since it was first documented, and the digests the README and `docs/DEMOS.md` quote are values of
//! it. The CPU state that field cannot see is folded into two fields beside it (issue #8):
//!
//! - `flay=` — every [`FlayCanvas`]'s depth buffer: the craters, how deep each went and where;
//! - `dermis=` — every [`DermisCanvas`]: the bruises and burns under the skin.
//!
//! **They fold every frame of the clip, not the last.** A body that breaks is despawned and its
//! pieces are not skin: at the end of `capture_preset`, and of `capture_skinned --break`, no canvas of
//! the body is left, and a fold of the world as it then stands is the same number whatever the bruises
//! and craters were, which is the blind spot again under another name (measured: both fields read
//! `cbf29ce484222325`, the empty fold, at the end of both clips). [`SkinTrace`] samples each frame, so
//! the canvases are seen while they exist.
//!
//! A pass that makes a crater's depth, a bruise's place or a burn's zones differ from run to run, or
//! from the baseline, moves one of them and not `digest=`.

use bevy::prelude::*;
use bevy_carnage::flaymap::FlayCanvas;
use bevy_carnage::preset::DermisCanvas;

/// The FNV-1a 64-bit offset basis and prime: the constants every digest in the crate uses.
const BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const PRIME: u64 = 0x0000_0100_0000_01b3;

/// **FNV-1a over a set of per-canvas digests**, folded in ascending order.
///
/// Sorted first: XOR-then-multiply is order-dependent, and the order a query hands canvases back in is
/// not a total order across two `App`s, which is exactly the comparison the line exists for. Two
/// canvases that swapped contents between entities fold to the same number, which is the cost of not
/// keying on an `Entity`, an id that is allocated rather than authored.
pub fn fold(mut values: Vec<u64>) -> u64 {
    values.sort_unstable();
    values.into_iter().fold(BASIS, |h, v| (h ^ v).wrapping_mul(PRIME))
}

/// **The craters, bruises and burns of a whole clip**, one sample a frame.
pub struct SkinTrace {
    flay: u64,
    dermis: u64,
}

impl Default for SkinTrace {
    /// A trace that has seen no frame: both fields the empty fold.
    fn default() -> Self {
        Self { flay: BASIS, dermis: BASIS }
    }
}

impl SkinTrace {
    /// **Fold this frame's canvases in**: the sorted fold of every `FlayCanvas` and of every
    /// `DermisCanvas` in the world, chained onto what the earlier frames left. Call it once a frame,
    /// after the frame's update, in every run alike.
    pub fn sample(&mut self, world: &mut World) {
        let flay = fold(world.query::<&FlayCanvas>().iter(world).map(FlayCanvas::digest).collect());
        let dermis = fold(world.query::<&DermisCanvas>().iter(world).map(DermisCanvas::digest).collect());
        self.flay = (self.flay ^ flay).wrapping_mul(PRIME);
        self.dermis = (self.dermis ^ dermis).wrapping_mul(PRIME);
    }

    /// **` flay=<hex> dermis=<hex>`**, with the leading space, for appending to the recorder's line.
    pub fn fields(&self) -> String {
        format!(" flay={:016x} dermis={:016x}", self.flay, self.dermis)
    }
}
