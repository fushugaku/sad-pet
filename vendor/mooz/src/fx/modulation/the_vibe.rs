//! TheVibe (THEVIBE.ZDL), a Uni-Vibe model, from the recovered
//! `Fx_MOD_TheVibe` callback (`the_vibe.rs`) and its decoded editors.
//!
//! Callback: a block-rate compressor (threshold −60 dB, slope 1/1.8 in linear
//! terms, so practically a fixed −5 dB above the threshold, +6 dB
//! makeup from `Fx_MOD_TheVibe_Comp_init`; peak detector on a 10 Hz high-passed
//! mean with a per-block decay c15, gain smoothed by 0.017 per sample) acts on
//! the stereo mean delayed 9 samples (c16). The LFO increment glides towards
//! the Speed value through a 1/128 one-pole (c19..21, the lamp's inertia); the
//! wave is `0.6 × triangle + 0.4 × parabolic sine`, scaled by Wave separately
//! for each half (c23/c24), clipped, then `Bias + 0.5 + 0.5 × Depth × wave`
//! clamped to 0..1 (c22, c25). Four all-passes take `offset + gain × wave`
//! (c27..34) through per-stage photocell lags (c35..38). Chorus mode adds the
//! dry drive to the all-pass output (c49), Vibrato uses the all-passes alone;
//! the result × 3 (c48) passes a wave-dependent one-pole low-pass (c39/c40),
//! a first-order low shelf (c43..47) and Level (c2) to both outputs.
//!
//! Editors: Speed → `context[22]` 0xbe37..0xedc5b square law over 51 steps
//! (0.5..10 Hz). Depth 0..1 linear. Bias −0.5..+0.5 and Wave (positive half
//! 1..2, negative half 1..2.5) through Main's power-1.41 curve; all ramped
//! over 705.6 samples. Mode ramps c49 to 0 (VIBRT) or 1 (CHORS). Level 0..1.5.
//! All editors decoded. The compressor's 8-sample blocks become blocks of
//! `8 × rate / 44.1 kHz` samples; filters, lags and all-pass weights are
//! re-derived for the host rate.
use super::common::{
    AllpassWarp, FirstOrder, Ramp, allpass, centered_triangle, curve, guard, increment_hz,
    one_pole_at, parabola, pole_at, rom_first_order, stepped,
};
use crate::dsp::{DelayLine, db_to_gain, scale_samples};
use crate::zoom::{DEFAULT_STEP, ORIGINAL_RATE};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_the_vibe",
    name: "TheVibe",
    category: Category::Modulation,
    zdl: "THEVIBE.ZDL",
    description: "Uni-Vibe style photocell phaser, chorus or vibrato",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Speed",
            max: 50,
            default: 25,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 60,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Bias",
            max: 100,
            default: 48,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Wave",
            max: 100,
            default: 24,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Mode",
            max: 1,
            default: 1,
            display: Display::labels(&["VIBRT", "CHORS"]),
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

/// c27..30, c31..34, c35..38: per-stage wave gain, offset and lag pole.
const STAGE_GAIN: [f32; 4] = [-1.473_461_3, -1.473_461_3, -0.059_952_66, -0.059_952_66];
const STAGE_OFFSET: [f32; 4] = [0.952_695_7, 0.952_695_7, 0.997_429_1, 0.997_429_1];
const STAGE_LAG: [f32; 4] = [0.968_276_26, 0.997_804_16, 0.999_286_2, 0.902_302_74];
/// c39/c40: tone pole `c40 + c39 × wave`.
const TONE_SLOPE: f32 = -0.181_672_9;
const TONE_REST: f32 = 0.355_273_96;

