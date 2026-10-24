//! RndmFLTR (RNDMFLTR.ZDL, `Fx_FLT_RandomFilter`): a filter jumping to
//! random cutoffs.
//!
//! Callback topology, followed as recovered: each signed rollover of a 2^32
//! phase counter steps a 32-bit Galois LFSR (eight `_NseedTbl` polynomials,
//! switching polynomial every ninth update) and holds `c26 + random * c25` as
//! the new frequency word; a fast one-pole (0.15/0.85 per 44.1 kHz sample)
//! softens the jump. The mono mean runs through one (2Pole) or two (4Pole)
//! clipped Chamberlin stages with input gain `c31`, damping `c33` and an HPF,
//! BPF or LPF output; each channel is `(dry * c6 + filtered * c5)` times Level.
//!
//! Editors decoded from the disassembly: Speed (linear integer curve
//! 48695..974939 per 2^32, 0.5..10 Hz; synced values use the Main LFO
//! divisions), Range (square integer curves for `c26` 30596198..91782385 and
//! `c25` 213923467..1467082879 per 2^31, about 100..800 Hz up to
//! 300..5200 Hz), Reso (`Q` = square curve 1.5..20, `c31 = 10^(Q a / 20)`,
//! `c33 = b / Q` with `(a, b)` = (-0.3, 1) for 2Pole and (-0.4, 2) for 4Pole),
//! Type (ROM mix table), Chara (second stage), Bal (two-slope curves for
//! `c5`/`c6`) and Level. Type and Chara changes mute the output for 16 ms,
//! switch, and fade back as the original does. Deviation: after a reset the
//! filter starts in the middle of the range instead of closed (held word 0).
use super::common::{self, Changes, ClippedSvf, Q31, SwitchGate, clip};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Smoother, db_to_gain, flush},
    zoom::{self, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_rndm_fltr",
    name: "RndmFLTR",
    category: Category::Filter,
    zdl: "RNDMFLTR.ZDL",
    description: "Filter that jumps to random cutoffs",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Speed",
            max: 77,
            default: 34,
            display: Display::sync_rate(49, 1.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Range",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Reso",
            max: 10,
            default: 6,
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
            name: "Bal",
            max: 100,
            default: 90,
            display: Display::INT,
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

/// `_NseedTbl` (ROM `0x800006b0`).
pub const POLYNOMIALS: [u32; 8] = [
    0xa2289441, 0x90018220, 0x81245423, 0x60604004, 0x9601000a, 0x42180482, 0xd0003301, 0x84940294,
];
const SEED: u32 = 0x9001_8221;
/// ROM `0x80000688`: Type → (high, band, low) weights.
const TYPES: [[f32; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
/// ROM `0x800006e8`: Chara → (gain exponent per Q in dB, damping numerator).
const CHARA: [(f32, f32); 2] = [(-0.3, 1.0), (-0.4, 2.0)];

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(RndmFltr::new(sample_rate))
}

/// Speed 0..49 → updates per second.
fn normal_speed_hz(value: f32) -> f32 {
    common::q32_hz(common::int_curve(974_939.0, 48_695.0, 50, value, 0))
}

/// Range → (`c26` offset, `c25` span) as SVF frequency words at 44.1 kHz.
fn range_words(value: f32) -> (f32, f32) {
    let offset = common::int_curve(91_782_385.0, 30_596_198.0, 101, value, 1);
    let span = common::int_curve(1_467_082_879.0, 213_923_467.0, 101, value, 1);
    ((offset / Q31) as f32, (span / Q31) as f32)
}

/// Reso and Chara → (`c31` input gain, `c33` damping).
fn resonance(value: f32, chara: usize) -> (f32, f32) {
    let q = zoom::curve(20.0, 1.5, 11, value.clamp(0.0, 10.0), 1);
    let (a, b) = CHARA[chara];
    (db_to_gain(q * a), b / q)
}

struct RndmFltr {
    sample_rate: f32,
    changes: Changes,
    phase: u32,
    increment: u32,
    wraps: u32,
    polynomial: usize,
    seed: u32,
    offset: f32,
    span: f32,
    held: f32,
    smoothed: f32,
    smooth_gain: f32,
    smooth_pole: f32,
    reso: f32,
    input_gain: Smoother,
    damping: Smoother,
    mix: [f32; 3],
    four_pole: bool,
    pending: (usize, usize),
    gate: SwitchGate,
    filters: [ClippedSvf; 2],
    wet: Ramp,
    dry: Ramp,
    level: Ramp,
}

impl RndmFltr {
    fn new(sample_rate: f32) -> Self {
        let (smooth_gain, smooth_pole) = common::one_pole(
            f32::from_bits(0x3e19_999a),
            f32::from_bits(0x3f59_999a),
            sample_rate,
        );
        Self {
            sample_rate,
            changes: Changes::new(),
            phase: 0,
            increment: 0,
            wraps: 0,
            polynomial: 0,
            seed: SEED,
            offset: 0.0,
            span: 0.0,
            held: 0.0,
            smoothed: 0.0,
            smooth_gain,
            smooth_pole,
            reso: 6.0,
            input_gain: Smoother::new(1.0, 0.005, sample_rate),
            damping: Smoother::new(1.0, 0.005, sample_rate),
            mix: TYPES[2],
            four_pole: true,
            pending: (2, 1),
            gate: SwitchGate::new(sample_rate),
            filters: [ClippedSvf::default(); 2],
            wet: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
            dry: Ramp::new(0.0, zoom::DEFAULT_STEP, sample_rate),
            level: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
        }
    }

    fn middle(&self) -> f32 {
        common::svf_f(self.offset + 0.5 * self.span, self.sample_rate)
    }

    /// One LFSR step and a new held frequency.
    fn next_target(&mut self) {
        self.wraps += 1;
        if self.wraps > 8 {
            self.polynomial = (self.polynomial + 1) & 7;
            self.wraps = 0;
        }
        let mask = 0u32.wrapping_sub(self.seed & 1);
        self.seed = (self.seed >> 1) ^ (mask & POLYNOMIALS[self.polynomial]);
        let random = (f64::from(self.seed as i32) / Q31) as f32 * 0.5 + 0.5;
        self.held = common::svf_f(self.offset + random * self.span, self.sample_rate);
    }

    fn apply_switch(&mut self) {
        let (kind, chara) = self.pending;
        self.mix = TYPES[kind];
        self.four_pole = chara == 1;
        let (gain, damping) = resonance(self.reso, chara);
        self.input_gain.jump(gain);
        self.damping.jump(damping);
    }
}

impl Effect for RndmFltr {
    fn set(&mut self, values: &Values, bpm: f32) {
        let (changed, tempo, first) = self.changes.update(values, bpm);
        if changed & 1 != 0 || (tempo && zoom::is_synced(values[0], 49)) {
            let hz = common::sync_hz(values[0], 49, bpm, normal_speed_hz);
            self.increment = common::increment(hz, self.sample_rate);
        }
        if changed & 2 != 0 {
            (self.offset, self.span) = range_words(values[1].clamp(0.0, 100.0));
            if first {
                self.held = self.middle();
                self.smoothed = self.held;
            }
        }
        let chara = common::index(values[4], 1);
        if changed & 4 != 0 {
            self.reso = values[2];
            let (gain, damping) = resonance(self.reso, self.pending.1);
            self.input_gain.set(gain);
            self.damping.set(damping);
        }
        if changed & 24 != 0 {
            let pending = (common::index(values[3], 2), chara);
            if first {
                self.pending = pending;
                self.apply_switch();
            } else if pending != self.pending {
                self.pending = pending;
                self.gate.request();
            }
        }
        if changed & 32 != 0 {
            let bal = values[5].clamp(0.0, 100.0);
            self.wet.set(zoom::curve(1.0, 0.0, 101, bal, 2));
            self.dry.set(zoom::curve(1.0, 0.0, 101, 100.0 - bal, 2));
            if first {
                self.wet.jump(self.wet.target);
                self.dry.jump(self.dry.target);
            }
        }
        if changed & 64 != 0 {
            self.level.set(values[6].max(0.0) / 100.0);
            if first {
                self.level.jump(self.level.target);
            }
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mean = 0.5 * (left + right);
        let old = self.phase;
        self.phase = old.wrapping_add(self.increment);
        if (old as i32) > (self.phase as i32) {
            self.next_target();
        }
        self.smoothed = flush(self.held * self.smooth_gain + self.smoothed * self.smooth_pole);
        let f = self.smoothed.abs();
        if self.gate.tick() {
            self.apply_switch();
        }
        let gain = self.input_gain.tick();
        let damping = self.damping.tick();
        let [high_weight, band_weight, low_weight] = self.mix;
        let (high, band, low) = self.filters[0].process(gain * mean, f, damping);
        let mut filtered = clip(low * low_weight + band * band_weight + high * high_weight);
        if self.four_pole {
            let (high, band, low) = self.filters[1].process(gain * filtered, f, damping);
            filtered = clip(high * high_weight + band * band_weight + low * low_weight);
        }
        let scale = self.gate.gain * self.level.tick();
        let dry = self.dry.tick();
        let wet = filtered * self.wet.tick();
        [(left * dry + wet) * scale, (right * dry + wet) * scale]
    }

    fn clear(&mut self) {
        self.phase = 0;
        self.wraps = 0;
        self.polynomial = 0;
        self.seed = SEED;
        self.held = self.middle();
        self.smoothed = self.held;
        for filter in &mut self.filters {
            filter.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    #[test]
    fn speed_and_range_follow_the_editors() {
        assert!((normal_speed_hz(0.0) - 0.5).abs() < 1e-3);
        assert!((normal_speed_hz(49.0) - 10.01).abs() < 1e-2);
        let hz = |f: f32| (f as f64 * 0.5).asin() * 44_100.0 / std::f64::consts::PI;
        let (offset, span) = range_words(0.0);
        assert!((hz(offset) - 100.0).abs() < 2.0, "{}", hz(offset));
        assert!(
            (hz(offset + span) - 800.0).abs() < 5.0,
            "{}",
            hz(offset + span)
        );
        let (offset, span) = range_words(100.0);
        assert!((hz(offset) - 300.0).abs() < 3.0);
        assert!(
            (hz(offset + span) - 5170.0).abs() < 60.0,
            "{}",
            hz(offset + span)
        );
    }

    #[test]
    fn lfsr_matches_the_recovered_generator() {
        // Reference: the recovered callback's integer generator.
        let (mut seed, mut polynomial, mut wraps) = (SEED, 0usize, 0u32);
        let mut effect = RndmFltr::new(44_100.0);
        effect.set(
            &Model::from_key("mooz_rndm_fltr").unwrap().defaults(),
            120.0,
        );
        for _ in 0..40 {
            wraps += 1;
            if 8 < wraps as i32 {
                polynomial = (polynomial + 1) & 7;
                wraps = 0;
            }
            seed = (seed >> 1) ^ (0u32.wrapping_sub(seed & 1) & POLYNOMIALS[polynomial]);
            effect.next_target();
            assert_eq!(effect.seed, seed);
            assert_eq!(effect.polynomial, polynomial);
        }
    }

    #[test]
    fn targets_stay_inside_the_range_and_update_at_the_speed() {
        let mut effect = RndmFltr::new(48_000.0);
        let mut values = Model::from_key("mooz_rndm_fltr").unwrap().defaults();
        values[0] = 57.0; // quarter notes: 2 Hz at 120 BPM
        effect.set(&values, 120.0);
        let (low, high) = (
            common::svf_f(effect.offset, 48_000.0),
            common::svf_f(effect.offset + effect.span, 48_000.0),
        );
        let mut updates = 0;
        let mut last = effect.held;
        for _ in 0..48_000 * 3 {
            effect.process([0.0; 2]);
            if effect.held != last {
                updates += 1;
                last = effect.held;
                assert!(effect.held >= low - 1e-6 && effect.held <= high + 1e-6);
            }
        }
        assert!((5..=6).contains(&updates), "{updates}");
    }

    #[test]
    fn type_change_fades_through_silence() {
        let mut effect = RndmFltr::new(44_100.0);
        let mut values = Model::from_key("mooz_rndm_fltr").unwrap().defaults();
        effect.set(&values, 120.0);
        for i in 0..4410 {
            effect.process([(i as f32 * 0.1).sin(); 2]);
        }
        values[3] = 0.0;
        effect.set(&values, 120.0);
        let mut minimum = f32::MAX;
        for i in 0..2000 {
            let [l, _] = effect.process([(i as f32 * 0.1).sin(); 2]);
            minimum = minimum.min(l.abs());
        }
        assert!(minimum < 1e-3);
        assert_eq!(effect.mix, TYPES[0]);
    }
}
