//! ParticleR (PARTICLE.ZDL): `Fx_REV_PerticleR` with its embedded
//! `Fx_SUB_Arena`, rebuilt from the recovered callback (`particle_reverb/`)
//! and the decoded editors.
//!
//! What follows the original:
//! - Front: the mono input (L+R)/2 runs through the ROM input filters, a gentle
//!   +3 dB air shelf (C73..77) and a 15 Hz high-pass (C78..82). It then feeds a
//!   serial chain of eight modulated delay "particles". Bank k is written with
//!   its own tap times the DWELL feedback (C15), plus the previous lane's tap.
//!   Bank 4 also gets lane 2 times 0.74 (C43). Lane 7 passes through a
//!   lowpass shelf (C106..110) before it feeds back and reaches the outputs.
//! - The lane delays are the ROM C7..C14: 100, 150, 185, 201, 250, 299, 360
//!   and 401 ms, plus one sample. Lanes 2..7 are swept by 60 samples with the
//!   firmware's parabolic sine (`0.5 + 2c(1-|c|)`) at 2 Hz, with phases i/8.
//!   Lanes 0 and 1 stand still.
//! - Lane 4 follows MODE (`Mode_edit` table at 0x80000b78):
//!   - STBL: 250 ms, swept by 60 samples at 2 Hz.
//!   - CRTCL: 500 ms, swept over the full 500 ms by a 0.059 Hz rising saw.
//!   - HZD: 500 ms, swept by a 1.245 Hz triangle.
//!
//!   The base delay glides through the C71/C72 one-pole (122 ms).
//! - Stereo output gains are the ROM C23..C38, and MODE swaps the lane 4..7
//!   gains (tables 0x80000cc8 and 0x80000ca8). Both channels pass through the
//!   ROM 20 Hz high-pass, the 3 kHz dip and the -1 dB shelf (C88..102).
//! - The Arena reverb gets the front output. Its path:
//!   - Mono input times 2, then a 250 Hz high-pass with 6 dB gain (C142..145)
//!     and a 2.75 kHz lowpass (0x80000d58).
//!   - A 400 ms pre-delay (C127 = 17640).
//!   - Two Arena diffusers of 2135 and 1548 samples (C153..156; `decay_set`
//!     gains ±0.622).
//!   - Input gain 0.375 into four damped combs of 8137, 7418, 6904 and 5283
//!     samples (one-pole C158/C159).
//!   - Output taps 0.25 each, then a +3 dB high shelf (0x80000d68), mixed
//!     0.475 (`curve(1,0,101,59,5)`) over the unchanged front signal.
//! - Editors:
//!   - `Dwell_edit`: comb feedback from the 9-point table 0.40..0.80. The
//!     Arena comb gains are `-(0.001^(ms/T))`, where `ms` is each comb's
//!     integer length in ms and T comes from the 11-point table
//!     2.5..100 s (0x80000c00). Both ramp with step 0.00189.
//!   - `Gain_edit`: -24..0 dB times 0.10777.
//!   - `WetMix_edit`: 9-point dry/wet tables.
//!   - `Mono_edit`: adds the other wet channel.
//!   - `Level_edit`: called with a fixed 106 of 150, giving 1.06.
//!   - The firmware ramps (16 ms timed, or per-sample steps) are kept.
//!
//! Approximations: the comb damping filters C117..119 and the second lane-7
//! filter are identity in ROM and never edited, so they are left out. The
//! lane-4 glide starts at its target on create/clear (the original starts
//! from 0 ms). ROM filters are moved to the host rate through their analog
//! prototypes. Delays, LFO rates and the glide are rescaled.
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, Coefficients, DelayLine, flush},
    zoom::{ORIGINAL_RATE, curve},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_particle_r",
    name: "ParticleR",
    category: Category::Reverb,
    zdl: "PARTICLE.ZDL",
    description: "Glitchy modulated delay cascade into a big reverb",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "DWELL",
            max: 100,
            default: 40,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "GAIN",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "MODE",
            max: 2,
            default: 0,
            display: Display::labels(&["STBL", "CRTCL", "HZD"]),
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "MIX",
            max: 100,
            default: 60,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "MONO",
            max: 1,
            default: 0,
            display: Display::OFF_ON,
        },
    ],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(ParticleR::new(sample_rate))
}