struct TheVibe {
    denormals: super::common::Denormals,
    sample_rate: f32,
    warp: AllpassWarp,
    // Compressor.
    ring: DelayLine,
    tap: f32,
    detector: FirstOrder,
    block: usize,
    counter: usize,
    detector_peak: f32,
    held_peak: f32,
    target_gain: f32,
    gain: f32,
    gain_coefficient: f32,
    makeup: f32,
    // Modulation.
    phase: u32,
    increment: f32,
    increment_target: f32,
    increment_coefficient: f32,
    depth: Ramp,
    bias: Ramp,
    positive: Ramp,
    negative: Ramp,
    chorus: Ramp,
    lags: [f32; 4],
    lag_poles: [f32; 4],
    stages: [f32; 4],
    tone: f32,
    tone_poles: [f32; 2],
    shelf: FirstOrder,
    level: Ramp,
    first: bool,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let ratio = sample_rate / ORIGINAL_RATE;
    Box::new(TheVibe {
        denormals: Default::default(),
        sample_rate,
        warp: AllpassWarp::new(sample_rate),
        ring: DelayLine::new(scale_samples(9.0, sample_rate).ceil() as usize + 4),
        tap: scale_samples(9.0, sample_rate),
        detector: FirstOrder::new(rom_first_order(
            [0.999_288_14, -0.999_288_14, 0.998_576_3],
            sample_rate,
        )),
        block: ((8.0 * ratio).round() as usize).max(1),
        counter: 0,
        detector_peak: 0.0,
        held_peak: 0.0,
        target_gain: 1.0,
        gain: 0.0,
        gain_coefficient: one_pole_at(0.017, sample_rate),
        makeup: db_to_gain(6.0),
        phase: 0,
        increment: 0.0,
        increment_target: 0.0,
        increment_coefficient: one_pole_at(1.0 / 128.0, sample_rate),
        depth: Ramp::with_duration(1.0, 705.6, sample_rate),
        bias: Ramp::with_duration(-0.2, 705.6, sample_rate),
        positive: Ramp::with_duration(1.0, 705.6, sample_rate),
        negative: Ramp::with_duration(1.0, 705.6, sample_rate),
        chorus: Ramp::with_duration(1.0, 705.6, sample_rate),
        lags: [0.0; 4],
        lag_poles: STAGE_LAG.map(|pole| pole_at(pole, sample_rate)),
        stages: [0.0; 4],
        tone: 0.0,
        tone_poles: [
            pole_at(TONE_REST, sample_rate),
            pole_at(TONE_REST + TONE_SLOPE, sample_rate),
        ],
        shelf: FirstOrder::new(rom_first_order(
            [1.005_343_8, -0.966_559_47, 0.971_903_26],
            sample_rate,
        )),
        level: Ramp::with_step(1.0, DEFAULT_STEP, sample_rate),
        first: true,
    })
}

/// Main's power-1.41 curve between `minimum` and `maximum` for 0..100.
fn power_curve(maximum: f32, minimum: f32, value: f32) -> f32 {
    stepped(value.clamp(0.0, 100.0), |n| {
        curve(maximum, minimum, 101, n, 5)
    })
}

impl TheVibe {
    /// The block-rate compressor update at the start of each 8-sample block.
    fn compressor_block(&mut self) {
        const THRESHOLD: f32 = 0.001;
        const SLOPE: f32 = 1.0 / 1.8;
        self.held_peak = if self.held_peak < self.detector_peak {
            self.detector_peak
        } else {
            self.held_peak * 0.988_923_1
        };
        self.target_gain = if THRESHOLD < self.held_peak {
            (THRESHOLD + SLOPE * (self.held_peak - THRESHOLD)) / self.held_peak
        } else {
            1.0
        };
        self.detector_peak = 0.0;
    }
}

impl Effect for TheVibe {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let [speed, depth, bias, wave, mode, level] = [0, 1, 2, 3, 4, 5].map(|i| values[i]);
        let increment = stepped(speed.clamp(0.0, 50.0), |n| {
            curve(973_915.0, 48_695.0, 51, n, 1)
        });
        let hz = increment_hz(increment);
        self.increment_target = hz / self.sample_rate * 4_294_967_296.0;
        self.depth.set(depth.clamp(0.0, 100.0) * 0.01);
        self.bias.set(power_curve(0.5, -0.5, bias));
        self.positive.set(power_curve(2.0, 1.0, wave));
        self.negative.set(power_curve(2.5, 1.0, wave));
        self.chorus.set(if mode.round() >= 1.0 { 1.0 } else { 0.0 });
        self.level.set(level.clamp(0.0, 150.0) * 0.01);
        if self.first {
            self.first = false;
            self.increment = self.increment_target;
            for ramp in [
                &mut self.depth,
                &mut self.bias,
                &mut self.positive,
                &mut self.negative,
                &mut self.chorus,
                &mut self.level,
            ] {
                ramp.jump(ramp.target);
            }
        }
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        if self.counter == 0 {
            self.compressor_block();
        }
        self.counter += 1;
        if self.counter >= self.block {
            self.counter = 0;
        }
        let mean = 0.5 * (left + right);
        self.ring.write(mean);
        let filtered = self.detector.process(mean);
        self.detector_peak = self.detector_peak.max(filtered.abs());
        self.gain += self.gain_coefficient * (self.target_gain - self.gain);
        let driven = 0.25 * self.makeup * self.gain * self.ring.read_linear(self.tap);

