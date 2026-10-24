//! CloneCho (CLONECHO.ZDL): `clone_chorus.rs` (wrapper 0ba0..0c20 around
//! SUB_CloneCho 0000..06a0), a fixed-mix chorus voiced by six ROM biquads.
//!
//! Callback: the mono mean `0.5 (L + R)` goes straight into the ring. A Q31
//! triangle `|phase| - 0.5` passes a one-pole (c49/c50, depth dependent) and
//! reads one tap at `center + (lfo + 0.5) * depth`. The tap (gain c8, the tap
//! mute) passes three "pre" biquads, is mixed `0.726 (mean + chorus)` and
//! passes three "post" biquads; the result times c72 = −0.94 is the output on
//! both channels (mono, polarity inverted as in the original).
//!
//! Editors decoded from the disassembly:
//! - DEPTH (shown 1/2): a 2-row table sets center 7.3 / 8.7 ms, depth 1.0 /
//!   5.7 ms and the LFO pole (≈8.6 / 3 Hz). The original closes the tap
//!   (16 ms), switches and reopens it; so does this port.
//! - RATE: c53 = 34535 + 823284 (Rate/100)^2.44: 0.35..8.8 Hz.
use super::common::{
    Q31, TimedRamp, UNSET, defaults, increment, one_pole, rom_biquad, sanitize, step, triangle,
    word,
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, DelayLine},
    zoom::ORIGINAL_RATE,
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_clone_cho",
    name: "CloneCho",
    category: Category::Modulation,
    zdl: "CLONECHO.ZDL",
    description: "Classic voiced chorus with two depth settings",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "DEPTH",
            max: 1,
            default: 1,
            display: Display::offset(1.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "RATE",
            max: 100,
            default: 23,
            display: Display::INT,
        },
    ],
};

/// ROM c34..c48: the three biquads after the tap.
const PRE: [[u32; 5]; 3] = [
    [0x3e0bfef3, 0x3e8bfef3, 0x3e0bfef3, 0x3f881beb, 0xbefed773],
    [0x3f74c609, 0xbfc3bb3e, 0x3f346e8f, 0x3fc3bb3e, 0xbf293498],
    [0x3dde67c7, 0x3e5e67c7, 0x3dde67c7, 0x3f881beb, 0xbefed773],
];
/// ROM c57..c71: the three biquads after the mix.
const POST: [[u32; 5]; 3] = [
    [0x3f7f03e5, 0xbfff03e5, 0x3f7f03e5, 0x3fff02ee, 0xbf7e09b9],
    [0x3f74bb17, 0xbfa86434, 0x3ee7babe, 0x3fa86434, 0xbed130ec],
    [0x3fb18489, 0x3e1cd9eb, 0xbec31151, 0xbe1cd9eb, 0xbbc034a0],
];
/// c10 = c11: mix of mean and chorus; c72: output gain.
const MIX: u32 = 0x3f39db23;
const OUTPUT: u32 = 0xbf70a3d7;
/// DEPTH table (800005f8, ms) and LFO smoothing inputs (80000608, Q31).
const CENTER_MS: [f32; 2] = [7.3, 8.7];
const DEPTH_MS: [f32; 2] = [1.0, 5.7];
const SMOOTHING: [u32; 2] = [0x0028_14b6, 0x000d_fd4d];

struct CloneCho {
    sample_rate: f32,
    scale: f32,
    line: DelayLine,
    phase: u32,
    increment: u32,
    smoothing: (f32, f32),
    shape: f32,
    center: f32,
    depth: f32,
    active: usize,
    pending: Option<usize>,
    tap: TimedRamp,
    pre: [Biquad; 3],
    post: [Biquad; 3],
    last: Values,
}

