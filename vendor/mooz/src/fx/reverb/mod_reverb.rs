//! ModReverb (MODREV.ZDL): a vibrato stage feeding the Hall kernel.
//!
//! Callback (zoom-ms70-cdr `mod_reverb.rs`): `SUB_MOD_Vibrato` writes the
//! stereo half-sum (its filters c12..c19 are identity) into its own ring and
//! reads it back with linear interpolation at `c29 + (c30 * p) >> 22`
//! samples, where `p` is a triangle (`|phase|`, phase += c22 per sample)
//! smoothed by the Q31 one-pole c27/c28; it outputs twice the tap on both
//! channels. `SUB_REV_Hall` is the Hall kernel (see
//! [`super::hall_family`]) with HALL.ZDL's tables, its c48/c49 set for wet
//! only and its input gain c20 = 1, so it hears `L + R`. The wrapper mixes
//! `Level * (dry + c5 * hall)`.
//!
//! Editors (all decoded from `MODREV.ZDL.dis`): Depth sets c30 to the integer
//! curve `0x5affd333 * Depth / 100` (ramped by 0x3deb7 per sample); Rate sets
//! c22 to `9739 + 535653 (Rate / 48)^2` (Q32 phase step: 0.1 Hz .. ~5.8 Hz);
//! Decay, Tone and PreD are the Hall editors; Mix sets c5 = `(Mix / 100)^1.41`
//! over 16 ms; Level sets c2 = Level / 100.
//!
//! Port notes: the delay, LFO step and smoothing pole are carried to the host
//! rate; the Hall stage is shared with the five Hall models.
use super::{
    common::{Glides, SETTER_SAMPLES, stepped},
    hall_family::{HallCore, PREDELAY_MAX, mix_curve},
    rom::{HALL, MOD_REVERB_VIBRATO},
};
use crate::{Effect, Values, catalog::*, dsp::DelayLine, zoom::ORIGINAL_RATE};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_mod_reverb",
    name: "ModReverb",
    category: Category::Reverb,
    zdl: "MODREV.ZDL",
    description: "Hall reverb fed through a vibrato for a wavering tail",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 38,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Decay",
            max: 29,
            default: 19,
            display: Display::offset(1.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 45,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Rate",
            max: 49,
            default: 19,
            display: Display::offset(1.0),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 6,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "PreD",
            max: 99,
            default: 29,
            display: Display::number(1.0, 1.0, 0, " ms"),
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

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(ModReverb::new(sample_rate))
}

/// Q31 depth word for Depth 100.
const DEPTH_MAX: f32 = 0x5aff_d333 as f32 / 2_147_483_648.0;
/// Integer setter step of the depth word, per 44.1 kHz sample.
const DEPTH_STEP: f32 = 0x0003_deb7 as f32 / 2_147_483_648.0;
/// The tap offset is `(depth * p) >> 22` in Q31: 512 samples at full scale.
const DELAY_SCALE: f32 = 512.0;

/// LFO frequency in Hz for a Rate value (integer square curve over 49 steps).
pub fn rate_hz(rate: f32) -> f32 {
    let t = rate.max(0.0) / 48.0;
    let step = 9739.0 + (545_392.0 - 9739.0) * t * t;
    step * ORIGINAL_RATE / 4_294_967_296.0
}

const DEPTH: usize = 0;
const MIX: usize = 1;
const LEVEL: usize = 2;

pub struct ModReverb {
    rate: f32,
    line: DelayLine,
    phase: f32,
    increment: f32,
    smooth_gain: f32,
    smooth_pole: f32,
    smoothed: f32,
    delay_scale: f32,
    hall: HallCore,
    glides: Glides<3>,
    last: [f32; 4],
    primed: bool,
}

impl ModReverb {
    pub fn new(rate: f32) -> Self {
        let c = |i: usize| MOD_REVERB_VIBRATO[i - 7] as f32 / 2_147_483_648.0;
        let (smooth_gain, smooth_pole) = super::common::one_pole_at(c(27), c(28), rate);
        let delay_scale = DELAY_SCALE * rate / ORIGINAL_RATE;
        Self {
            rate,
            line: DelayLine::new((DEPTH_MAX * delay_scale) as usize + 4),
            phase: 0.0,
            increment: 0.0,
            smooth_gain,
            smooth_pole,
            smoothed: 0.0,
            delay_scale,
            hall: HallCore::new(&HALL, rate),
            glides: Glides::new([0.0, 0.0, 1.0], rate),
            last: [-1.0; 4],
            primed: false,
        }
    }
}

