//! HPS (HPS.ZDL): intelligent harmonizer, one voice a diatonic interval away.
//!
//! Callback: the recovered HPS/MNPIT graph ([`super::dual_window`]): mono →
//! Tone one-pole (c22/c23, the pre/post biquads stay at their ROM identity) →
//! ring; two pitch-synchronous triangle-windowed taps; `out = c2 (in + c5
//! voice)` (HPS init sets the dry c6 = 1).
//!
//! Editors decoded: `pitchUpdata` (the core of the model, called by Main after
//! each detection and by `scale_edit`/`key_edit`): the played note relative to
//! Key picks the interval from the 10×12 scale table at 0x80000288, the ratio
//! is `2^(interval/12)` (table 0x80000758), the window is the detected period
//! ×8 halved below 1102 samples, reset 1323 (up) or 220.5 (down), and it gives
//! up below -50 dB or outside 16..2205 samples unless Scale/Key just changed.
//! `mix_edit`: c5 = Mix/100; `tone_edit`: `DUAL_WINDOW_TONE`; `outLv_edit`.
//! Until a note is detected the ROM words stand (+4 semitones), as on the
//! pedal.
//!
//! Approximation: Main's pitch detector is not recovered; [`PitchTracker`]
//! (YIN, 11.6 ms hop) supplies the period, the note and the level.
use super::{
    common::{Detection, PitchTracker, one_pole, out_level, percent, step},
    dual_window::{DualWindow, RESET_DOWN, RESET_UP, Retune, usable, window_length},
    rom::{DUAL_WINDOW_TONE, f},
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::flush,
    zoom::{DEFAULT_STEP, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_hps",
    name: "HPS",
    category: Category::Pitch,
    zdl: "HPS.ZDL",
    description: "Harmonizer: diatonic interval voice in a set key",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Scale",
            max: 9,
            default: 6,
            display: Display::labels(&["-6", "-5", "-4", "-3", "-m", "m", "3", "4", "5", "6"]),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Key",
            max: 11,
            default: 0,
            display: Display::labels(&[
                "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
            ]),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 70,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 6,
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

/// HPS 0x80000288: semitones per Scale for each note above the Key.
const SCALES: [[i8; 12]; 10] = [
    [-8, -8, -9, -9, -9, -8, -8, -8, -8, -9, -9, -9],
    [-7, -7, -7, -7, -7, -6, -6, -7, -7, -7, -7, -7],
    [-5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -6],
    [-3, -3, -3, -4, -4, -3, -3, -3, -4, -4, -4, -4],
    [-4, -4, -4, -3, -3, -3, -4, -4, -3, -3, -3, -4],
    [3, 4, 3, 4, 3, 3, 3, 3, 4, 3, 4, 3],
    [4, 4, 3, 3, 3, 4, 4, 4, 4, 3, 3, 3],
    [5, 5, 5, 5, 5, 6, 6, 5, 5, 5, 5, 5],
    [7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 6],
    [9, 9, 9, 9, 8, 9, 9, 9, 9, 8, 9, 8],
];

/// The interval HPS plays against `note` (C = 0) in `scale` and `key`.
fn interval(scale: usize, key: usize, note: u8) -> i8 {
    SCALES[scale][(usize::from(note) + 12 - key) % 12]
}

/// ROM c8/c9 (0x0011ee40, 0xffff7aeb): +4 semitones over a 3655-sample window.
pub(super) fn rom_retune(sample_rate: f32) -> Retune {
    let ratio = 1.0 + 34_069.0 / 131_072.0;
    let window = 1_175_104.0f32 / 4_294_967_296.0;
    Retune::new(ratio, (ratio - 1.0) / window, RESET_UP, sample_rate)
}

/// HPS/MNPIT taps reach 1000 + 1323 + 1110 samples at most.
pub(super) const LONGEST: f32 = 3_600.0;

/// The ROM state words: phase 0xffe96d00, taps 0x0a560000 and 0x052b0000.
pub(super) fn rom_shifter(sample_rate: f32) -> DualWindow {
    let mut shifter = DualWindow::new(
        sample_rate,
        1_000.0,
        LONGEST,
        false,
        rom_retune(sample_rate),
    );
    let scale = f64::from(sample_rate) / 44_100.0;
    shifter.set_phase(0xffe9_6d00, [1_323.0 * scale, 661.5 * scale]);
    shifter
}

struct Hps {
    sample_rate: f32,
    shifter: DualWindow,
    tracker: PitchTracker,
    tone: f32,
    filtered: f32,
    scale: usize,
    key: usize,
    applied: Option<(usize, usize)>,
    wet: Ramp,
    level: Ramp,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Hps {
        sample_rate,
        shifter: rom_shifter(sample_rate),
        tracker: PitchTracker::new(sample_rate),
        tone: 1.0,
        filtered: 0.0,
        scale: 6,
        key: 0,
        applied: None,
        wet: Ramp::new(0.7, DEFAULT_STEP, sample_rate),
        level: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
    })
}

impl Hps {
    /// `Fx_MOD_HPS_pitchUpdata`.
    fn update(&mut self, detection: Detection) {
        let changed = self.applied != Some((self.scale, self.key));
        if !changed && !usable(&detection) {
            return;
        }
        let Some(note) = detection.note else {
            return;
        };
        let interval = interval(self.scale, self.key, note);
        let period = if detection.period > 1_102.0 || detection.period <= 0.0 {
            1_102.0
        } else {
            detection.period
        };
        let window = window_length(period, 1_102.0, 1_110.0);
        let reset = if interval > 0 { RESET_UP } else { RESET_DOWN };
        let ratio = (f32::from(interval) / 12.0).exp2();
        self.shifter
            .retune(Retune::new(ratio, window, reset, self.sample_rate));
        self.applied = Some((self.scale, self.key));
    }
}

impl Effect for Hps {
    fn set(&mut self, values: &Values, _bpm: f32) {
        self.scale = step(values[0], 9);
        self.key = step(values[1], 11);
        if self
            .applied
            .is_some_and(|applied| applied != (self.scale, self.key))
        {
            self.update(self.tracker.detection);
        }
        self.wet.set(percent(values[2]));
        self.tone = one_pole(f(DUAL_WINDOW_TONE[step(values[3], 10)]), self.sample_rate);
        self.level.set(out_level(values[4]));
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mean = 0.5 * (left + right);
        if self.tracker.push(mean) {
            self.update(self.tracker.detection);
        }
        self.filtered = flush(self.filtered + self.tone * (mean - self.filtered));
        let voice = self.shifter.process(self.filtered) * self.wet.tick();
        let level = self.level.tick();
        [level * (left + voice), level * (right + voice)]
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

    #[test]
    fn scale_tables_are_diatonic() {
        // A third up in C major: C→E, D→F, E→G, B→D.
        assert_eq!(interval(6, 0, 0), 4);
        assert_eq!(interval(6, 0, 2), 3);
        assert_eq!(interval(6, 0, 4), 3);
        assert_eq!(interval(6, 0, 11), 3);
        // The same note in G: B is the third degree there (B→D).
        assert_eq!(interval(6, 7, 11), 3);
        assert_eq!(interval(6, 7, 7), 4);
    }

    #[test]
    fn harmonizes_a_played_note() {
        let sample_rate = 48_000.0;
        let mut effect = create(sample_rate);
        let mut values = crate::Model::Hps.defaults();
        values[2] = 100.0;
        values[3] = 10.0;
        effect.set(&values, 120.0);
        // A C (261.63 Hz) in C major, Scale "3": the voice is E (329.63 Hz).
        let hz = 261.63;
        let n = 2 * sample_rate as usize;
        let out: Vec<f32> = (0..n)
            .map(|i| {
                let p = i as f32 * hz / sample_rate * std::f32::consts::TAU;
                let x = 0.3 * p.sin() + 0.1 * (2.0 * p).sin();
                effect.process([x, x])[0] - x
            })
            .collect();
        let found = centroid(&out[n / 2..], sample_rate, 300.0, 360.0);
        assert!((found - 329.63).abs() < 2.0, "{found}");
    }
}
