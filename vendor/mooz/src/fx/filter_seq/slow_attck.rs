//! SlowATTCK (SLWATK.ZDL, `Fx_DYN_SlowATTCK`): a volume swell on every note.
//!
//! Callback topology, followed as recovered: the dry input feeds the attack
//! detector shared with SlowFLTR (100 Hz high-pass, decaying peak, fast and
//! slow followers; a rising attack restarts a timer that climbs linearly to
//! one in `Time`). The gain is `c11 * t^4 + c10 * t^2`, smoothed by a one-pole
//! (`c21`/`c22`), and multiplies the mono mean delayed 470 samples (10.7 ms
//! look-ahead, so the swell restarts before the attack sounds). The result
//! times Level goes to both channels. The EnvIn editor selects the external
//! (dry) detector input, as here.
//!
//! Editors decoded from the disassembly: Time (square curve 0.115..2.08 s,
//! `c9 = 1 / (T * 44100)`), Curve (`c10 = 0.5 - 0.5 t^2`,
//! `c11 = 0.5 + 0.5 t^2` with `t = Curve / 10`, float setter step 0x3af7c5ec)
//! and Level (`value / 100`).
use super::{
    common::{self, AttackDetector, Changes},
    slow_fltr::LOOKAHEAD,
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{DelayLine, flush},
    zoom::{self, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_slow_attck",
    name: "SlowATTCK",
    category: Category::Dynamics,
    zdl: "SLWATK.ZDL",
    description: "Volume swell restarted by each picked note",
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
    ],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(SlowAttck::new(sample_rate))
}

/// Time 0..49 → seconds.
fn time_seconds(value: f32) -> f32 {
    zoom::curve(
        2.08,
        f32::from_bits(0x3deb_851f),
        50,
        value.clamp(0.0, 49.0),
        1,
    )
}

struct SlowAttck {
    changes: Changes,
    detector: AttackDetector,
    square_weight: Ramp,
    fourth_weight: Ramp,
    gain: f32,
    gain_coefficient: f32,
    gain_pole: f32,
    line: DelayLine,
    delay: usize,
    level: Ramp,
}

impl SlowAttck {
    fn new(sample_rate: f32) -> Self {
        let delay = (LOOKAHEAD / common::ratio(sample_rate)).round() as usize;
        let (gain_coefficient, gain_pole) = common::one_pole(
            f32::from_bits(0x3c8a_2869),
            f32::from_bits(0x3f7b_aebd),
            sample_rate,
        );
        Self {
            changes: Changes::new(),
            detector: AttackDetector::new(sample_rate),
            square_weight: Ramp::new(0.0, common::FAST_STEP, sample_rate),
            fourth_weight: Ramp::new(1.0, common::FAST_STEP, sample_rate),
            gain: 0.0,
            gain_coefficient,
            gain_pole,
            line: DelayLine::new(delay + 1),
            delay,
            level: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
        }
    }
}

impl Effect for SlowAttck {
    fn set(&mut self, values: &Values, bpm: f32) {
        let (changed, _, first) = self.changes.update(values, bpm);
        if changed & 1 != 0 {
            self.detector.set_time(time_seconds(values[0]));
        }
        if changed & 2 != 0 {
            let t = values[1].clamp(0.0, 10.0) / 10.0;
            self.square_weight.set(0.5 - 0.5 * t * t);
            self.fourth_weight.set(0.5 + 0.5 * t * t);
            if first {
                self.square_weight.jump(self.square_weight.target);
                self.fourth_weight.jump(self.fourth_weight.target);
            }
        }
        if changed & 4 != 0 {
            self.level.set(values[2].max(0.0) / 100.0);
            if first {
                self.level.jump(self.level.target);
            }
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mean = 0.5 * (left + right);
        let timer = self.detector.process(mean);
        let square = timer * timer;
        let shape =
            self.fourth_weight.tick() * square * square + self.square_weight.tick() * square;
        self.gain = flush(self.gain * self.gain_pole + shape * self.gain_coefficient);
        self.line.write(mean);
        let out = self.gain * self.line.read(self.delay) * self.level.tick();
        [out, out]
    }

    fn clear(&mut self) {
        self.detector.clear();
        self.gain = 0.0;
        self.line.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    #[test]
    fn time_follows_the_editor_curve() {
        assert!((time_seconds(0.0) - 0.115).abs() < 1e-6);
        assert!((time_seconds(49.0) - 2.08).abs() < 1e-5);
    }

    #[test]
    fn notes_swell_in_over_time_and_reach_unity() {
        let rate = 44_100.0;
        let mut effect = SlowAttck::new(rate);
        let mut values = Model::from_key("mooz_slow_attck").unwrap().defaults();
        values[0] = 10.0;
        effect.set(&values, 120.0);
        let seconds = time_seconds(10.0);
        // A note with a sharp attack and a long sustain.
        let mut early = 0.0f32;
        let mut late = 0.0f32;
        let total = (rate * (seconds + 0.5)) as usize;
        for i in 0..total {
            let x = 0.5 * (i as f32 * 0.05).sin();
            let [out, _] = effect.process([x, x]);
            let t = i as f32 / rate;
            if t > 0.02 && t < 0.02 + seconds * 0.2 {
                early = early.max(out.abs());
            }
            if t > seconds + 0.2 {
                late = late.max(out.abs());
            }
        }
        assert!(early < 0.05, "{early}");
        assert!((late - 0.5).abs() < 0.02, "{late}");
    }
}
