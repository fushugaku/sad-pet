//! Echo (ECHO.ZDL): `Fx_REV_Echo` with its inner `Audio_Sub_Arena`.
//!
//! Signal flow of the recovered callback (`echo/front_frame.rs`,
//! `arena_frame.rs`, `output_mode_0/1.rs`):
//!
//! 1. Front: the L/R mean passes the Tone low-pass (c13..17) and a fixed
//!    ~100 Hz high-pass (c18..22) and enters a 300 ms line together with the
//!    feedback `clip(c12 · delayed)` (hard clip at ±1) after a ~20 Hz DC
//!    blocker (c23..27). The read position is a Q13 distance slewed by a
//!    one-pole (c38/c39, τ ≈ 122 ms), read with linear interpolation. The
//!    delayed signal feeds two 0.7-feedback combs (100/273 samples), two
//!    g = 0.4 all-pass sections (543/575) and a 250 ms tail comb (c92..97);
//!    the front output is `0.65·delayed + 0.2·diffused + 0.07·tail`.
//! 2. Arena: the fixed seven-line network of ARENA.ZDL (10 ms predelay, two
//!    diffusers, four combs with the gains `Fx_REV_Echo_onf_aft` writes, eight
//!    output taps), each side low-passed (c78..82, ~1 kHz) and added as
//!    `1.2 · (front + 0.35 · reverb)`.
//! 3. Output: a fixed low-frequency EQ biquad (c28..32) per side, then
//!    `c7 · dry + c6 · wet`; Mode MONO averages both sides (output_mode_0),
//!    STR keeps them (output_mode_1).
//!
//! Editors decoded from ECHO.ZDL.dis (all tables from `.const`, read through
//! the module's `TBL_TO_VAL` 11-point linear interpolation over 0..100):
//! - TIME (`time_edit`): `c11 = (441·TIME/10) << 13`, i.e. TIME ms (0..200);
//!   the slew glides to it (the callback's own smoothing).
//! - DECAY (`fb_edit`): `c12 = TBL(0x820)` feedback 0..1.1 (clipped loop).
//! - MIX/DECAY (`mix_edit`): `c6 = TBL(MIX, 0x7f0) · TBL(DECAY, 0x730)`,
//!   `c7 = TBL(MIX, 0x760)` (0.7, 0 at MIX 100). Both ramp with step
//!   0x3a77c5f5 like the float setter.
//! - TONE (`Tone_edit`): Main's biquad designer (context[25], type 2 =
//!   bilinear low-pass with prewarp) at `TBL(0x7c0)` Hz (1000..5250) and
//!   Q `TBL(0x790)` (0.35..0.75); redesigned here at the host rate.
//! - Mode (`OUTMODE_edit`): `c40` = MONO/STR; the original mutes c113 around
//!   the switch, here the two outputs crossfade over ~16 ms.
//!
//! The running-state gains come from `onf` and `onf_aft` (c0, c3, c5, c42,
//! c58, c65, c93, c97, c100, c105, c108, c112 = 1). The tap-mute that the
//! original applies on some time edits (`tapmuteClose`/`tapmuteOpen`) is not
//! reproduced: the slewed read head glides instead. ROM filters designed for
//! 44.1 kHz are re-derived for the host rate with `Coefficients::resample`,
//! delay lengths are scaled.
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{self, Biquad, Coefficients, DelayLine},
    zoom,
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_echo",
    name: "Echo",
    category: Category::Reverb,
    zdl: "ECHO.ZDL",
    description: "Short echo through diffusers into an Arena reverb",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "DECAY",
            max: 100,
            default: 25,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "TIME",
            max: 200,
            default: 125,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "TONE",
            max: 100,
            default: 70,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "MIX",
            max: 100,
            default: 80,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 7
        ParamInfo {
            name: "Mode",
            max: 1,
            default: 0,
            display: Display::labels(&["Mono", "Stereo"]),
        },
    ],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Echo::new(sample_rate))
}

const fn w(bits: u32) -> f32 {
    f32::from_bits(bits)
}