/// C7..C14 / 65536: lane delays in 44.1 kHz samples (lane 4 is MODE's).
const BASE: [f32; 8] = [
    4410.0, 6615.0, 8158.5, 8864.1, 11025.0, 13185.9, 15876.0, 17684.1,
];
/// C63..C70 / 65536: sweep depth in samples (lane 4 is MODE's).
const DEPTH: f32 = 60.0;
/// C47/C49/...: phase increments (2^32 per cycle) at 44.1 kHz; lane 4 is MODE's.
const INCREMENT: [u32; 8] = [0, 0, 194_783, 194_783, 194_783, 194_783, 194_783, 194_783];
/// Mode_edit table 0x80000b78: lane-4 base, depth (samples), increment and
/// the weights of the sine, saw (C104) and triangle (C105) shapes.
const MODES: [(f32, f32, u32, [f32; 3]); 3] = [
    (11025.0, 60.0, 194_783, [1.0, 0.0, 0.0]),
    (22050.0, 22050.0, 5_791, [0.0, 1.0, 0.0]),
    (22050.0, 22050.0, 121_245, [0.0, 0.0, 1.0]),
];
/// Output gains C23..C26 / C31..C34 (fixed) and Mode_edit tables for C27..30 / C35..38.
const LEFT_FIXED: [f32; 4] = [1.0, 0.0, 0.92, 0.084];
const RIGHT_FIXED: [f32; 4] = [0.0, 1.0, 0.046, 0.84];
/// 0x80000cc8 (STBL, CRTCL) and 0x80000ca8 (HZD): [left 4..7, right 4..7].
const MODE_GAINS: [[f32; 8]; 2] = [
    [0.74, 0.3392, 0.54, 0.4, 0.2738, 0.64, 0.3834, 0.4],
    [0.74, 0.0896, 0.54, 0.4, 0.0, 0.64, 0.4374, 0.4],
];
/// Dwell_edit tables: comb feedback (0x80000c30) and Arena decay in ms (0x80000c00).
const FEEDBACK: [f32; 9] = [0.4, 0.45, 0.5, 0.55, 0.6, 0.65, 0.7, 0.75, 0.8];
const DECAY_MS: [f32; 11] = [
    2500.0, 2500.0, 4000.0, 6000.0, 8000.0, 10000.0, 20000.0, 30000.0, 40000.0, 50000.0, 100000.0,
];
/// WetMix_edit tables: dry C5 (0x80000c58) and wet C6 (0x80000c80).
const DRY: [f32; 9] = [
    1.0,
    0.999_194_4,
    0.999_884_9,
    0.995_290_8,
    0.857_136_5,
    0.670_270_3,
    0.445_553_6,
    0.018_912_5,
    0.0,
];
const WET: [f32; 9] = [
    0.038_348_6,
    0.125_184_3,
    0.155_901_4,
    0.392_283_5,
    0.773_570_7,
    0.996_093_3,
    0.997_125_9,
    0.994_603_5,
    1.0,
];
/// Arena segment lengths at 44.1 kHz (differences of the C127..C141 read and
/// C172..C178 write offsets).
const PRE_DELAY: f32 = 17640.0;
const DIFFUSER: [f32; 2] = [2135.0, 1548.0];
const COMB: [f32; 4] = [8137.0, 7418.0, 6904.0, 5283.0];
/// Comb lengths in whole ms as Dwell_edit computes them (`10 * len / 441`).
const COMB_MS: [f32; 4] = [184.0, 168.0, 156.0, 119.0];
/// Output taps after each comb's write: L/R from combs 1 and 2, shared from 3 and 4.
const TAPS: [f32; 6] = [1236.0, 1555.0, 972.0, 1193.0, 721.0, 134.0];
/// Arena gains: diffuser write input (C153), feedback (C154), direct (C155),
/// tank input (C157), damping (C158), tap gain (C165/C166) and wet (C170).
const DIFFUSER_IN: f32 = 0.875_026_7;
const DIFFUSER_FEEDBACK: f32 = -0.621_997_7;
const DIFFUSER_DIRECT: f32 = 0.621_997_7;
const TANK_INPUT: f32 = 0.375_011_44;
const DAMPING: f32 = 0.580_197_8;
const TAP_GAIN: f32 = 0.250_007_63;
/// Level_edit with the fixed 106 of 150.
const LEVEL: f32 = 1.06;
/// Gain_edit's factor 0x3ddcb952.
const GAIN_SCALE: f32 = 0.107_775_35;
/// Editor steps per 44.1 kHz sample and the 16 ms timed ramp.
const DWELL_STEP: f32 = 0.001_890_359;
const MIX_STEP: f32 = 9.451_8e-4;
const TIMED: f32 = 705.6;
/// C71: lane-4 base delay glide per 44.1 kHz sample.
const GLIDE: f32 = 1.856_414_6e-4;

