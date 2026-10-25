//! SlwAtkDly (SLWATKDL.ZDL): a slow-attack (swell) stage feeding a modulated
//! stereo delay, after the recovered `Fx_DLY_SlwAtkDly` callback and its fully
//! recovered editors (`slow_attack_delay/parameters.rs`).
//!
//! Signal flow, as in the callback:
//! - Onset detector on the dry input (mono mid here, the left dry buffer in the
//!   original), run twice per frame: 200 Hz one-pole high-pass, peak hold with
//!   a 0.19 s release, a fast (0.67 ms) and a slow (11 ms) follower of the
//!   peak; when `0.64·fast > slow` an attack is detected and the swell restarts
//!   from 0, otherwise it rises by `1/(SWELL seconds)` per iteration.
//! - The input is delayed by 470 samples (10.7 ms look-ahead) and multiplied by
//!   the swell polynomial `c9·e + c10·e² + c11·e⁴ + c12·e⁶ + c13·e⁸` (the five
//!   50-word SWELL ROM tables), smoothed by a 0.67 ms one-pole.
//! - The swelled signal feeds a 1.95 s stereo delay with `F.B·tapB` feedback.
//!   Tap A reads the glided TIME, tap B adds `DEPTH·(½tri₁ + ½tri₂)` from two
//!   triangle LFOs (1.15 and 4.29 Hz). Mode Off sums both taps per channel with
//!   identical L/R LFOs; Mode On uses tap B only with right LFOs detuned by
//!   0.19 Hz and a 0.87/0.13 cross-mix.
//! - Output `wet·c64 + swelled·c63` (MIX: wet `0.9·(p/77)²` up to 77, then the
//!   swelled dry falls as `1 − √t` to 0 at 100).
//!
//! Editors: TIME `(p+1)` ms or tempo notes (max 1900 ms), F.B `(p/100)²`,
//! DEPTH `0x2461FD·(p/100)²` Q14 samples (3.3 ms), SWELL three curve segments
//! (0.11–0.33 s square, 0.33–0.9 s, 0.9–4 s) and the ROM polynomial, Mode with
//! the editor's fade-out/switch/fade-in. Deviation: a TIME change glides the
//! read position (the callback's own 0.12 s smoother, as the original does for
//! tempo changes) instead of the knob path's mute → jump → reopen, so
//! modulating TIME stays audible. The feedback write passes a soft safety
//! limiter above ±2 (the callback has none and 100 % feedback is unity).
use super::common::{
    DelayLine, DurationRamp, Glide, MuteSwap, Q31, RAMP_SAMPLES, Settled, Stage1, flush,
    one_pole_at, pole_at, rom_at, safety, samples, time_ms,
};
use crate::{
    Effect, Values,
    catalog::*,
    zoom::{DEFAULT_STEP, Ramp, curve, is_synced},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_slw_atk_dly",
    name: "SlwAtkDly",
    category: Category::Delay,
    zdl: "SLWATKDL.ZDL",
    description: "Swelled attacks into a modulated stereo delay",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "TIME",
            max: 1914,
            default: 489,
            display: Display::DelayTime {
                threshold: 1899,
                offset: 1,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 71,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "MIX",
            max: 100,
            default: 64,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "DEPTH",
            max: 100,
            default: 77,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "SWELL",
            max: 49,
            default: 24,
            display: Display::offset(1.0),
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Mode",
            max: 1,
            default: 0,
            display: Display::OFF_ON,
        },
    ],
};

const MAX_MS: f32 = 1900.0;
/// The detector and swell run two iterations per 44.1 kHz frame.
const DETECTOR_RATE: f32 = 88_200.0;

