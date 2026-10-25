//! DualRev (DUAL_REV.ZDL, `Fx_REV_DualRev`).
//!
//! Follows the recovered callback (`dual_reverb/samples.rs`): the mono sum
//! passes the ROM high-pass (about 63 Hz) and feeds two independent reverbs.
//! Each has its own pre-delay (B's read point swept by a 1 Hz raised-cosine
//! LFO, 56.5 samples deep; A's ROM depth is zero), four all-pass diffusers
//! (A: g 0.5, 17..67 ms; B: g 0.3, 7..29 ms), an eight-tap early reflection
//! line, and four combs fed with 0.43 of the reflections whose loops hold a
//! first-order high shelf and low shelf set by Size. Stereo outputs come from
//! fixed taps into the combs plus the reflections and the pre-delayed input,
//! then the ROM one-pole low-pass (A 3.75 kHz, B 2.75 kHz) and the Tone tilt
//! (650 Hz low shelf, 3.5 kHz high shelf of opposite gain). The A/B balance,
//! the fixed 1.4 / 0.57 reverb levels and the MIX table give the output. All
//! delay lengths are the ROM ones; the 20-word write table and the read
//! coefficients C9..C56 are never changed by an editor.
//!
//! Editors decoded: init, decayA/B_Calc_set (RT = 200 ms * DCY, comb gains
//! `thrLvl^(delay/RT)` from Fx_REV_DualRev_decay_thrLvl, diffuser gains),
//! size_edit (A1B1..A1B1 path, LoDmp/HiDmp tables, filter types 8/9),
//! mix_edit (MIX_Tbl), ABMix_edit, PreDelayA/B_edit ((value + 1) ms),
//! ToneA/B_edit (EqGain_tbl), onf_aft (C62/C63/C69/C207/C208 = 1).
//!
//! Approximations: Size swaps the damping filters directly instead of muting
//! the combs for 210 ms and restoring them through the deferred decayAB_edit;
//! each pre-delay uses its own interpolation fraction (the original applies
//! the previous frame's).
use super::dual_common::{self as common, FirstOrder, FirstOrderBank, Gains, Glide};
use super::fast::{Bank, Biquad, DelayLine};
use crate::dsp::Coefficients;
use crate::zoom::ORIGINAL_RATE;
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_dual_rev",
    name: "DualRev",
    category: Category::Reverb,
    zdl: "DUAL_REV.ZDL",
    description: "Two independent reverbs with their own decay and tone",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "PDlyA",
            max: 750,
            default: 350,
            display: Display::number(1.0, 1.0, 0, " ms"),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "PDlyB",
            max: 750,
            default: 700,
            display: Display::number(1.0, 1.0, 0, " ms"),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 30,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "ABMix",
            max: 18,
            default: 14,
            display: Display::labels(&[
                "A9B0", "A9B1", "A9B2", "A9B3", "A9B4", "A9B5", "A9B6", "A9B7", "A9B8", "A9B9",
                "A8B9", "A7B9", "A6B9", "A5B9", "A4B9", "A3B9", "A2B9", "A1B9", "A0B9",
            ]),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "DCY A",
            max: 100,
            default: 15,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "DCY B",
            max: 100,
            default: 30,
            display: Display::INT,
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "Size",
            max: 32,
            default: 21,
            display: Display::labels(&[
                "A1B1", "A2B1", "A3B1", "A4B1", "A5B1", "A6B1", "A7B1", "A8B1", "A9B1", "A9B2",
                "A9B3", "A9B4", "A9B5", "A9B6", "A9B7", "A9B8", "A9B9", "A8B9", "A7B9", "A6B9",
                "A5B9", "A4B9", "A3B9", "A2B9", "A1B9", "A1B8", "A1B7", "A1B6", "A1B5", "A1B4",
                "A1B3", "A1B2", "A1B1",
            ]),
        },
        // values[7] is firmware parameter word 9
        ParamInfo {
            name: "ToneA",
            max: 200,
            default: 100,
            display: Display::offset(-100.0),
        },
        // values[8] is firmware parameter word 10
        ParamInfo {
            name: "ToneB",
            max: 200,
            default: 70,
            display: Display::offset(-100.0),
        },
    ],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(DualReverb::new(sample_rate))
}