/// TBL_TO_VAL: linear interpolation of a table over `0..=count-1`.
fn table(values: &[f32], count: f32, value: f32) -> f32 {
    let last = values.len() - 1;
    let position = (value / (count - 1.0)).clamp(0.0, 1.0) * last as f32;
    let index = (position as usize).min(last - 1);
    let fraction = position - index as f32;
    values[index] + (values[index + 1] - values[index]) * fraction
}

/// A first-order ROM section `y = b0 x + b1 x1 + f y1` moved to `rate` through
/// its bilinear analog prototype.
fn first_order(b0: f32, b1: f32, feedback: f32, rate: f32) -> Coefficients {
    let (b0, b1, a1) = (b0 as f64, b1 as f64, -feedback as f64);
    let r = rate as f64 / ORIGINAL_RATE as f64;
    let (n0, n1) = (b0 + b1, (b0 - b1) * r);
    let (d0, d1) = (1.0 + a1, (1.0 - a1) * r);
    let a0 = d0 + d1;
    Coefficients {
        b0: ((n0 + n1) / a0) as f32,
        b1: ((n0 - n1) / a0) as f32,
        b2: 0.0,
        a1: ((d0 - d1) / a0) as f32,
        a2: 0.0,
    }
}

/// A ROM biquad stored as `[b0, b1, b2, +f1, +f2]` (feedback added).
fn rom_biquad(c: [f32; 5], rate: f32) -> Coefficients {
    Coefficients {
        b0: c[0],
        b1: c[1],
        b2: c[2],
        a1: -c[3],
        a2: -c[4],
    }
    .resample(ORIGINAL_RATE, rate)
}

/// The firmware float setters: a linear ramp by a fixed step per sample.
#[derive(Debug, Clone, Copy)]
struct Glide {
    current: f32,
    target: f32,
    step: f32,
}

impl Glide {
    fn new(value: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
        }
    }
    fn jump(&mut self, value: f32) {
        self.current = value;
        self.target = value;
    }
    /// context[8]: a fixed step per host sample.
    fn step_to(&mut self, target: f32, step: f32) {
        self.target = target;
        self.step = step;
    }
    /// context[7]: reach the target after `samples` host samples.
    fn over(&mut self, target: f32, samples: f32) {
        self.target = target;
        self.step = (target - self.current).abs() / samples.max(1.0);
    }
    #[inline]
    fn tick(&mut self) -> f32 {
        if self.current != self.target {
            let difference = self.target - self.current;
            if difference.abs() <= self.step {
                self.current = self.target;
            } else {
                self.current += self.step.copysign(difference);
            }
        }
        self.current
    }
}

/// One modulated "particle" delay of the front chain.
struct Lane {
    line: DelayLine,
    phase: f32,
    increment: f32,
}