/// SWELL ROM tables (`Fx_DLY_SlwAtkDly` editor, 50 words each), coefficients
/// of e, e², e⁴, e⁶ and e⁸.
const SWELL: [[u32; 50]; 5] = [
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0x3d9d89d9, 0x3e1d89d9, 0x3e6c4ec5, 0x3e9d89d9, 0x3ec4ec4f,
        0x3eec4ec5, 0x3f09d89e, 0x3f1d89d9, 0x3f313b14, 0x3f44ec4f, 0x3f589d8a, 0x3f6c4ec5,
        0x3f800000,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x3daaaaab,
        0x3e2aaaab, 0x3e800000, 0x3eaaaaab, 0x3ed55555, 0x3f000000, 0x3f155555, 0x3f2aaaab,
        0x3f400000, 0x3f555555, 0x3f6aaaab, 0x3f800000, 0x3f6c4ec5, 0x3f589d8a, 0x3f44ec4f,
        0x3f313b14, 0x3f1d89d9, 0x3f09d89e, 0x3eec4ec5, 0x3ec4ec4f, 0x3e9d89d9, 0x3e6c4ec5,
        0x3e1d89d9, 0x3d9d89d9, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x3daaaaab, 0x3e2aaaab, 0x3e800000, 0x3eaaaaab,
        0x3ed55555, 0x3f000000, 0x3f155555, 0x3f2aaaab, 0x3f400000, 0x3f555555, 0x3f6aaaab,
        0x3f800000, 0x3f6aaaab, 0x3f555555, 0x3f400000, 0x3f2aaaab, 0x3f155555, 0x3f000000,
        0x3ed55555, 0x3eaaaaab, 0x3e800000, 0x3e2aaaab, 0x3daaaaab, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0, 0x3daaaaab, 0x3e2aaaab, 0x3e800000, 0x3eaaaaab, 0x3ed55555, 0x3f000000, 0x3f155555,
        0x3f2aaaab, 0x3f400000, 0x3f555555, 0x3f6aaaab, 0x3f800000, 0x3f6aaaab, 0x3f555555,
        0x3f400000, 0x3f2aaaab, 0x3f155555, 0x3f000000, 0x3ed55555, 0x3eaaaaab, 0x3e800000,
        0x3e2aaaab, 0x3daaaaab, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0x3f800000, 0x3f6aaaab, 0x3f555555, 0x3f400000, 0x3f2aaaab, 0x3f155555, 0x3f000000,
        0x3ed55555, 0x3eaaaaab, 0x3e800000, 0x3e2aaaab, 0x3daaaaab, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
];

/// LFO increments (C37..C44) and the right-channel detune (C35/C36) of the
/// 44.1 kHz callback, u32 phase per sample.
const LFO_SLOW: u32 = 0x0001_b6f6;
const LFO_FAST: u32 = 0x0006_5e70;
const LFO_DETUNE: u32 = 0x0000_49f1;

/// SWELL time in seconds (three `context[21]` curve segments).
pub fn swell_seconds(p: u32) -> f32 {
    let p = p.min(49);
    if p < 25 {
        curve(0.33, 0.11, 25, p as f32, 1)
    } else if p < 38 {
        curve(0.9, 0.33, 13, (p - 25) as f32, 0)
    } else {
        curve(4.0, 0.9, 12, (p - 38) as f32, 0)
    }
}

/// MIX → (wet, swelled dry).
pub fn mix_gains(p: f32) -> (f32, f32) {
    let p = p.round().clamp(0.0, 100.0);
    if p < 77.0 {
        (curve(0.9, 0.0, 77, p, 1), 1.0)
    } else {
        (0.9, curve(0.0, 1.0, 24, p - 77.0, 3))
    }
}

struct SlwAtkDly {
    sample_rate: f32,
    // detector (two iterations per frame)
    detector_hp: Stage1,
    peak_decay: f32,
    fast: (f32, f32),
    slow: (f32, f32),
    peak: f32,
    fast_env: f32,
    slow_env: f32,
    swell: f32,
    swell_step: f32,
    swell_poly: [f32; 5],
    gain_smooth: (f32, f32),
    gain: f32,
    // delays
    short: [DelayLine; 2],
    /// The short lines' 470-sample (at 44.1 kHz) tap, prepared for
    /// `read_fixed`.
    short_tap: (usize, f32),
    long: [DelayLine; 2],
    feedback_tap: [f32; 2],
    time: Glide,
    depth: Glide,
    lfo: [u32; 4],
    increments: [[u32; 4]; 2],
    alternate: bool,
    mode_fade: MuteSwap,
    pending_alternate: bool,
    feedback: Ramp,
    wet: DurationRamp,
    dry: DurationRamp,
    last: [f32; 6],
    bpm: f32,
}

