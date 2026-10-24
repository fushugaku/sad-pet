//! BendCho (BENDCHO.ZDL): `bend_chorus.rs` (original 0000..0458). Every
//! picked note bends the pitch of a delayed copy into place.
//!
//! Callback: `0.7 (L + R)` through a fixed high-pass biquad (≈30 Hz, −12 dB)
//! and the Tone biquad (+12 dB low-pass) into the ring. A detector on the
//! dry input (`TrgIn` selects Main's dry buffer; here `0.5 (L + R)`) keeps a
//! held peak (decay c25), a fast (c20/c21, gain 1) and a slow (c22/c23,
//! gain 2) envelope; an onset (fast above −64 dB, c24, and above the slow
//! one) rising edge resets the bend phase to 1, which then decays by c28 each
//! sample. The delay is `1 + smoothed * 2048 * c31` samples where
//! `smoothed` follows `c27 + c26 phase²` through the one-pole c29/c30
//! (≈23 ms); the tap is mixed `c2 (wet c5 + dry c6)` into both channels.
//!
//! Editors decoded from the disassembly:
//! - Depth: c31 = curve(0x780f01df, 0x09012023, 101, linear) Q31: 144..1921
//!   samples (3..44 ms) of bend. Ramped over 16 ms here (the original
//!   stores it at once).
//! - Time: c28 from a 51-word ROM table: the bend settles (phase 0.1) in
//!   25..250 ms.
//! - Bal: wet c5 = curve(1, 0, 101, Bal, two-slope), dry c6 = the same of
//!   100 − Bal, timed setter.
//! - Mode: Up gives c26 = −1, c27 = 1 (the delay grows from 0 to its rest at
//!   the maximum: the note bends up into pitch), Down c26 = 1, c27 = 0.
//! - Tone: a 5-word biquad row into c12..c16. Level: c2 = Level/100.
use super::common::{
    TimedRamp, UNSET, decay, defaults, one_pole, rom_biquad, sanitize, step, word,
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, DelayLine, flush},
    zoom::{self, ORIGINAL_RATE, Ramp, curve},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_bend_cho",
    name: "BendCho",
    category: Category::Modulation,
    zdl: "BENDCHO.ZDL",
    description: "Each picked note bends a delayed copy into pitch",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 40,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Time",
            max: 50,
            default: 50,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Bal",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Mode",
            max: 1,
            default: 0,
            display: Display::labels(&["Up", "Down"]),
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

/// ROM c7..c11: high-pass ≈30 Hz with a 0.25 passband.
const INPUT: [u32; 5] = [0x3e803770, 0xbf003770, 0x3e803770, 0x3fff37ed, 0xbf7e710d];

/// Tone rows (BENDCHO 80000430): +12 dB low-pass biquads, row 10 open.
const TONE: [[u32; 5]; 11] = [
    [0x3b4b8280, 0x3bcb8280, 0x3b4b8280, 0x3ff597a5, 0xbf6bfbc5],
    [0x3b9d6e80, 0x3c1d6e78, 0x3b9d6e80, 0x3ff2ff18, 0xbf673a8c],
    [0x3c174cf0, 0x3c974cf0, 0x3c174cf0, 0x3fedd085, 0xbf5e011f],
    [0x3cb3ce54, 0x3d33ce54, 0x3cb3ce54, 0x3fe37fc7, 0xbf4ca4d7],
    [0x3d61c030, 0x3de1c030, 0x3d61c030, 0x3fd1a4d2, 0xbf3176d3],
    [0x3dfa95ea, 0x3e7a95ea, 0x3dfa95ea, 0x3fb8a9a2, 0xbf10cc22],
    [0x3e688836, 0x3ee88837, 0x3e688836, 0x3f9b9492, 0xbee323e7],
    [0x3ec598e8, 0x3f4598e8, 0x3ec598e8, 0x3f711e08, 0xbea8c57c],
    [0x3f1bb8d2, 0x3f9bb8d2, 0x3f1bb8d2, 0x3f205d0f, 0xbe734daf],
    [0x3f6b7b36, 0x3feb7b36, 0x3f6b7b36, 0x3e820a8c, 0xbe367c6c],
    [0x404ed99b, 0x40ced99b, 0x404ed99b, 0xbfcb5655, 0xbf28a8d7],
];

