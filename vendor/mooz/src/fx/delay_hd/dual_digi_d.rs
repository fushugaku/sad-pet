//! DualDigiD (DUALDIGD.ZDL): two independent digital delays (A left, B
//! right) with chorus-like modulation, a shelving FLTR in the loops and a
//! balance/stereo mixer, after the recovered `Fx_DLY_DualDigiD` callback and
//! the decoded editors.
//!
//! Per delay, as in the callback: `clip±1(Fdbk·filtered)` → 20 Hz high-pass →
//! `+ input` → 2 s line read through the editors' 200 ms two-tap crossfade
//! (XfdTime) → a short modulation line (up to 441 samples at 44.1 kHz) read at
//! `depth · LFO`, the LFO a 0.12·triangle + 0.88·parabolic sine (B half a
//! cycle later), 40 ms smoothed, depth 1/512 smoothed → the FLTR first-order
//! shelf: 0–100 low shelf (`LoEq` 480–1200 Hz, −12…0 dB), 101–200 high shelf
//! (`HiEq` 8–13 kHz, 0…−36 dB), with the editor's mute when the shelf type
//! changes. The shelf output is both the feedback source and the wet signal.
//!
//! Output: Depth M0–M50 sums to mono `0.95·Mix·(cA·A + cB·B) + 0.489·dry·(L+R)`
//! (DlyMx through `MONO_LRmixGain`); S0–S50 keeps stereo with the
//! `STR_LRmixGain` direct/cross curves, `0.96·wet` and full dry. Mix: dry 1 and
//! wet Mix/50 below 51, then wet 1 and dry (100−Mix)/50. Speed interpolates
//! `rate_tbl` (0.008–5 Hz), Depth `Depth_tbl`; for TimeB < 10 ms the B depth
//! is scaled by TimeB/10 as the editor does. TimeA/B are `p` ms (0–1990) or
//! tempo notes.
use super::{
    common::{
        CrossfadeTime, DelayLine, Kind, MuteSwap, Q31, Settled, Stage, Stage1, clip1, design,
        flush, one_pole_at, rom, samples, table, table_step, time_ms, words,
    },
    trem_delay::FEEDBACK_HP,
};
use crate::{
    Effect, Values,
    catalog::*,
    zoom::{DEFAULT_STEP, Ramp, is_synced},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_dual_digi_d",
    name: "DualDigiD",
    category: Category::Delay,
    zdl: "DUALDIGD.ZDL",
    description: "Two modulated digital delays with shelving filters",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "TimeA",
            max: 2005,
            default: 500,
            display: Display::DelayTime {
                threshold: 1990,
                offset: 0,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "TimeB",
            max: 2005,
            default: 375,
            display: Display::DelayTime {
                threshold: 1990,
                offset: 0,
            },
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "FdbkA",
            max: 110,
            default: 50,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "FdbkB",
            max: 110,
            default: 50,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Depth",
            max: 101,
            default: 0,
            display: Display::labels(&[
                "M0", "M1", "M2", "M3", "M4", "M5", "M6", "M7", "M8", "M9", "M10", "M11", "M12",
                "M13", "M14", "M15", "M16", "M17", "M18", "M19", "M20", "M21", "M22", "M23", "M24",
                "M25", "M26", "M27", "M28", "M29", "M30", "M31", "M32", "M33", "M34", "M35", "M36",
                "M37", "M38", "M39", "M40", "M41", "M42", "M43", "M44", "M45", "M46", "M47", "M48",
                "M49", "M50", "S0", "S1", "S2", "S3", "S4", "S5", "S6", "S7", "S8", "S9", "S10",
                "S11", "S12", "S13", "S14", "S15", "S16", "S17", "S18", "S19", "S20", "S21", "S22",
                "S23", "S24", "S25", "S26", "S27", "S28", "S29", "S30", "S31", "S32", "S33", "S34",
                "S35", "S36", "S37", "S38", "S39", "S40", "S41", "S42", "S43", "S44", "S45", "S46",
                "S47", "S48", "S49", "S50",
            ]),
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Speed",
            max: 50,
            default: 25,
            display: Display::INT,
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "FLTR",
            max: 200,
            default: 100,
            display: Display::offset(-100.0),
        },
        // values[7] is firmware parameter word 9
        ParamInfo {
            name: "DlyMx",
            max: 100,
            default: 25,
            display: Display::INT,
        },
        // values[8] is firmware parameter word 10
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 50,
            display: Display::INT,
        },
    ],
};

