//! Macros: knobs of the user's own. Each adds its value times a depth to
//! every destination it lists, at the control tick, like a matrix route
//! whose source is the knob.
use crate::preset::{Destination, finite_clamp};
use serde::{Deserialize, Serialize};

/// Host parameters `macro1`..`macro8` turn them.
pub const MAX_MACROS: usize = 8;
pub const MAX_TARGETS: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Target {
    pub destination: Destination,
    pub depth: f32,
}
impl Default for Target {
    fn default() -> Self {
        Self {
            destination: Destination::None,
            depth: 0.5,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Macro {
    pub name: String,
    /// The knob, 0..1. At 0 every destination sits at its own value.
    pub value: f32,
    pub targets: Vec<Target>,
}
impl Default for Macro {
    fn default() -> Self {
        Self {
            name: "Macro".into(),
            value: 0.0,
            targets: Vec::new(),
        }
    }
}
impl Macro {
    pub(crate) fn sanitize(&mut self) {
        self.name = self.name.chars().take(40).collect();
        finite_clamp(&mut self.value, 0.0, 1.0, 0.0);
        self.targets.truncate(MAX_TARGETS);
        for target in &mut self.targets {
            finite_clamp(&mut target.depth, -1.0, 1.0, 0.0);
        }
    }
}
