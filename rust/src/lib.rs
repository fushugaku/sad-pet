//! SAD PET granular instrument, developed from EVIL PET firmware analysis. See PORT_STATUS.txt for fidelity boundaries.
pub mod audio;
pub mod color;
pub mod dsp;
pub mod firmware;
pub mod matrix;
pub mod midi;
pub mod performance;
pub mod preset;
pub mod radias_delay;
pub mod recording;
pub mod samples;
pub mod server;
pub mod tables;
pub mod wav;

pub type Stereo = [f32; 2];
