//! Bus and Master settings control the granular engine on the sum of other
//! timbres. Everything starts switched off, and then the engine passes its
//! input through at almost no cost: no grains, no history writes, no filters
//! and no voices.
use serde::{Deserialize, Serialize};

/// Bus 1–8; Master is separate and always present.
pub const MAX_BUSES: usize = 8;
/// A bus history holds at most this much input, so the window never reallocates.
pub const MAX_BUFFER_SECONDS: f32 = 8.0;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// The scene's output stage rather than one of the buses.
    pub master: bool,
    /// Grains read a live history of the input; otherwise only effects apply.
    pub granular: bool,
    /// Filter 1 and 2; otherwise the input bypasses them.
    pub filters: bool,
    pub buffer_seconds: f32,
    /// Only Master uses this: protection after the complete mix and fader.
    /// Missing in older scenes, which receive the same protection as new ones.
    pub limiter: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            master: false,
            granular: false,
            filters: false,
            buffer_seconds: 4.0,
            limiter: true,
        }
    }
}
impl Settings {
    pub fn sanitize(&mut self) {
        self.buffer_seconds = if self.buffer_seconds.is_finite() {
            self.buffer_seconds.clamp(0.125, MAX_BUFFER_SECONDS)
        } else {
            4.0
        };
    }
    pub fn history(&self) -> crate::synth::History {
        crate::synth::History {
            granular: self.granular,
            buffer_seconds: self.buffer_seconds,
        }
    }
}
