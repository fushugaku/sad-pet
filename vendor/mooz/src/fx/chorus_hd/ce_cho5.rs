//! CE-Cho5 (CE_CHO5.ZDL), the BOSS CE-5 style chorus. Its callback is the
//! SuperCho equations ([`bbd`](super::bbd)); ROM and editors differ:
//!
//! - `rate_edit`: `p = (RATE/100)^2.44`, increment `c51 = 23870.67 + 555803 p`
//!   (0.25..5.95 Hz), wave depth `c55 = 1 - 0.2 p`.
//! - `depth_edit`: amplitude `curve(2.52, 0.12)` ms, center `curve(4.92, 4.12)` ms.
//! - `eq_low_edit`: c40..c44 = second-order high-pass (designer type 3, Q 1.2)
//!   at the ROM table 1000 Hz (LOW 0) .. 80 Hz (LOW 100).
//! - `eq_high_edit`: c35..c39 = first-order high shelf (type 8) at 9016 Hz,
//!   gain `curve(0, -24)` dB.
//! - `mix_edit`: `c6 = E.LVL / 100`; `mode_edit`: c56 mono / stereo routing.
//!
//! The first tone slot (c30..c34) is an identity in the CE-Cho5 ROM.
use super::bbd::Bbd;
use super::common::{Kind, design, table};
use super::super_cho::MONO_STEREO;
use crate::{Effect, Values, catalog::*, zoom::curve};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_ce_cho5",
    name: "CE-Cho5",
    category: Category::Modulation,
    zdl: "CE_CHO5.ZDL",
    description: "CE-5 style chorus with low and high filters",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "E.LVL",
            max: 120,
            default: 100,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "RATE",
            max: 100,
            default: 31,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "DEPTH",
            max: 100,
            default: 67,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "LOW",
            max: 100,
            default: 43,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "HIGH",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "MODE",
            max: 1,
            default: 0,
            display: MONO_STEREO,
        },
    ],
};

/// CE_CHO5 0x80000540: LOW high-pass corners, one per 10 steps.
const LOW_HZ: [f32; 11] = [
    1000.0, 872.0, 744.0, 616.0, 488.0, 360.0, 304.0, 248.0, 192.0, 136.0, 80.0,
];

struct CeCho5 {
    sample_rate: f32,
    core: Bbd,
    last: Values,
    fresh: bool,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = CeCho5 {
        sample_rate,
        core: Bbd::new(sample_rate, 10.0, 0.6, 0.4, None),
        last: [f32::NAN; crate::MAX_PARAMS],
        fresh: true,
    };
    effect.set(&crate::Model::CeCho5.defaults(), 120.0);
    Box::new(effect)
}

fn rate_words(rate: f32) -> (f32, f32) {
    let p = (rate.clamp(0.0, 100.0) / 100.0).powf(2.44);
    (23_870.668 + 555_803.8 * p, 1.0 - 0.2 * p)
}

impl Effect for CeCho5 {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let jump = self.fresh;
        self.fresh = false;
        let changed = |i: usize, last: &Values| values[i] != last[i];
        if changed(0, &self.last) {
            self.core.set_wet(curve(1.2, 0.0, 121, values[0], 0), jump);
        }
        if changed(1, &self.last) {
            let (increment, scale) = rate_words(values[1]);
            self.core.lfo.set_increment(increment, self.sample_rate);
            self.core.lfo.scale = scale;
        }
        if changed(2, &self.last) {
            let center = curve(4.92, 4.12, 101, values[2], 0);
            let amplitude = curve(2.52, 0.12, 101, values[2], 0);
            self.core.lfo.set_targets(center, amplitude);
        }
        if changed(3, &self.last) {
            let hz = table(&LOW_HZ, values[3], 100.0);
            self.core.tone[2].set(design(Kind::HighPass2, self.sample_rate, hz, 0.0, 1.2));
        }
        if changed(4, &self.last) {
            let gain = curve(0.0, -24.0, 101, values[4], 0);
            self.core.tone[1].set(design(
                Kind::HighShelf1,
                self.sample_rate,
                9016.0,
                gain,
                1.0,
            ));
        }
        if changed(5, &self.last) {
            self.core.set_stereo(values[5].round() >= 1.0, jump);
        }
        self.last = *values;
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
    use crate::dsp::gain_to_db;

    #[test]
    fn rate_matches_the_editor_constants() {
        let hz = |rate: f32| rate_words(rate).0 * 44_100.0 / 4_294_967_296.0;
        assert!((hz(0.0) - 0.245).abs() < 0.002);
        assert!((hz(100.0) - 5.95).abs() < 0.02);
        assert!((hz(31.0) - 0.573).abs() < 0.005);
    }

    #[test]
    fn high_knob_is_a_shelf_cut_above_9k() {
        let shelf = design(Kind::HighShelf1, 48_000.0, 9016.0, -12.0, 1.0);
        assert!(gain_to_db(shelf.magnitude(48_000.0, 200.0)).abs() < 0.2);
        assert!(gain_to_db(shelf.magnitude(48_000.0, 20_000.0)) < -9.0);
        let low = design(
            Kind::HighPass2,
            48_000.0,
            table(&LOW_HZ, 43.0, 100.0),
            0.0,
            1.2,
        );
        assert!(gain_to_db(low.magnitude(48_000.0, 100.0)) < -20.0);
        assert!(gain_to_db(low.magnitude(48_000.0, 5000.0)).abs() < 0.2);
    }
}
