//! The audio kernel shared byte for byte by HALL, ROOM, SPRING, TLDROOM and
//! ARENA (and the Hall stage of MODREV), with its editors.
//!
//! Callback (zoom-ms70-cdr `hall_reverb.rs`): the stereo half-sum times c20
//! runs through the ROM input section c21..c23 and the Tone low-pass
//! c24/c25/c27, and is written into one ring at offset c50. Fifteen taps
//! c5..c19 are read before the frame's seven writes c50..c56:
//!
//! * t0 (c5) is the pre-delay, scaled by the pre-delay duck c30;
//! * two diffusers: `w = c31 x + c32 t`, `y = c34 t + c33 x` over t1, t2;
//! * the diffuser output times c35 feeds four parallel combs t3..t6 whose
//!   feedback goes through the gains c38..c41 and a one-pole low-pass
//!   (c36 input, c37 pole), written back at c53..c56;
//! * the left output sums t7, t9, t11, t13 times c43, the right t8, t10, t12,
//!   t14 times c44, each through the first-order Tone shelf c45..c47.
//!
//! Editors (`HALL.ZDL.dis` and its siblings, identical code over per-model
//! tables): Decay ramps c33, c32 and c38..c41 from `decay_tbl[Decay]` at step
//! 0x3af7c5ec; Tone stores `tone1_tbl[Tone]` into c24, c25, c27 and copies
//! `tone2_tbl[Tone]` into c45..c47; PreD ducks c30 to 0, sets c5 to
//! `(PreD + 1) * 441 / 10` samples and brings c30 back; Mix sets c48 to the
//! Main curve `(Mix / 100)^1.41` over 16 ms; Level sets c2 = Level / 100.
//! The output is `Level * (dry + c48 * wet)`: the dry signal is never cut.
//!
//! Port notes: delays are re-laid for the host rate (each ring segment keeps
//! its tap delays scaled from 44.1 kHz), ROM first-order sections are
//! re-derived through their analog prototype and the comb damping keeps its
//! time constant. The original Decay setter moves its six coefficients one
//! after another; here they move together at the same step.
use super::common::{FirstOrder, Glides, Ring, STEP_FAST, f, place, stepped};
use crate::zoom::ORIGINAL_RATE;

/// One model's ROM: tap tables, the c20..c49 init block and the editor tables.
pub struct HallRom {
    pub reads: [u32; 15],
    pub writes: [u32; 7],
    pub init: [u32; 30],
    pub decay: [[u32; 6]; 30],
    pub tone1: [[u32; 3]; 11],
    pub tone2: [[u32; 3]; 11],
}

impl HallRom {
    /// Coefficient `index` (20..=49) of the init block.
    fn c(&self, index: usize) -> f32 {
        f(self.init[index - 20])
    }
}

/// Largest PreD value (shown 100 ms).
pub const PREDELAY_MAX: u16 = 99;

const DUCK: usize = 0;
const C32: usize = 1;
const C33: usize = 2;
const GAIN: usize = 3;

/// Pre-delay in samples for a PreD value, `(PreD + 1) * 44.1` truncated at 44.1 kHz.
fn predelay_samples(value: usize, rate: f32) -> usize {
    (((value + 1) as f32) * rate * 0.001) as usize
}

pub struct HallCore {
    rom: &'static HallRom,
    rate: f32,
    ring: Ring,
    read: [usize; 15],
    write: [usize; 7],
    input: FirstOrder,
    tone_in: FirstOrder,
    tone_out: [FirstOrder; 2],
    damp_gain: f32,
    damp_pole: f32,
    lowpass: [f32; 4],
    c31: f32,
    c34: f32,
    c35: f32,
    c42: f32,
    c43: f32,
    c44: f32,
    glides: Glides<7>,
    decay: Option<usize>,
    tone: Option<usize>,
    predelay: Option<usize>,
    pending_predelay: Option<usize>,
}