/// Per reverb (A, B): delays in 44.1 kHz samples from the ROM offsets.
const DIFFUSER: [[f32; 4]; 2] = [
    [754.0, 1327.0, 2242.0, 2972.0],
    [325.0, 572.0, 965.0, 1280.0],
];
const DIFFUSION: [f32; 2] = [0.5, 0.3];
const EARLY_DELAY: [[f32; 8]; 2] = [
    [88.0, 617.0, 1499.0, 2558.0, 3263.0, 5204.0, 5909.0, 7850.0],
    [325.0, 740.0, 941.0, 1194.0, 1508.0, 1902.0, 2396.0, 3014.0],
];
const EARLY_GAIN: [[f32; 8]; 2] = [[0.4, 0.3, 0.2, 0.1, 0.0, 0.0, 0.0, 0.0], [0.3; 8]];
const COMB: [[f32; 4]; 2] = [
    [7006.0, 5894.0, 5175.0, 4028.0],
    [5350.0, 4501.0, 3952.0, 3077.0],
];
/// Fixed output taps into each comb, `[reverb][output side][comb]`.
const OUTPUT_TAP: [[[f32; 4]; 2]; 2] = [
    [
        [1279.0, 743.0, 1103.0, 1669.0],
        [839.0, 571.0, 1367.0, 983.0],
    ],
    [[1129.0, 1279.0, 1453.0, 523.0], [43.0, 173.0, 353.0, 523.0]],
];
/// `Fx_REV_DualRev_decay_thrLvl`.
const THRESHOLD: [[f32; 4]; 2] = [
    [0.008_75, 0.008_96, 0.008_54, 0.008_33],
    [0.001_872_5, 0.001_917_44, 0.001_827_56, 0.001_782_62],
];
/// C95/C96 and C97/C98: early reflections to the output, comb tap sums
/// `[left, right]`, and C101/C102 the pre-delayed input to the output.
const EARLY_OUT: [f32; 2] = [0.2, 1.0];
const TAP_OUT: [[f32; 2]; 2] = [[0.85, 0.85], [1.0, 1.0]];
const DIRECT_OUT: [f32; 2] = [0.37, 0.57];
/// C153/C154: fixed reverb levels.
const LEVEL: [f32; 2] = [1.4, 0.57];
/// C114 in Q31 shifted right by 21: B's pre-delay sweep in samples.
const SWEEP: [f32; 2] = [0.0, 1024.0 * 0.055_179_14];
/// C117: 97391 per 44.1 kHz sample of a 2^32 wrap (1.0 Hz).
const SWEEP_INCREMENT: f32 = 97_391.0 / 4_294_967_296.0;
const MIX_TABLE: [f32; 5] = [0.0, 0.375_837_4, 0.707_945_8, 0.944_060_9, 1.0];
const EQ_GAIN: [f32; 9] = [6.5, 5.0, 3.0, 2.0, 0.0, -2.0, -3.0, -5.0, -6.5];
const HI_DAMP_HZ: [f32; 2] = [6500.0, 4000.0];
const HI_DAMP_DB: [f32; 2] = [-6.0, -2.0];
const LO_DAMP_HZ: [f32; 2] = [30.0, 60.0];
const LO_DAMP_DB: [f32; 2] = [-6.0, -2.0];
const MAX_PRE_MS: f32 = 751.0;

/// The ROM input high-pass C57..C61.
fn rom_highpass() -> Coefficients {
    Coefficients {
        b0: f32::from_bits(0x3f7c663c),
        b1: f32::from_bits(0xbffc6634),
        b2: f32::from_bits(0x3f7c663c),
        a1: -f32::from_bits(0x3ffc6506),
        a2: -f32::from_bits(0xbf78cec4),
    }
}

