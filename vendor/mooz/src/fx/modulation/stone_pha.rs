//! StonePha (STONEPHA.ZDL), a Small Stone style phaser, from the recovered
//! `Fx_MOD_StonePha` callback (`stone_phase.rs`) and its decoded editors.
//!
//! Callback, per sample: the LFO weight is `c9 + c8 (1 − 0.1 m − 0.9 m²)` of
//! the triangle `m = |phase|`. The stereo mean (c13) plus the fed-back tone
//! output (one sample late, through the feedback EQ c10..12, times c14) passes
//! the pre-EQ c15..17 and four first-order all-passes; the tone EQ c20..24
//! shapes the all-pass output. The mix `c18 × pre-EQ + c19 × tone +
//! (c50 + c51) × mean` runs through a five-biquad output EQ (c25..49) and the
//! colour level c52 to both outputs; there is no separate dry path.
//!
//! Editors: RATE interpolates `StonePha_Sawf_Tbl` per colour between decades
//! (`tbl[rate/10] + (tbl[rate/10+1] − tbl[rate/10]) × (rate%10)/10`): 0.10..13.5 Hz
//! (COLOR 1) or 0.07..9.3 Hz (COLOR 2). COLOR fades the wet gain c3 out
//! (705.6 samples), copies one row of every `StonePha_color_*` table (LFO
//! range, feedback EQ and amount, pre-EQ, mix, tone EQ, input blend, output
//! EQ, level), recomputes the rate and fades back in. All editors decoded; the
//! ROM filters are re-derived for the host rate from their bilinear prototypes
//! and the all-pass weights re-warped.
use super::common::{
    AllpassWarp, FirstOrder, Lfo, Ramp, allpass, guard, increment_hz, rom_biquad, rom_first_order,
};
use crate::dsp::Biquad;
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_stone_pha",
    name: "StonePha",
    category: Category::Modulation,
    zdl: "STONEPHA.ZDL",
    description: "Small Stone style four-stage phaser with two colors",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "COLOR",
            max: 1,
            default: 0,
            display: Display::offset(1.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "RATE",
            max: 100,
            default: 50,
            display: Display::INT,
        },
    ],
};

/// `StonePha_Sawf_Tbl`: increments per decade of RATE, [colour 1, colour 2].
const SAWF: [[f32; 2]; 12] = [
    [9_933.0, 6_810.0],
    [10_586.0, 7_268.0],
    [13_527.0, 9_275.0],
    [19_098.0, 13_157.0],
    [32_032.0, 22_137.0],
    [61_639.0, 42_715.0],
    [86_960.0, 60_119.0],
    [149_788.0, 102_553.0],
    [405_830.0, 286_428.0],
    [737_838.0, 518_025.0],
    [1_315_759.0, 901_748.0],
    [1_315_759.0, 901_748.0],
];

/// Everything one COLOR row sets.
struct Color {
    /// LFO scale c8 and offset c9.
    lfo: [f32; 2],
    /// Feedback EQ c10..12 (first order on the delayed tone output).
    feedback_eq: [f32; 3],
    /// c13 input gain, c14 feedback gain.
    gains: [f32; 2],
    pre_eq: [f32; 3],
    /// c18 (pre-EQ signal) and c19 (tone output).
    mix: [f32; 2],
    /// Tone EQ c20..24 (first order in both colours).
    tone_eq: [f32; 3],
    /// c50 + c51: dry mean added before the output EQ.
    dry: f32,
    output_eq: [[f32; 5]; 5],
    level: f32,
}

const COLORS: [Color; 2] = [
    Color {
        lfo: [0.270_088_2, 0.694_912_5],
        feedback_eq: [1.0, 0.0, 0.0],
        gains: [1.0, 0.24],
        pre_eq: [1.313_838_7, -0.386_987_7, 0.073_148_93],
        mix: [0.5, 0.5],
        tone_eq: [0.994_237_5, -0.966_300_85, 0.960_538_3],
        dry: 0.12,
        output_eq: [
            [1.004_801_5, -0.981_051_45, 0.0, 0.985_852_96, 0.0],
            [0.666_944_8, -0.145_681_35, 0.0, 0.478_736_55, 0.0],
            [
                0.924_661_5,
                -1.466_721_4,
                0.667_046_96,
                1.466_721_4,
                -0.591_708_4,
            ],
            [1.0, 0.0, 0.0, 0.0, 0.0],
            [
                0.955_368_3,
                -1.406_032_8,
                0.610_622_47,
                1.406_032_8,
                -0.565_990_75,
            ],
        ],
        level: 0.9156,
    },
    Color {
        lfo: [0.420_449_47, 0.567_231_3],
        feedback_eq: [0.985_052, -0.985_052, 0.970_104],
        gains: [1.0, 0.87],
        pre_eq: [0.746_582_57, -0.481_996_33, 0.735_413_8],
        mix: [0.6, 0.71],
        tone_eq: [0.979_072_7, -0.979_072_7, 0.958_145_4],
        dry: 0.0,
        output_eq: [
            [1.015_380_7, -0.963_472_96, 0.0, 0.978_853_76, 0.0],
            [
                1.101_067,
                -0.630_842_27,
                0.408_956_02,
                0.630_842_27,
                -0.510_023,
            ],
            [
                0.928_523_1,
                -1.614_214_5,
                0.784_888_9,
                1.614_214_5,
                -0.713_412_1,
            ],
            [
                1.004_007_3,
                -1.983_720_1,
                0.979_841_5,
                1.983_720_1,
                -0.983_848_87,
            ],
            [0.504_019_14, -0.406_625_54, 0.0, 0.902_606_4, 0.0],
        ],
        level: 0.8,
    },
];