impl Lane {
    /// The firmware's LFO shapes for a phase in turns: the parabolic sine
    /// `0.5 + 2c(1 - |c|)`, the triangle `|p|` and the rising saw `0.5 + p/2`.
    #[inline]
    fn shapes(&mut self) -> (f32, f32, f32) {
        self.phase += self.increment;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        let p = if self.phase < 0.5 {
            2.0 * self.phase
        } else {
            2.0 * self.phase - 2.0
        };
        let a = p.abs();
        let c = a - 0.5;
        (0.5 + 2.0 * c * (1.0 - c.abs()), a, 0.5 + 0.5 * p)
    }
}

struct Comb {
    line: DelayLine,
    length: f32,
    damping: f32,
    gain: Glide,
}

struct ParticleR {
    ratio: f32,
    fresh: bool,
    // Front.
    input_shelf: Biquad,
    input_highpass: Biquad,
    lanes: [Lane; 8],
    /// Host-rate base delays and depths (lane 4: glide target, current, depth).
    base: [f32; 8],
    depth: f32,
    lane4_target: f32,
    lane4_base: f32,
    lane4_depth: f32,
    lane4_weights: [f32; 3],
    glide: f32,
    lane7_filter: Biquad,
    feedback: Glide,
    gains: [Glide; 8],
    output_filters: [[Biquad; 3]; 2],
    // Arena.
    arena_highpass: Biquad,
    arena_lowpass: Biquad,
    pre_delay: DelayLine,
    pre_length: f32,
    diffusers: [DelayLine; 2],
    diffuser_length: [f32; 2],
    combs: [Comb; 4],
    pole: f32,
    taps: [f32; 6],
    tone: [Biquad; 2],
    wet_level: f32,
    // Output.
    dry: Glide,
    wet: Glide,
    gain: Glide,
    mono: Glide,
    mode: usize,
    last: Values,
}

impl ParticleR {
    fn new(sample_rate: f32) -> Self {
        let rate = sample_rate.max(1000.0);
        let ratio = rate / ORIGINAL_RATE;
        let line = |samples: f32| DelayLine::new((samples * ratio).ceil() as usize + 4);
        let lanes = std::array::from_fn(|i| Lane {
            line: line(if i == 4 { 22052.0 } else { BASE[i] + 2.0 }),
            phase: i as f32 / 8.0,
            increment: INCREMENT[i] as f32 / 4_294_967_296.0 / ratio,
        });
        let output = |rate| {
            [
                Biquad::new(rom_biquad(
                    [
                        0.997_966_77,
                        -1.995_933_5,
                        0.997_966_77,
                        1.995_929_5,
                        -0.995_937_5,
                    ],
                    rate,
                )),
                Biquad::new(rom_biquad(
                    [0.980_542_1, -1.513_202_2, 0.661_608, 1.513_202_2, -0.642_15],
                    rate,
                )),
                Biquad::new(first_order(0.922_350_3, -0.350_402_4, 0.428_052_1, rate)),
            ]
        };
        let pole = (1.0 - DAMPING).powf(1.0 / ratio);
        let mut effect = Self {
            ratio,
            fresh: true,
            input_shelf: Biquad::new(first_order(1.191_918_8, -0.122_35, -0.069_568_9, rate)),
            input_highpass: Biquad::new(rom_biquad(
                [
                    0.998_982_8,
                    -1.997_965_7,
                    0.998_982_8,
                    1.997_964_7,
                    -0.997_966_7,
                ],
                rate,
            )),
            lanes,
            base: BASE.map(|samples| samples * ratio),
            depth: DEPTH * ratio,
            lane4_target: MODES[0].0 * ratio,
            lane4_base: MODES[0].0 * ratio,
            lane4_depth: MODES[0].1 * ratio,
            lane4_weights: MODES[0].3,
            glide: 1.0 - (1.0 - GLIDE).powf(1.0 / ratio),
            lane7_filter: Biquad::new(first_order(0.614_416_8, -0.160_420_5, 0.546_003_7, rate)),
            feedback: Glide::new(FEEDBACK[0]),
            gains: std::array::from_fn(|_| Glide::new(0.0)),
            output_filters: [output(rate), output(rate)],
            arena_highpass: Biquad::new(first_order(
                2.0 * 1.960_345_9,
                2.0 * -1.960_345_9,
                0.965_000_6,
                rate,
            )),
            arena_lowpass: Biquad::new(first_order(0.117_227_73, 0.117_227_73, 0.668_822_8, rate)),
            pre_delay: line(PRE_DELAY),
            pre_length: PRE_DELAY * ratio,
            diffusers: DIFFUSER.map(line),
            diffuser_length: DIFFUSER.map(|samples| samples * ratio),
            combs: COMB.map(|samples| Comb {
                line: line(samples),
                length: samples * ratio,
                damping: 0.0,
                gain: Glide::new(0.0),
            }),
            pole,
            taps: TAPS.map(|samples| samples * ratio),
            tone: [0, 1]
                .map(|_| Biquad::new(first_order(1.360_784, -1.109_879_9, 0.749_096, rate))),
            wet_level: curve(1.0, 0.0, 101, 59.0, 5),
            dry: Glide::new(1.0),
            wet: Glide::new(0.0),
            gain: Glide::new(GAIN_SCALE),
            mono: Glide::new(0.0),
            mode: usize::MAX,
            last: [f32::NAN; crate::MAX_PARAMS],
        };
        let defaults = crate::Model::from_key(INFO.key)
            .map(|model| model.defaults())
            .unwrap_or_default();
        effect.set(&defaults, 120.0);
        effect
    }

