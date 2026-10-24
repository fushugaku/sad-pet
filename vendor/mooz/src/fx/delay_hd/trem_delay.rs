//! TremDelay (TREMDLY.ZDL): a stereo delay whose repeats pass a gritty drive
//! stage and a damping filter, with a tremolo on the delayed signal, after the
//! recovered `Fx_DLY_TremDelay` callback and the decoded editors.
//!
//! Per channel, as in the callback: feedback `clip±1(F.B · tap)` → 20 Hz
//! one-pole high-pass → `+ input` → GRIT stage (clean branch through the
//! GRIT pre high-pass, drive branch `0.046·tanh(42.5·x)` → two Q 0.7
//! low-passes at the GRIT post corner → the two fixed ROM EQ biquads → 65 Hz
//! high-pass, mixed `DrvMix·drive + DryMix·clean`) → the four DAMP biquads
//! (ROM `dampCoeTbl`) → 2 s delay line. The tap (gliding TIME, the callback's
//! Q31 smoother) is the feedback source; the tremolo gain multiplies it after,
//! then the HPF first-order high-pass (`HipassFreqTbl`, Off/20–900 Hz) and the
//! output `0.75·MixWet·wet + MixDry·dry`.
//!
//! Editors: TIME is `p + 60` ms (60–1900) or tempo notes; the tremolo rate is
//! SPEED's ratio (`trmSpeedTbl`, 1/32…32 cycles) per delay time; LFO selects
//! TRI/SQR/SIN/RAMP/SAW (`wave_tbl`); DEPTH gain `1 − d + d·wave`, smoothed by
//! a 2.9 ms one-pole; F.B `FB_tbl · 0.8 · grit_fb[g]` (GRIT ≤ 3) or
//! `FB_tbl · grit_fb[g] · (F.B/101)^CV[g]`; MIX `MixWetTbl`/`MixDryTbl`; DAMP
//! and GRIT tables as the editors copy them, DAMP with the editor's 16 ms
//! mute around the swap. The Main filter designer is ported for the GRIT and
//! HPF filters; ROM biquads are re-derived at the host rate. Deviation: a TIME
//! change glides (as the original does for tempo changes) instead of the knob
//! path's mute → jump → reopen. The unused delay-LFO (depth 0 at init) is
//! omitted.
use super::{
    common::{
        Cascade, DelayLine, DurationRamp, Glide, Kind, MuteSwap, Q31, RAMP_SAMPLES, Settled, Stage,
        Stage1, clip1, design, flush, one_pole_at, rom, rom_set, table, time_ms, words,
    },
    tables::DAMP,
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::tanh,
    zoom::{DEFAULT_STEP, Ramp, is_synced},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_trem_delay",
    name: "TremDelay",
    category: Category::Delay,
    zdl: "TREMDLY.ZDL",
    description: "Gritty delay with a tremolo synced to the repeats",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "TIME",
            max: 1855,
            default: 300,
            display: Display::DelayTime {
                threshold: 1840,
                offset: 60,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 70,
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
            name: "LFO",
            max: 4,
            default: 0,
            display: Display::labels(&["TRI", "SQR", "SIN", "RAMP", "SAW"]),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "DEPTH",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "SPEED",
            max: 34,
            default: 19,
            display: Display::labels(&SPEED_LABELS),
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "DAMP",
            max: 10,
            default: 4,
            display: Display::INT,
        },
        // values[7] is firmware parameter word 9
        ParamInfo {
            name: "HPF",
            max: 20,
            default: 4,
            display: Display::labels(&HPF_LABELS),
        },
        // values[8] is firmware parameter word 10
        ParamInfo {
            name: "GRIT",
            max: 10,
            default: 2,
            display: Display::INT,
        },
    ],
};

const MAX_MS: f32 = 1900.0;
const THRESHOLD: u16 = 1840;