/// The ROM one-pole low-passes C143..C147 (A) and C148..C152 (B) are
/// bilinear designs at 44.1 kHz (3750 Hz and 2750 Hz); recover the corner from
/// the pole and design it again at the host rate.
fn rom_lowpass(reverb: usize, sample_rate: f32) -> Coefficients {
    let pole = f64::from(f32::from_bits(if reverb == 0 {
        0x3f11fbc6
    } else {
        0x3f2b37fc
    }));
    let k = (1.0 - pole) / (1.0 + pole);
    let hz = k.atan() * f64::from(ORIGINAL_RATE) / std::f64::consts::PI;
    common::lowpass1(sample_rate, hz as f32)
}

/// Size 0..32 to the (A, B) damping indices 0..8 of size_edit.
fn size_indices(size: u32) -> [f32; 2] {
    let (a, b) = match size {
        0..=8 => (size, 0),
        9..=16 => (8, size - 8),
        17..=24 => (24 - size, 8),
        _ => (0, 32 - size.min(32)),
    };
    [a as f32, b as f32]
}

const G_COMB: usize = 0; // 0..8: A 0..4, B 4..8
const G_WET: usize = 8; // C107
const G_DRY: usize = 9; // C108
const G_LEVEL: usize = 10; // 10, 11: 1.4 * C105, 0.57 * C106
const GAINS: usize = 12;

struct Reverb {
    diffusers: Bank<4>,
    early: DelayLine,
    /// The four combs, also read by the output taps.
    combs: Bank<4>,
    /// High then low shelf inside each comb loop.
    damping: [FirstOrderBank<4>; 2],
    /// ROM low-pass, Tone low shelf, Tone high shelf per output side.
    output: [[FirstOrder; 3]; 2],
}

/// The per-frame read positions both reverbs share.
struct Delays<'a> {
    pre: &'a DelayLine,
    sweep: f32,
    diffuser: &'a [[usize; 4]; 2],
    early: &'a [[usize; 8]; 2],
    comb: &'a [[usize; 4]; 2],
    taps: &'a [[[usize; 4]; 2]; 2],
}

impl Reverb {
    /// One reverb (`R` = 0 for A, 1 for B) for one frame, its output added to
    /// `wet`. Written per reverb so the ROM constants fold (and the zero
    /// early-reflection weights of A drop out, as the skipped taps did).
    #[inline(always)]
    fn process<const R: usize>(
        &mut self,
        delays: &Delays,
        pre_delay: f32,
        g: &[f32; GAINS],
        wet: &mut [f32; 2],
    ) {
        let x = delays
            .pre
            .read_linear(pre_delay + SWEEP[R] * delays.sweep - 1.0);

        let mut y = x;
        let gain = DIFFUSION[R];
        let mut diffusers = self.diffusers.frame();
        for (i, &delay) in delays.diffuser[R].iter().enumerate() {
            let d = diffusers.tap(i, delay);
            let w = y + gain * d;
            diffusers.write_raw(i, w);
            y = d - gain * w;
        }
        self.diffusers.advance();

        let early_line = &mut self.early;
        let mut early = 0.0;
        for (&delay, &weight) in delays.early[R].iter().zip(&EARLY_GAIN[R]) {
            if weight != 0.0 {
                early += weight * early_line.read(delay - 1);
            }
        }
        // C109/C110 diffused (A 0.75, B 1.0); C111/C112 direct are zero.
        early_line.write(if R == 0 { 0.75 * y } else { y });

        let feed = 0.43 * early;
        let mut sums = [0.0; 2];
        let mut combs = self.combs.frame();
        let [high, low] = &mut self.damping;
        for i in 0..4 {
            let feedback = combs.tap(i, delays.comb[R][i]);
            sums[0] += combs.tap(i, delays.taps[R][0][i]);
            sums[1] += combs.tap(i, delays.taps[R][1][i]);
            let input = feed + g[G_COMB + R * 4 + i] * feedback;
            combs.write_raw(i, low.process(i, high.process(i, input)));
        }
        self.combs.advance();

        for side in 0..2 {
            let sum = EARLY_OUT[R] * early + TAP_OUT[R][side] * sums[side] + DIRECT_OUT[R] * x;
            let [lowpass, low, high] = &mut self.output[side];
            wet[side] += g[G_LEVEL + R] * high.process(low.process(lowpass.process(sum)));
        }
    }
}

