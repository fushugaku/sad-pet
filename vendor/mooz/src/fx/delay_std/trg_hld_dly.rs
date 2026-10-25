//! TrgHldDly (TGHLDDLY.ZDL), from the recovered `Fx_DLY_TrgHldDly`
//! (`trigger_hold_delay.rs`) and the decoded editors.
//!
//! Callback: a detector high-passes both inputs (~50 Hz, ×2 [7..9]), takes the
//! larger magnitude into a peak hold (decay [19] = 0.999/sample) and two
//! followers: a fast one ([14]/[15], τ ≈ 14 samples) and a slow one with gain
//! 2 ([16]/[17], τ ≈ 200 samples). An attack starts where the fast follower
//! is above the Threshold [18] and above the slow one. On an attack the hold target [12] drops to 0 and the capture target
//! [13] rises to 1: the ring records the mono input for `Duty` of the Time.
//! When Time has elapsed the hold target returns to 1 and the ring loops its
//! content (`hold · envelope · tap` recirculates), while an envelope
//! (`min(100·slow, 1)`, gated below 0.05, one-pole 0.002) keeps the loop alive
//! only while the player keeps playing. Hold and capture glide with one-poles
//! (0.01/0.99). Output: `Level·(dry + Mix·envelope·hold·tap)`.
//!
//! Editors: Time is `v + 10` ms, Duty `v + 25` % of it; a live Time/Duty edit
//! mutes the tap gain [3] in 16 ms and reopens it after `ms + 10` ms. THRSH is
//! `10^((-20 - v)/20)` (−20..−50 dB), Mix `(v/100)^1.41` with a 16 ms ramp,
//! Level `v/100`. The detector listens to the effect input (the original's
//! TrgIn editor selects the chain's dry input, the same signal here).
use super::common::{self, Biquad, DelayLine, Fade, Gate, Ramp, f, flush};
use crate::{Effect, Values, catalog::*, zoom};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_trg_hld_dly",
    name: "TrgHldDly",
    category: Category::Delay,
    zdl: "TGHLDDLY.ZDL",
    description: "Picking attacks capture a slice that loops while you play",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Time",
            max: 990,
            default: 40,
            display: Display::number(10.0, 1.0, 0, " ms"),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Duty",
            max: 75,
            default: 75,
            display: Display::number(25.0, 1.0, 0, "%"),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "THRSH",
            max: 30,
            default: 20,
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

const MAX_MS: f32 = 1000.0;

/// ROM 0x800004fc.
const DETECTOR: [u32; 5] = [0x3ffe8f7f, 0xbffe8f7f, 0x00000000, 0x3f7e2eca, 0x00000000];
const PEAK_DECAY: u32 = 0x3f7fbe77;
const FAST_POLE: u32 = 0x3f6e147b;
const SLOW_POLE: u32 = 0x3f7eb852;
/// The slow follower's input gain [16] over its complement: 0.01 / 0.005.
const SLOW_GAIN: f32 = 2.0;
const ENVELOPE_SCALE: u32 = 0x42c80000;
const ENVELOPE_FLOOR: u32 = 0x3d4ccccd;
const ENVELOPE_POLE: u32 = 0x3f7f7cee;
const HOLD_POLE: u32 = 0x3f7d70a4;

pub struct TrgHldDly {
    sample_rate: f32,
    line: DelayLine,
    detectors: [Biquad; 2],
    peak_decay: f32,
    fast_pole: f32,
    slow_pole: f32,
    envelope_pole: f32,
    hold_pole: f32,
    peak: f32,
    fast: f32,
    slow: f32,
    triggered: bool,
    elapsed: usize,
    hold_target: f32,
    capture_target: f32,
    hold: f32,
    capture: f32,
    envelope: f32,
    threshold: f32,
    gate: Gate,
    length: usize,
    duty: usize,
    pending: (usize, usize),
    reopen: u32,
    muting: bool,
    tap: f32,
    mix: Fade,
    level: Ramp,
    key: (u32, u32),
    started: bool,
}

impl TrgHldDly {
    pub fn new(sample_rate: f32) -> Self {
        let detector = common::rom_biquad(DETECTOR, sample_rate);
        Self {
            sample_rate,
            line: common::line_for_ms(MAX_MS, 2, sample_rate),
            detectors: [Biquad::new(detector); 2],
            peak_decay: common::pole_at(f(PEAK_DECAY), sample_rate),
            fast_pole: common::pole_at(f(FAST_POLE), sample_rate),
            slow_pole: common::pole_at(f(SLOW_POLE), sample_rate),
            envelope_pole: common::pole_at(f(ENVELOPE_POLE), sample_rate),
            hold_pole: common::pole_at(f(HOLD_POLE), sample_rate),
            peak: 0.0,
            fast: 0.0,
            slow: 0.0,
            triggered: false,
            elapsed: 0,
            hold_target: 0.0,
            capture_target: 1.0,
            hold: 0.0,
            capture: 0.0,
            envelope: 0.0,
            threshold: 0.03,
            gate: Gate::new(sample_rate, true),
            length: 1,
            duty: 1,
            pending: (1, 1),
            reopen: 0,
            muting: false,
            tap: 0.0,
            mix: Fade::new(0.0),
            level: common::setter(1.0, sample_rate),
            key: (u32::MAX, u32::MAX),
            started: false,
        }
    }

    pub fn threshold(value: f32) -> f32 {
        crate::dsp::db_to_gain(-20.0 - common::param(value, 30.0).round())
    }

