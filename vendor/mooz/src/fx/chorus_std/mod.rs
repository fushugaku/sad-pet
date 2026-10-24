//! Standard chorus group: the shared modulated-delay callback (Chorus,
//! StereoCho, Vibrato, Ba Chorus, ModDelay, ModDelay2), Ensemble / Ba Ensmbl,
//! BendCho, MirageCho, VintageCE and CloneCho.
pub mod ba_chorus;
pub mod ba_ensmbl;
pub mod bend_cho;
pub mod chorus;
pub mod clone_cho;
mod common;
pub mod ensemble;
pub mod mirage_cho;
pub mod mod_delay;
pub mod mod_delay2;
mod modulated;
pub mod stereo_cho;
pub mod vibrato;
pub mod vintage_ce;
