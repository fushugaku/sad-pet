//! Air (AIR.ZDL): a single filtered feedback delay heard through two taps.
//!
//! Callback (zoom-ms70-cdr `air_reverb.rs`): the stereo half-sum times c7
//! plus the c5 tap times c3 * c8 runs through the ROM biquad c11..c15 and is
//! written at the cursor. The left output adds `c9 * c3 * tap(c5)` to the dry
//! signal, the right subtracts `c9 * c3 * tap(c6)`; c2 is the level.
//!
//! Editors (all decoded from `AIR.ZDL.dis`): Size fades c3 out over 16 ms,
//! sets c5 = `441 * (1000 + 4000 t^2) / 1000` and c6 = c5 + `441 * (250 + 1250
//! t^2) / 1000` samples (Main integer curves, `t = Size / 99`), and brings c3
//! back over 16 ms 210 ms later; Tone copies `air_damp_tbl[Tone]` (low-pass
//! biquads from 100 Hz to 5 kHz) into c11..c15; Ref sets c8 = 0.25 * Ref / 10;
//! Mix sets c9 = Mix / 100; Level sets c2 = Level / 100.
//!
//! Port notes: delays are scaled from 44.1 kHz, the ROM biquads re-derived.
use super::{
    common::{Glides, Ring, SETTER_SAMPLES, f, ms, rom_biquad, scale, stepped},
    rom::{AIR_COE, AIR_DAMP},
};
use crate::{Effect, Values, catalog::*, dsp::Biquad, zoom};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_air",
    name: "Air",
    category: Category::Reverb,
    zdl: "AIR.ZDL",
    description: "Ambience that adds the air of a room",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Size",
            max: 99,
            default: 19,
            display: Display::offset(1.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 8,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 60,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Ref",
            max: 10,
            default: 5,
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
    Box::new(Air::new(sample_rate))
}

/// Main's integer curve (`context[22]`) with the square shape.
fn integer_square(maximum: u32, minimum: u32, count: u32, selected: usize) -> u32 {
    let t = selected as f64 / f64::from(count - 1);
    minimum + (f64::from(maximum - minimum) * t * t) as u32
}

/// The two tap delays at 44.1 kHz for a Size value.
pub fn taps(size: usize) -> (u32, u32) {
    let first = 441 * integer_square(5000, 1000, 100, size) / 1000;
    let second = first + 441 * integer_square(1500, 250, 100, size) / 1000;
    (first, second)
}

/// Delay after which the deferred callback restores the taps.
const RESTORE_MS: f32 = 210.0;

const TAPS: usize = 0;
const REF: usize = 1;
const MIX: usize = 2;
const LEVEL: usize = 3;

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Idle,
    FadeOut,
    Wait(usize),
}

pub struct Air {
    rate: f32,
    ring: Ring,
    taps: (usize, usize),
    damp: Biquad,
    glides: Glides<4>,
    input: f32,
    size: Option<usize>,
    pending: usize,
    phase: Phase,
    tone: Option<usize>,
    last: [f32; 3],
}

impl Air {
    pub fn new(rate: f32) -> Self {
        let (_, longest) = taps(99);
        Self {
            rate,
            ring: Ring::new(scale(longest as f32, rate) + 1),
            taps: (1, 1),
            damp: Biquad::default(),
            glides: Glides::new([0.0; 4], rate),
            input: f(AIR_COE[7]),
            size: None,
            pending: 0,
            phase: Phase::Idle,
            tone: None,
            last: [-1.0; 3],
        }
    }

    fn move_taps(&mut self) {
        let (first, second) = taps(self.pending);
        self.taps = (
            scale(first as f32, self.rate),
            scale(second as f32, self.rate),
        );
    }
}

impl Effect for Air {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let jump = self.size.is_none();
        let size = stepped(values[0], 99);
        if self.size != Some(size) {
            self.size = Some(size);
            self.pending = size;
            if jump {
                self.move_taps();
                self.glides.jump(TAPS, 1.0);
            } else {
                match self.phase {
                    Phase::Idle => {
                        self.glides.over(TAPS, 0.0, SETTER_SAMPLES);
                        self.phase = Phase::FadeOut;
                    }
                    Phase::FadeOut => {}
                    Phase::Wait(_) => {
                        self.move_taps();
                        self.phase = Phase::Wait(ms(RESTORE_MS, self.rate));
                    }
                }
            }
        }
        let tone = stepped(values[1], 10);
        if self.tone != Some(tone) {
            self.tone = Some(tone);
            self.damp.coefficients = rom_biquad(AIR_DAMP[tone].map(f), self.rate);
        }
        for (i, (index, value, target)) in [
            (MIX, values[2], values[2].clamp(0.0, 100.0) / 100.0),
            (
                REF,
                values[3],
                zoom::curve(0.25, 0.0, 11, values[3].clamp(0.0, 10.0), 0),
            ),
            (LEVEL, values[4], values[4].clamp(0.0, 150.0) / 100.0),
        ]
        .into_iter()
        .enumerate()
        {
            if value != self.last[i] {
                self.last[i] = value;
                if jump {
                    self.glides.jump(index, target);
                } else {
                    self.glides.to(index, target, zoom::DEFAULT_STEP);
                }
            }
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        self.glides.tick();
        match self.phase {
            Phase::Idle => {}
            Phase::FadeOut => {
                if self.glides.value[TAPS] == 0.0 {
                    self.move_taps();
                    self.phase = Phase::Wait(ms(RESTORE_MS, self.rate));
                }
            }
            Phase::Wait(left) => {
                if left <= 1 {
                    self.glides.over(TAPS, 1.0, SETTER_SAMPLES);
                    self.phase = Phase::Idle;
                } else {
                    self.phase = Phase::Wait(left - 1);
                }
            }
        }
        let [gate, reflection, mix, level] = self.glides.value;
        let feedback = gate * self.ring.read(self.taps.0);
        let output = gate * self.ring.read(self.taps.1);
        let input = reflection * feedback + self.input * 0.5 * (left + right);
        self.ring.write(0, self.damp.process(input));
        self.ring.advance();
        [
            level * (left + mix * feedback),
            level * (right - mix * output),
        ]
    }

    fn clear(&mut self) {
        self.ring.clear();
        self.damp.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::reverb::hall_family::tests::{first_arrival, values};

    #[test]
    fn size_sets_the_tap_times() {
        assert_eq!(taps(0), (441, 441 + 110));
        let (first, second) = taps(99);
        assert_eq!(first, 2205);
        assert_eq!(second, 2205 + 661);
        let (first, _) = taps(19);
        assert!((500..510).contains(&first), "{first}");
    }

    #[test]
    fn echo_arrives_at_the_first_tap() {
        for rate in [44_100.0, 48_000.0] {
            let mut air = Air::new(rate);
            air.set(&values(&[50.0, 10.0, 100.0, 5.0, 100.0]), 120.0);
            let (first, _) = taps(50);
            let arrival = first_arrival(&mut air, rate, 1e-6);
            assert_eq!(arrival, scale(first as f32, rate), "{rate}");
        }
    }

    #[test]
    fn tone_zero_is_dark() {
        let low = rom_biquad(AIR_DAMP[0].map(f), 48_000.0);
        let high = rom_biquad(AIR_DAMP[10].map(f), 48_000.0);
        assert!(low.magnitude(48_000.0, 1000.0) < 0.05);
        assert!(high.magnitude(48_000.0, 1000.0) > 0.9);
        assert!((low.magnitude(48_000.0, 20.0) - 1.0).abs() < 0.1);
    }
}