impl Effect for ModReverb {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let jump = !self.primed;
        self.primed = true;
        self.hall.set_decay(stepped(values[1], 29), jump);
        self.hall.set_tone(stepped(values[4], 10));
        self.hall
            .set_predelay(stepped(values[5], PREDELAY_MAX), jump);
        if values[3] != self.last[3] {
            self.last[3] = values[3];
            // The Rate editor is stepped; the step is a direct store.
            self.increment = rate_hz(values[3].round()) / self.rate;
        }
        let targets = [
            (
                DEPTH,
                values[0],
                DEPTH_MAX * values[0].clamp(0.0, 100.0) / 100.0,
            ),
            (MIX, values[2], mix_curve(values[2])),
            (LEVEL, values[6], values[6].clamp(0.0, 150.0) / 100.0),
        ];
        for (index, value, target) in targets {
            if value == self.last[index] {
                continue;
            }
            self.last[index] = value;
            if jump {
                self.glides.jump(index, target);
            } else {
                match index {
                    DEPTH => self.glides.to(index, target, DEPTH_STEP),
                    MIX => self.glides.over(index, target, SETTER_SAMPLES),
                    _ => self.glides.to(index, target, crate::zoom::DEFAULT_STEP),
                }
            }
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        self.glides.tick();
        let [depth, mix, level] = self.glides.value;
        // Vibrato: |phase| triangle in 0..1, Q31 one-pole smoothing.
        self.phase += self.increment;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        let triangle = 1.0 - (2.0 * self.phase - 1.0).abs();
        self.smoothed =
            crate::dsp::flush(self.smooth_gain * triangle + self.smooth_pole * self.smoothed);
        self.line.write(0.5 * (left + right));
        let delay = depth * self.smoothed * self.delay_scale;
        let tap = self.line.read_linear(delay);
        // Both vibrato outputs are 2 * tap; the Hall averages them.
        let wet = self.hall.process(2.0 * tap);
        [
            level * (left + mix * wet[0]),
            level * (right + mix * wet[1]),
        ]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.smoothed = 0.0;
        self.hall.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::reverb::common::f;
    use crate::fx::reverb::hall_family::tests::{tail_energy, values};

    #[test]
    fn rate_spans_the_editor_curve() {
        assert!((rate_hz(0.0) - 0.1).abs() < 0.01);
        assert!((rate_hz(48.0) - 5.6).abs() < 0.05);
        assert!((rate_hz(19.0) - 0.96).abs() < 0.02);
    }

    #[test]
    fn vibrato_tables_match_the_rom() {
        // c24 = 1.0 (|phase| only), c25 = 0, c29 base 0, shifts 22 and 10.
        assert_eq!(MOD_REVERB_VIBRATO[24 - 7], 0x7fff_ffff);
        assert_eq!(MOD_REVERB_VIBRATO[29 - 7], 0);
        assert_eq!(MOD_REVERB_VIBRATO[31 - 7], 22);
        assert_eq!(MOD_REVERB_VIBRATO[32 - 7], 10);
        assert_eq!(f(MOD_REVERB_VIBRATO[36 - 7]), 1.0);
    }

    #[test]
    fn depth_bends_the_delay_and_the_tail_rings() {
        let rate = 48_000.0;
        let mut effect = ModReverb::new(rate);
        effect.set(&values(&[100.0, 19.0, 100.0, 40.0, 6.0, 0.0, 100.0]), 120.0);
        let mut lo = f32::MAX;
        let mut hi = 0.0f32;
        for _ in 0..(rate as usize * 2) {
            effect.process([0.0; 2]);
            let delay = effect.glides.value[DEPTH] * effect.smoothed * effect.delay_scale;
            lo = lo.min(delay);
            hi = hi.max(delay);
        }
        assert!(hi > 100.0 && hi < 400.0 * rate / 44_100.0, "{lo} {hi}");
        assert!(tail_energy(&mut effect, rate, 0.5, 1.0) > 1e-4);
    }
}