/// `TBL_TO_VAL` tables (11 points over 0..100).
const FEEDBACK: [f32; 11] = [
    0.0,
    w(0x3e4c_cccd),
    w(0x3e99_999a),
    w(0x3ecc_cccd),
    0.5,
    w(0x3f28_f5c3),
    w(0x3f38_51ec),
    w(0x3f4f_5c29),
    w(0x3f66_6666),
    1.0,
    w(0x3f8c_cccd),
];
const WET_BY_MIX: [f32; 11] = [
    0.0,
    w(0x3d4c_cccd),
    w(0x3e19_999a),
    w(0x3e38_51ec),
    w(0x3e68_acd5),
    w(0x3e99_999a),
    w(0x3ecc_ad6c),
    w(0x3f10_e3f9),
    w(0x3f4b_4120),
    w(0x3f76_1676),
    1.0,
];
const DRY_BY_MIX: [f32; 11] = [
    w(0x3f33_3333),
    w(0x3f33_3333),
    w(0x3f33_3333),
    w(0x3f33_3333),
    w(0x3f33_3333),
    w(0x3f33_3333),
    w(0x3f33_3333),
    w(0x3f33_3333),
    w(0x3f33_3333),
    w(0x3f33_3333),
    0.0,
];
const WET_BY_DECAY: [f32; 11] = [
    1.0,
    w(0x3f59_999a),
    0.75,
    w(0x3f33_3333),
    w(0x3f26_6666),
    w(0x3f19_999a),
    w(0x3f19_999a),
    w(0x3f19_999a),
    w(0x3f19_999a),
    w(0x3f19_999a),
    w(0x3f19_999a),
];
const TONE_HZ: [f32; 11] = [
    1000.0, 1600.0, 2200.0, 3000.0, 4000.0, 4500.0, 4750.0, 5000.0, 5000.0, 5000.0, 5250.0,
];
const TONE_Q: [f32; 11] = [
    w(0x3eb3_3333),
    w(0x3eb3_3333),
    w(0x3eb3_3333),
    w(0x3eb3_3333),
    0.375,
    w(0x3ecc_cccd),
    w(0x3ee6_6666),
    0.5,
    w(0x3f0c_cccd),
    w(0x3f26_6666),
    0.75,
];

/// ECHO.ZDL `TBL_TO_VAL(101, value, table, 11)`.
fn table(points: &[f32; 11], value: f32) -> f32 {
    let value = value.clamp(0.0, 100.0);
    let position = value * 0.1;
    let index = (position as usize).min(9);
    let fraction = position - index as f32;
    points[index] + (points[index + 1] - points[index]) * fraction
}

/// A ROM biquad in the callback's convention (`+ a1 y1 + a2 y2`), as dsp form.
const fn rom(b: [u32; 3], a: [u32; 2]) -> Coefficients {
    Coefficients {
        b0: w(b[0]),
        b1: w(b[1]),
        b2: w(b[2]),
        a1: -w(a[0]),
        a2: -w(a[1]),
    }
}
/// c18..c22: ~100 Hz high-pass after the Tone filter.
const INPUT_HIGHPASS: Coefficients = rom(
    [0x3f7d_4876, 0xbffd_4876, 0x3f7d_4876],
    [0x3ffd_45cc, 0xbf7a_963f],
);
/// c23..c27: DC blocker in the feedback path.
const FEEDBACK_DC: Coefficients = rom([0x3f7f_a2c2, 0xbf7f_a2c2, 0], [0x3f7f_4585, 0]);
/// c28..c32: output EQ (c33..c37 is the identity).
const OUTPUT_EQ: Coefficients = rom(
    [0x3f80_153f, 0xbffe_94da, 0x3f7d_1cfc],
    [0x3ffe_94da, 0xbf7d_477a],
);
/// c78..c82: Arena output low-pass.
const ARENA_LOWPASS: Coefficients = rom(
    [0x3d09_cd59, 0x3d89_cd59, 0x3d09_cd59],
    [0x3f37_86a7, 0x3e18_180a],
);

