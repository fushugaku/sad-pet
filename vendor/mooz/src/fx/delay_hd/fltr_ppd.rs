//! FLTR PPD (FLTERPPD.ZDL): a ping-pong delay (TimeA delay feeding TimeB in
//! series) behind a swept resonant 24 dB low-pass, with a diffused feedback
//! path, after the recovered `Fx_DLY_FilterPPD` callback and decoded editors.
//!
//! As in the callback: the mono input passes two cascaded Chamberlin
//! state-variable low-passes (states clipped to ±1), frequency `1.3·w` and
//! damping `0.45 − 0.355·w` at 44.1 kHz (re-derived as cut-off frequency for
//! the host rate). The sweep `w` is a 7.9 ms one-pole of
//! `0.0281 + ramp·depth·wave`: `ramp` is a saw over four TimeA periods, the
//! LFO runs at `(rate+1)/8` cycles per TimeA with the Wave shape (sine²,
//! triangle or square; `wave_tbl`), depth from `Depth_tbl` (1/512 smoothed).
//! FLTR crossfades `dry` and `0.53·filtered`; `clip±1(Fdbk·fb)` through a 20 Hz
//! high-pass is added and the sum enters delay A (TimeA), whose output enters
//! delay B (TimeB). Delay B's output feeds back directly (`1 − APF`) and
//! through a 20 Hz high-pass, a 10 kHz low-pass and four allpass-like sections
//! (183/514/696/1204 samples, Slur sets their feedback `APF_FB_tbl`). Output
//! `WET(Mix)·DlyMx-balanced A/B + WET(100−Mix)·dry`, stereo (A left, B right)
//! for Depth S0–S10 or summed to mono (dry ×0.5, `LRMonoMixAtt`) for M0–M10.
//! Delay times change by the editors' 200 ms two-tap crossfade.
use super::{
    common::{
        CrossfadeTime, DurationRamp, Q31, RAMP_SAMPLES, Stage, clip1, one_pole_at, rom, samples,
        table, time_ms,
    },
    trem_delay::FEEDBACK_HP,
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{DelayLine, flush},
    zoom::{DEFAULT_STEP, Ramp, is_synced},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_fltr_ppd",
    name: "FLTR PPD",
    category: Category::Delay,
    zdl: "FLTERPPD.ZDL",
    description: "Ping-pong delay behind a swept resonant filter",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "TimeA",
            max: 1915,
            default: 500,
            display: Display::DelayTime {
                threshold: 1900,
                offset: 0,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "TimeB",
            max: 1915,
            default: 250,
            display: Display::DelayTime {
                threshold: 1900,
                offset: 0,
            },
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "DlyMx",
            max: 100,
            default: 25,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Fdbk",
            max: 110,
            default: 75,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Slur",
            max: 10,
            default: 3,
            display: Display::INT,
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "FLTR",
            max: 100,
            default: 80,
            display: Display::INT,
        },
        // values[7] is firmware parameter word 9
        ParamInfo {
            name: "Depth",
            max: 21,
            default: 8,
            display: Display::labels(&[
                "M0", "M1", "M2", "M3", "M4", "M5", "M6", "M7", "M8", "M9", "M10", "S0", "S1",
                "S2", "S3", "S4", "S5", "S6", "S7", "S8", "S9", "S10",
            ]),
        },
        // values[8] is firmware parameter word 10
        ParamInfo {
            name: "Wave",
            max: 47,
            default: 35,
            display: Display::labels(&[
                "SIN1", "SIN2", "SIN3", "SIN4", "SIN5", "SIN6", "SIN7", "SIN8", "SIN9", "SIN10",
                "SIN11", "SIN12", "SIN13", "SIN14", "SIN15", "SIN16", "TRI1", "TRI2", "TRI3",
                "TRI4", "TRI5", "TRI6", "TRI7", "TRI8", "TRI9", "TRI10", "TRI11", "TRI12", "TRI13",
                "TRI14", "TRI15", "TRI16", "SQR1", "SQR2", "SQR3", "SQR4", "SQR5", "SQR6", "SQR7",
                "SQR8", "SQR9", "SQR10", "SQR11", "SQR12", "SQR13", "SQR14", "SQR15", "SQR16",
            ]),
        },
    ],
};

