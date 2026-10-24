//! EarlyRef (EARLYREF.ZDL): early reflections without a reverb tail.
//!
//! Callback (zoom-ms70-cdr `early_reflections.rs`): the stereo half-sum runs
//! through the ROM high-pass c5..c7 (~200 Hz) and low-pass c8..c10 (~6 kHz)
//! and is written into ring region A together with `c53 * c30 * e3`, the last
//! early tap fed back. Four early taps e0..e3 (c15..c18) weighted by
//! c27..c30 are summed and written (times c52) into region B at c14; eight
//! late taps in B, four per side (c19..c26 pairs) weighted by c31/c33/c35/c37
//! (left) and c40/c42/c44/c46 (right), times c3, pass the first-order Tone
//! section c11..c13 and are added to the dry signal times c50.
//!
//! Editors (all decoded from `EARLYREF.ZDL.dis`): Decay computes the taps in
//! integer samples, `d0 = (Decay * 28665 / 29 + 2646) / 10`, each next early
//! tap `* 359 / 250`; late pairs `(3, 16)`, `(e, e + 30)`, `(f, f + 73)`,
//! `(g, g + 173)` with `e = 159 (d3 + 3) / 200`, `f = 159 (e + d3) / 200`,
//! `g = 159 (f + d3) / 200`. Changing it fades c3 out over 16 ms, zeroes c52
//! and c53, moves the taps, and after `((g + 173) / 441) * 10 + 10` ms brings
//! back c53 = -Decay / 200, c52 = 1 and c3 = 1 over 16 ms. Shape ramps the 12
//! gains from `shape_tbl[Shape]`; Tone copies `ToneTbl[Tone]` (low-pass for
//! 0..6, high-pass with gain for 7..10); Mix sets c50 = Mix / 100; Level sets
//! c2 = Level / 100. The output is `Level * (dry + c50 * wet)`.
//!
//! Port notes: the original Mix knob has no editor in the descriptor (Mix only
//! takes effect when the deferred Decay callback runs); here Mix acts at once.
//! Tap delays are scaled from 44.1 kHz and the ROM filters re-derived.
use super::{
    common::{FirstOrder, Glides, Ring, SETTER_SAMPLES, STEP_FAST, f, layout, ms, scale, stepped},
    rom::{EARLY_REF_COE, EARLY_REF_SHAPE, EARLY_REF_TONE},
};
use crate::{Effect, Values, catalog::*, zoom::ORIGINAL_RATE};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_early_ref",
    name: "EarlyRef",
    category: Category::Reverb,
    zdl: "EARLYREF.ZDL",
    description: "Early reflections only; Shape sets their envelope",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Decay",
            max: 29,
            default: 14,
            display: Display::offset(1.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Shape",
            max: 20,
            default: 20,
            display: Display::offset(-10.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 6,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(EarlyRef::new(sample_rate))
}

/// Tap delays at 44.1 kHz for a Decay value: four early taps in region A and
/// four (left, right) late pairs in region B, plus the restore wait in ms.
pub fn taps(decay: usize) -> ([u32; 4], [(u32, u32); 4], u32) {
    let decay = decay.min(29) as u32;
    let d0 = (decay * 28665 / 29 + 2646) / 10;
    let d1 = 359 * d0 / 250;
    let d2 = 359 * d1 / 250;
    let d3 = 359 * d2 / 250;
    let e = 159 * (d3 + 3) / 200;
    let f = 159 * (e + d3) / 200;
    let g = 159 * (f + d3) / 200;
    let wait = (g + 173) / 441 * 10 + 10;
    (
        [d0, d1, d2, d3],
        [(3, 16), (e, e + 30), (f, f + 73), (g, g + 173)],
        wait,
    )
}

// Glide indices: 12 shape gains in table order, then c3, c52, c53, c50, c2.
const SHAPE: usize = 0;
const WET: usize = 12;
const LATE_IN: usize = 13;
const FEEDBACK: usize = 14;
const MIX: usize = 15;
const LEVEL: usize = 16;

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Idle,
    /// Waiting for c3 to reach zero before moving the taps.
    FadeOut,
    /// Frames left until the deferred callback restores the reflections.
    Wait(usize),
}

