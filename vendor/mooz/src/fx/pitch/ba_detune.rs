//! Ba Detune (B_DETUNE.ZDL): bass detune, mono out.
//!
//! Callback: recovered `bass_detune.rs`, the DETUNE graph ([`super::two_tap`],
//! same window, sweep and ROM geometry) plus a first-order LoCut on the voice
//! (c26..c28) and a mono dry: `out = c2 (mean + c3² c15 LoCut(voice))` on both
//! channels. Editors decoded:
//! - `cent_edit`: c5 from 0x80000a30, -50..+50 cents in steps of 2
//!   (`GetString_0_50_to_minus50_50`), changed in place.
//! - `predly_edit`: c9 = 441·PreD/10 + 2 with the 16 ms c3 mute/fade-in.
//! - `mix_edit`: c15 = two-slope(Mix), 16 ms `context[7]` ramp.
//! - `locut_edit`: c26..c28 from `BA_DETUNE_LOCUT` (OFF ≈ 20 Hz, 60..800 Hz),
//!   `y = (1+a)/2 (x - x1) + a y1`, re-derived per rate as `a = e^(-2π fc/fs)`.
//! - `tone_edit`: c19/c20 input one-pole; `outLv_edit`: c2 = Level / 100.
use super::{
    common::{FADE_SECONDS, TimedRamp, decay, one_pole, out_level, step, two_slope},
    detune::{DETUNE_LONGEST, detune_geometry},
    rom::{BA_DETUNE_LOCUT, TWO_TAP_TONE, f},
    two_tap::TwoTapShifter,
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::flush,
    zoom::{DEFAULT_STEP, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_ba_detune",
    name: "Ba Detune",
    category: Category::Pitch,
    zdl: "B_DETUNE.ZDL",
    description: "Bass detune (±50 cents) with pre-delay and low cut",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Cent",
            max: 50,
            default: 35,
            display: Display::number(-50.0, 2.0, 0, ""),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "PreD",
            max: 50,
            default: 0,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 60,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 8,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "LoCut",
            max: 10,
            default: 1,
            display: Display::labels(&[
                "OFF", "60", "120", "180", "200", "280", "340", "400", "500", "630", "800",
            ]),
        },
    ],
};

struct BaDetune {
    sample_rate: f32,
    shifter: TwoTapShifter,
    pre_delay: Option<usize>,
    wet: TimedRamp,
    level: Ramp,
    locut: (f32, f32),
    locut_state: (f32, f32),
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(BaDetune {
        sample_rate,
        shifter: TwoTapShifter::new(sample_rate, DETUNE_LONGEST),
        pre_delay: None,
        wet: TimedRamp::new(two_slope(60.0), FADE_SECONDS, sample_rate),
        level: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
        locut: (1.0, 0.0),
        locut_state: (0.0, 0.0),
    })
}

impl Effect for BaDetune {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let pre_delay = step(values[1], 50);
        let cents = 2.0 * (step(values[0], 50) as f32 - 25.0);
        let geometry = detune_geometry(cents, pre_delay, self.sample_rate);
        let fade = self.pre_delay.is_some_and(|old| old != pre_delay);
        self.shifter.request(geometry, fade);
        self.pre_delay = Some(pre_delay);
        let tone = f(TWO_TAP_TONE[step(values[3], 10)]);
        self.shifter.set_tone(one_pole(tone, self.sample_rate));
        self.wet.set(two_slope(values[2]));
        self.level.set(out_level(values[4]));
        let feedback = decay(
            f(BA_DETUNE_LOCUT[3 * step(values[5], 10) + 2]),
            self.sample_rate,
        );
        self.locut = ((1.0 + feedback) * 0.5, feedback);
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mean = 0.5 * (left + right);
        let voice = self.shifter.process(mean);
        let (gain, feedback) = self.locut;
        let (previous, output) = self.locut_state;
        let cut = flush(gain * (voice - previous) + feedback * output);
        self.locut_state = (voice, cut);
        let out = self.level.tick() * (mean + self.wet.tick() * cut);
        [out, out]
    }

    fn clear(&mut self) {
        self.shifter.clear();
        self.locut_state = (0.0, 0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::pitch::two_tap::tests::centroid;

    #[test]
    fn locut_matches_the_rom_corners() {
        for (index, hz) in [(1usize, 60.0f32), (10, 800.0)] {
            let rom = f(BA_DETUNE_LOCUT[3 * index + 2]);
            let corner = -rom.ln() * 44_100.0 / std::f32::consts::TAU;
            assert!((corner - hz).abs() < hz * 0.02, "{index}: {corner}");
            assert!((f(BA_DETUNE_LOCUT[3 * index]) - (1.0 + rom) * 0.5).abs() < 1e-6);
        }
    }

    #[test]
    fn detunes_by_twice_the_step_in_cents() {
        let sample_rate = 44_100.0;
        let mut effect = create(sample_rate);
        let mut values = crate::Model::BaDetune.defaults();
        values[0] = 0.0; // -50 cents
        values[3] = 10.0;
        effect.set(&values, 120.0);
        let n = 3 * sample_rate as usize;
        let out: Vec<f32> = (0..n)
            .map(|i| {
                let x = 0.5 * (i as f32 * 244.14 / sample_rate * std::f32::consts::TAU).sin();
                effect.process([x, x])[1] - x
            })
            .collect();
        let found = centroid(&out[n / 3..], sample_rate, 227.19, 247.19);
        let expected = 244.14 * (-50.0f32 / 1200.0).exp2();
        assert!((found - expected).abs() < 0.5, "{found} vs {expected}");
    }
}