// Front section (c92..c112) at 44.1 kHz.
const COMB_DELAYS: [usize; 2] = [100, 273]; // c98, c99 (500-word banks)
const COMB_FEEDBACK: f32 = w(0x3f33_3333); // c102
const ALLPASS_DELAYS: [usize; 2] = [543, 575]; // c106, c107 (600-word banks)
const ALLPASS_GAIN: f32 = w(0x3ecc_cccd); // c109
const DIFFUSED_LEVEL: f32 = w(0x3e4c_cccd); // c110
const DELAYED_LEVEL: f32 = w(0x3f26_6666); // c111
const TAIL_DELAY: usize = 11025; // c92
const TAIL_FEEDBACK: f32 = w(0x3da3_d70a); // c94
const TAIL_LEVEL: f32 = w(0x3d8f_5c29); // c95

// Arena (c41..c91): delays are read offsets minus write offsets.
const ARENA_PREDELAY: usize = 441; // c43 - c85
const ARENA_DIFFUSERS: [usize; 2] = [2135, 1548]; // c44 - c86, c45 - c87
const ARENA_COMBS: [usize; 4] = [8137, 7418, 6904, 5283]; // c46.., c88..
const ARENA_LEFT: [usize; 4] = [1236, 972, 443, 134]; // c50, c52, c54, c56
const ARENA_RIGHT: [usize; 4] = [1555, 1193, 721, 134]; // c51, c53, c55, c57
const ARENA_COMB_GAINS: [f32; 4] = [
    w(0xbf0a_ef16),
    w(0xbf12_a125),
    w(0xbf18_6931),
    w(0xbf2c_2758),
]; // c71..c74 (onf_aft)
const ARENA_DIFFUSE_IN: f32 = w(0x3f60_01c0); // c66
const ARENA_DIFFUSE_FB: f32 = w(0xbf11_c924); // c67
const ARENA_DIFFUSE_OUT: f32 = w(0x3f11_c924); // c68
const ARENA_SHARED: f32 = w(0x3ec0_0180); // c70
const ARENA_TAP_LEVEL: f32 = w(0x3e80_0100); // c76, c77
const ARENA_OUTPUT: f32 = w(0x3f99_999a); // c41
const ARENA_WET: f32 = w(0x3eb3_3333); // c83

/// c38 = 1 - c39 (Q31): the distance slew per 44.1 kHz sample.
const SLEW_44K: f32 = 398_662.0 / 2_147_483_648.0;
/// Longest TIME (ms) and the original 300 ms line.
const MAX_TIME_MS: f32 = 200.0;
const TONE_INTERVAL: u32 = 16;

fn scaled(samples: usize, sample_rate: f32) -> usize {
    (dsp::scale_samples(samples as f32, sample_rate).round() as usize).max(1)
}

/// The seven-line Arena network with both output low-passes.
struct Arena {
    predelay: DelayLine,
    diffusers: [DelayLine; 2],
    combs: [DelayLine; 4],
    predelay_read: usize,
    diffuser_reads: [usize; 2],
    comb_reads: [usize; 4],
    left_reads: [usize; 4],
    right_reads: [usize; 4],
    lowpass: [Biquad; 2],
}

impl Arena {
    fn new(sample_rate: f32) -> Self {
        // Every tap is read before this frame's write: a delay of `d`
        // samples reads index `d - 1`.
        let read = |d: usize| scaled(d, sample_rate) - 1;
        let lowpass = Biquad::new(ARENA_LOWPASS.resample(zoom::ORIGINAL_RATE, sample_rate));
        Self {
            predelay: DelayLine::new(scaled(ARENA_PREDELAY, sample_rate)),
            diffusers: ARENA_DIFFUSERS.map(|d| DelayLine::new(scaled(d, sample_rate))),
            combs: ARENA_COMBS.map(|d| DelayLine::new(scaled(d, sample_rate))),
            predelay_read: read(ARENA_PREDELAY),
            diffuser_reads: ARENA_DIFFUSERS.map(read),
            comb_reads: ARENA_COMBS.map(read),
            left_reads: ARENA_LEFT.map(read),
            right_reads: ARENA_RIGHT.map(read),
            lowpass: [lowpass; 2],
        }
    }