/// Time table (BENDCHO 800005e0): per-sample bend phase decay c28.
const TIME: [u32; 51] = [
    0x3f7f7745, 0x3f7f8c1c, 0x3f7f9b6f, 0x3f7fa72f, 0x3f7fb079, 0x3f7fb800, 0x3f7fbe3b, 0x3f7fc377,
    0x3f7fc7ee, 0x3f7fcbc7, 0x3f7fcf23, 0x3f7fd216, 0x3f7fd4b3, 0x3f7fd708, 0x3f7fd921, 0x3f7fdb05,
    0x3f7fdcbc, 0x3f7fde4c, 0x3f7fdfba, 0x3f7fe10a, 0x3f7fe240, 0x3f7fe35f, 0x3f7fe469, 0x3f7fe560,
    0x3f7fe647, 0x3f7fe71e, 0x3f7fe7e8, 0x3f7fe8a6, 0x3f7fe958, 0x3f7fea00, 0x3f7fea9e, 0x3f7feb34,
    0x3f7febc1, 0x3f7fec48, 0x3f7fecc7, 0x3f7fed41, 0x3f7fedb4, 0x3f7fee22, 0x3f7fee8b, 0x3f7feef0,
    0x3f7fef4f, 0x3f7fefab, 0x3f7ff003, 0x3f7ff057, 0x3f7ff0a8, 0x3f7ff0f6, 0x3f7ff141, 0x3f7ff188,
    0x3f7ff1ce, 0x3f7ff210, 0x3f7ff250,
];

/// Detector thresholds and envelopes (ROM c17..c25, c29/c30).
const THRESHOLD: u32 = 0x3a2566d3;
const PEAK_DECAY: u32 = 0x3f7f7cee;
const FAST: (u32, u32) = (0x3d8f5c29, 0x3f6e147b);
const SLOW: (u32, u32) = (0x3c23d70a, 0x3f7eb852);
const SMOOTH: (u32, u32) = (0x3a83126f, 0x3f7fbe77);

struct BendCho {
    sample_rate: f32,
    scale: f32,
    line: DelayLine,
    input: Biquad,
    tone: Biquad,
    threshold: f32,
    peak_decay: f32,
    fast_coefficients: (f32, f32),
    slow_coefficients: (f32, f32),
    smooth_coefficients: (f32, f32),
    phase_decay: f32,
    bend: f32,
    rest: f32,
    peak: f32,
    fast: f32,
    slow: f32,
    onset: bool,
    phase: f32,
    smoothed: f32,
    depth: TimedRamp,
    wet: TimedRamp,
    dry: TimedRamp,
    level: Ramp,
    last: Values,
}

fn coefficients(pair: (u32, u32), sample_rate: f32) -> (f32, f32) {
    one_pole(word(pair.0), word(pair.1), sample_rate)
}

impl BendCho {
    fn new(sample_rate: f32) -> Self {
        let scale = sample_rate / ORIGINAL_RATE;
        let mut effect = Self {
            sample_rate,
            scale,
            line: DelayLine::new((2050.0 * scale).ceil() as usize + 8),
            input: Biquad::new(rom_biquad(INPUT, sample_rate)),
            tone: Biquad::default(),
            threshold: word(THRESHOLD),
            peak_decay: decay(word(PEAK_DECAY), sample_rate),
            fast_coefficients: coefficients(FAST, sample_rate),
            slow_coefficients: coefficients(SLOW, sample_rate),
            smooth_coefficients: coefficients(SMOOTH, sample_rate),
            phase_decay: 0.0,
            bend: -1.0,
            rest: 1.0,
            peak: 0.0,
            fast: 0.0,
            slow: 0.0,
            onset: false,
            phase: 0.0,
            smoothed: 1.0,
            depth: TimedRamp::new(0.0, sample_rate),
            wet: TimedRamp::new(0.0, sample_rate),
            dry: TimedRamp::new(0.0, sample_rate),
            level: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
            last: UNSET,
        };
        effect.set(&defaults(&INFO), 120.0);
        effect.depth.settle();
        effect.wet.settle();
        effect.dry.settle();
        effect.level.jump(effect.level.target);
        effect.smoothed = effect.rest;
        effect
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(BendCho::new(sample_rate))
}

impl Effect for BendCho {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let v = sanitize(values, &INFO);
        let last = self.last;
        if v[0] != last[0] {
            let q31 = curve(2_014_249_439.0, 151_068_707.0, 101, v[0], 0);
            self.depth
                .set(q31 * super::common::Q31 * 2048.0 * self.scale);
        }
        if step(v[1], 50) != step(last[1], 50) || last[1].is_nan() {
            self.phase_decay = decay(word(TIME[step(v[1], 50)]), self.sample_rate);
        }
        if v[2] != last[2] {
            self.wet.set(curve(1.0, 0.0, 101, v[2], 2));
            self.dry.set(curve(1.0, 0.0, 101, 100.0 - v[2], 2));
        }
        if step(v[3], 1) != step(last[3], 1) || last[3].is_nan() {
            (self.bend, self.rest) = if step(v[3], 1) == 0 {
                (-1.0, 1.0)
            } else {
                (1.0, 0.0)
            };
        }
        if step(v[4], 10) != step(last[4], 10) || last[4].is_nan() {
            self.tone.coefficients = rom_biquad(TONE[step(v[4], 10)], self.sample_rate);
        }
        if v[5] != last[5] {
            self.level.set(curve(1.5, 0.0, 151, v[5], 0));
        }
        self.last = v;
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let filtered = self.tone.process(self.input.process(0.7 * (left + right)));

