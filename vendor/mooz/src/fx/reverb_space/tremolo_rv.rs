//! TremoloRv (TREM_REV.ZDL), rebuilt from the recovered `Fx_REV_TremoloRv`
//! callback with its embedded `Fx_SUB_Tremolo`, and the decoded editors.
//!
//! Topology (callback): each input channel passes a 60 Hz high-pass and a
//! 3 kHz low-pass (ROM first-order sections) into its own line. A pre-delay
//! tap (PDLY; left 88 samples later than right, as `PreDelay_edit` sets
//! C122/C123) feeds four Schroeder all-passes per side (g 0.64), whose output
//! line is read by eight early-reflection taps (0.6 each). The left and right
//! reflection sums drive four combs each (input 0.43) with first-order 7 kHz
//! damping in the loop; output taps cross-mix the combs (0.6 own side, 0.13
//! other side) plus 0.5 of the reflections, then a high shelf and a low shelf.
//! The tremolo multiplies only this wet signal: a Q31 phase drives a weighted
//! sum of triangle, parabolic sine, square, `x^2` and falling ramp chosen by
//! Shape, mapped to `1 - d + d w` and smoothed by a 1/64 one-pole; Depth
//! 100..199 offsets the right channel by a quarter cycle (stereo).
//!
//! Editors decoded: Decay (`decay_Calc_set`: comb `g = 0.0005^(len_ms /
//! (100 Decay))`, all-pass 0.64), Size (`Size_edit`: three-point comb length
//! tables, feedback muted for 16 ms around the change), PDLY (`44.1 * ms`
//! samples, +90/+2), Mix (dry/wet tables), Low/High (`EQ_Calc_2Pole_Shelv`,
//! S = 2: 180/240 Hz low, 8000/4000 Hz high, -16..+12/+16 dB tables), Speed
//! (`(v + 10) / 10` Hz, notes above 190), Shape (weight table), Depth
//! (`context[22]` 0..99 % with the MN/ST split and the 0xc0000000 offset).
//!
//! Approximations: the reverb's own LFO (C116/C119) is never set by an editor,
//! so the pre-delay taps are static as in the firmware. RNDM renders like TRI:
//! the callback computes an LFSR and an accumulator for it but never uses
//! them. PDLY changes use a 16 ms tap mute on the reverb input.
use super::space_common::{self as common, Glide, TapMute};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, Coefficients, DelayLine, flush},
    zoom::{self, ORIGINAL_RATE, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_tremolo_rv",
    name: "TremoloRv",
    category: Category::Reverb,
    zdl: "TREM_REV.ZDL",
    description: "Stereo reverb with a tremolo on its tail",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Decay",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "PDLY",
            max: 500,
            default: 70,
            display: Display::number(0.0, 1.0, 0, " ms"),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 45,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Speed",
            max: 218,
            default: 28,
            display: Display::SyncRate {
                threshold: SYNC_THRESHOLD,
                offset: 1.0,
                scale: 0.1,
                decimals: 1,
                unit: " Hz",
            },
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Shape",
            max: 5,
            default: 0,
            display: Display::labels(&["SINE", "TRI", "PEAK", "RNDM", "RAMP", "SQR"]),
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Depth",
            max: 199,
            default: 199,
            display: Display::labels(&DEPTH_LABELS),
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "Size",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[7] is firmware parameter word 9
        ParamInfo {
            name: "Low",
            max: 200,
            default: 100,
            display: Display::offset(-100.0),
        },
        // values[8] is firmware parameter word 10
        ParamInfo {
            name: "High",
            max: 200,
            default: 100,
            display: Display::offset(-100.0),
        },
    ],
};

