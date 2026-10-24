//! DriveEcho (DRV_ECHO.ZDL): an EP-style preamp (`SUB_KawaOD_EP_Booster`)
//! feeding a tape-like echo (`SUB_DriveEcho`) and the original's three-way
//! mixer (`Fx_DLY_DriveEcho`).
//!
//! Follows the recovered callback:
//! - Preamp, per channel: four ROM input sections (c10..29: high-pass, two
//!   peaks, +18 dB first-order treble lift), ×4 oversampling through the
//!   original 5th-order IIR (gain −0.219 on the stuffed sample), the
//!   three-branch clipper `t·g + p·|t − 1|` on `16 · c62 · x` (c63..69),
//!   decimation through the same IIR, `× 0.4217 · c119`, five ROM output
//!   sections (c30..54) and `−2.2 ×` (c9, sign bit flipped like the original).
//! - Echo, per channel: the feedback (previous echo output) is scaled by c125,
//!   shelved (c141..145, −20 dB below ~40 Hz), clipped asymmetrically like the
//!   original (`> 0.16` tested after the shelf, `< −0.16` tested on the raw
//!   feedback word), DC-blocked (c126..130) and added to the preamp signal.
//!   Both rings are read at one shared, linearly interpolated delay; the read
//!   passes a first-order low shelf at 20 Hz, two Q 0.8 low-passes and a
//!   first-order high shelf at 2.5 kHz, whose settings follow the delay time
//!   (`FLTR_edit` tables, 248 ms steps): longer echoes are darker and thinner.
//! - Delay modulation: the target time glides through the original one-pole
//!   (c150/c151, ~122 ms) instead of jumping, so time changes bend the pitch
//!   like tape. WOW adds `(tri(2 Hz) + tri(5 Hz)) / 2` times the flutter
//!   product `Π min(tri(6/11/13.4 Hz) + 0.5, 1)²` times the depth, which is
//!   smoothed by c146/c147 (up to ±59.5 samples at 44.1 kHz).
//! - Mix: `P' = DRY ? input : preamp`, `A = c7·P' + c6·echo`, output
//!   `0.794 · A` (c2), summed to mono in Mode MONO like the original c8 = 0.
//!
//! Editors decoded: `init` (ROM copy of 172 coefficients), `Drive_edit`
//! (c62/c119 from the Drive and ClipOutLVL tables), `fb_edit` (c125 from the
//! FB table), `WetMix_edit` (c6 = (MIX/90)² below 91, then c7 = (100 − MIX)/9),
//! `time_edit` (TIME + 20 ms or a synced note ≤ 1990 ms, ×44.1 samples),
//! `FLTR_edit` (LPF/LoEQ/HiEQ tables into Main's filter designer c00bc2e0,
//! types 2/9/8, ported here), `WowFltr_edit` (depth 0..975424/8192 samples),
//! `DryThru_onf` (c5), `OUTMODE_edit` (c8).
//!
//! Approximations: the ROM biquads are re-derived for the host rate with the
//! bilinear prototype; the oversampling IIR keeps its coefficients (its corner
//! stays at the host Nyquist). Gains the original sets through Main's
//! `context[7]` (c5/c6/c7/c62/c119) ramp over 705.6 samples at 44.1 kHz;
//! Mode crossfades instead of muting the wet path; Main's output fader c4 is
//! taken as 1. TIME shows `value + 20` ms, as the pedal does.
use super::common::FirstOrder;
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, Coefficients, DelayLine},
    zoom::{self, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_drive_echo",
    name: "DriveEcho",
    category: Category::Delay,
    zdl: "DRV_ECHO.ZDL",
    description: "EP preamp drive into a darkening tape echo with wow",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "DRIVE",
            max: 100,
            default: 39,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "MIX",
            max: 100,
            default: 80,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "TIME",
            max: 1985,
            default: 355,
            display: Display::DelayTime {
                threshold: 1970,
                offset: 20,
            },
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 70,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "WOW",
            max: 100,
            default: 25,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "DRY",
            max: 1,
            default: 0,
            display: Display::labels(&["Drive", "Thru"]),
        },
        // values[6] is firmware parameter word 9
        ParamInfo {
            name: "Mode",
            max: 1,
            default: 0,
            display: Display::labels(&["Mono", "Stereo"]),
        },
    ],
};

const RATE: f32 = zoom::ORIGINAL_RATE;
const TIME_THRESHOLD: u16 = 1970;
const MAX_MS: f32 = 1990.0;

