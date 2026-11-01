#![doc = include_str!("../../docs/viscera.md")]

mod frame;
mod hash;
mod look;
mod plugin;
mod settings;
mod solver;
mod spill;
mod strand;
mod thread;
mod tube;

pub use plugin::{VisceraPlugin, VisceraSystems};
pub use look::{GUT_ROUGHNESS, SOAK_BASE, SOAK_FILM_MM, SOAK_PATCH, SOAK_PATCH_M, SOAK_STEPS, WOUND_REACH_M, gut_image, soak, soak_tint};
pub use settings::{
    ViscSettings, DEFAULT_COMPLIANCE_BEND, DEFAULT_COMPLIANCE_STRETCH, DEFAULT_DAMPING,
    DEFAULT_FLOOR_Y, DEFAULT_GRAVITY, DEFAULT_ITERATIONS, DEFAULT_MAX_STRANDS, DEFAULT_SUBSTEPS,
};
pub use solver::{step, COMPLIANCE_MESENTERY, FIXED_DT, FIXED_HZ, MAX_ANCHORS};
pub use spill::{spill, SPILL_CONE, SPILL_RADIUS, SPILL_REST_LEN, SPILL_SEGMENTS};
pub use strand::{
    Mesentery, Strand, DEFAULT_TEAR_STRAIN, MAX_NODES, MAX_SEGMENTS, MIN_REST_LEN,
    STRAND_TEAR_STRAIN,
};
pub use thread::{spill_threads, Thread, ThreadSite, BEAD, BEAD_REACH, BREAK_STRETCH, BREAK_TICKS, NECK_MIN, RECOIL_TICKS, REST_RANGE_M, SITE_SPREAD_M};
pub use tube::{soaked_tube, tube_mesh, LUMP_MAJOR, LUMP_MAJOR_NODES, LUMP_MINOR, LUMP_MINOR_NODES, MAX_SIDES, MIN_SIDES, SLUMP_FADE, SLUMP_SQUASH};
