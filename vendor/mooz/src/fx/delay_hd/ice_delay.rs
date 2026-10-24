//! ICE Delay (ICEDLY.ZDL): a normal delay blended with a reversed,
//! pitch-shifted "ice" delay, both with optional diffusion, after the
//! recovered `Fx_DLY_IceDelay` callback and the decoded editors.
//!
//! As in the callback: the input passes a first-order shelf (−3 dB at the top)
//! and is summed to mono. Both paths take `input + F.B·(ice·A + dry·B)`
//! (BLEND tables). Path A (ice): 2.6 kHz low-pass → DAMP (`dampCoeTbl`) →
//! written forward into a buffer of `(SLICE+1)·TIME` samples; a read head moves
//! backwards at the INTVL ratio (`interval_tbl`, −Oct…2Oct; Uni plays
//! reversed at pitch), interpolated as the callback does, with 20 ms read and
//! write windows that duck where the heads would collide and apply a pending
//! length change at the crossing. A transient detector (100 Hz high-pass peak,
//! 0.01 threshold, 0.001 re-arm) restarts the reverse head 900 samples after
//! an onset, just behind it. Path B: DAMP → normal delay (gliding TIME).
//! Each path then passes the SMEAR diffuser (eight nested allpasses of
//! 59…97 samples; coefficients 0.01 + 0.34·s/20 and 0.65·s/20, bypass at 0).
//! Output: HPF (first-order, `HipassFreqTbl`) of `ice·A + dry·B`, mixed
//! `MixWet·wet + MixDry·filtered input`.
//!
//! Editors: TIME `p + 60` ms (60–1300) or tempo notes, SLICE SHORT/LONG
//! doubles the ice buffer, F.B `FB_tbl`, DAMP with the editor's 16 ms mute.
//! Deviation: a TIME change glides path B (as for tempo changes) and hands the
//! new ice length to the callback's pending-length mechanism rather than
//! muting both taps for the new delay time. Both delay writes pass a soft
//! safety limiter above ±2 (the callback has no clip at 100 % feedback).
use super::{
    common::{
        Cascade, Glide, Kind, MuteSwap, Stage, design, pole, rom, rom_set, safety, samples, table,
        time_ms, words,
    },
    tables::ICE_DAMP,
    trem_delay::{FB_TBL, HPF_HZ, HPF_LABELS, MIX_DRY, MIX_WET},
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{DelayLine, flush},
    zoom::{DEFAULT_STEP, Ramp, is_synced},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_ice_delay",
    name: "ICE Delay",
    category: Category::Delay,
    zdl: "ICEDLY.ZDL",
    description: "Delay blended with reversed pitch-shifted slices",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "TIME",
            max: 1255,
            default: 440,
            display: Display::DelayTime {
                threshold: 1240,
                offset: 60,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 64,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "MIX",
            max: 100,
            default: 60,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "INTVL",
            max: 30,
            default: 23,
            display: Display::labels(&[
                "-Oct", "-M7", "-m7", "-M6", "-m6", "-P5", "-Tri", "-P4", "-M3", "-m3", "-M2",
                "-m2", "-50c", "-25c", "Uni", "+25c", "+50c", "+m2", "+M2", "+m3", "+M3", "+P4",
                "+Tri", "+P5", "+m6", "+M6", "+m7", "+M7", "+Oct", "Oc+5", "2Oct",
            ]),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "SLICE",
            max: 1,
            default: 1,
            display: Display::labels(&["SHORT", "LONG"]),
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "BLEND",
            max: 20,
            default: 12,
            display: Display::INT,
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "SMEAR",
            max: 20,
            default: 7,
            display: Display::INT,
        },
        // values[7] is firmware parameter word 9
        ParamInfo {
            name: "DAMP",
            max: 10,
            default: 2,
            display: Display::INT,
        },
        // values[8] is firmware parameter word 10
        ParamInfo {
            name: "HPF",
            max: 20,
            default: 1,
            display: Display::labels(&HPF_LABELS),
        },
    ],
};