/// `_GetString_TrmDepth`: "MN" 0..99 (mono), "ST" 0..99 (stereo).
const DEPTH_LABELS: [&str; 200] = [
    "MN0", "MN1", "MN2", "MN3", "MN4", "MN5", "MN6", "MN7", "MN8", "MN9", "MN10", "MN11", "MN12",
    "MN13", "MN14", "MN15", "MN16", "MN17", "MN18", "MN19", "MN20", "MN21", "MN22", "MN23", "MN24",
    "MN25", "MN26", "MN27", "MN28", "MN29", "MN30", "MN31", "MN32", "MN33", "MN34", "MN35", "MN36",
    "MN37", "MN38", "MN39", "MN40", "MN41", "MN42", "MN43", "MN44", "MN45", "MN46", "MN47", "MN48",
    "MN49", "MN50", "MN51", "MN52", "MN53", "MN54", "MN55", "MN56", "MN57", "MN58", "MN59", "MN60",
    "MN61", "MN62", "MN63", "MN64", "MN65", "MN66", "MN67", "MN68", "MN69", "MN70", "MN71", "MN72",
    "MN73", "MN74", "MN75", "MN76", "MN77", "MN78", "MN79", "MN80", "MN81", "MN82", "MN83", "MN84",
    "MN85", "MN86", "MN87", "MN88", "MN89", "MN90", "MN91", "MN92", "MN93", "MN94", "MN95", "MN96",
    "MN97", "MN98", "MN99", "ST0", "ST1", "ST2", "ST3", "ST4", "ST5", "ST6", "ST7", "ST8", "ST9",
    "ST10", "ST11", "ST12", "ST13", "ST14", "ST15", "ST16", "ST17", "ST18", "ST19", "ST20", "ST21",
    "ST22", "ST23", "ST24", "ST25", "ST26", "ST27", "ST28", "ST29", "ST30", "ST31", "ST32", "ST33",
    "ST34", "ST35", "ST36", "ST37", "ST38", "ST39", "ST40", "ST41", "ST42", "ST43", "ST44", "ST45",
    "ST46", "ST47", "ST48", "ST49", "ST50", "ST51", "ST52", "ST53", "ST54", "ST55", "ST56", "ST57",
    "ST58", "ST59", "ST60", "ST61", "ST62", "ST63", "ST64", "ST65", "ST66", "ST67", "ST68", "ST69",
    "ST70", "ST71", "ST72", "ST73", "ST74", "ST75", "ST76", "ST77", "ST78", "ST79", "ST80", "ST81",
    "ST82", "ST83", "ST84", "ST85", "ST86", "ST87", "ST88", "ST89", "ST90", "ST91", "ST92", "ST93",
    "ST94", "ST95", "ST96", "ST97", "ST98", "ST99",
];

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(TremoloRv::new(sample_rate))
}