const SPEED_LABELS: [&str; 35] = [
    "1/32", "1/24", "1/18", "1/16", "1/12", "1/10", "1/9", "1/8", "1/7", "1/6", "1/5", "1/4",
    "1/3", "1/2", "2/3", "3/4", "1/1", "4/3", "3/2", "2/1", "5/2", "3/1", "7/2", "4/1", "5/1",
    "6/1", "7/1", "8/1", "9/1", "10/1", "12/1", "16/1", "18/1", "24/1", "32/1",
];
/// `Fx_DLY_TremDelay_trmSpeedTbl`: tremolo cycles per delay time.
const SPEED: [(u16, u16); 35] = [
    (1, 32),
    (1, 24),
    (1, 18),
    (1, 16),
    (1, 12),
    (1, 10),
    (1, 9),
    (1, 8),
    (1, 7),
    (1, 6),
    (1, 5),
    (1, 4),
    (1, 3),
    (1, 2),
    (2, 3),
    (3, 4),
    (1, 1),
    (4, 3),
    (3, 2),
    (2, 1),
    (5, 2),
    (3, 1),
    (7, 2),
    (4, 1),
    (5, 1),
    (6, 1),
    (7, 1),
    (8, 1),
    (9, 1),
    (10, 1),
    (12, 1),
    (16, 1),
    (18, 1),
    (24, 1),
    (32, 1),
];
pub(super) const HPF_LABELS: [&str; 21] = [
    "OFF", "20", "40", "60", "80", "100", "120", "140", "160", "180", "200", "230", "260", "300",
    "350", "400", "500", "600", "700", "800", "900",
];
/// `HipassFreqTbl` (Hz; 0 is the table's 0.0001 Hz, i.e. off).
pub(super) const HPF_HZ: [f32; 21] = [
    0.0, 20.0, 40.0, 60.0, 80.0, 100.0, 120.0, 140.0, 160.0, 180.0, 200.0, 230.0, 260.0, 300.0,
    350.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0,
];
pub(super) const FB_TBL: [f32; 11] = [
    0.0, 0.01, 0.09505, 0.17404, 0.263846, 0.373121, 0.503037, 0.621012, 0.79873, 0.959843, 1.0,
];
pub(super) const MIX_DRY: [f32; 10] = [
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    0.878_617_8,
    0.176_197_6,
    0.0,
];
pub(super) const MIX_WET: [f32; 10] = [
    0.0,
    0.071_639_08,
    0.193_018_97,
    0.354_772_54,
    0.547_583,
    0.764_363_6,
    0.975_214_2,
    1.0,
    1.0,
    1.0,
];
const GRIT_FB_CV: [f32; 11] = [1.0, 0.9, 0.9, 0.6, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2];
const GRIT_DRIVE_MIX: [f32; 11] = [0.0, 0.2, 0.4, 0.5, 0.83, 1.04, 1.03, 1.01, 1.05, 1.08, 1.1];
const GRIT_DRY_MIX: [f32; 11] = [1.0, 1.0, 1.0, 0.85, 0.65, 0.6, 0.46, 0.38, 0.36, 0.22, 0.11];
const GRIT_POST_LPF: [f32; 11] = [
    3700.0, 3650.0, 3600.0, 3550.0, 3500.0, 3450.0, 3400.0, 3350.0, 3300.0, 3250.0, 3200.0,
];
const GRIT_PRE_HPF: [f32; 11] = [
    0.0, 60.0, 120.0, 180.0, 240.0, 300.0, 360.0, 420.0, 480.0, 540.0, 600.0,
];
const GRIT_FB: [f32; 11] = [
    0.86, 0.62, 0.52, 0.5, 0.4, 0.38, 0.38, 0.38, 0.38, 0.38, 0.38,
];
/// Fixed drive-branch ROM biquads C75..C89 and the feedback high-pass C94..C98.
const DRIVE_EQ: [[f32; 5]; 3] = [
    [
        1.378_398_2,
        -2.686_416_1,
        1.311_361_4,
        1.946_136_4,
        -0.948_558_6,
    ],
    [
        0.981_113_8,
        -1.750_413_5,
        0.780_731_9,
        1.750_413_5,
        -0.761_845_7,
    ],
    [0.995_390_8, -0.995_390_8, 0.0, 0.990_781_7, 0.0],
];
pub(super) const FEEDBACK_HP: [f32; 5] = [0.998_577_2, -0.998_577_2, 0.0, 0.997_154_5, 0.0];