const MAX_MS: f32 = 1900.0;
const THRESHOLD: u16 = 1900;
const APF_FB: [f32; 11] = [
    0.3,
    0.451_280_7,
    0.499_616_1,
    0.534_764_4,
    0.563_395,
    0.587_986_2,
    0.609_773_4,
    0.629_475_2,
    0.647_551_8,
    0.664_318,
    0.68,
];
const DEPTH: [u32; 11] = [
    0,
    240_324_145,
    383_636_709,
    582_069_489,
    758_454_183,
    934_838_877,
    1_089_175_484,
    1_265_560_178,
    1_530_137_218,
    1_816_762_346,
    2_147_483_647,
];
const MONO_ATT: [f32; 11] = [
    1.0, 0.96, 0.92, 0.9, 0.88, 0.86, 0.835, 0.8, 0.76, 0.73, 0.68,
];
const L_MIX: [f32; 11] = [
    0.0125,
    0.118_850_2,
    0.217_35,
    0.298_538_3,
    0.398_107_2,
    0.54,
    0.562_341_3,
    0.668_343_9,
    0.767_291_8,
    0.891_250_9,
    1.0,
];
const R_MIX: [f32; 11] = [
    0.0,
    0.118_850_2,
    0.217_35,
    0.298_538_3,
    0.398_107_2,
    0.54,
    0.562_341_3,
    0.668_343_9,
    0.767_291_8,
    0.891_250_9,
    1.0,
];
const WET: [f32; 11] = [
    0.0,
    0.211_348_9,
    0.398_107_2,
    0.630_957_3,
    0.794_328_2,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
];
const DIFFUSION: [usize; 4] = [183, 514, 696, 1204];
const SWEEP_POINTS: usize = 256;
const SWEEP_MAX: f32 = 1.1;

/// One Chamberlin low-pass stage with the callback's ±1 state clipping.
#[derive(Default, Clone, Copy)]
struct Svf {
    band: f32,
    low: f32,
}

impl Svf {
    #[inline]
    fn process(&mut self, x: f32, f: f32, q: f32) -> f32 {
        let high = clip1(x - self.band * q - self.low);
        self.band = clip1(self.band + f * high);
        self.low = clip1(self.low + f * self.band);
        self.low
    }
}

struct Diffuser {
    line: DelayLine,
    delay: usize,
}

struct FltrPpd {
    sample_rate: f32,
    sweep_f: [f32; SWEEP_POINTS + 1],
    svf: [Svf; 2],
    phase_ramp: u32,
    phase_lfo: u32,
    inc_ramp: u32,
    inc_lfo: u32,
    shape: usize,
    depth: f32,
    depth_target: f32,
    depth_smooth: (f32, f32),
    sweep: f32,
    sweep_smooth: (f32, f32),
    lines: [DelayLine; 2],
    times: [CrossfadeTime; 2],
    feedback_hp: Stage,
    feedback_src: f32,
    diff_hp: Stage,
    diff_lp: Stage,
    diffusers: [Diffuser; 4],
    apf_feedback: f32,
    apf_mix: DurationRamp,
    filter_mix: Ramp,
    feedback: Ramp,
    wet: Ramp,
    dry: Ramp,
    balance: [Ramp; 2],
    wet_att: Ramp,
    dry_att: f32,
    stereo: bool,
    mode_fade: super::common::MuteSwap,
    pending_stereo: bool,
    last: [f32; 9],
    bpm: f32,
}

