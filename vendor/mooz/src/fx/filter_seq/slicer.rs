//! Slicer (SLICER.ZDL, `Fx_MOD_Slicer`): a rhythmic gate pattern restarted by
//! picking.
//!
//! Callback topology, followed as recovered: a 50 Hz high-pass on the dry
//! input feeds a signed peak hold (`c16`), a fast envelope (`c12`/`c13`) and a
//! slow, doubled threshold follower (`c14`/`c15`); when the envelope rises
//! above both the absolute threshold `c17` and the follower, the pattern
//! restarts (phase and step to zero). A 2^32 phase counter advances the step on
//! its signed rollover and wraps after the pattern length; the step's level
//! (`Pattern_Lvl_TBL`) is smoothed (`c10`/`c11`) and gates the mono mean, which
//! is added to each dry channel as `(mono * gate * c5 + dry * c6)` times Level.
//!
//! Editors decoded from the disassembly: PTTRN (20 ROM patterns of 16 levels
//! and `Pattern_Length_TBL`, applied between a 16 ms fade-out of `c3` and the
//! scheduled fade-in), Speed (linear integer curve 389566..1947830 per 2^32,
//! 4..20 steps/s; synced values double the Main LFO division, so a quarter
//! note steps in eighths), Bal (two-slope curves for `c5`/`c6`), THRSH
//! (`c17 = 10^((-6 - THRSH) / 20)`) and Level (`value / 100`).
use super::common::{self, Changes, SwitchGate};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, flush},
    zoom::{self, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_slicer",
    name: "Slicer",
    category: Category::Dynamics,
    zdl: "SLICER.ZDL",
    description: "Rhythmic gate pattern restarted by picking",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "PTTRN",
            max: 19,
            default: 0,
            display: Display::offset(1.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Speed",
            max: 77,
            default: 24,
            display: Display::sync_rate(49, 1.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Bal",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "THRSH",
            max: 50,
            default: 20,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Level",
            max: 150,
            default: 130,
            display: Display::INT,
        },
    ],
};

/// ROM `Pattern_Lvl_TBL` (0x80000000): 20 patterns of 16 step levels.
const PATTERNS: [[f32; 16]; 20] = [
    [
        1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        1.0, 0.0, 0.5, 0.0, 0.25, 0.0, 0.125, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        1.0, 0.0, 0.8, 0.0, 0.6, 0.0, 0.5, 0.0, 0.4, 0.0, 0.3, 0.0, 0.2, 0.0, 0.1, 0.0,
    ],
    [
        1.0, 0.0, 0.5, 0.0, 0.25, 0.0, 0.125, 0.0, 0.0625, 0.0, 0.125, 0.0, 0.25, 0.0, 0.5, 0.0,
    ],
    [
        0.125, 0.0, 0.25, 0.0, 0.5, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        0.1, 0.0, 0.2, 0.0, 0.3, 0.0, 0.4, 0.0, 0.5, 0.0, 0.6, 0.0, 0.8, 0.0, 1.0, 0.0,
    ],
    [
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        1.0, 0.0, 0.5, 0.0, 0.25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        1.0, 0.0, 0.8, 0.0, 0.6, 0.0, 0.4, 0.0, 0.2, 0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        1.0, 0.0, 0.5, 0.0, 0.25, 0.0, 0.125, 0.0, 0.25, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    [
        0.25, 0.0, 0.5, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
];
/// ROM `Pattern_Length_TBL` (0x80000b18).
const LENGTHS: [usize; 20] = [
    4, 4, 4, 4, 8, 8, 8, 8, 16, 16, 8, 16, 3, 3, 6, 6, 6, 12, 12, 5,
];

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Slicer::new(sample_rate))
}

/// Speed 0..49 → steps per second.
fn normal_speed_hz(value: f32) -> f32 {
    common::q32_hz(common::int_curve(1_947_830.0, 389_566.0, 50, value, 0))
}

