//! KekLocator native Rust DSP, embedded as SAD PET SuperEcho. See NOTICE.md.
pub mod machine;
#[rustfmt::skip]
mod generated;
#[allow(
    unused_assignments,
    unused_variables,
    unused_parens,
    unused_comparisons,
    unreachable_code,
    clippy::all
)]
#[rustfmt::skip]
mod fused {
    include!(concat!(env!("OUT_DIR"), "/fused.rs"));
}
pub mod engine;
pub mod parameters;
pub use engine::Engine;
pub use generated::FUNCTIONS as FIRMWARE_FUNCTIONS;
