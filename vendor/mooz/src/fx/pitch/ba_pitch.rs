//! Ba Pitch (B_PITCH.ZDL): bass pitch shifter, mono out.
//!
//! Callback: recovered `bass_pitch.rs` ([`super::dual_window`] in latch
//! mode): mono → 27 Hz high-pass biquad (c20..c24) → Tone one-pole (c25/c26)
//! → ring; two triangle-windowed taps whose increments are latched at their
//! resets; the pending c12..c15 words commit per 8-frame block; → 5 kHz
//! low-pass biquad (c27..c31); `out = c2 (c6 mean + c5 voice)` on both
//! channels. ROM biquads are re-derived per rate.
//!
//! Editors decoded:
//! - `pitchUpdata`: ratio from the 26×51 table at 0x80000000, window = period
//!   ×8 halved below 1102 samples (down) or 2866 (up / unison+), then doubled
//!   if under 2874; reset 220.5 (down) or 3087 (up); -50 dB / 16..2205 checks.
//! - `shift_edit` (Shift, Fine): a knob turn mutes c3 over 16 ms, holds the
//!   update at unison for 141 ms (`shift_edit_aft`), then applies the new
//!   setting and fades c3 back in. The port does the same.
//! - `balance_edit`: c6 = two-slope(100 - Bal), c5 = 3 × two-slope(Bal);
//!   `tone_edit`: `DUAL_WINDOW_TONE`; `outLv_edit`: c2 = Level / 100.
//!
//! Approximation: Main's pitch detector is not recovered; [`PitchTracker`]
//! supplies the period and level.
use super::{
    common::{
        Detection, FADE_SECONDS, PitchTracker, SHIFT_LABELS, TimedRamp, one_pole, out_level,
        rom_biquad, shift_ratio, step, two_slope,
    },
    dual_window::{BASS_RESET_UP, DualWindow, RESET_DOWN, Retune, usable, window_length},
    rom::{BA_PITCH_ROM, DUAL_WINDOW_TONE, f},
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, flush},
    zoom::{DEFAULT_STEP, ORIGINAL_RATE, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_ba_pitch",
    name: "Ba Pitch",
    category: Category::Pitch,
    zdl: "B_PITCH.ZDL",
    description: "Bass pitch shifter with period-locked windows",
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
            default: 7,
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
            default: 140,
            display: Display::INT,
        },
    ],
};

/// The 141 ms `shift_edit_aft` timer.
const HOLD_SECONDS: f32 = 0.141;
const UNISON: (usize, usize) = (12, 25);

struct BaPitch {
    sample_rate: f32,
    shifter: DualWindow,
    tracker: PitchTracker,
    input: Biquad,
    output: Biquad,
    tone: f32,
    filtered: f32,
    requested: (usize, usize),
    current: (usize, usize),
    applied: Option<(usize, usize)>,
    hold: u32,
    period: f32,
    mute: TimedRamp,
    dry: Ramp,
    wet: Ramp,
    level: Ramp,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let idle = Retune::new(1.0, 1.0, 0.0, sample_rate);
    Box::new(BaPitch {
        sample_rate,
        // Up taps start at 3087 and an up/down switch can add one 2204 window.
        shifter: DualWindow::new(sample_rate, 0.0, 5_400.0, true, idle),
        tracker: PitchTracker::new(sample_rate),
        input: Biquad::new(rom_biquad(
            &BA_PITCH_ROM[20..25],
            ORIGINAL_RATE,
            sample_rate,
        )),
        output: Biquad::new(rom_biquad(
            &BA_PITCH_ROM[27..32],
            ORIGINAL_RATE,
            sample_rate,
        )),
        tone: 1.0,
        filtered: 0.0,
        requested: (0, 25),
        current: (0, 25),
        applied: None,
        hold: 0,
        period: 0.0,
        mute: TimedRamp::new(1.0, FADE_SECONDS, sample_rate),
        dry: Ramp::new(two_slope(50.0), DEFAULT_STEP, sample_rate),
        wet: Ramp::new(3.0 * two_slope(50.0), DEFAULT_STEP, sample_rate),
        level: Ramp::new(1.4, DEFAULT_STEP, sample_rate),
    })
}

