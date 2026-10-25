//! DynaRev (DYNAREV.ZDL, `Fx_REV_DynaRev`).
//!
//! Follows the recovered callback, the Ambience network with a dynamic wet
//! level: the stereo input runs through the ROM low shelf C52..56 and the
//! 5 kHz low-pass C57..61 into a mono injection; two pre-delay taps
//! (44 x PreD-table ms - 1 and + 132 samples) are swept together by a
//! 0.347 Hz, 43.5-sample LFO (C150..155, parabolic sine, linear
//! interpolation) and start two 4-stage lossy all-pass chains
//! (w = x + C63 d, y = -0.84 x + C65 d); their sum (x0.45) feeds four two-band
//! damped combs (52.8/91.7/77.2/67.8 ms; a second LFO on combs C/D has zero
//! depth in the ROM and no editor sets it). The outputs sum four comb taps
//! (x0.87 left, x1 right, x0.75), the diffused signal (C114) and the
//! pre-delayed input (C113) through the ROM low peak C118..122, the ROM
//! 15 kHz low-pass C128..132 and the Color shelf C123..127.
//! The detector follows the original exactly: the mono input (mean of L/R)
//! through the 80 Hz high-pass C6..8 and, for Sense >= 81, a +20 dB treble
//! shelf C9..11, scaled by C12; the level rises linearly by C14 per sample
//! (attack time from the Sense table) and falls by C13; a smoothed envelope
//! (C15/C16) is squared and scaled by 1.5 (clamped to 1). Sense below 81 makes
//! the reverb swell with the playing (wet = A x env), 81..100 is static (wet
//! = A, faded in quadratically from 81, 100 shows OFF), above 100 the reverb
//! ducks (wet = A (1 - env) + 0.003).
//!
//! Editors decoded: `decay_set` (T60 table 400..3750 ms, high band at
//! Color-table x T60, low band at 0.7 T60, `g^(d/T60)` per comb, Decay
//! tables for C63/C113/C114), `tone_edit` (type-8 shelf at 3 kHz,
//! -14..0 dB table), `HiDamp_edit` (type-0 low-pass at table(84) = 13.56 kHz
//! inside the combs), `sens_edit`, `amount_edit` (dry and wet tables, the
//! Sense modes), `predelay_edit` (integer ms table x 44 samples), `onf_aft`,
//! `Level_edit` (fixed 1.0). The early loop of the network (C88/C89) is
//! never opened by any editor, so it is left out.
//! Approximated: PreD changes crossfade the taps over 16 ms (the original
//! mutes, jumps and reopens); ROM filters are re-derived for the host rate.
use super::fast::Ramp;
use super::fast::{Bank, Biquad, flush};
use super::tc_common::{
    CombBank, Design, FirstOrder, Glide, Line, TapFade, decay_gain, design, diffuse, one_pole_rom,
    rom_biquad, samples, table, table_int,
};
use crate::zoom::{DEFAULT_STEP, ORIGINAL_RATE};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_dyna_rev",
    name: "DynaRev",
    category: Category::Reverb,
    zdl: "DYNAREV.ZDL",
    description: "Reverb that swells with or ducks under your playing",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Decay",
            max: 100,
            default: 82,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "PreD",
            max: 100,
            default: 0,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Color",
            max: 100,
            default: 76,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        // GetString_AwaSens: -100..-1, OFF at 100, 1..100.
        ParamInfo {
            name: "Sense",
            max: 200,
            default: 172,
            display: Display::labels(&SENSE_LABELS),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 40,
            display: Display::INT,
        },
    ],
};