/// Tremolo shapes of the block LFO over the u32 phase (0..1 outputs).
#[inline]
fn wave(shape: usize, phase: u32) -> f32 {
    let signed = phase as i32;
    let tri = signed.unsigned_abs() as f32 * Q31;
    match shape {
        0 => tri,
        1 => {
            if tri >= 0.5 {
                1.0
            } else {
                0.0
            }
        }
        2 => {
            let v = tri - 0.5;
            2.0 * v * (1.0 - v.abs()) + 0.5
        }
        3 => 0.5 * signed as f32 * Q31 + 0.5,
        _ => 0.5 - 0.5 * signed as f32 * Q31,
    }
}

struct Channel {
    feedback_hp: Stage1,
    clean_hp: Stage1,
    /// Post-drive low-passes and the second-order drive EQ stages.
    drive: [Stage; 4],
    /// The last (first-order) drive EQ stage.
    drive_last: Stage1,
    damp: Cascade<4>,
    line: DelayLine,
    tap: f32,
    post: Stage1,
}

struct TremDelay {
    sample_rate: f32,
    channels: [Channel; 2],
    time: Glide,
    delay_samples: f32,
    feedback: Ramp,
    wet: Ramp,
    dry: Ramp,
    drive_mix: DurationRamp,
    dry_mix: DurationRamp,
    damp_swap: MuteSwap,
    damp_pending: [[f32; 5]; 4],
    phase: u32,
    increment: u32,
    shape: usize,
    depth: f32,
    trem: f32,
    trem_smooth: (f32, f32),
    last: [f32; 9],
    bpm: f32,
}

impl TremDelay {
    fn new(sample_rate: f32) -> Self {
        let channel = || Channel {
            feedback_hp: Stage1::from(rom(FEEDBACK_HP, sample_rate)),
            clean_hp: Stage1::new(Stage::IDENTITY),
            drive: [
                Stage::default(),
                Stage::default(),
                Stage::from(rom(DRIVE_EQ[0], sample_rate)),
                Stage::from(rom(DRIVE_EQ[1], sample_rate)),
            ],
            drive_last: Stage1::from(rom(DRIVE_EQ[2], sample_rate)),
            damp: Cascade::default(),
            line: DelayLine::with_seconds(MAX_MS * 0.001 + 0.01, sample_rate),
            tap: 0.0,
            post: Stage1::new(Stage::IDENTITY),
        };
        let mut effect = Self {
            sample_rate,
            channels: [channel(), channel()],
            time: Glide::standard(0.0, sample_rate),
            delay_samples: 1.0,
            feedback: Ramp::new(0.0, DEFAULT_STEP, sample_rate),
            wet: Ramp::new(0.0, DEFAULT_STEP, sample_rate),
            dry: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
            drive_mix: DurationRamp::new(0.0, RAMP_SAMPLES, sample_rate),
            dry_mix: DurationRamp::new(1.0, RAMP_SAMPLES, sample_rate),
            damp_swap: MuteSwap::new(sample_rate),
            damp_pending: [Stage::IDENTITY; 4],
            phase: 0,
            increment: 0,
            shape: 0,
            depth: 1.0,
            trem: 1.0,
            trem_smooth: one_pole_at(1.0 / 128.0, 127.0 / 128.0, 44_100.0, sample_rate),
            last: [f32::NAN; 9],
            bpm: 0.0,
        };
        effect.set(&crate::Model::TremDelay.defaults(), 120.0);
        effect.time.value = effect.time.target;
        for ramp in [&mut effect.feedback, &mut effect.wet, &mut effect.dry] {
            ramp.jump(ramp.target);
        }
        effect.drive_mix.jump(effect.drive_mix.target);
        effect.dry_mix.jump(effect.dry_mix.target);
        effect
    }

