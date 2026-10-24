//! Phaser (PHASER.ZDL), from the recovered `Fx_MOD_Phaser` callback
//! (`phaser.rs`) and its decoded editors.
//!
//! Callback, per sample: the LFO weight is `c9 + c8 (1 − 0.06 m − 0.94 m²)` of
//! the triangle `m = |phase|` (c6/c7 fixed). The stereo mean times c10 plus
//! the two feedback taps (c11 × a one-pole low-pass of the 4th stage, c12 ×
//! the 8th stage) feeds eight first-order all-passes sharing that weight; the
//! output `c13 × input + c14 × stage 4 + c15 × stage 8`, times Level, goes to
//! both channels (no dry path besides c13).
//!
//! Editors: Rate 0 → increment 0x22f2 (0.09 Hz), 1..49 → `context[22]`
//! 0x45e6..0xb9af6 square law (0.18..7.8 Hz), above 49 tempo notes. Color
//! copies one row of `Fx_Mod_Pha_color_LFO_tbl` (c8/c9), `_FB_tbl` (c10..12)
//! and `_Mix_tbl` (c13..15): 4 or 8 stages, normal or inverted. The original
//! ramps the gains to zero over 705.6 samples, swaps the rows and ramps the
//! mix back in; that transition is kept. Level 0..1.5. All editors decoded.
//! The 44.1 kHz all-pass weights are re-warped to the host rate.
use super::common::{
    AllpassWarp, Lfo, Ramp, allpass, curve, guard, increment_hz, pole_at, rate_hz, stepped,
};
use crate::zoom::DEFAULT_STEP;
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_phaser",
    name: "Phaser",
    category: Category::Modulation,
    zdl: "PHASER.ZDL",
    description: "Four or eight stage phaser, normal or inverted",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Rate",
            max: 77,
            default: 11,
            display: Display::sync_rate(49, 1.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Color",
            max: 3,
            default: 3,
            display: Display::labels(&["4 STG", "8 STG", "inv 4", "inv 8"]),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

/// `Fx_Mod_Pha_color_LFO_tbl`: c8 (scale), c9 (offset) as Q31 fractions.
const LFO: [[f32; 2]; 4] = [
    [0.749_063, 0.172_327],
    [0.585_967, 0.302_384],
    [0.196_709, 0.691_643],
    [0.661_626, 0.215_760],
];
/// `Fx_Mod_Pha_color_FB_tbl`: c10 (input), c11 (stage-4 feedback), c12 (stage 8).
const FEEDBACK: [[f32; 3]; 4] = [
    [0.766_686, 0.319_987, 0.0],
    [0.833_338, 0.0, 0.489_975],
    [0.700_003, -0.640_034, 0.0],
    [0.866_665, 0.0, -0.537_797],
];
/// `Fx_Mod_Pha_color_Mix_tbl`: c13 (input), c14 (stage 4), c15 (stage 8).
const MIX: [[f32; 3]; 4] = [
    [0.900_021, 0.900_021, 0.0],
    [0.900_021, 0.0, 0.900_021],
    [0.900_021, -0.900_052, 0.0],
    [0.866_665, 0.0, -0.866_695],
];
/// c16/c17: one-pole low-pass of the 4th stage in the feedback (44.1 kHz).
const TONE_POLE: f32 = 0.208_893;
const TONE_GAIN: f32 = 0.791_046;

pub fn rate_increment(value: u32) -> f32 {
    if value == 0 {
        8_946.0
    } else {
        curve(760_566.0, 17_894.0, 50, value, 1)
    }
}

struct Phaser {
    denormals: super::common::Denormals,
    sample_rate: f32,
    warp: AllpassWarp,
    lfo: Lfo,
    color: usize,
    target_color: usize,
    fade: Ramp,
    fading_in: bool,
    stages: [f32; 8],
    tone: f32,
    tone_pole: f32,
    tone_gain: f32,
    last: f32,
    level: Ramp,
    rate: f32,
    bpm: f32,
    first: bool,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let tone_pole = pole_at(TONE_POLE, sample_rate);
    Box::new(Phaser {
        denormals: Default::default(),
        sample_rate,
        warp: AllpassWarp::new(sample_rate),
        lfo: Lfo::default(),
        color: 3,
        target_color: 3,
        fade: Ramp::with_duration(1.0, 705.6, sample_rate),
        fading_in: true,
        stages: [0.0; 8],
        tone: 0.0,
        tone_pole,
        tone_gain: TONE_GAIN / (1.0 - TONE_POLE) * (1.0 - tone_pole),
        last: 0.0,
        level: Ramp::with_step(1.0, DEFAULT_STEP, sample_rate),
        rate: f32::NAN,
        bpm: 0.0,
        first: true,
    })
}

impl Effect for Phaser {
    fn set(&mut self, values: &Values, bpm: f32) {
        let [rate, color, level] = [values[0], values[1], values[2]];
        if rate != self.rate || bpm != self.bpm {
            let hz = rate_hz(rate, 49, bpm, |v| increment_hz(stepped(v, rate_increment)));
            self.lfo.set_hz(hz, self.sample_rate);
            self.rate = rate;
            self.bpm = bpm;
        }
        self.target_color = color.round().clamp(0.0, 3.0) as usize;
        self.level.set(level.clamp(0.0, 150.0) * 0.01);
        if self.first {
            self.first = false;
            self.color = self.target_color;
            self.level.jump(self.level.target);
        } else if self.target_color != self.color && self.fading_in {
            self.fading_in = false;
            self.fade.set(0.0);
        }
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let fade = self.fade.tick();
        if !self.fading_in && fade == 0.0 {
            self.color = self.target_color;
            self.fading_in = true;
            self.fade.set(1.0);
        }
        let [scale, offset] = LFO[self.color];
        let [gain, tone_feedback, last_feedback] = FEEDBACK[self.color];
        let [dry, fourth_mix, last_mix] = MIX[self.color];
        let feedback_fade = if self.fading_in { 1.0 } else { fade };
        let m = self.lfo.tick().abs();
        let weight = self
            .warp
            .map(offset + scale * (1.0 - 0.059_998 * m - 0.939_972 * m * m));
        let mean = 0.5 * (left + right);
        let input =
            gain * mean + feedback_fade * (tone_feedback * self.tone + last_feedback * self.last);
        let mut value = input;
        let mut fourth = 0.0;
        for (stage, memory) in self.stages.iter_mut().enumerate() {
            value = allpass(memory, value, weight);
            if stage == 3 {
                fourth = value;
            }
        }
        self.last = value;
        self.denormals.sweep(&mut self.stages);
        self.tone = self.tone_pole * self.tone + self.tone_gain * fourth;
        let wet = fade * (dry * input + fourth_mix * fourth + last_mix * value);
        let out = guard(wet * self.level.tick());
        [out, out]
    }

    fn clear(&mut self) {
        self.stages = [0.0; 8];
        self.tone = 0.0;
        self.last = 0.0;
        self.lfo.phase = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate_curve_matches_the_editor() {
        assert!((increment_hz(rate_increment(0)) - 0.0919).abs() < 1e-3);
        assert!((increment_hz(rate_increment(1)) - 0.187).abs() < 2e-3);
        assert!((increment_hz(rate_increment(49)) - 7.81).abs() < 0.01);
    }

    #[test]
    fn four_stages_notch_and_inverted_peaks() {
        // Freeze the LFO at phase 0 (Rate 0 is 0.09 Hz) and compare the
        // response at a fixed frequency for "4 STG" and "inv 4".
        let measure = |color: f32, hz: f32| {
            let mut effect = create(44_100.0);
            effect.set(&[0.0, color, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 120.0);
            let mut energy = 0.0;
            for i in 0..4_410 {
                let x = (i as f32 * std::f32::consts::TAU * hz / 44_100.0).sin();
                let y = effect.process([x, x])[0];
                if i > 2_000 {
                    energy += y * y;
                }
            }
            energy
        };
        // Stage weight at phase 0 is c8 + c9 = 0.921: 90° near 580 Hz per stage,
        // so 4 stages give 360° (a notch for the summing color) near there.
        let normal: Vec<f32> = [200.0, 1_500.0, 6_000.0]
            .iter()
            .map(|&f| measure(0.0, f))
            .collect();
        let inverted: Vec<f32> = [200.0, 1_500.0, 6_000.0]
            .iter()
            .map(|&f| measure(2.0, f))
            .collect();
        assert!(
            normal
                .iter()
                .zip(&inverted)
                .any(|(a, b)| (a / b).abs() > 2.0 || (b / a).abs() > 2.0)
        );
    }

    #[test]
    fn color_change_ramps_without_jumps() {
        let mut effect = create(48_000.0);
        effect.set(&[20.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 120.0);
        let mut previous = 0.0f32;
        let mut biggest = 0.0f32;
        for i in 0..48_000 {
            if i == 10_000 {
                effect.set(&[20.0, 3.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 120.0);
            }
            let x = (i as f32 * 0.01).sin() * 0.5;
            let y = effect.process([x, x])[0];
            biggest = biggest.max((y - previous).abs());
            previous = y;
        }
        assert!(biggest < 0.1, "{biggest}");
    }
}

#[cfg(test)]
mod level_probe {
    #[test]
    fn defaults_level() {
        let mut effect = super::create(48_000.0);
        effect.set(&crate::Model::Phaser.defaults(), 120.0);
        let mut energy = 0.0;
        for i in 0..96_000 {
            let x = 0.3 * (i as f32 * 0.05).sin();
            let y = effect.process([x, x])[0];
            if i > 48_000 {
                energy += y * y;
            }
        }
        let rms = (energy / 48_000.0f32).sqrt();
        assert!(rms > 0.05 && rms < 0.6, "{rms}");
    }
}