/// GetString_AwaSens: -100..-1, OFF at 100, then 1..100.
const SENSE_LABELS: [&str; 201] = [
    "-100", "-99", "-98", "-97", "-96", "-95", "-94", "-93", "-92", "-91", "-90", "-89", "-88",
    "-87", "-86", "-85", "-84", "-83", "-82", "-81", "-80", "-79", "-78", "-77", "-76", "-75",
    "-74", "-73", "-72", "-71", "-70", "-69", "-68", "-67", "-66", "-65", "-64", "-63", "-62",
    "-61", "-60", "-59", "-58", "-57", "-56", "-55", "-54", "-53", "-52", "-51", "-50", "-49",
    "-48", "-47", "-46", "-45", "-44", "-43", "-42", "-41", "-40", "-39", "-38", "-37", "-36",
    "-35", "-34", "-33", "-32", "-31", "-30", "-29", "-28", "-27", "-26", "-25", "-24", "-23",
    "-22", "-21", "-20", "-19", "-18", "-17", "-16", "-15", "-14", "-13", "-12", "-11", "-10",
    "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1", "OFF", "1", "2", "3", "4", "5", "6", "7",
    "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23",
    "24", "25", "26", "27", "28", "29", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39",
    "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "50", "51", "52", "53", "54", "55",
    "56", "57", "58", "59", "60", "61", "62", "63", "64", "65", "66", "67", "68", "69", "70", "71",
    "72", "73", "74", "75", "76", "77", "78", "79", "80", "81", "82", "83", "84", "85", "86", "87",
    "88", "89", "90", "91", "92", "93", "94", "95", "96", "97", "98", "99", "100",
];

