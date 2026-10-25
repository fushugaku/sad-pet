//! PhaseDly (PHASEDLY.ZDL): a mono feedback delay whose echoes pass through
//! an LFO-swept all-pass phaser with its own regeneration.
//!
//! Follows the recovered callback `Fx_DLY_PhaseDly`:
//! - The input is summed to mono `(L + R) / 2` and written with the
//!   feedback `c6 · tap[n − 1]`; the tap reads `c33` samples back. The
//!   delay pre-filters (c10..21, c34, c35), the feedback filter (c22..26) and
//!   the output sections (c27..32) are identities in the ROM and untouched by
//!   the editors, so they are left out.
//! - The tap passes the phaser: `in = c46·tap + c47·previous` (`previous` is
//!   the last all-pass output), then `c45` first-order all-passes
//!   `w = v + a·s, v = s − a·w, s = w` sharing one coefficient `a`, and
//!   `c48·in + c49·v` is the wet signal, scaled by c50 and added to both dry
//!   channels: `out = c2·(x + c7·wet)`.
//! - `a` comes from the original Q31 LFO: `m = |phase|`, the curved triangle
//!   `1 − 0.06·m − 0.94·m²` (c39/c40) mapped to `c42 + c41·shape`.
//!
//! Editors decoded (`init` copies the 51-word ROM block 0x800004cc, then
//! runs every editor):
//! - `time_edit`: Time + 1 ms or a synced note (≤ 2000 ms) → `c33 =
//!   ⌊441·ms/10⌋` samples; the tap is muted (`c3`) and reopened by `onf_aft`
//!   after the new time, replaced here by a 16 ms crossfade.
//! - `fb_edit`: c6 = 0.95 · F.B/100. `mix_edit`: c7 = (Mix/100)^1.41 (curve
//!   shape 5), dry stays 1. `outLv_edit`: c2 = Level/100.
//! - `rate_edit`: phase increment `17894 + 742672·(Rate/49)²` per 44.1 kHz
//!   sample (0.18..7.8 Hz).
//! - `color_edit`: ramps c50 to 0, loads the stage count (0x800006a8),
//!   c41/c42 (0x80000648), c46/c47 (0x80000628), c48/c49 (0x80000668) and
//!   ramps c50 back to 1. The ROM stage counts are 4, 4, 8, 8 while the pedal
//!   labels read "4 STG", "8 STG", "inv 4", "inv 8"; the ROM is followed.
//!
//! Approximations: the LFO and its conversion to the host rate (the
//! all-pass corner `t = (1 − a)/(1 + a)` re-warped with `tan(r · atan t)`)
//! run every 16 samples with a linear ramp in between; a Color change fades
//! the phaser out and in around the swap instead of the original's
//! immediately retargeted ramp.
use super::common::{DelayLine, FadeTap, Ramp, flush};
use crate::{Effect, Values, catalog::*, zoom};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_phase_dly",
    name: "PhaseDly",
    category: Category::Delay,
    zdl: "PHASEDLY.ZDL",
    description: "Delay with echoes through a swept all-pass phaser",
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
            default: 28,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 57,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Rate",
            max: 49,
            default: 49,
            display: Display::offset(1.0),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Color",
            max: 3,
            default: 3,
            display: Display::labels(&["4 STG", "8 STG", "inv 4", "inv 8"]),
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

const RATE: f32 = zoom::ORIGINAL_RATE;
const TIME_THRESHOLD: u16 = 1999;
const MAX_MS: f32 = 2000.0;
const Q31: f32 = 1.0 / 2_147_483_648.0;
/// `rate_edit`: integer curve 17894..760566, square shape, 50 steps.
const RATE_MIN: f32 = 17_894.0;
const RATE_MAX: f32 = 760_566.0;
/// c39/c40: the LFO curve `1 + c39·m + c40·m²`.
const CURVE_LINEAR: f32 = 0xf852_0000_u32 as i32 as f32 * Q31;
const CURVE_SQUARE: f32 = 0x87af_0000_u32 as i32 as f32 * Q31;
const FEEDBACK_MAX: f32 = 0.95;
const BLOCK: u32 = 16;
const FADE_SECONDS: f32 = 705.6 / RATE;
const MAX_STAGES: usize = 8;

