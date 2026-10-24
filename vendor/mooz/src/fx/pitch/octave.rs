//! Octave (OCTAVE.ZDL): analog-style octave divider, one and two octaves down.
//!
//! Callback: recovered `octave.rs`. Mono → detector chain from the ROM
//! (30 Hz high-pass, 150/300 Hz +6 dB shelf, two 1.6 kHz low-passes) →
//! peak trackers decaying by c24 arm a zero-crossing flip-flop once per
//! period (both a new maximum and a new minimum must be seen). Divider 1
//! toggles every period (f/2), divider 2 every second toggle (f/4). Each
//! divider's square (+1 or −Chara) is smoothed by a 0.05 one-pole and
//! multiplies the detector signal (voice 1) or voice 1 (voice 2), each
//! through the Tone one-pole. `out = c2 (Dry in + Oct1 v1 + Oct2 v2)`.
//!
//! Editors decoded: Oct1/Oct2/Dry/Chara are linear 0..1 through the 16 ms
//! `context[7]` ramp (c22, c23, c21, c25); Tone copies a c19/c20 pair from
//! `OCTAVE_TONE`; OutLv is c2 = Level / 100. ROM filters are re-derived per
//! rate, per-sample decays rescaled.
use super::{
    common::{FADE_SECONDS, TimedRamp, decay, one_pole, out_level, percent, rom_first_order, step},
    rom::{OCTAVE_ROM, OCTAVE_TONE, f},
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, flush},
    zoom::{DEFAULT_STEP, ORIGINAL_RATE, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_octave",
    name: "Octave",
    category: Category::Pitch,
    zdl: "OCTAVE.ZDL",
    description: "Octaver: one and two octaves below the input",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Oct1",
            max: 100,
            default: 80,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Oct2",
            max: 100,
            default: 15,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Dry",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Chara",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 8,
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

/// The crossing divider shared with MonoSynth's detector logic in spirit:
/// the OCTAVE detector chain (c5..c16) at the instance rate.
pub(super) fn detector_chain(rom: &[u32], sample_rate: f32) -> [Biquad; 4] {
    std::array::from_fn(|stage| {
        Biquad::new(rom_first_order(
            &rom[5 + 3 * stage..8 + 3 * stage],
            ORIGINAL_RATE,
            sample_rate,
        ))
    })
}

struct Octave {
    sample_rate: f32,
    detector: [Biquad; 4],
    decay: f32,
    smoothing: f32,
    tone: f32,
    maximum: f32,
    minimum: f32,
    armed: [bool; 2],
    negative: bool,
    dividers: [bool; 2],
    excitation: [f32; 2],
    smoothed: [f32; 2],
    voices: [f32; 2],
    oct1: TimedRamp,
    oct2: TimedRamp,
    dry: TimedRamp,
    character: TimedRamp,
    level: Ramp,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let fade = |value| TimedRamp::new(value, FADE_SECONDS, sample_rate);
    Box::new(Octave {
        sample_rate,
        detector: detector_chain(&OCTAVE_ROM, sample_rate),
        decay: decay(f(OCTAVE_ROM[24]), sample_rate),
        smoothing: one_pole(f(OCTAVE_ROM[17]), sample_rate),
        tone: 1.0,
        maximum: 0.0,
        minimum: 0.0,
        armed: [false; 2],
        negative: false,
        dividers: [false; 2],
        excitation: [0.0; 2],
        smoothed: [0.0; 2],
        voices: [0.0; 2],
        oct1: fade(0.8),
        oct2: fade(0.15),
        dry: fade(1.0),
        character: fade(1.0),
        level: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
    })
}

impl Effect for Octave {
    fn set(&mut self, values: &Values, _bpm: f32) {
        self.oct1.set(percent(values[0]));
        self.oct2.set(percent(values[1]));
        self.dry.set(percent(values[2]));
        self.character.set(percent(values[3]));
        let tone = f(OCTAVE_TONE[2 * step(values[4], 10)]);
        self.tone = one_pole(tone, self.sample_rate);
        self.level.set(out_level(values[5]));
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mean = 0.5 * (left + right);
        let input = self
            .detector
            .iter_mut()
            .fold(mean, |x, stage| stage.process(x));
        let character = self.character.tick();
        let negative = input < 0.0;
        if input > self.maximum {
            self.maximum = input;
            self.armed[0] = true;
        } else {
            self.maximum *= self.decay;
        }
        if input < self.minimum {
            self.minimum = input;
            self.armed[1] = true;
        } else {
            self.minimum *= self.decay;
        }
        if negative != self.negative && self.armed[0] && self.armed[1] {
            // Divider words start at +1.0; a set flag means negative.
            self.dividers[0] = !self.dividers[0];
            if self.dividers[0] {
                self.dividers[1] = !self.dividers[1];
            }
            let square = |negative: bool| if negative { -character } else { 1.0 };
            self.excitation = [square(self.dividers[0]), square(self.dividers[1])];
            self.armed = [false; 2];
        }
        self.negative = negative;
        let s = self.smoothing;
        self.smoothed[0] = flush(self.smoothed[0] + s * (self.excitation[0] - self.smoothed[0]));
        self.smoothed[1] = flush(self.smoothed[1] + s * (self.excitation[1] - self.smoothed[1]));
        let t = self.tone;
        let first = input * self.smoothed[0];
        self.voices[0] = flush(self.voices[0] + t * (first - self.voices[0]));
        let second = self.voices[0] * self.smoothed[1];
        self.voices[1] = flush(self.voices[1] + t * (second - self.voices[1]));
        let wet = self.oct1.tick() * self.voices[0] + self.oct2.tick() * self.voices[1];
        let (dry, level) = (self.dry.tick(), self.level.tick());
        [level * (dry * left + wet), level * (dry * right + wet)]
    }

    fn clear(&mut self) {
        self.detector.iter_mut().for_each(Biquad::clear);
        self.maximum = 0.0;
        self.minimum = 0.0;
        self.armed = [false; 2];
        self.negative = false;
        self.dividers = [false; 2];
        self.excitation = [0.0; 2];
        self.smoothed = [0.0; 2];
        self.voices = [0.0; 2];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dsp::gain_to_db, fx::pitch::two_tap::tests::centroid};

    #[test]
    fn detector_chain_is_a_band_around_the_guitar_range() {
        for sample_rate in [44_100.0f32, 96_000.0] {
            let chain = detector_chain(&OCTAVE_ROM, sample_rate);
            let response = |hz: f32| {
                chain
                    .iter()
                    .map(|b| b.coefficients.magnitude(sample_rate, hz))
                    .product::<f32>()
            };
            assert!(gain_to_db(response(10.0)) < -8.0);
            assert!(gain_to_db(response(150.0)).abs() < 2.0);
            // Two 1.6 kHz poles: -6 dB at the corner, plus the shelf's +6 dB.
            assert!(
                gain_to_db(response(1_600.0)).abs() < 1.5,
                "{}",
                response(1_600.0)
            );
            assert!(gain_to_db(response(8_000.0)) < -15.0);
        }
    }

    #[test]
    fn divides_the_input_frequency() {
        let sample_rate = 48_000.0;
        for (oct1, oct2, low, high, expected) in [
            (100.0, 0.0, 90.0, 130.0, 110.0),
            (0.0, 100.0, 45.0, 65.0, 55.0),
        ] {
            let mut effect = create(sample_rate);
            let mut values = crate::Model::Octave.defaults();
            values[0] = oct1;
            values[1] = oct2;
            values[2] = 0.0;
            values[4] = 10.0;
            effect.set(&values, 120.0);
            let n = 2 * sample_rate as usize;
            let out: Vec<f32> = (0..n)
                .map(|i| {
                    let p = i as f32 * 220.0 / sample_rate * std::f32::consts::TAU;
                    effect.process([0.4 * p.sin(), 0.4 * p.sin()])[0]
                })
                .collect();
            let found = centroid(&out[n / 2..], sample_rate, low, high);
            assert!((found - expected).abs() < 2.0, "{found} vs {expected}");
            let rms = (out[n / 2..].iter().map(|x| x * x).sum::<f32>() / (n / 2) as f32).sqrt();
            assert!(rms > 0.01, "{rms}");
        }
    }
}
