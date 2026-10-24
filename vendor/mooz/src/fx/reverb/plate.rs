//! Plate (PLATE.ZDL): four lossy input diffusers, five two-band feedback
//! lanes, output diffusers and a separate early-reflection line.
//!
//! Callback (zoom-ms70-cdr `plate.rs`): the stereo half-sum runs through the
//! ROM high-pass c30..c32 and EQ biquad c33..c37 and is written (times c38)
//! into the ring. The pre-delay tap c6 times c39 passes four diffusers
//! (`w = c40 x + c41 d` through the HiDMP low-pass c85..c87, `y = c42 x +
//! c43 d`) and excites five comb lanes. Each lane splits its tap with the
//! one-pole c45 (~200 Hz) into low and high bands weighted by c46+2k (low)
//! and c47+2k (high), adds the excitation, and writes back through the HiDMP
//! low-pass c85..c87 and the LoDMP high-pass c88..c90. Ten lane taps times
//! c56/c57 go through two unfiltered diffusers per side, times c58, and the
//! ROM sections c59..c63 (high-pass) and c64..c68 (shelf). A second ring
//! region holds the pre-delayed signal: four taps c113..c116 (gains c92..c95)
//! form `reflected = c39 (pre + sum)`, written at c91 and read by eight taps
//! c117..c124 (gains c96..c103 left, c104..c111 right). The output is
//! `Level * (c70 dry + c69 c3 (tail + c112 early))`.
//!
//! Editors (all decoded from `PLATE.ZDL.dis`; Decay/Color also recovered as
//! `plate_parameters.rs`): Decay gives a time `T` in ms (347.66..7743.75 by
//! a square curve up to 90, then linearly to 61250), Color a factor
//! 0.01..1; each lane gets `0.01^(P / (Color T))` for its low band and
//! `0.01^(P / T)` for its high band (P: the lane periods 96.4, 86.56, 82.64,
//! 69.52, 61.04 ms), the diffusion c41 steps from 0.375 to 0.75 with `T` and
//! c43 = -0.5, all ramped at step 0x3af7c5ec. PreD ducks c39, sets c6 to
//! `(PreD + 1) * 441 / 10` samples and restores c39. Mix sets c69 =
//! `(Mix / 99)^1.41` and keeps the dry c70 = 1 up to 90, fading it as
//! `1 - ((Mix - 91) / 9)^1.41` above (both over 16 ms). LoDMP designs Main's
//! first-order high-pass at `round(360 - 340 (LoDMP / 100)^1.62)` Hz, HiDMP
//! the first-order low-pass at `round(1600 + 20400 (HiDMP / 100)^2)` Hz.
//! Level sets c2 = Level / 100.
//!
//! Port notes: delays are scaled from 44.1 kHz and the ROM sections
//! re-derived; the damping filters are designed at the host rate (the HiDMP
//! corner is kept below Nyquist).
use super::{
    common::{
        Biquad, FirstOrder, Glides, Ring, SETTER_SAMPLES, STEP_FAST, f, place, rom_biquad, stepped,
    },
    rom::{PLATE_COE, PLATE_COMB_PERIODS},
};
use crate::{Effect, Values, catalog::*, zoom};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_plate",
    name: "Plate",
    category: Category::Reverb,
    zdl: "PLATE.ZDL",
    description: "Plate reverb with color and low/high damping",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "PreD",
            max: 199,
            default: 8,
            display: Display::number(1.0, 1.0, 0, " ms"),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Decay",
            max: 100,
            default: 52,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 44,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Color",
            max: 100,
            default: 58,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "LoDMP",
            max: 100,
            default: 97,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "HiDMP",
            max: 100,
            default: 95,
            display: Display::INT,
        },
        // values[6] is firmware parameter word 9
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Plate::new(sample_rate))
}

fn c(index: usize) -> f32 {
    f(PLATE_COE[index])
}