/// Detector filters: C6..8 high-pass and the C9..11 treble boost (Sense >= 81).
const DETECTOR_HP: (f32, f32, f32) = (0.994_333_2, -0.994_333_2, 0.988_666_4);
const DETECTOR_BOOST: (f32, f32, f32) = (9.936_338, -9.922_192, 0.985_853);
/// C15/C16 envelope smoothing and C17 envelope scale.
const ENVELOPE: (f32, f32) = (0.012_784_057, 0.999_757_2);
const ENVELOPE_SCALE: f32 = 1.5;
/// `sens_edit` tables (201 steps): C12 gain, C13 fall, attack ms (C14).
const SENSE_GAIN: [f32; 11] = [0.8, 0.6, 0.18, 0.03, 0.009, 0.0, 0.05, 0.2, 0.4, 0.6, 0.8];
const SENSE_FALL: [f32; 11] = [
    0.99981, 0.99981, 0.99981, 0.99981, 0.99981, 0.994, 0.994, 0.994, 0.994, 0.994, 0.994,
];
const SENSE_ATTACK_MS: [f32; 11] = [
    5000.0, 5000.0, 2500.0, 50.0, 50.0, 1500.0, 1500.0, 1500.0, 1500.0, 1500.0, 1500.0,
];
/// Input sections C52..56 and C57..61, injection gain C116 = C117.
const INPUT_SHELF: (f32, f32, f32) = (0.998_829, -0.993_152, 0.991_982);
const INPUT_LP: (f32, f32, f32) = (0.271_168, 0.271_168, 0.457_663);
const INJECTION: f32 = 0.502_641;
/// Pre-delay LFO C153 (phase step of a 2^32 cycle at 44.1 kHz) and depth.
const LFO_HZ: f32 = 33_795.0 / 4_294_967_296.0 * 44_100.0;
const LFO_DEPTH: f32 = 730_144_440.0 / 16_777_216.0;
/// The second pre-delay tap reads 132 samples further (C157 = C156 + 132).
const SECOND_TAP: f32 = 132.0;
/// All-pass C62 = 1, C64 = -0.84, C65 = 0.84 (table constant).
const DIFFUSER_OUT: (f32, f32) = (-0.84, 0.84);
const CHAINS: [[f32; 4]; 2] = [
    [2777.0, 2281.0, 1847.0, 1471.0],
    [1153.0, 887.0, 1759.0, 1447.0],
];
/// Combs C, D, A, B (ring regions C83..86) and their output taps.
const COMB: [f32; 4] = [2324.0, 4043.0, 3403.0, 2988.0];
const OUTPUT_TAPS: [[f32; 4]; 2] = [
    [1111.0, 2143.0, 1820.0, 1350.0],
    [1420.0, 2452.0, 2216.0, 1659.0],
];
const TAP_GAIN: [f32; 2] = [0.87, 1.0];
const TAP_SUM: f32 = 0.75;
const COMB_INPUT: f32 = 0.45;
/// C87 crossover, C93..95 fixed shelf, `HiDamp_edit` low-pass corner.
const CROSSOVER: f32 = 0.247_016;
const LOOP_SHELF: (f32, f32, f32) = (0.999_918, -0.985_772, 0.985_69);
const DAMP_HZ: [f32; 11] = [
    3000.0, 4200.0, 5400.0, 6600.0, 7800.0, 9000.0, 10_200.0, 11_400.0, 12_600.0, 15_000.0,
    22_000.0,
];
/// Output C118..122 (low peak) and C128..132 (first-order low-pass).
const OUTPUT_EQ: [f32; 5] = [1.002_324, -1.943_806, 0.942_27, 1.943_806, -0.944_595];
const OUTPUT_LP: (f32, f32, f32) = (0.645_495, 0.645_495, -0.290_99);
/// `decay_set` tables.
const T60_MS: [f32; 11] = [
    400.0, 450.0, 900.0, 1550.0, 1900.0, 2200.0, 2850.0, 3100.0, 3500.0, 3670.0, 3750.0,
];
const COLOR_DECAY: [f32; 11] = [
    0.1, 0.33, 0.44, 0.54, 0.62, 0.69, 0.76, 0.83, 0.89, 0.94, 1.0,
];
const COMB_DECAY: [(f32, f32); 4] = [
    (52.75, 0.001_238_05),
    (91.73, 0.001_303_21),
    (77.17, 0.001_371_8),
    (67.75, 0.001_444),
];
const DIFFUSION: [f32; 11] = [
    0.53, 0.53, 0.5, 0.5, 0.47, 0.47, 0.47, 0.47, 0.47, 0.47, 0.47,
];
const DIRECT: [f32; 11] = [
    0.67, 0.67, 0.57, 0.57, 0.47, 0.47, 0.47, 0.47, 0.47, 0.47, 0.47,
];
const DIFFUSED: [f32; 11] = [
    0.77, 0.77, 0.97, 0.97, 1.17, 1.17, 1.17, 1.17, 1.17, 1.17, 1.17,
];
/// `tone_edit` shelf gain in dB.
const COLOR_DB: [f32; 11] = [
    -14.0, -11.7, -9.2, -6.5, -4.8, -3.5, -2.8, -2.2, -1.2, -0.8, 0.0,
];
/// `amount_edit` dry and wet tables.
const DRY: [f32; 11] = [
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    0.779_675_1,
    0.489_796,
    0.265_306_1,
    0.118_700_6,
    0.0,
];
const WET: [f32; 11] = [
    0.0,
    0.006_339_5,
    0.039_622_3,
    0.069_037_9,
    0.101_433_2,
    0.158_489_3,
    0.158_489_3,
    0.158_489_3,
    0.158_489_3,
    0.158_489_3,
    0.158_489_3,
];
/// `predelay_edit` integer table (ms, 44 samples each).
const PREDELAY_MS: [i32; 11] = [8, 9, 11, 14, 16, 27, 57, 82, 115, 145, 149];
const GAIN_STEP: f32 = 0.001_890_359;

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(DynaRev::new(sample_rate))
}

/// The detector of the callback's front end.
#[derive(Debug, Clone)]
struct Detector {
    highpass: FirstOrder,
    boost: FirstOrder,
    boost_on: bool,
    gain: f32,
    fall: f32,
    rise: f32,
    smooth_in: f32,
    smooth_keep: f32,
    level: f32,
    envelope: f32,
}