struct DualReverb {
    ratio: f32,
    sample_rate: f32,
    last: Option<Values>,
    input: Biquad,
    pre: DelayLine,
    pre_delay: Glide<2>,
    reverbs: [Reverb; 2],
    diffuser: [[usize; 4]; 2],
    early: [[usize; 8]; 2],
    comb: [[usize; 4]; 2],
    taps: [[[usize; 4]; 2]; 2],
    gains: Gains<GAINS>,
    phase: f32,
    increment: f32,
}

impl DualReverb {
    fn new(sample_rate: f32) -> Self {
        let ratio = sample_rate / ORIGINAL_RATE;
        let samples = |at_44k: f32| ((at_44k * ratio).round() as usize).max(1);
        let line = |at_44k: f32| DelayLine::new(samples(at_44k) + 4);
        let longest = |delays: &[f32]| samples(delays.iter().fold(0.0f32, |a, &b| a.max(b)));
        let reverbs = std::array::from_fn(|r| Reverb {
            diffusers: Bank::new([1; 4], longest(&DIFFUSER[r])),
            early: line(EARLY_DELAY[r][7]),
            combs: Bank::new(
                [1; 4],
                longest(&COMB[r])
                    .max(longest(&OUTPUT_TAP[r][0]))
                    .max(longest(&OUTPUT_TAP[r][1])),
            ),
            damping: [FirstOrderBank::default(); 2],
            output: std::array::from_fn(|_| {
                let mut chain = [FirstOrder::default(); 3];
                chain[0].set(rom_lowpass(r, sample_rate));
                chain
            }),
        });
        let mut effect = Self {
            ratio,
            sample_rate,
            last: None,
            input: Biquad::new(rom_highpass().resample(ORIGINAL_RATE, sample_rate)),
            pre: DelayLine::new(((MAX_PRE_MS * 44.1 + 70.0) * ratio).ceil() as usize + 8),
            pre_delay: Glide::new(sample_rate),
            reverbs,
            diffuser: std::array::from_fn(|r| std::array::from_fn(|i| samples(DIFFUSER[r][i]))),
            early: std::array::from_fn(|r| std::array::from_fn(|i| samples(EARLY_DELAY[r][i]))),
            comb: std::array::from_fn(|r| std::array::from_fn(|i| samples(COMB[r][i]))),
            taps: std::array::from_fn(|r| {
                std::array::from_fn(|side| std::array::from_fn(|i| samples(OUTPUT_TAP[r][side][i])))
            }),
            gains: Gains::new(sample_rate),
            phase: 0.0,
            increment: SWEEP_INCREMENT * ORIGINAL_RATE / sample_rate,
        };
        let defaults = crate::Model::from_key(INFO.key)
            .map(crate::Model::defaults)
            .unwrap_or([0.0; crate::MAX_PARAMS]);
        effect.apply(&defaults, true);
        effect
    }