    fn feedback_gain(fb: f32, grit: usize) -> f32 {
        let base = table(&FB_TBL, fb, 101);
        if grit <= 3 {
            base * 0.8 * GRIT_FB[grit]
        } else {
            base * GRIT_FB[grit] * (fb / 101.0).powf(GRIT_FB_CV[grit])
        }
    }
}

impl Effect for TremDelay {
    fn set(&mut self, values: &Values, bpm: f32) {
        if values[..9] == self.last && bpm == self.bpm {
            return;
        }
        let sr = self.sample_rate;
        let first = self.last[0].is_nan();
        let changed = |i: usize| values[i] != self.last[i] || first;
        let fb = values[1].round().clamp(0.0, 100.0);
        let grit = values[8].round().clamp(0.0, 10.0) as usize;
        let tempo = bpm != self.bpm && is_synced(values[0], THRESHOLD);
        if changed(0) || tempo || changed(5) {
            let ms = time_ms(values[0], THRESHOLD, 60.0, MAX_MS, bpm);
            self.delay_samples = (ms * 0.001 * sr).max(1.0);
            self.time.target = self.delay_samples;
            let (numerator, denominator) = SPEED[values[5].round().clamp(0.0, 34.0) as usize];
            let cycles = f64::from(numerator) / f64::from(denominator);
            self.increment =
                (4_294_967_295.0 / f64::from(self.delay_samples) * cycles).min(2e9) as u32;
        }
        if changed(1) || changed(8) {
            self.feedback.set(Self::feedback_gain(fb, grit));
        }
        if changed(2) {
            let mix = values[2].round().clamp(0.0, 100.0);
            self.wet.set(0.75 * table(&MIX_WET, mix, 101));
            self.dry.set(table(&MIX_DRY, mix, 101));
        }
        self.shape = values[3].round().clamp(0.0, 4.0) as usize;
        self.depth = values[4].round().clamp(0.0, 100.0) * 0.01;
        if changed(6) {
            let damp = values[6].round().clamp(0.0, 10.0) as usize;
            self.damp_pending = rom_set(&DAMP[damp], sr);
            if first {
                for ch in &mut self.channels {
                    ch.damp.load(&self.damp_pending);
                }
            } else {
                self.damp_swap.start();
            }
        }
        if changed(7) {
            let hpf = values[7].round().clamp(0.0, 20.0) as usize;
            let c = if hpf == 0 {
                Stage::IDENTITY
            } else {
                words(design(Kind::Highpass1, sr, HPF_HZ[hpf], 0.0, 0.7))
            };
            for ch in &mut self.channels {
                ch.post.load(c);
            }
        }
        if changed(8) {
            self.drive_mix.set(GRIT_DRIVE_MIX[grit]);
            self.dry_mix.set(GRIT_DRY_MIX[grit]);
            let lowpass = words(design(Kind::Lowpass2, sr, GRIT_POST_LPF[grit], 0.0, 0.7));
            let clean = if grit == 0 {
                Stage::IDENTITY
            } else {
                words(design(Kind::Highpass1, sr, GRIT_PRE_HPF[grit], 0.0, 0.7))
            };
            for ch in &mut self.channels {
                ch.drive[0].c = lowpass;
                ch.drive[1].c = lowpass;
                ch.clean_hp.load(clean);
            }
        }
        self.last.copy_from_slice(&values[..9]);
        self.bpm = bpm;
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.phase = self.phase.wrapping_add(self.increment);
        let gain = 1.0 - self.depth + self.depth * wave(self.shape, self.phase);
        self.trem = self.trem_smooth.0 * gain + self.trem_smooth.1 * self.trem;
        let delay = self.time.tick();
        let feedback = self.feedback.step();
        let wet_gain = self.wet.step();
        let dry_gain = self.dry.step();
        let drive_mix = self.drive_mix.tick();
        let dry_mix = self.dry_mix.tick();
        let (write_gain, swap) = self.damp_swap.tick();
        if swap {
            for ch in &mut self.channels {
                ch.damp.load(&self.damp_pending);
            }
        }
        let mut out = [0.0; 2];
        for (ch, channel) in self.channels.iter_mut().enumerate() {
            let fed = channel.feedback_hp.process(clip1(feedback * channel.tap)) + frame[ch];
            let mut grit = dry_mix * channel.clean_hp.process(fed);
            if drive_mix != 0.0 {
                let mut driven = 0.046 * tanh(42.5 * fed);
                for stage in &mut channel.drive {
                    driven = stage.process(driven);
                }
                driven = channel.drive_last.process(driven);
                grit += drive_mix * driven;
            }
            let damped = channel.damp.process(grit);
            channel.line.write(flush(write_gain * damped));
            channel.tap = channel.line.read_linear(delay);
            let post = channel.post.process(self.trem * channel.tap);
            out[ch] = wet_gain * post + dry_gain * frame[ch];
        }
        out
    }