impl Detector {
    fn new(sample_rate: f32) -> Self {
        let ratio = ORIGINAL_RATE / sample_rate;
        let smooth_keep = ENVELOPE.1.powf(ratio);
        Self {
            highpass: FirstOrder::rom(DETECTOR_HP.0, DETECTOR_HP.1, DETECTOR_HP.2, sample_rate),
            boost: FirstOrder::rom(
                DETECTOR_BOOST.0,
                DETECTOR_BOOST.1,
                DETECTOR_BOOST.2,
                sample_rate,
            ),
            boost_on: false,
            gain: 1.0,
            fall: 1.0,
            rise: 0.0,
            // Keep the steady-state gain C15 / (1 - C16) of the original.
            smooth_in: ENVELOPE.0 * (1.0 - smooth_keep) / (1.0 - ENVELOPE.1),
            smooth_keep,
            level: 0.0,
            envelope: 0.0,
        }
    }

    /// Returns the clamped `min(1.5 env^2, 1)` scale.
    #[inline]
    fn process(&mut self, x: f32) -> f32 {
        let x = self.highpass.process(x);
        let x = if self.boost_on {
            self.boost.process(x)
        } else {
            x
        };
        let incoming = (x * self.gain).abs().min(1.0);
        self.level = if self.level < incoming {
            self.level + self.rise
        } else {
            flush(self.level * self.fall)
        };
        self.envelope = flush(self.smooth_keep * self.envelope + self.smooth_in * self.level);
        let e = self.envelope.min(1.0);
        (e * e * ENVELOPE_SCALE).min(1.0)
    }

    fn clear(&mut self) {
        self.highpass.clear();
        self.boost.clear();
        self.level = 0.0;
        self.envelope = 0.0;
    }
}

struct DynaRev {
    sample_rate: f32,
    detector: Detector,
    input_shelf: [FirstOrder; 2],
    input_lp: [FirstOrder; 2],
    injection: Line,
    predelay: TapFade,
    second_tap: f32,
    lfo_phase: f32,
    lfo_step: f32,
    lfo_depth: f32,
    /// Chain 0 in regions 0..4, chain 1 in 4..8.
    chains: Bank<8>,
    combs: CombBank<4>,
    output_taps: [[usize; 4]; 2],
    output_eq: [Biquad; 2],
    output_lp: [FirstOrder; 2],
    output_shelf: [FirstOrder; 2],
    crossover: f32,
    dry: Glide,
    wet_scale: Ramp,
    wet_offset: Ramp,
    diffusion: Ramp,
    direct: Ramp,
    diffused: Ramp,
    comb_low: [Ramp; 4],
    comb_high: [Ramp; 4],
    last: [f32; 5],
}

impl DynaRev {
    fn new(sample_rate: f32) -> Self {
        let rom1 = |c: (f32, f32, f32)| FirstOrder::rom(c.0, c.1, c.2, sample_rate);
        let damp = table(&DAMP_HZ, 101, 84.0);
        let lowpass = design(Design::LowPass, damp, 0.0, sample_rate);
        let tone = FirstOrder::new(lowpass.0, lowpass.1, lowpass.2);
        let shelf = rom1(LOOP_SHELF);
        let ramp = |v: f32| Ramp::new(v, GAIN_STEP, sample_rate);
        let max_predelay = samples(149.0 * 44.0 + SECOND_TAP + LFO_DEPTH + 4.0, sample_rate);
        let mut effect = Self {
            sample_rate,
            detector: Detector::new(sample_rate),
            input_shelf: [rom1(INPUT_SHELF); 2],
            input_lp: [rom1(INPUT_LP); 2],
            injection: Line::new(max_predelay),
            predelay: TapFade::new(1.0, sample_rate),
            second_tap: SECOND_TAP * sample_rate / ORIGINAL_RATE,
            lfo_phase: 0.0,
            lfo_step: LFO_HZ / sample_rate,
            lfo_depth: LFO_DEPTH * sample_rate / ORIGINAL_RATE,
            chains: Bank::new(
                std::array::from_fn(|i| samples(CHAINS[i / 4][i % 4], sample_rate)),
                0,
            ),
            combs: CombBank::new(
                Bank::new(COMB.map(|d| samples(d, sample_rate)), 0),
                tone,
                shelf,
            ),
            output_taps: OUTPUT_TAPS.map(|side| side.map(|d| samples(d, sample_rate))),
            output_eq: [rom_biquad(OUTPUT_EQ, sample_rate); 2],
            output_lp: [rom1(OUTPUT_LP); 2],
            output_shelf: [FirstOrder::IDENTITY; 2],
            crossover: one_pole_rom(CROSSOVER, sample_rate),
            dry: Glide::new(1.0, 0.016, sample_rate),
            wet_scale: Ramp::new(0.0, DEFAULT_STEP, sample_rate),
            wet_offset: Ramp::new(0.0, DEFAULT_STEP, sample_rate),
            diffusion: ramp(0.47),
            direct: ramp(0.47),
            diffused: ramp(1.17),
            comb_low: [ramp(0.0); 4],
            comb_high: [ramp(0.0); 4],
            last: [f32::NAN; 5],
        };
        let mut defaults = [0.0; crate::MAX_PARAMS];
        for (value, param) in defaults.iter_mut().zip(INFO.params) {
            *value = f32::from(param.default);
        }
        effect.set(&defaults, 120.0);
        effect.settle();
        effect
    }

