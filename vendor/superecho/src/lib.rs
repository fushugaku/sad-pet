//! KekLocator native Rust DSP, embedded as SAD PET SuperEcho. See NOTICE.md.
pub mod machine;
#[rustfmt::skip]
mod generated;
pub mod engine;
pub mod parameters;
pub use engine::Engine;
pub use generated::FUNCTIONS as FIRMWARE_FUNCTIONS;