impl HallCore {
    pub fn new(rom: &'static HallRom, rate: f32) -> Self {
        let mut min_lengths = [0; 7];
        min_lengths[0] = predelay_samples(usize::from(PREDELAY_MAX), rate);
        let (write, mut read, total) = place(&rom.writes, &rom.reads, min_lengths, rate);
        read[0] = write[0] + predelay_samples(0, rate);
        let (damp_gain, damp_pole) = super::common::one_pole_at(rom.c(36), rom.c(37), rate);
        let c = |i| rom.c(i);
        Self {
            rom,
            rate,
            ring: Ring::new(total),
            read,
            write,
            input: FirstOrder::resampled(c(21), c(22), c(23), ORIGINAL_RATE, rate),
            tone_in: FirstOrder::default(),
            tone_out: [FirstOrder::default(); 2],
            damp_gain,
            damp_pole,
            lowpass: [0.0; 4],
            c31: c(31),
            c34: c(34),
            c35: c(35),
            c42: c(42),
            c43: c(43),
            c44: c(44),
            glides: Glides::new([1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], rate),
            decay: None,
            tone: None,
            predelay: None,
            pending_predelay: None,
        }
    }

    /// Decay 0..29: ramps the diffusers and comb gains (jumps when `jump`).
    pub fn set_decay(&mut self, decay: usize, jump: bool) {
        if self.decay == Some(decay) {
            return;
        }
        self.decay = Some(decay);
        let row = self.rom.decay[decay.min(29)];
        let targets = [
            (C33, f(row[4])),
            (C32, f(row[5])),
            (GAIN, f(row[0])),
            (GAIN + 1, f(row[1])),
            (GAIN + 2, f(row[2])),
            (GAIN + 3, f(row[3])),
        ];
        for (index, value) in targets {
            if jump {
                self.glides.jump(index, value);
            } else {
                self.glides.to(index, value, STEP_FAST);
            }
        }
    }

    /// Tone 0..10: stores both ROM sections at once, as the editor does.
    pub fn set_tone(&mut self, tone: usize) {
        if self.tone == Some(tone) {
            return;
        }
        self.tone = Some(tone);
        let one = self.rom.tone1[tone.min(10)];
        let two = self.rom.tone2[tone.min(10)];
        self.tone_in
            .set_resampled(f(one[0]), f(one[1]), f(one[2]), ORIGINAL_RATE, self.rate);
        for section in &mut self.tone_out {
            section.set_resampled(f(two[0]), f(two[1]), f(two[2]), ORIGINAL_RATE, self.rate);
        }
    }

    /// PreD 0..99: ducks the reverb input, moves the tap, restores the input.
    pub fn set_predelay(&mut self, value: usize, jump: bool) {
        if self.predelay == Some(value) {
            return;
        }
        self.predelay = Some(value);
        if jump {
            self.apply_predelay(value);
            self.glides.jump(DUCK, 1.0);
            self.pending_predelay = None;
        } else {
            self.pending_predelay = Some(value);
            self.glides.to(DUCK, 0.0, STEP_FAST);
        }
    }

    fn apply_predelay(&mut self, value: usize) {
        self.read[0] = self.write[0] + predelay_samples(value, self.rate);
    }

    /// The pre-delay tap in samples (tests).
    #[cfg(test)]
    pub fn predelay_tap(&self) -> usize {
        self.read[0] - self.write[0]
    }

    /// One frame of the mono reverb input; returns the filtered wet pair.
    #[inline]
    pub fn process(&mut self, input: f32) -> [f32; 2] {
        self.glides.tick();
        if let Some(value) = self.pending_predelay
            && self.glides.value[DUCK] == 0.0
        {
            self.apply_predelay(value);
            self.pending_predelay = None;
            self.glides.to(DUCK, 1.0, STEP_FAST);
        }
        let g = &self.glides.value;
        let ring = &self.ring;
        let r = &self.read;
        let t: [f32; 15] = std::array::from_fn(|j| ring.read(r[j]));

        let first = self.input.process(input);
        let third = self.tone_in.process(first);

        let x = g[DUCK] * t[0];
        let (c32, c33) = (g[C32], g[C33]);
        let w1 = self.c31 * x + c32 * t[1];
        let mid = self.c34 * t[1] + c33 * x;
        let w2 = self.c31 * mid + c32 * t[2];
        let diffusion = self.c34 * t[2] + c33 * mid;
        let shared = self.c35 * diffusion;
        let mut combs = [0.0f32; 4];
        for (k, comb) in combs.iter_mut().enumerate() {
            let lowpass = &mut self.lowpass[k];
            *lowpass = crate::dsp::flush(
                self.damp_pole * *lowpass + self.damp_gain * (g[GAIN + k] * t[3 + k]),
            );
            *comb = shared + *lowpass;
        }
        let wet_left = self.c42 * t[0] + self.c43 * (t[7] + t[9] + t[11] + t[13]);
        let wet_right = self.c42 * t[0] + self.c44 * (t[8] + t[10] + t[12] + t[14]);
        let out = [
            self.tone_out[0].process(wet_left),
            self.tone_out[1].process(wet_right),
        ];

        let w = &self.write;
        let ring = &mut self.ring;
        ring.write(w[0], third);
        ring.write(w[1], w1);
        ring.write(w[2], w2);
        for (k, comb) in combs.into_iter().enumerate() {
            ring.write(w[3 + k], comb);
        }
        ring.advance();
        out
    }