const MAX_MS: f32 = 1300.0;
const THRESHOLD: u16 = 1240;
const INTERVAL: [f64; 31] = [
    0.5,
    0.529_731_547_179_65,
    0.561_231_024_154_69,
    0.594_603_557_501_36,
    0.629_960_524_947_44,
    0.667_419_927_085_02,
    std::f64::consts::FRAC_1_SQRT_2,
    0.749_153_538_438_34,
    0.793_700_525_984_1,
    0.840_896_415_253_72,
    0.890_898_718_140_34,
    0.943_874_312_681_69,
    0.971_531_941_153_61,
    0.985_663_198_640_19,
    1.0,
    1.014_545_334_937_52,
    1.029_302_236_643_49,
    1.059_463_094_359_3,
    1.122_462_048_309_37,
    1.189_207_115_002_72,
    1.259_921_049_894_87,
    1.334_839_854_170_03,
    std::f64::consts::SQRT_2,
    1.498_307_076_876_68,
    1.587_401_051_968_2,
    1.681_792_830_507_43,
    1.781_797_436_280_68,
    1.887_748_625_363_39,
    2.0,
    2.996_614_153_753_36,
    4.0,
];
const BLEND_DRY: [f32; 21] = [
    1.0,
    0.980_618_3,
    0.944_060_9,
    0.894_334_55,
    0.855_066_7,
    0.815_642_8,
    0.776_247_1,
    0.737_055_2,
    0.697_429,
    0.657_657_86,
    0.618_016_4,
    0.578_096,
    0.538_269_76,
    0.498_884_5,
    0.449_262_32,
    0.399_944_75,
    0.350_348_3,
    0.300_261_74,
    0.250_899_6,
    0.200_909_29,
    0.002_561_53,
];
const BLEND_ICE: [f32; 21] = [
    0.0,
    0.019_975_61,
    0.049_831_05,
    0.099_770_01,
    0.139_636_84,
    0.179_680_1,
    0.219_533_09,
    0.259_716_8,
    0.299_916_24,
    0.339_625_27,
    0.379_751_95,
    0.419_758_98,
    0.459_727,
    0.499_459_18,
    0.549_540_9,
    0.599_791_1,
    0.650_129_7,
    0.700_648_2,
    0.749_894_2,
    0.799_834_25,
    1.0,
];
const DIFFUSION: [usize; 8] = [59, 73, 79, 61, 67, 71, 89, 97];

/// The SMEAR diffuser (eight nested allpasses on 100-sample lanes).
struct Smear {
    lanes: [DelayLine; 8],
    delays: [usize; 8],
}

impl Smear {
    fn new(sample_rate: f32) -> Self {
        let n = |d: usize| (samples(d as f32, sample_rate).round() as usize).max(1);
        Self {
            lanes: std::array::from_fn(|i| DelayLine::new(n(DIFFUSION[i]) + 2)),
            delays: DIFFUSION.map(n),
        }
    }
    #[inline]
    fn process(&mut self, x: f32, g1: f32, g2: f32) -> f32 {
        let t: [f32; 8] = std::array::from_fn(|i| self.lanes[i].read(self.delays[i] - 1));
        let a = t[1] - g2 * t[0];
        let b = t[2] - g2 * a;
        let mut prev = b - g1 * x;
        self.lanes[0].write(flush(g1 * prev + x));
        self.lanes[1].write(flush(g2 * a + t[0]));
        self.lanes[2].write(flush(g2 * b + a));
        for (lane, tap) in self.lanes[3..].iter_mut().zip(&t[3..]) {
            let v = tap - g2 * prev;
            lane.write(flush(g2 * v + prev));
            prev = v;
        }
        prev
    }
    fn clear(&mut self) {
        for lane in &mut self.lanes {
            lane.clear();
        }
    }
}

struct IceDelay {
    sample_rate: f32,
    input_shelf: [Stage; 2],
    // detector
    detector_hp: Stage,
    envelope: f32,
    envelope_decay: f32,
    armed: bool,
    countdown: i32,
    countdown_length: i32,
    // ice path
    ice_lp: Stage,
    ice_damp: Cascade<4>,
    ring: Vec<f32>,
    length: usize,
    pending_length: usize,
    write: usize,
    phase: f64,
    ratio: f64,
    read_gain: f32,
    write_gain: f32,
    window_step: f32,
    ice_out: f32,
    // normal path
    damp: Cascade<4>,
    line: DelayLine,
    time: Glide,
    dry_out: f32,
    smear: [Smear; 2],
    smear_coefficients: (f32, f32),
    smear_mix: (Ramp, Ramp),
    damp_swap: MuteSwap,
    damp_pending: [[f32; 5]; 4],
    hpf: Stage,
    feedback: Ramp,
    blend: (Ramp, Ramp),
    wet: Ramp,
    dry: Ramp,
    last: [f32; 9],
    bpm: f32,
}