        self.increment += self.increment_coefficient * (self.increment_target - self.increment);
        self.phase = self.phase.wrapping_add(self.increment as u32);
        let triangle = centered_triangle(super::common::signed(self.phase));
        let mut wave = 0.6 * (2.0 * triangle) + 0.4 * parabola(triangle);
        wave *= if wave > 0.0 {
            self.positive.tick()
        } else {
            self.negative.tick()
        };
        let wave = wave.clamp(-1.0, 1.0);
        let wave = (self.bias.tick() + 0.5 + 0.5 * wave * self.depth.tick()).clamp(0.0, 1.0);

        let mut value = driven;
        for stage in 0..4 {
            let target = STAGE_OFFSET[stage] + wave * STAGE_GAIN[stage];
            let pole = self.lag_poles[stage];
            self.lags[stage] = pole * self.lags[stage] + (1.0 - pole) * target;
            let weight = self.warp.map(self.lags[stage]);
            value = allpass(&mut self.stages[stage], value, weight);
        }
        self.denormals.sweep(&mut self.stages);
        let signal = 3.0 * (driven * self.chorus.tick() + value);
        let blend = self.tone_poles[0] + (self.tone_poles[1] - self.tone_poles[0]) * wave;
        self.tone = self.tone * blend + signal * (1.0 - blend);
        let out = guard(self.shelf.process(self.tone) * self.level.tick());
        [out, out]
    }

    fn clear(&mut self) {
        self.ring.clear();
        self.detector.clear();
        self.shelf.clear();
        self.detector_peak = 0.0;
        self.held_peak = 0.0;
        self.gain = 0.0;
        self.stages = [0.0; 4];
        self.lags = [0.0; 4];
        self.tone = 0.0;
        self.phase = 0;
        self.counter = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::modulation::common::testing::rms;

    fn render(values: [f32; 6], rate: f32, amplitude: f32) -> Vec<f32> {
        let mut effect = create(rate);
        let mut v = [0.0; crate::MAX_PARAMS];
        v[..6].copy_from_slice(&values);
        effect.set(&v, 120.0);
        (0..rate as usize * 2)
            .map(|i| {
                let x = amplitude * (i as f32 * std::f32::consts::TAU * 330.0 / rate).sin();
                effect.process([x, x])[0]
            })
            .collect()
    }

    #[test]
    fn defaults_sound_near_unity() {
        let out = render([25.0, 60.0, 48.0, 24.0, 1.0, 100.0], 48_000.0, 0.3);
        let level = rms(&out[48_000..]) / (0.3 / std::f32::consts::SQRT_2);
        assert!(level > 0.4 && level < 2.5, "{level}");
    }

    #[test]
    fn compressor_scales_above_minus_60_db() {
        // The original's gain (T + (P - T) / 1.8) / P is ~0.56 well above the
        // -60 dB threshold and 1 below it: 40 dB more input gives ~35 dB more.
        let quiet = rms(&render([25.0, 60.0, 48.0, 24.0, 0.0, 100.0], 44_100.0, 0.000_5)[44_100..]);
        let loud = rms(&render([25.0, 60.0, 48.0, 24.0, 0.0, 100.0], 44_100.0, 0.05)[44_100..]);
        let db = 20.0 * (loud / quiet).log10();
        assert!(db > 34.0 && db < 36.5, "{db}");
    }

    #[test]
    fn bias_and_wave_follow_the_power_curve() {
        assert!((power_curve(0.5, -0.5, 0.0) + 0.5).abs() < 1e-6);
        assert!((power_curve(0.5, -0.5, 100.0) - 0.5).abs() < 1e-6);
        let mid = power_curve(2.0, 1.0, 50.0);
        assert!((mid - (1.0 + 0.5f32.powf(1.41))).abs() < 1e-4);
    }
}