/// Speed values above this select tempo notes (0x800006b8).
const SYNC_THRESHOLD: u16 = 190;
/// context[8] step 0x3af7c5ec.
const STEP: f32 = 0.001_890_359;
/// All-pass lengths (44.1 kHz samples, ROM offsets) and gain (`decay_Calc_set`).
const AP_LEFT: [f32; 4] = [361.0, 635.0, 1073.0, 1422.0];
const AP_RIGHT: [f32; 4] = [393.0, 602.0, 1138.0, 1357.0];
const AP_GAIN: f32 = 0.64;
/// Early-reflection taps on the diffused lines, gain 0.6 each.
const ER_LEFT: [f32; 8] = [361.0, 823.0, 1046.0, 1326.0, 1676.0, 2113.0, 2663.0, 3349.0];
const ER_RIGHT: [f32; 8] = [361.0, 816.0, 1037.0, 1316.0, 1663.0, 2097.0, 2642.0, 3323.0];
/// Comb lengths over Size 0/50/100 (0x800009a0 left-fed, 0x800009d0 right-fed).
const COMB_LEFT: [[f32; 3]; 4] = [
    [2604.0, 4082.0, 5115.0],
    [2506.0, 3494.0, 4345.0],
    [2244.0, 3158.0, 3876.0],
    [1875.0, 2368.0, 2956.0],
];
const COMB_RIGHT: [[f32; 3]; 4] = [
    [2672.0, 4082.0, 5115.0],
    [2506.0, 3494.0, 4345.0],
    [2278.0, 3158.0, 3876.0],
    [1886.0, 2368.0, 2956.0],
];
/// Output taps into the left-fed / right-fed combs, per output side.
const OUT_LEFT_FROM_LEFT: [f32; 4] = [397.0, 1367.0, 743.0, 1669.0];
const OUT_LEFT_FROM_RIGHT: [f32; 4] = [1013.0, 571.0, 1709.0, 1321.0];
const OUT_RIGHT: [f32; 4] = [701.0, 967.0, 1231.0, 1499.0];
/// Mix tables (0x80000a30 wet, 0x80000a00 dry).
const WET: [f32; 11] = [
    0.002_913, 0.020_64, 0.203_517, 0.417_83, 0.583_848, 0.711_705, 0.806_678, 0.894_026,
    0.946_019, 0.985_145, 1.0,
];
const DRY: [f32; 11] = [
    1.0, 1.0, 0.979_828, 0.910_437, 0.811_335, 0.703_315, 0.591_153, 0.448_539, 0.316_628,
    0.157_997, 0.003_078,
];
/// Shelf gains in dB over 0..100 steps away from the centre.
const HIGH_CUT: [f32; 5] = [0.0, -3.0, -6.0, -12.0, -16.0];
const HIGH_BOOST: [f32; 5] = [0.0, 3.0, 6.0, 12.0, 16.0];
const LOW_CUT: [f32; 5] = [0.0, -4.0, -8.0, -13.5, -16.0];
const LOW_BOOST: [f32; 5] = [0.0, 3.0, 6.0, 9.0, 12.0];
/// Shape weights (0x80000928): triangle, square, sine, falling ramp, square law.
const SHAPES: [[f32; 5]; 6] = [
    [0.0, 0.0, 1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 1.0],
    [1.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 1.0, 0.0],
    [0.0, 1.0, 0.0, 0.0, 0.0],
];

/// The tremolo LFO of `Fx_SUB_Tremolo` for one channel's phase (turns).
#[inline]
fn tremolo_wave(weights: &[f32; 5], turns: f32) -> f32 {
    let u = turns - turns.floor();
    let x = common::triangle(u);
    let [triangle, square, sine, ramp, peak] = *weights;
    let mut w = triangle * x + peak * x * x;
    if sine != 0.0 {
        w += sine * common::parabolic(u);
    }
    if square != 0.0 && x >= 0.5 {
        w += square;
    }
    if ramp != 0.0 {
        let rising = u + 0.5;
        w += ramp * (1.0 - (rising - rising.floor()));
    }
    w
}

/// `Size_edit`: TBL_TO_VAL_int over three points.
fn comb_length(points: &[f32; 3], size: f32) -> f32 {
    let position = (size * 2.0 / 100.0).clamp(0.0, 2.0);
    let index = (position as usize).min(1);
    points[index] + ((position - index as f32) * (points[index + 1] - points[index])).trunc()
}

struct Side {
    input_high: Biquad,
    input_low: Biquad,
    input: DelayLine,
    allpasses: [DelayLine; 4],
    ap_delay: [usize; 4],
    diffused: DelayLine,
    er_delay: [usize; 8],
    combs: [DelayLine; 4],
    damping: [Biquad; 4],
    comb_gain: [Ramp; 4],
    comb_delay: [usize; 4],
    comb_points: [[f32; 3]; 4],
    high: Biquad,
    low: Biquad,
}

impl Side {
    fn new(sample_rate: f32, ap: &[f32; 4], er: &[f32; 8], comb_points: &[[f32; 3]; 4]) -> Self {
        let scale = sample_rate / ORIGINAL_RATE;
        let line = |at_44k: f32| DelayLine::new((at_44k * scale).ceil() as usize + 8);
        Self {
            input_high: Biquad::new(Coefficients::highpass1(sample_rate, 60.0)),
            input_low: Biquad::new(Coefficients::lowpass1(sample_rate, 3000.0)),
            input: line(500.0 * 44.1 + 91.0),
            allpasses: std::array::from_fn(|i| line(ap[i])),
            ap_delay: ap.map(|d| common::samples(d, sample_rate)),
            diffused: line(er[7]),
            er_delay: er.map(|d| common::samples(d, sample_rate)),
            combs: std::array::from_fn(|i| line(comb_points[i][2])),
            damping: [Biquad::new(Coefficients::lowpass1(sample_rate, 7000.0)); 4],
            comb_gain: [Ramp::new(0.0, STEP, sample_rate); 4],
            comb_delay: [1; 4],
            comb_points: *comb_points,
            high: Biquad::new(Coefficients::IDENTITY),
            low: Biquad::new(Coefficients::IDENTITY),
        }
    }

