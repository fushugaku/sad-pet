//! SpaceHole (SPC_HOLE.ZDL), rebuilt from the recovered `Fx_REV_SpaceHole`
//! callback and its decoded editors.
//!
//! Topology (callback): the mono mean of the input passes a 10 Hz DC blocker
//! and a fixed +2.5 dB/160 Hz peak (ROM biquads), adds the F.B feedback taken
//! one frame late from the first (Decay > 100) or last (Decay <= 100) tap of
//! the early-reflection FIR, passes a second DC blocker and enters the input
//! line. A pre-delay tap (PDLY + 1 sample) modulated by a fixed 0.19 Hz LFO
//! (`Depth` x 1024 samples) feeds the FIR line: 20 taps -> 6 Schroeder
//! all-passes. Their output (plus, for Decay > 100, the pre-delayed signal)
//! drives four parallel combs (101/85/72/61 ms) through 0.19; four comb taps
//! per side give the stereo tail, which is also written to a chorus line read
//! by two taps modulated in opposite phase by the Speed LFO. Wet = chorus +
//! tail (+ direct network and pre-delay terms for Decay > 100), then a 350 Hz
//! low shelf, a high shelf and a fixed +2 dB bass shelf (ROM), mixed with the
//! stereo dry signal. Both LFOs are the callback's parabolic-bent triangle,
//! sharing one depth smoothed over ~0.39 s.
//!
//! Editors decoded: Decay/Size (`FB_tapmuteOpen` + `decay_Calc_set`: two
//! ranges 0..100 and 101..200 with their own FIR tap/gain tables, all-pass
//! lengths and gains, comb RT (`g = t1^(t0/T)`), output gains, and for the
//! upper range the direct terms `0.5 (d/99)^1.41`), PDLY (`44.1 * ms + 1`
//! samples behind a 220-sample tap mute), Mix (dry/wet tables, wet x 6 dB x
//! 0.71), F.B (`0.99 * v / 100`), Depth (table), Speed (Q31 increment table,
//! ~0..1 Hz), Low/High (`EQ_Calc_2Pole_Shelv`: 350 Hz S 1.5/1.6, 1400 Hz S 3.9
//! or 3200 Hz S 1.6, -19..+12 dB tables). Displays follow
//! `GetString_offset_minus100` (Decay, Low, High show -100..100).
//!
//! Approximations: the direct terms of the upper Decay range follow the
//! current Decay (the firmware latches them when the range is entered); Size
//! changes inside a range use a short tap mute (the firmware jumps the taps);
//! a range change mutes, holds silence for the FIR length + 10 ms (assumed ms,
//! like the firmware's deferred `FB_tapmuteOpen`) and reopens.
use super::space_common::{self as common, Glide, TapMute};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, Coefficients, DelayLine, Smoother, flush},
    zoom::{ORIGINAL_RATE, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_space_hole",
    name: "SpaceHole",
    category: Category::Reverb,
    zdl: "SPC_HOLE.ZDL",
    description: "Swelling modulated space with echo feedback",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Decay",
            max: 200,
            default: 50,
            display: Display::offset(-100.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "PDLY",
            max: 1000,
            default: 80,
            display: Display::number(0.0, 1.0, 0, " ms"),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: " Mix",
            max: 100,
            default: 40,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: " F.B",
            max: 100,
            default: 45,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 58,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Speed",
            max: 100,
            default: 39,
            display: Display::INT,
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "Size",
            max: 100,
            default: 29,
            display: Display::INT,
        },
        // values[7] is firmware parameter word 9
        ParamInfo {
            name: " Low",
            max: 200,
            default: 87,
            display: Display::offset(-100.0),
        },
        // values[8] is firmware parameter word 10
        ParamInfo {
            name: "High",
            max: 200,
            default: 82,
            display: Display::offset(-100.0),
        },
    ],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(SpaceHole::new(sample_rate))
}

