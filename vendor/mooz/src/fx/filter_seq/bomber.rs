//! Bomber (BOMBER.ZDL, `Fx_SFX_Bomber`): picking fires a synthesized
//! explosion.
//!
//! Callback topology, followed as recovered: the dry input's magnitude feeds
//! a decaying peak and fast/slow followers; when the fast one rises above the
//! THRSH level and the slow one, a four-stage envelope restarts (linear attack
//! to 1, linear decay to the sustain level, a hold count, exponential
//! release), smoothed by a slow one-pole. A 32-bit LFSR noise (one new value
//! per 8 samples at 44.1 kHz) runs through two one-poles whose pole follows
//! the envelope (`c23 + c22 * env`), is driven by `Power`, clipped to ±1 and
//! scaled by the pattern's output gain, then through two more envelope poles,
//! two Tone poles and the ROM low shelf (`c28..c32`, +6 dB below ~150 Hz).
//! Each channel is `(boom * c5 + dry * c6)` times Level.
//!
//! Editors decoded from the disassembly: PTTRN (ROM attack/decay/sustain/
//! hold/pole/gain tables for HndGn, Arm, Bomb, Thndr; resets the voice and is
//! applied between a 16 ms fade-out of `c3` and the scheduled fade-in), Decay
//! (release per pattern, linear between two ROM factors), Bal (two-slope
//! curves, 16 ms ramps), THRSH (`c7 = 10^((-6 - THRSH) / 20)`), Power
//! (`c20 = 10^((Power + 20) / 20)`), Tone (ROM pole table, 16 ms ramp) and
//! Level (`value / 100`). Per-sample increments, counts and poles are
//! converted to the host rate; the envelope pole is converted per sample.
use super::common::{self, Changes, SwitchGate, TimedRamp};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, Coefficients, db_to_gain, flush},
    zoom::{self, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_bomber",
    name: "Bomber",
    category: Category::Synth,
    zdl: "BOMBER.ZDL",
    description: "Picking triggers synthesized explosion sounds",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "PTTRN",
            max: 3,
            default: 3,
            display: Display::labels(&["HndGn", "Arm", "Bomb", "Thndr"]),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Decay",
            max: 99,
            default: 49,
            display: Display::offset(1.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Bal",
            max: 100,
            default: 15,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "THRSH",
            max: 50,
            default: 40,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Power",
            max: 30,
            default: 30,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 4,
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

/// One explosion type, at 44.1 kHz.
#[derive(Debug, Clone, Copy)]
struct Pattern {
    /// `c13`: attack increment per sample.
    attack: f32,
    /// `c14`: decay increment per sample (negative).
    decay: f32,
    /// `c16`: sustain level.
    sustain: f32,
    /// `c15`: hold samples.
    hold: f32,
    /// `c22`, `c23`: envelope pole slope and offset.
    slope: f32,
    offset: f32,
    /// `c21`: output gain.
    gain: f32,
    /// ROM `0x80000648`/`0x80000658`: release factor at Decay 0 and 99.
    release: (f32, f32),
}

const fn bits(word: u32) -> f32 {
    f32::from_bits(word)
}

const PATTERNS: [Pattern; 4] = [
    Pattern {
        attack: bits(0x3c4b_2965),
        decay: bits(0xbbcf_4c5d),
        sustain: bits(0x3d40_6df6),
        hold: 1030.0,
        slope: bits(0xbf09_fe62),
        offset: bits(0x3f54_bc37),
        gain: bits(0x3d4c_cccd),
        release: (bits(0x3f7f_e4a2), bits(0x3f7f_f250)),
    },
    Pattern {
        attack: bits(0x3cb9_c278),
        decay: bits(0xbc70_6fd0),
        sustain: bits(0x3eb4_a5e9),
        hold: 61.0,
        slope: bits(0x3e44_0e44),
        offset: bits(0x3f4e_cda7),
        gain: bits(0x3d75_c28f),
        release: (bits(0x3f7f_f639), bits(0x3f7f_fa86)),
    },
    Pattern {
        attack: bits(0x3ab9_c27b),
        decay: bits(0xbcda_8a6f),
        sustain: bits(0x3f1c_28f6),
        hold: 3396.0,
        slope: bits(0x3e7c_cc53),
        offset: bits(0x3f40_c38f),
        gain: bits(0x3d75_c28f),
        release: (bits(0x3f7f_f772), bits(0x3f7f_fc33)),
    },
    Pattern {
        attack: bits(0x3a50_6d8c),
        decay: bits(0xba10_6dce),
        sustain: bits(0x3f1a_a181),
        hold: 962.0,
        slope: bits(0xbf47_f145),
        offset: bits(0x3f7c_5eb8),
        gain: bits(0x3d23_d70a),
        release: (bits(0x3f7f_fb70), bits(0x3f7f_fedc)),
    },
];

/// ROM `0x80000618`: Tone → coefficient of the two fixed one-poles.
const TONES: [u32; 11] = [
    0x3d0fa5b3, 0x3d7d8022, 0x3e0d189d, 0x3e7fe7cf, 0x3ec27176, 0x3f020834, 0x3f1e883b, 0x3f34c3f3,
    0x3f448888, 0x3f4e8df1, 0x3f538ee1,
];

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Bomber::new(sample_rate))
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Stage {
    Attack,
    Decay,
    Hold,
    Release,
}

struct Bomber {
    sample_rate: f32,
    ratio: f32,
    changes: Changes,
    // Pattern-derived, at the host rate.
    pattern: usize,
    pending: usize,
    attack: f32,
    decay: f32,
    sustain: f32,
    hold_reload: i32,
    release: f32,
    slope: f32,
    offset: f32,
    out_gain: f32,
    decay_value: f32,
    // Detector.
    peak: f32,
    peak_decay: f32,
    fast: f32,
    fast_gain: f32,
    fast_pole: f32,
    slow: f32,
    slow_gain: f32,
    slow_pole: f32,
    threshold: f32,
    latch: bool,
    // Envelope.
    stage: Stage,
    envelope: f32,
    hold: i32,
    smooth: f32,
    smooth_gain: f32,
    smooth_pole: f32,
    // Synthesis.
    noise_word: u32,
    noise: f32,
    noise_phase: f32,
    noise_step: f32,
    history: f32,
    shaped: f32,
    first: f32,
    second: f32,
    third: f32,
    fourth: f32,
    drive: f32,
    tone: TimedRamp,
    shelf: Biquad,
    switch: SwitchGate,
    wet: TimedRamp,
    dry: TimedRamp,
    level: Ramp,
}

impl Bomber {
    fn new(sample_rate: f32) -> Self {
        let (fast_gain, fast_pole) =
            common::one_pole(bits(0x3d8f_5c29), bits(0x3f6e_147b), sample_rate);
        let (slow_gain, slow_pole) =
            common::one_pole(bits(0x3c23_d70a), bits(0x3f7e_b852), sample_rate);
        let (smooth_gain, smooth_pole) =
            common::one_pole(bits(0x3a83_126f), bits(0x3f7f_be77), sample_rate);
        let shelf = Biquad::new(
            Coefficients {
                b0: bits(0x3f80_e6b0),
                b1: bits(0xbf7a_937c),
                b2: 0.0,
                a1: -bits(0x3f7c_60dc),
                a2: 0.0,
            }
            .resample(zoom::ORIGINAL_RATE, sample_rate),
        );
        let mut bomber = Self {
            sample_rate,
            ratio: common::ratio(sample_rate),
            changes: Changes::new(),
            pattern: 3,
            pending: 3,
            attack: 0.0,
            decay: 0.0,
            sustain: 0.0,
            hold_reload: 0,
            release: 0.0,
            slope: 0.0,
            offset: 0.0,
            out_gain: 0.0,
            decay_value: 49.0,
            peak: 0.0,
            peak_decay: common::pole(bits(0x3f7f_7cee), sample_rate),
            fast: 0.0,
            fast_gain,
            fast_pole,
            slow: 0.0,
            slow_gain,
            slow_pole,
            threshold: db_to_gain(-46.0),
            latch: false,
            stage: Stage::Release,
            envelope: 0.0,
            hold: 0,
            smooth: 0.0,
            smooth_gain,
            smooth_pole,
            noise_word: 1,
            noise: 0.0,
            noise_phase: 0.0,
            noise_step: zoom::ORIGINAL_RATE / (8.0 * sample_rate),
            history: 0.0,
            shaped: 0.0,
            first: 0.0,
            second: 0.0,
            third: 0.0,
            fourth: 0.0,
            drive: db_to_gain(50.0),
            tone: TimedRamp::new(0.0, sample_rate),
            shelf,
            switch: SwitchGate::new(sample_rate),
            wet: TimedRamp::new(0.0, sample_rate),
            dry: TimedRamp::new(1.0, sample_rate),
            level: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
        };
        bomber.load_pattern();
        bomber
    }

    fn load_pattern(&mut self) {
        let pattern = PATTERNS[self.pattern];
        self.attack = pattern.attack * self.ratio;
        self.decay = pattern.decay * self.ratio;
        self.sustain = pattern.sustain;
        self.hold_reload = (pattern.hold / self.ratio).round() as i32;
        self.slope = pattern.slope;
        self.offset = pattern.offset;
        self.out_gain = pattern.gain;
        self.load_release();
    }

    fn load_release(&mut self) {
        let (short, long) = PATTERNS[self.pattern].release;
        let factor = short + (long - short) * self.decay_value.clamp(0.0, 99.0) / 99.0;
        self.release = common::pole(factor, self.sample_rate);
    }

    /// The pattern editor's state reset: stage 3, silent voice, noise word 1.
    fn reset_voice(&mut self) {
        self.peak = 0.0;
        self.fast = 0.0;
        self.slow = 0.0;
        self.latch = false;
        self.stage = Stage::Release;
        self.envelope = 0.0;
        self.hold = 0;
        self.smooth = 0.0;
        self.noise_word = 1;
        self.noise = 0.0;
        self.history = 0.0;
        self.shaped = 0.0;
        self.first = 0.0;
        self.second = 0.0;
        self.third = 0.0;
        self.fourth = 0.0;
        self.shelf.clear();
    }

    #[inline]
    fn envelope_step(&mut self, triggered: bool) {
        if triggered && !self.latch {
            self.stage = Stage::Attack;
            self.envelope = 0.0;
        } else {
            match self.stage {
                Stage::Attack => {
                    self.envelope += self.attack;
                    if self.envelope >= 1.0 {
                        self.envelope = 1.0;
                        self.stage = Stage::Decay;
                    }
                }
                Stage::Decay => {
                    self.envelope += self.decay;
                    if self.envelope <= self.sustain {
                        self.envelope = self.sustain;
                        self.stage = Stage::Hold;
                    }
                }
                Stage::Hold => {
                    self.hold -= 1;
                    if self.hold <= 0 {
                        self.stage = Stage::Release;
                        self.hold = self.hold_reload;
                    }
                }
                Stage::Release => self.envelope = flush(self.envelope * self.release),
            }
        }
        self.latch = triggered;
    }
}

impl Effect for Bomber {
    fn set(&mut self, values: &Values, bpm: f32) {
        let (changed, _, first) = self.changes.update(values, bpm);
        if changed & 1 != 0 {
            let pattern = common::index(values[0], 3);
            if first {
                self.pattern = pattern;
                self.pending = pattern;
                self.load_pattern();
            } else if pattern != self.pending {
                self.pending = pattern;
                self.switch.request();
            }
        }
        if changed & 2 != 0 {
            self.decay_value = values[1];
            self.load_release();
        }
        if changed & 4 != 0 {
            let bal = values[2].clamp(0.0, 100.0);
            self.wet.set(zoom::curve(1.0, 0.0, 101, bal, 2));
            self.dry.set(zoom::curve(1.0, 0.0, 101, 100.0 - bal, 2));
            if first {
                self.wet.jump(zoom::curve(1.0, 0.0, 101, bal, 2));
                self.dry.jump(zoom::curve(1.0, 0.0, 101, 100.0 - bal, 2));
            }
        }
        if changed & 8 != 0 {
            self.threshold = db_to_gain(-6.0 - values[3].clamp(0.0, 50.0));
        }
        if changed & 16 != 0 {
            self.drive = db_to_gain(values[4].clamp(0.0, 30.0) + 20.0);
        }
        if changed & 32 != 0 {
            let tone = values[5].clamp(0.0, 10.0);
            let i = (tone as usize).min(9);
            let (a, b) = (bits(TONES[i]), bits(TONES[i + 1]));
            let coefficient = a + (b - a) * (tone - i as f32);
            let converted = 1.0 - common::pole(1.0 - coefficient, self.sample_rate);
            self.tone.set(converted);
            if first {
                self.tone.jump(converted);
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
        if self.switch.tick() {
            self.pattern = self.pending;
            self.load_pattern();
            self.reset_voice();
        }
        self.noise_phase += self.noise_step;
        if self.noise_phase >= 1.0 {
            self.noise_phase -= 1.0;
            let mask = 0xd000_0001 & 0u32.wrapping_sub(self.noise_word & 1);
            self.noise_word = mask ^ (self.noise_word >> 1);
            self.noise = self.noise_word as i32 as f32 * (1.0 / 2_147_483_648.0);
        }
        // Detector on the dry input (both channels read the same source).
        let magnitude = (0.5 * (left + right)).abs() * self.switch.gain;
        let decayed = self.peak * self.peak_decay;
        self.peak = flush(if decayed < magnitude {
            magnitude
        } else {
            decayed
        });
        self.fast = flush(self.fast * self.fast_pole + self.fast_gain * self.peak);
        self.slow = flush(self.slow * self.slow_pole + self.slow_gain * self.peak);
        let triggered = self.threshold < self.fast && self.slow < self.fast;
        self.envelope_step(triggered);
        self.smooth = flush(self.smooth * self.smooth_pole + self.smooth_gain * self.envelope);
        // Synthesis.
        let pole44 = self.offset + self.slope * self.smooth;
        let pole = if self.ratio == 1.0 {
            pole44
        } else {
            common::pow01(pole44, self.ratio)
        };
        let complement = 1.0 - pole;
        self.history = flush(self.history * pole + self.noise * complement);
        self.shaped = flush(self.shaped * pole + complement * self.history);
        let burst = (self.drive * self.smooth * self.shaped).clamp(-1.0, 1.0) * self.out_gain;
        self.first = flush(burst * complement + self.first * pole);
        self.second = flush(self.first * complement + self.second * pole);
        let tone = self.tone.tick();
        self.third = flush(self.second * tone + self.third * (1.0 - tone));
        self.fourth = flush(self.third * tone + self.fourth * (1.0 - tone));
        let boom = self.shelf.process(self.fourth) * self.wet.tick() * self.switch.gain;
        let dry = self.dry.tick();
        let level = self.level.tick();
        [(boom + left * dry) * level, (boom + right * dry) * level]
    }

    fn clear(&mut self) {
        self.reset_voice();
        self.noise_phase = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    fn boom_of(pattern: f32, rate: f32) -> (f32, usize) {
        let mut effect = Bomber::new(rate);
        let mut values = Model::from_key("mooz_bomber").unwrap().defaults();
        values[0] = pattern;
        values[2] = 100.0; // explosion only
        effect.set(&values, 120.0);
        let mut peak = 0.0f32;
        let mut last_loud = 0;
        for i in 0..(rate as usize * 3) {
            let x = if i < 2000 {
                0.5 * (i as f32 * 0.2).sin() * (-(i as f32) / 400.0).exp()
            } else {
                0.0
            };
            let [l, r] = effect.process([x, x]);
            assert_eq!(l, r);
            if i > 2000 {
                peak = peak.max(l.abs());
                if l.abs() > 1e-3 {
                    last_loud = i;
                }
            }
        }
        (peak, last_loud)
    }

    #[test]
    fn picking_fires_every_pattern() {
        for pattern in 0..4 {
            let (peak, _) = boom_of(pattern as f32, 48_000.0);
            assert!(peak > 0.005 && peak < 1.0, "{pattern}: {peak}");
        }
    }

    #[test]
    fn thunder_rings_longer_than_the_hand_grenade() {
        let (_, grenade) = boom_of(0.0, 44_100.0);
        let (_, thunder) = boom_of(3.0, 44_100.0);
        assert!(thunder > grenade * 2, "{grenade} {thunder}");
    }

    #[test]
    fn silence_does_not_fire() {
        let mut effect = Bomber::new(44_100.0);
        effect.set(&Model::from_key("mooz_bomber").unwrap().defaults(), 120.0);
        for _ in 0..44_100 {
            assert_eq!(effect.process([0.0; 2]), [0.0; 2]);
        }
    }

    #[test]
    fn envelope_runs_its_four_stages() {
        let mut effect = Bomber::new(44_100.0);
        effect.set(&Model::from_key("mooz_bomber").unwrap().defaults(), 120.0);
        effect.pattern = 1;
        effect.load_pattern();
        effect.envelope_step(true);
        assert_eq!(effect.stage, Stage::Attack);
        let mut seen = vec![];
        for _ in 0..20_000 {
            effect.envelope_step(true);
            if seen.last() != Some(&effect.stage) {
                seen.push(effect.stage);
            }
        }
        assert_eq!(
            seen,
            [Stage::Attack, Stage::Decay, Stage::Hold, Stage::Release]
        );
    }
}