    fn settle(&mut self) {
        let jump = |r: &mut Ramp| r.jump(r.target);
        self.dry.current = table(&DRY, 101, self.last[4].clamp(0.0, 100.0));
        jump(&mut self.wet_scale);
        jump(&mut self.wet_offset);
        jump(&mut self.diffusion);
        jump(&mut self.direct);
        jump(&mut self.diffused);
        self.comb_low.iter_mut().for_each(jump);
        self.comb_high.iter_mut().for_each(jump);
        self.predelay.jump();
    }

    /// `decay_set` (Decay and Color).
    fn set_decay(&mut self, decay: f32, color: f32) {
        let t60 = table(&T60_MS, 101, decay);
        let t60_high = table(&COLOR_DECAY, 101, color) * t60;
        let t60_low = 0.7 * t60;
        for (i, &(ms, g)) in COMB_DECAY.iter().enumerate() {
            self.comb_high[i].set(decay_gain(g, ms, t60_high));
            self.comb_low[i].set(decay_gain(g, ms, t60_low));
        }
        self.diffusion.set(table(&DIFFUSION, 101, decay));
        self.direct.set(table(&DIRECT, 101, decay));
        self.diffused.set(table(&DIFFUSED, 101, decay));
    }

    /// `tone_edit`: the 3 kHz output shelf.
    fn set_color(&mut self, color: f32) {
        let gain = table(&COLOR_DB, 101, color);
        let shelf = design(Design::HighShelf, 3000.0, gain, self.sample_rate);
        self.output_shelf.iter_mut().for_each(|f| f.set(shelf));
    }

    /// `sens_edit` and the Sense branches of `amount_edit`.
    fn set_sense_and_mix(&mut self, sense: f32, mix: f32) {
        let ratio = ORIGINAL_RATE / self.sample_rate;
        let d = &mut self.detector;
        d.gain = table(&SENSE_GAIN, 201, sense);
        d.fall = table(&SENSE_FALL, 201, sense).powf(ratio);
        d.rise = 1000.0 / (table(&SENSE_ATTACK_MS, 201, sense) * self.sample_rate);
        let step = sense.round();
        d.boost_on = step >= 81.0;
        self.dry.set(table(&DRY, 101, mix));
        let mut wet = table(&WET, 101, mix);
        let (scale, offset) = if step < 81.0 {
            (wet, 0.0)
        } else if step < 101.0 {
            let t = (sense - 81.0).clamp(0.0, 19.0) / 19.0;
            wet *= 0.01 + 0.99 * t * t;
            (0.0, wet)
        } else {
            (-wet, wet + 0.003)
        };
        self.wet_scale.set(scale);
        self.wet_offset.set(offset);
    }

    /// The parabolic LFO of the callback: a triangle phase `p` shaped to
    /// `0.5 + 2 x (1 - |x|)`, `x = p - 0.5`, from 0 to 1.
    #[inline]
    fn lfo(&mut self) -> f32 {
        let phase = self.lfo_phase;
        self.lfo_phase += self.lfo_step;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }
        let p = if phase < 0.5 {
            2.0 * phase
        } else {
            2.0 - 2.0 * phase
        };
        let x = p - 0.5;
        0.5 + 2.0 * x * (1.0 - x.abs())
    }
}

