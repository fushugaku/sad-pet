//! M-Filter (M_FILTER.ZDL): envelope filter with selectable type and slope.
//!
//! Callback `Fx_FLT_M_Filter` (0x7800..0x7f00), per sample:
//! - `m = (L + R) / 2`; detector: the AutoWah sections c7/c8/c9 and
//!   c10/c11/c12 (here the second keeps its own history), magnitude times the
//!   Sense gain c20 clipped to 1, peak hold with decay c13, `g = c21 peak`;
//! - envelope `e = c15 e + c14 g`; SVF coefficient `f = c18 + e^2 c19 c17`
//!   (written to c23);
//! - two cascaded Chamberlin SVFs, input gain c16, damping c24, every state
//!   and output clipped to +-1, output `c25 high + c26 band + c27 low`; c22
//!   selects the second (4-pole) or the first (2-pole) output;
//! - `out = c2 (x c6 + c3 c5 y)` per channel (c0 = 1).
//!
//! Editors decoded from the disassembly:
//! - Freq/Reso (`MFlt_SVF_Q_Fc`): `Q = 1.5 + 18.5 (Reso / 10)^2`;
//!   c16 = dB->linear(Q w0) and c24 = w1 / Q with (w0, w1) = (-0.3, 1) for
//!   2Pole, (-0.4, 2) for 4Pole (0x80000548), both through Main's 16 ms
//!   timed setter; `c18 = 0.00285 + 0.13951 (Freq / 100)^2` (20 Hz..1 kHz at
//!   rest) and `c17 = 1 + 0.7321 (Reso / 10)^2 - c18` (the sweep top).
//! - Sense (`Sens`): c20 = dB->linear(3 Sense) (0..+30 dB).
//! - Type: c25..c27 = HPF/BPF/LPF one-hot (0x80000508); Chara/VLCTY
//!   (`Chara`): c22 = Chara, (c14, c15) = 0x80000558 (Fast 0.002/0.998, Slow
//!   0.0007/0.9993), then the SVF editor. These fade c3 out and in.
//! - Bal: c5 = curve(Bal, shape 2), c6 = curve(100 - Bal, shape 2).
//!   Level: c2 = Level / 100.
//!
//! Freq, Sense and Reso follow fractional (modulated) values continuously;
//! the curves are continuous in the original too.
use super::common::{
    Duck, FirstOrder, GLIDE_16MS, Glide, STEP_DEFAULT, SvfFrequency, clamp_value, decay, one_pole,
    step, w,
};
use crate::dsp::{db_to_gain, flush};
use crate::zoom::{Ramp, curve};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_m_filter",
    name: "M-Filter",
    category: Category::Filter,
    zdl: "M_FILTER.ZDL",
    description: "Envelope filter: HPF/BPF/LPF, 2- or 4-pole",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Freq",
            max: 100,
            default: 56,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Sense",
            max: 10,
            default: 5,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Reso",
            max: 10,
            default: 7,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Type",
            max: 2,
            default: 2,
            display: Display::labels(&["HPF", "BPF", "LPF"]),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Chara",
            max: 1,
            default: 1,
            display: Display::labels(&["2Pole", "4Pole"]),
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "VLCTY",
            max: 1,
            default: 0,
            display: Display::labels(&["Fast", "Slow"]),
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "Bal",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[7] is firmware parameter word 9
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

