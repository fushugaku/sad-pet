//! HD Reverb (HD_REV.ZDL): a half-rate network of diffusers and six combs.
//!
//! Callback (zoom-ms70-cdr `hd_reverb.rs`): at the full rate the stereo
//! half-sum runs through the ROM high-pass c33..c35 (with +6 dB) and the Tone
//! low-pass c36..c40. Every other frame the network reads 27 taps and writes
//! 15 words: two pre-delay taps (c6, c6 + 1698) times the duck c42 sum into
//! four allpass diffusers (`node = c43 x + c44 d`, `y = c45 node + c46 d`)
//! feeding six parallel combs (gains c50..c55) without damping; the second
//! pre-delay tap and the last diffuser also pass two first-order sections
//! (c63..c68, a 500 Hz low-passed "body", times c71). Twelve comb taps times
//! c69/c70 go through two more allpasses per side, times c73, plus the body.
//! The network output is interpolated back to the full rate and filtered by
//! the HPF section c74..c78 and the ROM shelf c79..c83; the output is
//! `Level * (dry + c3 * c84 * wet)`.
//!
//! Editors (all decoded from `HD_REV.ZDL.dis`): Decay ramps c44 = g, c45 = -g
//! and c50..c55 from `HD_Rev_DecayTbl[Decay]` (step 0x3af7c5ec); Tone copies
//! `tone1_tbl[Tone]` into c36..c40; PreD ducks c42, sets c6 = `(PreD + 1) *
//! 441 / 20` half-rate samples and c7 = c6 + 1698, and restores c42; HPF
//! designs Main's first-order high-pass (designer type 1) at `30 + 470 (HPF /
//! 10)^1.41` Hz into c74..c78; Mix sets c84 = `(Mix / 100)^1.41` over 16 ms;
//! Level sets c2 = Level / 100.
//!
//! Port notes: the network runs at half the host rate with its delays scaled
//! from 22.05 kHz; ROM sections are re-derived for the host (or half) rate.
use super::{
    common::{FirstOrder, Glides, Ring, SETTER_SAMPLES, STEP_FAST, f, place, scale, stepped},
    hall_family::mix_curve,
    rom::{HD_COE, HD_DECAY, HD_TONE},
};
use crate::{Effect, Values, catalog::*, zoom};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_hd_reverb",
    name: "HD Reverb",
    category: Category::Reverb,
    zdl: "HD_REV.ZDL",
    description: "Dense high-definition reverb with pre-delay and HPF",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Decay",
            max: 100,
            default: 10,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 7,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 46,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "PreD",
            max: 199,
            default: 53,
            display: Display::number(1.0, 1.0, 0, " ms"),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "HPF",
            max: 10,
            default: 7,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(HdReverb::new(sample_rate))
}

/// HPF corner in Hz for an HPF value (Main curve 30..500 Hz, shape 5).
pub fn hpf_hz(value: f32) -> f32 {
    zoom::curve(500.0, 30.0, 11, value.clamp(0.0, 10.0), 5)
}

/// Pre-delay in network samples, `(PreD + 1) * 22.05` truncated at 22.05 kHz.
fn predelay_samples(value: usize, rate: f32) -> usize {
    (((value + 1) as f32) * rate * 0.0005) as usize
}

const SECOND_TAP: f32 = 1698.0;

const DUCK: usize = 0;
const ALLPASS: usize = 1;
const COMB: usize = 2;
const MIX: usize = 8;
const LEVEL: usize = 9;

fn c(index: usize) -> f32 {
    f(HD_COE[index])
}

pub struct HdReverb {
    rate: f32,
    ring: Ring,
    read: [usize; 27],
    write: [usize; 15],
    second_tap: usize,
    input: FirstOrder,
    tone: FirstOrder,
    body: [FirstOrder; 2],
    highpass: [FirstOrder; 2],
    shelf: [FirstOrder; 2],
    glides: Glides<10>,
    even: bool,
    wet: [f32; 2],
    previous: [f32; 2],
    decay: Option<usize>,
    tone_index: Option<usize>,
    hpf: f32,
    predelay: Option<usize>,
    pending_predelay: Option<usize>,
    last: [f32; 2],
}

