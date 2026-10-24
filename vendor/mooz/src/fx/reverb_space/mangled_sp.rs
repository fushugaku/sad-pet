//! MangledSp (MNGLD_SP.ZDL, `Fx_REV_MangledSp`).
//!
//! Follows the recovered callback (`mangled_space/samples.rs`): the inputs are
//! cross-blended (0.67/0.11), run through the input EQ (C42 800 Hz bell, C47
//! Low shelf, C4C High filter) and into two pre-delays whose read points are
//! modulated by a raised-cosine LFO (C94, 2.9..3.6 Hz) shaped by a 4.64 Hz
//! amplitude LFO (C170/C174, depth 0.37). The right pre-delay is longer by
//! `(1 - Decay) * (3 + 1.17 Size)` ms. Each side then runs four diffusion
//! stages of the original's peculiar form `w = x + g d`, `y = d - x`, mixes
//! `Decay * x + (1 - 0.6 Decay^2) * diffused` into an eight-tap early
//! reflection line whose gains swell towards the late taps (Decay tables at
//! 0x80000810), feeds `x` and the reflections into the ±0.1 hard clipper
//! (Clip drive table, 2.2387 makeup), and the clipped signal drives four
//! damping-free combs per side. The output sums fixed taps of all eight combs
//! (same side and crossed), the clipped signal, a feedback loop on the
//! pre-delayed input (C8D/C8E) and `1.35 Decay^5` of the pre-delayed input,
//! then the output filters (C99 High low-pass, C9E Clip low-pass, the ROM
//! 54 Hz high-pass) and the 1.8 * Size-dependent wet gain.
//!
//! Editors decoded: decay_edit, decay_Calc_set (RT = 650 + 10000 (Decay
//! Size)^2.6 ms, comb gains `base^(delay/RT)`), size_edit (all delay tables,
//! the Size < 10 crossfades), mix_edit, Clip_edit, Low_edit, High_edit (filter
//! service types 2/9/10 rebuilt in `dual_common`), Mod_edit, PreDelay_edit,
//! Level_edit, onf_aft (C51/C58/C59 = 1).
//!
//! Approximations: the editors' input-mute dance and Size_tapmuteMute (which
//! silences the tail while delays are recomputed) are replaced by a glide of
//! every delay length; with Size >= 10 the clip input keeps the ROM direct gain
//! C179 = 1.0, as after a cold patch load (the firmware only sets it below 10);
//! each pre-delay uses its own interpolation fraction (the original applies the
//! right channel's previous fraction to both); the 6.67 Hz LFO with depth 1e-5
//! is omitted; TBL_TO_VAL reads past its table for High = 200 and is clamped.
use super::dual_common::{self as common, Gains, Glide};
use crate::dsp::{Biquad, Coefficients, DelayLine};
use crate::zoom::ORIGINAL_RATE;
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_mangled_sp",
    name: "MangledSp",
    category: Category::Reverb,
    zdl: "MNGLD_SP.ZDL",
    description: "Reverb with clipped, swelling early reflections",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "PDLY",
            max: 500,
            default: 80,
            display: Display::number(0.0, 1.0, 0, " ms"),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Clip",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 38,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Decay",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Mod",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Size",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "Low",
            max: 200,
            default: 97,
            display: Display::offset(-100.0),
        },
        // values[7] is firmware parameter word 9
        ParamInfo {
            name: "High",
            max: 200,
            default: 101,
            display: Display::offset(-100.0),
        },
        // values[8] is firmware parameter word 10
        ParamInfo {
            name: "Level",
            max: 200,
            default: 85,
            display: Display::INT,
        },
    ],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(MangledSpace::new(sample_rate))
}

