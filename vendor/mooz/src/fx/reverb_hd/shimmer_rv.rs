//! ShimmerRv (SHIMMER.ZDL): pitch-shifted feed into the HD Hall network.
//!
//! Follows the recovered callback (zoom-ms70-cdr `shimmer/`):
//! - mono input `(L + R) / 2` into a 0.5 s line read by three window pairs
//!   (Q31 phase oscillators with ROM increments and depths: octave up ×0.38,
//!   octave + fifth ×0.31, two octaves ×0) through trapezoid windows that are
//!   negative as in the original's saturating fixed-point shape;
//! - a second line fed by that pitched sum plus 0.14 × its own window-pair
//!   output (another octave + fifth), mixed back at 0.343;
//! - SHIMMER_PIT_POST on the pitched path, SHIMMER_DRY_FLT on the dry mono,
//!   their sum through SHIMMER_POST_FLT into the HD Hall network of
//!   [`super::hall`] (early reflections fed from the diffuser);
//! - output `dry · DryTbl(MIX) + hall`.
//!
//! Editors (disassembly + firmware sweep `shimmer-parameters-v2`):
//! - PreD: `TBL_TO_VAL_int(100, PreD, Fx_SUB_HD_Hall_predly_tbl) + 39` ms.
//! - DECAY: `T = TBL_TO_VAL(101, DECAY, Fx_REV_Shimmer_DecayTbl)` ms, combs
//!   reach -60 dB (`0.001^(D/T)`), the low band uses `0.82·T` (the LoDMP ratio
//!   `curve(0.9, 0.1, 101, 90)` is hard-wired here); all-pass gain from
//!   HD_Hall_APF_FB_tbl.
//! - MIX: dry and wet from Fx_REV_Shimmer_DryTbl / WetTbl with the editor's
//!   count of 100 (so 100 reads one step past the tables, as the pedal does),
//!   early reflections `0.675 · wet`.
//! - HiDMP: Main design 0 (first-order low-pass) at `2000 + 100·HiDMP` Hz in
//!   every comb.
//! - LoDMP: `LowDamp_edit` designs a first-order high-pass at
//!   `curve(10, 160, 101, LoDMP)` Hz into the second comb section. The
//!   original's init never calls it (the ROM shelf stays until the knob is
//!   touched); here the knob always applies.
use super::common::{Design, Ring, comb_gain, design1, rom_biquad, scale, tbl};
use super::hall::{COMB_MS, EarlyInput, Hall, HallSettings, apf_feedback};
use crate::dsp::Biquad;
use crate::zoom::{ORIGINAL_RATE, curve};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_shimmer_rv",
    name: "ShimmerRv",
    category: Category::Reverb,
    zdl: "SHIMMER.ZDL",
    description: "Hall reverb fed by octave-up pitch shifting",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "PreD",
            max: 99,
            default: 39,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "DECAY",
            max: 100,
            default: 90,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "MIX",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "LoDMP",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "HiDMP",
            max: 100,
            default: 74,
            display: Display::INT,
        },
    ],
};

const PREDELAY_TBL: [i32; 9] = [70, 73, 76, 81, 98, 123, 162, 202, 252];
const DECAY_TBL: [f32; 11] = [
    694.13, 742.24, 762.29, 898.3, 1028.62, 1276.37, 1832.18, 2230.75, 4387.12, 8343.54, 66540.02,
];
const DRY_TBL: [f32; 11] = [
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    0.251_188_6,
    0.003_981_1,
];
const WET_TBL: [f32; 11] = [
    0.0,
    0.008_798_3,
    0.049_739_3,
    0.121_688_6,
    0.235_450_7,
    0.374_498_4,
    0.546_135,
    0.760_063_7,
    1.0,
    1.0,
    1.0,
];