impl HdReverb {
    pub fn new(rate: f32) -> Self {
        let writes: [u32; 15] = std::array::from_fn(|i| HD_COE[86 + i]);
        let reads: [u32; 27] = std::array::from_fn(|i| HD_COE[6 + i]);
        let mut min_lengths = [0; 15];
        let second_tap = scale(SECOND_TAP, rate);
        min_lengths[0] = predelay_samples(199, rate) + second_tap;
        let (write, read, total) = place(&writes, &reads, min_lengths, rate);
        let half = rate * 0.5;
        let network = |b0: usize| {
            FirstOrder::resampled(c(b0), c(b0 + 1), c(b0 + 2), zoom::ORIGINAL_RATE * 0.5, half)
        };
        let full = |b0: usize, a: usize| {
            FirstOrder::resampled(c(b0), c(b0 + 1), c(a), zoom::ORIGINAL_RATE, rate)
        };
        Self {
            rate,
            ring: Ring::new(total),
            read,
            write,
            second_tap,
            input: full(33, 35),
            tone: FirstOrder::default(),
            body: [network(63), network(66)],
            highpass: [FirstOrder::default(); 2],
            shelf: [full(79, 82); 2],
            glides: Glides::new([1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0], rate),
            even: true,
            wet: [0.0; 2],
            previous: [0.0; 2],
            decay: None,
            tone_index: None,
            hpf: -1.0,
            predelay: None,
            pending_predelay: None,
            last: [-1.0; 2],
        }
    }

    fn apply_predelay(&mut self, value: usize) {
        let first = self.write[0] + predelay_samples(value, self.rate);
        self.read[0] = first;
        self.read[1] = first + self.second_tap;
    }

    fn network(&mut self, drive: f32) {
        let ring = &self.ring;
        let read = &self.read;
        // Taps read where used (all before the frame's writes), kept in locals.
        let t = |j: usize| ring.read(read[j]);
        let g = &self.glides.value;
        let (c43, c46, c47) = (c(43), 1.0, c(47));
        let gain = g[ALLPASS];
        let a = g[DUCK] * t(0);
        let b = g[DUCK] * t(1);
        let mut feedback = a + b;
        let mut weighted = c(57) * a + c(58) * b;
        let mut nodes = [0.0f32; 4];
        for (i, node) in nodes.iter_mut().enumerate() {
            let tap = t(2 + i);
            *node = gain * tap + c43 * feedback;
            feedback = -gain * *node + c46 * tap;
            weighted += c(59 + i) * tap;
        }
        let first = self.body[0].process(weighted);
        let body = self.body[1].process(first);
        let seed = c47 * feedback;
        let combs: [f32; 6] = std::array::from_fn(|k| seed + g[COMB + k] * t(6 + k));
        let o = |j: usize| t(12 + j);
        let wet_left = c(69) * (o(0) + o(2) + o(4) + o(6) + o(8) + o(10));
        // The right sum repeats tap 22 (o(10)) as the original LDW schedule does.
        let wet_right = c(70) * (o(1) + o(3) + o(5) + o(7) + o(9) + o(10));
        let allpass_pair = |x: f32, d1: f32, d2: f32| {
            let first = c43 * x + gain * d1;
            let between = -gain * first + c46 * d1;
            let second = c43 * between + gain * d2;
            (first, second, -gain * second + c46 * d2)
        };
        let (left_a, left_b, left_out) = allpass_pair(wet_left, o(11), o(12));
        let (right_a, right_b, right_out) = allpass_pair(wet_right, o(13), o(14));
        let shared = c(71) * body;
        self.wet = [c(73) * left_out + shared, c(73) * right_out + shared];

        let w = &self.write;
        let mut ring = self.ring.writer();
        ring.write(w[0], c(41) * drive);
        for (i, node) in nodes.into_iter().enumerate() {
            ring.write(w[1 + i], node);
        }
        for (k, comb) in combs.into_iter().enumerate() {
            ring.write(w[5 + k], comb);
        }
        ring.write(w[11], left_a);
        ring.write(w[12], left_b);
        ring.write(w[13], right_a);
        ring.write(w[14], right_b);
        self.ring.advance();
    }
}