    /// Input filters, pre-delay tap, diffusion and reflections; returns the
    /// reflection sum (C67 x 0.6 x taps).
    #[inline]
    fn front(&mut self, x: f32, predelay: usize, gate: f32) -> f32 {
        let filtered = self.input_low.process(self.input_high.process(x));
        let mut v = gate * common::tap(&self.input, predelay);
        self.input.write(filtered);
        for (line, &delay) in self.allpasses.iter_mut().zip(&self.ap_delay) {
            let delayed = common::tap(line, delay);
            let w = flush(v + AP_GAIN * delayed);
            line.write(w);
            v = delayed - AP_GAIN * w;
        }
        let mut reflections = 0.0;
        for &delay in &self.er_delay {
            reflections += common::tap(&self.diffused, delay);
        }
        self.diffused.write(v);
        0.6 * reflections
    }

    /// Comb loop: read feedback taps, damp and write.
    #[inline]
    fn combs(&mut self, reflections: f32, feedback_scale: f32) {
        let input = 0.43 * reflections;
        for (((line, damping), gain), &delay) in self
            .combs
            .iter_mut()
            .zip(&mut self.damping)
            .zip(&mut self.comb_gain)
            .zip(&self.comb_delay)
        {
            let fed = feedback_scale * gain.tick() * common::tap(line, delay);
            line.write(damping.process(fed + input));
        }
    }

    fn set_size(&mut self, size: f32, sample_rate: f32) {
        for (delay, points) in self.comb_delay.iter_mut().zip(&self.comb_points) {
            *delay = common::samples(comb_length(points, size), sample_rate);
        }
    }

    fn set_decay(&mut self, decay: f32, sample_rate: f32, jump: bool) {
        for (gain, &delay) in self.comb_gain.iter_mut().zip(&self.comb_delay) {
            let ms = delay as f32 / sample_rate * 1000.0;
            let g = if decay > 0.0 {
                0.0005f32.powf(ms / (100.0 * decay))
            } else {
                0.0
            };
            if jump {
                gain.jump(g);
            } else {
                gain.set(g);
            }
        }
    }

    fn clear(&mut self) {
        self.input_high.clear();
        self.input_low.clear();
        self.input.clear();
        for line in self.allpasses.iter_mut().chain(self.combs.iter_mut()) {
            line.clear();
        }
        self.diffused.clear();
        for filter in &mut self.damping {
            filter.clear();
        }
        self.high.clear();
        self.low.clear();
    }
}

struct TremoloRv {
    sample_rate: f32,
    left: Side,
    right: Side,
    out_from_left: [usize; 4],
    out_from_right: [usize; 4],
    out_right: [usize; 4],
    predelay: [usize; 2],
    predelay_target: [usize; 2],
    predelay_mute: TapMute,
    size_mute: TapMute,
    size: f32,
    size_target: f32,
    decay: f32,
    wet: Glide,
    dry: Glide,
    phase: f32,
    increment: f32,
    weights: [f32; 5],
    depth: f32,
    right_offset: f32,
    smooth: f32,
    gain: [f32; 2],
    speed: f32,
    bpm: f32,
    last: Option<Values>,
}