impl BaPitch {
    /// `Fx_MOD_Ba_Pitch_pitchUpdata`.
    fn update(&mut self, detection: Option<Detection>) {
        let changed = self.applied != Some(self.current);
        if let Some(detection) = detection {
            if !changed && !usable(&detection) {
                return;
            }
            self.period = detection.period;
        } else if !changed {
            return;
        }
        let period = if self.period > 2_866.0 || self.period <= 0.0 {
            2_866.0
        } else {
            self.period
        };
        let (shift, fine) = self.current;
        let up = shift > 12 || (shift == 12 && fine > 24);
        let window = window_length(period, if up { 2_866.0 } else { 1_102.0 }, 2_874.0);
        let ratio = shift_ratio(shift, fine);
        let reset = if ratio > 1.0 {
            BASS_RESET_UP
        } else {
            RESET_DOWN
        };
        self.shifter
            .retune(Retune::new(ratio, window, reset, self.sample_rate));
        self.applied = Some(self.current);
    }
}

impl Effect for BaPitch {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let requested = (step(values[0], 25), step(values[3], 50));
        if self.applied.is_none() {
            self.requested = requested;
            self.current = requested;
            self.update(None);
        } else if requested != self.requested {
            self.requested = requested;
            self.current = UNISON;
            self.hold = (HOLD_SECONDS * self.sample_rate) as u32;
            self.mute.set(0.0);
            self.update(None);
        }
        self.tone = one_pole(f(DUAL_WINDOW_TONE[step(values[1], 10)]), self.sample_rate);
        self.dry.set(two_slope(100.0 - values[2]));
        self.wet.set(3.0 * two_slope(values[2]));
        self.level.set(out_level(values[4]));
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        if self.hold > 0 {
            self.hold -= 1;
            if self.hold == 0 {
                self.current = self.requested;
                self.mute.set(1.0);
                self.update(None);
            }
        }
        let mean = 0.5 * (left + right);
        if self.tracker.push(mean) {
            self.update(Some(self.tracker.detection));
        }
        let cut = self.input.process(mean);
        self.filtered = flush(self.filtered + self.tone * (cut - self.filtered));
        let mute = self.mute.tick();
        let voice = self
            .output
            .process(self.shifter.process(self.filtered) * mute);
        let out = self.level.tick() * (self.dry.tick() * mean + self.wet.tick() * voice);
        [out, out]
    }

    fn clear(&mut self) {
        self.shifter.clear();
        self.tracker.clear();
        self.input.clear();
        self.output.clear();
        self.filtered = 0.0;
        if self.hold > 0 {
            self.hold = 0;
            self.current = self.requested;
            self.update(None);
        }
        self.mute.jump(1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dsp::gain_to_db, fx::pitch::two_tap::tests::centroid};

    #[test]
    fn rom_filters_are_a_27_hz_high_pass_and_a_5_khz_low_pass() {
        let high = rom_biquad(&BA_PITCH_ROM[20..25], ORIGINAL_RATE, 48_000.0);
        assert!(gain_to_db(high.magnitude(48_000.0, 10.0)) < -12.0);
        assert!(gain_to_db(high.magnitude(48_000.0, 100.0)).abs() < 0.3);
        let low = rom_biquad(&BA_PITCH_ROM[27..32], ORIGINAL_RATE, 96_000.0);
        assert!((gain_to_db(low.magnitude(96_000.0, 5_000.0)) + 3.0).abs() < 0.3);
        assert!(gain_to_db(low.magnitude(96_000.0, 100.0)).abs() < 0.05);
    }

    #[test]
    fn default_is_an_octave_down() {
        let sample_rate = 44_100.0;
        let mut effect = create(sample_rate);
        let mut values = crate::Model::BaPitch.defaults();
        values[2] = 100.0;
        values[1] = 10.0;
        effect.set(&values, 120.0);
        let n = 2 * sample_rate as usize;
        let out: Vec<f32> = (0..n)
            .map(|i| {
                let p = i as f32 * 110.0 / sample_rate * std::f32::consts::TAU;
                effect.process([0.3 * p.sin(), 0.3 * p.sin()])[0]
            })
            .collect();
        let found = centroid(&out[n / 2..], sample_rate, 48.0, 62.0);
        assert!((found - 55.0).abs() < 1.0, "{found}");
    }

    #[test]
    fn a_shift_turn_mutes_and_holds() {
        let sample_rate = 48_000.0;
        let mut effect = create(sample_rate);
        let mut values = crate::Model::BaPitch.defaults();
        values[2] = 100.0;
        effect.set(&values, 120.0);
        let mut quiet = 0;
        values[0] = 24.0;
        effect.set(&values, 120.0);
        for i in 0..(sample_rate as usize / 4) {
            let p = i as f32 * 110.0 / sample_rate * std::f32::consts::TAU;
            let y = effect.process([0.3 * p.sin(), 0.3 * p.sin()])[0];
            if i > 1_000 && i < 6_000 {
                // Only the dry 0.0 (Bal 100) is left while muted.
                quiet += usize::from(y.abs() < 1e-4);
            }
        }
        assert!(quiet > 4_900, "{quiet}");
    }
}