impl SlwAtkDly {
    fn new(sample_rate: f32) -> Self {
        let detector_rate = 2.0 * sample_rate;
        let max_delay = (MAX_MS * 0.001 + 0.01) * sample_rate + samples(150.0, sample_rate);
        let short_delay = samples(470.0, sample_rate);
        let per_sample = 44_100.0 / sample_rate;
        let scale = |inc: u32| (inc as f64 * f64::from(per_sample)).round() as u32;
        let (mix_wet, mix_dry) = mix_gains(64.0);
        let mut effect = Self {
            sample_rate,
            detector_hp: Stage1::new(super::common::words(rom_at(
                [0.992_926_5, -0.992_926_5, 0.0, 0.985_853, 0.0],
                DETECTOR_RATE,
                detector_rate,
            ))),
            peak_decay: pole_at(0.999_939, DETECTOR_RATE, detector_rate),
            fast: one_pole_at(0.016_864_972, 0.983_135, DETECTOR_RATE, detector_rate),
            slow: one_pole_at(0.000_999_202_3, 0.999_000_8, DETECTOR_RATE, detector_rate),
            peak: 0.0,
            fast_env: 0.0,
            slow_env: 0.0,
            swell: 0.0,
            swell_step: 0.0,
            swell_poly: [0.0; 5],
            gain_smooth: one_pole_at(0.016_864_972, 0.983_135, DETECTOR_RATE, detector_rate),
            gain: 0.0,
            short: [
                DelayLine::new(short_delay as usize + 4),
                DelayLine::new(short_delay as usize + 4),
            ],
            short_tap: (0, 0.0),
            long: [
                DelayLine::with_seconds(max_delay / sample_rate, sample_rate),
                DelayLine::with_seconds(max_delay / sample_rate, sample_rate),
            ],
            feedback_tap: [0.0; 2],
            time: Glide::standard(490.0 * 0.001 * sample_rate, sample_rate),
            depth: Glide::standard(0.0, sample_rate),
            lfo: [0; 4],
            increments: [
                [
                    scale(LFO_SLOW),
                    scale(LFO_SLOW),
                    scale(LFO_FAST),
                    scale(LFO_FAST),
                ],
                [
                    scale(LFO_SLOW),
                    scale(LFO_SLOW + LFO_DETUNE),
                    scale(LFO_FAST),
                    scale(LFO_FAST + LFO_DETUNE),
                ],
            ],
            alternate: false,
            mode_fade: MuteSwap::new(sample_rate),
            pending_alternate: false,
            feedback: Ramp::new(0.0, DEFAULT_STEP, sample_rate),
            wet: DurationRamp::new(mix_wet, RAMP_SAMPLES, sample_rate),
            dry: DurationRamp::new(mix_dry, RAMP_SAMPLES, sample_rate),
            last: [f32::NAN; 6],
            bpm: 0.0,
        };
        effect.short_tap = effect.short[0].fixed(short_delay);
        let defaults = crate::Model::SlwAtkDly.defaults();
        effect.set(&defaults, 120.0);
        effect.time.value = effect.time.target;
        effect.depth.value = effect.depth.target;
        effect.feedback.jump(effect.feedback.target);
        effect.wet.jump(effect.wet.target);
        effect.dry.jump(effect.dry.target);
        effect
    }