const DRY_FLT: [[f32; 5]; 3] = [
    [
        0.950_938_5,
        -1.757_807,
        0.824_861_7,
        1.757_807,
        -0.775_800_3,
    ],
    [0.338_700_6, -0.236_004_9, 0.0, 0.817_378_4, 0.0],
    [1.001_539_3, -0.997_037, 0.0, 0.998_576_3, 0.0],
];
const PIT_POST: [[f32; 5]; 3] = [
    [0.136_309_5, 0.272_619, 0.136_309_5, 0.391_227, 0.063_535],
    [
        0.755_784_6,
        -1.233_837_2,
        0.529_897_4,
        1.233_837_2,
        -0.285_682,
    ],
    [
        0.969_119_1,
        -1.852_093_1,
        0.889_762_1,
        1.852_093_1,
        -0.858_881_1,
    ],
];
const POST_FLT: [[f32; 5]; 3] = [
    [2.097_608, -3.458_940_5, 1.396_739, 1.733_577, -0.751_322_2],
    [
        1.002_826_8,
        -1.982_758_8,
        0.980_221_8,
        1.982_758_8,
        -0.983_048_6,
    ],
    [
        0.991_444_4,
        -1.939_637_5,
        0.949_966_4,
        1.939_637_5,
        -0.941_410_8,
    ],
];

/// Window oscillators c[9,11,13,15] (Q32 phase per 44.1 kHz sample), depths
/// c[17..21] (Q31 of an 8192-sample span), base offsets c[22..25], bank gains
/// c[26..29].
const INCREMENTS: [i32; 4] = [-990_765, -1_698_622, -2_547_934, -1_698_622];
const DEPTHS: [f32; 4] = [0.529_2, 0.617_4, 0.617_4, 0.617_4];
const BASES: [u32; 4] = [3384, 7128, 1, 1];
const BANK_GAINS: [f32; 3] = [0.38, 0.31, 0.0];
/// c[29] (second-line window gain), c[32] (its recirculation), c[34] (its
/// share of the wet input).
const LINE_GAIN: f32 = 0.343;
const LINE_FEEDBACK: f32 = 0.140_608;
const LINE_MIX: f32 = 0.343;
const LINE_LENGTH: u32 = 22050;

/// The pitch window: `sat(1.6129 · (|x| + 0.07 - 1))` of the Q31 phase `x`.
#[inline]
fn window(phase: u32) -> f32 {
    let a = (phase as i32).unsigned_abs() as f32 * (1.0 / 2_147_483_648.0);
    -(1.612_903 * (0.93 - a).max(0.0)).min(1.0)
}

/// Delay position (0..1 of the depth) of a Q31 phase.
#[inline]
fn position(phase: u32) -> f32 {
    ((phase as i32 >> 1) as f32 + 1_073_741_824.0) * (1.0 / 2_147_483_648.0)
}

/// `TBL_TO_VAL_int(100, value, table)`: integer steps of `table[i+1]-table[i]`
/// divided by the integer step 12.
fn predelay_ms(value: f32) -> f32 {
    let at = |v: i32| -> f32 {
        let v = v.clamp(0, 99);
        if v == 99 {
            return PREDELAY_TBL[8] as f32;
        }
        let step = 99 / 8;
        let i = (v / step) as usize;
        let next = if i + 1 < 9 { PREDELAY_TBL[i + 1] } else { 0 };
        (PREDELAY_TBL[i] + (v % step) * ((next - PREDELAY_TBL[i]) / step)) as f32
    };
    let low = value.floor();
    let fraction = value - low;
    let a = at(low as i32);
    let b = at(low as i32 + 1);
    a + (b - a) * fraction + 39.0
}

/// Editor results for the hall stage.
pub fn settings(values: &Values, sample_rate: f32) -> (HallSettings, f32) {
    let [predelay, decay, mix, lodmp, hidmp, ..] = *values;
    let time = tbl(&DECAY_TBL, 101, decay, 0.0);
    let ratio = curve(0.9, 0.1, 101, 90.0, 0);
    let wet = tbl(&WET_TBL, 100, mix, 0.0);
    let dry = tbl(&DRY_TBL, 100, mix, WET_TBL[0]);
    let hall = HallSettings {
        predelay: (predelay_ms(predelay) * 44.1).floor(),
        allpass: apf_feedback(decay),
        low: COMB_MS.map(|d| comb_gain(0.001, d, time * ratio)),
        high: COMB_MS.map(|d| comb_gain(0.001, d, time)),
        wet,
        early: 0.675 * wet,
        dry: 0.0,
        damp: design1(
            Design::LowPass1,
            sample_rate,
            curve(12_000.0, 2000.0, 101, hidmp, 0),
            0.0,
        ),
        lowcut: design1(
            Design::HighPass1,
            sample_rate,
            curve(10.0, 160.0, 101, lodmp, 0),
            0.0,
        ),
    };
    (hall, dry)
}

