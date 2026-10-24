//! WarpPhase (WRPPHASE.ZDL), from the recovered `Fx_MOD_WarpPhase` callback
//! (`warp_phase.rs`) and its decoded editors.
//!
//! Callback, per sample: one phase drives two eight-stage all-pass banks half
//! a cycle apart. Each bank's coefficient follows a saw (`0.5 + phase / 2`)
//! through `1 − 0.58 (0.34 h + 0.66 h²)` (c6..9), and its output gain is
//! `1 − min(1.07 |phase|, 1)`, so each bank fades out at the end of its sweep
//! while the other fades in: an endless rising (Go) or falling (Back) sweep.
//! Each bank adds its input to its all-pass output (c10 = 1) and feeds its
//! last stage back with Reso (c11); the banks are summed × 0.7 (c12) and
//! Level (c2) scales the mono result on both outputs.
//!
//! Editors: Speed 0 → increment 0x22f2, 1..49 → `context[22]` 0x45e6..0xb9af6
//! square law (as Phaser), above 49 tempo notes at half the note rate (the
//! original shifts the synced increment right by one); DRCTN Back negates it.
//! Reso 0..10 → 0.5..0.9 (`context[21]`, ramp over 705.6 samples). Level
//! 0..1.5. All editors decoded; the 44.1 kHz all-pass weights are re-warped to
//! the host rate.
use super::common::{AllpassWarp, Lfo, Ramp, allpass, guard, increment_hz, rate_hz, stepped};
use super::phaser::rate_increment;
use crate::zoom::{self, DEFAULT_STEP};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_warp_phase",
    name: "WarpPhase",
    category: Category::Modulation,
    zdl: "WRPPHASE.ZDL",
    description: "Endless rising or falling phaser sweep",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Speed",
            max: 77,
            default: 24,
            display: Display::sync_rate(49, 1.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Reso",
            max: 10,
            default: 7,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "DRCTN",
            max: 1,
            default: 0,
            display: Display::labels(&["Go", "Back"]),
        },
    ],
};

struct Bank {
    denormals: super::common::Denormals,
    stages: [f32; 8],
    last: f32,
}

impl Bank {
    #[inline]
    fn process(&mut self, input: f32, weight: f32) -> f32 {
        let mut value = input;
        for memory in &mut self.stages {
            value = allpass(memory, value, weight);
        }
        self.last = value;
        self.denormals.sweep(&mut self.stages);
        value
    }
}

struct WarpPhase {
    sample_rate: f32,
    warp: AllpassWarp,
    lfo: Lfo,
    banks: [Bank; 2],
    reso: Ramp,
    level: Ramp,
    first: bool,
    speed: [f32; 2],
    bpm: f32,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(new(sample_rate))
}

fn new(sample_rate: f32) -> WarpPhase {
    WarpPhase {
        sample_rate,
        warp: AllpassWarp::new(sample_rate),
        lfo: Lfo::default(),
        banks: [
            Bank {
                denormals: Default::default(),
                stages: [0.0; 8],
                last: 0.0,
            },
            Bank {
                denormals: Default::default(),
                stages: [0.0; 8],
                last: 0.0,
            },
        ],
        reso: Ramp::with_duration(0.78, 705.6, sample_rate),
        level: Ramp::with_step(1.0, DEFAULT_STEP, sample_rate),
        first: true,
        speed: [f32::NAN; 2],
        bpm: 0.0,
    }
}

/// Sweep weight and bank gain for one bank's phase.
#[inline]
fn bank_shape(s: f32) -> (f32, f32) {
    let half = 0.5 * s + 0.5;
    let shaped = 0.342_3 * half + 0.657_7 * half * half;
    let weight = 0.999_997 - 0.581_749_6 * shaped;
    let gain = 1.0 - (1.07 * s.abs()).min(1.0);
    (weight, gain)
}

impl Effect for WarpPhase {
    fn set(&mut self, values: &Values, bpm: f32) {
        let [speed, reso, level, direction] = [values[0], values[1], values[2], values[3]];
        if [speed, direction] != self.speed || bpm != self.bpm {
            let hz = if zoom::is_synced(speed, 49) {
                0.5 * rate_hz(speed, 49, bpm, |_| 0.0)
            } else {
                increment_hz(stepped(speed.min(49.0), rate_increment))
            };
            self.lfo.set_hz(hz, self.sample_rate);
            if direction.round() >= 1.0 {
                self.lfo.increment = self.lfo.increment.wrapping_neg();
            }
            self.speed = [speed, direction];
            self.bpm = bpm;
        }
        self.reso.set(0.5 + 0.04 * reso.clamp(0.0, 10.0));
        self.level.set(level.clamp(0.0, 150.0) * 0.01);
        if self.first {
            self.first = false;
            self.reso.jump(self.reso.target);
            self.level.jump(self.level.target);
        }
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let s = self.lfo.tick();
        let (weight0, gain0) = bank_shape(s);
        let (weight1, gain1) = bank_shape(super::common::signed(self.lfo.phase ^ 0x8000_0000));
        let driven = 0.5 * (left + right);
        let reso = self.reso.tick();
        let first_input = self.banks[0].last * reso + driven;
        let second_input = driven + self.banks[1].last * reso;
        let first = self.banks[0].process(first_input, self.warp.map(weight0)) + first_input;
        let second = self.banks[1].process(second_input, self.warp.map(weight1)) + second_input;
        let wet = 0.7 * (second * gain1 + gain0 * first);
        let out = guard(wet * self.level.tick());
        [out, out]
    }

    fn clear(&mut self) {
        for bank in &mut self.banks {
            bank.stages = [0.0; 8];
            bank.last = 0.0;
        }
        self.lfo.phase = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banks_crossfade_with_constant_total_weight() {
        for i in 0..100 {
            let s = i as f32 / 50.0 - 1.0;
            let (_, a) = bank_shape(s);
            let other = if s >= 0.0 { s - 1.0 } else { s + 1.0 };
            let (_, b) = bank_shape(other);
            assert!(a + b > 0.85 && a + b <= 1.0001, "{s}: {a} {b}");
        }
    }

    #[test]
    fn synced_speed_runs_at_half_the_note() {
        let mut effect = new(48_000.0);
        // Speed 57 = index 7 = quarter note: 2 Hz at 120 BPM, halved.
        effect.set(&[57.0, 7.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 120.0);
        let hz = f64::from(effect.lfo.increment) * 48_000.0 / 4_294_967_296.0;
        assert!((hz - 1.0).abs() < 1e-3, "{hz}");
        // Back runs the same speed downwards.
        effect.set(&[57.0, 7.0, 100.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0], 120.0);
        let hz = f64::from(effect.lfo.increment.wrapping_neg()) * 48_000.0 / 4_294_967_296.0;
        assert!((hz - 1.0).abs() < 1e-3, "{hz}");
    }

    #[test]
    fn resonant_sweep_stays_bounded() {
        let mut effect = new(96_000.0);
        effect.set(&[49.0, 10.0, 150.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 120.0);
        for i in 0..96_000 {
            let x = if i % 300 == 0 { 1.0 } else { 0.0 };
            let [l, r] = effect.process([x, x]);
            assert!(l.is_finite() && l.abs() < 16.0 && l == r);
        }
    }
}
