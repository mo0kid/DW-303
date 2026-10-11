//! Allocation-free TB-303 pattern storage, sample clock and monophonic playback.
#![no_std]

mod clock;
mod model;
mod playback;
pub use clock::*;
pub use model::*;
pub use playback::*;

/// Preparation error shared with the circuit stages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StageError {
    /// Rate must be finite and in the supported 44.1–384 kHz range.
    InvalidSampleRate,
}
