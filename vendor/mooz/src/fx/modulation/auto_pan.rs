//! AutoPan (AUTOPAN.ZDL), from the recovered `Fx_SFX_AutoPan` callback
//! (`auto_pan.rs`) and its decoded editors.
//!
//! Callback, per sample: from the LFO phase, the pan position is
//! `width × (0.5 sat(clip × dir × −sine) − 0.5)` (the parabolic sine of the
//! callbacks, c8 direction, c10 clip gain 1..16, c9 width) and a trapezoid
//! amplitude envelope `1 − depth (0.5 − 0.5 sat(4 tri'))` runs a quarter cycle
//! ahead (c6 depth, c7 fixed). The left input is multiplied by
//! `(1 + pan) × env`, the right by `(1 − width − pan) × env` — a linear pan of
//! each channel's own signal — and Level (c2) scales both.
//!
//! Editors: Rate 0 stops the LFO and zeroes width and depth, 1..50 →
//! `context[22]` 0xbe37..0xedc5b square law (0.5..10.4 Hz), above 50 tempo
//! notes. Width: L50..0..R50; the side sets the direction c8 (ramped), the
//! value n (Width below 50, or 100 − Width above it) gives
//! `c9 = 1 − (1 − 2^-8)(n/50)²` (0 at the centre), ramped over 706 samples. Depth 0..10 → c6 = 2^-8..1 square law.
//! Clip 0..10 → c10 = 1/16..1 square law (gain 1..16, integer ramp). Level
//! 0..1.5. All editors decoded.
use super::common::{
    Lfo, PAN_LABELS, Ramp, centered_triangle, curve, guard, increment_hz, parabola, rate_hz,
    stepped,
};
use super::tremolo::{DEPTH_STEP, DURATION, rate_increment};
use crate::zoom::DEFAULT_STEP;
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_auto_pan",
    name: "AutoPan",
    category: Category::Modulation,
    zdl: "AUTOPAN.ZDL",
    description: "Auto panner with width, swell depth and clipping",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Rate",
            max: 78,
            default: 5,
            display: Display::sync_rate(50, 0.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Width",
            max: 100,
            default: 100,
            display: Display::labels(&PAN_LABELS),
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
            name: "Depth",
            max: 10,
            default: 7,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Clip",
            max: 10,
            default: 0,
            display: Display::INT,
        },
    ],
};

/// c8 ramp step 0x00949b92 per sample, in units of the ±1 direction.
const DIRECTION_STEP: f32 = 9_739_154.0 / 1_073_741_824.0;
const MINIMUM: f32 = 1.0 / 256.0;

struct AutoPan {
    sample_rate: f32,
    lfo: Lfo,
    direction: Ramp,
    width: Ramp,
    depth: Ramp,
    clip: Ramp,
    level: Ramp,
    first: bool,
    rate: f32,
    bpm: f32,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(AutoPan {
        sample_rate,
        lfo: Lfo {
            phase: 0xc000_0000,
            increment: 0,
        },
        direction: Ramp::with_step(1.0, DIRECTION_STEP, sample_rate),
        width: Ramp::with_duration(1.0, DURATION, sample_rate),
        depth: Ramp::with_duration(0.0, 705.0, sample_rate),
        clip: Ramp::with_step(1.0, DEPTH_STEP * 16.0, sample_rate),
        level: Ramp::with_step(1.0, DEFAULT_STEP, sample_rate),
        first: true,
        rate: f32::NAN,
        bpm: 0.0,
    })
}

