//! SlapBack (SLAPBACK.ZDL): `Fx_REV_Slapback` runs `SUB_DLY_Delay`, then the
//! module's own `SUB_REV_Hall`, then mixes the result into the saved dry
//! input.
//!
//! - Delay (c7..17): the L/R mean (c13) plus `c14 · previous tap` is written
//!   to the ring; the tap c17 = `44 · ms` samples back (the editor's
//!   `(441 / 10) · ms`, minus one at exactly 1000 ms) is the mono output on
//!   both sides. F.B is `c14 = 0.95 · F.B / 100` (`Delay_fb_edit`).
//! - Hall (c18..74): `incoming = 2 · mean`, a ×1.96 / 250 Hz high-pass
//!   (c39..41), the Tone low-pass (c42/43/45, 11-entry ROM table 0x910,
//!   500 Hz .. 20 kHz at −3 dB), a 1 ms predelay (c23 = 44 from
//!   `Hall_predly_edit`), two diffusers (`0.875 x − g z` written,
//!   `g x + z` out, g = c51 = −c50 from the Decay table), and four combs
//!   (8137/7418/6904/5283 samples) whose feedback gains c56..59 (Decay ROM
//!   table 0x2f8, 30 entries) pass the c54/c55 damping pole. Eight taps inside
//!   the combs form L and R (×0.25 each, c61/c62), both through the c63..65
//!   +3 dB high shelf. The hall output is `c67 · filtered input +
//!   c66 · reverb`: DRBal sets c66 = curve(DRBal) and c67 = curve(100 − DRBal)
//!   with Main's two-slope shape (`Hall_bal_edit`).
//! - Wrapper: `out = c2 · (dry + c5 · hall)` with Mix `c5 = (Mix/100)^1.41`
//!   (`mix_edit`, curve shape 5) and Level `c2 = Level / 100` (`outLv_edit`).
//!
//! Deviations: delay-time edits crossfade the tap (16 ms) instead of muting
//! it until the new time has passed (`Delay_tapmuteMute` / `TapOpen`); the
//! hall's delay lengths and ROM filters are rescaled to the host rate; Mix
//! glides like Main's `context[7]` setter (16 ms); Tone and Decay
//! interpolate between table entries while modulated.
use super::common::{DelayLine, FadeTap, FirstOrder, Glide, Ramp, flush};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::scale_samples,
    zoom::{self, DEFAULT_STEP},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_slap_back",
    name: "SlapBack",
    category: Category::Reverb,
    zdl: "SLAPBACK.ZDL",
    description: "Slapback echo feeding a hall reverb",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Time",
            max: 1010,
            default: 379,
            display: Display::DelayTime {
                threshold: 999,
                offset: 1,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Decay",
            max: 29,
            default: 9,
            display: Display::offset(1.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 48,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 43,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 10,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "DRBal",
            max: 100,
            default: 70,
            display: Display::INT,
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(SlapBack::new(sample_rate))
}

const TIME_THRESHOLD: u16 = 999;
const MAX_MS: f32 = 1000.0;
/// Fade length of a delay-time change (the 705.6-sample argument of the
/// tap-mute setter).
const FADE_SECONDS: f32 = 705.6 / zoom::ORIGINAL_RATE;
/// Step of the hall setters (0x3af7c5ec).
const HALL_STEP: f32 = 1.888_398e-3;

/// Decay ROM table (0x800002f8): c56, c57, c58, c59, c51, c50 per entry.
const DECAY_TABLE: [[u32; 6]; 30] = [
    [
        0xbedaf1b6, 0xbeebf9d8, 0xbef901f2, 0xbf137927, 0x3f0ccb1a, 0xbf0ccb1a,
    ],
    [
        0xbeead9d6, 0xbefb91f7, 0xbf042308, 0xbf1a5735, 0x3f0df31c, 0xbf0df31c,
    ],
    [
        0xbef939f2, 0xbf04c90a, 0xbf0af916, 0xbf206b41, 0x3f0f191e, 0xbf0f191e,
    ],
    [
        0xbf052d0a, 0xbf0d151a, 0xbf130926, 0xbf277d4f, 0x3f10a321, 0xbf10a321,
    ],
    [
        0xbf0aef16, 0xbf12a125, 0xbf186931, 0xbf2c2758, 0x3f11c924, 0xbf11c924,
    ],
    [
        0xbf11d924, 0xbf194533, 0xbf1ed13e, 0xbf31ab63, 0x3f135327, 0xbf135327,
    ],
    [
        0xbf16892d, 0xbf1dc13c, 0xbf232546, 0xbf355b6b, 0x3f147929, 0xbf147929,
    ],
    [
        0xbf1c3938, 0xbf232d46, 0xbf285b51, 0xbf39c574, 0x3f16032c, 0xbf16032c,
    ],
    [
        0xbf201b40, 0xbf26df4e, 0xbf2be558, 0xbf3cc17a, 0x3f17292e, 0xbf17292e,
    ],
    [
        0xbf24d74a, 0xbf2b5f57, 0xbf303560, 0xbf405d81, 0x3f18b331, 0xbf18b331,
    ],
    [
        0xbf281b50, 0xbf2e775d, 0xbf332966, 0xbf42d586, 0x3f19d934, 0xbf19d934,
    ],
    [
        0xbf2b2756, 0xbf315763, 0xbf35e96c, 0xbf451d8a, 0x3f1b0136, 0xbf1b0136,
    ],
    [
        0xbf2ee75e, 0xbf34e16a, 0xbf394973, 0xbf47e790, 0x3f1c8b39, 0xbf1c8b39,
    ],
    [
        0xbf318163, 0xbf37556f, 0xbf3b9f77, 0xbf49d594, 0x3f1db13b, 0xbf1db13b,
    ],
    [
        0xbf34bb69, 0xbf3a5d75, 0xbf3e837d, 0xbf4c3598, 0x3f1f3b3e, 0xbf1f3b3e,
    ],
    [
        0xbf36fb6e, 0xbf3c7b79, 0xbf408781, 0xbf4ddb9c, 0x3f206141, 0xbf206141,
    ],
    [
        0xbf39c974, 0xbf3f1d7e, 0xbf430786, 0xbf4fe5a0, 0x3f21eb44, 0xbf21eb44,
    ],
    [
        0xbf3bc178, 0xbf40f582, 0xbf44c78a, 0xbf5151a3, 0x3f231146, 0xbf231146,
    ],
    [
        0xbf3e357c, 0xbf434387, 0xbf46f58e, 0xbf5319a6, 0x3f249b49, 0xbf249b49,
    ],
    [
        0xbf3ff180, 0xbf44e18a, 0xbf487f91, 0xbf5457a9, 0x3f25c14c, 0xbf25c14c,
    ],
    [
        0xbf419583, 0xbf466b8d, 0xbf49f394, 0xbf5585ab, 0x3f26e94e, 0xbf26e94e,
    ],
    [
        0xbf43a787, 0xbf485991, 0xbf4bc798, 0xbf56ffae, 0x3f287151, 0xbf287151,
    ],
    [
        0xbf451d8a, 0xbf49b593, 0xbf4d119a, 0xbf5809b0, 0x3f299953, 0xbf299953,
    ],
    [
        0xbf46f58e, 0xbf4b6d97, 0xbf4eb39d, 0xbf5959b3, 0x3f2b2156, 0xbf2b2156,
    ],
    [
        0xbf484391, 0xbf4ca599, 0xbf4fd9a0, 0xbf5a47b5, 0x3f2c4959, 0xbf2c4959,
    ],
    [
        0xbf49eb94, 0xbf4e319c, 0xbf514fa3, 0xbf5b71b7, 0x3f2dd15c, 0xbf2dd15c,
    ],
    [
        0xbf4b1996, 0xbf4f499f, 0xbf5257a5, 0xbf5c47b9, 0x3f2ef95e, 0xbf2ef95e,
    ],
    [
        0xbf4c9799, 0xbf50ada1, 0xbf53a7a7, 0xbf5d53bb, 0x3f308161, 0xbf308161,
    ],
    [
        0xbf4da99b, 0xbf51aba3, 0xbf5497a9, 0xbf5e13bc, 0x3f31a963, 0xbf31a963,
    ],
    [
        0xbf4f039e, 0xbf52eda6, 0xbf55c7ac, 0xbf5f05be, 0x3f333166, 0xbf333166,
    ],
];

/// Tone ROM table (0x80000910): c42 (b0), c43 (b1), c45 (pole) per entry.
const TONE_TABLE: [[u32; 3]; 11] = [
    [0x3cc78c70, 0x3cc78c70, 0x3f6e6215],
    [0x3d4123cc, 0x3d4123cc, 0x3f5de5d9],
    [0x3d8c748f, 0x3d8c748f, 0x3f4e6681],
    [0x3db5e3b3, 0x3db5e3b3, 0x3f3fc4c2],
    [0x3dc9c81e, 0x3dc9c81e, 0x3f38be73],
    [0x3ddd2b49, 0x3ddd2b49, 0x3f31e5c8],
    [0x3df01518, 0x3df01518, 0x3f2b37f9],
    [0x3e014674, 0x3e014674, 0x3f24b26c],
    [0x3e243da3, 0x3e243da3, 0x3f0c007b],
    [0x3e8d9029, 0x3e8d9029, 0x3e6025a5],
    [0x3f1dfed2, 0x3f1dfed2, 0xbf3e594e],
];

/// Hall core ROM (0x80000a20 → c38..): input gain and the high-pass.
const HALL_INPUT_GAIN: f32 = 2.0; // c38
const HIGH_PASS: (f32, f32, f32) = (1.960_345_9, -1.960_345_9, 0.965_000_6); // c39..41
const PREDELAY_INPUT: f32 = 1.0; // c47
const DIFFUSER_INPUT: f32 = 0.875_026_7; // c49
const DIFFUSER_DELAY_GAIN: f32 = 1.0; // c52
const COMB_INPUT: f32 = 0.375_011_44; // c53
const DAMPING: (f32, f32) = (0.496_194_2, 0.503_805_8); // c54, c55
const TAP_GAIN: f32 = 0.250_007_63; // c61, c62
/// Tone's second memcpy (0x80000998): the same +3 dB shelf for every Tone.
const OUTPUT_SHELF: (f32, f32, f32) = (1.360_783_9, -1.109_879_9, 0.749_096_04);

/// Read offsets c23..37 and write offsets c68..74 at 44.1 kHz. Tap `k`
/// reads the line of the nearest write below it, `read - write` samples
/// back; c23 is replaced by `Hall_predly_edit` with 44 (1 ms).
const PREDELAY: f32 = 44.0;
const DIFFUSER_DELAYS: [f32; 2] = [6547.0 - 4412.0, 8096.0 - 6548.0];
const COMB_WRITES: [f32; 4] = [8097.0, 16235.0, 23654.0, 30559.0];
const COMB_READS: [f32; 4] = [16234.0, 23653.0, 30558.0, 35842.0];
/// Output taps 7..14 (even index → left, odd → right), per comb.
const OUTPUT_READS: [[f32; 2]; 4] = [
    [9333.0, 9652.0],
    [17207.0, 17428.0],
    [24097.0, 24375.0],
    [30693.0, 30693.0],
];

fn word(bits: u32) -> f32 {
    f32::from_bits(bits)
}

/// Main's curve shape 2 (two slopes) over 0..100.
fn two_slope(value: f32) -> f32 {
    zoom::curve(1.0, 0.0, 101, value.clamp(0.0, 100.0), 2)
}

/// Linear interpolation between two table entries for a fractional index.
fn table_row<const N: usize>(table: &[[u32; N]], value: f32) -> [f32; N] {
    let value = value.clamp(0.0, (table.len() - 1) as f32);
    let index = (value as usize).min(table.len() - 2);
    let t = value - index as f32;
    std::array::from_fn(|k| {
        let a = word(table[index][k]);
        let b = word(table[index + 1][k]);
        a + (b - a) * t
    })
}

/// A delay line read at fixed integer distances (the hall's ring segments).
struct Segment {
    line: DelayLine,
}

impl Segment {
    fn new(max_delay: usize) -> Self {
        Self {
            line: DelayLine::new(max_delay + 1),
        }
    }
    /// The sample written `delay` frames ago, read before this frame's write.
    #[inline]
    fn tap(&self, delay: usize) -> f32 {
        self.line.read(delay - 1)
    }
}

/// `SUB_REV_Hall` of SLAPBACK.ZDL.
struct Hall {
    high_pass: FirstOrder,
    tone: FirstOrder,
    shelf: [FirstOrder; 2],
    predelay: Segment,
    predelay_length: usize,
    diffusers: [Segment; 2],
    diffuser_lengths: [usize; 2],
    combs: [Segment; 4],
    comb_lengths: [usize; 4],
    output_lengths: [[usize; 2]; 4],
    damping: [f32; 4],
    damping_in: f32,
    damping_pole: f32,
    /// c56..59, then c51 (diffuser gain g; c50 = −g).
    decay: [Ramp; 5],
    /// c66 (reverb) and c67 (filtered input).
    wet: Ramp,
    through: Ramp,
    tone_index: f32,
    sample_rate: f32,
}

impl Hall {
    fn new(sample_rate: f32) -> Self {
        let samples = |at_44k: f32| (scale_samples(at_44k, sample_rate).round() as usize).max(1);
        let (b0, b1, a1) = FirstOrder::resampled(
            HIGH_PASS.0,
            HIGH_PASS.1,
            HIGH_PASS.2,
            zoom::ORIGINAL_RATE,
            sample_rate,
        );
        let (s0, s1, sa) = FirstOrder::resampled(
            OUTPUT_SHELF.0,
            OUTPUT_SHELF.1,
            OUTPUT_SHELF.2,
            zoom::ORIGINAL_RATE,
            sample_rate,
        );
        let predelay_length = samples(PREDELAY);
        let diffuser_lengths = DIFFUSER_DELAYS.map(samples);
        let comb_lengths: [usize; 4] =
            std::array::from_fn(|j| samples(COMB_READS[j] - COMB_WRITES[j]));
        let output_lengths: [[usize; 2]; 4] =
            std::array::from_fn(|j| OUTPUT_READS[j].map(|read| samples(read - COMB_WRITES[j])));
        // The c54/c55 pole keeps its corner at the host rate, unity at DC.
        let damping_pole = DAMPING.1.powf(zoom::ORIGINAL_RATE / sample_rate);
        let mut hall = Self {
            high_pass: FirstOrder::new(b0, b1, a1),
            tone: FirstOrder::default(),
            shelf: [FirstOrder::new(s0, s1, sa); 2],
            predelay: Segment::new(predelay_length),
            predelay_length,
            diffusers: diffuser_lengths.map(Segment::new),
            diffuser_lengths,
            combs: std::array::from_fn(|j| {
                Segment::new(
                    comb_lengths[j]
                        .max(output_lengths[j][0])
                        .max(output_lengths[j][1]),
                )
            }),
            comb_lengths,
            output_lengths,
            damping: [0.0; 4],
            damping_in: DAMPING.0 * (1.0 - damping_pole) / (1.0 - DAMPING.1),
            damping_pole,
            decay: [Ramp::new(0.0, HALL_STEP, sample_rate); 5],
            wet: Ramp::new(0.0, DEFAULT_STEP, sample_rate),
            through: Ramp::new(0.0, DEFAULT_STEP, sample_rate),
            tone_index: -1.0,
            sample_rate,
        };
        hall.set_tone(10.0);
        hall
    }

    fn set_decay(&mut self, value: f32, jump: bool) {
        let row = table_row(&DECAY_TABLE, value);
        for (ramp, target) in self.decay.iter_mut().zip(row) {
            if jump {
                ramp.jump(target);
            } else {
                ramp.set(target);
            }
        }
    }

    fn set_tone(&mut self, value: f32) {
        if value == self.tone_index {
            return;
        }
        self.tone_index = value;
        let [b0, b1, a1] = table_row(&TONE_TABLE, value);
        let (b0, b1, a1) = FirstOrder::resampled(b0, b1, a1, zoom::ORIGINAL_RATE, self.sample_rate);
        self.tone.set(b0, b1, a1);
    }

    fn set_balance(&mut self, value: f32, jump: bool) {
        let (wet, through) = (two_slope(value), two_slope(100.0 - value));
        if jump {
            self.wet.jump(wet);
            self.through.jump(through);
        } else {
            self.wet.set(wet);
            self.through.set(through);
        }
    }

    /// Mono input (the delay output on both sides) → stereo hall output.
    #[inline]
    fn process(&mut self, input: f32) -> [f32; 2] {
        let incoming = HALL_INPUT_GAIN * input;
        let filtered = self.tone.process(self.high_pass.process(incoming));

        let [g0, g1, g2, g3, g] = [
            self.decay[0].tick(),
            self.decay[1].tick(),
            self.decay[2].tick(),
            self.decay[3].tick(),
            self.decay[4].tick(),
        ];
        let comb_gains = [g0, g1, g2, g3];

        // All taps are read before any write, like the original frame.
        let diffuse_input = self.predelay.tap(self.predelay_length);
        let z1 = self.diffusers[0].tap(self.diffuser_lengths[0]);
        let z2 = self.diffusers[1].tap(self.diffuser_lengths[1]);
        let mut wet = [0.0f32; 2];
        let mut comb_taps = [0.0f32; 4];
        for (j, (tap, comb)) in comb_taps.iter_mut().zip(&self.combs).enumerate() {
            *tap = comb.tap(self.comb_lengths[j]);
            wet[0] += comb.tap(self.output_lengths[j][0]);
            wet[1] += comb.tap(self.output_lengths[j][1]);
        }

        let write1 = DIFFUSER_INPUT * diffuse_input - g * z1;
        let mid = g * diffuse_input + DIFFUSER_DELAY_GAIN * z1;
        let write2 = DIFFUSER_INPUT * mid - g * z2;
        let diffusion = g * mid + DIFFUSER_DELAY_GAIN * z2;
        let shared = COMB_INPUT * diffusion;

        self.predelay.line.write(PREDELAY_INPUT * filtered);
        self.diffusers[0].line.write(flush(write1));
        self.diffusers[1].line.write(flush(write2));
        for j in 0..4 {
            self.damping[j] = flush(
                self.damping_in * (comb_gains[j] * comb_taps[j])
                    + self.damping_pole * self.damping[j],
            );
            self.combs[j].line.write(shared + self.damping[j]);
        }

        let wet_amount = self.wet.tick();
        let through = self.through.tick() * filtered;
        let left = self.shelf[0].process(TAP_GAIN * wet[0]);
        let right = self.shelf[1].process(TAP_GAIN * wet[1]);
        [through + wet_amount * left, through + wet_amount * right]
    }

    fn clear(&mut self) {
        self.high_pass.clear();
        self.tone.clear();
        for shelf in &mut self.shelf {
            shelf.clear();
        }
        self.predelay.line.clear();
        for segment in self.diffusers.iter_mut().chain(self.combs.iter_mut()) {
            segment.line.clear();
        }
        self.damping = [0.0; 4];
    }
}

struct SlapBack {
    sample_rate: f32,
    line: DelayLine,
    tap: FadeTap,
    last: f32,
    feedback: Ramp,
    mix: Glide,
    level: Ramp,
    hall: Hall,
    previous: Option<(Values, f32)>,
}

/// The editor's delay: `ms = Time + 1` (or the synced note, at most 1000 ms,
/// in whole ms), `44 · ms` samples at 44.1 kHz, one less at 1000 ms.
fn delay_samples_44k(time: f32, bpm: f32) -> f32 {
    let ms = if zoom::is_synced(time, TIME_THRESHOLD) {
        let index = zoom::sync_index(time, TIME_THRESHOLD);
        zoom::delay_note_ms(index, bpm, MAX_MS).floor().max(1.0)
    } else {
        // Continuous while modulated: the original steps in whole ms.
        time.clamp(0.0, f32::from(TIME_THRESHOLD)) + 1.0
    };
    let samples = 44.0 * ms;
    if ms >= MAX_MS { samples - 1.0 } else { samples }
}

impl SlapBack {
    fn new(sample_rate: f32) -> Self {
        let max_delay = scale_samples(44.0 * MAX_MS, sample_rate).ceil() as usize + 4;
        let defaults = crate::Model::from_key(INFO.key)
            .map(|model| model.defaults())
            .unwrap_or([0.0; crate::MAX_PARAMS]);
        let initial = scale_samples(delay_samples_44k(defaults[0], 120.0), sample_rate).round();
        let mut effect = Self {
            sample_rate,
            line: DelayLine::new(max_delay),
            tap: FadeTap::new(initial, FADE_SECONDS, sample_rate),
            last: 0.0,
            feedback: Ramp::new(0.0, DEFAULT_STEP, sample_rate),
            mix: Glide::new(0.0, sample_rate),
            level: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
            hall: Hall::new(sample_rate),
            previous: None,
        };
        effect.apply(&defaults, 120.0, true);
        effect
    }

    fn apply(&mut self, values: &Values, bpm: f32, jump: bool) {
        let previous = self.previous;
        let changed = |i: usize| previous.is_none_or(|(old, _)| old[i] != values[i]);
        let bpm_changed = previous.is_none_or(|(_, old)| old != bpm);

        if changed(0) || bpm_changed {
            // Whole samples, like the original's integer offset.
            let delay = scale_samples(delay_samples_44k(values[0], bpm), self.sample_rate).round();
            if jump {
                self.tap.jump(delay);
            } else {
                self.tap.set(delay);
            }
        }
        let set = |ramp: &mut Ramp, target: f32| {
            if jump {
                ramp.jump(target);
            } else {
                ramp.set(target);
            }
        };
        if changed(1) {
            self.hall.set_decay(values[1], jump);
        }
        if changed(2) {
            // `context[7]`: a 16 ms glide.
            let mix = zoom::curve(1.0, 0.0, 101, values[2].clamp(0.0, 100.0), 5);
            if jump {
                self.mix.jump(mix);
            } else {
                self.mix.set(mix);
            }
        }
        if changed(3) {
            let feedback = zoom::curve(0.95, 0.0, 101, values[3].clamp(0.0, 100.0), 0);
            set(&mut self.feedback, feedback);
        }
        if changed(4) {
            self.hall.set_tone(values[4]);
        }
        if changed(5) {
            self.hall.set_balance(values[5], jump);
        }
        if changed(6) {
            let level = zoom::curve(1.5, 0.0, 151, values[6].clamp(0.0, 150.0), 0);
            set(&mut self.level, level);
        }
        self.previous = Some((*values, bpm));
    }
}

impl Effect for SlapBack {
    fn set(&mut self, values: &Values, bpm: f32) {
        self.apply(values, bpm, false);
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let incoming = 0.5 * (right + left);
        self.line
            .write(flush(self.feedback.tick() * self.last + incoming));
        let delayed = self.tap.read(&self.line);
        self.last = delayed;

        let [hall_left, hall_right] = self.hall.process(delayed);
        let mix = self.mix.tick();
        let level = self.level.tick();
        [
            level * (mix * hall_left + left),
            level * (mix * hall_right + right),
        ]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.last = 0.0;
        self.tap.jump(self.tap.target());
        self.hall.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    fn defaults() -> Values {
        Model::from_key(INFO.key).unwrap().defaults()
    }

    #[test]
    fn delay_time_follows_the_editor() {
        assert_eq!(delay_samples_44k(379.0, 120.0), 44.0 * 380.0);
        assert_eq!(delay_samples_44k(999.0, 120.0), 43_999.0);
        // 1/8 at 120 BPM = 250 ms; 1/4 = 500 ms; 1/4×4 = 2 s halves to 1 s.
        assert_eq!(delay_samples_44k(1003.0, 120.0), 44.0 * 250.0);
        assert_eq!(delay_samples_44k(1006.0, 120.0), 44.0 * 500.0);
        assert_eq!(delay_samples_44k(1010.0, 120.0), 43_999.0);
    }

    #[test]
    fn the_echo_arrives_after_the_time() {
        for sample_rate in [44_100.0, 96_000.0] {
            let mut effect = SlapBack::new(sample_rate);
            let mut values = defaults();
            values[0] = 99.0; // 100 ms
            values[3] = 0.0;
            effect.apply(&values, 120.0, true);
            let expected = scale_samples(4400.0, sample_rate).round() as usize;
            let mut peak = (0, 0.0f32);
            for i in 0..expected * 2 {
                let x = if i == 0 { 1.0 } else { 0.0 };
                effect.line.write(x + effect.feedback.tick() * effect.last);
                let y = effect.tap.read(&effect.line);
                effect.last = y;
                if y.abs() > peak.1 {
                    peak = (i, y.abs());
                }
            }
            assert_eq!(peak.0, expected, "{sample_rate}");
            assert!((peak.1 - 1.0).abs() < 1e-6);
        }
    }

    #[test]
    fn hall_tables_match_the_rom() {
        let row = table_row(&DECAY_TABLE, 9.0);
        assert!((row[0] + 0.643_910_05).abs() < 1e-6);
        assert!((row[4] - 0.596_484_24).abs() < 1e-6);
        assert_eq!(word(DECAY_TABLE[9][5]), -word(DECAY_TABLE[9][4]));
        // Every Tone entry is a −3 dB low-pass at DC.
        for entry in TONE_TABLE {
            let dc = (word(entry[0]) + word(entry[1])) / (1.0 - word(entry[2]));
            assert!((dc - 0.708).abs() < 0.002, "{dc}");
        }
        assert!((two_slope(70.0) - 0.79).abs() < 1e-5);
        assert!((two_slope(30.0) - 0.39).abs() < 1e-5);
    }

    #[test]
    fn defaults_stay_near_unity() {
        let mut effect = create(48_000.0);
        effect.set(&defaults(), 120.0);
        let (mut input, mut output) = (0.0f64, 0.0f64);
        for i in 0..96_000 {
            let t = i as f32 / 48_000.0;
            let x = 0.3 * (t * 440.0 * std::f32::consts::TAU).sin();
            let [l, r] = effect.process([x, x]);
            input += f64::from(x * x) * 2.0;
            output += f64::from(l * l + r * r);
        }
        let db = 10.0 * (output / input).log10();
        assert!((-3.0..6.0).contains(&db), "{db} dB");
    }

    #[test]
    fn maximum_feedback_and_decay_stay_bounded() {
        let mut effect = create(48_000.0);
        let mut values = defaults();
        values[0] = 0.0;
        values[1] = 29.0;
        values[2] = 100.0;
        values[3] = 100.0;
        values[5] = 100.0;
        values[6] = 150.0;
        effect.set(&values, 120.0);
        let mut noise = crate::dsp::Noise::new(3);
        let mut peak = 0.0f32;
        for i in 0..48_000 * 10 {
            let x = if i < 48_000 {
                noise.bipolar() * 0.5
            } else {
                0.0
            };
            let [l, r] = effect.process([x, x]);
            peak = peak.max(l.abs()).max(r.abs());
        }
        assert!(peak.is_finite() && peak < 16.0, "{peak}");
        // And it decays once the input stops.
        let mut tail = 0.0f32;
        for _ in 0..48_000 * 20 {
            let [l, _] = effect.process([0.0; 2]);
            tail = tail.max(l.abs());
        }
        let mut late = 0.0f32;
        for _ in 0..48_000 {
            let [l, _] = effect.process([0.0; 2]);
            late = late.max(l.abs());
        }
        assert!(late < tail * 0.1 + 1e-6, "{late} vs {tail}");
    }
}
