//! The tapped-line reverb of RVS_REV.ZDL and GATE_REV.ZDL (`Fx_REV_Rvs_Rev`,
//! `Fx_REV_GateRev`), following the recovered callbacks (zoom-ms70-cdr
//! `reverse_reverb/`, `gate_reverb/frame.rs`):
//!
//! stereo input filters → mono → four diffusers (`w = x + g d`,
//! `y = d + c x`) → four lattice all-passes (gain `g`) → their mix is written
//! into one long line read by 20 taps (positions from *_Decay_Tbl, gains from
//! *_FIR_MIX_Tbl: a rising ramp for ReverseRv, ±0.19 for GateRev); the first
//! 19 taps pass an 8 kHz low-pass, the last is added directly → tone shelf,
//! fixed 8 kHz low-pass, shelf and peaking sections (plus the HPF) →
//! `level · wet + Dry · input`.
//!
//! Editors (disassembly + firmware sweeps): `decay_edit` interpolates every
//! tap position from the 11-column *_Decay_Tbl, the all-pass gain from
//! *_APFFB_Tbl, the high-pass (second order, Q 0.707) from *_HPF_Tbl and, for
//! ReverseRv, the level compensation *_WetGain_Tbl; `tone_edit` designs a
//! Main type-8 high shelf at 12 kHz with *_ToneGain_Tbl dB (five points);
//! `Level_edit` reads *_Level_Tbl; `Dry` switches the dry path.
use super::common::{BLOCK, Design, Ramps, Ring, design, rom_biquad, scale, tbl};
use crate::dsp::{Biquad, Coefficients};
use crate::zoom::ORIGINAL_RATE;
use crate::{Effect, Values};

pub struct Config {
    pub fir_gains: [f32; 20],
    /// Taps 2..20, eleven Decay columns each.
    pub decay_taps: [[u32; 11]; 19],
    pub level_tbl: [f32; 11],
    pub wet_gain_tbl: Option<[f32; 11]>,
    pub hpf_tbl: [f32; 11],
    /// GateRev filters the input with the HPF; ReverseRv the output.
    pub hpf_at_input: bool,
    /// The diffusers' direct coefficient (c[41]: -0.3 / -1).
    pub direct: f32,
    /// Feedback write mix of the diffuser and lattice outputs.
    pub feedback: [f32; 2],
}

const APF_TBL: [f32; 11] = [
    0.15, 0.18, 0.2, 0.26, 0.32, 0.38, 0.4, 0.42, 0.45, 0.475, 0.5,
];
const TONE_DB: [f32; 5] = [-20.0, -9.0, -3.0, -1.0, 0.0];
const READS: [u32; 9] = [706, 1491, 2363, 3332, 4408, 5604, 6933, 8409, 8454];
const WRITES: [u32; 9] = [0, 707, 1492, 2364, 3333, 4409, 5605, 6934, 8410];
const INPUT_EQ: [f32; 5] = [0.966_023, -1.826_04, 0.878_709, 1.826_04, -0.844_732];
const SHELF: [f32; 5] = [0.746_582_5, -0.481_996_3, 0.0, 0.735_413_8, 0.0];
const PEAK: [f32; 5] = [0.995_086_2, -1.979_904, 0.985_211_8, 1.979_904, -0.980_298];
const STEP: f32 = 0.001_889;

const R_APF: usize = 0;
const R_LEVEL: usize = 1;
const R_DRY: usize = 2;
const R_TAPS: usize = 3; // 19
const R_TONE: usize = 22; // 3 words (first order)
const R_HPF: usize = 25; // 5 words
const RAMPS: usize = 30;

pub struct FirReverb {
    config: &'static Config,
    sample_rate: f32,
    ratio: f32,
    ring: Ring,
    reads: [usize; 9],
    writes: [usize; 9],
    input_eq: [Biquad; 2],
    hpf: [Biquad; 2],
    fir_lowpass: Biquad,
    tone: [Biquad; 2],
    lowpass: [Biquad; 2],
    shelf: [Biquad; 2],
    peak: [Biquad; 2],
    pending_feedback: f32,
    ramps: Ramps<RAMPS>,
    moving: bool,
    counter: usize,
    values: Option<Values>,
}

impl FirReverb {
    pub fn new(config: &'static Config, sample_rate: f32) -> Self {
        let longest = config
            .decay_taps
            .iter()
            .flatten()
            .copied()
            .max()
            .unwrap_or(0);
        let biquad =
            |row: [f32; 5]| Biquad::new(rom_biquad(row).resample(ORIGINAL_RATE, sample_rate));
        let lowpass = Biquad::new(design(Design::LowPass1, sample_rate, 8000.0, 0.707, 0.0));
        Self {
            config,
            sample_rate,
            ratio: sample_rate / ORIGINAL_RATE,
            ring: Ring::new(scale(longest, sample_rate) + 4),
            reads: READS.map(|o| scale(o, sample_rate)),
            writes: WRITES.map(|o| scale(o, sample_rate)),
            input_eq: [biquad(INPUT_EQ), biquad(INPUT_EQ)],
            hpf: [Biquad::default(), Biquad::default()],
            fir_lowpass: lowpass,
            tone: [Biquad::default(), Biquad::default()],
            lowpass: [lowpass, lowpass],
            shelf: [biquad(SHELF), biquad(SHELF)],
            peak: [biquad(PEAK), biquad(PEAK)],
            pending_feedback: 0.0,
            ramps: Ramps::new(STEP, BLOCK, sample_rate),
            moving: false,
            counter: 0,
            values: None,
        }
    }