    fn clear(&mut self) {
        for ch in &mut self.channels {
            ch.feedback_hp.clear();
            ch.clean_hp.clear();
            for stage in &mut ch.drive {
                stage.clear();
            }
            ch.drive_last.clear();
            ch.damp.clear();
            ch.line.clear();
            ch.tap = 0.0;
            ch.post.clear();
        }
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(TremDelay::new(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_has_the_sixty_ms_offset_and_speed_is_per_delay() {
        let mut effect = TremDelay::new(48_000.0);
        let mut values = crate::Model::TremDelay.defaults();
        values[0] = 440.0; // 500 ms
        values[5] = 16.0; // 1/1: one tremolo cycle per repeat
        effect.set(&values, 120.0);
        assert!((effect.delay_samples - 24_000.0).abs() < 0.5);
        let hz = effect.increment as f64 / 4_294_967_296.0 * 48_000.0;
        assert!((hz - 2.0).abs() < 1e-3, "{hz}");
    }

    #[test]
    fn echo_arrives_at_the_delay_time_and_feedback_is_bounded() {
        let sr = 44_100.0;
        let mut effect = TremDelay::new(sr);
        let mut values = crate::Model::TremDelay.defaults();
        values[0] = 40.0; // 100 ms
        values[2] = 100.0;
        values[4] = 0.0; // no tremolo
        values[7] = 0.0;
        values[8] = 0.0; // clean
        effect.set(&values, 120.0);
        effect.time.value = effect.time.target;
        let mut out = Vec::new();
        for i in 0..8820 {
            let x = if i == 0 { 0.5 } else { 0.0 };
            out.push(effect.process([x, x])[0]);
        }
        let peak = out
            .iter()
            .enumerate()
            .skip(10)
            .max_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
            .unwrap()
            .0;
        assert!((peak as i32 - 4410).abs() < 20, "{peak}");
        values[1] = 100.0;
        values[8] = 10.0;
        effect.set(&values, 120.0);
        for i in 0..(sr as usize * 5) {
            let x = if i % 3000 < 300 { 0.9 } else { 0.0 };
            let y = effect.process([x, -x]);
            assert!(y[0].is_finite() && y[0].abs() < 4.0);
        }
    }

    #[test]
    fn waves_cover_zero_to_one() {
        for shape in 0..5 {
            let mut lo = 1.0f32;
            let mut hi = 0.0f32;
            for i in 0..1000u32 {
                let w = wave(shape, i.wrapping_mul(4_294_967));
                lo = lo.min(w);
                hi = hi.max(w);
            }
            assert!(lo < 0.01 && hi > 0.99, "{shape}: {lo} {hi}");
        }
    }
}