const MAX_MS: f32 = 1990.0;
const THRESHOLD: u16 = 1990;
const RATE: [f32; 26] = [
    779.0, 973.0, 1460.0, 1947.0, 2921.0, 3408.0, 3895.0, 5843.0, 7791.0, 9739.0, 12660.0, 17530.0,
    22400.0, 29217.0, 35060.0, 46747.0, 61356.0, 78887.0, 103_235.0, 131_478.0, 168_487.0,
    209_391.0, 273_670.0, 367_166.0, 453_844.0, 487_931.0,
];
const DEPTH: [f32; 11] = [
    0.0,
    185_330_780.0,
    370_661_560.0,
    555_992_341.0,
    741_323_121.0,
    926_653_902.0,
    1_111_984_682.0,
    1_297_315_463.0,
    1_482_646_243.0,
    1_667_977_024.0,
    1_853_307_804.0,
];
const HI_FC: [f32; 11] = [
    8000.0, 9000.0, 10000.0, 11000.0, 12000.0, 13000.0, 13000.0, 13000.0, 13000.0, 13000.0, 13000.0,
];
const HI_GAIN: [f32; 11] = [
    0.0, -3.6, -7.2, -10.8, -14.4, -18.0, -21.6, -25.2, -28.8, -32.4, -36.0,
];
const LO_FC: [f32; 11] = [
    480.0, 566.4, 652.8, 739.2, 825.6, 912.0, 998.4, 1084.8, 1171.2, 1200.0, 1200.0,
];
const LO_GAIN: [f32; 11] = [
    -12.0, -10.8, -9.6, -8.4, -7.2, -6.0, -4.8, -3.6, -2.4, -1.2, 0.0,
];
const STR_DIRECT: [f32; 26] = [
    0.0, 0.0869, 0.1739, 0.2391, 0.3261, 0.413, 0.4783, 0.5435, 0.6304, 0.7174, 0.7826, 0.8478,
    0.9348, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
];
const STR_CROSS: [f32; 26] = [
    0.0, 0.0625, 0.1458, 0.2083, 0.2708, 0.3333, 0.3542, 0.3542, 0.375, 0.3333, 0.2916, 0.2292,
    0.1458, 0.0, 0.125, 0.2292, 0.3542, 0.4583, 0.5833, 0.6667, 0.75, 0.8542, 0.8958, 0.9583,
    0.9792, 1.0,
];
const MONO: [f32; 26] = [
    0.0, 0.1145, 0.2061, 0.2672, 0.3511, 0.3969, 0.4389, 0.4962, 0.5344, 0.5821, 0.6298, 0.6794,
    0.7176, 0.7443, 0.7863, 0.8244, 0.8473, 0.8855, 0.916, 0.9351, 0.9542, 0.9695, 0.978, 0.9847,
    0.9923, 1.0,
];

/// The modulation wave of `wave_0`/`wave_1` (0..1).
#[inline]
fn wave(phase: u32) -> f32 {
    let v = (phase as i32).unsigned_abs() as f32 * Q31 - 0.5;
    let tri = 2.0 * v;
    let parabola = 4.0 * v * (1.0 - v.abs());
    0.5 + 0.5 * (0.12 * tri + 0.88 * parabola)
}