impl TremoloRv {
    fn new(sample_rate: f32) -> Self {
        let samples = |taps: [f32; 4]| taps.map(|d| common::samples(d, sample_rate));
        let mut effect = Self {
            sample_rate,
            left: Side::new(sample_rate, &AP_LEFT, &ER_LEFT, &COMB_LEFT),
            right: Side::new(sample_rate, &AP_RIGHT, &ER_RIGHT, &COMB_RIGHT),
            out_from_left: samples(OUT_LEFT_FROM_LEFT),
            out_from_right: samples(OUT_LEFT_FROM_RIGHT),
            out_right: samples(OUT_RIGHT),
            predelay: [1; 2],
            predelay_target: [1; 2],
            predelay_mute: TapMute::new(705.6, 705.6, sample_rate),
            size_mute: TapMute::new(705.6, 705.6, sample_rate),
            size: 50.0,
            size_target: 50.0,
            decay: 50.0,
            wet: Glide::new(0.0, 705.6, sample_rate),
            dry: Glide::new(1.0, 705.6, sample_rate),
            phase: 0.0,
            increment: 0.0,
            weights: SHAPES[0],
            depth: 0.0,
            right_offset: 0.0,
            // 1/64 per 44.1 kHz sample
            smooth: 1.0 - (63.0f32 / 64.0).powf(ORIGINAL_RATE / sample_rate),
            gain: [1.0; 2],
            speed: 28.0,
            bpm: 120.0,
            last: None,
        };
        effect.apply_size();
        effect
    }

    fn apply_size(&mut self) {
        self.size = self.size_target;
        let sr = self.sample_rate;
        self.left.set_size(self.size, sr);
        self.right.set_size(self.size, sr);
        self.left.set_decay(self.decay, sr, true);
        self.right.set_decay(self.decay, sr, true);
    }

    fn set_rate(&mut self) {
        let hz = if zoom::is_synced(self.speed, SYNC_THRESHOLD) {
            zoom::rate_note_hz(zoom::sync_index(self.speed, SYNC_THRESHOLD), self.bpm)
        } else {
            (self.speed.min(f32::from(SYNC_THRESHOLD)) + 10.0) / 10.0
        };
        self.increment = hz / self.sample_rate;
    }
}