    fn set_mode(&mut self, mode: usize, jump: bool) {
        self.mode = mode;
        let (base, depth, increment, weights) = MODES[mode];
        self.lane4_target = base * self.ratio;
        if jump {
            self.lane4_base = self.lane4_target;
        }
        self.lane4_depth = depth * self.ratio;
        self.lane4_weights = weights;
        self.lanes[4].increment = increment as f32 / 4_294_967_296.0 / self.ratio;
        self.lanes[4].phase = 0.0;
        // Gains are indexed [left 4..7, right 4..7]; lanes 0..3 use fixed gains.
        let table = &MODE_GAINS[usize::from(mode == 2)];
        for (gain, &value) in self.gains.iter_mut().zip(table) {
            if jump {
                gain.jump(value);
            } else {
                gain.over(value, TIMED * self.ratio);
            }
        }
    }

    fn reset(&mut self) {
        for lane in &mut self.lanes {
            lane.line.clear();
        }
        for (i, lane) in self.lanes.iter_mut().enumerate() {
            lane.phase = if i == 4 { 0.0 } else { i as f32 / 8.0 };
        }
        self.lane4_base = self.lane4_target;
        for filter in [
            &mut self.input_shelf,
            &mut self.input_highpass,
            &mut self.lane7_filter,
            &mut self.arena_highpass,
            &mut self.arena_lowpass,
        ] {
            filter.clear();
        }
        for filter in self.output_filters.iter_mut().flatten() {
            filter.clear();
        }
        for filter in &mut self.tone {
            filter.clear();
        }
        self.pre_delay.clear();
        for line in &mut self.diffusers {
            line.clear();
        }
        for comb in &mut self.combs {
            comb.line.clear();
            comb.damping = 0.0;
        }
    }
}

