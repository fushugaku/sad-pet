//! fCycle (FCYCLE.ZDL, `Fx_FLT_fCycle`): an LFO-swept resonant low-pass.
//!
//! Callback topology, followed as recovered: a 2^32 phase LFO is shaped into
//! the selected wave (`c27`: squared parabolic sine of the folded phase,
//! `c25`: triangle `|phase|`, `c26`: saw `0.5 + phase / 2`), scaled by Depth
//! (`c10 + shaped * c9`) and smoothed by a slow one-pole (`c11`/`c12`,
//! 2^-11 per 44.1 kHz sample) into the frequency word of a clipped Chamberlin
//! filter. The mono mean (times the switch gain `c3`) feeds one stage whose
//! low-pass output (the ROM weights `c16..c18`; the second stage and the two
//! post filters stay disabled/identity in every editor path) times Level goes
//! to both channels.
//!
//! Editors decoded from the disassembly: Rate (square integer curve
//! 48695..974939 per 2^32, 0.5..10 Hz; synced values use the Main LFO
//! divisions; SawDn negates the increment), Wave (ROM `0x800005a8`, applied
//! between a 16 ms fade-out of `c3` and the scheduled fade-in), Depth (square
//! integer curve: `c9` 152949940..757972470, `c10` = 152949940 per 2^31, i.e.
//! a 500 Hz floor sweeping up to 1..3 kHz), Reso (damping `c15` linear
//! 1..1/7, float setter step) and Level (`value / 100`).
use super::common::{self, Changes, ClippedSvf, Q31, SvfTuning, SwitchGate};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::flush,
    zoom::{self, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_f_cycle",
    name: "fCycle",
    category: Category::Filter,
    zdl: "FCYCLE.ZDL",
    description: "Resonant filter swept by a cyclic LFO",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Rate",
            max: 77,
            default: 5,
            display: Display::sync_rate(49, 1.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Wave",
            max: 3,
            default: 3,
            display: Display::labels(&["Sine", "Tri", "SawUp", "SawDn"]),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Reso",
            max: 10,
            default: 8,
            display: Display::INT,
        },
    ],
};

const FLOOR: f64 = 152_949_940.0;

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(FCycle::new(sample_rate))
}

/// Rate 0..49 → LFO Hz.
fn normal_rate_hz(value: f32) -> f32 {
    common::q32_hz(common::int_curve(974_939.0, 48_695.0, 50, value, 1))
}

/// The LFO shapes of the callback, `magnitude = |phase|` in 0..1.
#[inline]
fn shape(wave: usize, phase: i32) -> f32 {
    let magnitude = (phase as f32).abs() * (1.0 / 2_147_483_648.0);
    match wave {
        0 => {
            let folded = magnitude - 0.5;
            let curved = folded * (1.0 - folded.abs());
            let window = (2.0 * curved + 0.5).clamp(0.0, 1.0);
            window * window
        }
        1 => magnitude,
        _ => 0.5 + phase as f32 * (1.0 / 4_294_967_296.0),
    }
}

struct FCycle {
    sample_rate: f32,
    changes: Changes,
    hz: f32,
    phase: u32,
    increment: u32,
    wave: usize,
    pending: usize,
    gate: SwitchGate,
    span: f32,
    floor: f32,
    raw: f32,
    raw_gain: f32,
    raw_pole: f32,
    tuning: SvfTuning,
    filter: ClippedSvf,
    damping: Ramp,
    level: Ramp,
}

impl FCycle {
    fn new(sample_rate: f32) -> Self {
        // c11 = 0x00100000, c12 = 0x7ff00000 (Q31).
        let (raw_gain, raw_pole) = common::one_pole(1.0 / 2048.0, 1.0 - 1.0 / 2048.0, sample_rate);
        Self {
            sample_rate,
            changes: Changes::new(),
            hz: 0.0,
            phase: 0,
            increment: 0,
            wave: 3,
            pending: 3,
            gate: SwitchGate::new(sample_rate),
            span: 0.0,
            floor: (FLOOR / Q31) as f32,
            raw: (FLOOR / Q31) as f32,
            raw_gain,
            raw_pole,
            tuning: SvfTuning::new(sample_rate),
            filter: ClippedSvf::default(),
            damping: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
            level: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
        }
    }