/// ROM block 0x80000588 (44.1 kHz).
const DETECTOR_FIRST: [f32; 3] = [w(0x3f7c_2496), w(0xbf78_9347), w(0x3f74_b7dc)];
const DETECTOR_SECOND: [f32; 3] = [w(0x3ffd_9a99), w(0xbffa_01ed), w(0x3f78_cea7)];
const PEAK_DECAY: f32 = w(0x3f7f_be77);
const C19: f32 = 1.0;
const C21: f32 = w(0x3f7f_be77);
/// 0x80000548: (w0, w1) per Chara.
const CHARA: [(f32, f32); 2] = [(w(0xbe99_999a), 1.0), (w(0xbecc_cccd), 2.0)];
/// 0x80000558: (c14, c15) per VLCTY.
const VELOCITY: [(u32, u32); 2] = [(0x3b03_126f, 0x3f7f_7cee), (0x3a37_8034, 0x3f7f_d220)];
/// 0x80000508: (c25, c26, c27) per Type.
const TYPES: [[f32; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

#[derive(Debug, Clone, Copy, PartialEq)]
struct Structure {
    kind: usize,
    four_pole: bool,
    velocity: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Settings {
    structure: Structure,
    c16: f32,
    c24: f32,
    c17: f32,
    c18: f32,
    c20: f32,
}

#[derive(Debug, Clone, Copy, Default)]
struct Svf {
    band: f32,
    low: f32,
}

impl Svf {
    #[inline]
    fn tick(&mut self, x: f32, f: f32, damping: f32, mix: &[f32; 3]) -> f32 {
        let high = (x - damping * self.band - self.low).clamp(-1.0, 1.0);
        self.band = flush((self.band + high * f).clamp(-1.0, 1.0));
        self.low = flush((self.low + self.band * f).clamp(-1.0, 1.0));
        (mix[0] * high + mix[1] * self.band + mix[2] * self.low).clamp(-1.0, 1.0)
    }
}

struct MFilter {
    first: FirstOrder,
    second: FirstOrder,
    c13: f32,
    velocity: [(f32, f32); 2],
    structure: Structure,
    active: bool,
    pending: Option<Settings>,
    duck: Duck,
    c16: Glide,
    c24: Glide,
    c17: Glide,
    c18: Glide,
    c20: f32,
    c5: Ramp,
    c6: Ramp,
    level: Ramp,
    frequency: SvfFrequency,
    input: f32,
    first_out: f32,
    second_out: f32,
    peak: f32,
    env: f32,
    sections: [Svf; 2],
    last: Option<Values>,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let [b0, b1, a] = DETECTOR_FIRST;
    let first = FirstOrder::rom(b0, b1, a, sample_rate);
    let [b0, b1, a] = DETECTOR_SECOND;
    let second = FirstOrder::rom(b0, b1, a, sample_rate);
    let mut effect = MFilter {
        first,
        second,
        c13: decay(PEAK_DECAY, sample_rate),
        velocity: VELOCITY.map(|(b, a)| one_pole(w(b), w(a), sample_rate)),
        structure: Structure {
            kind: 2,
            four_pole: true,
            velocity: 0,
        },
        active: false,
        pending: None,
        duck: Duck::new(sample_rate),
        c16: Glide::new(1.0, GLIDE_16MS, sample_rate),
        c24: Glide::new(1.0, GLIDE_16MS, sample_rate),
        c17: Glide::new(0.3, GLIDE_16MS, sample_rate),
        c18: Glide::new(0.0, GLIDE_16MS, sample_rate),
        c20: 12.0,
        c5: Ramp::new(1.0, STEP_DEFAULT, sample_rate),
        c6: Ramp::new(0.0, STEP_DEFAULT, sample_rate),
        level: Ramp::new(1.0, STEP_DEFAULT, sample_rate),
        frequency: SvfFrequency::new(sample_rate),
        input: 0.0,
        first_out: 0.0,
        second_out: 0.0,
        peak: 0.0,
        env: 0.0,
        sections: [Svf::default(); 2],
        last: None,
    };
    let mut values = [0.0; crate::MAX_PARAMS];
    for (value, param) in values.iter_mut().zip(INFO.params) {
        *value = f32::from(param.default);
    }
    effect.set(&values, 120.0);
    Box::new(effect)
}

fn settings(values: &Values) -> Settings {
    let freq = clamp_value(values[0], 100.0);
    let sense = clamp_value(values[1], 10.0);
    let reso = clamp_value(values[2], 10.0);
    let structure = Structure {
        kind: step(values[3], 2),
        four_pole: step(values[4], 1) == 1,
        velocity: step(values[5], 1),
    };
    let q = curve(20.0, 1.5, 11, reso, 1);
    let (w0, w1) = CHARA[usize::from(structure.four_pole)];
    let c18 = curve(w(0x3e11_c59c), w(0x3b3a_bef2), 101, freq, 1);
    Settings {
        structure,
        c16: db_to_gain(q * w0),
        c24: w1 / q,
        c17: curve(w(0x3fdd_b3d7), 1.0, 11, reso, 1) - c18,
        c18,
        c20: db_to_gain(curve(30.0, 0.0, 11, sense, 0)),
    }
}

impl MFilter {
    fn jump(&mut self, s: &Settings) {
        self.structure = s.structure;
        self.c16.jump(s.c16);
        self.c24.jump(s.c24);
        self.c17.jump(s.c17);
        self.c18.jump(s.c18);
        self.c20 = s.c20;
        self.active = true;
    }
}

impl Effect for MFilter {
    fn set(&mut self, values: &Values, _bpm: f32) {
        if self.last.as_ref() == Some(values) {
            return;
        }
        self.last = Some(*values);
        let s = settings(values);
        let balance = clamp_value(values[6], 100.0);
        let c5 = curve(1.0, 0.0, 101, balance, 2);
        let c6 = curve(1.0, 0.0, 101, 100.0 - balance, 2);
        let level = clamp_value(values[7], 150.0) / 100.0;
        if !self.active {
            self.jump(&s);
            self.c5.jump(c5);
            self.c6.jump(c6);
            self.level.jump(level);
            return;
        }
        if s.structure != self.structure || self.duck.is_falling() {
            self.pending = Some(s);
            self.duck.start();
        } else {
            self.c16.set(s.c16);
            self.c24.set(s.c24);
            self.c17.set(s.c17);
            self.c18.set(s.c18);
            self.c20 = s.c20;
        }
        self.c5.set(c5);
        self.c6.set(c6);
        self.level.set(level);
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        if self.duck.tick()
            && let Some(s) = self.pending.take()
        {
            self.jump(&s);
        }
        let c16 = self.c16.tick();
        let c24 = self.c24.tick();
        let c17 = self.c17.tick();
        let c18 = self.c18.tick();
        let c5 = self.c5.tick();
        let c6 = self.c6.tick();
        let level = self.level.tick();
        let (c14, c15) = self.velocity[self.structure.velocity];
        let x = 0.5 * (frame[0] + frame[1]);
        let first = self.first.b0 * x + self.first.b1 * self.input + self.first.a * self.first_out;
        let second = self.second.b0 * first
            + self.second.b1 * self.first_out
            + self.second.a * self.second_out;
        self.input = x;
        self.first_out = flush(first);
        self.second_out = flush(second);
        let candidate = (second.abs() * self.c20).min(1.0);
        self.peak = if self.peak < candidate {
            candidate
        } else {
            flush(self.peak * self.c13)
        };
        self.env = flush(self.env * c15 + C21 * self.peak * c14);
        let f = self.frequency.map(c18 + self.env * self.env * C19 * c17);
        let mix = &TYPES[self.structure.kind];
        let one = self.sections[0].tick(c16 * x, f, c24, mix);
        let two = self.sections[1].tick(c16 * one, f, c24, mix);
        let selected = if self.structure.four_pole { two } else { one };
        let wet = self.duck.gain * c5 * selected;
        [(frame[0] * c6 + wet) * level, (frame[1] * c6 + wet) * level]
    }

    fn clear(&mut self) {
        self.input = 0.0;
        self.first_out = 0.0;
        self.second_out = 0.0;
        self.peak = 0.0;
        self.env = 0.0;
        self.sections = [Svf::default(); 2];
        if let Some(s) = self.pending.take() {
            self.jump(&s);
        }
        self.duck.reset();
        self.active = false;
        self.last = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    fn gain(effect: &mut dyn Effect, hz: f32, amplitude: f32, sample_rate: f32) -> f32 {
        let frames = sample_rate as usize;
        let mut peak = 0.0f32;
        for i in 0..frames {
            let x = amplitude * (std::f32::consts::TAU * hz * i as f32 / sample_rate).sin();
            let y = effect.process([x, x])[0];
            if i > frames / 2 {
                peak = peak.max(y.abs());
            }
        }
        peak / amplitude
    }

    fn with(changes: &[(usize, f32)], sample_rate: f32) -> Box<dyn Effect> {
        let mut values = Model::MFilter.defaults();
        for &(i, v) in changes {
            values[i] = v;
        }
        let mut effect = create(sample_rate);
        effect.set(&values, 120.0);
        effect.clear();
        effect.set(&values, 120.0);
        effect
    }

    #[test]
    fn freq_sets_the_resting_cutoff() {
        // Freq 100, Sense 0, quiet input: f = 0.1424 (about 1 kHz), 2-pole LPF.
        let rest = &[(0, 100.0), (1, 0.0), (2, 0.0), (4, 0.0)];
        let mut below = with(rest, 48_000.0);
        let mut above = with(rest, 48_000.0);
        let a = gain(below.as_mut(), 200.0, 1e-3, 48_000.0);
        let b = gain(above.as_mut(), 8000.0, 1e-3, 48_000.0);
        // c16 at Reso 0 is -0.45 dB.
        assert!((a - db_to_gain(-0.45)).abs() < 0.05, "{a}");
        assert!(b < 0.05 * a, "{b}");
    }

    #[test]
    fn types_select_high_band_and_low_pass() {
        let base = [(0, 60.0), (1, 0.0), (2, 0.0), (4, 0.0)];
        let mut gains = [[0.0; 2]; 3];
        for (kind, row) in gains.iter_mut().enumerate() {
            let mut changes = base.to_vec();
            changes.push((3, kind as f32));
            for (slot, hz) in [50.0, 5000.0].into_iter().enumerate() {
                let mut effect = with(&changes, 44_100.0);
                row[slot] = gain(effect.as_mut(), hz, 1e-3, 44_100.0);
            }
        }
        assert!(gains[0][1] > 5.0 * gains[0][0], "HPF {:?}", gains[0]);
        assert!(gains[2][0] > 5.0 * gains[2][1], "LPF {:?}", gains[2]);
        assert!(gains[1][0] < 0.5 && gains[1][1] < 0.5, "BPF {:?}", gains[1]);
    }

    #[test]
    fn four_pole_rolls_off_faster() {
        let base = [(0, 60.0), (1, 0.0), (2, 0.0)];
        let mut two = base.to_vec();
        two.push((4, 0.0));
        let mut four = base.to_vec();
        four.push((4, 1.0));
        let a = gain(with(&two, 44_100.0).as_mut(), 6000.0, 1e-3, 44_100.0);
        let b = gain(with(&four, 44_100.0).as_mut(), 6000.0, 1e-3, 44_100.0);
        assert!(b < 0.2 * a, "{a} {b}");
    }

    #[test]
    fn loud_input_raises_the_cutoff() {
        let changes = [(0, 30.0), (1, 10.0), (2, 0.0), (4, 0.0)];
        let quiet = gain(with(&changes, 48_000.0).as_mut(), 3000.0, 1e-4, 48_000.0);
        let loud = gain(with(&changes, 48_000.0).as_mut(), 3000.0, 0.3, 48_000.0);
        assert!(loud > 5.0 * quiet, "{quiet} {loud}");
    }
}