/// Early reflection times (ms) at Size 0 and 100 (0x800008b0, both sides).
const EARLY_MS: [(f32, f32); 8] = [
    (0.2, 17.0),
    (0.7, 47.0),
    (1.7, 83.0),
    (2.9, 139.0),
    (4.1, 211.0),
    (5.3, 271.0),
    (6.7, 317.0),
    (7.9, 409.0),
];
/// Early reflection gains over Decay 0, 25, 50, 75, 100 (0x80000810).
const EARLY_GAIN: [[f32; 5]; 8] = [
    [0.01, 0.139_691_9, 0.355, 0.252_631_4, 0.3],
    [0.02, 0.145_219_8, 0.36, 0.254_264_8, 0.3],
    [0.01, 0.139_691_9, 0.355, 0.252_631_4, 0.3],
    [0.07, 0.172_859_1, 0.385, 0.262_431_8, 0.3],
    [0.15, 0.217_082, 0.425, 0.275_499, 0.3],
    [0.24, 0.266_832_8, 0.47, 0.290_199_6, 0.3],
    [0.45, 0.382_918, 0.575, 0.324_501, 0.3],
    [0.98, 0.675_894_8, 0.84, 0.411_071_2, 0.3],
];
/// Diffuser delays (ms) at Size 0 and 100, left then right (0x80000930).
const DIFFUSER_MS: [[(f32, f32); 4]; 2] = [
    [
        (3.0, 37.043),
        (2.464_470_4, 30.430_46),
        (1.995_208_9, 24.636_173),
        (1.591_898_6, 19.656_233),
    ],
    [
        (4.0, 39.043),
        (3.464_470_4, 32.430_46),
        (2.995_208_7, 26.636_173),
        (2.591_898_7, 21.656_233),
    ],
];
/// Comb delays (ms) at Size 0 and 100, left then right (0x80000970).
const COMB_MS: [[(f32, f32); 4]; 2] = [
    [
        (29.0, 144.433_53),
        (31.0, 121.503_14),
        (37.0, 106.682_27),
        (19.0, 83.052_77),
    ],
    [
        (30.0, 146.433_53),
        (32.0, 123.503_14),
        (38.0, 108.682_27),
        (20.0, 85.052_77),
    ],
];
/// `base^(delay / RT)` per comb pair (0x80000aa0) and for the input loops.
const COMB_BASE: [f32; 4] = [0.001_238_05, 0.001_303_21, 0.001_371_8, 0.001_444];
const LOOP_BASE: f32 = 0.001_24;
/// Fixed output taps into each comb (44.1 kHz samples after its write point):
/// the left output reads the first row, the right output the second.
const OUTPUT_TAPS: [[f32; 4]; 2] = [
    [1129.0, 1279.0, 1453.0, 1627.0],
    [43.0, 173.0, 353.0, 523.0],
];
/// Right input loop C8E -> C3A, never changed by an editor.
const RIGHT_LOOP: f32 = 3219.0;
const MIX_TABLE: [f32; 5] = [0.0, 0.375_837_4, 0.707_945_8, 0.944_060_9, 1.0];
const DRIVE_TABLE: [f32; 3] = [0.251_188_64, 0.605_312_6, 14.125_376];
const CLIP_HZ: [f32; 3] = [20_000.0, 18_000.0, 14_000.0];
const MAKEUP: f32 = 2.238_721;
const LOW_CUT_DB: [f32; 5] = [-12.0, -11.0, -9.0, -5.0, 0.0];
const LOW_BOOST_DB: [f32; 5] = [0.0, 2.5, 5.0, 7.5, 9.0];
const PRESENCE_DB: [f32; 5] = [0.0, 2.0, 4.0, 6.0, 8.0];
const BODY_DB: [f32; 5] = [0.0, -1.5, -3.0, -4.5, -5.0];
const DAMP_HZ: [f32; 5] = [3600.0, 3800.0, 4000.0, 4200.0, 4300.0];
const DARK_HZ: [f32; 5] = [2300.0, 2600.0, 2800.0, 5000.0, 20_000.0];
const SMALL_WET: [f32; 5] = [0.75, 0.7, 0.4, 0.3, 0.24];
const LARGE_WET: [f32; 5] = [0.24, 0.24, 0.195, 0.165, 0.13];
/// Maximum modulation, C145 = 0x09616296 in Q31 shifted right by 21.
const MOD_SAMPLES: f32 = 1024.0 * 0.073_284_46;

/// The ROM high-pass in C163..C167 (about 54 Hz), never changed by an editor.
fn rom_highpass() -> Coefficients {
    Coefficients {
        b0: f32::from_bits(0x3f7e2f9d),
        b1: f32::from_bits(0xbffe2f9d),
        b2: f32::from_bits(0x3f7e2f9d),
        a1: -f32::from_bits(0x3ffe2ecb),
        a2: -f32::from_bits(0xbf7c60dd),
    }
}