impl IceDelay {
    fn new(sample_rate: f32) -> Self {
        let sr = sample_rate;
        let shelf = || Stage::from(rom([0.777_428_6, -0.301_606_9, 0.0, 0.524_178_3, 0.0], sr));
        let ring = (2.0 * MAX_MS * 0.001 * sr) as usize + 8;
        let mut effect = Self {
            sample_rate,
            input_shelf: [shelf(), shelf()],
            detector_hp: Stage::from(rom([0.992_926_5, -0.992_926_5, 0.0, 0.985_853, 0.0], sr)),
            envelope: 0.0,
            envelope_decay: pole(0.999_939, sr),
            armed: true,
            countdown: 0,
            countdown_length: samples(900.0, sr).round() as i32,
            ice_lp: Stage::from(rom([0.157_799_8, 0.157_799_8, 0.0, 0.684_400_4, 0.0], sr)),
            ice_damp: Cascade::default(),
            ring: vec![0.0; ring],
            length: 1,
            pending_length: 0,
            write: 0,
            phase: 0.0,
            ratio: 1.0,
            read_gain: 0.0,
            write_gain: 0.0,
            window_step: 0.001_112 * 44_100.0 / sr,
            ice_out: 0.0,
            damp: Cascade::default(),
            line: DelayLine::with_seconds(MAX_MS * 0.001 + 0.01, sr),
            time: Glide::standard(0.0, sr),
            dry_out: 0.0,
            smear: [Smear::new(sr), Smear::new(sr)],
            smear_coefficients: (0.1, 0.2),
            smear_mix: (
                Ramp::new(0.0, DEFAULT_STEP, sr),
                Ramp::new(1.0, DEFAULT_STEP, sr),
            ),
            damp_swap: MuteSwap::new(sr),
            damp_pending: [Stage::IDENTITY; 4],
            hpf: Stage::new(Stage::IDENTITY),
            feedback: Ramp::new(0.0, DEFAULT_STEP, sr),
            blend: (
                Ramp::new(0.0, DEFAULT_STEP, sr),
                Ramp::new(1.0, DEFAULT_STEP, sr),
            ),
            wet: Ramp::new(0.0, DEFAULT_STEP, sr),
            dry: Ramp::new(1.0, DEFAULT_STEP, sr),
            last: [f32::NAN; 9],
            bpm: 0.0,
        };
        effect.set(&crate::Model::IceDelay.defaults(), 120.0);
        effect.time.value = effect.time.target;
        for ramp in [
            &mut effect.feedback,
            &mut effect.wet,
            &mut effect.dry,
            &mut effect.blend.0,
            &mut effect.blend.1,
            &mut effect.smear_mix.0,
            &mut effect.smear_mix.1,
        ] {
            ramp.jump(ramp.target);
        }
        effect
    }

    /// Transient detector on the mono input, once per sample.
    #[inline]
    fn detect(&mut self, x: f32) {
        self.envelope =
            (self.envelope * self.envelope_decay).max(self.detector_hp.process(x).abs());
        if self.countdown == 0 && self.armed && self.envelope > 0.01 {
            self.armed = false;
            self.countdown = self.countdown_length;
        } else if self.envelope < 0.001 {
            self.armed = true;
        }
        if self.countdown > 0 {
            self.countdown -= 1;
            if self.countdown == 0 {
                // restart the reverse head just behind the onset
                let length = self.length as i64;
                let mut offset = 1 + i64::from(self.countdown_length) + length - self.write as i64;
                while offset > length {
                    offset -= length;
                }
                self.phase = offset.max(0) as f64;
            }
        }
    }