/// context[8] step 0x3af7c5ec used for every network gain.
const STEP: f32 = 0.001_890_359;
/// FIR taps `t0 + size * t1` ms (0x80000830 for Decay <= 100, 0x80000790 above).
const FIR_MS: [[(f32, f32); 20]; 2] = [
    [
        (2.0, 0.0),
        (8.32, 10.68),
        (20.3, 26.7),
        (34.5, 44.5),
        (48.48, 60.52),
        (63.78, 87.22),
        (80.64, 110.36),
        (95.5, 133.5),
        (115.92, 153.08),
        (129.44, 181.56),
        (151.86, 201.14),
        (167.38, 229.62),
        (188.02, 250.98),
        (206.66, 272.34),
        (227.52, 295.48),
        (240.58, 336.42),
        (269.9, 347.1),
        (285.2, 373.8),
        (306.72, 402.28),
        (324.46, 432.54),
    ],
    [
        (2.0, 0.0),
        (7.0, 12.0),
        (17.0, 30.0),
        (29.0, 50.0),
        (41.0, 68.0),
        (53.0, 98.0),
        (67.0, 124.0),
        (79.0, 150.0),
        (97.0, 172.0),
        (107.0, 204.0),
        (127.0, 226.0),
        (139.0, 258.0),
        (157.0, 282.0),
        (173.0, 306.0),
        (191.0, 332.0),
        (199.0, 378.0),
        (227.0, 390.0),
        (239.0, 420.0),
        (257.0, 452.0),
        (271.0, 486.0),
    ],
];
/// FIR tap gains (0x800008d0 / 0x80000920), copied over the ROM by `decay_Calc_set`.
const FIR_GAIN: [[f32; 20]; 2] = [
    [
        0.01, 0.03, 0.05, 0.07, 0.1, 0.12, 0.14, 0.17, 0.23, 0.29, 0.33, 0.41, 0.46, 0.53, 0.59,
        0.68, 0.78, 0.87, 0.98, 1.0,
    ],
    [
        0.99, 0.97, 0.95, 0.93, 0.9, 0.88, 0.86, 0.83, 0.77, 0.71, 0.67, 0.59, 0.54, 0.47, 0.41,
        0.32, 0.22, 0.13, 0.02, 0.0,
    ],
];
/// All-pass lengths `t0 + size * t1` ms (0x80000a50 / 0x80000a20).
const AP_MS: [[(f32, f32); 6]; 2] = [
    [
        (23.53, 20.47),
        (19.33, 16.82),
        (15.88, 13.81),
        (13.04, 11.35),
        (10.72, 9.32),
        (8.8, 7.66),
    ],
    [
        (21.0, 23.0),
        (17.25, 18.89),
        (14.17, 15.52),
        (11.64, 12.75),
        (9.56, 10.47),
        (7.86, 8.6),
    ],
];
/// Per-range Decay tables (11 entries): T from/to (ms), output gain from/to,
/// all-pass gain from/to; each pair is interpolated by Size.
struct DecayTables {
    time: [[f32; 11]; 2],
    output: [[f32; 11]; 2],
    allpass: [[f32; 11]; 2],
}
const DECAY: [DecayTables; 2] = [
    DecayTables {
        time: [
            [
                100.0, 250.0, 300.0, 350.0, 400.0, 450.0, 500.0, 550.0, 600.0, 700.0, 1000.0,
            ],
            [
                500.0, 900.0, 1000.0, 1100.0, 1200.0, 1300.0, 1400.0, 1500.0, 1600.0, 1800.0,
                2000.0,
            ],
        ],
        output: [
            [1.0; 11],
            [0.95, 0.92, 0.92, 0.92, 0.92, 0.92, 0.92, 0.9, 0.9, 0.9, 0.9],
        ],
        allpass: [
            [0.6, 0.6, 0.6, 0.6, 0.6, 0.7, 0.6, 0.6, 0.7, 0.72, 0.75],
            [0.6, 0.6, 0.6, 0.6, 0.6, 0.8, 0.8, 0.8, 0.8, 0.8, 0.8],
        ],
    },
    DecayTables {
        time: [
            [
                300.0, 360.0, 420.0, 480.0, 540.0, 600.0, 660.0, 700.0, 1000.0, 1250.0, 1500.0,
            ],
            [
                5000.0, 5010.0, 5080.0, 5270.0, 5640.0, 6250.0, 7160.0, 8430.0, 10120.0, 12290.0,
                15000.0,
            ],
        ],
        output: [
            [
                0.6, 0.59, 0.58, 0.57, 0.56, 0.55, 0.54, 0.5, 0.45, 0.37, 0.3,
            ],
            [
                0.3, 0.29, 0.27, 0.26, 0.24, 0.23, 0.21, 0.2, 0.18, 0.17, 0.15,
            ],
        ],
        allpass: [
            [0.5, 0.5, 0.5, 0.5, 0.5, 0.6, 0.65, 0.7, 0.7, 0.7, 0.7],
            [0.8; 11],
        ],
    },
];
/// Comb `(t0 ms, t1)`: `g = t1^(t0 / T)` (0x80000d80); t0 is each comb's length.
const COMB: [(f32, f32); 4] = [
    (101.103_47, 0.01238),
    (85.398_66, 0.01238),
    (72.133_34, 0.013032),
    (60.928_574, 0.013718),
];
/// Comb lengths and output taps in 44.1 kHz samples (ROM offsets).
const COMB_LENGTH: [f32; 4] = [4459.0, 3766.0, 3181.0, 2687.0];
const TAPS_LEFT: [(usize, f32); 4] = [(1, 1279.0), (0, 1499.0), (3, 523.0), (2, 353.0)];
const TAPS_RIGHT: [(usize, f32); 4] = [(1, 173.0), (0, 43.0), (3, 523.0), (2, 353.0)];
/// Mix tables (0x80000d50 wet, 0x80000ab0 dry).
const WET: [f32; 11] = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
const DRY: [f32; 11] = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.8, 0.6, 0.4, 0.2, 0.0];
/// `mix_edit`: wet x context[26](6 dB) x 0.71.
const WET_SCALE: f32 = 1.995_262_3 * 0.71;
/// Speed: Q31 phase increments per 44.1 kHz sample (0x80000d20); 0 is 1.
const SPEED: [f32; 11] = [
    97.0, 9826.0, 19556.0, 29285.0, 39015.0, 48744.0, 58473.0, 68203.0, 77932.0, 87662.0, 97391.0,
];
/// The fixed pre-delay LFO increment (C128 = 18583, 0.19 Hz).
const PREDELAY_LFO: f32 = 18583.0;
/// Low/High gain in dB over 0..200 (0x800009c8 / 0x80000970).
const SHELF_DB: [f32; 21] = [
    -19.0, -17.1, -15.2, -13.3, -11.4, -9.5, -7.6, -5.7, -3.8, -1.9, 0.0, 1.2, 2.4, 3.6, 4.8, 6.0,
    7.2, 8.4, 9.6, 10.8, 12.0,
];
/// Fixed ROM filters at 44.1 kHz.
const PEAK: Coefficients = Coefficients {
    b0: 1.009_24,
    b1: -1.944_088,
    b2: 0.935_354,
    a1: -1.944_088,
    a2: 0.944_593,
};
const BASS: Coefficients = Coefficients {
    b0: 1.002_557,
    b1: -0.977_693,
    b2: 0.0,
    a1: -0.980_25,
    a2: 0.0,
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Layout {
    upper: bool,
    size: i32,
}

#[derive(Debug, Clone, Copy)]
struct Network {
    layout: Layout,
    fir_delay: [usize; 20],
    fir_gain: [f32; 20],
    fir_ms: f32,
    ap_delay: [usize; 6],
    ap_gain: f32,
    comb_gain: [f32; 4],
    output: f32,
    direct: f32,
    predelay_to_net: f32,
}

impl Network {
    fn new(decay: i32, size: i32, sample_rate: f32) -> Self {
        let upper = decay > 100;
        let range = usize::from(upper);
        let (d, count) = if upper {
            ((decay - 101) as f32, 100)
        } else {
            (decay as f32, 101)
        };
        let s = size as f32 / 100.0;
        let lerp = |pair: &[[f32; 11]; 2]| {
            let a = common::table(&pair[0], count, d);
            let b = common::table(&pair[1], count, d);
            a + s * (b - a)
        };
        let tables = &DECAY[range];
        let time = lerp(&tables.time);
        let mut fir_delay = [1; 20];
        let mut fir_ms = 0.0;
        for (delay, &(t0, t1)) in fir_delay.iter_mut().zip(&FIR_MS[range]) {
            fir_ms = t0 + s * t1;
            *delay = common::samples((44.1 * fir_ms).trunc(), sample_rate);
        }
        let mut ap_delay = [1; 6];
        for (delay, &(t0, t1)) in ap_delay.iter_mut().zip(&AP_MS[range]) {
            *delay = common::samples((44.1 * (t0 + s * t1)).trunc(), sample_rate);
        }
        let mut comb_gain = [0.0; 4];
        for (gain, &(t0, t1)) in comb_gain.iter_mut().zip(&COMB) {
            *gain = if time > 0.0 { t1.powf(t0 / time) } else { 0.0 };
        }
        let direct = if upper {
            crate::zoom::curve(0.5, 0.0, 100, d, 5)
        } else {
            0.0
        };
        Self {
            layout: Layout { upper, size },
            fir_delay,
            fir_gain: FIR_GAIN[range],
            fir_ms,
            ap_delay,
            ap_gain: lerp(&tables.allpass),
            comb_gain,
            output: 2.0 * lerp(&tables.output),
            direct,
            predelay_to_net: if upper { 1.0 } else { 0.0 },
        }
    }
}

struct Channel {
    low: Biquad,
    high: Biquad,
    bass: Biquad,
}

impl Channel {
    #[inline]
    fn process(&mut self, x: f32) -> f32 {
        self.bass.process(self.high.process(self.low.process(x)))
    }
    fn clear(&mut self) {
        self.low.clear();
        self.high.clear();
        self.bass.clear();
    }
}

struct SpaceHole {
    sample_rate: f32,
    scale: f32,
    input_block: Biquad,
    peak: Biquad,
    feedback_block: Biquad,
    input_line: DelayLine,
    fir_line: DelayLine,
    allpasses: [DelayLine; 6],
    combs: [DelayLine; 4],
    chorus_line: DelayLine,
    comb_length: [usize; 4],
    taps_left: [(usize, usize); 4],
    taps_right: [(usize, usize); 4],
    chorus_left: f32,
    chorus_right: f32,
    modulation_span: f32,
    applied: Network,
    pending: Network,
    ap_gain: Ramp,
    comb_gain: [Ramp; 4],
    output: Ramp,
    direct: Ramp,
    predelay_to_net: Ramp,
    feedback_first: Ramp,
    feedback_last: Ramp,
    feedback: f32,
    previous_first: f32,
    previous_last: f32,
    predelay: f32,
    predelay_target: f32,
    predelay_mute: TapMute,
    network_mute: TapMute,
    depth: Smoother,
    predelay_phase: f32,
    predelay_increment: f32,
    chorus_phase: f32,
    chorus_increment: f32,
    wet: Glide,
    dry: Glide,
    left: Channel,
    right: Channel,
    last: Option<Values>,
}

impl SpaceHole {
    fn new(sample_rate: f32) -> Self {
        let scale = sample_rate / ORIGINAL_RATE;
        let line = |at_44k: f32| DelayLine::new((at_44k * scale).ceil() as usize + 8);
        let identity = Biquad::new(Coefficients::IDENTITY);
        let channel = || Channel {
            low: identity,
            high: identity,
            bass: Biquad::new(BASS.resample(ORIGINAL_RATE, sample_rate)),
        };
        let max_fir = FIR_MS
            .iter()
            .map(|taps| taps[19].0 + taps[19].1)
            .fold(0.0, f32::max);
        let max_ap = |i: usize| {
            AP_MS
                .iter()
                .map(|taps| taps[i].0 + taps[i].1)
                .fold(0.0, f32::max)
                * 44.1
        };
        let network = Network::new(50, 29, sample_rate);
        let ramp = |value: f32| Ramp::new(value, STEP, sample_rate);
        let mut effect = Self {
            sample_rate,
            scale,
            input_block: Biquad::new(Coefficients::highpass1(sample_rate, 10.0)),
            peak: Biquad::new(PEAK.resample(ORIGINAL_RATE, sample_rate)),
            feedback_block: Biquad::new(Coefficients::highpass1(sample_rate, 10.0)),
            input_line: line(1000.0 * 44.1 + 1.0 + 1025.0),
            fir_line: line(max_fir * 44.1 + 1.0),
            allpasses: std::array::from_fn(|i| line(max_ap(i) + 1.0)),
            combs: std::array::from_fn(|i| line(COMB_LENGTH[i])),
            chorus_line: line(504.0 + 1025.0),
            comb_length: COMB_LENGTH.map(|length| common::samples(length, sample_rate)),
            taps_left: TAPS_LEFT.map(|(comb, delay)| (comb, common::samples(delay, sample_rate))),
            taps_right: TAPS_RIGHT.map(|(comb, delay)| (comb, common::samples(delay, sample_rate))),
            chorus_left: 441.0 * scale,
            chorus_right: 504.0 * scale,
            modulation_span: 1024.0 * scale,
            applied: network,
            pending: network,
            ap_gain: ramp(0.0),
            comb_gain: [ramp(0.0); 4],
            output: ramp(0.0),
            direct: ramp(0.0),
            predelay_to_net: ramp(0.0),
            feedback_first: ramp(0.0),
            feedback_last: ramp(0.0),
            feedback: 0.0,
            previous_first: 0.0,
            previous_last: 0.0,
            predelay: scale,
            predelay_target: scale,
            predelay_mute: TapMute::new(220.0, 220.0, sample_rate),
            network_mute: TapMute::new(396.9, 220.5, sample_rate),
            depth: Smoother::new(0.0, 0.387, sample_rate),
            predelay_phase: 0.0,
            predelay_increment: PREDELAY_LFO / 4_294_967_296.0 / scale,
            chorus_phase: 0.0,
            chorus_increment: 0.0,
            wet: Glide::new(0.0, 705.6, sample_rate),
            dry: Glide::new(1.0, 705.6, sample_rate),
            left: channel(),
            right: channel(),
            last: None,
        };
        effect.apply_network(network, true);
        effect
    }

    /// Install a network: delays always, gains either at once (while muted or
    /// at start) or through their ramps.
    fn apply_network(&mut self, network: Network, jump: bool) {
        self.applied = network;
        let set = |ramp: &mut Ramp, value: f32| {
            if jump {
                ramp.jump(value)
            } else {
                ramp.set(value)
            }
        };
        set(&mut self.ap_gain, network.ap_gain);
        for (ramp, &gain) in self.comb_gain.iter_mut().zip(&network.comb_gain) {
            set(ramp, gain);
        }
        set(&mut self.output, network.output);
        set(&mut self.direct, network.direct);
        set(&mut self.predelay_to_net, network.predelay_to_net);
        let (first, last) = if network.layout.upper {
            (self.feedback, 0.0)
        } else {
            (0.0, self.feedback)
        };
        set(&mut self.feedback_first, first);
        set(&mut self.feedback_last, last);
        let hold = ((network.fir_ms + 10.0).max(130.0) * 0.001 * self.sample_rate) as u32;
        self.network_mute.set_hold_samples(hold);
    }

    fn set_shelves(&mut self, low: f32, high: f32) {
        let sr = self.sample_rate;
        let low_db = common::table(&SHELF_DB, 201, low);
        let low = common::shelf(
            sr,
            350.0,
            low_db,
            if low < 100.0 { 1.6 } else { 1.5 },
            false,
        );
        let high_db = common::table(&SHELF_DB, 201, high);
        let high = if high < 100.0 {
            common::shelf(sr, 3200.0, high_db, 1.6, true)
        } else {
            common::shelf(sr, 1400.0, high_db, 3.9, true)
        };
        for channel in [&mut self.left, &mut self.right] {
            channel.low.coefficients = low;
            channel.high.coefficients = high;
        }
    }
}

impl Effect for SpaceHole {
    fn set(&mut self, values: &Values, _bpm: f32) {
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

        if changed(2) {
            let mix = value(2, 100.0);
            let wet = common::table(&WET, 101, mix) * WET_SCALE;
            let dry = common::table(&DRY, 101, mix);
            if first {
                self.wet.jump(wet);
                self.dry.jump(dry);
            } else {
                self.wet.set(wet);
                self.dry.set(dry);
            }
        }
        if changed(4) {
            let depth = value(4, 100.0) / 100.0;
            if first {
                self.depth.jump(depth);
            } else {
                self.depth.set(depth);
            }
        }
        if changed(5) {
            let speed = value(5, 100.0);
            let increment = if speed < 0.5 {
                1.0
            } else {
                common::table(&SPEED, 101, speed)
            };
            self.chorus_increment = increment / 4_294_967_296.0 / self.scale;
        }
        if changed(7) || changed(8) {
            self.set_shelves(value(7, 200.0), value(8, 200.0));
        }
        if changed(1) {
            let ms = value(1, 1000.0);
            self.predelay_target = ((44.1 * ms).trunc() + 1.0) * self.scale;
            if first {
                self.predelay = self.predelay_target;
            } else if self.predelay_target != self.predelay {
                self.predelay_mute.request(false);
            }
        }
        if changed(0) || changed(3) || changed(6) {
            self.feedback = 0.99 * value(3, 100.0) / 100.0;
            let decay = value(0, 200.0).round() as i32;
            let size = value(6, 100.0).round() as i32;
            let network = Network::new(decay, size, self.sample_rate);
            self.pending = network;
            if first {
                self.apply_network(network, true);
            } else if network.layout == self.applied.layout {
                self.apply_network(network, false);
            } else {
                let range_changed = network.layout.upper != self.applied.layout.upper;
                self.network_mute.request(range_changed);
            }
        }
        self.last = Some(*values);
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        if self.network_mute.tick() {
            self.apply_network(self.pending, true);
        }
        let mute = self.network_mute.gain;
        if self.predelay_mute.tick() {
            self.predelay = self.predelay_target;
        }

        let depth = self.depth.tick() * self.modulation_span;
        common::advance(&mut self.predelay_phase, self.predelay_increment);
        common::advance(&mut self.chorus_phase, self.chorus_increment);
        let predelay_mod = common::parabolic(self.predelay_phase) * depth;
        let chorus_mod_left = common::parabolic(self.chorus_phase) * depth;
        let chorus_mod_right = common::parabolic(self.chorus_phase + 0.5) * depth;

        // Input, F.B feedback (one frame late), DC blocks.
        let x = self
            .peak
            .process(self.input_block.process(0.5 * (left + right)));
        let fed = x + mute
            * (self.feedback_first.tick() * self.previous_first
                + self.feedback_last.tick() * self.previous_last);
        let input = self.feedback_block.process(fed);

        // Modulated pre-delay tap.
        let pre = mute
            * self.predelay_mute.gain
            * common::tap_linear(&self.input_line, self.predelay + predelay_mod);

        // Early-reflection FIR and diffusion.
        let network = &self.applied;
        let mut fir = 0.0;
        for (&delay, &gain) in network.fir_delay.iter().zip(&network.fir_gain) {
            fir += gain * common::tap(&self.fir_line, delay);
        }
        let first_tap = common::tap(&self.fir_line, network.fir_delay[0]);
        let last_tap = common::tap(&self.fir_line, network.fir_delay[19]);
        let g = self.ap_gain.tick();
        let mut diffused = mute * fir;
        for (line, &delay) in self.allpasses.iter_mut().zip(&network.ap_delay) {
            let delayed = common::tap(line, delay);
            let w = flush(diffused + g * delayed);
            line.write(w);
            diffused = delayed - g * w;
        }
        let net = mute * (self.predelay_to_net.tick() * pre + diffused);

        // Combs and their output taps.
        let read_side = |taps: &[(usize, usize); 4], combs: &[DelayLine; 4]| {
            taps.iter()
                .map(|&(comb, delay)| common::tap(&combs[comb], delay))
                .sum::<f32>()
        };
        let comb_left = read_side(&self.taps_left, &self.combs);
        let comb_right = read_side(&self.taps_right, &self.combs);
        let comb_input = 0.19 * net;
        for ((line, &length), gain) in self
            .combs
            .iter_mut()
            .zip(&self.comb_length)
            .zip(&mut self.comb_gain)
        {
            let fed_back = gain.tick() * mute * common::tap(line, length);
            line.write(flush(comb_input + fed_back));
        }
        let output = self.output.tick();
        let direct = self.direct.tick();
        let shared = direct * net;
        let tail_left = 0.368_362 * (output * comb_left + shared) + direct * pre;
        let tail_right = 0.368_362 * (output * comb_right + shared) + direct * pre;

        // Chorus line, read in opposite phases.
        let chorus_left = common::tap_linear(&self.chorus_line, self.chorus_left + chorus_mod_left);
        let chorus_right =
            common::tap_linear(&self.chorus_line, self.chorus_right + chorus_mod_right);
        self.chorus_line
            .write(flush(0.5 * (tail_left + tail_right)));

        self.input_line.write(input);
        self.fir_line.write(pre);
        self.previous_first = first_tap;
        self.previous_last = last_tap;

        let wet = self.wet.tick() * mute;
        let dry = self.dry.tick();
        let out_left = self.left.process(chorus_left + tail_left);
        let out_right = self.right.process(chorus_right + tail_right);
        [dry * left + wet * out_left, dry * right + wet * out_right]
    }

    fn clear(&mut self) {
        self.input_block.clear();
        self.peak.clear();
        self.feedback_block.clear();
        self.input_line.clear();
        self.fir_line.clear();
        for line in self.allpasses.iter_mut().chain(self.combs.iter_mut()) {
            line.clear();
        }
        self.chorus_line.clear();
        self.left.clear();
        self.right.clear();
        self.previous_first = 0.0;
        self.previous_last = 0.0;
        if self.network_mute.busy() {
            self.network_mute.reset();
            self.apply_network(self.pending, true);
        }
        if self.predelay_mute.busy() {
            self.predelay_mute.reset();
            self.predelay = self.predelay_target;
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
    fn decay_editor_matches_the_tables() {
        // Decay 50, Size 29: T = 450 + 0.29 * (1300 - 450) ms.
        let network = Network::new(50, 29, 44_100.0);
        let time = 450.0 + 0.29 * 850.0;
        assert!((network.comb_gain[0] - 0.01238f32.powf(101.103_47 / time)).abs() < 1e-5);
        assert!((network.ap_gain - (0.7 + 0.29 * 0.1)).abs() < 1e-5);
        assert!((network.output - 2.0 * (1.0 + 0.29 * -0.08)).abs() < 1e-5);
        assert_eq!(network.fir_delay[0], 88);
        assert_eq!(
            network.fir_delay[19],
            (44.1f32 * (324.46 + 0.29 * 432.54)).trunc() as usize
        );
        assert_eq!(network.direct, 0.0);
        // Upper range: direct terms 0.5 (d/99)^1.41.
        let upper = Network::new(200, 100, 44_100.0);
        assert!(upper.layout.upper);
        assert!((upper.direct - 0.5).abs() < 1e-5);
        assert!((upper.fir_ms - 757.0).abs() < 1e-3);
        assert_eq!(upper.fir_gain[0], 0.99);
    }

    #[test]
    fn predelay_sets_the_first_arrival() {
        let sample_rate = 48_000.0;
        let mut effect = SpaceHole::new(sample_rate);
        let mut values = defaults();
        values[0] = 200.0; // upper range: the pre-delayed signal reaches the output
        values[1] = 100.0; // 100 ms
        values[2] = 100.0; // wet only
        values[4] = 0.0; // no modulation
        effect.set(&values, 120.0);
        let mut first = None;
        for i in 0..20_000 {
            let x = if i == 0 { 1.0 } else { 0.0 };
            let [l, r] = effect.process([x, x]);
            if first.is_none() && (l.abs() > 1e-3 || r.abs() > 1e-3) {
                first = Some(i);
            }
        }
        let expected = (4410.0 + 1.0) * sample_rate / 44_100.0;
        let first = first.expect("no wet output") as f32;
        assert!((first - expected).abs() < 3.0, "{first} vs {expected}");
    }

    #[test]
    fn mix_and_level_at_defaults() {
        let mut effect = SpaceHole::new(48_000.0);
        effect.set(&defaults(), 120.0);
        // dry passes at unity at Mix 40 (dry table is 1 below 50)
        let [l, _] = effect.process([0.5, 0.5]);
        assert!((l - 0.5).abs() < 1e-3);
        assert!((effect.wet.current - 0.8 * WET_SCALE).abs() < 1e-4);
    }

    #[test]
    fn no_runaway_with_maximum_feedback() {
        for decay in [100.0, 200.0] {
            let mut effect = SpaceHole::new(44_100.0);
            let mut values = defaults();
            values[0] = decay;
            values[3] = 100.0;
            values[6] = 100.0;
            values[7] = 200.0;
            values[8] = 200.0;
            effect.set(&values, 120.0);
            let mut noise = crate::dsp::Noise::new(3);
            let mut peak = 0.0f32;
            for i in 0..44_100 * 8 {
                let x = if i < 44_100 {
                    noise.bipolar() * 0.5
                } else {
                    0.0
                };
                let [l, r] = effect.process([x, x]);
                assert!(l.is_finite() && r.is_finite());
                peak = peak.max(l.abs()).max(r.abs());
            }
            assert!(peak < 16.0, "decay {decay}: peak {peak}");
        }
    }

    #[test]
    fn speed_table_is_about_one_hertz_at_maximum() {
        let hz = SPEED[10] * 44_100.0 / 4_294_967_296.0;
        assert!((hz - 1.0).abs() < 0.01, "{hz}");
        let hz = PREDELAY_LFO * 44_100.0 / 4_294_967_296.0;
        assert!((hz - 0.19).abs() < 0.01, "{hz}");
    }
}