impl Effect for TremoloRv {
    fn set(&mut self, values: &Values, bpm: f32) {
        let first = self.last.is_none();
        let last = self.last.unwrap_or([f32::NAN; crate::MAX_PARAMS]);
        let changed = |i: usize| values[i] != last[i];
        let value = |i: usize, max: f32| {
            if values[i].is_finite() {
                values[i].clamp(0.0, max)
            } else {
                0.0
            }
        };
        let sr = self.sample_rate;

        if changed(2) {
            let mix = value(2, 100.0);
            let wet = common::table(&WET, 101, mix);
            let dry = common::table(&DRY, 101, mix);
            if first {
                self.wet.jump(wet);
                self.dry.jump(dry);
            } else {
                self.wet.set(wet);
                self.dry.set(dry);
            }
        }
        if changed(1) {
            let samples = (44.1 * value(1, 500.0)).trunc();
            self.predelay_target = [
                common::samples(samples + 90.0, sr),
                common::samples(samples + 2.0, sr),
            ];
            if first {
                self.predelay = self.predelay_target;
            } else if self.predelay_target != self.predelay {
                self.predelay_mute.request(false);
            }
        }
        if changed(6) {
            self.size_target = value(6, 100.0).round();
            if first {
                self.apply_size();
            } else if self.size_target != self.size {
                self.size_mute.request(false);
            }
        }
        if changed(0) {
            self.decay = value(0, 100.0);
            self.left.set_decay(self.decay, sr, first);
            self.right.set_decay(self.decay, sr, first);
        }
        if changed(7) || changed(8) {
            let low = value(7, 200.0);
            let low = if low < 100.0 {
                common::shelf(
                    sr,
                    180.0,
                    common::table(&LOW_CUT, 101, 100.0 - low),
                    2.0,
                    false,
                )
            } else {
                common::shelf(
                    sr,
                    240.0,
                    common::table(&LOW_BOOST, 101, low - 100.0),
                    2.0,
                    false,
                )
            };
            let high = value(8, 200.0);
            let high = if high < 100.0 {
                common::shelf(
                    sr,
                    8000.0,
                    common::table(&HIGH_CUT, 101, 100.0 - high),
                    2.0,
                    true,
                )
            } else {
                common::shelf(
                    sr,
                    4000.0,
                    common::table(&HIGH_BOOST, 101, high - 100.0),
                    2.0,
                    true,
                )
            };
            for side in [&mut self.left, &mut self.right] {
                side.low.coefficients = low;
                side.high.coefficients = high;
            }
        }
        if changed(3) || bpm != self.bpm {
            self.speed = value(3, 218.0);
            self.bpm = bpm;
            self.set_rate();
        }
        if changed(4) {
            self.weights = SHAPES[value(4, 5.0).round() as usize];
        }
        if changed(5) {
            let depth = value(5, 199.0);
            let stereo = depth.round() > 99.0;
            self.depth = if stereo { depth - 100.0 } else { depth } / 100.0;
            self.depth = self.depth.clamp(0.0, 0.99);
            self.right_offset = if stereo { -0.25 } else { 0.0 };
        }
        if first {
            let w = tremolo_wave(&self.weights, 0.0);
            self.gain = [1.0 - self.depth + self.depth * w; 2];
        }
        self.last = Some(*values);
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        if self.predelay_mute.tick() {
            self.predelay = self.predelay_target;
        }
        if self.size_mute.tick() {
            self.apply_size();
        }
        let gate = self.predelay_mute.gain;
        let feedback_scale = self.size_mute.gain;

        let reflections_left = self.left.front(left, self.predelay[0], gate);
        let reflections_right = self.right.front(right, self.predelay[1], gate);

        // Output taps read before the combs are written this frame.
        let sum = |line: &[DelayLine; 4], taps: &[usize; 4]| {
            line.iter()
                .zip(taps)
                .map(|(line, &delay)| common::tap(line, delay))
                .sum::<f32>()
        };
        let left_own = sum(&self.left.combs, &self.out_from_left);
        let left_cross = sum(&self.right.combs, &self.out_from_right);
        let right_own = sum(&self.right.combs, &self.out_right);
        let right_cross = sum(&self.left.combs, &self.out_right);
        self.left.combs(reflections_left, feedback_scale);
        self.right.combs(reflections_right, feedback_scale);

        let tail_left = 0.6 * left_own + 0.13 * left_cross + 0.5 * reflections_left;
        let tail_right = 0.6 * right_own + 0.13 * right_cross + 0.5 * reflections_right;
        let wet_left = self.left.low.process(self.left.high.process(tail_left));
        let wet_right = self.right.low.process(self.right.high.process(tail_right));

        // Fx_SUB_Tremolo on the wet signal.
        common::advance(&mut self.phase, self.increment);
        let depth = self.depth;
        let target_left = 1.0 - depth + depth * tremolo_wave(&self.weights, self.phase);
        let target_right =
            1.0 - depth + depth * tremolo_wave(&self.weights, self.phase + self.right_offset);
        self.gain[0] += (target_left - self.gain[0]) * self.smooth;
        self.gain[1] += (target_right - self.gain[1]) * self.smooth;

        let wet = self.wet.tick();
        let dry = self.dry.tick();
        [
            dry * left + wet * self.gain[0] * wet_left,
            dry * right + wet * self.gain[1] * wet_right,
        ]
    }