impl Effect for ParticleR {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let jump = self.fresh;
        self.fresh = false;
        let dwell = values[0].clamp(0.0, 100.0);
        if dwell != self.last[0] {
            self.last[0] = dwell;
            let feedback = table(&FEEDBACK, 101.0, dwell);
            let decay = table(&DECAY_MS, 101.0, dwell);
            let step = DWELL_STEP / self.ratio;
            if jump {
                self.feedback.jump(feedback);
            } else {
                self.feedback.step_to(feedback, step);
            }
            for (comb, ms) in self.combs.iter_mut().zip(COMB_MS) {
                let gain = -(0.001f32.powf(ms / decay));
                if jump {
                    comb.gain.jump(gain);
                } else {
                    comb.gain.step_to(gain, step);
                }
            }
        }
        let gain = values[1].clamp(0.0, 100.0);
        if gain != self.last[1] {
            self.last[1] = gain;
            let db = curve(0.0, -24.0, 101, gain, 0);
            let linear = 10f32.powf(db / 20.0) * GAIN_SCALE;
            if jump {
                self.gain.jump(linear);
            } else {
                self.gain.over(linear, TIMED * self.ratio);
            }
        }
        let mode = (values[2].round().clamp(0.0, 2.0)) as usize;
        if mode != self.mode {
            self.set_mode(mode, jump);
        }
        let mix = values[3].clamp(0.0, 100.0);
        if mix != self.last[3] {
            self.last[3] = mix;
            let (dry, wet) = (table(&DRY, 101.0, mix), table(&WET, 101.0, mix));
            if jump {
                self.dry.jump(dry);
                self.wet.jump(wet);
            } else {
                let step = MIX_STEP / self.ratio;
                self.dry.step_to(dry, step);
                self.wet.step_to(wet, step);
            }
        }
        let mono = if values[4] >= 0.5 { 1.0 } else { 0.0 };
        if mono != self.last[4] {
            self.last[4] = mono;
            if jump {
                self.mono.jump(mono);
            } else {
                self.mono.over(mono, TIMED * self.ratio);
            }
        }
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        // Front: input filters and the eight particle delays.
        let mono = 0.5 * (frame[0] + frame[1]);
        let input = self.input_highpass.process(self.input_shelf.process(mono));
        let mut taps = [0.0f32; 8];
        for (i, (lane, tap)) in self.lanes.iter_mut().zip(&mut taps).enumerate() {
            let (sine, triangle, saw) = lane.shapes();
            let delay = if i == 4 {
                self.lane4_base += (self.lane4_target - self.lane4_base) * self.glide;
                let w = self.lane4_weights;
                let shape = sine * w[0] + saw * w[1] + triangle * w[2];
                (self.lane4_base - shape * self.lane4_depth).max(0.0)
            } else {
                self.base[i] - sine * self.depth
            };
            // One sample beyond the Q16 delay, as the cursor moves before the write.
            *tap = lane.line.read_linear(delay + self.ratio - 1.0);
        }
        let last = self.lane7_filter.process(taps[7]);
        let feedback = self.feedback.tick();
        let writes = [
            feedback * taps[0] + input,
            feedback * taps[1] + taps[0],
            feedback * taps[2] + taps[1],
            feedback * taps[3] + taps[2],
            feedback * taps[4] + taps[3] + taps[2] * 0.74,
            feedback * taps[5] + taps[4],
            feedback * taps[6] + taps[5],
            feedback * last + taps[6],
        ];
        for (lane, write) in self.lanes.iter_mut().zip(writes) {
            lane.line.write(flush(write));
        }
        let mut gains = [0.0f32; 8];
        for (value, gain) in gains.iter_mut().zip(&mut self.gains) {
            *value = gain.tick();
        }
        let mut left = taps[2] * LEFT_FIXED[2] + taps[3] * LEFT_FIXED[3] + taps[0];
        let mut right = taps[2] * RIGHT_FIXED[2] + taps[3] * RIGHT_FIXED[3] + taps[1];
        left += taps[4] * gains[0] + taps[5] * gains[1] + taps[6] * gains[2] + last * gains[3];
        right += taps[4] * gains[4] + taps[5] * gains[5] + taps[6] * gains[6] + last * gains[7];
        for filter in &mut self.output_filters[0] {
            left = filter.process(left);
        }
        for filter in &mut self.output_filters[1] {
            right = filter.process(right);
        }