    fn update_increment(&mut self) {
        let increment = common::increment(self.hz, self.sample_rate);
        self.increment = if self.wave == 3 {
            increment.wrapping_neg()
        } else {
            increment
        };
    }
}

impl Effect for FCycle {
    fn set(&mut self, values: &Values, bpm: f32) {
        let (changed, tempo, first) = self.changes.update(values, bpm);
        if changed & 2 != 0 {
            let wave = common::index(values[1], 3);
            if first {
                self.wave = wave;
                self.pending = wave;
            } else if wave != self.pending {
                self.pending = wave;
                self.gate.request();
            }
        }
        if changed & 1 != 0 || (tempo && zoom::is_synced(values[0], 49)) {
            self.hz = common::sync_hz(values[0], 49, bpm, normal_rate_hz);
            self.update_increment();
        }
        if changed & 4 != 0 {
            self.level.set(values[2].max(0.0) / 100.0);
            if first {
                self.level.jump(self.level.target);
            }
        }
        if changed & 8 != 0 {
            let word = common::int_curve(757_972_470.0, FLOOR, 101, values[3], 1);
            self.span = (word / Q31) as f32;
        }
        if changed & 16 != 0 {
            let damping = zoom::curve(
                f32::from_bits(0x3e12_4925),
                1.0,
                11,
                values[4].clamp(0.0, 10.0),
                0,
            );
            self.damping.set(damping);
            if first {
                self.damping.jump(damping);
            }
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        if self.gate.tick() {
            self.wave = self.pending;
            self.update_increment();
        }
        self.phase = self.phase.wrapping_add(self.increment);
        let driven = shape(self.wave, self.phase as i32) * self.span + self.floor;
        self.raw = flush(driven * self.raw_gain + self.raw * self.raw_pole);
        let f = self.tuning.map(self.raw);
        let input = 0.5 * (left + right) * self.gate.gain;
        let (_, _, low) = self.filter.process(input, f, self.damping.tick());
        let out = low * self.level.tick();
        [out, out]
    }

    fn clear(&mut self) {
        self.phase = 0;
        self.raw = self.floor;
        self.filter.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    #[test]
    fn rate_follows_the_editor_curve() {
        assert!((normal_rate_hz(0.0) - 0.5).abs() < 1e-3);
        assert!((normal_rate_hz(49.0) - 10.01).abs() < 1e-2);
    }

    #[test]
    fn waves_cover_zero_to_one() {
        for wave in 0..4 {
            let (mut low, mut high) = (f32::MAX, f32::MIN);
            for i in 0..1000u32 {
                let phase = (i as u64 * (1u64 << 32) / 1000) as u32;
                let value = shape(wave, phase as i32);
                low = low.min(value);
                high = high.max(value);
            }
            assert!(
                low > -1e-3 && low < 0.01 && high > 0.99 && high < 1.0001,
                "{wave}"
            );
        }
    }

    #[test]
    fn sweep_spans_the_original_cutoffs() {
        let mut effect = FCycle::new(48_000.0);
        let mut values = Model::from_key("mooz_f_cycle").unwrap().defaults();
        values[1] = 1.0; // triangle
        values[0] = 0.0; // 0.5 Hz
        effect.set(&values, 120.0);
        let hz = |f: f32| (f as f64 * 0.5).asin() * 48_000.0 / std::f64::consts::PI;
        let (mut low, mut high) = (f64::MAX, f64::MIN);
        for _ in 0..48_000 * 4 {
            effect.process([0.0; 2]);
            let f = effect.tuning.map(effect.raw);
            low = low.min(hz(f));
            high = high.max(hz(f));
        }
        assert!((low - 503.0).abs() < 15.0, "{low}");
        assert!(high > 2800.0 && high < 3010.0, "{high}");
    }

    #[test]
    fn lfo_period_matches_rate() {
        let mut effect = FCycle::new(44_100.0);
        let mut values = Model::from_key("mooz_f_cycle").unwrap().defaults();
        values[1] = 2.0; // saw up
        values[0] = 57.0; // quarter note at 120 BPM: 2 Hz
        effect.set(&values, 120.0);
        let mut wraps = 0;
        let mut last = effect.phase;
        for _ in 0..44_200 {
            effect.process([0.0; 2]);
            if effect.phase < last {
                wraps += 1;
            }
            last = effect.phase;
        }
        assert_eq!(wraps, 2);
    }
}