struct Cascade([Biquad; 3]);

impl Cascade {
    fn new(rows: &[[f32; 5]; 3], sample_rate: f32) -> Self {
        Self(rows.map(|row| Biquad::new(rom_biquad(row).resample(ORIGINAL_RATE, sample_rate))))
    }
    #[inline]
    fn process(&mut self, x: f32) -> f32 {
        let [a, b, c] = &mut self.0;
        c.process(b.process(a.process(x)))
    }
    fn clear(&mut self) {
        self.0.iter_mut().for_each(Biquad::clear);
    }
}

struct Shimmer {
    sample_rate: f32,
    input_line: Ring,
    second_line: Ring,
    phases: [u32; 4],
    increments: [u32; 4],
    depths: [f32; 4],
    bases: [f32; 4],
    feedback: f32,
    dry_filter: Cascade,
    wet_filter: Cascade,
    post_filter: Cascade,
    hall: Hall,
    dry: crate::dsp::Smoother,
    values: Option<Values>,
}

impl Shimmer {
    /// The window pair of `bank` read from `line`: (phase, twin) outputs.
    #[inline]
    fn pair(line: &Ring, phase: u32, depth: f32, base: f32) -> (f32, f32) {
        let twin = phase.wrapping_add(0x8000_0000);
        let a = line.read_frac(base + position(phase) * depth) * window(phase);
        let b = line.read_frac(base + position(twin) * depth) * window(twin);
        (a, b)
    }
}

impl Effect for Shimmer {
    fn set(&mut self, values: &Values, _bpm: f32) {
        if self.values.as_ref() == Some(values) {
            return;
        }
        let first = self.values.is_none();
        self.values = Some(*values);
        let (hall, dry) = settings(values, self.sample_rate);
        self.hall.apply(&hall);
        if first {
            self.dry.jump(dry);
        } else {
            self.dry.set(dry);
        }
    }

