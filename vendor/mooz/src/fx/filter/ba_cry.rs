//! Ba Cry (B_CRY.ZDL): an envelope-controlled comb with a negative, low-passed
//! feedback loop whose tap slides with the playing level ("crying" vowels).
//!
//! The callback and every editor are ported in `cry_core` (CRY.ZDL and
//! B_CRY.ZDL share the editor code and tables; ROM block at 0x80000558).
//! Ba Cry differs from Cry only in its wet path, which blends the mono input
//! (`c11 m`) instead of each channel.
use super::common::SENSE_LABELS;
use super::cry_core::{CryCore, RANGE_DISPLAY};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_ba_cry",
    name: "Ba Cry",
    category: Category::Filter,
    zdl: "B_CRY.ZDL",
    description: "Bass talk-box envelope comb filter",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Range",
            max: 9,
            default: 4,
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

struct BaCry {
    core: CryCore,
    last: Option<Values>,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = BaCry {
        core: CryCore::new(sample_rate, true),
        last: None,
    };
    let mut values = [0.0; crate::MAX_PARAMS];
    for (value, param) in values.iter_mut().zip(INFO.params) {
        *value = f32::from(param.default);
    }
    effect.set(&values, 120.0);
    Box::new(effect)
}

impl Effect for BaCry {
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
    fn balance_zero_is_the_mono_input() {
        let mut values = Model::BaCry.defaults();
        values[3] = 0.0;
        let mut effect = create(48_000.0);
        effect.set(&values, 120.0);
        for _ in 0..2400 {
            effect.process([0.0, 0.0]);
        }
        let out = effect.process([0.4, 0.2]);
        assert!((out[0] - 0.3).abs() < 1e-6 && (out[1] - 0.3).abs() < 1e-6);
    }

    #[test]
    fn default_output_stays_moderate() {
        let mut effect = create(44_100.0);
        let mut peak = 0.0f32;
        for i in 0..44_100 {
            let x = 0.5 * (i as f32 * 0.02).sin();
            let y = effect.process([x, x]);
            peak = peak.max(y[0].abs());
        }
        assert!(peak > 0.05 && peak < 4.0, "{peak}");
    }
}