    #[inline]
    fn process(&mut self, input: f32) -> [f32; 2] {
        let predelayed = self.predelay.read(self.predelay_read);
        let d0 = self.diffusers[0].read(self.diffuser_reads[0]);
        let d1 = self.diffusers[1].read(self.diffuser_reads[1]);
        let fed: [f32; 4] = std::array::from_fn(|i| self.combs[i].read(self.comb_reads[i]));
        let mut left = 0.0;
        let mut right = 0.0;
        for i in 0..4 {
            left += self.combs[i].read(self.left_reads[i]);
            right += self.combs[i].read(self.right_reads[i]);
        }
        let left = self.lowpass[0].process(ARENA_TAP_LEVEL * left);
        let right = self.lowpass[1].process(ARENA_TAP_LEVEL * right);

        let first = ARENA_DIFFUSE_OUT * predelayed + d0;
        let second = ARENA_DIFFUSE_OUT * first + d1;
        let shared = ARENA_SHARED * second;
        self.predelay.write(input);
        self.diffusers[0].write(dsp::flush(
            ARENA_DIFFUSE_FB * d0 + ARENA_DIFFUSE_IN * predelayed,
        ));
        self.diffusers[1].write(dsp::flush(ARENA_DIFFUSE_FB * d1 + ARENA_DIFFUSE_IN * first));
        for (i, comb) in self.combs.iter_mut().enumerate() {
            comb.write(dsp::flush(shared + ARENA_COMB_GAINS[i] * fed[i]));
        }
        [
            ARENA_OUTPUT * (input + ARENA_WET * left),
            ARENA_OUTPUT * (input + ARENA_WET * right),
        ]
    }

    fn clear(&mut self) {
        self.predelay.clear();
        self.diffusers.iter_mut().for_each(DelayLine::clear);
        self.combs.iter_mut().for_each(DelayLine::clear);
        self.lowpass.iter_mut().for_each(Biquad::clear);
    }
}

struct Echo {
    sample_rate: f32,
    // Front delay.
    line: DelayLine,
    distance: f32,
    target_distance: f32,
    slew: f32,
    fresh: bool,
    delayed: f32,
    tone: Biquad,
    tone_hz: f32,
    tone_q: f32,
    target_hz: f32,
    target_q: f32,
    tone_countdown: u32,
    input_highpass: Biquad,
    feedback_dc: Biquad,
    feedback: zoom::Ramp,
    // Diffusers and tail.
    combs: [DelayLine; 2],
    comb_delays: [usize; 2],
    comb_out: [f32; 2],
    allpasses: [DelayLine; 2],
    allpass_delays: [usize; 2],
    allpass_out: [f32; 2],
    tail: DelayLine,
    tail_delay: usize,
    tail_out: f32,
    arena: Arena,
    // Output.
    output_eq: [Biquad; 2],
    wet: zoom::Ramp,
    dry: zoom::Ramp,
    stereo: zoom::Ramp,
}