/// `predelay_edit`: 44 samples per table millisecond, the first tap one less.
fn predelay_samples(value: f32, sample_rate: f32) -> f32 {
    let ms = table_int(&PREDELAY_MS, 101, value.clamp(0.0, 100.0));
    ((44.0 * ms - 1.0) * sample_rate / ORIGINAL_RATE).max(1.0)
}

impl Effect for DynaRev {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let [decay, predelay, color, sense, mix] = [
            values[0].clamp(0.0, 100.0),
            values[1],
            values[2].clamp(0.0, 100.0),
            values[3].clamp(0.0, 200.0),
            values[4].clamp(0.0, 100.0),
        ];
        if decay != self.last[0] || color != self.last[2] {
            self.set_decay(decay, color);
        }
        if color != self.last[2] {
            self.set_color(color);
        }
        if sense != self.last[3] || mix != self.last[4] {
            self.last[3] = sense;
            self.last[4] = mix;
            self.set_sense_and_mix(sense, mix);
        }
        self.last[0] = decay;
        self.last[1] = predelay;
        self.last[2] = color;
        self.predelay
            .set(predelay_samples(predelay, self.sample_rate));
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        let scale = self.detector.process(0.5 * (frame[0] + frame[1]));
        let wet = self.wet_offset.tick() + self.wet_scale.tick() * scale;

        let mut injection = 0.0;
        for (c, &x) in frame.iter().enumerate() {
            injection += self.input_lp[c].process(self.input_shelf[c].process(x));
        }
        let injection = injection * INJECTION;

        self.predelay.tick();
        let sweep = self.lfo() * self.lfo_depth;
        let pre = [
            self.predelay.read(&self.injection, sweep),
            self.predelay.read(&self.injection, sweep + self.second_tap),
        ];
        self.injection.write(injection);

        let g = self.diffusion.tick();
        let (c, h) = DIFFUSER_OUT;
        let mut diffused = pre;
        let mut stages = self.chains.frame();
        for (chain, x) in diffused.iter_mut().enumerate() {
            for stage in 0..4 {
                *x = diffuse(&mut stages, chain * 4 + stage, *x, g, c, h);
            }
        }
        self.chains.advance();

        let mut sums = [0.0; 2];
        for (side, sum) in sums.iter_mut().enumerate() {
            for (i, &tap) in self.output_taps[side].iter().enumerate() {
                *sum += self.combs.lines.tap(i, tap);
            }
        }
        let input = COMB_INPUT * (diffused[0] + diffused[1]);
        let mut low = [0.0; 4];
        let mut high = [0.0; 4];
        for i in 0..4 {
            low[i] = self.comb_low[i].tick();
            high[i] = self.comb_high[i].tick();
        }
        self.combs.run([input; 4], self.crossover, low, high);