fn speed_hz(value: f32, bpm: f32) -> f32 {
    if zoom::is_synced(value, 49) {
        2.0 * zoom::rate_note_hz(zoom::sync_index(value, 49), bpm)
    } else {
        normal_speed_hz(value.clamp(0.0, 49.0))
    }
}

struct Slicer {
    sample_rate: f32,
    changes: Changes,
    detector: Biquad,
    peak: f32,
    peak_decay: f32,
    envelope: f32,
    envelope_gain: f32,
    envelope_pole: f32,
    follower: f32,
    follower_gain: f32,
    follower_pole: f32,
    gate_level: f32,
    gate_gain: f32,
    gate_pole: f32,
    threshold: f32,
    pulse: bool,
    phase: u32,
    increment: u32,
    step: usize,
    pattern: usize,
    pending: usize,
    switch: SwitchGate,
    wet: Ramp,
    dry: Ramp,
    level: Ramp,
}

impl Slicer {
    fn new(sample_rate: f32) -> Self {
        let bits = f32::from_bits;
        let (envelope_gain, envelope_pole) =
            common::one_pole(bits(0x3d8f_5c29), bits(0x3f6e_147b), sample_rate);
        let (follower_gain, follower_pole) =
            common::one_pole(bits(0x3c23_d70a), bits(0x3f7e_b852), sample_rate);
        let (gate_gain, gate_pole) =
            common::one_pole(bits(0x3c23_d70a), bits(0x3f7d_70a4), sample_rate);
        Self {
            sample_rate,
            changes: Changes::new(),
            detector: common::first_order(
                bits(0x3f7f_1765),
                bits(0xbf7f_1765),
                bits(0x3f7e_2eca),
                sample_rate,
            ),
            peak: 0.0,
            peak_decay: common::pole(bits(0x3f7f_3b64), sample_rate),
            envelope: 0.0,
            envelope_gain,
            envelope_pole,
            follower: 0.0,
            follower_gain,
            follower_pole,
            gate_level: 0.0,
            gate_gain,
            gate_pole,
            threshold: bits(0x3cf5_c28f),
            pulse: false,
            phase: 0,
            increment: 0,
            step: 0,
            pattern: 0,
            pending: 0,
            switch: SwitchGate::new(sample_rate),
            wet: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
            dry: Ramp::new(0.0, zoom::DEFAULT_STEP, sample_rate),
            level: Ramp::new(1.3, zoom::DEFAULT_STEP, sample_rate),
        }
    }
}