    pub fn clear(&mut self) {
        self.ring.clear();
        self.input.clear();
        self.tone_in.clear();
        for section in &mut self.tone_out {
            section.clear();
        }
        self.lowpass = [0.0; 4];
    }
}

/// Main curve with shape 5 used by the Mix editors: `(Mix / 100)^1.41`.
pub fn mix_curve(mix: f32) -> f32 {
    crate::zoom::curve(1.0, 0.0, 101, mix.clamp(0.0, 100.0), 5)
}

const MIX: usize = 0;
const LEVEL: usize = 1;

/// The standalone Hall/Room/Spring/TiledRoom/Arena effect.
pub struct HallModel {
    core: HallCore,
    glides: Glides<2>,
    mix: f32,
    level: f32,
    primed: bool,
}

impl HallModel {
    pub fn new(rom: &'static HallRom, rate: f32) -> Self {
        Self {
            core: HallCore::new(rom, rate),
            glides: Glides::new([0.0, 1.0], rate),
            mix: -1.0,
            level: -1.0,
            primed: false,
        }
    }
}

impl crate::Effect for HallModel {
    fn set(&mut self, values: &crate::Values, _bpm: f32) {
        let jump = !self.primed;
        self.primed = true;
        self.core.set_decay(stepped(values[0], 29), jump);
        self.core.set_tone(stepped(values[1], 10));
        self.core
            .set_predelay(stepped(values[3], PREDELAY_MAX), jump);
        if values[2] != self.mix {
            self.mix = values[2];
            let target = mix_curve(values[2]);
            if jump {
                self.glides.jump(MIX, target);
            } else {
                self.glides.over(MIX, target, super::common::SETTER_SAMPLES);
            }
        }
        if values[4] != self.level {
            self.level = values[4];
            let target = values[4].clamp(0.0, 150.0) / 100.0;
            if jump {
                self.glides.jump(LEVEL, target);
            } else {
                self.glides.to(LEVEL, target, crate::zoom::DEFAULT_STEP);
            }
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let wet = self.core.process(0.5 * (left + right));
        self.glides.tick();
        let [mix, level] = self.glides.value;
        [
            level * (left + mix * wet[0]),
            level * (right + mix * wet[1]),
        ]
    }

    fn clear(&mut self) {
        self.core.clear();
    }
}

/// Parameter list shared by the five models, with their defaults.
macro_rules! hall_family_params {
    ($decay:expr, $tone:expr, $mix:expr, $predelay:expr) => {
        &[
            // values[0] is firmware parameter word 2
            ParamInfo {
                name: "Decay",
                max: 29,
                default: $decay,
                display: Display::offset(1.0),
            },
            // values[1] is firmware parameter word 3
            ParamInfo {
                name: "Tone",
                max: 10,
                default: $tone,
                display: Display::INT,
            },
            // values[2] is firmware parameter word 4
            ParamInfo {
                name: "Mix",
                max: 100,
                default: $mix,
                display: Display::INT,
            },
            // values[3] is firmware parameter word 5
            ParamInfo {
                name: "PreD",
                max: 99,
                default: $predelay,
                display: Display::number(1.0, 1.0, 0, " ms"),
            },
            // values[4] is firmware parameter word 6
            ParamInfo {
                name: "Level",
                max: 150,
                default: 100,
                display: Display::INT,
            },
        ]
    };
}
pub(crate) use hall_family_params;

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{Effect, Values};

