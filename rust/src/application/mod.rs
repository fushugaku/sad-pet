//! Shared control-side use cases. Hosts decode their protocols and publish
//! prepared updates; the application decides what an edit means for the model.
//! These operations may clone settings, so they run outside the audio callback.
pub mod presets;