/// Decay time in ms for a Decay value (`Fx_REV_Plate_decay_Calc_set`).
pub fn decay_ms(decay: usize) -> f32 {
    if decay < 91 {
        zoom::curve(f(0x45f1_fe00), f(0x43ad_d429), 91, decay as f32, 1)
    } else {
        zoom::curve(f(0x476f_4200), f(0x45f1_fe00), 11, (decay - 90) as f32, 0)
    }
}

/// Color factor 0.01..1.
pub fn color_factor(color: usize) -> f32 {
    zoom::curve(1.0, f(0x3c23_d70a), 101, color as f32, 0)
}

/// Diffusion c41 chosen by the decay time.
pub fn diffusion(time: f32) -> f32 {
    f(if time < 244.0 {
        0x3ec0_0180
    } else if time < 500.0 {
        0x3f06_670d
    } else if time < 1280.0 {
        0x3f19_9933
    } else if time < 3380.0 {
        0x3f2c_cd5a
    } else {
        0x3f40_0180
    })
}

/// Lane gains: (low band, high band) for each of the five lanes.
pub fn lane_gains(decay: usize, color: usize) -> [(f32, f32); 5] {
    let time = decay_ms(decay);
    let colored = color_factor(color) * time;
    std::array::from_fn(|lane| {
        let period = f(PLATE_COMB_PERIODS[lane]);
        (0.01f32.powf(period / colored), 0.01f32.powf(period / time))
    })
}

/// LoDMP high-pass corner in Hz.
pub fn low_damp_hz(value: f32) -> f32 {
    (360.0 - 340.0 * (value.clamp(0.0, 100.0) / 100.0).powf(f(0x3fcf_5c29))).round()
}

/// HiDMP low-pass corner in Hz.
pub fn high_damp_hz(value: f32) -> f32 {
    let x = value.clamp(0.0, 100.0) / 100.0;
    (1600.0 + 20400.0 * x * x).round()
}

// Glides: c39 duck, c41, c43, ten lane gains (low, high per lane), c69, c70, c2.
const DUCK: usize = 0;
const DIFFUSION: usize = 1;
const C43: usize = 2;
const LANES: usize = 3;
const WET: usize = 13;
const DRY: usize = 14;
const LEVEL: usize = 15;

const READS: usize = 24 + 12;
const WRITES: usize = 14 + 2;

pub struct Plate {
    rate: f32,
    ring: Ring,
    read: [usize; READS],
    write: [usize; WRITES],
    highpass: FirstOrder,
    color_eq: Biquad,
    diffuser_damp: [FirstOrder; 4],
    lane_damp: [FirstOrder; 5],
    lane_dc: [FirstOrder; 5],
    lane_low: [f32; 5],
    crossover: f32,
    tail: [[Biquad; 2]; 2],
    glides: Glides<16>,
    decay: Option<(usize, usize)>,
    predelay: Option<usize>,
    pending_predelay: Option<usize>,
    damping: [f32; 2],
    mix: f32,
    level: f32,
}

/// Pre-delay in samples for a PreD value, `(PreD + 1) * 44.1` truncated.
fn predelay_samples(value: usize, rate: f32) -> usize {
    (((value + 1) as f32) * rate * 0.001) as usize
}

