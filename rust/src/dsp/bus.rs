//! Bus input history preparation and ownership.
use super::sample::LiveInput;

/// The input history allocated off the audio thread for a granular Bus.
/// The engine takes it over in `exchange_preset`.
#[derive(Default)]
pub struct Prepared(pub Option<LiveInput>);

impl std::fmt::Debug for Prepared {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreparedBus")
            .field("history", &self.0.is_some())
            .finish()
    }
}