/// One Color setting: stage count, LFO range c41 / base c42 (Q31), input
/// c46 / regeneration c47, direct c48 / phased c49.
#[derive(Debug, Clone, Copy)]
struct Color {
    stages: usize,
    range: f32,
    base: f32,
    input: f32,
    regeneration: f32,
    direct: f32,
    phased: f32,
}

const fn color(stages: usize, words: [u32; 6]) -> Color {
    Color {
        stages,
        range: words[0] as f32 * Q31,
        base: words[1] as f32 * Q31,
        input: f32::from_bits(words[2]),
        regeneration: f32::from_bits(words[3]),
        direct: f32::from_bits(words[4]),
        phased: f32::from_bits(words[5]),
    }
}

const COLORS: [Color; 4] = [
    color(
        4,
        [
            0x5fe14b6e, 0x160ecba3, 0x3f444589, 0x3ea3d548, 0x3f6667cd, 0x3f6667cd,
        ],
    ),
    color(
        4,
        [
            0x4b00f767, 0x26b486f0, 0x3f5555ab, 0x3efaddf6, 0x3f6667cd, 0x3f6667cd,
        ],
    ),
    color(
        8,
        [
            0x192dbf24, 0x5887bf33, 0x3f333366, 0xbf23d948, 0x3f6667cd, 0xbf6669cd,
        ],
    ),
    color(
        8,
        [
            0x54b027da, 0x1b9e0876, 0x3f5dddbc, 0xbf09ad13, 0x3f5dddbc, 0xbf5ddfbc,
        ],
    ),
];

/// The LFO curve (c39/c40 + INT_MAX), 1 at phase 0 falling to 0 at ½ turn.
#[inline]
fn lfo_shape(phase: u32) -> f32 {
    let m = (phase as i32).unsigned_abs() as f32 * Q31;
    (1.0 + CURVE_LINEAR * m + CURVE_SQUARE * m * m).clamp(0.0, 1.0)
}

/// Re-derive a first-order all-pass coefficient from 44.1 kHz for the host
/// rate, keeping its 90° frequency (`ratio` = 44100 / host rate).
#[inline]
fn retune(a: f32, ratio: f32) -> f32 {
    if ratio == 1.0 {
        return a;
    }
    let t = ((1.0 - a) / (1.0 + a)).max(0.0);
    let t = (ratio * t.atan()).tan();
    (1.0 - t) / (1.0 + t)
}

struct PhaseDly {
    sample_rate: f32,
    ratio: f32,
    line: DelayLine,
    tap: FadeTap,
    feedback_sample: f32,
    feedback: Ramp,
    mix: Ramp,
    level: Ramp,
    /// c50: the phaser output gain, dipped around Color changes.
    gate: Ramp,
    color: Color,
    color_index: usize,
    pending_color: Option<usize>,
    stages: [f32; MAX_STAGES],
    previous: f32,
    phase: u32,
    increment: u32,
    countdown: u32,
    coefficient: f32,
    coefficient_step: f32,
    configured: bool,
}

impl PhaseDly {
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
            gate: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
            color: COLORS[3],
            color_index: 3,
            pending_color: None,
            stages: [0.0; MAX_STAGES],
            previous: 0.0,
            phase: 0,
            increment: 0,
            countdown: 0,
            coefficient: 0.0,
            coefficient_step: 0.0,
            configured: false,
        }
    }

    fn time_ms(value: f32, bpm: f32) -> f32 {
        zoom::delay_time_ms(value, TIME_THRESHOLD, bpm, MAX_MS)
    }

    fn lfo_target(&self) -> f32 {
        let a = (self.color.base + self.color.range * lfo_shape(self.phase)).min(1.0);
        retune(a, self.ratio)
    }

    #[inline]
    fn next_block(&mut self) {
        self.phase = self.phase.wrapping_add(self.increment.wrapping_mul(BLOCK));
        let target = self.lfo_target();
        self.coefficient_step = (target - self.coefficient) / BLOCK as f32;
        self.countdown = BLOCK;
    }

    fn apply_color(&mut self, index: usize) {
        let color = COLORS[index];
        // Stages that were idle start from silence.
        for stage in &mut self.stages[self.color.stages.min(color.stages)..] {
            *stage = 0.0;
        }
        self.color = color;
        self.color_index = index;
        self.pending_color = None;
    }
}