pub struct EarlyRef {
    rate: f32,
    ring: Ring,
    base_b: usize,
    early: [usize; 4],
    late: [(usize, usize); 4],
    highpass: FirstOrder,
    lowpass: FirstOrder,
    tone: [FirstOrder; 2],
    glides: Glides<17>,
    decay: Option<usize>,
    pending: usize,
    phase: Phase,
    shape: Option<usize>,
    tone_index: Option<usize>,
    mix: f32,
    level: f32,
}

impl EarlyRef {
    pub fn new(rate: f32) -> Self {
        let (early, late, _) = taps(29);
        let length_a = scale(early[3] as f32, rate);
        let length_b = scale(late[3].1 as f32, rate);
        let ([_, base_b], total) = layout([length_a, length_b]);
        let c = |i: usize| f(EARLY_REF_COE[i]);
        let mut effect = Self {
            rate,
            ring: Ring::new(total),
            base_b,
            early: [1; 4],
            late: [(1, 1); 4],
            highpass: FirstOrder::resampled(c(5), c(6), c(7), ORIGINAL_RATE, rate),
            lowpass: FirstOrder::resampled(c(8), c(9), c(10), ORIGINAL_RATE, rate),
            tone: [FirstOrder::default(); 2],
            glides: Glides::new([0.0; 17], rate),
            decay: None,
            pending: 0,
            phase: Phase::Idle,
            shape: None,
            tone_index: None,
            mix: -1.0,
            level: -1.0,
        };
        effect.move_taps(14);
        effect
    }

    fn move_taps(&mut self, decay: usize) {
        let (early, late, _) = taps(decay);
        let rate = self.rate;
        self.early = early.map(|d| scale(d as f32, rate));
        self.late = late.map(|(l, r)| {
            (
                self.base_b + scale(l as f32, rate),
                self.base_b + scale(r as f32, rate),
            )
        });
    }

    fn set_decay(&mut self, decay: usize, jump: bool) {
        if self.decay == Some(decay) {
            return;
        }
        self.decay = Some(decay);
        self.pending = decay;
        if jump {
            self.move_taps(decay);
            self.glides.jump(WET, 1.0);
            self.glides.jump(LATE_IN, 1.0);
            self.glides.jump(FEEDBACK, -(decay as f32) / 200.0);
            self.phase = Phase::Idle;
        } else if self.phase == Phase::Idle {
            self.glides.over(WET, 0.0, SETTER_SAMPLES);
            self.phase = Phase::FadeOut;
        } else if let Phase::Wait(_) = self.phase {
            self.mute_and_move();
        }
    }

    /// c3 is silent: zero c52/c53, move the taps, schedule the restore.
    fn mute_and_move(&mut self) {
        self.glides.jump(LATE_IN, 0.0);
        self.glides.jump(FEEDBACK, 0.0);
        self.move_taps(self.pending);
        let (_, _, wait) = taps(self.pending);
        self.phase = Phase::Wait(ms(wait as f32, self.rate).max(1));
    }

    fn restore(&mut self) {
        let decay = self.pending as f32;
        self.glides.over(FEEDBACK, -decay / 200.0, SETTER_SAMPLES);
        self.glides.over(LATE_IN, 1.0, SETTER_SAMPLES);
        self.glides.over(WET, 1.0, SETTER_SAMPLES);
        self.phase = Phase::Idle;
    }

    #[inline]
    fn transition(&mut self) {
        match self.phase {
            Phase::Idle => {}
            Phase::FadeOut => {
                if self.glides.value[WET] == 0.0 {
                    self.mute_and_move();
                }
            }
            Phase::Wait(left) => {
                if left <= 1 {
                    self.restore();
                } else {
                    self.phase = Phase::Wait(left - 1);
                }
            }
        }
    }
}

