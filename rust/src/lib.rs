//! SAD PET granular instrument, developed from EVIL PET firmware analysis. See PORT_STATUS.txt for fidelity boundaries.
#[cfg(all(feature = "desktop", not(target_arch = "wasm32")))]
pub mod audio;
pub mod bus;
pub mod color;
pub mod drums;
pub mod dsp;
pub mod effect_presets;
pub mod firmware;
pub mod imported_fx;
pub mod macros;
pub mod matrix;
pub mod midi;
pub mod parameters;
pub mod performance;
pub mod preset;
pub mod radias_delay;
#[cfg(all(feature = "desktop", not(target_arch = "wasm32")))]
pub mod recording;
pub mod resynth;
pub mod sampler;
pub mod samples;
pub mod sequence;
#[cfg(all(feature = "desktop", not(target_arch = "wasm32")))]
pub mod server;
pub mod startup;
pub mod synth;
pub mod tables;
pub mod taps;
pub mod timbres;
pub mod vocal;
pub mod wav;

pub type Stereo = [f32; 2];
