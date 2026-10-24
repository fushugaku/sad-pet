//! Cry (CRY.ZDL): an envelope-controlled comb with a negative, low-passed
//! feedback loop whose tap slides with the playing level ("crying" vowels).
//!
//! The callback and every editor are ported in `cry_core` (CRY.ZDL and
//! B_CRY.ZDL share the editor code and tables; ROM block at 0x80000568).
//! Cry blends each input channel (`c11 L`, `c11 R`) into its wet path.
use super::common::SENSE_LABELS;
use super::cry_core::{CryCore, RANGE_DISPLAY};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_cry",
    name: "Cry",
    category: Category::Filter,
    zdl: "CRY.ZDL",
    description: "Talk-box style envelope comb filter",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Range",
            max: 9,
            default: 6,
            display: RANGE_DISPLAY,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Reso",
            max: 10,
            default: 8,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Sense",
            max: 19,
            default: 16,
            display: Display::labels(&SENSE_LABELS),
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Bal",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

struct Cry {
    core: CryCore,
    last: Option<Values>,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = Cry {
        core: CryCore::new(sample_rate, false),
        last: None,
    };
    let mut values = [0.0; crate::MAX_PARAMS];
    for (value, param) in values.iter_mut().zip(INFO.params) {
        *value = f32::from(param.default);
    }
    effect.set(&values, 120.0);
    Box::new(effect)
}

impl Effect for Cry {
    fn set(&mut self, values: &Values, _bpm: f32) {
        if self.last.as_ref() == Some(values) {
            return;
        }
        self.last = Some(*values);
        self.core.set(values);
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.core.process(frame)
    }

    fn clear(&mut self) {
        self.core.clear();
        self.last = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    #[test]
    fn balance_zero_is_the_dry_signal_per_channel() {
        let mut values = Model::Cry.defaults();
        values[3] = 0.0;
        let mut effect = create(48_000.0);
        effect.set(&values, 120.0);
        for _ in 0..2400 {
            effect.process([0.0, 0.0]);
        }
        for i in 0..4800 {
            let l = 0.3 * (i as f32 * 0.01).sin();
            let r = -0.2 * (i as f32 * 0.013).sin();
            let out = effect.process([l, r]);
            assert!((out[0] - l).abs() < 1e-6 && (out[1] - r).abs() < 1e-6);
        }
    }
}