impl Effect for EarlyRef {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let jump = self.decay.is_none();
        self.set_decay(stepped(values[0], 29), jump);
        let shape = stepped(values[1], 20);
        if self.shape != Some(shape) {
            self.shape = Some(shape);
            for (i, &word) in EARLY_REF_SHAPE[shape].iter().enumerate() {
                if jump {
                    self.glides.jump(SHAPE + i, f(word));
                } else {
                    self.glides.to(SHAPE + i, f(word), STEP_FAST);
                }
            }
        }
        let tone = stepped(values[3], 10);
        if self.tone_index != Some(tone) {
            self.tone_index = Some(tone);
            let [b0, b1, a] = EARLY_REF_TONE[tone].map(f);
            for section in &mut self.tone {
                section.set_resampled(b0, b1, a, ORIGINAL_RATE, self.rate);
            }
        }
        for (index, value, last, target) in [
            (
                MIX,
                values[2],
                &mut self.mix,
                values[2].clamp(0.0, 100.0) / 100.0,
            ),
            (
                LEVEL,
                values[4],
                &mut self.level,
                values[4].clamp(0.0, 150.0) / 100.0,
            ),
        ] {
            if value != *last {
                *last = value;
                if jump {
                    self.glides.jump(index, target);
                } else {
                    self.glides.to(index, target, crate::zoom::DEFAULT_STEP);
                }
            }
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        self.glides.tick();
        self.transition();
        let g = &self.glides.value;
        let ring = &self.ring;
        let early: [f32; 4] = std::array::from_fn(|j| ring.read(self.early[j]));
        let mut late_left = 0.0;
        let mut late_right = 0.0;
        // Shape table order: c27..c30, then (c31, c40), (c33, c42), (c35, c44), (c37, c46).
        for (j, &(l, r)) in self.late.iter().enumerate() {
            late_left += g[SHAPE + 4 + 2 * j] * ring.read(l);
            late_right += g[SHAPE + 5 + 2 * j] * ring.read(r);
        }
        let early_sum = g[SHAPE] * early[0]
            + g[SHAPE + 1] * early[1]
            + g[SHAPE + 2] * early[2]
            + g[SHAPE + 3] * early[3];
        let weighted_last = g[SHAPE + 3] * early[3];

        let first = self.highpass.process(0.5 * (left + right));
        let second = self.lowpass.process(first);
        let write_a = second + g[FEEDBACK] * weighted_last;
        let write_b = g[LATE_IN] * early_sum;
        let wet = [
            self.tone[0].process(g[WET] * late_left),
            self.tone[1].process(g[WET] * late_right),
        ];
        let (mix, level) = (g[MIX], g[LEVEL]);
        self.ring.write(0, write_a);
        self.ring.write(self.base_b, write_b);
        self.ring.advance();
        [
            level * (left + mix * wet[0]),
            level * (right + mix * wet[1]),
        ]
    }

    fn clear(&mut self) {
        self.ring.clear();
        self.highpass.clear();
        self.lowpass.clear();
        for section in &mut self.tone {
            section.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::reverb::hall_family::tests::{first_arrival, values};

    #[test]
    fn decay_taps_follow_the_editor_arithmetic() {
        let (early, late, wait) = taps(14);
        assert_eq!(early, [1648, 2366, 3397, 4878]);
        assert_eq!(late[1], (3880, 3910));
        assert_eq!(late[3], (9412, 9585));
        assert_eq!(wait, 220);
        let (early, late, _) = taps(0);
        assert_eq!(early[0], 264);
        assert_eq!(late[0], (3, 16));
    }

    #[test]
    fn first_reflection_arrives_after_the_first_taps() {
        for rate in [44_100.0, 96_000.0] {
            let mut effect = EarlyRef::new(rate);
            effect.set(&values(&[14.0, 20.0, 100.0, 6.0, 100.0]), 120.0);
            // The earliest path: early tap d0 then the late pair (3, 16).
            let expected = scale(1648.0, rate) + scale(3.0, rate);
            let arrival = first_arrival(&mut effect, rate, 1e-5);
            assert!(
                arrival.abs_diff(expected) <= 2,
                "{rate}: {arrival} vs {expected}"
            );
        }
    }

    #[test]
    fn decay_change_mutes_then_restores() {
        let rate = 48_000.0;
        let mut effect = EarlyRef::new(rate);
        effect.set(&values(&[14.0, 10.0, 100.0, 6.0, 100.0]), 120.0);
        effect.set(&values(&[29.0, 10.0, 100.0, 6.0, 100.0]), 120.0);
        for _ in 0..1000 {
            effect.process([0.0; 2]);
        }
        assert_eq!(effect.glides.value[WET], 0.0);
        assert!(matches!(effect.phase, Phase::Wait(_)));
        for _ in 0..(rate as usize / 2) {
            effect.process([0.0; 2]);
        }
        assert_eq!(effect.glides.value[WET], 1.0);
        assert!((effect.glides.value[FEEDBACK] + 29.0 / 200.0).abs() < 1e-6);
    }
}