    fn apply(&mut self, values: &Values, jump: bool) {
        let last = self.last;
        let changed = |i: usize| last.is_none_or(|l| l[i] != values[i]);
        let sr = self.sample_rate;

        // PreDelayA/B_edit: (value + 1) ms truncated at 44.1 kHz, plus one
        // sample for the interpolation pair. The original mutes the reverb
        // input while it moves the read point; here the read point glides.
        let pre_delay: [f32; 2] = std::array::from_fn(|r| {
            let ms = values[r].clamp(0.0, 750.0) + 1.0;
            ((44.1 * ms).trunc() + 1.0) * self.ratio
        });
        self.pre_delay.set(&pre_delay, jump);

        // mix_edit and ABMix_edit.
        let mix = values[2].clamp(0.0, 100.0);
        self.gains
            .set(G_WET, common::table(101, mix, &MIX_TABLE), jump);
        self.gains
            .set(G_DRY, common::table(101, 100.0 - mix, &MIX_TABLE), jump);
        let ab = values[3].clamp(0.0, 18.0);
        let (level_a, level_b) = if ab < 10.0 {
            (1.0, ab / 9.0)
        } else {
            (1.0 - (ab - 9.0) / 9.0, 1.0)
        };
        self.gains.set(G_LEVEL, LEVEL[0] * level_a, jump);
        self.gains.set(G_LEVEL + 1, LEVEL[1] * level_b, jump);

        // decayA/B_Calc_set: RT = 20 s * DCY, gain = thrLvl^(delay / RT).
        for r in 0..2 {
            let rt = 20_000.0 * values[4 + r].clamp(0.0, 100.0) / 100.0;
            for i in 0..4 {
                let ms = COMB[r][i] / 44.1;
                let gain = if rt > 0.0 {
                    THRESHOLD[r][i].powf(ms / rt)
                } else {
                    0.0
                };
                self.gains.set(G_COMB + r * 4 + i, gain, jump);
            }
        }

        // size_edit: damping shelves inside the comb loops.
        if changed(6) {
            let indices = size_indices(values[6].round().clamp(0.0, 32.0) as u32);
            for (reverb, &index) in self.reverbs.iter_mut().zip(&indices) {
                let high = common::high_shelf1(
                    sr,
                    common::table(9, index, &HI_DAMP_HZ),
                    common::table(9, index, &HI_DAMP_DB),
                );
                let low = common::low_shelf1(
                    sr,
                    common::table(9, index, &LO_DAMP_HZ),
                    common::table(9, index, &LO_DAMP_DB),
                );
                reverb.damping[0].set(high);
                reverb.damping[1].set(low);
            }
        }

        // ToneA/B_edit: a tilt around 650 Hz / 3.5 kHz.
        for r in 0..2 {
            if changed(7 + r) {
                let tone = values[7 + r].clamp(0.0, 200.0);
                let low = common::low_shelf1(sr, 650.0, common::table(201, tone, &EQ_GAIN));
                let high =
                    common::high_shelf1(sr, 3500.0, common::table(201, 200.0 - tone, &EQ_GAIN));
                for chain in self.reverbs[r].output.iter_mut() {
                    chain[1].set(low);
                    chain[2].set(high);
                }
            }
        }
        self.last = Some(*values);
    }
}

impl Effect for DualReverb {
    fn set(&mut self, values: &Values, _bpm: f32) {
        if self.last != Some(*values) {
            self.apply(values, false);
        }
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        self.gains.tick();
        if self.pre_delay.moving {
            self.pre_delay.tick();
        }
        let g = &self.gains.value;
        let sweep = common::lfo_shape(self.phase) * self.ratio;
        common::advance(&mut self.phase, self.increment);

        let filtered = self.input.process(0.5 * (left + right));
        let mut wet = [0.0; 2];
        let [a, b] = &mut self.reverbs;
        let delays = Delays {
            pre: &self.pre,
            sweep,
            diffuser: &self.diffuser,
            early: &self.early,
            comb: &self.comb,
            taps: &self.taps,
        };
        a.process::<0>(&delays, self.pre_delay.current[0], g, &mut wet);
        b.process::<1>(&delays, self.pre_delay.current[1], g, &mut wet);
        self.pre.write(filtered);
        [
            g[G_DRY] * left + g[G_WET] * wet[0],
            g[G_DRY] * right + g[G_WET] * wet[1],
        ]
    }