const fn f(bits: u32) -> f32 {
    f32::from_bits(bits)
}

/// c10..29: four input sections `[b0, b1, b2, a1, a2]`, `y = Σ b x + a1 y1 + a2 y2`.
const INPUT_SECTIONS: [[u32; 5]; 4] = [
    [0x3f7c6013, 0xbffc6014, 0x3f7c6013, 0x3ffc58b3, 0xbf78cee9],
    [0x3fa65cac, 0xbfd8ee93, 0x3eff9a90, 0x3fd8ee93, 0xbf4c86a1],
    [0x3f86a25d, 0xbff4b80a, 0x3f612de9, 0x3ff4b80a, 0xbf6e72a3],
    [0x40836050, 0xc04000b1, 0x00000000, 0xbdd7fdb8, 0x00000000],
];
/// c30..54: the five output sections in use (c55..59 is the identity).
const OUTPUT_SECTIONS: [[u32; 5]; 5] = [
    [0x3f82b171, 0xbffe6fe2, 0x3f7781a4, 0x3ffe6fe2, 0xbf7ce486],
    [0x3f789ed5, 0xbfcad5c8, 0x3f3f9e76, 0x3fcad5c8, 0xbf383d4c],
    [0x3f8070d7, 0xbffc3cca, 0x3f79d4fb, 0x3ffc3cca, 0xbf7ab6ab],
    [0x3eeddf69, 0xbcfae7c8, 0x00000000, 0x3f10e78a, 0x00000000],
    [0x3f7c7a49, 0xbff752c1, 0x3f756644, 0x3ff752c1, 0xbf71e08d],
];
/// c141/c142/c144: first-order feedback shelf before the clipper.
const FEEDBACK_SHELF: [u32; 3] = [0x3f799d6c, 0xbf78322e, 0x3f71cf9a];
/// c126/c127/c129: first-order DC blocker after the clipper.
const FEEDBACK_DC: [u32; 3] = [0x3f7fa2c2, 0xbf7fa2c2, 0x3f7f4585];

/// ROM 0x80000b18: the polyphase oversampling IIR, numerator
/// `[B0, B1, B2, B2, B1, B0]`, feedback `+A1..A5` (transposed direct form).
const IIR_B: [f32; 3] = [f(0x3a6867aa), f(0x3b762dbb), f(0x3be388c6)];
const IIR_A: [f32; 5] = [
    f(0x40672e31),
    f(0xc0b748a3),
    f(0x409c739a),
    f(0xc00eda14),
    f(0x3eded247),
];
const UPSAMPLE_GAIN: f32 = f(0xbe604428);
const DECIMATED_GAIN: f32 = f(0x3ed7e910);
/// c9 with c60 = 1, c61 = 0 (the preamp output stage).
const PREAMP_GAIN: f32 = f(0x400ccccd);
/// c63..c69: the clipper branches.
const CLIP_P: f32 = f(0x3f3abe11);
const CLIP_G1: f32 = f(0x3e4ccccd);
const CLIP_G0: f32 = f(0x3ecccccd);
const CLIP_T: f32 = f(0x3fa96d96);
/// c2: the fixed wet/output level (−2 dB).
const OUTPUT_LEVEL: f32 = f(0x3f4b5914);
/// The feedback clip threshold (DP 0.16 compared, SP 0x3e23d70a stored).
const FEEDBACK_CLIP: f32 = f(0x3e23d70a);