// Glide slots (host samples).
const DIFFUSER: usize = 0; // 0..8: left 0..4, right 4..8
const EARLY: usize = 8; // 8..16
const COMB: usize = 16; // 16..24: left 0..4, right 4..8
const LOOP_LEFT: usize = 24;
const PRE: usize = 25; // 25, 26
const DELAYS: usize = 27;

// Smoothed gains.
const G_DIFFUSION: usize = 0; // C84/C85
const G_DIRECT: usize = 1; // C144
const G_DIFFUSED: usize = 2; // C143
const G_CLIP_DIRECT: usize = 3; // C179
const G_CLIP_EARLY: usize = 4; // C180
const G_DRIVE: usize = 5; // C168
const G_COMB: usize = 6; // 6..14: C99..C106
const G_LOOP_LEFT: usize = 14; // C176
const G_LOOP_RIGHT: usize = 15; // C177
const G_LOOP_OUT: usize = 16; // C178 * C182
const G_SAME: usize = 17; // C109/C112
const G_CROSS: usize = 18; // C110/C111
const G_INPUT_OUT: usize = 19; // C113/C114
const G_WET: usize = 20; // 1.8 * C183 * C119 * C2
const G_DRY: usize = 21; // C120 * C2
const G_EARLY: usize = 22; // 22..30: C90..C97
const G_MOD: usize = 30; // C145 in host samples
const GAINS: usize = 31;

struct MangledSpace {
    ratio: f32,
    sample_rate: f32,
    last: Option<Values>,
    input: [[Biquad; 3]; 2],
    output: [[Biquad; 3]; 2],
    pre: [DelayLine; 2],
    diffusers: [[DelayLine; 4]; 2],
    early: [DelayLine; 2],
    combs: [[DelayLine; 4]; 2],
    loops: [DelayLine; 2],
    taps: [[usize; 4]; 2],
    right_loop: f32,
    glide: Glide<DELAYS>,
    gains: Gains<GAINS>,
    phase_lfo: f32,
    phase_am: f32,
    rate_lfo: f32,
    rate_am: f32,
}

fn line_ms(ms: f32, ratio: f32) -> DelayLine {
    DelayLine::new((ms * 44.1 * ratio).ceil() as usize + 8)
}

impl MangledSpace {
    fn new(sample_rate: f32) -> Self {
        let ratio = sample_rate / ORIGINAL_RATE;
        let pre_samples =
            |ms: f32| DelayLine::new(((ms * 44.1 + 80.0) * ratio).ceil() as usize + 8);
        let mut effect = Self {
            ratio,
            sample_rate,
            last: None,
            input: [[Biquad::default(); 3]; 2],
            output: [[Biquad::default(); 3]; 2],
            pre: [pre_samples(500.0), pre_samples(620.0)],
            diffusers: std::array::from_fn(|side| {
                std::array::from_fn(|i| line_ms(DIFFUSER_MS[side][i].1, ratio))
            }),
            early: std::array::from_fn(|_| line_ms(EARLY_MS[7].1, ratio)),
            combs: std::array::from_fn(|side| {
                std::array::from_fn(|i| {
                    line_ms(COMB_MS[side][i].1.max(OUTPUT_TAPS[0][3] / 44.1), ratio)
                })
            }),
            loops: [line_ms(88.0, ratio), line_ms(74.0, ratio)],
            taps: std::array::from_fn(|row| {
                std::array::from_fn(|i| ((OUTPUT_TAPS[row][i] * ratio).round() as usize).max(1))
            }),
            right_loop: (RIGHT_LOOP * ratio).round().max(2.0),
            glide: Glide::new(sample_rate),
            gains: Gains::new(sample_rate),
            phase_lfo: 0.0,
            phase_am: 0.0,
            rate_lfo: 0.0,
            // C170: 451896 per 44.1 kHz sample of a 2^32 wrap.
            rate_am: 451_896.0 / 4_294_967_296.0 * ORIGINAL_RATE / sample_rate,
        };
        for side in 0..2 {
            effect.output[side][2] =
                Biquad::new(rom_highpass().resample(ORIGINAL_RATE, sample_rate));
        }
        let defaults = crate::Model::from_key(INFO.key)
            .map(crate::Model::defaults)
            .unwrap_or([0.0; crate::MAX_PARAMS]);
        effect.apply(&defaults, true);
        effect
    }

