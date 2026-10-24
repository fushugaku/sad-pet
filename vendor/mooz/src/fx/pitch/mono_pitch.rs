//! MonoPitch (MNPIT.ZDL): pitch shifter for single notes, with windows locked
//! to the played pitch period.
//!
//! Callback: the recovered HPS/MNPIT graph ([`super::dual_window`]), mono →
//! Tone one-pole → ring, two triangle-windowed taps, `out = c2 (c6 in + c5
//! voice)`. Editors decoded:
//! - `pitchUpdata` (Main after each detection; `shift_edit` for Shift and
//!   Fine): ratio from the 26×51 table at 0x80000000 (`2^((shift +
//!   (fine-25)/100)/12)`), window = detected period ×8 halved below 1102
//!   samples, reset 1323/220.5 for up/down. Without a fresh Shift/Fine it
//!   needs a level above -50 dB and a period of 16..2205 samples; with none
//!   detected yet the window is 1102 samples.
//! - `balance_edit`: c6 = two-slope(100 - Bal), c5 = two-slope(Bal);
//!   `tone_edit`: `DUAL_WINDOW_TONE`; `outLv_edit`: c2 = Level / 100.
//!
//! Approximation: Main's pitch detector is not recovered; [`PitchTracker`]
//! supplies the period and level.
use super::{
    common::{
        Detection, PitchTracker, SHIFT_LABELS, one_pole, out_level, shift_ratio, step, two_slope,
    },
    dual_window::{DualWindow, RESET_DOWN, RESET_UP, Retune, usable, window_length},
    hps::rom_shifter,
    rom::{DUAL_WINDOW_TONE, f},
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::flush,
    zoom::{DEFAULT_STEP, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_mono_pitch",
    name: "MonoPitch",
    category: Category::Pitch,
    zdl: "MNPIT.ZDL",
    description: "Low-artifact pitch shifter for single notes",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Shift",
            max: 25,
            default: 0,
            display: Display::labels(SHIFT_LABELS),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 6,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Bal",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Fine",
            max: 50,
            default: 25,
            display: Display::offset(-25.0),
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

struct MonoPitch {
    sample_rate: f32,
    shifter: DualWindow,
    tracker: PitchTracker,
    tone: f32,
    filtered: f32,
    shift: (usize, usize),
    applied: Option<(usize, usize)>,
    period: f32,
    dry: Ramp,
    wet: Ramp,
    level: Ramp,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(MonoPitch {
        sample_rate,
        shifter: rom_shifter(sample_rate),
        tracker: PitchTracker::new(sample_rate),
        tone: 1.0,
        filtered: 0.0,
        shift: (0, 25),
        applied: None,
        period: 0.0,
        dry: Ramp::new(two_slope(50.0), DEFAULT_STEP, sample_rate),
        wet: Ramp::new(two_slope(50.0), DEFAULT_STEP, sample_rate),
        level: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
    })
}

impl MonoPitch {
    /// `Fx_MOD_MonoPitch_pitchUpdata`. The period cell keeps Main's last
    /// report, clamped to 1102 when it is longer or empty.
    fn update(&mut self, detection: Option<Detection>) {
        let changed = self.applied != Some(self.shift);
        if let Some(detection) = detection {
            if !changed && !usable(&detection) {
                return;
            }
            self.period = detection.period;
        } else if !changed {
            return;
        }
        if self.period > 1_102.0 || self.period <= 0.0 {
            self.period = 1_102.0;
        }
        let window = window_length(self.period, 1_102.0, 1_110.0);
        let ratio = shift_ratio(self.shift.0, self.shift.1);
        let reset = if ratio > 1.0 { RESET_UP } else { RESET_DOWN };
        self.shifter
            .retune(Retune::new(ratio, window, reset, self.sample_rate));
        self.applied = Some(self.shift);
    }
}

impl Effect for MonoPitch {
    fn set(&mut self, values: &Values, _bpm: f32) {
        self.shift = (step(values[0], 25), step(values[3], 50));
        if self.applied != Some(self.shift) {
            self.update(None);
        }
        self.tone = one_pole(f(DUAL_WINDOW_TONE[step(values[1], 10)]), self.sample_rate);
        self.dry.set(two_slope(100.0 - values[2]));
        self.wet.set(two_slope(values[2]));
        self.level.set(out_level(values[4]));
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mean = 0.5 * (left + right);
        if self.tracker.push(mean) {
            self.update(Some(self.tracker.detection));
        }
        self.filtered = flush(self.filtered + self.tone * (mean - self.filtered));
        let voice = self.shifter.process(self.filtered) * self.wet.tick();
        let (dry, level) = (self.dry.tick(), self.level.tick());
        [level * (dry * left + voice), level * (dry * right + voice)]
    }

    fn clear(&mut self) {
        self.shifter.clear();
        self.tracker.clear();
        self.filtered = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::pitch::two_tap::tests::centroid;

    fn voice(shift: f32, hz: f32, sample_rate: f32) -> f32 {
        let ratio = shift_ratio(shift as usize, 25);
        let mut effect = create(sample_rate);
        let mut values = crate::Model::MonoPitch.defaults();
        values[0] = shift;
        values[1] = 10.0;
        values[2] = 100.0;
        effect.set(&values, 120.0);
        let n = 2 * sample_rate as usize;
        let out: Vec<f32> = (0..n)
            .map(|i| {
                let p = i as f32 * hz / sample_rate * std::f32::consts::TAU;
                effect.process([0.4 * p.sin(), 0.4 * p.sin()])[0]
            })
            .collect();
        centroid(
            &out[n / 2..],
            sample_rate,
            hz * ratio * 0.88,
            hz * ratio * 1.12,
        )
    }

    #[test]
    fn shifts_by_the_shift_step() {
        let found = voice(0.0, 330.0, 44_100.0);
        assert!((found - 165.0).abs() < 1.5, "{found}");
        let found = voice(19.0, 220.0, 96_000.0);
        assert!((found - 220.0 * 1.4983).abs() < 2.0, "{found}");
        let found = voice(25.0, 196.0, 48_000.0);
        assert!((found - 784.0).abs() < 4.0, "{found}");
    }

    #[test]
    fn window_follows_the_detected_period() {
        let mut effect = MonoPitch {
            sample_rate: 44_100.0,
            shifter: rom_shifter(44_100.0),
            tracker: PitchTracker::new(44_100.0),
            tone: 1.0,
            filtered: 0.0,
            shift: (24, 25),
            applied: None,
            period: 0.0,
            dry: Ramp::new(0.0, DEFAULT_STEP, 44_100.0),
            wet: Ramp::new(1.0, DEFAULT_STEP, 44_100.0),
            level: Ramp::new(1.0, DEFAULT_STEP, 44_100.0),
        };
        effect.update(None);
        assert_eq!(effect.period, 1_102.0);
        effect.update(Some(Detection {
            period: 200.0,
            note: Some(9),
            level: 0.1,
        }));
        assert_eq!(effect.period, 200.0);
        // Quiet input leaves the window alone.
        effect.update(Some(Detection {
            period: 300.0,
            note: Some(9),
            level: 0.001,
        }));
        assert_eq!(effect.period, 200.0);
    }
}
