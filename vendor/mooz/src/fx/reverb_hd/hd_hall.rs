//! HD Hall (HDHALL.ZDL): the stereo HD Hall network of [`super::hall`].
//!
//! Editors decoded from the disassembly and checked against the firmware
//! parameter sweep (zoom-ms70-cdr `hd-hall-parameters-v1`):
//! - `predly_edit`: tap offset `(PreD + 1) * 44.1` samples, i.e. 1..200 ms.
//! - `decay_Calc_set`: `T = curve(60000, 700, 101, Decay, cube)` ms; the
//!   LoDMP ratio `curve(0.9, 0.1, 101, LoDMP, linear)` shortens the low band:
//!   high gain `0.01^(D/T)`, low gain `0.01^(D/(T·ratio))` for the six comb
//!   lengths D of HD_Hall_CombFB_Delay_tbl; the all-pass gain comes from
//!   HD_Hall_APF_FB_tbl. `Decay_PostEQ2_set` designs the 5 kHz low-pass.
//! - `HiDamp_edit`: Main design 8 (first-order high shelf) at 2000 Hz,
//!   `curve(-1, -10, 101, HiDMP, linear)` dB, in every comb.
//! - `mix_edit`: wet `curve(1, 0, 100, Mix, 1.41)` (reaching 1.014 at 100, as
//!   the original's count of 100 does), early reflections `0.675 · wet`, dry 1
//!   up to 90 and `1 - curve(1, 0, 10, Mix - 91, 1.41)` above.
use super::common::{Design, design1};
use super::hall::{COMB_MS, EarlyInput, Hall, HallSettings, LANE_LOW_SHELF, apf_feedback};
use crate::zoom::curve;
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_hd_hall",
    name: "HD Hall",
    category: Category::Reverb,
    zdl: "HDHALL.ZDL",
    description: "Dense stereo hall with low and high damping",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "PreD",
            max: 199,
            default: 80,
            display: Display::number(1.0, 1.0, 0, " ms"),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Decay",
            max: 100,
            default: 45,
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
            name: "LoDMP",
            max: 100,
            default: 32,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "HiDMP",
            max: 100,
            default: 70,
            display: Display::INT,
        },
    ],
};

/// Editor results for HD Hall knob values (pedal units).
pub fn settings(values: &Values, sample_rate: f32) -> HallSettings {
    let [predelay, decay, mix, lodmp, hidmp, ..] = *values;
    let time = curve(60_000.0, 700.0, 101, decay, 6);
    let ratio = curve(0.9, 0.1, 101, lodmp, 0);
    let wet = curve(1.0, 0.0, 100, mix, 5);
    let dry = if mix < 91.0 {
        1.0
    } else {
        1.0 - curve(1.0, 0.0, 10, mix - 91.0, 5)
    };
    HallSettings {
        predelay: (predelay + 1.0) * 44.1,
        allpass: apf_feedback(decay),
        low: COMB_MS.map(|d| super::common::comb_gain(0.01, d, time * ratio)),
        high: COMB_MS.map(|d| super::common::comb_gain(0.01, d, time)),
        wet,
        early: 0.675 * wet,
        dry,
        damp: design1(
            Design::HighShelf1,
            sample_rate,
            2000.0,
            curve(-1.0, -10.0, 101, hidmp, 0),
        ),
        lowcut: LANE_LOW_SHELF.resample(sample_rate),
    }
}

struct HdHall {
    hall: Hall,
    sample_rate: f32,
    values: Option<Values>,
}

impl Effect for HdHall {
    fn set(&mut self, values: &Values, _bpm: f32) {
        if self.values.as_ref() == Some(values) {
            return;
        }
        self.values = Some(*values);
        self.hall.apply(&settings(values, self.sample_rate));
    }

    #[inline]
    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.hall.process(frame[0], frame[1], frame)
    }

    fn clear(&mut self) {
        self.hall.clear();
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = HdHall {
        hall: Hall::new(sample_rate, EarlyInput::HdHall),
        sample_rate,
        values: None,
    };
    let defaults = crate::Model::HdHall.defaults();
    effect.set(&defaults, 120.0);
    Box::new(effect)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    fn sweep_check(values: &Values) -> HallSettings {
        settings(values, 44_100.0)
    }

    #[test]
    fn editors_match_the_firmware_sweep() {
        let mut v = Model::HdHall.defaults();
        // Decay 45, LoDMP 32 (defaults): comb 1 low/high gains c[48], c[49].
        let s = sweep_check(&v);
        assert!((s.low[0] - 0.667_121_7).abs() < 2e-4, "{}", s.low[0]);
        assert!((s.high[0] - 0.865_798_8).abs() < 2e-4, "{}", s.high[0]);
        assert!((s.high[5] - 0.924_027).abs() < 2e-4);
        assert_eq!(s.allpass, 0.7775);
        assert!((s.predelay - 3572.1).abs() < 0.5);
        // Mix: wet c[73], early c[126], dry c[74].
        assert!((s.wet - 0.516_924).abs() < 1e-4);
        assert!((s.early - 0.348_923_7).abs() < 1e-4);
        v[2] = 95.0;
        let s = sweep_check(&v);
        assert!((s.dry - 0.681_270_2).abs() < 1e-4, "{}", s.dry);
        v[2] = 100.0;
        assert!((sweep_check(&v).wet - 1.014_272).abs() < 1e-4);
        // Decay 0: T = 700 ms.
        v[1] = 0.0;
        assert!((sweep_check(&v).high[0] - 0.284_644).abs() < 2e-4);
        // HiDMP 0: shelf words c[90..93].
        v[4] = 0.0;
        let d = sweep_check(&v).damp;
        assert!((d.b0 - 0.345_725_8).abs() < 3e-3 && (d.f1 - 0.913_719_3).abs() < 3e-3);
    }

    #[test]
    fn rings_out_and_stays_bounded() {
        let mut effect = create(48_000.0);
        let mut values = Model::HdHall.defaults();
        values[1] = 100.0;
        values[3] = 100.0;
        values[4] = 100.0;
        effect.set(&values, 120.0);
        let mut peak: f32 = 0.0;
        let mut late = 0.0;
        for n in 0..48_000 * 6 {
            let x = if n < 4800 {
                ((n as f32) * 0.05).sin() * 0.5
            } else {
                0.0
            };
            let [l, r] = effect.process([x, x]);
            assert!(l.is_finite() && r.is_finite());
            peak = peak.max(l.abs()).max(r.abs());
            if n > 48_000 * 5 {
                late += l * l;
            }
        }
        assert!(peak < 4.0, "{peak}");
        assert!(late > 1e-6, "long decay should still ring after 5 s");
    }
}