    /// Two-sample detector and swell iteration for one frame.
    #[inline]
    fn detect(&mut self, x: f32) -> f32 {
        for _ in 0..2 {
            let magnitude = self.detector_hp.process(x).abs();
            self.peak = (self.peak * self.peak_decay).max(magnitude);
            self.fast_env = self.fast.0 * (0.64 * self.peak) + self.fast.1 * self.fast_env;
            self.slow_env = flush(self.slow.0 * self.peak + self.slow.1 * self.slow_env);
            self.swell = if self.fast_env > self.slow_env {
                0.0
            } else {
                (self.swell + self.swell_step).min(1.0)
            };
        }
        self.swell
    }
}

impl Effect for SlwAtkDly {
    fn set(&mut self, values: &Values, bpm: f32) {
        if values[..6] == self.last && bpm == self.bpm {
            return;
        }
        let sr = self.sample_rate;
        if values[0] != self.last[0] || (bpm != self.bpm && is_synced(values[0], 1899)) {
            let ms = time_ms(values[0], 1899, 1.0, MAX_MS, bpm);
            self.time.target = ms * 0.001 * sr;
        }
        let fb = values[1].round().clamp(0.0, 100.0);
        self.feedback.set(curve(1.0, 0.0, 101, fb, 1));
        let (wet, dry) = mix_gains(values[2]);
        self.wet.set(wet);
        self.dry.set(dry);
        let depth = curve(
            2_384_381.0,
            0.0,
            101,
            values[3].round().clamp(0.0, 100.0),
            1,
        );
        self.depth.target = depth / 16_384.0 * sr / 44_100.0;
        let swell = values[4].round().clamp(0.0, 49.0) as usize;
        self.swell_step = 1.0 / (swell_seconds(swell as u32) * sr);
        for (c, table) in self.swell_poly.iter_mut().zip(&SWELL) {
            *c = f32::from_bits(table[swell]);
        }
        let alternate = values[5].round() >= 1.0;
        if alternate != self.pending_alternate {
            self.pending_alternate = alternate;
            if self.last[5].is_nan() {
                self.alternate = alternate;
            } else {
                self.mode_fade.start();
            }
        }
        self.last.copy_from_slice(&values[..6]);
        self.bpm = bpm;
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        let envelope = self.detect(0.5 * (frame[0] + frame[1]));
        let e2 = envelope * envelope;
        let e4 = e2 * e2;
        let e8 = e4 * e4;
        let c = &self.swell_poly;
        let poly = c[0] * envelope + c[1] * e2 + c[2] * e4 + c[3] * e2 * e4 + c[4] * e8;
        let (g, p) = self.gain_smooth;
        let gain_left = g * poly + p * self.gain;
        let gain_right = g * poly + p * gain_left;
        self.gain = gain_right;
        let mut swelled = [0.0; 2];
        for (ch, gain) in [gain_left, gain_right].into_iter().enumerate() {
            self.short[ch].write(frame[ch]);
            swelled[ch] = gain * self.short[ch].read_fixed(self.short_tap);
        }

        let (mode_gain, switch) = self.mode_fade.tick();
        if switch {
            self.alternate = self.pending_alternate;
        }
        let base = self.time.tick();
        let depth = self.depth.tick();
        let increments = &self.increments[usize::from(self.alternate)];
        for (phase, inc) in self.lfo.iter_mut().zip(increments) {
            *phase = phase.wrapping_add(*inc);
        }
        let tri = |phase: u32| (phase as i32).unsigned_abs() as f32 * Q31;
        let modulation = [
            0.5 * (tri(self.lfo[0]) + tri(self.lfo[2])),
            0.5 * (tri(self.lfo[1]) + tri(self.lfo[3])),
        ];
        let feedback = self.feedback.step();
        let mut tap_a = [0.0; 2];
        let mut tap_b = [0.0; 2];
        for ch in 0..2 {
            let line = &mut self.long[ch];
            line.write(safety(flush(
                feedback * self.feedback_tap[ch] + swelled[ch],
            )));
            tap_a[ch] = mode_gain * line.read_linear(base);
            tap_b[ch] = mode_gain * line.read_linear(base + depth * modulation[ch]);
            self.feedback_tap[ch] = tap_b[ch];
        }
        let wet = if self.alternate {
            [
                0.87 * tap_b[0] + 0.13 * tap_b[1],
                0.87 * tap_b[1] + 0.13 * tap_b[0],
            ]
        } else {
            [0.5 * (tap_a[0] + tap_b[0]), 0.5 * (tap_a[1] + tap_b[1])]
        };
        let wet_gain = self.wet.tick() * mode_gain;
        let dry_gain = self.dry.tick();
        [
            wet_gain * wet[0] + dry_gain * swelled[0],
            wet_gain * wet[1] + dry_gain * swelled[1],
        ]
    }