impl FltrPpd {
    fn new(sample_rate: f32) -> Self {
        let sr = sample_rate;
        let mut sweep_f = [0.0; SWEEP_POINTS + 1];
        for (i, f) in sweep_f.iter_mut().enumerate() {
            let w = i as f32 * SWEEP_MAX / SWEEP_POINTS as f32;
            let half = (1.3 * w * 0.5).min(1.0);
            let hz = (half.asin() * 44_100.0 / std::f32::consts::PI).min(0.45 * sr);
            *f = 2.0 * (std::f32::consts::PI * hz / sr).sin();
        }
        let n = |d: usize| (samples(d as f32, sr).round() as usize).max(1);
        let diffuser = |d: usize| Diffuser {
            line: DelayLine::new(n(d) + 2),
            delay: n(d),
        };
        let mut effect = Self {
            sample_rate,
            sweep_f,
            svf: [Svf::default(); 2],
            phase_ramp: 0,
            phase_lfo: 0,
            inc_ramp: 0,
            inc_lfo: 0,
            shape: 0,
            depth: 0.0,
            depth_target: 0.0,
            depth_smooth: one_pole_at(1.0 / 512.0, 511.0 / 512.0, 44_100.0, sr),
            sweep: 0.028_105_64,
            sweep_smooth: one_pole_at(0.002_852_21, 0.997_147_8, 44_100.0, sr),
            lines: [
                DelayLine::with_seconds(MAX_MS * 0.001 + 0.01, sr),
                DelayLine::with_seconds(MAX_MS * 0.001 + 0.01, sr),
            ],
            times: [CrossfadeTime::new(1.0, 200.0, sr); 2],
            feedback_hp: Stage::from(rom(FEEDBACK_HP, sr)),
            feedback_src: 0.0,
            diff_hp: Stage::from(rom([0.998_577, -0.998_577, 0.0, 0.997_155, 0.0], sr)),
            diff_lp: Stage::from(rom([0.463_426, 0.463_426, 0.0, 0.073_149, 0.0], sr)),
            diffusers: DIFFUSION.map(diffuser),
            apf_feedback: 0.3,
            apf_mix: DurationRamp::new(0.0, RAMP_SAMPLES, sr),
            filter_mix: Ramp::new(0.0, DEFAULT_STEP, sr),
            feedback: Ramp::new(0.0, DEFAULT_STEP, sr),
            wet: Ramp::new(0.0, DEFAULT_STEP, sr),
            dry: Ramp::new(1.0, DEFAULT_STEP, sr),
            balance: [Ramp::new(1.0, DEFAULT_STEP, sr); 2],
            wet_att: Ramp::new(1.0, DEFAULT_STEP, sr),
            dry_att: 1.0,
            stereo: false,
            mode_fade: super::common::MuteSwap::new(sr),
            pending_stereo: false,
            last: [f32::NAN; 9],
            bpm: 0.0,
        };
        effect.set(&crate::Model::FltrPpd.defaults(), 120.0);
        for ramp in [
            &mut effect.filter_mix,
            &mut effect.feedback,
            &mut effect.wet,
            &mut effect.dry,
            &mut effect.wet_att,
        ] {
            ramp.jump(ramp.target);
        }
        for ramp in &mut effect.balance {
            ramp.jump(ramp.target);
        }
        effect.apf_mix.jump(effect.apf_mix.target);
        effect.depth = effect.depth_target;
        effect
    }

    #[inline]
    fn sweep_coefficients(&self, w: f32) -> (f32, f32) {
        let position = (w.clamp(0.0, SWEEP_MAX) * (SWEEP_POINTS as f32 / SWEEP_MAX))
            .min(SWEEP_POINTS as f32 - 1e-3);
        let i = position as usize;
        let t = position - i as f32;
        let f = self.sweep_f[i] + (self.sweep_f[i + 1] - self.sweep_f[i]) * t;
        (f, 0.45 - 0.355 * w)
    }