impl CloneCho {
    fn new(sample_rate: f32) -> Self {
        let scale = sample_rate / ORIGINAL_RATE;
        let biquads =
            |rows: &[[u32; 5]; 3]| rows.map(|row| Biquad::new(rom_biquad(row, sample_rate)));
        let mut effect = Self {
            sample_rate,
            scale,
            line: DelayLine::new(((384.0 + 252.0) * scale).ceil() as usize + 8),
            phase: 0,
            increment: 0,
            smoothing: (0.0, 0.0),
            shape: 0.0,
            center: 0.0,
            depth: 0.0,
            active: 0,
            pending: None,
            tap: TimedRamp::new(1.0, sample_rate),
            pre: biquads(&PRE),
            post: biquads(&POST),
            last: UNSET,
        };
        effect.set(&defaults(&INFO), 120.0);
        if let Some(depth) = effect.pending.take() {
            effect.switch(depth);
        }
        effect.tap.jump(1.0);
        effect
    }

    fn switch(&mut self, depth: usize) {
        self.active = depth;
        self.center = (CENTER_MS[depth] * 44.1).round() * self.scale;
        self.depth = (DEPTH_MS[depth] * 44.1).round() * self.scale;
        let input = SMOOTHING[depth] as f32 * Q31;
        self.smoothing = one_pole(input, 1.0 - input, self.sample_rate);
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(CloneCho::new(sample_rate))
}

impl Effect for CloneCho {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let v = sanitize(values, &INFO);
        let depth = step(v[0], 1);
        if self.last[0].is_nan() || depth != self.pending.unwrap_or(self.active) {
            self.pending = Some(depth);
            self.tap.set(0.0);
        }
        if v[1] != self.last[1] {
            let t = (v[1] / 100.0).powf(2.44);
            self.increment = increment((34_535.0 + 823_284.0 * t).round(), self.sample_rate);
        }
        self.last = v;
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mean = 0.5 * (left + right);
        self.line.write(mean);

        self.phase = self.phase.wrapping_add(self.increment);
        let (smooth_in, smooth_pole) = self.smoothing;
        self.shape = (triangle(self.phase) - 0.5) * smooth_in + self.shape * smooth_pole;
        let gain = self.tap.tick();
        if gain == 0.0
            && let Some(depth) = self.pending.take()
        {
            self.switch(depth);
            self.tap.set(1.0);
        }
        let delay = self.center + (self.shape + 0.5) * self.depth;
        let mut chorus = self.line.read_linear(delay) * gain;
        for biquad in &mut self.pre {
            chorus = biquad.process(chorus);
        }
        let mut out = word(MIX) * (mean + chorus);
        for biquad in &mut self.post {
            out = biquad.process(out);
        }
        let out = word(OUTPUT) * out;
        [out, out]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.shape = 0.0;
        for biquad in self.pre.iter_mut().chain(&mut self.post) {
            biquad.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::chorus_std::common::increment_hz;

    #[test]
    fn depth_switch_closes_switches_and_reopens_the_tap() {
        let mut effect = CloneCho::new(48_000.0);
        assert_eq!(effect.active, 1);
        assert!((effect.center / 48.0 - 384.0 / 44.1).abs() < 0.01);
        let mut values = defaults(&INFO);
        values[0] = 0.0;
        effect.set(&values, 120.0);
        let mut closed = false;
        for _ in 0..2000 {
            effect.process([0.1, 0.1]);
            closed |= effect.tap.current == 0.0;
        }
        assert!(closed);
        assert_eq!(effect.active, 0);
        assert_eq!(effect.tap.current, 1.0);
        assert!((effect.depth / 48.0 - 44.0 / 44.1).abs() < 0.01);
    }

    #[test]
    fn rate_curve_and_mono_output() {
        let mut effect = CloneCho::new(48_000.0);
        let mut values = defaults(&INFO);
        values[1] = 100.0;
        effect.set(&values, 120.0);
        assert!((increment_hz(effect.increment, 48_000.0) - 8.81).abs() < 0.02);
        let mut energy = (0.0, 0.0);
        for i in 0..48_000 {
            let x = (i as f32 * 0.04).sin() * 0.5;
            let [l, r] = effect.process([x, 0.2 * x]);
            assert_eq!(l, r);
            energy.0 += (0.6 * x) * (0.6 * x);
            energy.1 += l * l;
        }
        let ratio = (energy.1 / energy.0).sqrt();
        assert!(ratio > 0.5 && ratio < 2.0, "{ratio}");
    }
}