struct Channel {
    feedback_hp: Stage1,
    line: DelayLine,
    time: CrossfadeTime,
    short: DelayLine,
    modulation: f32,
    depth: f32,
    depth_target: f32,
    fltr: Stage1,
    filtered: f32,
    feedback: Ramp,
}

struct DualDigiD {
    sample_rate: f32,
    channels: [Channel; 2],
    phase: u32,
    increment: u32,
    modulation_smooth: (f32, f32),
    depth_smooth: (f32, f32),
    short_scale: f32,
    high_shelf: bool,
    fltr_pending: [f32; 5],
    fltr_swap: MuteSwap,
    stereo: bool,
    pending_stereo: bool,
    mode_fade: MuteSwap,
    wet: Ramp,
    dry: Ramp,
    matrix: [Ramp; 4],
    last: [f32; 9],
    bpm: f32,
}

impl DualDigiD {
    fn new(sample_rate: f32) -> Self {
        let sr = sample_rate;
        let channel = || Channel {
            feedback_hp: Stage1::from(rom(FEEDBACK_HP, sr)),
            line: DelayLine::with_seconds(MAX_MS * 0.001 + 0.01, sr),
            time: CrossfadeTime::new(1.0, 200.0, sr),
            short: DelayLine::new(samples(445.0, sr) as usize + 4),
            modulation: 0.5,
            depth: 0.0,
            depth_target: 0.0,
            fltr: Stage1::new(Stage::IDENTITY),
            filtered: 0.0,
            feedback: Ramp::new(0.0, DEFAULT_STEP, sr),
        };
        let mut effect = Self {
            sample_rate,
            channels: [channel(), channel()],
            phase: 0,
            increment: 0,
            modulation_smooth: one_pole_at(
                0x0012_b6b1 as f32 * Q31,
                0x7fed_494f as f32 * Q31,
                44_100.0,
                sr,
            ),
            depth_smooth: one_pole_at(1.0 / 512.0, 511.0 / 512.0, 44_100.0, sr),
            short_scale: samples(512.0, sr),
            high_shelf: false,
            fltr_pending: Stage::IDENTITY,
            fltr_swap: MuteSwap::new(sr),
            stereo: false,
            pending_stereo: false,
            mode_fade: MuteSwap::new(sr),
            wet: Ramp::new(0.0, DEFAULT_STEP, sr),
            dry: Ramp::new(1.0, DEFAULT_STEP, sr),
            matrix: [Ramp::new(0.0, DEFAULT_STEP, sr); 4],
            last: [f32::NAN; 9],
            bpm: 0.0,
        };
        effect.set(&crate::Model::DualDigiD.defaults(), 120.0);
        for ramp in effect
            .matrix
            .iter_mut()
            .chain([&mut effect.wet, &mut effect.dry])
        {
            ramp.jump(ramp.target);
        }
        for ch in &mut effect.channels {
            ch.feedback.jump(ch.feedback.target);
            ch.depth = ch.depth_target;
        }
        effect
    }

    /// FLTR 0..200 → (high shelf?, coefficients).
    fn fltr(value: f32, sample_rate: f32) -> (bool, [f32; 5]) {
        if value > 100.0 {
            let p = value - 100.0;
            let c = design(
                Kind::HighShelf1,
                sample_rate,
                table(&HI_FC, p, 101),
                table(&HI_GAIN, p, 101),
                0.8,
            );
            (true, words(c))
        } else {
            let c = design(
                Kind::LowShelf1,
                sample_rate,
                table(&LO_FC, value, 101),
                table(&LO_GAIN, value, 101),
                0.8,
            );
            (false, words(c))
        }
    }
}