/// Fx_DLY_DriveEcho_Drive_tbl, ClipOutLVL_tbl and FB_tbl (11 points over 0..100).
const DRIVE_TABLE: [f32; 11] = [1.0, 1.5, 2.0, 2.5, 3.0, 4.0, 9.0, 14.0, 19.0, 25.0, 30.0];
const CLIP_OUT_TABLE: [f32; 11] = [
    0.5,
    1.0 / 3.0,
    0.25,
    0.2,
    1.0 / 6.0,
    0.125,
    1.0 / 12.0,
    0.075,
    0.065,
    0.058,
    0.0536,
];
const FB_TABLE: [f32; 11] = [
    0.0, 0.03, 0.05, 0.09, 0.16, 0.28, 0.40, 0.55, 0.78, 1.0, 1.25,
];
/// FLTR_edit tables, indexed by delay time in 248 ms steps.
const LPF_HZ: [f32; 10] = [
    12500.0, 5500.0, 3500.0, 1500.0, 1000.0, 1000.0, 1000.0, 900.0, 800.0, 0.0,
];
const LO_EQ_DB: [f32; 10] = [
    -22.0, -22.0, -19.0, -16.0, -15.0, -15.0, -15.0, -15.0, -15.0, 0.0,
];
const HI_EQ_DB: [f32; 10] = [0.0, 0.0, -9.0, -9.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
const FLTR_STEP_MS: f32 = 248.0;
const LO_EQ_HZ: f32 = 20.0;
const HI_EQ_HZ: f32 = 2500.0;
const LPF_Q: f32 = 0.8;

/// WowFltr_edit maximum (Q31 integer, delay units of 1/8192 sample).
const WOW_DEPTH_MAX: f32 = 975_424.0 / 8192.0;
/// c152/c153 and c157/c159/c161: LFO phase increments per 44.1 kHz sample.
const SLOW_INCREMENTS: [u32; 2] = [194_783, 486_957];
const FAST_INCREMENTS: [u32; 3] = [587_999, 1_076_781, 1_308_895];
/// c146/2^31 (depth smoothing) and c150/2^31 (time glide) per 44.1 kHz sample.
const DEPTH_POLE: f32 = 613_296.0 / 2_147_483_648.0;
const TIME_POLE: f32 = 398_662.0 / 2_147_483_648.0;
/// Main's `context[7]` ramp: 705.6 samples at 44.1 kHz.
const SWITCH_SAMPLES: f32 = 705.6;

/// `TBL_TO_VAL`: linear interpolation of an 11-point table over 0..100.
fn table(points: &[f32; 11], value: f32) -> f32 {
    let position = (value.clamp(0.0, 100.0)) / 10.0;
    let index = (position as usize).min(9);
    let fraction = position - index as f32;
    points[index] + (points[index + 1] - points[index]) * fraction
}

/// The FLTR_edit interpolation over 248 ms steps.
fn fltr_table(points: &[f32; 10], ms: f32) -> f32 {
    let index = ((ms / FLTR_STEP_MS) as usize).min(8);
    let remainder = ms - index as f32 * FLTR_STEP_MS;
    points[index] + remainder * (points[index + 1] - points[index]) / FLTR_STEP_MS
}

/// Main c00bc2e0 type 9: first-order low shelf, DC gain `db`; a cut moves the
/// corner up to `hz / gain`. Returned in the `FirstOrder` sign convention.
fn low_shelf(sample_rate: f32, hz: f32, db: f32) -> (f32, f32, f32) {
    let gain = 10f32.powf(db / 20.0);
    let corner = if db < 0.0 { hz / gain } else { hz };
    let k = (std::f32::consts::PI * corner.min(sample_rate * 0.49) / sample_rate).tan();
    let norm = 1.0 / (1.0 + k);
    (
        (1.0 + k * gain) * norm,
        (k * gain - 1.0) * norm,
        (1.0 - k) * norm,
    )
}

/// Main c00bc2e0 type 8: first-order high shelf, Nyquist gain `db`; a cut
/// moves the corner down to `hz · gain`.
fn high_shelf(sample_rate: f32, hz: f32, db: f32) -> (f32, f32, f32) {
    let gain = 10f32.powf(db / 20.0);
    let corner = if db < 0.0 { hz * gain } else { hz };
    let k = (std::f32::consts::PI * corner.min(sample_rate * 0.49) / sample_rate).tan();
    let norm = 1.0 / (1.0 + k);
    ((gain + k) * norm, (k - gain) * norm, (1.0 - k) * norm)
}

/// A ROM section `[b0, b1, b2, a1, a2]` (`+a` convention) for the host rate.
fn rom_biquad(words: [u32; 5], sample_rate: f32) -> Biquad {
    let [b0, b1, b2, a1, a2] = words.map(f32::from_bits);
    Biquad::new(
        Coefficients {
            b0,
            b1,
            b2,
            a1: -a1,
            a2: -a2,
        }
        .resample(RATE, sample_rate),
    )
}

fn rom_first_order(words: [u32; 3], sample_rate: f32) -> FirstOrder {
    let [b0, b1, a1] = words.map(f32::from_bits);
    let (b0, b1, a1) = FirstOrder::resampled(b0, b1, a1, RATE, sample_rate);
    FirstOrder::new(b0, b1, a1)
}

/// The oversampling IIR of `SUB_KawaOD_EP_Booster` (its sixth state word is
/// read but never written, so it stays zero).
#[derive(Debug, Clone, Copy, Default)]
struct Oversampler {
    z: [f32; 5],
}

impl Oversampler {
    #[inline]
    fn tick(&mut self, v: f32) -> f32 {
        let z = &mut self.z;
        let y = z[0] + IIR_B[0] * v;
        z[0] = IIR_A[0] * y + IIR_B[1] * v + z[1];
        z[1] = IIR_A[1] * y + IIR_B[2] * v + z[2];
        z[2] = IIR_A[2] * y + IIR_B[2] * v + z[3];
        z[3] = IIR_A[3] * y + IIR_B[1] * v + z[4];
        z[4] = IIR_A[4] * y + IIR_B[0] * v;
        y
    }

    #[inline]
    fn tick_zero(&mut self) -> f32 {
        let z = &mut self.z;
        let y = z[0];
        z[0] = IIR_A[0] * y + z[1];
        z[1] = IIR_A[1] * y + z[2];
        z[2] = IIR_A[2] * y + z[3];
        z[3] = IIR_A[3] * y + z[4];
        z[4] = IIR_A[4] * y;
        y
    }

    fn clear(&mut self) {
        self.z = [0.0; 5];
    }
}

#[inline]
fn clip1(x: f32) -> f32 {
    x.clamp(-1.0, 1.0)
}

/// The preamp clipper on `driven = c62 · 16 · x`: odd, slope 1.26 at zero,
/// 0.2 above |0.755| and flat at ±1 beyond ±3.
#[inline]
fn clipper(driven: f32) -> f32 {
    let (t, g) = if driven < 0.0 {
        (clip1(-CLIP_T * driven), clip1(-CLIP_G0 + CLIP_G1 * driven))
    } else {
        (clip1(CLIP_T * driven), clip1(CLIP_G0 + CLIP_G1 * driven))
    };
    let p = clip1(CLIP_P * driven);
    t * g + p * (t - 1.0).abs()
}

/// One channel of `SUB_KawaOD_EP_Booster`.
#[derive(Debug, Clone)]
struct Preamp {
    input: [Biquad; 3],
    treble: FirstOrder,
    up: Oversampler,
    down: Oversampler,
    output: [Biquad; 3],
    mellow: FirstOrder,
    presence: Biquad,
}

impl Preamp {
    fn new(sample_rate: f32) -> Self {
        let first = |w: [u32; 5]| rom_first_order([w[0], w[1], w[3]], sample_rate);
        Self {
            input: std::array::from_fn(|i| rom_biquad(INPUT_SECTIONS[i], sample_rate)),
            treble: first(INPUT_SECTIONS[3]),
            up: Oversampler::default(),
            down: Oversampler::default(),
            output: std::array::from_fn(|i| rom_biquad(OUTPUT_SECTIONS[i], sample_rate)),
            mellow: first(OUTPUT_SECTIONS[3]),
            presence: rom_biquad(OUTPUT_SECTIONS[4], sample_rate),
        }
    }

    /// `drive` is c62 · 16, `level` is 0.4217 · c118 · c119.
    #[inline]
    fn process(&mut self, x: f32, drive: f32, level: f32) -> f32 {
        let mut v = x;
        for section in &mut self.input {
            v = section.process(v);
        }
        v = self.treble.process(v);
        // ×4 zero stuffing; the tiny offset keeps the IIR out of denormals.
        let stuffed = UPSAMPLE_GAIN * v + 1e-18;
        let up = [
            self.up.tick(stuffed),
            self.up.tick_zero(),
            self.up.tick_zero(),
            self.up.tick_zero(),
        ];
        let decimated = self.down.tick(clipper(drive * up[0]));
        for &u in &up[1..] {
            self.down.tick(clipper(drive * u));
        }
        let mut v = decimated * level;
        v = self.output[0].process(v);
        v = self.output[1].process(v);
        v = self.output[2].process(v);
        v = self.mellow.process(v);
        v = self.presence.process(v);
        -PREAMP_GAIN * v
    }

    fn clear(&mut self) {
        self.input.iter_mut().for_each(Biquad::clear);
        self.treble.clear();
        self.up.clear();
        self.down.clear();
        self.output.iter_mut().for_each(Biquad::clear);
        self.mellow.clear();
        self.presence.clear();
    }
}

/// The per-channel echo filters of `SUB_DriveEcho`.
#[derive(Debug, Clone)]
struct EchoChannel {
    line: DelayLine,
    feedback: f32,
    shelf: FirstOrder,
    dc: FirstOrder,
    emphasis: FirstOrder,
    low_pass: [Biquad; 2],
    tone: FirstOrder,
}

impl EchoChannel {
    fn new(sample_rate: f32) -> Self {
        Self {
            line: DelayLine::with_seconds(2.01, sample_rate),
            feedback: 0.0,
            shelf: rom_first_order(FEEDBACK_SHELF, sample_rate),
            dc: rom_first_order(FEEDBACK_DC, sample_rate),
            emphasis: FirstOrder::new(1.0, 0.0, 0.0),
            low_pass: [Biquad::default(); 2],
            tone: FirstOrder::new(1.0, 0.0, 0.0),
        }
    }

    /// Feedback path and ring write; `gain` is c125.
    #[inline]
    fn write(&mut self, input: f32, gain: f32) {
        let raw = self.feedback;
        let first = self.shelf.process(gain * raw);
        let clipped = if first > FEEDBACK_CLIP {
            FEEDBACK_CLIP
        } else if raw < -FEEDBACK_CLIP {
            -FEEDBACK_CLIP
        } else {
            first
        };
        let regenerated = self.dc.process(clipped);
        self.line.write(input + regenerated);
    }

    #[inline]
    fn read(&mut self, delay: f32) -> f32 {
        let mut v = self.line.read_linear(delay);
        v = self.emphasis.process(v);
        v = self.low_pass[0].process(v);
        v = self.low_pass[1].process(v);
        v = self.tone.process(v);
        self.feedback = v;
        v
    }

    fn clear(&mut self) {
        self.line.clear();
        self.feedback = 0.0;
        self.shelf.clear();
        self.dc.clear();
        self.emphasis.clear();
        self.low_pass.iter_mut().for_each(Biquad::clear);
        self.tone.clear();
    }
}

/// The integer LFO block of `SUB_DriveEcho` (c148..c165, state81/82/126/127).
#[derive(Debug, Clone)]
struct Modulation {
    slow: [u32; 2],
    fast: [u32; 3],
    slow_increments: [u32; 2],
    fast_increments: [u32; 3],
    depth: f32,
    depth_target: f32,
    depth_pole: f32,
    time: f32,
    time_target: f32,
    time_pole: f32,
}

impl Modulation {
    fn new(sample_rate: f32) -> Self {
        let ratio = RATE / sample_rate;
        let scale = |increment: u32| (increment as f32 * ratio).round() as u32;
        let pole = |p: f32| 1.0 - (1.0 - p).powf(ratio);
        Self {
            slow: [0; 2],
            fast: [0; 3],
            slow_increments: SLOW_INCREMENTS.map(scale),
            fast_increments: FAST_INCREMENTS.map(scale),
            depth: 0.0,
            depth_target: 0.0,
            depth_pole: pole(DEPTH_POLE),
            time: 0.0,
            time_target: 0.0,
            time_pole: pole(TIME_POLE),
        }
    }

    /// `|phase|` of a Q31 phase word as 0..1.
    #[inline]
    fn fold(phase: u32) -> f32 {
        (phase as i32).unsigned_abs() as f32 * (1.0 / 2_147_483_648.0)
    }

    /// The delay in samples for this frame.
    #[inline]
    fn tick(&mut self) -> f32 {
        for (phase, increment) in self.slow.iter_mut().zip(self.slow_increments) {
            *phase = phase.wrapping_add(increment);
        }
        for (phase, increment) in self.fast.iter_mut().zip(self.fast_increments) {
            *phase = phase.wrapping_add(increment);
        }
        let flutter = self.fast.iter().fold(1.0, |product, &phase| {
            product * (Self::fold(phase) + 0.5).min(1.0)
        });
        let wow = 0.5 * (Self::fold(self.slow[0]) - 0.5) + 0.5 * (Self::fold(self.slow[1]) - 0.5);
        self.depth += (self.depth_target - self.depth) * self.depth_pole;
        self.time += (self.time_target - self.time) * self.time_pole;
        self.time + wow * flutter * flutter * self.depth
    }

    fn clear(&mut self) {
        self.slow = [0; 2];
        self.fast = [0; 3];
        self.depth = self.depth_target;
        self.time = self.time_target;
    }
}

struct DriveEcho {
    sample_rate: f32,
    preamp: [Preamp; 2],
    echo: [EchoChannel; 2],
    modulation: Modulation,
    /// c62 · 16 and 0.4217 · c119.
    drive: Ramp,
    level: Ramp,
    /// c125, c6 (wet), c7 (dry), c5 (dry thru), stereo amount (c8).
    feedback: Ramp,
    wet: Ramp,
    dry: Ramp,
    thru: Ramp,
    stereo: Ramp,
    filter_ms: f32,
    configured: bool,
}

impl DriveEcho {
    fn new(sample_rate: f32) -> Self {
        let switch = |value: f32, range: f32| Ramp::new(value, range / SWITCH_SAMPLES, sample_rate);
        let mut effect = Self {
            sample_rate,
            preamp: [Preamp::new(sample_rate), Preamp::new(sample_rate)],
            echo: [EchoChannel::new(sample_rate), EchoChannel::new(sample_rate)],
            modulation: Modulation::new(sample_rate),
            drive: switch(16.0, 16.0 * 29.0),
            level: switch(DECIMATED_GAIN * 0.5, DECIMATED_GAIN * 0.45),
            feedback: Ramp::new(0.0, zoom::DEFAULT_STEP, sample_rate),
            wet: switch(0.0, 1.0),
            dry: switch(1.0, 1.0),
            thru: switch(0.0, 1.0),
            stereo: switch(0.0, 1.0),
            filter_ms: -1.0,
            configured: false,
        };
        effect.design_filters(20.0);
        effect
    }

    /// `FLTR_edit`: the echo filters for a delay time in ms.
    fn design_filters(&mut self, ms: f32) {
        self.filter_ms = ms;
        let sr = self.sample_rate;
        let low_pass = Coefficients::lowpass(sr, fltr_table(&LPF_HZ, ms), LPF_Q);
        let (e0, e1, ea) = low_shelf(sr, LO_EQ_HZ, fltr_table(&LO_EQ_DB, ms));
        let (t0, t1, ta) = high_shelf(sr, HI_EQ_HZ, fltr_table(&HI_EQ_DB, ms));
        for channel in &mut self.echo {
            channel.low_pass[0].coefficients = low_pass;
            channel.low_pass[1].coefficients = low_pass;
            channel.emphasis.set(e0, e1, ea);
            channel.tone.set(t0, t1, ta);
        }
    }

    /// `time_edit`: TIME + 20 ms, or a synced note limited to 1990 ms.
    fn time_ms(value: f32, bpm: f32) -> f32 {
        if zoom::is_synced(value, TIME_THRESHOLD) {
            zoom::delay_note_ms(zoom::sync_index(value, TIME_THRESHOLD), bpm, MAX_MS)
        } else {
            (value.max(0.0) + 20.0).min(MAX_MS)
        }
    }
}

impl Effect for DriveEcho {
    fn set(&mut self, values: &Values, bpm: f32) {
        let [drive, mix, time, feedback, wow, dry, mode, ..] = *values;
        self.drive.set(16.0 * table(&DRIVE_TABLE, drive));
        self.level
            .set(DECIMATED_GAIN * table(&CLIP_OUT_TABLE, drive));
        self.feedback.set(table(&FB_TABLE, feedback));
        let mix = mix.clamp(0.0, 100.0);
        if mix < 91.0 {
            let t = mix / 90.0;
            self.wet.set(t * t);
            self.dry.set(1.0);
        } else {
            self.wet.set(1.0);
            self.dry.set((100.0 - mix) / 9.0);
        }
        self.thru.set(if dry.round() >= 1.0 { 1.0 } else { 0.0 });
        self.stereo.set(if mode.round() >= 1.0 { 1.0 } else { 0.0 });

        let ms = Self::time_ms(time, bpm);
        self.modulation.time_target = ms * 0.001 * self.sample_rate;
        self.modulation.depth_target =
            wow.clamp(0.0, 100.0) * 0.01 * WOW_DEPTH_MAX * self.sample_rate / RATE;
        if ms != self.filter_ms {
            self.design_filters(ms);
        }
        if !self.configured {
            self.configured = true;
            self.modulation.clear();
            for ramp in [
                &mut self.drive,
                &mut self.level,
                &mut self.feedback,
                &mut self.wet,
                &mut self.dry,
                &mut self.thru,
                &mut self.stereo,
            ] {
                ramp.jump(ramp.target);
            }
        }
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        let drive = self.drive.tick();
        let level = self.level.tick();
        let feedback = self.feedback.tick();
        let wet = self.wet.tick();
        let dry = self.dry.tick();
        let thru = self.thru.tick();
        let stereo = self.stereo.tick();
        let delay = self.modulation.tick();

        let mut mixed = [0.0; 2];
        for channel in 0..2 {
            let preamp = self.preamp[channel].process(frame[channel], drive, level);
            let echo = &mut self.echo[channel];
            echo.write(preamp, feedback);
            let delayed = echo.read(delay);
            let direct = preamp + (frame[channel] - preamp) * thru;
            mixed[channel] = dry * direct + wet * delayed;
        }
        let mono = 0.5 * (mixed[0] + mixed[1]);
        [
            OUTPUT_LEVEL * (mono + (mixed[0] - mono) * stereo),
            OUTPUT_LEVEL * (mono + (mixed[1] - mono) * stereo),
        ]
    }

    fn clear(&mut self) {
        self.preamp.iter_mut().for_each(Preamp::clear);
        self.echo.iter_mut().for_each(EchoChannel::clear);
        self.modulation.clear();
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(DriveEcho::new(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    fn values(edits: &[(usize, f32)]) -> Values {
        let mut values = Model::from_key("mooz_drive_echo").unwrap().defaults();
        for &(index, value) in edits {
            values[index] = value;
        }
        values
    }

    /// Index of the strongest output sample after an impulse.
    fn echo_peak(sample_rate: f32, edits: &[(usize, f32)], bpm: f32, seconds: f32) -> usize {
        let mut effect = DriveEcho::new(sample_rate);
        effect.set(&values(edits), bpm);
        let frames = (seconds * sample_rate) as usize;
        let mut best = (0, 0.0f32);
        for i in 0..frames {
            let x = if i == 0 { 0.05 } else { 0.0 };
            let y = effect.process([x, x])[0].abs();
            // Skip the dry/preamp transient.
            if i > (0.01 * sample_rate) as usize && y > best.1 {
                best = (i, y);
            }
        }
        best.0
    }

    #[test]
    fn echo_lands_at_time_plus_20_ms() {
        // MIX 100: dry off; F.B 0; WOW 0.
        let edits = [(1, 100.0), (2, 280.0), (3, 0.0), (4, 0.0)];
        for sample_rate in [44_100.0, 96_000.0] {
            let peak = echo_peak(sample_rate, &edits, 120.0, 0.6) as f32 / sample_rate;
            assert!((peak - 0.300).abs() < 0.002, "{sample_rate}: {peak}");
        }
    }

    #[test]
    fn synced_time_follows_tempo() {
        // 1971 + 6 is a quarter note: 500 ms at 120 BPM.
        let edits = [(1, 100.0), (2, 1977.0), (3, 0.0), (4, 0.0)];
        let peak = echo_peak(48_000.0, &edits, 120.0, 0.8) as f32 / 48_000.0;
        assert!((peak - 0.5).abs() < 0.002, "{peak}");
        // 1/4×8 at 120 BPM is 4000 ms, halved twice to fit 1990 ms.
        assert!((DriveEcho::time_ms(1985.0, 120.0) - 1000.0).abs() < 1e-3);
        assert!(DriveEcho::time_ms(1985.0, 60.0) <= MAX_MS);
    }

    #[test]
    fn editor_tables_interpolate_like_tbl_to_val() {
        assert!((table(&DRIVE_TABLE, 39.0) - 2.95).abs() < 1e-5);
        assert!((table(&FB_TABLE, 100.0) - 1.25).abs() < 1e-6);
        assert!((table(&FB_TABLE, 70.0) - 0.55).abs() < 1e-6);
        assert!((fltr_table(&LPF_HZ, 20.0) - (12500.0 - 20.0 / 248.0 * 7000.0)).abs() < 1e-2);
        assert!((fltr_table(&LO_EQ_DB, 1990.0) + 15.0 - 6.0 / 248.0 * 15.0).abs() < 1e-4);
    }

    #[test]
    fn designer_shelves_hit_their_gains() {
        let response = |(b0, b1, a): (f32, f32, f32), sample_rate: f32, hz: f32| {
            Coefficients {
                b0,
                b1,
                b2: 0.0,
                a1: -a,
                a2: 0.0,
            }
            .magnitude(sample_rate, hz)
        };
        let low = low_shelf(44_100.0, 20.0, -22.0);
        assert!((response(low, 44_100.0, 1.0) - 10f32.powf(-1.1)).abs() < 2e-3);
        assert!((response(low, 44_100.0, 15_000.0) - 1.0).abs() < 1e-2);
        let high = high_shelf(48_000.0, 2500.0, -9.0);
        assert!((response(high, 48_000.0, 23_900.0) - 10f32.powf(-0.45)).abs() < 2e-2);
        assert!((response(high, 48_000.0, 10.0) - 1.0).abs() < 1e-3);
        let flat = high_shelf(48_000.0, 2500.0, 0.0);
        assert!((response(flat, 48_000.0, 5000.0) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn clipper_is_odd_soft_and_bounded() {
        assert!((clipper(0.01) / 0.01 - 1.259).abs() < 0.01);
        let mut previous = clipper(-5.0);
        for i in -500..=500 {
            let x = i as f32 * 0.01;
            let y = clipper(x);
            assert!((y + clipper(-x)).abs() < 1e-6);
            assert!(y.abs() <= 1.0 && y >= previous - 1e-6, "{x}");
            previous = y;
        }
        assert!((clipper(0.7555) - 0.551).abs() < 1e-3);
    }

    #[test]
    fn oversampler_has_unity_dc_gain() {
        let mut iir = Oversampler::default();
        let mut y = 0.0;
        for _ in 0..20_000 {
            y = iir.tick(1.0);
        }
        assert!((y - 1.0).abs() < 1e-3, "{y}");
    }

    #[test]
    fn wow_depth_matches_the_editor_range() {
        for sample_rate in [44_100.0, 96_000.0] {
            let mut modulation = Modulation::new(sample_rate);
            modulation.time_target = 1000.0;
            modulation.depth_target = WOW_DEPTH_MAX * sample_rate / RATE;
            modulation.clear();
            let (mut low, mut high) = (f32::MAX, f32::MIN);
            for _ in 0..(4.0 * sample_rate) as usize {
                let delay = modulation.tick();
                low = low.min(delay);
                high = high.max(delay);
            }
            let limit = 59.6 * sample_rate / RATE;
            assert!(
                high - 1000.0 <= limit && 1000.0 - low <= limit,
                "{low} {high}"
            );
            assert!(high - low > 0.5 * limit, "{low} {high}");
        }
    }

    #[test]
    fn feedback_at_maximum_stays_bounded() {
        let mut effect = DriveEcho::new(48_000.0);
        effect.set(
            &values(&[(0, 100.0), (1, 100.0), (2, 0.0), (3, 100.0)]),
            120.0,
        );
        let mut peak = 0.0f32;
        for i in 0..48_000 * 6 {
            let x = if i < 48_000 {
                (i as f32 * 0.05).sin()
            } else {
                0.0
            };
            let [l, r] = effect.process([x, x]);
            assert!(l.is_finite() && r.is_finite());
            peak = peak.max(l.abs()).max(r.abs());
        }
        assert!(peak < 4.0, "{peak}");
    }

    #[test]
    fn defaults_sound_near_unity() {
        let mut effect = DriveEcho::new(48_000.0);
        effect.set(&values(&[]), 120.0);
        let (mut input, mut output) = (0.0f64, 0.0f64);
        for i in 0..48_000 * 2 {
            let x = 0.3 * (i as f32 * std::f32::consts::TAU * 220.0 / 48_000.0).sin();
            let [l, r] = effect.process([x, x]);
            input += f64::from(x * x);
            output += f64::from(0.5 * (l * l + r * r));
        }
        let ratio = (output / input).sqrt();
        assert!((0.3..3.0).contains(&ratio), "{ratio}");
    }

    #[test]
    fn mono_mode_sums_and_stereo_keeps_sides() {
        let mut effect = DriveEcho::new(48_000.0);
        effect.set(&values(&[(5, 1.0), (1, 0.0)]), 120.0);
        let mut last = [0.0; 2];
        for i in 0..4800 {
            let x = (i as f32 * 0.03).sin() * 0.2;
            last = effect.process([x, 0.0]);
        }
        assert!((last[0] - last[1]).abs() < 1e-6, "{last:?}");
        effect.set(&values(&[(5, 1.0), (1, 0.0), (6, 1.0)]), 120.0);
        let mut right = 0.0f32;
        for i in 0..4800 {
            let x = (i as f32 * 0.03).sin() * 0.2;
            let y = effect.process([x, 0.0]);
            if i > 2000 {
                right = right.max(y[1].abs());
            }
        }
        assert!(right < 1e-4, "{right}");
    }
}