        // Arena: input filters, pre-delay, two diffusers and four damped combs.
        let arena_in = self
            .arena_lowpass
            .process(self.arena_highpass.process(0.5 * (left + right)));
        let pre = self.pre_delay.read_linear(self.pre_length - 1.0);
        self.pre_delay.write(arena_in);
        let mut x = pre;
        for (line, &length) in self.diffusers.iter_mut().zip(&self.diffuser_length) {
            let delayed = line.read_linear(length - 1.0);
            line.write(flush(DIFFUSER_IN * x + DIFFUSER_FEEDBACK * delayed));
            x = DIFFUSER_DIRECT * x + delayed;
        }
        let tank_in = x * TANK_INPUT;
        let t = &self.taps;
        let [c0, c1, c2, c3] = &mut self.combs;
        let shared = c2.line.read_linear(t[4] - 1.0) + c3.line.read_linear(t[5] - 1.0);
        let tank_left = c0.line.read_linear(t[0] - 1.0) + c1.line.read_linear(t[2] - 1.0) + shared;
        let tank_right = c0.line.read_linear(t[1] - 1.0) + c1.line.read_linear(t[3] - 1.0) + shared;
        for comb in &mut self.combs {
            let end = comb.line.read_linear(comb.length - 1.0);
            let gain = comb.gain.tick();
            comb.damping = flush(self.pole * comb.damping + (1.0 - self.pole) * gain * end);
            comb.line.write(tank_in + comb.damping);
        }
        let wet_left = left + self.wet_level * self.tone[0].process(tank_left * TAP_GAIN);
        let wet_right = right + self.wet_level * self.tone[1].process(tank_right * TAP_GAIN);