impl Effect for DualDigiD {
    #[allow(clippy::needless_range_loop)]
    fn set(&mut self, values: &Values, bpm: f32) {
        if values[..9] == self.last && bpm == self.bpm {
            return;
        }
        let sr = self.sample_rate;
        let first = self.last[0].is_nan();
        let tempo = bpm != self.bpm;
        let mut time_b = 0.0;
        for ch in 0..2 {
            let ms = time_ms(values[ch], THRESHOLD, 0.0, MAX_MS, bpm);
            if ch == 1 {
                time_b = ms;
            }
            if values[ch] != self.last[ch] || (tempo && is_synced(values[ch], THRESHOLD)) {
                let delay = ms * 0.001 * sr;
                if first {
                    self.channels[ch].time.jump(delay);
                } else {
                    self.channels[ch].time.request(delay);
                }
            }
            self.channels[ch]
                .feedback
                .set(values[2 + ch].round().clamp(0.0, 110.0) * 0.01);
        }
        let depth = values[4].round().clamp(0.0, 101.0);
        let stereo = depth >= 51.0;
        let amount = table(&DEPTH, if stereo { depth - 51.0 } else { depth }, 51) * Q31;
        self.channels[0].depth_target = amount;
        self.channels[1].depth_target = if time_b < 10.0 {
            amount * time_b.round() * 0.1
        } else {
            amount
        };
        if stereo != self.pending_stereo || first {
            self.pending_stereo = stereo;
            if first {
                self.stereo = stereo;
            } else {
                self.mode_fade.start();
            }
        }
        let speed = values[5].round().clamp(0.0, 50.0);
        self.increment = (table_step(&RATE, speed, 2.0) * 44_100.0 / sr) as u32;
        let fltr = values[6].round().clamp(0.0, 200.0);
        if fltr != self.last[6] {
            let (high, words) = Self::fltr(fltr, sr);
            self.fltr_pending = words;
            if first || high == self.high_shelf {
                for ch in &mut self.channels {
                    ch.fltr.load(words);
                }
            } else {
                self.fltr_swap.start();
            }
            self.high_shelf = high;
        }
        let balance = values[7].round().clamp(0.0, 100.0);
        let gains = if stereo {
            [
                table_step(&STR_DIRECT, 100.0 - balance, 4.0),
                table_step(&STR_CROSS, balance, 4.0),
                table_step(&STR_CROSS, 100.0 - balance, 4.0),
                table_step(&STR_DIRECT, balance, 4.0),
            ]
        } else {
            [
                table_step(&MONO, 100.0 - balance, 4.0),
                0.0,
                0.0,
                table_step(&MONO, balance, 4.0),
            ]
        };
        for (ramp, gain) in self.matrix.iter_mut().zip(gains) {
            ramp.set(gain);
        }
        let mix = values[8].round().clamp(0.0, 100.0);
        if mix < 51.0 {
            self.dry.set(1.0);
            self.wet.set(mix / 50.0);
        } else {
            self.dry.set((100.0 - mix) / 50.0);
            self.wet.set(1.0);
        }
        self.last.copy_from_slice(&values[..9]);
        self.bpm = bpm;
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.phase = self.phase.wrapping_add(self.increment);
        let waves = [wave(self.phase), wave(self.phase.wrapping_add(0x7fff_ffff))];
        let (fltr_gain, swap) = self.fltr_swap.tick();
        if swap {
            for ch in &mut self.channels {
                ch.fltr.load(self.fltr_pending);
            }
        }
        let (mg, mp) = self.modulation_smooth;
        let (dg, dp) = self.depth_smooth;
        let mut wet = [0.0; 2];
        for (ch, channel) in self.channels.iter_mut().enumerate() {
            let feedback = channel.feedback.step();
            let fed = channel
                .feedback_hp
                .process(clip1(feedback * channel.filtered))
                + frame[ch];
            channel.line.write(flush(fed));
            let blend = channel.time.tick();
            let taps = channel.time.taps;
            let long = channel.line.read_crossfade(blend, taps);
            channel.short.write(long);
            channel.modulation = mg * waves[ch] + mp * channel.modulation;
            channel.depth = dg * channel.depth_target + dp * channel.depth;
            let offset = channel.modulation * channel.depth * self.short_scale;
            let modulated = channel.short.read_linear(offset);
            channel.filtered = fltr_gain * channel.fltr.process(modulated);
            wet[ch] = channel.filtered;
        }
        let (mode, switch) = self.mode_fade.tick();
        if switch {
            self.stereo = self.pending_stereo;
        }
        let m = [
            self.matrix[0].step(),
            self.matrix[1].step(),
            self.matrix[2].step(),
            self.matrix[3].step(),
        ];
        let wet_gain = self.wet.step();
        let dry_gain = self.dry.step();
        if self.stereo {
            let w = 0.96 * wet_gain;
            [
                mode * (dry_gain * frame[0] + w * (m[0] * wet[0] + m[1] * wet[1])),
                mode * (w * (m[2] * wet[0] + m[3] * wet[1]) + dry_gain * frame[1]),
            ]
        } else {
            let sum = 0.95 * wet_gain * (m[0] * wet[0] + m[3] * wet[1])
                + 0.489 * dry_gain * (frame[0] + frame[1]);
            [mode * sum, mode * sum]
        }
    }