    /// Phase window of the ice read head; returns the interpolated tap.
    #[inline]
    fn ice_tap(&mut self) -> f32 {
        let length = self.length;
        self.phase += self.ratio;
        if self.phase > length as f64 {
            self.phase %= length as f64;
        }
        let whole = self.phase.floor();
        let fraction = (self.phase - whole) as f32;
        let target = length.saturating_sub(whole as usize);
        let advance = (900.0 * self.ratio) as usize * self.countdown_length as usize / 900;
        let end = advance + self.countdown_length as usize + self.write;
        let safe = if end < length {
            target > end || target < self.write
        } else {
            self.write > target && target > end - length
        };
        if safe && self.countdown <= 0 {
            self.write_gain = (self.write_gain + self.window_step).min(1.0);
            self.read_gain = (self.read_gain + self.window_step).min(1.0);
        } else {
            self.read_gain = (self.read_gain - self.window_step).max(0.0);
            if self.pending_length != 0 {
                self.write_gain = (self.write_gain - self.window_step).max(0.0);
            }
        }
        // the heads approach by 1 + ratio per sample: "meet" within that step
        let reach = 2 + self.ratio as usize;
        if self.pending_length != 0 && self.write.abs_diff(target) <= reach {
            self.length = self.pending_length;
            self.pending_length = 0;
            self.phase = 0.0;
        }
        let tap0 = self.ring[target];
        let tap1 = self.ring[if target >= length { 0 } else { target + 1 }];
        self.read_gain * (tap0 + fraction * (tap0 - tap1))
    }
}

impl Effect for IceDelay {
    fn set(&mut self, values: &Values, bpm: f32) {
        if values[..9] == self.last && bpm == self.bpm {
            return;
        }
        let sr = self.sample_rate;
        let first = self.last[0].is_nan();
        let tempo = bpm != self.bpm && is_synced(values[0], THRESHOLD);
        if values[0] != self.last[0] || values[4] != self.last[4] || tempo {
            let ms = time_ms(values[0], THRESHOLD, 60.0, MAX_MS, bpm);
            let delay = ms * 0.001 * sr;
            self.time.target = delay;
            let slice = 1 + usize::from(values[4].round() >= 1.0);
            let length = ((slice as f32 * delay) as usize).clamp(2, self.ring.len() - 2);
            if first {
                self.length = length;
            } else if length != self.length {
                self.pending_length = length;
            }
        }
        self.feedback
            .set(table(&FB_TBL, values[1].round().clamp(0.0, 100.0), 101));
        let mix = values[2].round().clamp(0.0, 100.0);
        self.wet.set(table(&MIX_WET, mix, 101));
        self.dry.set(table(&MIX_DRY, mix, 101));
        self.ratio = INTERVAL[values[3].round().clamp(0.0, 30.0) as usize];
        let blend = values[5].round().clamp(0.0, 20.0) as usize;
        self.blend.0.set(BLEND_ICE[blend]);
        self.blend.1.set(BLEND_DRY[blend]);
        let smear = values[6].round().clamp(0.0, 20.0);
        if smear == 0.0 {
            self.smear_mix.0.set(0.0);
            self.smear_mix.1.set(1.0);
        } else {
            self.smear_coefficients = (0.01 + 0.34 * smear / 20.0, 0.65 * smear / 20.0);
            self.smear_mix.0.set(1.0);
            self.smear_mix.1.set(0.0);
        }
        if values[7] != self.last[7] {
            let damp = values[7].round().clamp(0.0, 10.0) as usize;
            self.damp_pending = rom_set(&ICE_DAMP[damp], sr);
            if first {
                self.ice_damp.load(&self.damp_pending);
                self.damp.load(&self.damp_pending);
            } else {
                self.damp_swap.start();
            }
        }
        if values[8] != self.last[8] {
            let hpf = values[8].round().clamp(0.0, 20.0) as usize;
            self.hpf.c = if hpf == 0 {
                Stage::IDENTITY
            } else {
                words(design(Kind::Highpass1, sr, HPF_HZ[hpf], 0.0, 0.7))
            };
        }
        self.last.copy_from_slice(&values[..9]);
        self.bpm = bpm;
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        let filtered = [
            self.input_shelf[0].process(frame[0]),
            self.input_shelf[1].process(frame[1]),
        ];
        let mono = 0.5 * (filtered[0] + filtered[1]);
        self.detect(0.5 * (frame[0] + frame[1]));
        let feedback = self.feedback.tick();
        let ice = self.blend.0.tick();
        let dry_blend = self.blend.1.tick();
        let (damp_gain, swap) = self.damp_swap.tick();
        if swap {
            self.ice_damp.load(&self.damp_pending);
            self.damp.load(&self.damp_pending);
        }
        let (g1, g2) = self.smear_coefficients;
        let diffused = self.smear_mix.0.tick();
        let direct = self.smear_mix.1.tick();

        // path A: reversed pitch-shifted slices
        self.write += 1;
        if self.write > self.length {
            self.write = 0;
        }
        let input_a = mono + feedback * (ice * self.ice_out + dry_blend * self.dry_out);
        let a = self.ice_damp.process(self.ice_lp.process(input_a));
        self.ring[self.write] = safety(flush(self.write_gain * damp_gain * a));
        let tap = self.ice_tap();
        let smeared = if diffused != 0.0 {
            self.smear[1].process(tap, g1, g2)
        } else {
            0.0
        };
        self.ice_out = direct * tap + diffused * smeared;

        // path B: normal delay
        let input_b = mono + feedback * (ice * self.ice_out + dry_blend * self.dry_out);
        self.line
            .write(safety(flush(damp_gain * self.damp.process(input_b))));
        let tap = self.line.read_linear(self.time.tick());
        let smeared = if diffused != 0.0 {
            self.smear[0].process(tap, g1, g2)
        } else {
            0.0
        };
        self.dry_out = direct * tap + diffused * smeared;

        let wet = self
            .hpf
            .process(dry_blend * self.dry_out + ice * self.ice_out);
        let wet_gain = self.wet.tick();
        let dry_gain = self.dry.tick();
        [
            wet_gain * wet + dry_gain * filtered[0],
            wet_gain * wet + dry_gain * filtered[1],
        ]
    }

