//! Ba Octave (B_OCTAVE.ZDL): bass octave-down with tone and EQ, mono out.
//!
//! Callback: recovered `bass_octave.rs`. Mono → 200 Hz first-order low-pass
//! → 120 Hz one-pole → detector. Decaying peak trackers (c10) arm a
//! negative-going zero crossing once per period; each trigger toggles a
//! flag, and while the flag is clear the negative half waves are flipped:
//! the ±1 word (smoothed by c15/c16) times the input alternates `x` and
//! `|x|` period by period, which puts an octave below. Three biquads follow
//! (Tone, Low shelf, Mid peak) and `out = c2 (Oct v + Dry mean)` on both
//! channels.
//!
//! Editors decoded: Oct/Dry linear 0..1 with `context[8]` ramps (c33, c34);
//! Tone copies a low-pass biquad from `BA_OCTAVE_TONE`; Low designs Main's
//! first-order low shelf at 120 Hz with 0..15 dB (`context[25]` type 9),
//! Mid a Zölzer peak at 440 Hz, Q 1.5, 0..20 dB (type 10); Level c2.
use super::{
    common::{
        decay, low_shelf, one_pole, out_level, peak, percent, rom_biquad, rom_first_order, step,
    },
    rom::{BA_OCTAVE_ROM, BA_OCTAVE_TONE, f},
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, flush},
    zoom::{DEFAULT_STEP, ORIGINAL_RATE, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_ba_octave",
    name: "Ba Octave",
    category: Category::Pitch,
    zdl: "B_OCTAVE.ZDL",
    description: "Bass octave-down with tone, low and mid boost",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Oct",
            max: 100,
            default: 80,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Dry",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 8,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Low",
            max: 10,
            default: 3,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Mid",
            max: 10,
            default: 4,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

struct BaOctave {
    sample_rate: f32,
    lowpass: Biquad,
    detector_gain: f32,
    detector: f32,
    decay: f32,
    smoothing: f32,
    maximum: f32,
    minimum: f32,
    armed: [bool; 2],
    positive: bool,
    flag: bool,
    voice: f32,
    filters: [Biquad; 3],
    settings: Option<[usize; 3]>,
    oct: Ramp,
    dry: Ramp,
    level: Ramp,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(BaOctave {
        sample_rate,
        lowpass: Biquad::new(rom_first_order(
            &BA_OCTAVE_ROM[5..8],
            ORIGINAL_RATE,
            sample_rate,
        )),
        detector_gain: one_pole(f(BA_OCTAVE_ROM[8]), sample_rate),
        detector: 0.0,
        decay: decay(f(BA_OCTAVE_ROM[10]), sample_rate),
        smoothing: one_pole(f(BA_OCTAVE_ROM[15]), sample_rate),
        maximum: 0.0,
        minimum: 0.0,
        armed: [false; 2],
        positive: false,
        flag: false,
        voice: 0.0,
        filters: [Biquad::default(); 3],
        settings: None,
        oct: Ramp::new(0.8, DEFAULT_STEP, sample_rate),
        dry: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
        level: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
    })
}

impl Effect for BaOctave {
    fn set(&mut self, values: &Values, _bpm: f32) {
        self.oct.set(percent(values[0]));
        self.dry.set(percent(values[1]));
        let settings = [
            step(values[2], 10),
            step(values[3], 10),
            step(values[4], 10),
        ];
        if self.settings != Some(settings) {
            let [tone, low, mid] = settings;
            let fs = self.sample_rate;
            self.filters[0].coefficients =
                rom_biquad(&BA_OCTAVE_TONE[5 * tone..5 * tone + 5], ORIGINAL_RATE, fs);
            self.filters[1].coefficients = low_shelf(fs, 120.0, 1.5 * low as f32);
            self.filters[2].coefficients = peak(fs, 440.0, 2.0 * mid as f32, 1.5);
            self.settings = Some(settings);
        }
        self.level.set(out_level(values[5]));
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mean = 0.5 * (left + right);
        let first = self.lowpass.process(mean);
        self.detector = flush(self.detector + self.detector_gain * (first - self.detector));
        let input = self.detector;
        let upper = input < self.minimum * self.decay;
        let lower = input > self.maximum * self.decay;
        self.minimum = if upper {
            input
        } else {
            self.minimum * self.decay
        };
        self.maximum = if lower {
            input
        } else {
            self.maximum * self.decay
        };
        self.armed[1] |= upper;
        self.armed[0] |= lower;
        let negative = input < 0.0;
        let crossing = negative && self.positive;
        if crossing && self.armed[0] && self.armed[1] {
            self.flag = !self.flag;
        }
        if crossing {
            self.armed = [false; 2];
        }
        self.positive = !negative;
        let sign = if !self.flag && negative { -1.0 } else { 1.0 };
        self.voice = flush(self.voice + self.smoothing * (sign - self.voice));
        let wet = self
            .filters
            .iter_mut()
            .fold(mean * self.voice, |x, filter| filter.process(x));
        let out = self.level.tick() * (self.oct.tick() * wet + self.dry.tick() * mean);
        [out, out]
    }

    fn clear(&mut self) {
        self.lowpass.clear();
        self.detector = 0.0;
        self.maximum = 0.0;
        self.minimum = 0.0;
        self.armed = [false; 2];
        self.positive = false;
        self.flag = false;
        self.voice = 0.0;
        self.filters.iter_mut().for_each(Biquad::clear);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dsp::gain_to_db, fx::pitch::two_tap::tests::centroid};

    #[test]
    fn eq_editors_follow_the_designer() {
        let fs = 48_000.0;
        let low = low_shelf(fs, 120.0, 15.0);
        assert!((gain_to_db(low.magnitude(fs, 5.0)) - 15.0).abs() < 0.2);
        assert!(gain_to_db(low.magnitude(fs, 15_000.0)).abs() < 0.3);
        let mid = peak(fs, 440.0, 20.0, 1.5);
        assert!((gain_to_db(mid.magnitude(fs, 440.0)) - 20.0).abs() < 0.05);
        let tone = rom_biquad(&BA_OCTAVE_TONE[50..55], ORIGINAL_RATE, fs);
        let rom = rom_biquad(&BA_OCTAVE_TONE[50..55], ORIGINAL_RATE, ORIGINAL_RATE);
        let dc = gain_to_db(rom.magnitude(ORIGINAL_RATE, 20.0));
        assert!((gain_to_db(tone.magnitude(fs, 20.0)) - dc).abs() < 0.2);
        assert!(gain_to_db(tone.magnitude(fs, 5_000.0)) < -20.0);
    }

    #[test]
    fn adds_an_octave_below() {
        let sample_rate = 44_100.0;
        let mut effect = create(sample_rate);
        let mut values = crate::Model::BaOctave.defaults();
        values[1] = 0.0;
        values[3] = 0.0;
        values[4] = 0.0;
        values[2] = 10.0;
        effect.set(&values, 120.0);
        let n = 2 * sample_rate as usize;
        let out: Vec<f32> = (0..n)
            .map(|i| {
                let p = i as f32 * 82.4 / sample_rate * std::f32::consts::TAU;
                effect.process([0.4 * p.sin(), 0.4 * p.sin()])[0]
            })
            .collect();
        let found = centroid(&out[n / 2..], sample_rate, 36.0, 46.4);
        assert!((found - 41.2).abs() < 1.0, "{found}");
    }
}