impl Effect for PhaseDly {
    fn set(&mut self, values: &Values, bpm: f32) {
        let [time, feedback, mix, rate, color, level, ..] = *values;
        let ms = Self::time_ms(time, bpm);
        self.tap.set(ms * 0.001 * self.sample_rate);
        self.feedback
            .set(FEEDBACK_MAX * feedback.clamp(0.0, 100.0) * 0.01);
        self.mix.set((mix.clamp(0.0, 100.0) * 0.01).powf(1.41));
        self.level.set(level.clamp(0.0, 150.0) * 0.01);
        let t = rate.clamp(0.0, 49.0) / 49.0;
        let per_44k = RATE_MIN + (RATE_MAX - RATE_MIN) * t * t;
        self.increment = (per_44k * self.ratio).round() as u32;
        let color = (color.round().clamp(0.0, 3.0)) as usize;
        if !self.configured {
            self.configured = true;
            self.apply_color(color);
            self.tap.jump(self.tap.target());
            for ramp in [&mut self.feedback, &mut self.mix, &mut self.level] {
                ramp.jump(ramp.target);
            }
            self.gate.jump(1.0);
            self.coefficient = self.lfo_target();
            self.coefficient_step = 0.0;
        } else if color != self.color_index {
            self.pending_color = Some(color);
            self.gate.set(0.0);
        } else if self.pending_color.is_some() {
            self.pending_color = None;
            self.gate.set(1.0);
        }
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        if self.countdown == 0 {
            self.next_block();
        }
        self.countdown -= 1;
        let a = self.coefficient;
        self.coefficient += self.coefficient_step;

        let feedback = self.feedback.tick();
        let mono = 0.5 * (frame[0] + frame[1]);
        self.line
            .write(flush(mono + feedback * self.feedback_sample));
        let tap = self.tap.read(&self.line);
        self.feedback_sample = tap;

        let color = self.color;
        let incoming = color.input * tap + color.regeneration * self.previous;
        let mut v = incoming;
        for stage in &mut self.stages[..color.stages] {
            let w = v + a * *stage;
            v = *stage - a * w;
            *stage = flush(w);
        }
        self.previous = flush(v);
        let phased = color.direct * incoming + color.phased * v;

        let gate = self.gate.tick();
        if let Some(index) = self.pending_color
            && gate == 0.0
        {
            self.apply_color(index);
            self.gate.set(1.0);
        }
        let wet = self.mix.tick() * gate * phased;
        let level = self.level.tick();
        [level * (frame[0] + wet), level * (frame[1] + wet)]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.tap.jump(self.tap.target());
        self.feedback_sample = 0.0;
        self.stages = [0.0; MAX_STAGES];
        self.previous = 0.0;
        if let Some(index) = self.pending_color {
            self.apply_color(index);
        }
        self.gate.jump(1.0);
        self.phase = 0;
        self.coefficient = self.lfo_target();
        self.coefficient_step = 0.0;
        self.countdown = 0;
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(PhaseDly::new(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    fn values(edits: &[(usize, f32)]) -> Values {
        let mut values = Model::from_key("mooz_phase_dly").unwrap().defaults();
        for &(index, value) in edits {
            values[index] = value;
        }
        values
    }

    fn echo_onset(sample_rate: f32, edits: &[(usize, f32)], bpm: f32) -> usize {
        let mut effect = PhaseDly::new(sample_rate);
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
        let onset = echo_onset(48_000.0, &[(0, 2006.0), (1, 0.0)], 120.0);
        assert!((onset as f32 - 24_000.0).abs() <= 1.0, "{onset}");
    }

    #[test]
    fn color_tables_follow_the_rom() {
        assert_eq!(COLORS.map(|c| c.stages), [4, 4, 8, 8]);
        // Color 3 sweeps a over 0.216..0.877.
        let c = COLORS[3];
        assert!((c.base - 0.215_760_3).abs() < 1e-6 && (c.base + c.range - 0.877_386).abs() < 1e-5);
        assert!(COLORS[0].phased > 0.0 && COLORS[2].phased < 0.0);
        assert!((lfo_shape(0) - 1.0).abs() < 1e-6 && lfo_shape(0x8000_0000) < 1e-3);
    }

    #[test]
    fn rate_follows_the_editor() {
        let mut effect = PhaseDly::new(RATE);
        effect.set(&values(&[(3, 49.0)]), 120.0);
        let hz = effect.increment as f32 * RATE / 4_294_967_296.0;
        assert!((hz - 7.81).abs() < 0.01, "{hz}");
        effect.set(&values(&[(3, 0.0)]), 120.0);
        let hz = effect.increment as f32 * RATE / 4_294_967_296.0;
        assert!((hz - 0.1837).abs() < 0.001, "{hz}");
    }

    #[test]
    fn retuned_all_pass_keeps_its_corner() {
        let corner =
            |a: f32, rate: f32| ((1.0 - a) / (1.0 + a)).atan() * rate / std::f32::consts::PI;
        for a in [0.2, 0.5, 0.9] {
            let b = retune(a, RATE / 96_000.0);
            assert!((corner(a, RATE) - corner(b, 96_000.0)).abs() < 1.0, "{a}");
        }
    }

    #[test]
    fn all_pass_chain_keeps_energy() {
        // Without the direct path the phaser alone is an all-pass.
        let a = 0.6;
        let mut stages = [0.0f32; 8];
        let (mut input, mut output) = (0.0f64, 0.0f64);
        let mut noise = crate::dsp::Noise::new(3);
        for _ in 0..200_000 {
            let x = noise.bipolar();
            let mut v = x;
            for stage in &mut stages {
                let w = v + a * *stage;
                v = *stage - a * w;
                *stage = w;
            }
            input += f64::from(x * x);
            output += f64::from(v * v);
        }
        assert!((output / input - 1.0).abs() < 0.02, "{}", output / input);
    }

    #[test]
    fn color_changes_do_not_click() {
        let mut effect = PhaseDly::new(48_000.0);
        effect.set(&values(&[(0, 10.0), (2, 100.0)]), 120.0);
        let mut previous = 0.0f32;
        let mut largest = 0.0f32;
        for i in 0..48_000 * 2 {
            if i % 9600 == 4800 {
                effect.set(
                    &values(&[(0, 10.0), (2, 100.0), (4, (i / 9600 % 4) as f32)]),
                    120.0,
                );
            }
            let x = 0.3 * (i as f32 * std::f32::consts::TAU * 110.0 / 48_000.0).sin();
            let y = effect.process([x, x])[0] - x;
            if i > 4800 {
                largest = largest.max((y - previous).abs());
            }
            previous = y;
        }
        assert!(largest < 0.05, "{largest}");
        assert_eq!(effect.color_index, (48_000 * 2 / 9600 - 1) % 4);
    }

    #[test]
    fn feedback_at_maximum_stays_bounded() {
        for color in 0..4 {
            let mut effect = PhaseDly::new(48_000.0);
            effect.set(
                &values(&[
                    (0, 10.0),
                    (1, 100.0),
                    (2, 100.0),
                    (4, color as f32),
                    (5, 150.0),
                ]),
                120.0,
            );
            let mut peak = 0.0f32;
            for i in 0..48_000 * 6 {
                let x = if i < 48_000 {
                    (i as f32 * 0.07).sin()
                } else {
                    0.0
                };
                let [l, r] = effect.process([x, x]);
                assert!(l.is_finite() && r.is_finite());
                peak = peak.max(l.abs()).max(r.abs());
            }
            assert!(peak < 16.0, "{color}: {peak}");
        }
    }

    #[test]
    fn defaults_sound_near_unity() {
        let mut effect = PhaseDly::new(48_000.0);
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
}