        let magnitude = (0.5 * (left + right)).abs();
        self.peak = if self.peak < magnitude {
            magnitude
        } else {
            flush(self.peak * self.peak_decay)
        };
        let (fast_in, fast_pole) = self.fast_coefficients;
        let (slow_in, slow_pole) = self.slow_coefficients;
        self.fast = flush(self.fast * fast_pole + fast_in * self.peak);
        self.slow = flush(self.slow * slow_pole + slow_in * self.peak);
        let onset = self.threshold < self.fast && self.slow < self.fast;
        let start = if onset && !self.onset {
            1.0
        } else {
            self.phase
        };
        self.onset = onset;
        self.phase = flush(self.phase_decay * start);
        let target = self.rest + self.bend * self.phase * self.phase;
        let (smooth_in, smooth_pole) = self.smooth_coefficients;
        self.smoothed = self.smoothed * smooth_pole + target * smooth_in;

        self.line.write(filtered);
        let delay = self.scale + self.smoothed * self.depth.tick();
        let tap = self.line.read_linear(delay);
        let wet = self.wet.tick() * tap;
        let dry = self.dry.tick();
        let level = self.level.tick();
        [level * (wet + left * dry), level * (wet + right * dry)]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.input.clear();
        self.tone.clear();
        self.peak = 0.0;
        self.fast = 0.0;
        self.slow = 0.0;
        self.onset = false;
        self.phase = 0.0;
        self.smoothed = self.rest;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn delay_after_onset(mode: f32) -> Vec<f32> {
        let mut effect = BendCho::new(48_000.0);
        let mut values = defaults(&INFO);
        values[3] = mode;
        effect.set(&values, 120.0);
        effect.clear();
        let mut delays = Vec::new();
        for i in 0..48_000 {
            let x = if i >= 4_800 {
                0.5 * (i as f32 * 0.05).sin()
            } else {
                0.0
            };
            effect.process([x, x]);
            delays.push(effect.smoothed);
        }
        delays
    }

    #[test]
    fn up_mode_bends_from_short_to_rest() {
        let delays = delay_after_onset(0.0);
        assert!((delays[4_000] - 1.0).abs() < 1e-3);
        // Right after the onset the delay drops toward 0, then returns.
        let low = delays[4_800..12_000].iter().cloned().fold(1.0, f32::min);
        assert!(low < 0.55, "{low}");
        assert!((delays[47_999] - 1.0).abs() < 0.01);
    }

    #[test]
    fn down_mode_rests_at_zero_delay() {
        let delays = delay_after_onset(1.0);
        assert!(delays[4_000].abs() < 1e-3);
        let high = delays[4_800..12_000].iter().cloned().fold(0.0, f32::max);
        assert!(high > 0.45, "{high}");
        assert!(delays[47_999].abs() < 0.01);
    }

    #[test]
    fn depth_spans_three_to_forty_four_ms() {
        let q = |d| curve(2_014_249_439.0, 151_068_707.0, 101, d, 0) * super::super::common::Q31;
        assert!(((1.0 + q(0.0) * 2048.0) / 44.1 - 3.29).abs() < 0.05);
        assert!(((1.0 + q(100.0) * 2048.0) / 44.1 - 43.6).abs() < 0.1);
    }
}