    fn clear(&mut self) {
        self.detector_hp.clear();
        self.peak = 0.0;
        self.fast_env = 0.0;
        self.slow_env = 0.0;
        self.swell = 0.0;
        self.gain = 0.0;
        for line in self.short.iter_mut().chain(self.long.iter_mut()) {
            line.clear();
        }
        self.feedback_tap = [0.0; 2];
        self.lfo = [0; 4];
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(SlwAtkDly::new(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(effect: &mut SlwAtkDly, input: impl Fn(usize) -> f32, frames: usize) -> Vec<f32> {
        (0..frames)
            .map(|i| {
                let x = input(i);
                effect.process([x, x])[0]
            })
            .collect()
    }

    #[test]
    fn swell_curve_follows_the_editor_segments() {
        assert!((swell_seconds(0) - 0.11).abs() < 1e-6);
        assert!((swell_seconds(24) - 0.33).abs() < 1e-6);
        assert!((swell_seconds(25) - 0.33).abs() < 1e-6);
        assert!((swell_seconds(37) - 0.9).abs() < 1e-5);
        assert!((swell_seconds(49) - 4.0).abs() < 1e-5);
        assert_eq!(mix_gains(100.0), (0.9, 0.0));
        assert!((mix_gains(76.0).0 - 0.9 * (76.0f32 / 76.0).powi(2)).abs() < 1e-6);
    }

    #[test]
    fn attack_is_swelled_in() {
        let sr = 48_000.0;
        let mut effect = SlwAtkDly::new(sr);
        let mut values = crate::Model::SlwAtkDly.defaults();
        values[2] = 0.0; // dry (swelled) path only
        values[4] = 30.0; // ~0.5 s curve
        effect.set(&values, 120.0);
        effect.wet.jump(0.0);
        // a sudden sine burst: the first milliseconds stay quiet, then it swells
        let out = run(
            &mut effect,
            |i| {
                if i > 4800 {
                    0.5 * (i as f32 * 0.05).sin()
                } else {
                    0.0
                }
            },
            48_000,
        );
        let early = out[4800 + 600..4800 + 1200]
            .iter()
            .fold(0.0f32, |m, x| m.max(x.abs()));
        let late = out[40_000..48_000]
            .iter()
            .fold(0.0f32, |m, x| m.max(x.abs()));
        assert!(early < 0.05, "early {early}");
        assert!(late > 0.4, "late {late}");
    }

    #[test]
    fn delay_time_and_feedback_bound() {
        let sr = 44_100.0;
        let mut effect = SlwAtkDly::new(sr);
        let mut values = crate::Model::SlwAtkDly.defaults();
        values[0] = 199.0; // 200 ms
        values[1] = 100.0;
        values[3] = 0.0;
        values[4] = 0.0;
        effect.set(&values, 120.0);
        effect.time.value = effect.time.target;
        assert!((effect.time.target - 8820.0).abs() < 0.5);
        let out = run(
            &mut effect,
            |i| if (i / 4410) % 10 == 0 { 0.7 } else { 0.0 },
            sr as usize * 10,
        );
        assert!(out.iter().all(|x| x.is_finite() && x.abs() < 4.0));
    }
}