    fn clear(&mut self) {
        self.input.clear();
        self.pre.clear();
        for reverb in &mut self.reverbs {
            reverb.diffusers.clear();
            reverb.combs.clear();
            reverb.early.clear();
            reverb.damping.iter_mut().for_each(FirstOrderBank::clear);
            for filter in reverb.output.iter_mut().flatten() {
                filter.clear();
            }
        }
        self.phase = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Values {
        crate::Model::from_key(INFO.key).unwrap().defaults()
    }

    #[test]
    fn comb_gains_follow_decay_calc_set() {
        let effect = DualReverb::new(44_100.0);
        // DCY A 15: RT 3000 ms, first A comb 7006 samples.
        let expected = 0.008_75f32.powf(7006.0 / 44.1 / 3000.0);
        assert!((effect.gains.value[G_COMB] - expected).abs() < 1e-6);
        assert!((expected - 0.778).abs() < 2e-3);
        let mut values = defaults();
        values[4] = 0.0;
        let mut effect = DualReverb::new(44_100.0);
        effect.apply(&values, true);
        assert_eq!(effect.gains.value[G_COMB], 0.0);
    }

    #[test]
    fn ab_mix_and_size_follow_the_labels() {
        let mut values = defaults();
        let mut effect = DualReverb::new(48_000.0);
        for (ab, a, b) in [
            (0.0, 1.0, 0.0),
            (9.0, 1.0, 1.0),
            (14.0, 4.0 / 9.0, 1.0),
            (18.0, 0.0, 1.0),
        ] {
            values[3] = ab;
            effect.apply(&values, true);
            assert!((effect.gains.value[G_LEVEL] - 1.4 * a).abs() < 1e-6, "{ab}");
            assert!(
                (effect.gains.value[G_LEVEL + 1] - 0.57 * b).abs() < 1e-6,
                "{ab}"
            );
        }
        assert_eq!(size_indices(0), [0.0, 0.0]);
        assert_eq!(size_indices(12), [8.0, 4.0]);
        assert_eq!(size_indices(21), [3.0, 8.0]);
        assert_eq!(size_indices(30), [0.0, 2.0]);
        assert_eq!(size_indices(32), [0.0, 0.0]);
    }

    #[test]
    fn impulse_arrives_after_pre_delay_a() {
        let mut values = defaults();
        values[0] = 99.0; // 100 ms
        values[1] = 750.0;
        values[2] = 100.0;
        values[3] = 0.0; // A only
        let mut effect = DualReverb::new(44_100.0);
        effect.apply(&values, true);
        let first = (0..6000)
            .map(|i| effect.process([if i == 0 { 1.0 } else { 0.0 }; 2]))
            .position(|f| f[0].abs() > 1e-9)
            .unwrap();
        assert_eq!(first, 4411);
    }

    #[test]
    fn rom_lowpasses_are_redesigned_at_the_host_rate() {
        let a = rom_lowpass(0, 44_100.0);
        assert!((a.b0 - 0.214_876).abs() < 1e-5 && (-a.a1 - 0.570_248).abs() < 1e-5);
        let b = rom_lowpass(1, 44_100.0);
        assert!((b.b0 - 0.165_589).abs() < 1e-5 && (-b.a1 - 0.668_823).abs() < 1e-5);
        let at96 = rom_lowpass(0, 96_000.0);
        let db = crate::dsp::gain_to_db(at96.magnitude(96_000.0, 3750.0));
        assert!((db + 3.01).abs() < 0.05, "{db}");
    }

    #[test]
    fn tone_tilts_the_output() {
        let mut values = defaults();
        values[7] = 0.0;
        let mut effect = DualReverb::new(44_100.0);
        effect.apply(&values, true);
        let chain = &effect.reverbs[0].output[0];
        // Tone 0: +6.5 dB lows; the high shelf then cuts 6.5 dB at the top.
        let low = common::low_shelf1(44_100.0, 650.0, 6.5);
        assert!((crate::dsp::gain_to_db(low.magnitude(44_100.0, 10.0)) - 6.5).abs() < 0.1);
        let mut probe = chain[2];
        probe.clear();
        let mut peak = 0.0f32;
        for i in 0..2000 {
            let y = probe.process(if i % 2 == 0 { 1.0 } else { -1.0 });
            if i > 1000 {
                peak = peak.max(y.abs());
            }
        }
        assert!((crate::dsp::gain_to_db(peak) + 6.5).abs() < 0.2, "{peak}");
    }

    #[test]
    fn stays_bounded_at_maximum_decay() {
        let mut effect = DualReverb::new(48_000.0);
        let values = [0.0, 0.0, 100.0, 9.0, 100.0, 100.0, 16.0, 200.0, 200.0];
        effect.set(&values, 120.0);
        let mut noise = crate::dsp::Noise::new(9);
        let mut peak = 0.0f32;
        for i in 0..(48_000 * 8) {
            let x = if i < 48_000 {
                0.5 * noise.bipolar()
            } else {
                0.0
            };
            for y in effect.process([x, x]) {
                assert!(y.is_finite());
                peak = peak.max(y.abs());
            }
        }
        assert!(peak < 8.0, "{peak}");
    }
}
