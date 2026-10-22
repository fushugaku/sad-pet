//! Singing synthesis for SAD PET, in the manner of VOCALOID5: a score of
//! notes and lyrics becomes timed phonemes, units cut from a voicebank are
//! joined in the spectral domain, and a source-filter vocoder sings them on
//! the generated pitch. Everything here is plain Rust and runs on the audio
//! worker threads of the plug-in and app as well as in a browser Worker.
pub mod analysis;
pub mod bank;
pub mod concat;
pub mod formant;
pub mod g2p;
pub mod params;
pub mod phoneme;
pub mod pitch;
pub mod render;
pub mod score;
pub mod spectrum;
pub mod timing;
pub mod utau;
pub mod vocoder;
pub mod wav;

pub use phoneme::{Language, Phoneme, Syllable};
