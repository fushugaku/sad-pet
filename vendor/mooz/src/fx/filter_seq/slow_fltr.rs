//! SlowFLTR (SLOWFLTR.ZDL, `Fx_FLT_SlowFilter`): a filter that sweeps open
//! (or closed) after every picked note.
//!
//! Callback topology, followed as recovered: the dry input feeds the attack
//! detector (100 Hz high-pass, decaying peak, fast and slow followers); a
//! rising attack restarts a timer that climbs linearly to one in `Time`. The
//! squared timer is raised to the Curve exponent (Main `pow`), smoothed
//! (`c21`/`c22`) and mapped to the frequency word `c29 + c28 * sweep`. The
//! mono mean is delayed 470 samples (10.7 ms look-ahead, so the sweep restarts
//! before the attack reaches the filter) and runs through one (2Pole) or two
//! (4Pole) clipped Chamberlin low-pass stages with input gain `c27` and
//! damping `c32`; the result times Level goes to both channels.
//!
//! Editors decoded from the disassembly: Time (square curve 0.075..2.08 s,
//! `c9 = 1 / (T * 44100)`), Curve (exponent linear 4.0..0.4, float setter
//! step), Reso (`Q` = square curve 1.5..20, `c27 = 10^(Q a / 20)`,
//! `c32 = b / Q`, ROM `(a, b)` = (-0.3, 1) / (-0.4, 2) by Chara, 16 ms timed
//! ramps), Chara and DRCTN (Open: `c28 = 0.3, c29 = 0.05`, about 350 Hz up to
//! 2.5 kHz; Close: `c28 = -0.3, c29 = 0.35`), both applied between a 16 ms
//! fade-out of `c3` and the scheduled fade-in, and Level (`value / 100`).
use super::common::{self, AttackDetector, Changes, ClippedSvf, SvfTuning, SwitchGate, TimedRamp};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{DelayLine, db_to_gain, flush},
    zoom::{self, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_slow_fltr",
    name: "SlowFLTR",
    category: Category::Filter,
    zdl: "SLOWFLTR.ZDL",
    description: "Filter sweep restarted by each picked note",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Time",
            max: 49,
            default: 20,
            display: Display::offset(1.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Curve",
            max: 10,
            default: 10,
            display: Display::INT,
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
            name: "Reso",
            max: 10,
            default: 6,
            display: Display::INT,
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
            name: "DRCTN",
            max: 1,
            default: 0,
            display: Display::labels(&["Open", "Close"]),
        },
    ],
};

/// ROM `c23`: the audio look-ahead in 44.1 kHz samples.
pub const LOOKAHEAD: f32 = 470.0;
/// ROM `0x800006a0`: Chara → (gain exponent per Q in dB, damping numerator).
const CHARA: [(f32, f32); 2] = [(-0.3, 1.0), (-0.4, 2.0)];
/// DRCTN → (`c28` sweep scale, `c29` offset).
const DIRECTIONS: [(f32, f32); 2] = [(0.3, 0.05), (-0.3, 0.35)];

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(SlowFltr::new(sample_rate))
}

/// Time 0..49 → seconds.
pub fn time_seconds(value: f32) -> f32 {
    zoom::curve(2.08, 0.075, 50, value.clamp(0.0, 49.0), 1)
}

fn resonance(value: f32, chara: usize) -> (f32, f32) {
    let q = zoom::curve(20.0, 1.5, 11, value.clamp(0.0, 10.0), 1);
    let (a, b) = CHARA[chara];
    (db_to_gain(q * a), b / q)
}

struct SlowFltr {
    changes: Changes,
    detector: AttackDetector,
    exponent: Ramp,
    sweep: f32,
    sweep_gain: f32,
    sweep_pole: f32,
    direction: usize,
    four_pole: bool,
    pending: (usize, usize),
    reso: f32,
    gate: SwitchGate,
    line: DelayLine,
    delay: usize,
    tuning: SvfTuning,
    input_gain: TimedRamp,
    damping: TimedRamp,
    filters: [ClippedSvf; 2],
    level: Ramp,
}

impl SlowFltr {
    fn new(sample_rate: f32) -> Self {
        let delay = (LOOKAHEAD / common::ratio(sample_rate)).round() as usize;
        let (sweep_gain, sweep_pole) = common::one_pole(
            f32::from_bits(0x3c8a_2869),
            f32::from_bits(0x3f7b_aebd),
            sample_rate,
        );
        Self {
            changes: Changes::new(),
            detector: AttackDetector::new(sample_rate),
            exponent: Ramp::new(1.0, common::FAST_STEP, sample_rate),
            sweep: 0.0,
            sweep_gain,
            sweep_pole,
            direction: 0,
            four_pole: true,
            pending: (1, 0),
            reso: 6.0,
            gate: SwitchGate::new(sample_rate),
            line: DelayLine::new(delay + 1),
            delay,
            tuning: SvfTuning::new(sample_rate),
            input_gain: TimedRamp::new(1.0, sample_rate),
            damping: TimedRamp::new(1.0, sample_rate),
            filters: [ClippedSvf::default(); 2],
            level: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
        }
    }