        // Output mix: dry C5, wet C6, Gain C116, Mono C121 and Level C2.
        let (dry, wet) = (self.dry.tick(), self.wet.tick() * self.gain.tick());
        let mono = self.mono.tick();
        [
            LEVEL * (frame[0] * dry + (wet_left + mono * wet_right) * wet),
            LEVEL * (frame[1] * dry + (wet_right + mono * wet_left) * wet),
        ]
    }

    fn clear(&mut self) {
        self.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> Values {
        crate::Model::from_key(INFO.key).unwrap().defaults()
    }

    fn render(effect: &mut dyn Effect, frames: usize, impulse: bool) -> Vec<[f32; 2]> {
        (0..frames)
            .map(|i| {
                let x = if impulse && i == 0 { 1.0 } else { 0.0 };
                effect.process([x, x])
            })
            .collect()
    }

    fn first_peak(output: &[[f32; 2]], channel: usize, from: usize, to: usize) -> usize {
        (from..to)
            .max_by(|&a, &b| {
                output[a][channel]
                    .abs()
                    .partial_cmp(&output[b][channel].abs())
                    .unwrap()
            })
            .unwrap()
    }

    #[test]
    fn tables_interpolate_like_tbl_to_val() {
        assert!((table(&FEEDBACK, 101.0, 0.0) - 0.4).abs() < 1e-6);
        assert!((table(&FEEDBACK, 101.0, 100.0) - 0.8).abs() < 1e-6);
        assert!((table(&FEEDBACK, 101.0, 50.0) - 0.6).abs() < 1e-6);
        assert!((table(&DECAY_MS, 101.0, 40.0) - 8000.0).abs() < 1e-2);
        // MIX 60: position 4.8 between 0.857 and 0.670.
        assert!((table(&DRY, 101.0, 60.0) - 0.707_09).abs() < 1e-3);
    }

    #[test]
    fn first_lane_echo_lands_at_the_rom_delay() {
        for rate in [44_100.0, 48_000.0, 96_000.0] {
            let mut effect = ParticleR::new(rate);
            let mut values = defaults();
            values[3] = 100.0; // all wet
            effect.set(&values, 120.0);
            let output = render(&mut effect, (6000.0 * rate / 44_100.0) as usize, true);
            // Lane 0 only reaches the left channel (C23 = 1, C31 = 0).
            let expected = 4411.0 * rate / 44_100.0;
            let peak = first_peak(&output, 0, 100, output.len());
            assert!(
                (peak as f32 - expected).abs() <= 3.0,
                "{rate}: {peak} vs {expected}"
            );
        }
    }

    #[test]
    fn right_channel_starts_one_lane_later() {
        let mut effect = ParticleR::new(44_100.0);
        let mut values = defaults();
        values[3] = 100.0;
        effect.set(&values, 120.0);
        let output = render(&mut effect, 12_000, true);
        // Lane 1 (C32 = 1) hears lane 0's output: 4411 + 6615 + 1 samples, less
        // its static sweep offset (phase 1/8 gives shape 0.125 of 60 samples).
        let early: f32 = output[200..10_900].iter().map(|f| f[1].abs()).sum();
        assert!(early < 1e-3, "{early}");
        let peak = first_peak(&output, 1, 10_900, 12_000);
        assert!((peak as i32 - 11_019).abs() <= 2, "{peak}");
    }

    #[test]
    fn arena_comb_gains_follow_the_decay_table() {
        let mut effect = ParticleR::new(44_100.0);
        let mut values = defaults();
        values[0] = 0.0;
        effect.fresh = true;
        effect.set(&values, 120.0);
        let expected = -(0.001f32.powf(184.0 / 2500.0));
        assert!((effect.combs[0].gain.current - expected).abs() < 1e-6);
        assert!((effect.feedback.current - 0.4).abs() < 1e-6);
    }

    #[test]
    fn mode_sets_the_lane_four_sweep() {
        let mut effect = ParticleR::new(48_000.0);
        let mut values = defaults();
        values[2] = 1.0;
        effect.set(&values, 120.0);
        let hz = effect.lanes[4].increment * 48_000.0;
        assert!((hz - 0.059_46).abs() < 1e-4, "{hz}");
        assert!((effect.lane4_target - 22_050.0 * 48_000.0 / 44_100.0).abs() < 0.1);
        values[2] = 2.0;
        effect.set(&values, 120.0);
        let hz = effect.lanes[4].increment * 48_000.0;
        assert!((hz - 1.2449).abs() < 1e-3, "{hz}");
        assert_eq!(effect.gains[1].target, 0.0896);
    }

    #[test]
    fn gain_runs_from_minus_24_db() {
        let mut effect = ParticleR::new(44_100.0);
        let mut values = defaults();
        values[1] = 0.0;
        effect.fresh = true;
        effect.set(&values, 120.0);
        let db = 20.0 * (effect.gain.current / GAIN_SCALE).log10();
        assert!((db + 24.0).abs() < 1e-3, "{db}");
    }

    #[test]
    fn dry_passes_at_mix_zero() {
        let mut effect = ParticleR::new(44_100.0);
        let mut values = defaults();
        values[3] = 0.0;
        effect.fresh = true;
        effect.set(&values, 120.0);
        let output = effect.process([0.5, -0.25]);
        assert!((output[0] - 0.5 * LEVEL).abs() < 1e-3, "{output:?}");
        assert!((output[1] + 0.25 * LEVEL).abs() < 1e-3, "{output:?}");
    }

    #[test]
    fn maximum_dwell_stays_bounded() {
        for mode in 0..3 {
            let mut effect = ParticleR::new(48_000.0);
            let mut values = defaults();
            values[0] = 100.0;
            values[2] = mode as f32;
            values[3] = 100.0;
            effect.set(&values, 120.0);
            let mut noise = crate::dsp::Noise::new(3);
            let mut peak = 0.0f32;
            for i in 0..48_000 * 20 {
                let x = if i < 48_000 * 10 {
                    0.5 * noise.bipolar()
                } else {
                    0.0
                };
                let output = effect.process([x, x]);
                peak = peak.max(output[0].abs()).max(output[1].abs());
                assert!(output.iter().all(|v| v.is_finite()));
            }
            assert!(peak < 16.0, "mode {mode}: {peak}");
        }
    }

    #[test]
    fn defaults_sound_near_unity() {
        let mut effect = ParticleR::new(48_000.0);
        effect.set(&defaults(), 120.0);
        let mut noise = crate::dsp::Noise::new(1);
        let (mut input, mut output) = (0.0f64, 0.0f64);
        for i in 0..48_000 * 6 {
            let t = i as f32 / 48_000.0;
            let envelope = (-(t * 2.0).fract() * 6.0).exp();
            let x =
                0.5 * (t * 196.0 * std::f32::consts::TAU).sin() * envelope + 0.02 * noise.bipolar();
            let y = effect.process([x, x]);
            if i > 48_000 * 2 {
                input += f64::from(x * x);
                output += f64::from(0.5 * (y[0] * y[0] + y[1] * y[1]));
            }
        }
        let db = 10.0 * (output / input).log10();
        assert!(db.abs() < 4.0, "{db}");
    }
}