    fn clear(&mut self) {
        self.left.clear();
        self.right.clear();
        if self.predelay_mute.busy() {
            self.predelay_mute.reset();
            self.predelay = self.predelay_target;
        }
        if self.size_mute.busy() {
            self.size_mute.reset();
            self.apply_size();
        }
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
    fn tremolo_rate_follows_the_editor() {
        let mut effect = TremoloRv::new(48_000.0);
        let mut values = defaults();
        values[3] = 0.0;
        effect.set(&values, 120.0);
        assert!((effect.increment * 48_000.0 - 1.0).abs() < 1e-4);
        values[3] = 190.0;
        effect.set(&values, 120.0);
        assert!((effect.increment * 48_000.0 - 20.0).abs() < 1e-3);
        values[3] = 191.0 + 7.0; // quarter note
        effect.set(&values, 120.0);
        assert!((effect.increment * 48_000.0 - 2.0).abs() < 1e-3);
        effect.set(&values, 60.0);
        assert!((effect.increment * 48_000.0 - 1.0).abs() < 1e-3);
    }

    #[test]
    fn tremolo_waves_span_the_depth() {
        for weights in SHAPES {
            let (mut low, mut high) = (f32::MAX, f32::MIN);
            for i in 0..1000 {
                let w = tremolo_wave(&weights, i as f32 / 1000.0);
                low = low.min(w);
                high = high.max(w);
            }
            assert!(low.abs() < 0.01 && (high - 1.0).abs() < 0.01, "{weights:?}");
        }
    }

    #[test]
    fn stereo_depth_offsets_the_right_channel() {
        let mut effect = TremoloRv::new(44_100.0);
        let mut values = defaults();
        values[5] = 150.0;
        effect.set(&values, 120.0);
        assert!((effect.depth - 0.5).abs() < 1e-6);
        assert_eq!(effect.right_offset, -0.25);
        values[5] = 50.0;
        effect.set(&values, 120.0);
        assert_eq!(effect.right_offset, 0.0);
    }

    #[test]
    fn decay_and_size_set_the_combs() {
        let mut effect = TremoloRv::new(44_100.0);
        effect.set(&defaults(), 120.0);
        assert_eq!(effect.left.comb_delay, [4082, 3494, 3158, 2368]);
        let g = effect.left.comb_gain[0].target;
        let expected = 0.0005f32.powf(4082.0 / 44.1 / 5000.0);
        assert!((g - expected).abs() < 1e-5, "{g} {expected}");
        assert_eq!(comb_length(&COMB_LEFT[0], 25.0), 2604.0 + 739.0);
        assert_eq!(comb_length(&COMB_LEFT[0], 100.0), 5115.0);
    }

    #[test]
    fn predelay_sets_the_first_arrival() {
        let sample_rate = 48_000.0;
        let mut effect = TremoloRv::new(sample_rate);
        let mut values = defaults();
        values[1] = 100.0;
        values[2] = 100.0;
        values[5] = 0.0;
        effect.set(&values, 120.0);
        let mut first = [None; 2];
        for i in 0..20_000 {
            let x = if i == 0 { 1.0 } else { 0.0 };
            let out = effect.process([x, x]);
            for (side, y) in out.into_iter().enumerate() {
                if first[side].is_none() && y.abs() > 1e-4 && i > 10 {
                    first[side] = Some(i as f32);
                }
            }
        }
        // pre-delay + first reflection tap (the all-passes pass -g^4 at once)
        let scale = sample_rate / 44_100.0;
        let right = (4410.0 + 2.0 + 361.0) * scale;
        let left = (4410.0 + 90.0 + 361.0) * scale;
        assert!(
            (first[1].unwrap() - right).abs() < 4.0,
            "{:?} {right}",
            first[1]
        );
        assert!(
            (first[0].unwrap() - left).abs() < 4.0,
            "{:?} {left}",
            first[0]
        );
    }

    #[test]
    fn no_runaway_at_maximum_decay() {
        let mut effect = TremoloRv::new(44_100.0);
        let mut values = defaults();
        values[0] = 100.0;
        values[6] = 100.0;
        values[7] = 200.0;
        values[8] = 200.0;
        effect.set(&values, 120.0);
        let mut noise = crate::dsp::Noise::new(5);
        let mut peak = 0.0f32;
        for i in 0..44_100 * 8 {
            let x = if i < 44_100 {
                noise.bipolar() * 0.5
            } else {
                0.0
            };
            let [l, r] = effect.process([x, -x]);
            assert!(l.is_finite() && r.is_finite());
            peak = peak.max(l.abs()).max(r.abs());
        }
        assert!(peak < 16.0, "{peak}");
    }
}