impl Echo {
    fn new(sample_rate: f32) -> Self {
        let resample = |c: Coefficients| c.resample(zoom::ORIGINAL_RATE, sample_rate);
        let max_delay = (MAX_TIME_MS * 0.001 * sample_rate).ceil() as usize + 2;
        let ramp = |value| zoom::Ramp::new(value, zoom::DEFAULT_STEP, sample_rate);
        let mut echo = Self {
            sample_rate,
            line: DelayLine::new(max_delay),
            distance: 0.0,
            target_distance: 0.0,
            slew: 1.0 - (1.0 - SLEW_44K).powf(zoom::ORIGINAL_RATE / sample_rate),
            fresh: true,
            delayed: 0.0,
            tone: Biquad::default(),
            tone_hz: 0.0,
            tone_q: 0.0,
            target_hz: TONE_HZ[7],
            target_q: TONE_Q[7],
            tone_countdown: 0,
            input_highpass: Biquad::new(resample(INPUT_HIGHPASS)),
            feedback_dc: Biquad::new(resample(FEEDBACK_DC)),
            feedback: ramp(0.0),
            combs: COMB_DELAYS.map(|d| DelayLine::new(scaled(d, sample_rate))),
            comb_delays: COMB_DELAYS.map(|d| scaled(d, sample_rate)),
            comb_out: [0.0; 2],
            allpasses: ALLPASS_DELAYS.map(|d| DelayLine::new(scaled(d, sample_rate))),
            allpass_delays: ALLPASS_DELAYS.map(|d| scaled(d, sample_rate)),
            allpass_out: [0.0; 2],
            tail: DelayLine::new(scaled(TAIL_DELAY, sample_rate)),
            tail_delay: scaled(TAIL_DELAY, sample_rate),
            tail_out: 0.0,
            arena: Arena::new(sample_rate),
            output_eq: [Biquad::new(resample(OUTPUT_EQ)); 2],
            wet: ramp(0.0),
            dry: ramp(0.0),
            stereo: zoom::Ramp::new(0.0, 1.0 / (0.016 * zoom::ORIGINAL_RATE), sample_rate),
        };
        echo.design_tone(TONE_HZ[7], TONE_Q[7]);
        echo
    }

    fn design_tone(&mut self, hz: f32, q: f32) {
        self.tone_hz = hz;
        self.tone_q = q;
        self.tone.coefficients = Coefficients::lowpass(self.sample_rate, hz, q);
    }

    /// Move the Tone filter towards its target (block rate, ~1 ms steps).
    #[inline]
    fn update_tone(&mut self) {
        if self.tone_countdown > 0 {
            self.tone_countdown -= 1;
            return;
        }
        self.tone_countdown = TONE_INTERVAL - 1;
        if self.tone_hz == self.target_hz && self.tone_q == self.target_q {
            return;
        }
        let close = |a: f32, b: f32, tolerance: f32| (a - b).abs() <= tolerance;
        let hz = if close(self.tone_hz, self.target_hz, 1.0) {
            self.target_hz
        } else {
            self.tone_hz + (self.target_hz - self.tone_hz) * 0.25
        };
        let q = if close(self.tone_q, self.target_q, 1e-3) {
            self.target_q
        } else {
            self.tone_q + (self.target_q - self.tone_q) * 0.25
        };
        self.design_tone(hz, q);
    }

    #[inline]
    fn front(&mut self, mono: f32) -> f32 {
        // Delay line: input filters plus clipped, DC-blocked feedback.
        let feedback = (self.feedback.tick() * self.delayed).clamp(-1.0, 1.0);
        let feedback = self.feedback_dc.process(feedback);
        let input = self.input_highpass.process(self.tone.process(mono));
        self.line.write(dsp::flush(feedback + input));
        self.distance += (self.target_distance - self.distance) * self.slew;
        let delayed = self.line.read_linear(self.distance);
        self.delayed = delayed;

        // Two combs (c101 = c103 = -1, c104 = 1, c100 = c105 = 1).
        let first_in = -delayed;
        self.combs[0].write(dsp::flush(first_in + COMB_FEEDBACK * self.comb_out[0]));
        self.comb_out[0] = self.combs[0].read(self.comb_delays[0]);
        let second_in = -(delayed + self.comb_out[0]);
        self.combs[1].write(dsp::flush(second_in + COMB_FEEDBACK * self.comb_out[1]));
        self.comb_out[1] = self.combs[1].read(self.comb_delays[1]);

        // Two all-pass sections (c108 = c112 = 1).
        let mut x = delayed + self.comb_out[0] + self.comb_out[1];
        for i in 0..2 {
            self.allpasses[i].write(dsp::flush(x + ALLPASS_GAIN * self.allpass_out[i]));
            let y = self.allpasses[i].read(self.allpass_delays[i]) - ALLPASS_GAIN * x;
            self.allpass_out[i] = y;
            x = y;
        }

        // Tail comb (c93 = c96 = c97 = 1).
        let sum = DELAYED_LEVEL * delayed + DIFFUSED_LEVEL * x;
        self.tail
            .write(dsp::flush(sum + TAIL_FEEDBACK * self.tail_out));
        self.tail_out = self.tail.read(self.tail_delay);
        sum + TAIL_LEVEL * self.tail_out
    }
}