    #[inline]
    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        for (phase, &increment) in self.phases.iter_mut().zip(&self.increments) {
            *phase = phase.wrapping_add(increment);
        }
        let mono = 0.5 * (frame[0] + frame[1]);
        self.input_line.advance();
        self.input_line.write(0, mono);
        let mut pitched = 0.0;
        for (bank, gain) in BANK_GAINS.iter().enumerate() {
            let (a, b) = Self::pair(
                &self.input_line,
                self.phases[bank],
                self.depths[bank],
                self.bases[bank],
            );
            pitched += gain * (a + b);
        }
        self.second_line.advance();
        self.second_line
            .write(0, self.feedback * LINE_FEEDBACK + pitched);
        let (a, b) = Self::pair(
            &self.second_line,
            self.phases[3],
            self.depths[3],
            self.bases[3],
        );
        self.feedback = crate::dsp::flush(LINE_GAIN * (a + b));
        let wet = self.wet_filter.process(self.feedback * LINE_MIX + pitched);
        let dry = self.dry_filter.process(mono);
        let post = self.post_filter.process(dry + wet);
        let hall = self.hall.process(post, post, [0.0; 2]);
        let gain = self.dry.tick();
        [gain * frame[0] + hall[0], gain * frame[1] + hall[1]]
    }

    fn clear(&mut self) {
        self.input_line.clear();
        self.second_line.clear();
        self.feedback = 0.0;
        self.dry_filter.clear();
        self.wet_filter.clear();
        self.post_filter.clear();
        self.hall.clear();
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let ratio = sample_rate / ORIGINAL_RATE;
    let span = (8192.0 * ratio) as usize;
    let mut effect = Shimmer {
        sample_rate,
        input_line: Ring::new(scale(LINE_LENGTH, sample_rate)),
        second_line: Ring::new(span + scale(1, sample_rate) + 4),
        phases: [0; 4],
        increments: INCREMENTS.map(|i| (i as f32 / ratio).round() as i32 as u32),
        depths: DEPTHS.map(|d| d * 8192.0 * ratio),
        bases: BASES.map(|b| b as f32 * ratio),
        feedback: 0.0,
        dry_filter: Cascade::new(&DRY_FLT, sample_rate),
        wet_filter: Cascade::new(&PIT_POST, sample_rate),
        post_filter: Cascade::new(&POST_FLT, sample_rate),
        hall: Hall::new(sample_rate, EarlyInput::Shimmer),
        dry: crate::dsp::Smoother::new(1.0, 0.01, sample_rate),
        values: None,
    };
    effect.set(&crate::Model::ShimmerRv.defaults(), 120.0);
    Box::new(effect)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    #[test]
    fn editors_match_the_firmware_sweep() {
        let mut v = Model::ShimmerRv.defaults();
        let (s, dry) = settings(&v, 44_100.0);
        // DECAY 90: comb 1 low/high c[87], c[88]; HiDMP 74 low-pass.
        assert!((s.low[0] - 0.824_615_4).abs() < 3e-4, "{}", s.low[0]);
        assert!((s.high[0] - 0.853_741).abs() < 3e-4, "{}", s.high[0]);
        assert_eq!(dry, 1.0);
        assert!((s.wet - 0.383_166_9).abs() < 1e-4, "{}", s.wet);
        // PreD 39 -> 5424 samples (c[45]).
        assert_eq!(s.predelay, 5424.0);
        for (pred, expected) in [(0.0, 4806.0), (42.0, 5556.0), (96.0, 12833.0)] {
            v[0] = pred;
            assert_eq!(settings(&v, 44_100.0).0.predelay, expected, "{pred}");
        }
        // MIX 90 and 100: the count-100 table reads.
        v[2] = 90.0;
        assert!((settings(&v, 44_100.0).1 - 0.228_715_2).abs() < 1e-4);
        v[2] = 100.0;
        let (s, dry) = settings(&v, 44_100.0);
        assert!((dry - 0.003_579).abs() < 1e-5 && (s.wet - 0.898_99).abs() < 1e-4);
        // HiDMP 0 and 100 (c[129..132]).
        v[4] = 50.0;
        let d = settings(&v, 44_100.0).0.damp;
        assert!((d.b0 - 0.352_568_4).abs() < 3e-3, "{d:?}");
    }

    #[test]
    fn pitch_banks_shift_by_the_rom_ratios() {
        // Rate of change of the read position = 1 - pitch ratio.
        for (bank, ratio) in [(0, 2.0), (1, 3.0), (2, 4.0)] {
            let span = DEPTHS[bank] * 8192.0;
            let slope = INCREMENTS[bank].unsigned_abs() as f32 / 4_294_967_296.0 * span;
            assert!((1.0 + slope - ratio).abs() < 2e-3, "{bank}: {slope}");
        }
    }

    #[test]
    fn octave_up_appears_in_the_output() {
        // Feed 300 Hz; the tail must carry far more 600 Hz (octave up) than
        // 450 Hz (no partial there). Magnitudes of short DFT blocks, since
        // each window grain restarts the shifted phase.
        let fs = 48_000.0;
        let mut effect = create(fs);
        let mut v = Model::ShimmerRv.defaults();
        v[2] = 100.0;
        effect.set(&v, 120.0);
        let block = 2048;
        let (mut e600, mut e450) = (0.0f64, 0.0f64);
        let mut acc = [(0.0f64, 0.0f64); 2];
        for i in 0..48_000 * 2 {
            let t = i as f32 / fs;
            let x = 0.3 * (t * 300.0 * std::f32::consts::TAU).sin();
            let [l, _] = effect.process([x, x]);
            if i >= 48_000 {
                for (a, hz) in acc.iter_mut().zip([600.0f32, 450.0]) {
                    let (s, c) = (t * hz * std::f32::consts::TAU).sin_cos();
                    a.0 += f64::from(l * s);
                    a.1 += f64::from(l * c);
                }
                if (i + 1) % block == 0 {
                    e600 += acc[0].0.hypot(acc[0].1);
                    e450 += acc[1].0.hypot(acc[1].1);
                    acc = [(0.0, 0.0); 2];
                }
            }
        }
        assert!(e600 > 4.0 * e450, "600 Hz {e600} vs 450 Hz {e450}");
    }
}
