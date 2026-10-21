//! SAD PET granular instrument, developed from EVIL PET firmware analysis. See PORT_STATUS.txt for fidelity boundaries.
#[cfg(all(feature = "desktop", not(target_arch = "wasm32")))]
pub mod audio;
pub mod color;
pub mod dsp;
pub mod firmware;
pub mod imported_fx;
pub mod matrix;
pub mod midi;
pub mod parameters;
pub mod performance;
pub mod preset;
pub mod radias_delay;
#[cfg(all(feature = "desktop", not(target_arch = "wasm32")))]
pub mod recording;
pub mod resynth;
pub mod samples;
pub mod sequence;
#[cfg(all(feature = "desktop", not(target_arch = "wasm32")))]
pub mod server;
pub mod tables;
pub mod wav;

pub type Stereo = [f32; 2];