    /// Wet energy between `from` and `to` seconds after one impulse.
    pub fn tail_energy(effect: &mut dyn Effect, rate: f32, from: f32, to: f32) -> f32 {
        let mut energy = 0.0;
        for n in 0..(rate * to) as usize {
            let x = if n == 0 { 1.0 } else { 0.0 };
            let [l, r] = effect.process([x, x]);
            if n as f32 >= rate * from {
                energy += (l - x) * (l - x) + (r - x) * (r - x);
            }
        }
        energy
    }

    /// Frames until the wet output first exceeds `threshold` after an impulse.
    pub fn first_arrival(effect: &mut dyn Effect, rate: f32, threshold: f32) -> usize {
        for n in 0..(rate as usize * 2) {
            let x = if n == 0 { 1.0 } else { 0.0 };
            let [l, r] = effect.process([x, x]);
            if (l - x).abs() > threshold || (r - x).abs() > threshold {
                return n;
            }
        }
        usize::MAX
    }

    pub fn values(list: &[f32]) -> Values {
        let mut values = [0.0; crate::MAX_PARAMS];
        values[..list.len()].copy_from_slice(list);
        values
    }

    #[test]
    fn layout_keeps_the_rom_delays_at_44k() {
        let core = HallCore::new(&super::super::rom::HALL, 44_100.0);
        // Comb 0 loop: write c53 to read c8, 12368 - 6944 samples.
        assert_eq!(core.read[3] - core.write[3], 12368 - 6944);
        assert_eq!(core.read[6] - core.write[6], 25443 - 21920);
        assert_eq!(core.read[1] - core.write[1], 5836 - 4412);
        // Output tap t7 sits 1280 samples into comb 0.
        assert_eq!(core.read[7] - core.write[3], 8224 - 6944);
    }

    #[test]
    fn predelay_follows_the_editor() {
        for rate in [44_100.0, 96_000.0] {
            let mut model = HallModel::new(&super::super::rom::HALL, rate);
            model.set(&values(&[9.0, 5.0, 100.0, 48.0, 100.0]), 120.0);
            assert_eq!(model.core.predelay_tap(), (49.0 * rate / 1000.0) as usize);
            // The first reflections reach the output after the pre-delay plus
            // the shortest comb output tap (t13: 45 samples into comb 3) and
            // the two diffusers' first path.
            let arrival = first_arrival(&mut model, rate, 1e-6);
            let expected = model.core.predelay_tap();
            assert!(arrival > expected, "{rate}: {arrival} <= {expected}");
            model.set(&values(&[9.0, 5.0, 100.0, 0.0, 100.0]), 120.0);
            for _ in 0..(rate as usize / 10) {
                model.process([0.0; 2]);
            }
            assert_eq!(model.core.predelay_tap(), (rate / 1000.0) as usize);
        }
    }

    #[test]
    fn decay_lengthens_the_tail() {
        let rate = 48_000.0;
        let mut short = HallModel::new(&super::super::rom::HALL, rate);
        short.set(&values(&[0.0, 5.0, 100.0, 0.0, 100.0]), 120.0);
        let mut long = HallModel::new(&super::super::rom::HALL, rate);
        long.set(&values(&[29.0, 5.0, 100.0, 0.0, 100.0]), 120.0);
        let short = tail_energy(&mut short, rate, 1.0, 2.0);
        let long = tail_energy(&mut long, rate, 1.0, 2.0);
        assert!(long > 100.0 * short, "{long} {short}");
    }

    #[test]
    fn mix_curve_is_the_power_curve() {
        assert!((mix_curve(100.0) - 1.0).abs() < 1e-6);
        assert!((mix_curve(46.0) - 0.46f32.powf(1.41)).abs() < 1e-4);
        assert_eq!(mix_curve(0.0), 0.0);
    }

    #[test]
    fn dry_passes_at_mix_zero() {
        let mut model = HallModel::new(&super::super::rom::ARENA, 44_100.0);
        model.set(&values(&[29.0, 10.0, 0.0, 0.0, 100.0]), 120.0);
        for n in 0..10_000 {
            let x = ((n as f32) * 0.05).sin() * 0.5;
            let [l, r] = model.process([x, -x]);
            assert!((l - x).abs() < 1e-6 && (r + x).abs() < 1e-6);
        }
    }
}