    /// Delay in host samples for a firmware length truncated at 44.1 kHz.
    fn samples(&self, at_44k: f32) -> f32 {
        (at_44k.trunc() * self.ratio).round().max(1.0)
    }

    fn apply(&mut self, values: &Values, jump: bool) {
        let last = self.last;
        let changed = |i: usize| last.is_none_or(|l| l[i] != values[i]);
        let pdly = values[0].clamp(0.0, 500.0);
        let clip = values[1].clamp(0.0, 100.0);
        let mix = values[2].clamp(0.0, 100.0);
        let decay = values[3].clamp(0.0, 100.0);
        let modulation = values[4].clamp(0.0, 100.0);
        let size = values[5].clamp(0.0, 100.0);
        let low = values[6].clamp(0.0, 200.0);
        let high = values[7].clamp(0.0, 200.0);
        let level = values[8].clamp(0.0, 200.0);
        let d = decay / 100.0;
        let s = size / 100.0;
        let sr = self.sample_rate;

        // Delay lengths: size_edit, decay_edit and PreDelay_edit.
        let mut delays = [1.0; DELAYS];
        let lerp = |(a, b): (f32, f32)| a + (b - a) * s;
        let mut comb_44k = [[0.0; 4]; 2];
        for side in 0..2 {
            for i in 0..4 {
                delays[DIFFUSER + side * 4 + i] = self.samples(44.1 * lerp(DIFFUSER_MS[side][i]));
                comb_44k[side][i] = (44.1 * lerp(COMB_MS[side][i])).trunc();
                delays[COMB + side * 4 + i] = self.samples(comb_44k[side][i]);
            }
        }
        for (i, &range) in EARLY_MS.iter().enumerate() {
            delays[EARLY + i] = self.samples(44.1 * lerp(range));
        }
        let loop_44k = (44.1 * (41.0 + 46.0 * s)).trunc();
        delays[LOOP_LEFT] = self.samples(loop_44k);
        // Pre-delays: one sample plus the truncated time, the right one longer.
        delays[PRE] = (1.0 + (44.1 * pdly).trunc()) * self.ratio;
        delays[PRE + 1] =
            (1.0 + (44.1 * (pdly + (1.0 - d) * (3.0 + 1.17 * size))).trunc()) * self.ratio;
        self.glide.set(&delays, jump);

        // decay_Calc_set: comb feedback for the requested reverb time.
        let rt = 650.0 + 10_000.0 * (d * s).powf(2.6);
        for (side, lengths) in comb_44k.iter().enumerate() {
            for (i, (&length, base)) in lengths.iter().zip(COMB_BASE).enumerate() {
                let gain = base.powf(length / 44.1 / rt);
                self.gains.set(G_COMB + side * 4 + i, gain, jump);
            }
        }
        self.gains
            .set(G_LOOP_LEFT, LOOP_BASE.powf(loop_44k / 44.1 / rt), jump);
        self.gains
            .set(G_LOOP_RIGHT, LOOP_BASE.powf(RIGHT_LOOP / 44.1 / rt), jump);
        self.gains.set(G_DIFFUSION, 0.25 + 0.39 * s, jump);

        // decay_edit.
        for (i, table) in EARLY_GAIN.iter().enumerate() {
            self.gains
                .set(G_EARLY + i, common::table(101, decay, table), jump);
        }
        self.gains.set(G_INPUT_OUT, 1.35 * d.powi(5), jump);
        self.gains.set(G_DIFFUSED, 1.0 - 0.6 * d * d, jump);
        self.gains.set(G_DIRECT, d, jump);

        // size_edit: below 10 the clipper and output blend change with Size.
        let (clip_direct, clip_early, cross, loop_size, wet_size) = if size < 10.0 {
            let t = (size / 9.0).min(1.0);
            (
                1.0 - 0.9 * t,
                0.9 * t,
                0.8 * t.sqrt(),
                0.1 + 0.9 * t * t,
                common::table(11, size, &SMALL_WET),
            )
        } else {
            (1.0, 0.9, 0.8, 1.0, common::table(101, size, &LARGE_WET))
        };
        self.gains.set(G_CLIP_DIRECT, clip_direct, jump);
        self.gains.set(G_CLIP_EARLY, clip_early, jump);
        self.gains.set(G_CROSS, cross, jump);
        self.gains.set(G_SAME, 0.8 * s.sqrt(), jump);
        self.gains
            .set(G_LOOP_OUT, loop_size * 0.5 * d.powf(0.75), jump);

        // mix_edit and Level_edit.
        let level = level / 100.0;
        let wet = common::table(101, mix, &MIX_TABLE);
        let dry = common::table(101, 100.0 - mix, &MIX_TABLE);
        self.gains.set(G_WET, 1.8 * wet_size * wet * level, jump);
        self.gains.set(G_DRY, dry * level, jump);

        // Clip_edit: drive and the output low-pass.
        self.gains
            .set(G_DRIVE, common::table(101, clip, &DRIVE_TABLE), jump);
        if changed(1) {
            let lowpass = common::lowpass(sr, common::table(101, clip, &CLIP_HZ), 0.75);
            for side in 0..2 {
                self.output[side][1].coefficients = lowpass;
            }
        }

        // Mod_edit: depth C145 and the C148 rate (282435..350609 per 2^32).
        self.gains
            .set(G_MOD, MOD_SAMPLES * modulation / 100.0 * self.ratio, jump);
        let increment = 282_435.0 + 68_174.0 * modulation / 100.0;
        self.rate_lfo = increment / 4_294_967_296.0 * ORIGINAL_RATE / sr;

        // Low_edit: first-order low shelf, a cut at 50 Hz or a boost at 130 Hz.
        if changed(6) {
            let shelf = if low < 101.0 {
                common::low_shelf1(sr, 50.0, common::table(101, low, &LOW_CUT_DB))
            } else {
                common::low_shelf1(sr, 130.0, common::table(101, low - 100.0, &LOW_BOOST_DB))
            };
            for side in 0..2 {
                self.input[side][1].coefficients = shelf;
            }
        }
        // High_edit: 800 Hz bell, 5200 Hz bell or darkening low-pass on the
        // input, and the 3600..4300 Hz low-pass on the output.
        if changed(7) {
            let (body, top, damp) = if high < 100.0 {
                (
                    common::peak(sr, 800.0, 1.0, 0.0),
                    common::lowpass(sr, common::table(100, high, &DARK_HZ), 0.75),
                    common::lowpass(sr, 3600.0, 0.75),
                )
            } else {
                let t = high - 100.0;
                (
                    common::peak(sr, 800.0, 1.0, common::table(100, t, &BODY_DB)),
                    common::peak(sr, 5200.0, 1.0, common::table(100, t, &PRESENCE_DB)),
                    common::lowpass(sr, common::table(100, t, &DAMP_HZ), 0.75),
                )
            };
            for side in 0..2 {
                self.input[side][0].coefficients = body;
                self.input[side][2].coefficients = top;
                self.output[side][0].coefficients = damp;
            }
        }
        self.last = Some(*values);
    }