impl Plate {
    pub fn new(rate: f32) -> Self {
        // Main ring: writes c71..c84 and reads c6..c29. The reflection region
        // (second half of the original ring) becomes two more segments placed
        // past the main ring's offsets: pre-delayed at 50000, reflected at
        // 50000 + c91.
        const ER: u32 = 50_000;
        let mut writes = [0u32; WRITES];
        writes[..14].copy_from_slice(&PLATE_COE[71..85]);
        writes[14] = ER;
        writes[15] = ER + PLATE_COE[91];
        let mut reads = [0u32; READS];
        reads[..24].copy_from_slice(&PLATE_COE[6..30]);
        for i in 0..4 {
            reads[24 + i] = ER + PLATE_COE[113 + i];
        }
        for i in 0..8 {
            reads[28 + i] = ER + PLATE_COE[91] + PLATE_COE[117 + i];
        }
        let mut min_lengths = [0; WRITES];
        min_lengths[0] = predelay_samples(199, rate);
        let (write, read, total) = place(&writes, &reads, min_lengths, rate);
        let tail_section = |at: usize| rom_biquad(std::array::from_fn(|i| c(at + i)), rate);
        let (low, high) = (tail_section(59), tail_section(64));
        Self {
            rate,
            ring: Ring::new(total),
            read,
            write,
            highpass: FirstOrder::resampled(c(30), c(31), c(32), zoom::ORIGINAL_RATE, rate),
            color_eq: Biquad::new(input_eq(rate)),
            diffuser_damp: [FirstOrder::default(); 4],
            lane_damp: [FirstOrder::default(); 5],
            lane_dc: [FirstOrder::default(); 5],
            lane_low: [0.0; 5],
            crossover: super::common::one_pole_at(c(45), 1.0 - c(45), rate).0,
            tail: [[Biquad::new(low), Biquad::new(high)]; 2],
            glides: Glides::new([0.0; 16], rate),
            decay: None,
            predelay: None,
            pending_predelay: None,
            damping: [-1.0; 2],
            mix: -1.0,
            level: -1.0,
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
}

/// The ROM input EQ biquad c33..c37.
fn input_eq(rate: f32) -> crate::dsp::Coefficients {
    rom_biquad(std::array::from_fn(|i| c(33 + i)), rate)
}

impl Effect for Plate {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let jump = self.decay.is_none();
        let predelay = stepped(values[0], 199);
        if self.predelay != Some(predelay) {
            self.predelay = Some(predelay);
            if jump {
                self.apply_predelay(predelay);
                self.glides.jump(DUCK, 1.0);
            } else {
                self.pending_predelay = Some(predelay);
                self.glides.to(DUCK, 0.0, STEP_FAST);
            }
        }
        let decay = (stepped(values[1], 100), stepped(values[3], 100));
        if self.decay != Some(decay) {
            self.decay = Some(decay);
            let mut targets = [(0usize, 0.0f32); 12];
            targets[0] = (DIFFUSION, diffusion(decay_ms(decay.0)));
            targets[1] = (C43, -0.5);
            for (lane, (low, high)) in lane_gains(decay.0, decay.1).into_iter().enumerate() {
                targets[2 + 2 * lane] = (LANES + 2 * lane, low);
                targets[3 + 2 * lane] = (LANES + 2 * lane + 1, high);
            }
            for (index, value) in targets {
                if jump {
                    self.glides.jump(index, value);
                } else {
                    self.glides.to(index, value, STEP_FAST);
                }
            }
        }
        let damping = [values[4].round(), values[5].round()];
        if damping[0] != self.damping[0] {
            self.damping[0] = damping[0];
            for section in &mut self.lane_dc {
                section.design(true, low_damp_hz(damping[0]), self.rate);
            }
        }
        if damping[1] != self.damping[1] {
            self.damping[1] = damping[1];
            let hz = high_damp_hz(damping[1]).min(self.rate * 0.49);
            for section in self.lane_damp.iter_mut().chain(&mut self.diffuser_damp) {
                section.design(false, hz, self.rate);
            }
        }
        if values[2] != self.mix {
            self.mix = values[2];
            let mix = values[2].clamp(0.0, 100.0);
            let wet = zoom::curve(1.0, 0.0, 100, mix, 5);
            let dry = if mix.round() < 91.0 {
                1.0
            } else {
                zoom::curve(0.0, 1.0, 10, mix - 91.0, 5)
            };
            if jump {
                self.glides.jump(WET, wet);
                self.glides.jump(DRY, dry);
            } else {
                self.glides.over(WET, wet, SETTER_SAMPLES);
                self.glides.over(DRY, dry, SETTER_SAMPLES);
            }
        }
        if values[6] != self.level {
            self.level = values[6];
            let target = values[6].clamp(0.0, 150.0) / 100.0;
            if jump {
                self.glides.jump(LEVEL, target);
            } else {
                self.glides.to(LEVEL, target, zoom::DEFAULT_STEP);
            }
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
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
        let read = &self.read;
        // Taps read where used (all before the frame's writes), kept in locals.
        let t = |j: usize| ring.read(read[j]);
        let (c40, c41, c42, c43) = (c(40), g[DIFFUSION], c(42), g[C43]);

        let high_passed = self.highpass.process(0.5 * (left + right));
        let colored_input = self.color_eq.process(high_passed);

        // Input diffusers.
        let predelayed = g[DUCK] * t(0);
        let mut signal = predelayed;
        let mut diffusers = [0.0f32; 4];
        for (stage, written) in diffusers.iter_mut().enumerate() {
            let tap = t(1 + stage);
            let next = c43 * tap + c42 * signal;
            *written = self.diffuser_damp[stage].process(c40 * signal + c41 * tap);
            signal = next;
        }
        let excitation = c(44) * signal;

        // Feedback lanes.
        let keep = 1.0 - self.crossover;
        let mut lanes = [0.0f32; 5];
        for (lane, written) in lanes.iter_mut().enumerate() {
            let tap = t(5 + lane);
            let memory = keep * self.lane_low[lane];
            let low = self.crossover * tap + memory;
            let high = keep * tap - memory;
            self.lane_low[lane] = super::common::flush(low);
            let colored = excitation + g[LANES + 2 * lane + 1] * high + g[LANES + 2 * lane] * low;
            *written = self.lane_dc[lane].process(self.lane_damp[lane].process(colored));
        }

        // Output diffusers.
        let left_sum = c(56) * (t(10) + t(12) + t(14) + t(16) + t(18));
        let right_sum = c(57) * (t(11) + t(13) + t(15) + t(17) + t(19));
        let pair = |x: f32, d1: f32, d2: f32| {
            let next = c42 * x + c43 * d1;
            let first = c41 * d1 + c40 * x;
            let feed = c(58) * (c43 * d2 + c42 * next);
            let second = c41 * d2 + c40 * next;
            (first, second, feed)
        };
        let (left_first, left_second, left_feed) = pair(left_sum, t(20), t(21));
        let (right_first, right_second, right_feed) = pair(right_sum, t(22), t(23));
        let [[left_low, left_shelf], [right_low, right_shelf]] = &mut self.tail;
        let tail_left = left_shelf.process(left_low.process(left_feed));
        let tail_right = right_shelf.process(right_low.process(right_feed));

        // Early reflections.
        let mut first = predelayed;
        for i in 0..4 {
            first += c(92 + i) * t(24 + i);
        }
        let reflected = g[DUCK] * first;
        let mut early_left = 0.0;
        let mut early_right = 0.0;
        for i in 0..8 {
            early_left += c(96 + i) * t(28 + i);
            early_right += c(104 + i) * t(28 + i);
        }

        let (wet, dry, level) = (g[WET], g[DRY], g[LEVEL]);
        let w = &self.write;
        let mut ring = self.ring.writer();
        ring.write(w[0], c(38) * colored_input);
        // Diffuser and lane values leave flushing sections.
        for (i, value) in diffusers.into_iter().enumerate() {
            ring.write_flushed(w[1 + i], value);
        }
        for (i, value) in lanes.into_iter().enumerate() {
            ring.write_flushed(w[5 + i], value);
        }
        ring.write(w[10], left_first);
        ring.write(w[11], left_second);
        ring.write(w[12], right_first);
        ring.write(w[13], right_second);
        ring.write(w[14], predelayed);
        ring.write(w[15], reflected);
        self.ring.advance();

        let wet_left = tail_left + c(112) * early_left;
        let wet_right = tail_right + c(112) * early_right;
        [
            level * (dry * left + wet * wet_left),
            level * (dry * right + wet * wet_right),
        ]
    }

    fn clear(&mut self) {
        self.ring.clear();
        self.highpass.clear();
        self.color_eq.clear();
        for section in self
            .diffuser_damp
            .iter_mut()
            .chain(&mut self.lane_damp)
            .chain(&mut self.lane_dc)
        {
            section.clear();
        }
        self.lane_low = [0.0; 5];
        for side in &mut self.tail {
            for section in side {
                section.clear();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::reverb::common::scale;
    use crate::fx::reverb::hall_family::tests::{first_arrival, tail_energy, values};

    #[test]
    fn decay_and_color_follow_the_recovered_editor() {
        assert!((decay_ms(0) - 347.66).abs() < 0.01);
        assert!((decay_ms(90) - 7743.75).abs() < 0.01);
        assert!((decay_ms(100) - 61250.0).abs() < 0.01);
        assert!((color_factor(0) - 0.01).abs() < 1e-6);
        let gains = lane_gains(52, 100);
        // Color 100: both bands decay by 40 dB over T.
        assert!((gains[0].0 - gains[0].1).abs() < 1e-6);
        let time = decay_ms(52);
        assert!((gains[0].1 - 0.01f32.powf(96.4 / time)).abs() < 1e-5);
        assert_eq!(diffusion(200.0), f(0x3ec0_0180));
        assert_eq!(diffusion(5000.0), f(0x3f40_0180));
    }

    #[test]
    fn lanes_keep_the_rom_periods() {
        let plate = Plate::new(44_100.0);
        let periods = [4251, 3817, 3644, 3066, 2692];
        for (lane, period) in periods.into_iter().enumerate() {
            assert_eq!(plate.read[5 + lane] - plate.write[5 + lane], period);
            let ms = f(PLATE_COMB_PERIODS[lane]);
            assert!((period as f32 / 44.1 - ms).abs() < 0.05, "{lane}");
        }
    }

    #[test]
    fn damping_corners() {
        assert_eq!(low_damp_hz(0.0), 360.0);
        assert_eq!(low_damp_hz(100.0), 20.0);
        assert_eq!(high_damp_hz(0.0), 1600.0);
        assert_eq!(high_damp_hz(100.0), 22000.0);
        assert_eq!(high_damp_hz(95.0), 20011.0);
    }

    #[test]
    fn mix_fades_the_dry_above_90() {
        let mut plate = Plate::new(48_000.0);
        plate.set(&values(&[8.0, 52.0, 100.0, 58.0, 97.0, 95.0, 100.0]), 120.0);
        assert!(plate.glides.value[DRY].abs() < 1e-6);
        assert!((plate.glides.value[WET] - 1.0143).abs() < 1e-3);
        let mut plate = Plate::new(48_000.0);
        plate.set(&values(&[8.0, 52.0, 44.0, 58.0, 97.0, 95.0, 100.0]), 120.0);
        assert_eq!(plate.glides.value[DRY], 1.0);
    }

    #[test]
    fn predelay_and_tail() {
        for rate in [44_100.0, 96_000.0] {
            let mut plate = Plate::new(rate);
            plate.set(&values(&[49.0, 80.0, 50.0, 58.0, 97.0, 95.0, 100.0]), 120.0);
            assert_eq!(plate.predelay_tap(), (50.0 * rate / 1000.0) as usize);
            let arrival = first_arrival(&mut plate, rate, 1e-6);
            // The diffusers' direct path excites lane 4, whose output tap t19
            // sits 193 samples into the lane: the first sound after pre-delay.
            let expected = plate.predelay_tap() + scale(193.0, rate);
            assert!(
                arrival.abs_diff(expected) <= 2,
                "{rate}: {arrival} vs {expected}"
            );
            assert!(tail_energy(&mut plate, rate, 1.0, 2.0) > 1e-4);
        }
    }
}
