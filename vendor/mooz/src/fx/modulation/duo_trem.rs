//! DuoTrem (DUOTREM.ZDL), from the recovered `Fx_MOD_DuoTrem` callback
//! (`duo_tremolo.rs`) and its decoded editors.
//!
//! Two tremolo voices with the Tremolo wave set (see `tremolo::Voice`): each
//! gain is `(1 − d) + d × window`, smoothed per voice. Link (c24 from
//! `Link_tbl`): Seri runs the left input through A then B (B sees A's output
//! one sample late, both outputs get the result), Para applies A and B to the
//! stereo mean and sums them × 0.7, STR puts A on the left input and B on the
//! right. Level (c2) and the wet fade c3 scale the outputs; there is no dry path.
//!
//! Editors: RateA/B 0 mutes that voice's depth, 1..50 → `context[22]`
//! 0xbe37..0xedc5b square law over 51 steps (0.5..10 Hz), above 50 tempo notes,
//! negated for the DWN group. DPT_A/B → `context[22]` 0x06666666..0x7fffffff
//! (depth 0.05..1, ramped over 706 samples). WaveA/B as Tremolo; a group change
//! and every Link change fade the wet signal out and back in (705.6 samples
//! each). Level 0..1.5. All editors decoded; c22/c23 (B follows/mirrors A) are
//! never set by any editor and are left out.
use super::common::{Ramp, TREMOLO_WAVES, guard};
use super::tremolo::Voice;
use crate::zoom::DEFAULT_STEP;
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_duo_trem",
    name: "DuoTrem",
    category: Category::Modulation,
    zdl: "DUOTREM.ZDL",
    description: "Two tremolos in series, parallel or stereo",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "RateA",
            max: 78,
            default: 46,
            display: Display::sync_rate(50, 0.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "RateB",
            max: 78,
            default: 5,
            display: Display::sync_rate(50, 0.0),
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
            name: "DPT_A",
            max: 100,
            default: 80,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "DPT_B",
            max: 100,
            default: 90,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Link",
            max: 2,
            default: 0,
            display: Display::labels(&["Seri", "Para", "STR"]),
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "WaveA",
            max: 29,
            default: 19,
            display: Display::labels(&TREMOLO_WAVES),
        },
        // values[7] is firmware parameter word 9
        ParamInfo {
            name: "WaveB",
            max: 29,
            default: 21,
            display: Display::labels(&TREMOLO_WAVES),
        },
    ],
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Link {
    Series,
    Parallel,
    Stereo,
}

fn link(value: f32) -> Link {
    match value.round() as i32 {
        i32::MIN..=0 => Link::Series,
        1 => Link::Parallel,
        _ => Link::Stereo,
    }
}

struct DuoTrem {
    voices: [Voice; 2],
    link: Link,
    pending_link: Link,
    pending_waves: [f32; 2],
    fade: Ramp,
    previous: f32,
    level: Ramp,
    first: bool,
    rates: [f32; 2],
    bpm: f32,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(DuoTrem {
        voices: [Voice::new(sample_rate, true), Voice::new(sample_rate, true)],
        link: Link::Series,
        pending_link: Link::Series,
        pending_waves: [19.0, 21.0],
        fade: Ramp::with_duration(1.0, 705.6, sample_rate),
        previous: 0.0,
        level: Ramp::with_step(1.0, DEFAULT_STEP, sample_rate),
        first: true,
        rates: [f32::NAN; 2],
        bpm: 0.0,
    })
}

impl DuoTrem {
    fn apply_pending(&mut self) {
        self.link = self.pending_link;
        for (voice, wave) in self.voices.iter_mut().zip(self.pending_waves) {
            voice.set_wave(wave, true);
        }
    }
}