    #[inline]
    fn detect(&mut self, left: f32, right: f32) {
        let a = self.detectors[0].process(left).abs();
        let b = self.detectors[1].process(right).abs();
        let candidate = a.max(b);
        self.peak = (self.peak * self.peak_decay).max(candidate);
        self.fast = flush(self.peak + (self.fast - self.peak) * self.fast_pole);
        self.slow =
            flush((1.0 - self.slow_pole) * SLOW_GAIN * self.peak + self.slow * self.slow_pole);
        let trigger = self.fast >= self.threshold && self.slow < self.fast;
        let edge = trigger && !self.triggered;
        self.triggered = trigger;
        if edge {
            self.elapsed = 0;
            self.hold_target = 0.0;
            self.capture_target = 1.0;
        }
        self.elapsed += 1;
        if self.elapsed >= self.length {
            self.elapsed = self.length;
            self.hold_target = 1.0;
        }
        if self.elapsed >= self.duty {
            self.capture_target = 0.0;
        }
        let mut scaled = (self.slow * f(ENVELOPE_SCALE)).min(1.0);
        if scaled < f(ENVELOPE_FLOOR) {
            scaled = 0.0;
        }
        self.hold = flush(self.hold_target + (self.hold - self.hold_target) * self.hold_pole);
        self.capture =
            flush(self.capture_target + (self.capture - self.capture_target) * self.hold_pole);
        self.envelope = flush(scaled + (self.envelope - scaled) * self.envelope_pole);
    }
}

impl Effect for TrgHldDly {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let ms = common::param(values[0], 990.0).round() + 10.0;
        let duty = common::param(values[1], 75.0).round() + 25.0;
        let mix = zoom::curve(1.0, 0.0, 101, common::param(values[2], 100.0), 5);
        let level = common::param(values[4], 150.0) / 100.0;
        self.threshold = Self::threshold(values[3]);
        self.level.set(level);
        let ramp = common::TAP_RAMP_44K * common::rate_ratio(self.sample_rate);
        if self.started {
            if mix != self.mix.target {
                self.mix.start(mix, ramp);
            }
        } else {
            self.mix.jump(mix);
            self.level.jump(level);
        }
        let key = (ms.to_bits(), duty.to_bits());
        if key != self.key {
            self.key = key;
            let length = common::tap_samples(ms, self.sample_rate, &self.line).max(1);
            self.pending = (length, (length as f32 * duty / 100.0) as usize);
            self.reopen = common::ms_to_samples(ms + 10.0, self.sample_rate) as u32;
            if self.started {
                self.muting = true;
                self.gate.mute();
            } else {
                (self.length, self.duty) = self.pending;
            }
        }
        self.started = true;
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let level = self.level.tick();
        let mix = self.mix.tick();
        if self.muting && self.gate.closed() {
            self.muting = false;
            (self.length, self.duty) = self.pending;
            self.gate.open_after(self.reopen);
        }
        let gain = self.gate.tick();
        self.detect(left, right);
        let mono = 0.5 * (left + right);
        let looped = self.envelope * self.tap * self.hold;
        self.line
            .write(looped + mono * (1.0 - self.hold) * self.capture);
        self.tap = flush(gain * self.line.read(self.length));
        let wet = self.envelope * self.tap * self.hold * mix;
        [level * (left + wet), level * (right + wet)]
    }

    fn clear(&mut self) {
        self.line.clear();
        for detector in &mut self.detectors {
            detector.clear();
        }
        self.peak = 0.0;
        self.fast = 0.0;
        self.slow = 0.0;
        self.envelope = 0.0;
        self.hold = 0.0;
        self.capture = 0.0;
        self.triggered = false;
        self.tap = 0.0;
        if self.muting {
            self.muting = false;
            (self.length, self.duty) = self.pending;
        }
        self.gate.clear();
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(TrgHldDly::new(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::super::common::testing::*;
    use super::*;
    use crate::Model;

    fn defaults() -> Values {
        Model::from_key(INFO.key).unwrap().defaults()
    }

    /// A plucked note: a sharp attack then a slow decay.
    fn pluck(sample_rate: f32) -> impl FnMut(usize) -> [f32; 2] {
        move |i| {
            let t = i as f32 / sample_rate;
            let x = 0.5 * (-t * 2.0).exp() * (std::f32::consts::TAU * 330.0 * t).sin();
            [x, x]
        }
    }

    #[test]
    fn an_attack_captures_and_then_loops() {
        let sample_rate = 48_000.0;
        let mut effect = TrgHldDly::new(sample_rate);
        // Time 40 -> 50 ms, Duty 100 %.
        effect.set(&defaults(), 120.0);
        assert_eq!((effect.length, effect.duty), (2400, 2400));
        let out = render(&mut effect, 24_000, pluck(sample_rate));
        let mut input = pluck(sample_rate);
        // After the first period the loop adds to the dry note.
        let wet: f32 = (6000..24_000)
            .map(|i| (out[i][0] - input(i)[0]).abs())
            .sum::<f32>()
            / 18_000.0;
        assert!(wet > 0.02, "{wet}");
        assert_eq!(effect.hold_target, 1.0);
    }

    #[test]
    fn silence_does_not_trigger() {
        let mut effect = TrgHldDly::new(44_100.0);
        effect.set(&defaults(), 120.0);
        let out = render(&mut effect, 44_100, |_| [1e-5, 1e-5]);
        assert!(out.iter().all(|f| (f[0] - 1e-5).abs() < 1e-9));
        assert!(!effect.triggered);
    }

    #[test]
    fn threshold_in_decibels() {
        assert!((TrgHldDly::threshold(0.0) - 0.1).abs() < 1e-6);
        assert!((TrgHldDly::threshold(30.0) - 0.003_162_3).abs() < 1e-6);
    }
}