    fn apply_switch(&mut self) {
        let (chara, direction) = self.pending;
        self.four_pole = chara == 1;
        self.direction = direction;
        let (gain, damping) = resonance(self.reso, chara);
        self.input_gain.jump(gain);
        self.damping.jump(damping);
    }
}

impl Effect for SlowFltr {
    fn set(&mut self, values: &Values, bpm: f32) {
        let (changed, _, first) = self.changes.update(values, bpm);
        if changed & 1 != 0 {
            self.detector.set_time(time_seconds(values[0]));
        }
        if changed & 2 != 0 {
            self.exponent.set(4.0 - 0.36 * values[1].clamp(0.0, 10.0));
            if first {
                self.exponent.jump(self.exponent.target);
            }
        }
        if changed & 4 != 0 {
            self.level.set(values[2].max(0.0) / 100.0);
            if first {
                self.level.jump(self.level.target);
            }
        }
        if changed & 8 != 0 {
            self.reso = values[3];
            let (gain, damping) = resonance(self.reso, self.pending.0);
            self.input_gain.set(gain);
            self.damping.set(damping);
        }
        if changed & 48 != 0 {
            let pending = (common::index(values[4], 1), common::index(values[5], 1));
            if first {
                self.pending = pending;
                self.apply_switch();
            } else if pending != self.pending {
                self.pending = pending;
                self.gate.request();
            }
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mean = 0.5 * (left + right);
        if self.gate.tick() {
            self.apply_switch();
        }
        let timer = self.detector.process(mean);
        let curved = common::pow01(timer * timer, self.exponent.tick());
        self.sweep = flush(self.sweep * self.sweep_pole + curved * self.sweep_gain);
        let (scale, offset) = DIRECTIONS[self.direction];
        let f = self.tuning.map(offset + scale * self.sweep);
        self.line.write(mean);
        let delayed = self.line.read(self.delay);
        let gain = self.input_gain.tick();
        let damping = self.damping.tick();
        let mute = self.gate.gain;
        let (_, _, mut out) = self.filters[0].process(mute * mute * gain * delayed, f, damping);
        if self.four_pole {
            out = self.filters[1].process(out * gain, f, damping).2;
        }
        let out = out * self.level.tick();
        [out, out]
    }

    fn clear(&mut self) {
        self.detector.clear();
        self.sweep = 0.0;
        self.line.clear();
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
    fn time_follows_the_editor_curve() {
        assert!((time_seconds(0.0) - 0.075).abs() < 1e-6);
        assert!((time_seconds(49.0) - 2.08).abs() < 1e-5);
    }

    #[test]
    fn attack_restarts_the_sweep_from_the_bottom() {
        let rate = 48_000.0;
        let mut effect = SlowFltr::new(rate);
        let mut values = Model::from_key("mooz_slow_fltr").unwrap().defaults();
        values[0] = 0.0; // 75 ms
        effect.set(&values, 120.0);
        // Silence lets the timer reach the top.
        for _ in 0..(rate as usize) {
            effect.process([0.0; 2]);
        }
        assert!(effect.sweep > 0.99);
        // A plucked burst restarts it.
        let mut lowest = f32::MAX;
        for i in 0..(rate as usize / 20) {
            let x = 0.5 * (i as f32 * 0.2).sin() * (-(i as f32) / 2000.0).exp();
            effect.process([x, x]);
            lowest = lowest.min(effect.sweep);
        }
        assert!(lowest < 0.05, "{lowest}");
        // ...and it has climbed back after Time.
        for _ in 0..(rate as usize / 4) {
            effect.process([0.0; 2]);
        }
        assert!(effect.sweep > 0.95);
    }

    #[test]
    fn audio_is_delayed_by_the_lookahead() {
        let mut effect = SlowFltr::new(96_000.0);
        effect.set(
            &Model::from_key("mooz_slow_fltr").unwrap().defaults(),
            120.0,
        );
        assert_eq!(effect.delay, 1023);
        let mut first = None;
        for i in 0..2000 {
            let x = if i == 0 { 1.0 } else { 0.0 };
            let [l, _] = effect.process([x, x]);
            if first.is_none() && l.abs() > 1e-9 {
                first = Some(i);
            }
        }
        assert_eq!(first, Some(1023));
    }
}