    /// Tap position (44.1 kHz samples) of tap `n` (0..19 for taps 2..20).
    pub fn tap(config: &Config, n: usize, decay: f32) -> f32 {
        tbl(&config.decay_taps[n].map(|t| t as f32), 101, decay, 0.0)
    }

    fn apply_coefficients(&mut self) {
        let t = &self.ramps.current;
        let tone = Coefficients {
            b0: t[R_TONE],
            b1: t[R_TONE + 1],
            b2: 0.0,
            a1: t[R_TONE + 2],
            a2: 0.0,
        };
        let hpf = Coefficients {
            b0: t[R_HPF],
            b1: t[R_HPF + 1],
            b2: t[R_HPF + 2],
            a1: t[R_HPF + 3],
            a2: t[R_HPF + 4],
        };
        for ch in 0..2 {
            self.tone[ch].coefficients = tone;
            self.hpf[ch].coefficients = hpf;
        }
    }
}

impl Effect for FirReverb {
    /// `values`: Level, Tone, Decay, Dry.
    fn set(&mut self, values: &Values, _bpm: f32) {
        if self.values.as_ref() == Some(values) {
            return;
        }
        let first = self.values.is_none();
        self.values = Some(*values);
        let [level, tone, decay, dry, ..] = *values;
        let c = self.config;
        let t = &mut self.ramps.target;
        t[R_APF] = tbl(&APF_TBL, 101, decay, 0.0);
        let compensation = c.wet_gain_tbl.map_or(1.0, |w| tbl(&w, 101, decay, 0.0));
        t[R_LEVEL] = tbl(&c.level_tbl, 101, level, 0.0) * compensation;
        t[R_DRY] = if dry >= 0.5 { 1.0 } else { 0.0 };
        for n in 0..19 {
            t[R_TAPS + n] = Self::tap(c, n, decay) * self.ratio;
        }
        let shelf = design(
            Design::HighShelf1,
            self.sample_rate,
            12_000.0,
            0.707,
            tbl(&TONE_DB, 101, tone, 0.0),
        );
        t[R_TONE..R_TONE + 3].copy_from_slice(&[shelf.b0, shelf.b1, shelf.a1]);
        let hpf = design(
            Design::HighPass2,
            self.sample_rate,
            tbl(&c.hpf_tbl, 101, decay, 0.0),
            0.707,
            0.0,
        );
        t[R_HPF..R_HPF + 5].copy_from_slice(&[hpf.b0, hpf.b1, hpf.b2, hpf.a1, hpf.a2]);
        if first {
            self.ramps.jump();
            self.apply_coefficients();
        } else {
            self.moving = true;
        }
    }

    #[inline]
    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        if self.moving {
            self.counter += 1;
            if self.counter >= BLOCK {
                self.counter = 0;
                self.moving = self.ramps.tick();
                self.apply_coefficients();
            }
        }
        let c = self.config;
        let g = &self.ramps.current;
        let apf = g[R_APF];
        self.ring.advance();
        let ring = &self.ring;

        let mut filtered = [0.0; 2];
        for ch in 0..2 {
            let x = if c.hpf_at_input {
                self.hpf[ch].process(frame[ch])
            } else {
                frame[ch]
            };
            filtered[ch] = self.input_eq[ch].process(x);
        }
        let mono = 0.5 * (filtered[0] + filtered[1]);

        let mut writes = [0.0; 9];
        let mut x = mono;
        for (write, &offset) in writes.iter_mut().zip(&self.reads).take(4) {
            let d = ring.read(offset);
            *write = x + apf * d;
            x = d + c.direct * x;
        }
        let diffused = x;
        for (write, &offset) in writes.iter_mut().zip(&self.reads).skip(4).take(4) {
            let d = ring.read(offset);
            let a = d - apf * x;
            *write = x + apf * a;
            x = a;
        }
        let feedback = c.feedback[0] * diffused + c.feedback[1] * x;

        let mut sum = c.fir_gains[0] * ring.read(self.reads[8]);
        for n in 0..18 {
            sum += c.fir_gains[n + 1] * ring.read_frac(g[R_TAPS + n]);
        }
        let last = c.fir_gains[19] * ring.read_frac(g[R_TAPS + 18]);
        let wet = self.fir_lowpass.process(sum) + last;

        let ring = &mut self.ring;
        for (k, &value) in writes.iter().take(8).enumerate() {
            ring.write(self.writes[k], value);
        }
        // ReverseRv writes its feedback one frame late, GateRev at once.
        let line = if c.hpf_at_input {
            feedback
        } else {
            std::mem::replace(&mut self.pending_feedback, feedback)
        };
        ring.write(self.writes[8], line);

        let (level, dry) = (g[R_LEVEL], g[R_DRY]);
        let mut out = [0.0; 2];
        for ch in 0..2 {
            let mut y = self.tone[ch].process(wet);
            y = self.lowpass[ch].process(y);
            y = self.shelf[ch].process(y);
            y = self.peak[ch].process(y);
            if !c.hpf_at_input {
                y = self.hpf[ch].process(y);
            }
            out[ch] = level * y + dry * frame[ch];
        }
        out
    }

    fn clear(&mut self) {
        self.ring.clear();
        self.pending_feedback = 0.0;
        for b in self
            .input_eq
            .iter_mut()
            .chain(&mut self.hpf)
            .chain(&mut self.tone)
            .chain(&mut self.lowpass)
            .chain(&mut self.shelf)
            .chain(&mut self.peak)
        {
            b.clear();
        }
        self.fir_lowpass.clear();
    }
}
