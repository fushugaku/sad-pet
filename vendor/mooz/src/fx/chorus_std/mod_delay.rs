//! ModDelay (MODDLY.ZDL): the shared modulated-delay callback (see
//! [`super::modulated`]) as a delay up to 2 s: fixed ROM filters in the loop
//! (HP 100 Hz, LP 10 kHz), feedback from the tap, a fixed sweep c23 =
//! 0x27d11f8c (159 samples) and the ROM LFO smoothing (≈2.4 Hz, gain 0.83),
//! both outputs from the same tap over the stereo dry.
//!
//! Editors decoded from the disassembly:
//! - Time: 0..1999 is 1..2000 ms, above the tempo divisions (halved until
//!   they fit 2000 ms); c22 = 441 * ms / 10 samples. The original mutes the
//!   tap and its feedback, waits for the line to refill and fades back in;
//!   here small moves glide and jumps over 5 ms crossfade (see `TapTime`).
//! - F.B: c6 = curve(0.95, 0, 101, linear), stepped setter.
//! - Mix: c29 = curve(1, 0, 101, power 1.41), timed setter (wet added twice).
//! - Rate: c15 = curve(974939, 19478, 50, square): 0.2..10 Hz.
//! - Level: c2 = Level/100.
use super::{
    common::{UNSET, defaults, sanitize},
    modulated::{Core, Layout},
};
use crate::{
    Effect, Values,
    catalog::*,
    zoom::{self, curve},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_mod_delay",
    name: "ModDelay",
    category: Category::Delay,
    zdl: "MODDLY.ZDL",
    description: "Delay up to 2 s with a chorus-like modulated tap",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Time",
            max: 2014,
            default: 499,
            display: Display::DelayTime {
                threshold: 1999,
                offset: 1,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 62,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Rate",
            max: 49,
            default: 20,
            display: Display::offset(1.0),
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

pub(super) const THRESHOLD: u16 = 1999;
pub(super) const MAXIMUM_MS: f32 = 2000.0;

pub(super) const LAYOUT: Layout = Layout {
    max_base: MAXIMUM_MS * 44.1,
    span: 512.0,
    smoothing: (613_279, 2_146_747_713),
    offset: 0,
    jump: 5.0 * 44.1,
    timed_wet: true,
};

/// ROM c7..c12 of MODDLY and MODDLY2: HP 100 Hz, LP 10 kHz.
pub(super) const HP: [u32; 3] = [0x3f7e306e, 0xbf7e306e, 0x3f7c60dc];
pub(super) const LP: [u32; 3] = [0x3eed461d, 0x3eed461d, 0x3d95cf1b];

/// ROM c23 of both, also ModDelay2's Depth 50.
pub(super) const DEPTH: f32 = 668_016_524.0;

/// The editors shared with ModDelay2 (Time, F.B, Mix, Rate, Level in pedal
/// order).
pub(super) fn apply(core: &mut Core, v: &Values, last: &Values, bpm: f32, last_bpm: f32) {
    let synced = zoom::is_synced(v[0], THRESHOLD);
    if v[0] != last[0] || (synced && bpm != last_bpm) {
        let ms = zoom::delay_time_ms(v[0], THRESHOLD, bpm, MAXIMUM_MS);
        core.set_base(44.1 * ms);
    }
    if v[1] != last[1] {
        core.set_feedback(curve(0.95, 0.0, 101, v[1], 0));
    }
    if v[2] != last[2] {
        core.set_wet(curve(1.0, 0.0, 101, v[2], 5));
    }
    if v[3] != last[3] {
        core.set_increment_44k(curve(974_939.0, 19_478.0, 50, v[3], 1));
    }
    if v[4] != last[4] {
        core.set_level(curve(1.5, 0.0, 151, v[4], 0));
    }
}

pub(super) fn new_core(sample_rate: f32) -> Core {
    let mut core = Core::new(LAYOUT, sample_rate);
    core.set_filters(HP, LP);
    core.set_depth(DEPTH);
    core
}

struct ModDelay {
    core: Core,
    last: Values,
    bpm: f32,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = ModDelay {
        core: new_core(sample_rate),
        last: UNSET,
        bpm: f32::NAN,
    };
    effect.set(&defaults(&INFO), 120.0);
    effect.core.settle();
    Box::new(effect)
}

impl Effect for ModDelay {
    fn set(&mut self, values: &Values, bpm: f32) {
        let v = sanitize(values, &INFO);
        apply(&mut self.core, &v, &self.last, bpm, self.bpm);
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

    fn delay(sample_rate: f32, values: &Values, bpm: f32) -> ModDelay {
        let mut effect = ModDelay {
            core: new_core(sample_rate),
            last: UNSET,
            bpm: f32::NAN,
        };
        effect.set(values, bpm);
        effect.core.settle();
        effect
    }

    #[test]
    fn echo_arrives_after_the_set_time() {
        let mut values = defaults(&INFO);
        values[0] = 99.0; // 100 ms
        values[1] = 0.0;
        values[2] = 100.0;
        let mut effect = delay(48_000.0, &values, 120.0);
        let mut peak = (0, 0.0f32);
        for i in 0..12_000 {
            let x = if i == 0 { 1.0 } else { 0.0 };
            let [l, _] = effect.process([x, x]);
            if i > 10 && l.abs() > peak.1 {
                peak = (i, l.abs());
            }
        }
        // 100 ms plus at most the 159-sample (3.6 ms) sweep.
        let ms = peak.0 as f32 / 48.0;
        assert!((100.0..104.0).contains(&ms), "{ms}");
    }

    #[test]
    fn synced_time_follows_the_tempo() {
        let mut values = defaults(&INFO);
        values[0] = 2006.0; // 1/4
        let mut effect = delay(48_000.0, &values, 120.0);
        assert!((effect.core.delays().0 - 24_000.0).abs() < 1.0);
        effect.set(&values, 60.0);
        effect.core.settle();
        assert!((effect.core.delays().0 - 48_000.0).abs() < 1.0);
    }

    #[test]
    fn maximum_feedback_stays_bounded() {
        let mut values = defaults(&INFO);
        values[0] = 0.0;
        values[1] = 100.0;
        values[2] = 100.0;
        values[3] = 49.0;
        values[4] = 150.0;
        for time in [0.0, 9.0, 300.0, 1999.0] {
            values[0] = time;
            let mut effect = delay(48_000.0, &values, 120.0);
            let mut peak = 0.0f32;
            for i in 0..480_000 {
                let x = (i as f32 * 997.0 / 48_000.0 * std::f32::consts::TAU).sin() * 0.5;
                let [l, r] = effect.process([x, x]);
                peak = peak.max(l.abs()).max(r.abs());
            }
            assert!(peak.is_finite() && peak < 16.0, "{time}: {peak}");
        }
    }
}