/// Rate as an original increment for a COLOR index and RATE value.
fn rate_increment(color: usize, rate: f32) -> f32 {
    let rate = rate.clamp(0.0, 100.0);
    let decade = ((rate / 10.0) as usize).min(10);
    let fraction = (rate - 10.0 * decade as f32) / 10.0;
    let low = SAWF[decade][color];
    low + (SAWF[decade + 1][color] - low) * fraction
}

struct StonePha {
    denormals: super::common::Denormals,
    sample_rate: f32,
    warp: AllpassWarp,
    lfo: Lfo,
    color: usize,
    target_color: usize,
    rate: f32,
    fade: Ramp,
    stages: [f32; 4],
    feedback_eq: FirstOrder,
    pre_eq: FirstOrder,
    tone_eq: FirstOrder,
    output_eq: [Biquad; 5],
    last_tone: f32,
    first: bool,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = StonePha {
        denormals: Default::default(),
        sample_rate,
        warp: AllpassWarp::new(sample_rate),
        lfo: Lfo::default(),
        color: 0,
        target_color: 0,
        rate: 50.0,
        fade: Ramp::with_duration(1.0, 705.6, sample_rate),
        stages: [0.0; 4],
        feedback_eq: FirstOrder::default(),
        pre_eq: FirstOrder::default(),
        tone_eq: FirstOrder::default(),
        output_eq: [Biquad::default(); 5],
        last_tone: 0.0,
        first: true,
    };
    effect.load_color();
    Box::new(effect)
}

impl StonePha {
    fn load_color(&mut self) {
        let color = &COLORS[self.color];
        let rate = self.sample_rate;
        self.feedback_eq
            .set(rom_first_order(color.feedback_eq, rate));
        self.pre_eq.set(rom_first_order(color.pre_eq, rate));
        self.tone_eq.set(rom_first_order(color.tone_eq, rate));
        for (biquad, rom) in self.output_eq.iter_mut().zip(color.output_eq) {
            biquad.coefficients = rom_biquad(rom, rate);
        }
        self.apply_rate();
    }

    fn apply_rate(&mut self) {
        let hz = increment_hz(rate_increment(self.color, self.rate));
        self.lfo.set_hz(hz, self.sample_rate);
    }
}

impl Effect for StonePha {
    fn set(&mut self, values: &Values, _bpm: f32) {
        self.target_color = values[0].round().clamp(0.0, 1.0) as usize;
        if values[1] != self.rate {
            self.rate = values[1];
            self.apply_rate();
        }
        if self.first {
            self.first = false;
            if self.target_color != self.color {
                self.color = self.target_color;
                self.load_color();
            }
        } else if self.target_color != self.color && self.fade.target != 0.0 {
            self.fade.set(0.0);
        }
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let fade = self.fade.tick();
        if fade == 0.0 && self.fade.target == 0.0 {
            self.color = self.target_color;
            self.load_color();
            self.fade.set(1.0);
        }
        let color = &COLORS[self.color];
        let m = self.lfo.tick().abs();
        let weight = self
            .warp
            .map(color.lfo[1] + color.lfo[0] * (1.0 - 0.1 * m - 0.9 * m * m));
        let mean = 0.5 * (left + right);
        let feedback = self.feedback_eq.process(self.last_tone);
        let incoming = color.gains[1] * feedback + color.gains[0] * mean;
        let first = self.pre_eq.process(incoming);
        let mut value = first;
        for memory in &mut self.stages {
            value = allpass(memory, value, weight);
        }
        self.denormals.sweep(&mut self.stages);
        let tone = self.tone_eq.process(value);
        self.last_tone = tone;
        let mut mixed = color.dry * mean + color.mix[1] * tone + color.mix[0] * first;
        for biquad in &mut self.output_eq {
            mixed = biquad.process(mixed);
        }
        let out = guard(mixed * color.level * fade);
        [out, out]
    }

    fn clear(&mut self) {
        self.stages = [0.0; 4];
        self.feedback_eq.clear();
        self.pre_eq.clear();
        self.tone_eq.clear();
        for biquad in &mut self.output_eq {
            biquad.clear();
        }
        self.last_tone = 0.0;
        self.lfo.phase = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::modulation::common::testing::rms;

    #[test]
    fn rate_table_interpolates_by_decade() {
        assert_eq!(rate_increment(0, 0.0), 9_933.0);
        assert_eq!(rate_increment(1, 100.0), 901_748.0);
        let mid = rate_increment(0, 55.0);
        assert!((mid - (61_639.0 + 86_960.0) / 2.0).abs() < 1.0);
        assert!((increment_hz(rate_increment(0, 100.0)) - 13.51).abs() < 0.01);
    }

    #[test]
    fn both_colors_pass_signal_near_unity() {
        for color in [0.0, 1.0] {
            for rate in [44_100.0, 96_000.0] {
                let mut effect = create(rate);
                effect.set(&[color, 50.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 120.0);
                let n = rate as usize;
                let out: Vec<f32> = (0..n)
                    .map(|i| {
                        let x = 0.3 * (i as f32 * 0.03).sin() + 0.2 * (i as f32 * 0.21).sin();
                        effect.process([x, x])[0]
                    })
                    .collect();
                let level = rms(&out[n / 2..]);
                assert!(level > 0.08 && level < 0.6, "{color} {rate}: {level}");
            }
        }
    }
}