    fn clear(&mut self) {
        for s in &mut self.input_shelf {
            s.clear();
        }
        self.detector_hp.clear();
        self.envelope = 0.0;
        self.armed = true;
        self.countdown = 0;
        self.ice_lp.clear();
        self.ice_damp.clear();
        self.ring.fill(0.0);
        self.ice_out = 0.0;
        self.damp.clear();
        self.line.clear();
        self.dry_out = 0.0;
        for s in &mut self.smear {
            s.clear();
        }
        self.hpf.clear();
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(IceDelay::new(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unison_plays_the_buffer_backwards() {
        let sr = 44_100.0;
        let mut effect = IceDelay::new(sr);
        let mut values = crate::Model::IceDelay.defaults();
        values[0] = 40.0; // 100 ms
        values[3] = 14.0; // Uni
        values[4] = 0.0;
        effect.set(&values, 120.0);
        // the new length waits for the heads to cross
        assert_eq!(effect.pending_length, 4410);
        for _ in 0..100_000 {
            effect.process([0.0, 0.0]);
        }
        assert_eq!((effect.length, effect.pending_length), (4410, 0));
        // read head moves one sample backwards per sample
        effect.phase = 1.0;
        let before = effect.phase;
        effect.process([0.0, 0.0]);
        assert!((effect.phase - before - 1.0).abs() < 1e-9);
        assert!((effect.ratio - 1.0).abs() < 1e-12);
    }

    #[test]
    fn ramp_input_comes_back_reversed() {
        let sr = 44_100.0;
        let mut effect = IceDelay::new(sr);
        let mut values = crate::Model::IceDelay.defaults();
        values[0] = 40.0;
        values[1] = 0.0;
        values[2] = 100.0;
        values[3] = 14.0;
        values[4] = 0.0;
        values[5] = 20.0; // ice only
        values[6] = 0.0;
        values[7] = 0.0;
        values[8] = 0.0;
        effect.set(&values, 120.0);
        let mut out = Vec::new();
        for i in 0..44_100 {
            // slow ramp: reversed playback falls where the input rises
            let x = 0.2 * ((i % 4410) as f32 / 4410.0);
            out.push(effect.process([x, x])[0]);
        }
        let falling = out[20_000..40_000]
            .windows(2)
            .filter(|w| w[1] < w[0] - 1e-6)
            .count();
        let rising = out[20_000..40_000]
            .windows(2)
            .filter(|w| w[1] > w[0] + 1e-6)
            .count();
        assert!(falling > rising, "{falling} {rising}");
    }

    #[test]
    fn max_feedback_stays_bounded() {
        let mut effect = IceDelay::new(96_000.0);
        let mut values = crate::Model::IceDelay.defaults();
        values[1] = 100.0;
        values[6] = 20.0;
        effect.set(&values, 120.0);
        for i in 0..96_000 * 4 {
            let x = if i % 20_000 < 1000 { 0.9 } else { 0.0 };
            let y = effect.process([x, x]);
            assert!(y[0].is_finite() && y[0].abs() < 16.0);
        }
    }
}