impl Effect for Slicer {
    fn set(&mut self, values: &Values, bpm: f32) {
        let (changed, tempo, first) = self.changes.update(values, bpm);
        if changed & 1 != 0 {
            let pattern = common::index(values[0], 19);
            if first {
                self.pattern = pattern;
                self.pending = pattern;
            } else if pattern != self.pending {
                self.pending = pattern;
                self.switch.request();
            }
        }
        if changed & 2 != 0 || (tempo && zoom::is_synced(values[1], 49)) {
            self.increment = common::increment(speed_hz(values[1], bpm), self.sample_rate);
        }
        if changed & 4 != 0 {
            let bal = values[2].clamp(0.0, 100.0);
            self.wet.set(zoom::curve(1.0, 0.0, 101, bal, 2));
            self.dry.set(zoom::curve(1.0, 0.0, 101, 100.0 - bal, 2));
            if first {
                self.wet.jump(self.wet.target);
                self.dry.jump(self.dry.target);
            }
        }
        if changed & 8 != 0 {
            self.threshold = crate::dsp::db_to_gain(-6.0 - values[3].clamp(0.0, 50.0));
        }
        if changed & 16 != 0 {
            self.level.set(values[4].max(0.0) / 100.0);
            if first {
                self.level.jump(self.level.target);
            }
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mean = 0.5 * (left + right);
        if self.switch.tick() {
            self.pattern = self.pending;
        }
        let detected = self.detector.process(mean);
        let old = self.phase;
        self.phase = old.wrapping_add(self.increment);
        if (old as i32) > (self.phase as i32) {
            self.step += 1;
            if self.step >= LENGTHS[self.pattern] {
                self.step = 0;
            }
        }
        let decayed = self.peak * self.peak_decay;
        self.peak = flush(if decayed < detected {
            detected
        } else {
            decayed
        });
        let level = PATTERNS[self.pattern][self.step.min(15)];
        self.envelope = flush(self.envelope * self.envelope_pole + self.envelope_gain * self.peak);
        self.follower = flush(self.follower * self.follower_pole + self.follower_gain * self.peak);
        self.gate_level = flush(self.gate_level * self.gate_pole + self.gate_gain * level);
        let pulse = self.threshold < self.envelope && self.follower < self.envelope;
        if pulse && !self.pulse {
            self.phase = 0;
            self.step = 0;
        }
        self.pulse = pulse;
        let sliced = mean * self.gate_level * self.switch.gain * self.wet.tick();
        let dry = self.dry.tick();
        let level = self.level.tick();
        [
            (sliced + left * dry) * level,
            (sliced + right * dry) * level,
        ]
    }

    fn clear(&mut self) {
        self.detector.clear();
        self.peak = 0.0;
        self.envelope = 0.0;
        self.follower = 0.0;
        self.gate_level = 0.0;
        self.pulse = false;
        self.phase = 0;
        self.step = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    #[test]
    fn speed_follows_the_editor_curve() {
        assert!((normal_speed_hz(0.0) - 4.0).abs() < 1e-3);
        assert!((normal_speed_hz(49.0) - 20.0).abs() < 1e-2);
        assert!((speed_hz(57.0, 120.0) - 4.0).abs() < 1e-4);
    }

    #[test]
    fn pattern_gates_a_steady_tone() {
        let rate = 48_000.0;
        let mut effect = Slicer::new(rate);
        let mut values = Model::from_key("mooz_slicer").unwrap().defaults();
        values[0] = 3.0; // one hit in four steps
        values[1] = 0.0; // 4 steps per second
        effect.set(&values, 120.0);
        // Steps last 0.25 s, but the restart at the tone's onset zeroes the
        // phase and the step advances at the signed rollover: the first step
        // is half as long, as in the original.
        let per_step = (rate / 4.0) as usize;
        let mut t = 0usize;
        let mut tone = |effect: &mut Slicer| {
            let x = 0.3 * (t as f32 * 0.05).sin();
            t += 1;
            effect.process([x, x])[0]
        };
        for _ in 0..per_step / 2 {
            tone(&mut effect);
        }
        let mut loud = 0;
        let mut quiet = 0;
        for _ in 0..8 {
            let mut peak = 0.0f32;
            for i in 0..per_step {
                let out = tone(&mut effect);
                if i > per_step / 4 && i < per_step * 3 / 4 {
                    peak = peak.max(out.abs());
                }
            }
            if peak > 0.3 {
                loud += 1;
            } else if peak < 0.01 {
                quiet += 1;
            }
        }
        assert_eq!((loud, quiet), (2, 6));
    }

    #[test]
    fn threshold_follows_the_editor() {
        let mut effect = Slicer::new(44_100.0);
        let mut values = Model::from_key("mooz_slicer").unwrap().defaults();
        values[3] = 50.0;
        effect.set(&values, 120.0);
        assert!((crate::dsp::gain_to_db(effect.threshold) + 56.0).abs() < 1e-3);
    }

    #[test]
    fn picking_restarts_the_pattern() {
        let rate = 44_100.0;
        let mut effect = Slicer::new(rate);
        effect.set(&Model::from_key("mooz_slicer").unwrap().defaults(), 120.0);
        for _ in 0..(rate as usize / 3) {
            effect.process([0.0; 2]);
        }
        assert!(effect.phase > 0);
        for i in 0..200 {
            let x = if i < 100 {
                0.8 * (i as f32 * 0.3).sin()
            } else {
                0.0
            };
            effect.process([x, x]);
        }
        assert_eq!(effect.step, 0);
        assert!(effect.phase < effect.increment.saturating_mul(200));
    }
}