    fn clear(&mut self) {
        for ch in &mut self.channels {
            ch.feedback_hp.clear();
            ch.line.clear();
            ch.short.clear();
            ch.fltr.clear();
            ch.filtered = 0.0;
        }
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(DualDigiD::new(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fltr_center_is_flat_and_ends_cut() {
        let sr = 48_000.0;
        let (_, flat) = DualDigiD::fltr(100.0, sr);
        let c = |w: [f32; 5]| crate::dsp::Coefficients {
            b0: w[0],
            b1: w[1],
            b2: w[2],
            a1: -w[3],
            a2: -w[4],
        };
        assert!((c(flat).magnitude(sr, 100.0) - 1.0).abs() < 1e-3);
        let (_, low) = DualDigiD::fltr(0.0, sr);
        assert!(crate::dsp::gain_to_db(c(low).magnitude(sr, 20.0)) < -11.0);
        let (high, cut) = DualDigiD::fltr(200.0, sr);
        assert!(high);
        assert!(crate::dsp::gain_to_db(c(cut).magnitude(sr, 20_000.0)) < -20.0);
    }

    #[test]
    fn delays_and_balance() {
        let sr = 44_100.0;
        let mut effect = DualDigiD::new(sr);
        let mut values = crate::Model::DualDigiD.defaults();
        values[0] = 100.0;
        values[1] = 200.0;
        values[2] = 0.0;
        values[3] = 0.0;
        values[4] = 51.0; // stereo, no modulation
        values[7] = 50.0;
        values[8] = 100.0;
        effect.set(&values, 120.0);
        let mut left = vec![];
        let mut right = vec![];
        for i in 0..sr as usize / 2 {
            let x = if i == 0 { 1.0 } else { 0.0 };
            let y = effect.process([x, x]);
            left.push(y[0]);
            right.push(y[1]);
        }
        // the mode fade from the first mono→stereo request may still run: use peaks
        let peak = |v: &[f32]| {
            v.iter()
                .enumerate()
                .skip(10)
                .max_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
                .unwrap()
                .0
        };
        assert!((peak(&left) as i32 - 4410).abs() < 3);
        assert!((peak(&right) as i32 - 8820).abs() < 3);
    }

    #[test]
    fn max_feedback_bounded() {
        let mut effect = DualDigiD::new(96_000.0);
        let mut values = crate::Model::DualDigiD.defaults();
        values[2] = 110.0;
        values[3] = 110.0;
        values[4] = 101.0;
        effect.set(&values, 120.0);
        for i in 0..96_000 * 4 {
            let x = if i % 30_000 < 2000 { 0.9 } else { 0.0 };
            let y = effect.process([x, -x]);
            assert!(y[0].is_finite() && y[0].abs() < 8.0);
        }
    }
}