impl Effect for DuoTrem {
    fn set(&mut self, values: &Values, bpm: f32) {
        let rates = [values[0], values[1]];
        let depths = [values[3], values[4]];
        self.pending_link = link(values[5]);
        self.pending_waves = [values[6], values[7]];
        let mut restart = self.pending_link != self.link;
        for (voice, wave) in self.voices.iter_mut().zip(self.pending_waves) {
            if voice.group_changes(wave) {
                restart = true;
            } else {
                voice.set_wave(wave, false);
            }
        }
        if self.first {
            self.apply_pending();
        } else if restart {
            self.fade.set(0.0);
        }
        for (i, voice) in self.voices.iter_mut().enumerate() {
            if rates[i] != self.rates[i] || bpm != self.bpm {
                voice.set_rate(rates[i], bpm, 51);
            }
            voice.set_depth(0.05 + 0.95 * depths[i].clamp(0.0, 100.0) * 0.01);
        }
        self.rates = rates;
        self.bpm = bpm;
        self.level.set(values[2].clamp(0.0, 150.0) * 0.01);
        if self.first {
            self.first = false;
            for voice in &mut self.voices {
                voice.settle();
            }
            self.level.jump(self.level.target);
        }
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let fade = self.fade.tick();
        if fade == 0.0 && self.fade.target == 0.0 {
            self.apply_pending();
            self.fade.set(1.0);
        }
        let a = self.voices[0].gain();
        let b = self.voices[1].gain();
        let (wet_left, wet_right) = match self.link {
            Link::Series => {
                let first = left * a;
                let second = self.previous * b;
                self.previous = first;
                (second, second)
            }
            Link::Parallel => {
                let mean = 0.5 * (left + right);
                self.previous = mean * a;
                let both = 0.7 * (mean * a + mean * b);
                (both, both)
            }
            Link::Stereo => {
                self.previous = left * a;
                (left * a, right * b)
            }
        };
        let gain = fade * self.level.tick();
        [guard(wet_left * gain), guard(wet_right * gain)]
    }

    fn clear(&mut self) {
        for voice in &mut self.voices {
            voice.clear();
        }
        self.previous = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(values: [f32; 8], input: [f32; 2], n: usize) -> Vec<[f32; 2]> {
        let mut effect = create(44_100.0);
        let mut v = [0.0; crate::MAX_PARAMS];
        v[..8].copy_from_slice(&values);
        effect.set(&v, 120.0);
        (0..n).map(|_| effect.process(input)).collect()
    }

    #[test]
    fn stereo_link_modulates_each_side() {
        // A fast on the left, B muted (rate 0) on the right.
        let out = run(
            [40.0, 0.0, 100.0, 100.0, 100.0, 2.0, 20.0, 20.0],
            [1.0, 1.0],
            44_100,
        );
        let left: Vec<f32> = out[22_050..].iter().map(|f| f[0]).collect();
        let right: Vec<f32> = out[22_050..].iter().map(|f| f[1]).collect();
        let span = |x: &[f32]| {
            x.iter().copied().fold(f32::MIN, f32::max) - x.iter().copied().fold(f32::MAX, f32::min)
        };
        assert!(span(&left) > 0.9, "{}", span(&left));
        assert!(span(&right) < 1e-3, "{}", span(&right));
    }

    #[test]
    fn minimum_depth_still_moves() {
        // DPT 0 is 5 % depth in the original.
        let out = run(
            [40.0, 0.0, 100.0, 0.0, 0.0, 2.0, 20.0, 20.0],
            [1.0, 1.0],
            44_100,
        );
        let left: Vec<f32> = out[22_050..].iter().map(|f| f[0]).collect();
        let min = left.iter().copied().fold(f32::MAX, f32::min);
        assert!((min - 0.95).abs() < 0.01, "{min}");
    }

    #[test]
    fn parallel_sums_at_seventy_percent() {
        let out = run(
            [0.0, 0.0, 100.0, 100.0, 100.0, 1.0, 20.0, 20.0],
            [0.5, 0.5],
            4_410,
        );
        assert!((out[4_000][0] - 0.7).abs() < 1e-3, "{:?}", out[4_000]);
    }
}