        let direct = self.direct.tick();
        let diffused_gain = self.diffused.tick();
        let dry = self.dry.tick();
        let mut out = [0.0; 2];
        for side in 0..2 {
            let x = TAP_SUM * TAP_GAIN[side] * sums[side]
                + diffused_gain * diffused[side]
                + direct * pre[side];
            let x = self.output_eq[side].process(x);
            let x = self.output_shelf[side].process(self.output_lp[side].process(x));
            out[side] = wet * x + dry * frame[side];
        }
        out
    }

    fn clear(&mut self) {
        self.detector.clear();
        for side in 0..2 {
            self.input_shelf[side].clear();
            self.input_lp[side].clear();
            self.output_eq[side].clear();
            self.output_lp[side].clear();
            self.output_shelf[side].clear();
        }
        self.injection.clear();
        self.chains.clear();
        self.combs.clear();
        self.lfo_phase = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Values {
        let mut values = [0.0; crate::MAX_PARAMS];
        for (value, param) in values.iter_mut().zip(INFO.params) {
            *value = f32::from(param.default);
        }
        values
    }

    fn with(sense: f32, mix: f32) -> DynaRev {
        let mut effect = DynaRev::new(48_000.0);
        let mut values = defaults();
        values[3] = sense;
        values[4] = mix;
        effect.set(&values, 120.0);
        effect.settle();
        effect
    }

    #[test]
    fn sense_selects_swell_static_and_duck() {
        let swell = with(40.0, 100.0);
        assert!((swell.wet_scale.current - 0.158_489_3).abs() < 1e-6);
        assert_eq!(swell.wet_offset.current, 0.0);
        let still = with(100.0, 100.0);
        assert_eq!(still.wet_scale.current, 0.0);
        assert!((still.wet_offset.current - 0.158_489_3).abs() < 1e-6);
        let faded = with(81.0, 100.0);
        assert!((faded.wet_offset.current - 0.001_584_893).abs() < 1e-7);
        let duck = with(172.0, 100.0);
        assert!((duck.wet_scale.current + 0.158_489_3).abs() < 1e-6);
        assert!((duck.wet_offset.current - 0.161_489_3).abs() < 1e-6);
        assert!(duck.detector.boost_on && !swell.detector.boost_on);
    }

    #[test]
    fn detector_rises_with_the_attack_time() {
        // Sense 60: 50 ms attack, gain 0.03, so a full-scale input saturates
        // the envelope within a few tens of milliseconds.
        let mut effect = with(60.0, 50.0);
        let mut scale = 0.0;
        for i in 0..4800 {
            let x = if i % 2 == 0 { 1.0 } else { -1.0 };
            scale = effect.detector.process(x);
        }
        assert!(scale > 0.9, "{scale}");
        for _ in 0..48_000 {
            scale = effect.detector.process(0.0);
        }
        assert!(scale < 1e-3, "{scale}");
    }

    #[test]
    fn mix_tables_and_predelay() {
        assert!((table(&DRY, 101, 70.0) - 0.489_796).abs() < 1e-6);
        assert!((table(&WET, 101, 40.0) - 0.101_433_2).abs() < 1e-6);
        assert_eq!(predelay_samples(0.0, 44_100.0), 351.0);
        assert!((predelay_samples(100.0, 44_100.0) - (44.0 * 149.0 - 1.0)).abs() < 1e-2);
        assert!((LFO_HZ - 0.347).abs() < 1e-3);
        assert!((LFO_DEPTH - 43.52).abs() < 1e-2);
    }

    #[test]
    fn ducking_reverb_rings_after_the_input_stops() {
        let mut effect = with(172.0, 100.0);
        let mut during = 0.0f32;
        let mut after = 0.0f32;
        for i in 0..96_000 {
            let x = if i < 48_000 {
                (i as f32 * 0.05).sin() * 0.8
            } else {
                0.0
            };
            let y = effect.process([x, x]);
            assert!(y[0].is_finite() && y[0].abs() < 4.0);
            if (24_000..48_000).contains(&i) {
                during = during.max(y[0].abs());
            } else if (52_800..60_000).contains(&i) {
                after = after.max(y[0].abs());
            }
        }
        // Mix 100 has no dry; the tail rises once the playing stops.
        assert!(after > 1e-3, "{after}");
        assert!(during < 2.0, "{during}");
    }

    #[test]
    fn stays_bounded_at_maximum_decay() {
        let mut effect = DynaRev::new(96_000.0);
        let values = [100.0, 100.0, 100.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0];
        effect.set(&values, 120.0);
        let mut peak = 0.0f32;
        for i in 0..96_000 * 3 {
            let x = if i < 96_000 {
                ((i * 7) % 13) as f32 / 13.0 - 0.5
            } else {
                0.0
            };
            let y = effect.process([x, -x]);
            peak = peak.max(y[0].abs()).max(y[1].abs());
        }
        assert!(peak.is_finite() && peak < 4.0, "{peak}");
    }
}
