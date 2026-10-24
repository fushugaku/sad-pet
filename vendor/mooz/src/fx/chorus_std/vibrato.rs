//! Vibrato (VIBRATO.ZDL): the shared modulated-delay callback (see
//! [`super::modulated`]) with no base delay (c22 = 0), so the tap only
//! sweeps; the LFO passes the ROM one-pole c20/c21 (≈2.4 Hz, gain 0.83).
//!
//! Editors decoded from the disassembly:
//! - Depth: c23 = curve(0x5affd333, 0, 101, linear), integer ramp (0..364
//!   samples of sweep before smoothing).
//! - Rate: 0..50 is c15 = curve(974939, 19478, 49, square) (count 49 as in the
//!   original, so 49 and 50 run past 10 Hz); above 50 the tempo divisions.
//! - Bal: c28 = (100 - Bal)/100 dry and wet `2 c29` = curve(1, 0, 101, Bal,
//!   two-slope), both by the stepped setter.
//! - Tone: the shared tone table; Level: c2 = Level/100.
use super::{
    common::{TONE, UNSET, defaults, sanitize, step, tone_hp, tone_lp},
    modulated::{Core, Layout},
};
use crate::{
    Effect, Values,
    catalog::*,
    zoom::{self, curve},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_vibrato",
    name: "Vibrato",
    category: Category::Modulation,
    zdl: "VIBRATO.ZDL",
    description: "Pitch vibrato with dry balance and tempo sync",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 40,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Rate",
            max: 78,
            default: 30,
            display: Display::sync_rate(50, 0.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Bal",
            max: 100,
            default: 72,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 7,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Level",
            max: 150,
            default: 120,
            display: Display::INT,
        },
    ],
};

const SYNC: u16 = 50;

const LAYOUT: Layout = Layout {
    max_base: 0.0,
    span: 512.0,
    smoothing: (613_279, 2_146_747_713),
    offset: 0,
    jump: 400.0,
    timed_wet: false,
};

struct Vibrato {
    core: Core,
    last: Values,
    bpm: f32,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = Vibrato {
        core: Core::new(LAYOUT, sample_rate),
        last: UNSET,
        bpm: f32::NAN,
    };
    effect.set(&defaults(&INFO), 120.0);
    effect.core.settle();
    Box::new(effect)
}

impl Effect for Vibrato {
    fn set(&mut self, values: &Values, bpm: f32) {
        let v = sanitize(values, &INFO);
        let last = self.last;
        if v[0] != last[0] {
            self.core
                .set_depth(curve(1_526_715_187.0, 0.0, 101, v[0], 0));
        }
        let synced = zoom::is_synced(v[1], SYNC);
        if v[1] != last[1] || (synced && bpm != self.bpm) {
            if synced {
                self.core
                    .set_hz(zoom::rate_note_hz(zoom::sync_index(v[1], SYNC), bpm));
            } else {
                self.core
                    .set_increment_44k(curve(974_939.0, 19_478.0, 49, v[1], 1));
            }
        }
        if v[2] != last[2] {
            self.core.set_dry(curve(1.0, 0.0, 101, 100.0 - v[2], 0));
            self.core.set_wet(0.5 * curve(1.0, 0.0, 101, v[2], 2));
        }
        if step(v[3], 10) != step(last[3], 10) || last[3].is_nan() {
            let row = &TONE[step(v[3], 10)];
            self.core.set_filters(tone_hp(row), tone_lp(row));
        }
        if v[4] != last[4] {
            self.core.set_level(curve(1.5, 0.0, 151, v[4], 0));
        }
        self.last = v;
        self.bpm = bpm;
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.core.process(frame)
    }

    fn clear(&mut self) {
        self.core.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::chorus_std::common::increment_hz;

    fn vibrato(sample_rate: f32) -> Vibrato {
        let mut effect = Vibrato {
            core: Core::new(LAYOUT, sample_rate),
            last: UNSET,
            bpm: f32::NAN,
        };
        effect.set(&defaults(&INFO), 120.0);
        effect.core.settle();
        effect
    }

    #[test]
    fn synced_rate_follows_the_tempo() {
        let mut effect = vibrato(48_000.0);
        let mut values = defaults(&INFO);
        values[1] = 58.0; // 1/4
        effect.set(&values, 120.0);
        assert!((increment_hz(effect.core.increment(), 48_000.0) - 2.0).abs() < 1e-3);
        effect.set(&values, 90.0);
        assert!((increment_hz(effect.core.increment(), 48_000.0) - 1.5).abs() < 1e-3);
        values[1] = 48.0;
        effect.set(&values, 90.0);
        assert!((increment_hz(effect.core.increment(), 48_000.0) - 10.01).abs() < 0.02);
    }

    #[test]
    fn full_balance_has_no_dry() {
        let mut effect = vibrato(48_000.0);
        let mut values = defaults(&INFO);
        values[2] = 100.0;
        values[4] = 100.0;
        effect.set(&values, 120.0);
        effect.core.settle();
        let (level, dry, wet, _) = effect.core.gains();
        assert_eq!((level, dry), (1.0, 0.0));
        assert!((2.0 * wet - 1.0).abs() < 1e-6);
    }
}