    #[inline]
    fn wave(&self, phase: u32) -> f32 {
        let p = (phase as i32).unsigned_abs() as f32 * Q31;
        match self.shape {
            0 => {
                let v = p - 0.5;
                let s = 2.0 * v * (1.0 - v.abs()) + 0.5;
                s * s
            }
            1 => p,
            _ => {
                if p >= 0.5 {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }
}

impl Effect for FltrPpd {
    #[allow(clippy::needless_range_loop)]
    fn set(&mut self, values: &Values, bpm: f32) {
        if values[..9] == self.last && bpm == self.bpm {
            return;
        }
        let sr = self.sample_rate;
        let first = self.last[0].is_nan();
        let tempo = bpm != self.bpm;
        let mut ms_a = 0.0;
        for ch in 0..2 {
            let ms = time_ms(values[ch], THRESHOLD, 0.0, MAX_MS, bpm);
            if ch == 0 {
                ms_a = ms;
            }
            if values[ch] != self.last[ch] || (tempo && is_synced(values[ch], THRESHOLD)) {
                let delay = ms * 0.001 * sr;
                if first {
                    self.times[ch].jump(delay);
                } else {
                    self.times[ch].request(delay);
                }
            }
        }
        let wave = values[8].round().clamp(0.0, 47.0) as u32;
        self.shape = (wave / 16) as usize;
        let rate = wave % 16;
        let period = (4.0 * ms_a * 0.001 * sr).max(4.0);
        self.inc_ramp = (4_294_967_295.0 / f64::from(period)) as u32;
        self.inc_lfo = (f64::from(self.inc_ramp) * f64::from(rate + 1) * 0.5) as u32;

        let mix = values[2].round().clamp(0.0, 100.0);
        self.wet.set(table(&WET, mix, 101));
        self.dry.set(table(&WET, 100.0 - mix, 101));
        let depth = values[7].round().clamp(0.0, 21.0) as usize;
        let stereo = depth >= 11;
        self.depth_target = DEPTH[if stereo { depth - 11 } else { depth }] as f32 * Q31;
        let balance = values[3].round().clamp(0.0, 100.0);
        if balance < 50.0 {
            self.balance[0].set(1.0);
            self.balance[1].set(table(&R_MIX, balance, 51));
        } else {
            self.balance[0].set(table(&L_MIX, 100.0 - balance, 51));
            self.balance[1].set(1.0);
        }
        let att = table(&MONO_ATT, balance.min(100.0 - balance), 51);
        if stereo != self.pending_stereo || first {
            self.pending_stereo = stereo;
            if first {
                self.stereo = stereo;
            } else {
                self.mode_fade.start();
            }
        }
        self.wet_att.set(if stereo { 0.95 } else { att });
        self.dry_att = if self.stereo { 1.0 } else { 0.5 };
        self.feedback
            .set(values[4].round().clamp(0.0, 110.0) * 0.01);
        let slur = values[5].round().clamp(0.0, 10.0) as usize;
        self.apf_feedback = APF_FB[slur];
        self.apf_mix.set(if slur == 0 { 0.0 } else { 1.0 });
        self.filter_mix
            .set(values[6].round().clamp(0.0, 100.0) * 0.01);
        self.last.copy_from_slice(&values[..9]);
        self.bpm = bpm;
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        // filter sweep
        self.phase_ramp = self.phase_ramp.wrapping_add(self.inc_ramp);
        self.phase_lfo = self.phase_lfo.wrapping_add(self.inc_lfo);
        self.depth = self.depth_smooth.0 * self.depth_target + self.depth_smooth.1 * self.depth;
        let ramp = 0.5 * (self.phase_ramp as i32) as f32 * Q31 + 0.5;
        let control = 0.028_105_64 + ramp * self.depth * self.wave(self.phase_lfo);
        self.sweep = self.sweep_smooth.0 * control + self.sweep_smooth.1 * self.sweep;
        let (f, q) = self.sweep_coefficients(self.sweep);
        let mono = 0.5 * (frame[0] + frame[1]);
        let stage = clip1(self.svf[0].process(mono, f, q));
        let filtered = self.svf[1].process(stage, f, q);
        let filter_mix = self.filter_mix.tick();
        let mixed = (1.0 - filter_mix) * mono + 0.53 * filter_mix * filtered;
        let feedback = self.feedback.tick();
        let fed = mixed
            + self
                .feedback_hp
                .process(clip1(feedback * self.feedback_src));

        // ping-pong: delay A feeds delay B
        let mut outs = [0.0; 2];
        let mut input = fed;
        for (ch, out) in outs.iter_mut().enumerate() {
            let blend = self.times[ch].tick();
            let line = &mut self.lines[ch];
            line.write(flush(input));
            let taps = self.times[ch].taps;
            *out = blend * line.read_linear(taps[0]) + (1.0 - blend) * line.read_linear(taps[1]);
            input = *out;
        }

        // diffusion of delay B into the feedback
        let mut x = self.diff_lp.process(self.diff_hp.process(outs[1]));
        for diffuser in &mut self.diffusers {
            let tap = diffuser.line.read(diffuser.delay - 1);
            diffuser
                .line
                .write(flush(self.apf_feedback * tap + 0.45 * x));
            x = 0.93 * tap - 0.79 * x;
        }
        let apf = self.apf_mix.tick();
        self.feedback_src = (1.0 - apf) * outs[1] + apf * x;

        let (mode, switch) = self.mode_fade.tick();
        if switch {
            self.stereo = self.pending_stereo;
            self.dry_att = if self.stereo { 1.0 } else { 0.5 };
        }
        let wet = self.wet.tick() * self.wet_att.tick();
        let dry = self.dry.tick() * self.dry_att;
        let left = dry * frame[0] + wet * self.balance[0].tick() * outs[0];
        let right = wet * self.balance[1].tick() * outs[1] + dry * frame[1];
        if self.stereo {
            [mode * left, mode * right]
        } else {
            let sum = mode * (left + right);
            [sum, sum]
        }
    }

    fn clear(&mut self) {
        self.svf = [Svf::default(); 2];
        for line in &mut self.lines {
            line.clear();
        }
        self.feedback_hp.clear();
        self.feedback_src = 0.0;
        self.diff_hp.clear();
        self.diff_lp.clear();
        for d in &mut self.diffusers {
            d.line.clear();
        }
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(FltrPpd::new(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep_frequency_keeps_its_cutoff_across_rates() {
        let a = FltrPpd::new(44_100.0);
        let b = FltrPpd::new(96_000.0);
        let (fa, qa) = a.sweep_coefficients(0.5);
        assert!((fa - 0.65).abs() < 1e-3 && (qa - 0.2725).abs() < 1e-6);
        let (fb, _) = b.sweep_coefficients(0.5);
        let hz_a = (fa / 2.0).asin() * 44_100.0 / std::f32::consts::PI;
        let hz_b = (fb / 2.0).asin() * 96_000.0 / std::f32::consts::PI;
        assert!((hz_a - hz_b).abs() < 5.0, "{hz_a} {hz_b}");
    }

    #[test]
    fn delays_run_in_series() {
        let sr = 44_100.0;
        let mut effect = FltrPpd::new(sr);
        let mut values = crate::Model::FltrPpd.defaults();
        values[0] = 100.0;
        values[1] = 50.0;
        values[2] = 100.0;
        values[3] = 50.0;
        values[4] = 0.0;
        values[6] = 0.0; // dry, unfiltered input
        values[7] = 11.0; // stereo
        effect.set(&values, 120.0);
        let mut left = Vec::new();
        let mut right = Vec::new();
        for i in 0..sr as usize / 2 {
            let x = if i == 0 { 1.0 } else { 0.0 };
            let y = effect.process([x, x]);
            left.push(y[0]);
            right.push(y[1]);
        }
        let argmax = |v: &[f32]| {
            v.iter()
                .enumerate()
                .skip(10)
                .max_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
                .unwrap()
                .0
        };
        assert!((argmax(&left) as i32 - 4410).abs() < 3);
        assert!((argmax(&right) as i32 - 6615).abs() < 3);
    }

    #[test]
    fn max_feedback_is_bounded() {
        let mut effect = FltrPpd::new(48_000.0);
        let mut values = crate::Model::FltrPpd.defaults();
        values[4] = 110.0;
        values[5] = 10.0;
        effect.set(&values, 120.0);
        for i in 0..48_000 * 5 {
            let x = if i % 9000 < 500 { 0.9 } else { 0.0 };
            let y = effect.process([x, x]);
            assert!(y[0].is_finite() && y[0].abs() < 8.0);
        }
    }
}
