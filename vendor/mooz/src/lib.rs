//! mooz: the 137 effects of the Zoom MS-70CDR 2.10 firmware, rebuilt as native
//! f32 DSP from the zoom-ms70-cdr reconstruction (recovered audio callbacks and
//! the disassembled parameter editors of every ZDL module).
//!
//! Every model runs at the host sample rate. Parameters arrive in pedal units:
//! index `i` of [`Values`] is the `i`-th user parameter of the model's catalog
//! entry (the OnOff, name and Tail records of the original descriptor are not
//! user parameters here), ranging over `0..=max` and possibly fractional while
//! a modulator moves it.
pub mod catalog;
pub mod dsp;
mod fx;
mod registry;
pub mod zoom;

pub use catalog::{Category, Display, ModelInfo, ParamInfo};
pub use registry::Model;

/// Largest user parameter count of any model.
pub const MAX_PARAMS: usize = 9;

/// Parameter values in pedal units, in catalog order. Unused entries are 0.
pub type Values = [f32; MAX_PARAMS];

/// A running effect instance. It is created (and allocates) on a control
/// thread; `set`, `process` and `clear` run on the audio thread and must not
/// allocate, lock or block.
pub trait Effect: Send {
    /// Apply parameter values and the host tempo. Called whenever either
    /// changes, possibly every few milliseconds while modulated.
    fn set(&mut self, values: &Values, bpm: f32);
    /// Process one stereo frame at the instance's sample rate.
    fn process(&mut self, frame: [f32; 2]) -> [f32; 2];
    /// Silence every history without reallocating.
    fn clear(&mut self);
}

impl Model {
    /// Default parameter values in pedal units.
    pub fn defaults(self) -> Values {
        let mut values = [0.0; MAX_PARAMS];
        for (value, param) in values.iter_mut().zip(self.info().params) {
            *value = f32::from(param.default);
        }
        values
    }

    pub fn key(self) -> &'static str {
        self.info().key
    }

    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|model| model.key() == key)
    }

    /// Normalized (0..1) values to pedal units, as stored by SAD PET slots.
    pub fn denormalize(self, normalized: &[f32; MAX_PARAMS]) -> Values {
        let mut values = [0.0; MAX_PARAMS];
        for ((value, param), &n) in values.iter_mut().zip(self.info().params).zip(normalized) {
            let n = if n.is_finite() {
                n.clamp(0.0, 1.0)
            } else {
                0.0
            };
            *value = n * f32::from(param.max);
        }
        values
    }

    /// Default values normalized to 0..1.
    pub fn normalized_defaults(self) -> [f32; MAX_PARAMS] {
        let mut values = [0.0; MAX_PARAMS];
        for (value, param) in values.iter_mut().zip(self.info().params) {
            *value = if param.max == 0 {
                0.0
            } else {
                f32::from(param.default) / f32::from(param.max)
            };
        }
        values
    }
}
