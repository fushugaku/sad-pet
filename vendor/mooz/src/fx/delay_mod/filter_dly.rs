//! FilterDly (FLTRDLY.ZDL): a mono feedback delay whose echoes pass through
//! a resonant, LFO-swept low-pass.
//!
//! Follows the recovered callback `Fx_DLY_FilterDly`:
//! - The input is summed to mono `(L + R) / 2` and written with the
//!   feedback `c6 · tap[n − 1]`; the tap reads `c33` samples back. The
//!   pre-filter (c10..16), loop filters (c17..26, c42) and output tone
//!   sections (c27..32) are identities in the ROM and no editor touches them,
//!   so they are left out.
//! - The tap (not the feedback) passes the original clipped Chamberlin state
//!   variable filter: `hp = clip(x − c46·bp − lp)`, `bp = clip(bp + f·hp)`,
//!   `lp = clip(lp + f·bp)` (all clipped to ±1) and the low-pass output is
//!   the wet signal, added to both dry channels: `out = c2·(x + c7·wet)`.
//! - The filter coefficient `f` comes from the original Q31 LFO: the phase
//!   triangle is folded into the parabolic sine `2·u·(1 − |u|) + ½`
//!   (`u = |phase| − ½`) and mapped to `c38 + shaped · c37`.
//!
//! Editors decoded (`init` copies the 47-word ROM block 0x80000524, then
//! runs every editor):
//! - `time_edit`: Time + 1 ms or a synced note (≤ 2000 ms) → `c33 =
//!   ⌊441·ms/10⌋` samples; the tap is muted (`c3`) and reopened by `onf_aft`
//!   after the new time, replaced here by a 16 ms crossfade.
//! - `fb_edit`: c6 = 0.95 · F.B/100. `mix_edit`: c7 = (Mix/100)^1.41 (curve
//!   shape 5), dry stays 1. `outLv_edit`: c2 = Level/100.
//! - `rate_edit`: phase increment `19478 + 954437·(Rate/49)²` per 44.1 kHz
//!   sample (0.2..10 Hz). `depth_edit`: c38 = 0x091dd4b4 (f = 0.071, about
//!   500 Hz) and c37 = `0x091dd4b4 + (0x2d2dbdf6 − 0x091dd4b4)·(Depth/100)²`,
//!   so the sweep spans 500 Hz..1 kHz at Depth 0 and 500 Hz..3 kHz at 100.
//! - `reso_edit`: c46 = 1 − (6/7)·Reso/10 (Q 1..7).
//!
//! Approximations: the LFO and its conversion to the host rate
//! (`f' = 2 sin(r · asin(f/2))`) run every 16 samples with a linear ramp in
//! between (the original computes one value per sample at 44.1 kHz).
use super::common::{DelayLine, FadeTap, Ramp, flush};
use crate::{Effect, Values, catalog::*, zoom};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_filter_dly",
    name: "FilterDly",
    category: Category::Delay,
    zdl: "FLTRDLY.ZDL",
    description: "Delay with echoes through a resonant LFO-swept low-pass",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Time",
            max: 2014,
            default: 499,
            display: Display::DelayTime {
                threshold: 1999,
                offset: 1,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 90,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Rate",
            max: 49,
            default: 6,
            display: Display::offset(1.0),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Reso",
            max: 10,
            default: 8,
            display: Display::INT,
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

const RATE: f32 = zoom::ORIGINAL_RATE;
const TIME_THRESHOLD: u16 = 1999;
const MAX_MS: f32 = 2000.0;
/// Q31 scale.
const Q31: f32 = 1.0 / 2_147_483_648.0;
/// `rate_edit`: integer curve 19478..973915, square shape, 50 steps.
const RATE_MIN: f32 = 19_478.0;
const RATE_MAX: f32 = 973_915.0;
/// `depth_edit`: integer curve 0x091dd4b4..0x2d2dbdf6, square shape.
const DEPTH_MIN: f32 = 0x091d_d4b4 as f32 * Q31;
const DEPTH_MAX: f32 = 0x2d2d_bdf6 as f32 * Q31;
/// `reso_edit`: 1.0 at Reso 0 down to 0x3e124925 (1/7) at 10.
const RESO_MIN: f32 = 1.0 / 7.0;
const FEEDBACK_MAX: f32 = 0.95;
/// Samples per LFO/coefficient update.
const BLOCK: u32 = 16;
/// The tap mute of `time_edit`/`onf_aft` as a crossfade (705.6 samples at 44.1 kHz).
const FADE_SECONDS: f32 = 705.6 / RATE;

/// The folded parabolic sine of the LFO, 0..1, for a Q31 phase word.
#[inline]
fn lfo_shape(phase: u32) -> f32 {
    let folded = (phase as i32).unsigned_abs() as f32 * Q31 - 0.5;
    2.0 * folded * (1.0 - folded.abs()) + 0.5
}

/// Re-derive a state-variable `f = 2 sin(π fc / fs)` from 44.1 kHz for the
/// host rate (`ratio` = 44100 / host rate).
#[inline]
fn retune(f: f32, ratio: f32) -> f32 {
    if ratio == 1.0 {
        f
    } else {
        2.0 * (ratio * (0.5 * f).clamp(-1.0, 1.0).asin()).sin()
    }
}

struct FilterDly {
    sample_rate: f32,
    ratio: f32,
    line: DelayLine,
    tap: FadeTap,
    feedback_sample: f32,
    feedback: Ramp,
    mix: Ramp,
    level: Ramp,
    damping: Ramp,
    phase: u32,
    increment: u32,
    base: f32,
    range: f32,
    countdown: u32,
    coefficient: f32,
    coefficient_step: f32,
    band: f32,
    low: f32,
    configured: bool,
}

impl FilterDly {
    fn new(sample_rate: f32) -> Self {
        let delay = 500.0 * 0.001 * sample_rate;
        Self {
            sample_rate,
            ratio: RATE / sample_rate,
            line: DelayLine::with_seconds(MAX_MS * 0.001 + 0.01, sample_rate),
            tap: FadeTap::new(delay, FADE_SECONDS, sample_rate),
            feedback_sample: 0.0,
            feedback: Ramp::new(0.0, zoom::DEFAULT_STEP, sample_rate),
            mix: Ramp::new(0.0, zoom::DEFAULT_STEP, sample_rate),
            level: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
            damping: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
            phase: 0,
            increment: 0,
            base: DEPTH_MIN,
            range: DEPTH_MIN,
            countdown: 0,
            coefficient: 0.0,
            coefficient_step: 0.0,
            band: 0.0,
            low: 0.0,
            configured: false,
        }
    }

    /// `time_edit`: Time + 1 ms, or a synced note limited to 2000 ms.
    fn time_ms(value: f32, bpm: f32) -> f32 {
        zoom::delay_time_ms(value, TIME_THRESHOLD, bpm, MAX_MS)
    }

    /// The filter coefficient for the current phase at the host rate.
    fn lfo_target(&self) -> f32 {
        let mapped = (self.base + lfo_shape(self.phase) * self.range).min(1.0);
        retune(mapped, self.ratio)
    }

    /// Advance the LFO one block and ramp the coefficient towards it.
    #[inline]
    fn next_block(&mut self) {
        self.phase = self.phase.wrapping_add(self.increment.wrapping_mul(BLOCK));
        let target = self.lfo_target();
        self.coefficient_step = (target - self.coefficient) / BLOCK as f32;
        self.countdown = BLOCK;
    }
}

impl Effect for FilterDly {
    fn set(&mut self, values: &Values, bpm: f32) {
        let [time, feedback, mix, rate, depth, reso, level, ..] = *values;
        let ms = Self::time_ms(time, bpm);
        self.tap.set(ms * 0.001 * self.sample_rate);
        self.feedback
            .set(FEEDBACK_MAX * feedback.clamp(0.0, 100.0) * 0.01);
        self.mix.set((mix.clamp(0.0, 100.0) * 0.01).powf(1.41));
        self.level.set(level.clamp(0.0, 150.0) * 0.01);
        self.damping
            .set(1.0 + (RESO_MIN - 1.0) * reso.clamp(0.0, 10.0) * 0.1);
        let t = rate.clamp(0.0, 49.0) / 49.0;
        let per_44k = RATE_MIN + (RATE_MAX - RATE_MIN) * t * t;
        self.increment = (per_44k * self.ratio).round() as u32;
        let d = depth.clamp(0.0, 100.0) * 0.01;
        self.range = DEPTH_MIN + (DEPTH_MAX - DEPTH_MIN) * d * d;
        if !self.configured {
            self.configured = true;
            self.tap.jump(self.tap.target());
            for ramp in [
                &mut self.feedback,
                &mut self.mix,
                &mut self.level,
                &mut self.damping,
            ] {
                ramp.jump(ramp.target);
            }
            self.coefficient = self.lfo_target();
            self.coefficient_step = 0.0;
        }
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        if self.countdown == 0 {
            self.next_block();
        }
        self.countdown -= 1;
        let f = self.coefficient;
        self.coefficient += self.coefficient_step;

        let feedback = self.feedback.tick();
        let mono = 0.5 * (frame[0] + frame[1]);
        self.line
            .write(flush(mono + feedback * self.feedback_sample));
        let tap = self.tap.read(&self.line);
        self.feedback_sample = tap;

        let damping = self.damping.tick();
        let high = (tap - self.band * damping - self.low).clamp(-1.0, 1.0);
        self.band = flush((self.band + high * f).clamp(-1.0, 1.0));
        self.low = flush((self.low + self.band * f).clamp(-1.0, 1.0));

        let wet = self.mix.tick() * self.low;
        let level = self.level.tick();
        [level * (frame[0] + wet), level * (frame[1] + wet)]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.tap.jump(self.tap.target());
        self.feedback_sample = 0.0;
        self.band = 0.0;
        self.low = 0.0;
        self.phase = 0;
        self.coefficient = self.lfo_target();
        self.coefficient_step = 0.0;
        self.countdown = 0;
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(FilterDly::new(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    fn values(edits: &[(usize, f32)]) -> Values {
        let mut values = Model::from_key("mooz_filter_dly").unwrap().defaults();
        for &(index, value) in edits {
            values[index] = value;
        }
        values
    }

    /// First wet sample after an impulse (Level 100, dry removed by subtraction).
    fn echo_onset(sample_rate: f32, edits: &[(usize, f32)], bpm: f32) -> usize {
        let mut effect = FilterDly::new(sample_rate);
        effect.set(&values(edits), bpm);
        for i in 0..(2.5 * sample_rate) as usize {
            let x = if i == 0 { 0.5 } else { 0.0 };
            let y = effect.process([x, x])[0] - x;
            if y.abs() > 1e-5 {
                return i;
            }
        }
        usize::MAX
    }

    #[test]
    fn echo_lands_at_time_plus_one_ms() {
        for sample_rate in [44_100.0, 48_000.0, 96_000.0] {
            let onset = echo_onset(sample_rate, &[(1, 0.0)], 120.0);
            let expected = 0.5 * sample_rate;
            assert!(
                (onset as f32 - expected).abs() <= 1.0,
                "{sample_rate}: {onset}"
            );
        }
    }

    #[test]
    fn synced_time_follows_tempo() {
        // 2000 + 6 is a quarter note: 500 ms at 120 BPM, 1000 ms at 60.
        let onset = echo_onset(48_000.0, &[(0, 2006.0), (1, 0.0)], 60.0);
        assert!((onset as f32 - 48_000.0).abs() <= 1.0, "{onset}");
        assert!((FilterDly::time_ms(2014.0, 120.0) - 2000.0).abs() < 1e-3);
    }

    #[test]
    fn lfo_shape_is_a_folded_parabolic_sine() {
        assert!(lfo_shape(0).abs() < 1e-6);
        assert!((lfo_shape(1 << 30) - 0.5).abs() < 1e-6);
        assert!((lfo_shape(0x8000_0000) - 1.0).abs() < 1e-6);
        assert!((lfo_shape(1 << 29) - 0.125).abs() < 1e-6);
        for i in 0..64u32 {
            let p = i.wrapping_mul(1 << 26);
            let sine = 0.5 - 0.5 * (i as f32 / 64.0 * std::f32::consts::TAU).cos();
            assert!((lfo_shape(p) - sine).abs() < 0.06, "{i}");
        }
    }

    #[test]
    fn rate_and_depth_follow_the_editors() {
        let mut effect = FilterDly::new(RATE);
        effect.set(&values(&[(3, 49.0), (4, 100.0)]), 120.0);
        let hz = effect.increment as f32 * RATE / 4_294_967_296.0;
        assert!((hz - 10.0).abs() < 0.01, "{hz}");
        effect.set(&values(&[(3, 0.0)]), 120.0);
        let hz = effect.increment as f32 * RATE / 4_294_967_296.0;
        assert!((hz - 0.2).abs() < 0.001, "{hz}");
        // Depth 100 sweeps f over 0.071..0.424 (500 Hz..3 kHz at 44.1 kHz).
        let (mut low, mut high) = (f32::MAX, f32::MIN);
        effect.set(&values(&[(3, 49.0), (4, 100.0)]), 120.0);
        for _ in 0..44_100 {
            effect.process([0.0; 2]);
            low = low.min(effect.coefficient);
            high = high.max(effect.coefficient);
        }
        let hz = |f: f32| (0.5 * f).asin() * RATE / std::f32::consts::PI;
        assert!((hz(low) - 500.0).abs() < 10.0, "{}", hz(low));
        assert!((hz(high) - 3000.0).abs() < 30.0, "{}", hz(high));
    }

    #[test]
    fn retuned_coefficient_keeps_the_cutoff() {
        let f = 2.0 * (std::f32::consts::PI * 2000.0 / RATE).sin();
        let g = retune(f, RATE / 96_000.0);
        let hz = (0.5 * g).asin() * 96_000.0 / std::f32::consts::PI;
        assert!((hz - 2000.0).abs() < 0.5, "{hz}");
    }

    #[test]
    fn state_variable_low_pass_has_unity_dc_gain() {
        let mut effect = FilterDly::new(48_000.0);
        // Time 0 (1 ms), F.B 0, Mix 100.
        effect.set(&values(&[(0, 0.0), (1, 0.0), (2, 100.0)]), 120.0);
        let mut y = 0.0;
        for _ in 0..48_000 {
            y = effect.process([0.2, 0.2])[0];
        }
        assert!((y - 0.4).abs() < 1e-3, "{y}");
    }

    #[test]
    fn feedback_at_maximum_stays_bounded() {
        let mut effect = FilterDly::new(48_000.0);
        effect.set(
            &values(&[(0, 10.0), (1, 100.0), (2, 100.0), (5, 10.0), (6, 150.0)]),
            120.0,
        );
        let mut peak = 0.0f32;
        for i in 0..48_000 * 8 {
            let x = if i < 48_000 {
                (i as f32 * 0.07).sin()
            } else {
                0.0
            };
            let [l, r] = effect.process([x, x]);
            assert!(l.is_finite() && r.is_finite());
            peak = peak.max(l.abs()).max(r.abs());
        }
        assert!(peak < 4.0, "{peak}");
    }

    #[test]
    fn defaults_sound_near_unity() {
        let mut effect = FilterDly::new(48_000.0);
        effect.set(&values(&[]), 120.0);
        let (mut input, mut output) = (0.0f64, 0.0f64);
        for i in 0..48_000 * 3 {
            let x = 0.3 * (i as f32 * std::f32::consts::TAU * 220.0 / 48_000.0).sin();
            let [l, r] = effect.process([x, x]);
            input += f64::from(x * x);
            output += f64::from(0.5 * (l * l + r * r));
        }
        let ratio = (output / input).sqrt();
        assert!((0.7..2.5).contains(&ratio), "{ratio}");
    }

    #[test]
    fn time_changes_do_not_click() {
        let mut effect = FilterDly::new(48_000.0);
        effect.set(&values(&[(2, 100.0), (1, 60.0)]), 120.0);
        let mut previous = 0.0f32;
        let mut largest = 0.0f32;
        for i in 0..48_000 * 3 {
            if i % 4800 == 0 {
                effect.set(
                    &values(&[
                        (0, 100.0 + (i / 4800 % 7) as f32 * 90.0),
                        (2, 100.0),
                        (1, 60.0),
                    ]),
                    120.0,
                );
            }
            let x = 0.3 * (i as f32 * std::f32::consts::TAU * 110.0 / 48_000.0).sin();
            let y = effect.process([x, x])[0] - x;
            if i > 48_000 {
                largest = largest.max((y - previous).abs());
            }
            previous = y;
        }
        assert!(largest < 0.1, "{largest}");
    }
}