impl Effect for Echo {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let decay = values[0];
        let time = values[1].clamp(0.0, MAX_TIME_MS);
        let tone = values[2];
        let mix = values[3];
        self.feedback.set(table(&FEEDBACK, decay));
        self.wet
            .set(table(&WET_BY_MIX, mix) * table(&WET_BY_DECAY, decay));
        self.dry.set(table(&DRY_BY_MIX, mix));
        self.stereo
            .set(if values[4].round() >= 1.0 { 1.0 } else { 0.0 });
        self.target_hz = table(&TONE_HZ, tone);
        self.target_q = table(&TONE_Q, tone);
        // TIME ms = 44.1 · TIME samples at 44.1 kHz.
        self.target_distance = time * 0.001 * self.sample_rate;
        if self.fresh {
            self.fresh = false;
            self.distance = self.target_distance;
            self.feedback.jump(self.feedback.target);
            self.wet.jump(self.wet.target);
            self.dry.jump(self.dry.target);
            self.stereo.jump(self.stereo.target);
            self.design_tone(self.target_hz, self.target_q);
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        self.update_tone();
        let front = self.front(0.5 * (left + right));
        let arena = self.arena.process(front);
        let wet_left = self.output_eq[0].process(arena[0]);
        let wet_right = self.output_eq[1].process(arena[1]);
        let (wet, dry) = (self.wet.tick(), self.dry.tick());
        let stereo = self.stereo.tick();
        let out_left = dry * left + wet * wet_left;
        let out_right = dry * right + wet * wet_right;
        let mono = 0.5 * (out_left + out_right);
        [
            mono + (out_left - mono) * stereo,
            mono + (out_right - mono) * stereo,
        ]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.distance = self.target_distance;
        self.delayed = 0.0;
        self.tone.clear();
        self.input_highpass.clear();
        self.feedback_dc.clear();
        self.combs.iter_mut().for_each(DelayLine::clear);
        self.allpasses.iter_mut().for_each(DelayLine::clear);
        self.tail.clear();
        self.comb_out = [0.0; 2];
        self.allpass_out = [0.0; 2];
        self.tail_out = 0.0;
        self.arena.clear();
        self.output_eq.iter_mut().for_each(Biquad::clear);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    fn values(decay: f32, time: f32, tone: f32, mix: f32, mode: f32) -> Values {
        let mut v = [0.0; crate::MAX_PARAMS];
        v[..5].copy_from_slice(&[decay, time, tone, mix, mode]);
        v
    }

    #[test]
    fn tables_interpolate_like_tbl_to_val() {
        assert!((table(&FEEDBACK, 25.0) - 0.35).abs() < 1e-6);
        assert!((table(&FEEDBACK, 100.0) - 1.1).abs() < 1e-6);
        assert_eq!(table(&FEEDBACK, 0.0), 0.0);
        assert!((table(&DRY_BY_MIX, 95.0) - 0.35).abs() < 1e-6);
        assert_eq!(table(&TONE_HZ, 70.0), 5000.0);
        // Defaults: wet = 0.794 · 0.725, dry = 0.7.
        let wet = table(&WET_BY_MIX, 80.0) * table(&WET_BY_DECAY, 25.0);
        assert!((wet - 0.793_962_5 * 0.725).abs() < 1e-5, "{wet}");
    }

    #[test]
    fn mix_zero_is_the_attenuated_dry_signal() {
        let mut echo = Echo::new(48_000.0);
        echo.set(&values(25.0, 125.0, 70.0, 0.0, 1.0), 120.0);
        for i in 0..2000 {
            let x = ((i as f32) * 0.05).sin() * 0.5;
            let [l, r] = echo.process([x, -x]);
            assert!((l - 0.7 * x).abs() < 1e-6 && (r + 0.7 * x).abs() < 1e-6);
        }
        // Mono mode averages the sides.
        echo.set(&values(25.0, 125.0, 70.0, 0.0, 0.0), 120.0);
        for _ in 0..4000 {
            echo.process([0.3, 0.1]);
        }
        let [l, r] = echo.process([0.3, 0.1]);
        assert!(
            (l - 0.14).abs() < 1e-5 && (r - 0.14).abs() < 1e-5,
            "{l} {r}"
        );
    }

    fn first_echo(sample_rate: f32, time: f32) -> usize {
        let mut echo = Echo::new(sample_rate);
        echo.set(&values(0.0, time, 100.0, 100.0, 1.0), 120.0);
        let mut best = (0, 0.0f32);
        for i in 0..(sample_rate as usize / 2) {
            let x = if i == 0 { 1.0 } else { 0.0 };
            let y = echo.process([x, x])[0].abs();
            if i > 20 && y > best.1 {
                best = (i, y);
            }
        }
        best.0
    }

    #[test]
    fn time_is_milliseconds_at_any_rate() {
        for (rate, time) in [(44_100.0, 100.0), (48_000.0, 125.0), (96_000.0, 200.0)] {
            let expected = time * 0.001 * rate;
            let at = first_echo(rate, time) as f32;
            // The tone low-pass and 100 Hz high-pass smear the click by a few samples.
            assert!(
                (at - expected).abs() < 0.002 * rate,
                "{rate}: {at} vs {expected}"
            );
        }
    }

    #[test]
    fn time_changes_glide_with_the_original_slew() {
        let rate = 48_000.0;
        let mut echo = Echo::new(rate);
        echo.set(&values(25.0, 0.0, 70.0, 80.0, 0.0), 120.0);
        echo.set(&values(25.0, 100.0, 70.0, 80.0, 0.0), 120.0);
        // τ = 1 / c38 samples at 44.1 kHz ≈ 122 ms.
        let tau = (1.0 / SLEW_44K / zoom::ORIGINAL_RATE * rate) as usize;
        for _ in 0..tau {
            echo.process([0.0; 2]);
        }
        let fraction = echo.distance / echo.target_distance;
        assert!((fraction - 0.632).abs() < 0.01, "{fraction}");
    }

    #[test]
    fn tone_is_the_designed_lowpass() {
        let echo = Echo::new(44_100.0);
        let lowpass = Coefficients::lowpass(44_100.0, table(&TONE_HZ, 0.0), table(&TONE_Q, 0.0));
        assert!(dsp::gain_to_db(lowpass.magnitude(44_100.0, 1000.0)) < -8.0);
        assert!(dsp::gain_to_db(echo.tone.coefficients.magnitude(44_100.0, 100.0)).abs() < 0.5);
        assert!(dsp::gain_to_db(echo.tone.coefficients.magnitude(44_100.0, 15_000.0)) < -10.0);
    }

    #[test]
    fn maximum_decay_stays_bounded() {
        let model = Model::from_key("mooz_echo").unwrap();
        let mut effect = model.create(48_000.0);
        effect.set(&values(100.0, 200.0, 100.0, 100.0, 1.0), 120.0);
        let mut noise = dsp::Noise::new(3);
        let mut peak = 0.0f32;
        for i in 0..48_000 * 6 {
            let x = if i < 48_000 { noise.bipolar() } else { 0.0 };
            let [l, r] = effect.process([x, x]);
            assert!(l.is_finite() && r.is_finite());
            peak = peak.max(l.abs()).max(r.abs());
        }
        assert!(peak < 6.0, "{peak}");
    }

    #[test]
    fn rom_filters_keep_their_shape_at_96k() {
        let at44 = ARENA_LOWPASS;
        let at96 = ARENA_LOWPASS.resample(44_100.0, 96_000.0);
        for hz in [100.0, 1000.0, 4000.0] {
            let a = dsp::gain_to_db(at44.magnitude(44_100.0, hz));
            let b = dsp::gain_to_db(at96.magnitude(96_000.0, hz));
            assert!((a - b).abs() < 0.5, "{hz}: {a} {b}");
        }
    }
}