impl Effect for AutoPan {
    fn set(&mut self, values: &Values, bpm: f32) {
        let [rate, width, level, depth, clip] = [0, 1, 2, 3, 4].map(|i| values[i]);
        let stopped = rate < 0.5;
        if rate != self.rate || bpm != self.bpm {
            let hz = if stopped {
                0.0
            } else {
                rate_hz(rate, 50, bpm, |v| {
                    increment_hz(stepped(v.max(1.0), |n| rate_increment(n, 50)))
                })
            };
            self.lfo.set_hz(hz, self.sample_rate);
            self.rate = rate;
            self.bpm = bpm;
        }
        let width = width.clamp(0.0, 100.0);
        if width < 50.0 {
            self.direction.set(-1.0);
        } else if width > 50.0 {
            self.direction.set(1.0);
        }
        let distance = if width > 50.0 { 100.0 - width } else { width };
        let magnitude = if stopped || (width - 50.0).abs() < 0.5 {
            0.0
        } else {
            stepped(distance.min(49.0), |n| curve(MINIMUM, 1.0, 51, n, 1))
        };
        self.width.set(magnitude);
        let swell = if stopped {
            0.0
        } else {
            stepped(depth.clamp(0.0, 10.0), |n| curve(1.0, MINIMUM, 11, n, 1))
        };
        self.depth.set(swell);
        self.clip
            .set(16.0 * stepped(clip.clamp(0.0, 10.0), |n| curve(1.0, 0.0625, 11, n, 1)));
        self.level.set(level.clamp(0.0, 150.0) * 0.01);
        if self.first {
            self.first = false;
            for ramp in [
                &mut self.direction,
                &mut self.width,
                &mut self.depth,
                &mut self.clip,
                &mut self.level,
            ] {
                ramp.jump(ramp.target);
            }
        }
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let s = self.lfo.tick();
        let curved = -self.direction.tick() * parabola(centered_triangle(s));
        let width = self.width.tick();
        let pan = width * (0.5 * (curved * self.clip.tick()).clamp(-1.0, 1.0) - 0.5);
        let quarter = super::common::signed(self.lfo.phase.wrapping_add(0x4000_0000));
        let trapezoid = (4.0 * centered_triangle(quarter)).clamp(-1.0, 1.0);
        let envelope = 1.0 - self.depth.tick() * (0.5 - 0.5 * trapezoid);
        let level = self.level.tick();
        let left_gain = (1.0 + pan) * envelope * level;
        let right_gain = (1.0 - width - pan) * envelope * level;
        [guard(left * left_gain), guard(right * right_gain)]
    }

    fn clear(&mut self) {
        self.lfo.phase = 0xc000_0000;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(values: [f32; 5], n: usize) -> Vec<[f32; 2]> {
        let mut effect = create(44_100.0);
        let mut v = [0.0; crate::MAX_PARAMS];
        v[..5].copy_from_slice(&values);
        effect.set(&v, 120.0);
        (0..n).map(|_| effect.process([1.0, 1.0])).collect()
    }

    #[test]
    fn full_width_pans_hard_left_and_right() {
        // Depth 0 (swell 2^-8), Clip 0, Width R50.
        let out = run([30.0, 100.0, 100.0, 0.0, 0.0], 88_200);
        let left_max = out.iter().map(|f| f[0]).fold(f32::MIN, f32::max);
        let left_min = out.iter().map(|f| f[0]).fold(f32::MAX, f32::min);
        assert!(left_max > 0.98 && left_min < 0.02, "{left_min} {left_max}");
        for frame in &out {
            assert!((frame[0] + frame[1] - 1.0).abs() < 0.01);
        }
    }

    #[test]
    fn centre_width_does_not_pan() {
        let out = run([30.0, 50.0, 100.0, 0.0, 0.0], 44_100);
        for frame in &out[10_000..] {
            assert!((frame[0] - 1.0).abs() < 0.01 && (frame[1] - 1.0).abs() < 0.01);
        }
    }

    #[test]
    fn sides_swap_the_direction() {
        let right = run([30.0, 100.0, 100.0, 0.0, 0.0], 8_000);
        let left = run([30.0, 0.0, 100.0, 0.0, 0.0], 8_000);
        assert!((right[7_000][0] - left[7_000][1]).abs() < 0.01);
    }
}