    #[inline(always)]
    fn tap<const GLIDE: bool>(line: &DelayLine, delay: f32) -> f32 {
        if GLIDE {
            line.read_linear(delay - 1.0)
        } else {
            line.read(delay as usize - 1)
        }
    }

    #[inline(always)]
    fn frame<const GLIDE: bool>(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        self.gains.tick();
        if GLIDE {
            self.glide.tick();
        }
        let g = &self.gains.value;
        let delay = &self.glide.current;

        // Pre-delay modulation: C94 raised cosine times the C170 envelope.
        let envelope = 1.0 - 0.37 * (1.0 - common::lfo_shape(self.phase_am));
        let modulation = g[G_MOD] * common::lfo_shape(self.phase_lfo) * envelope;
        common::advance(&mut self.phase_am, self.rate_am);
        common::advance(&mut self.phase_lfo, self.rate_lfo);
        let modulated = [modulation, modulation * envelope];

        let crossed = [0.67 * left + 0.11 * right, 0.67 * right + 0.11 * left];
        let mut x = [0.0; 2];
        let mut clipped = [0.0; 2];
        for side in 0..2 {
            let [body, low, top] = &mut self.input[side];
            let filtered = top.process(low.process(body.process(crossed[side])));
            let pre = &mut self.pre[side];
            x[side] = pre.read_linear(delay[PRE + side] + modulated[side] - 1.0);
            pre.write(filtered);

            let mut y = x[side];
            for (i, line) in self.diffusers[side].iter_mut().enumerate() {
                let d = Self::tap::<GLIDE>(line, delay[DIFFUSER + side * 4 + i]);
                line.write(y + g[G_DIFFUSION] * d);
                y = d - y;
            }
            let early_line = &mut self.early[side];
            let mut early = 0.0;
            for i in 0..8 {
                early += g[G_EARLY + i] * Self::tap::<GLIDE>(early_line, delay[EARLY + i]);
            }
            early_line.write(g[G_DIRECT] * x[side] + g[G_DIFFUSED] * y);
            let drive = g[G_DRIVE] * (g[G_CLIP_DIRECT] * x[side] + g[G_CLIP_EARLY] * early);
            clipped[side] = MAKEUP * drive.clamp(-0.1, 0.1);
        }

        // Combs: [side fed][output side] sums of the fixed taps.
        let mut sums = [[0.0; 2]; 2];
        for side in 0..2 {
            let feed = 0.43 * clipped[side];
            for (i, line) in self.combs[side].iter_mut().enumerate() {
                let feedback = Self::tap::<GLIDE>(line, delay[COMB + side * 4 + i]);
                sums[side][0] += line.read(self.taps[0][i] - 1);
                sums[side][1] += line.read(self.taps[1][i] - 1);
                line.write(feed + g[G_COMB + side * 4 + i] * feedback);
            }
        }

        // Input loops C8D/C8E: the feedback is taken one sample later.
        let loop_left = &mut self.loops[0];
        let left_loop = Self::tap::<GLIDE>(loop_left, delay[LOOP_LEFT]);
        let left_feedback = Self::tap::<GLIDE>(loop_left, delay[LOOP_LEFT] + 1.0);
        loop_left.write(x[0] + g[G_LOOP_LEFT] * left_feedback);
        let loop_right = &mut self.loops[1];
        let right_loop = loop_right.read(self.right_loop as usize - 1);
        let right_feedback = loop_right.read(self.right_loop as usize);
        loop_right.write(x[1] + g[G_LOOP_RIGHT] * right_feedback);

        let direct = g[G_INPUT_OUT] * (x[0] + x[1]);
        let wet = [
            g[G_SAME] * sums[0][0]
                + g[G_CROSS] * sums[1][0]
                + clipped[0]
                + g[G_LOOP_OUT] * left_loop
                + direct,
            g[G_SAME] * sums[1][1]
                + g[G_CROSS] * sums[0][1]
                + clipped[1]
                + g[G_LOOP_OUT] * right_loop
                + direct,
        ];
        let dry = [left, right];
        let mut out = [0.0; 2];
        for side in 0..2 {
            let [damp, clip, highpass] = &mut self.output[side];
            let filtered = highpass.process(clip.process(damp.process(wet[side])));
            out[side] = g[G_WET] * filtered + g[G_DRY] * dry[side];
        }
        out
    }
}