impl Effect for HdReverb {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let jump = self.decay.is_none();
        let decay = stepped(values[0], 100);
        if self.decay != Some(decay) {
            self.decay = Some(decay);
            let row = HD_DECAY[decay].map(f);
            for (index, value) in (ALLPASS..ALLPASS + 7).zip(row) {
                if jump {
                    self.glides.jump(index, value);
                } else {
                    self.glides.to(index, value, STEP_FAST);
                }
            }
        }
        let tone = stepped(values[1], 10);
        if self.tone_index != Some(tone) {
            self.tone_index = Some(tone);
            let row = HD_TONE[tone].map(f);
            self.tone
                .set_resampled(row[0], row[1], row[3], zoom::ORIGINAL_RATE, self.rate);
        }
        let predelay = stepped(values[3], 199);
        if self.predelay != Some(predelay) {
            self.predelay = Some(predelay);
            if jump {
                self.apply_predelay(predelay);
            } else {
                self.pending_predelay = Some(predelay);
                self.glides.to(DUCK, 0.0, STEP_FAST);
            }
        }
        let hpf = values[4].round();
        if hpf != self.hpf {
            self.hpf = hpf;
            for section in &mut self.highpass {
                section.design(true, hpf_hz(hpf), self.rate);
            }
        }
        for (i, (index, value, target)) in [
            (MIX, values[2], mix_curve(values[2])),
            (LEVEL, values[5], values[5].clamp(0.0, 150.0) / 100.0),
        ]
        .into_iter()
        .enumerate()
        {
            if value != self.last[i] {
                self.last[i] = value;
                if jump {
                    self.glides.jump(index, target);
                } else if index == MIX {
                    self.glides.over(index, target, SETTER_SAMPLES);
                } else {
                    self.glides.to(index, target, zoom::DEFAULT_STEP);
                }
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
        let first = self.input.process(0.5 * (left + right));
        let drive = self.tone.process(first);
        if self.even {
            self.network(drive);
        }
        self.even = !self.even;
        let average = [
            0.5 * (self.previous[0] + self.wet[0]),
            0.5 * (self.previous[1] + self.wet[1]),
        ];
        self.previous = self.wet;
        let out: [f32; 2] =
            std::array::from_fn(|i| self.shelf[i].process(self.highpass[i].process(average[i])));
        let (mix, level) = (self.glides.value[MIX], self.glides.value[LEVEL]);
        [
            level * (left + mix * out[0]),
            level * (right + mix * out[1]),
        ]
    }

    fn clear(&mut self) {
        self.ring.clear();
        self.input.clear();
        self.tone.clear();
        for section in self
            .body
            .iter_mut()
            .chain(&mut self.highpass)
            .chain(&mut self.shelf)
        {
            section.clear();
        }
        self.wet = [0.0; 2];
        self.previous = [0.0; 2];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::reverb::hall_family::tests::{first_arrival, tail_energy, values};

    #[test]
    fn hpf_curve_spans_30_to_500_hz() {
        assert!((hpf_hz(0.0) - 30.0).abs() < 1e-3);
        assert!((hpf_hz(10.0) - 500.0).abs() < 1e-3);
        let mut section = FirstOrder::default();
        section.design(true, hpf_hz(7.0), 48_000.0);
        let corner = hpf_hz(7.0);
        assert!(
            (section.magnitude(48_000.0, corner) - std::f32::consts::FRAC_1_SQRT_2).abs() < 0.01
        );
    }

    #[test]
    fn network_taps_keep_the_rom_delays() {
        let effect = HdReverb::new(44_100.0);
        // Diffuser 1: write c87 = 6615, read c8 = 7148.
        assert_eq!(effect.read[2] - effect.write[1], 7148 - 6615);
        // Comb 0: write c91 = 9921, read c12 = 14132.
        assert_eq!(effect.read[6] - effect.write[5], 14132 - 9921);
    }

    #[test]
    fn predelay_sets_the_first_arrival() {
        for rate in [44_100.0, 96_000.0] {
            let mut effect = HdReverb::new(rate);
            effect.set(&values(&[50.0, 10.0, 100.0, 99.0, 0.0, 100.0]), 120.0);
            // 100 ms of pre-delay at the half rate, then the diffusers' direct
            // path (node then `c46 * tap` of the next stage) adds no delay.
            let arrival = first_arrival(&mut effect, rate, 1e-6);
            let expected = 2 * predelay_samples(99, rate);
            assert!(
                arrival >= expected && arrival < expected + (rate * 0.06) as usize,
                "{rate}: {arrival} vs {expected}"
            );
        }
    }

    #[test]
    fn decay_100_rings_long() {
        let rate = 48_000.0;
        let mut effect = HdReverb::new(rate);
        effect.set(&values(&[100.0, 7.0, 100.0, 0.0, 0.0, 100.0]), 120.0);
        let long = tail_energy(&mut effect, rate, 2.0, 3.0);
        let mut effect = HdReverb::new(rate);
        effect.set(&values(&[0.0, 7.0, 100.0, 0.0, 0.0, 100.0]), 120.0);
        let short = tail_energy(&mut effect, rate, 2.0, 3.0);
        assert!(long > 1e3 * short.max(1e-30), "{long} {short}");
    }
}