impl Effect for MangledSpace {
    fn set(&mut self, values: &Values, _bpm: f32) {
        if self.last != Some(*values) {
            self.apply(values, false);
        }
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        if self.glide.moving {
            self.frame::<true>(frame)
        } else {
            self.frame::<false>(frame)
        }
    }

    fn clear(&mut self) {
        for side in 0..2 {
            for filter in self.input[side].iter_mut().chain(&mut self.output[side]) {
                filter.clear();
            }
            self.pre[side].clear();
            self.early[side].clear();
            self.loops[side].clear();
            for line in self.diffusers[side].iter_mut().chain(&mut self.combs[side]) {
                line.clear();
            }
        }
        self.phase_lfo = 0.0;
        self.phase_am = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Values {
        crate::Model::from_key(INFO.key).unwrap().defaults()
    }

    fn render(
        effect: &mut MangledSpace,
        input: impl Fn(usize) -> [f32; 2],
        n: usize,
    ) -> Vec<[f32; 2]> {
        (0..n).map(|i| effect.process(input(i))).collect()
    }

    #[test]
    fn comb_gains_follow_decay_calc_set() {
        let mut values = defaults();
        values[3] = 100.0;
        values[5] = 100.0;
        let mut effect = MangledSpace::new(44_100.0);
        effect.apply(&values, true);
        // RT = 10650 ms, first left comb 6369 samples = 144.42 ms.
        let expected = COMB_BASE[0].powf(6369.0 / 44.1 / 10_650.0);
        assert!((effect.gains.value[G_COMB] - expected).abs() < 1e-6);
        assert!((expected - 0.9132).abs() < 1e-3);
        assert_eq!(effect.glide.current[COMB], 6369.0);
        // Size 0: 29 ms.
        values[5] = 0.0;
        effect.apply(&values, true);
        assert_eq!(effect.glide.current[COMB], (44.1f32 * 29.0).trunc());
        assert_eq!(effect.glide.current[EARLY + 7], (44.1f32 * 7.9).trunc());
    }

    #[test]
    fn pre_delay_and_right_offset_in_samples() {
        let mut values = defaults();
        values[0] = 100.0;
        values[3] = 50.0;
        values[5] = 50.0;
        let mut effect = MangledSpace::new(48_000.0);
        effect.apply(&values, true);
        let ratio = 48_000.0 / 44_100.0;
        assert!((effect.glide.current[PRE] - 4411.0 * ratio).abs() < 1e-2);
        // Right: 100 + 0.5 * (3 + 58.5) ms.
        let right = 1.0 + (44.1f32 * (100.0 + 0.5 * (3.0 + 58.5))).trunc();
        assert!((effect.glide.current[PRE + 1] - right * ratio).abs() < 1e-2);
    }

    #[test]
    fn impulse_arrives_after_the_pre_delay() {
        let mut values = defaults();
        values[0] = 100.0; // 100 ms
        values[2] = 100.0; // wet only
        values[4] = 0.0; // no modulation
        let mut effect = MangledSpace::new(44_100.0);
        effect.apply(&values, true);
        let out = render(&mut effect, |i| [if i == 0 { 1.0 } else { 0.0 }; 2], 6000);
        let first = out.iter().position(|f| f[0].abs() > 1e-9).unwrap();
        assert_eq!(first, 4411);
    }

    #[test]
    fn default_level_is_sane_and_max_settings_stay_bounded() {
        let mut effect = MangledSpace::new(48_000.0);
        let mut noise = crate::dsp::Noise::new(3);
        let mut energy_in = 0.0;
        let mut energy_out = 0.0;
        for _ in 0..96_000 {
            let x = 0.3 * noise.bipolar();
            let [l, r] = effect.process([x, x]);
            energy_in += x * x;
            energy_out += 0.5 * (l * l + r * r);
        }
        let ratio_db = 10.0 * (energy_out / energy_in).log10();
        assert!((-10.0..3.0).contains(&ratio_db), "{ratio_db} dB");

        let max = [
            500.0, 100.0, 100.0, 100.0, 100.0, 100.0, 200.0, 200.0, 200.0,
        ];
        effect.set(&max, 120.0);
        let mut peak = 0.0f32;
        for i in 0..(48_000 * 6) {
            let x = if i < 48_000 {
                0.5 * noise.bipolar()
            } else {
                0.0
            };
            for y in effect.process([x, -x]) {
                assert!(y.is_finite());
                peak = peak.max(y.abs());
            }
        }
        assert!(peak < 8.0, "{peak}");
    }

    #[test]
    fn size_changes_glide_without_jumps() {
        let mut effect = MangledSpace::new(44_100.0);
        let tone = |i: usize| {
            let x = 0.3 * (i as f32 * 0.05).sin();
            [x, x]
        };
        let mut previous = render(&mut effect, tone, 20_000).last().copied().unwrap();
        let mut values = defaults();
        values[5] = 100.0;
        effect.set(&values, 120.0);
        assert!(effect.glide.moving);
        let mut largest = 0.0f32;
        for i in 20_000..60_000 {
            let out = effect.process(tone(i));
            largest = largest.max((out[0] - previous[0]).abs());
            previous = out;
        }
        assert!(!effect.glide.moving);
        assert!(largest < 0.1, "{largest}");
    }
}
