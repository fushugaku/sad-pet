//! SuperCho (SUPERCHO.ZDL), the BOSS CH-1 style chorus: the shared
//! [`bbd`](super::bbd) callback with SuperCho's ROM and decoded editors.
//!
//! - `rate_edit`: `p = (Rate/100)^1.62` (Main power), phase increment
//!   `c51 = 22595 + 355284 p` (0.23..3.88 Hz), wave depth `c55 = 1 - 0.17 p`.
//! - `depth_edit`: amplitude `curve(2.64, 0.16)` ms, center `curve(4.84, 3.96)`
//!   ms (stored as samples << 14 at 44.1 kHz), both one-pole smoothed.
//! - `eq_edit`: c40..c44 = first-order low-pass (Main designer type 0) at the
//!   ROM table 500..16000 Hz, `EQ/10` with `EQ%10` interpolation.
//! - `mix_edit`: wet level `c6 = E.LVL / 100` with the 0x3a77c5f5 ramp.
//! - `mode_edit`: c56, 0 = mono mix on both outputs, 1 = wet left, dry right.
//!
//! Fixed ROM tone filters: first-order high-pass at 51 Hz, second-order
//! low-pass at 10 kHz (Q 0.78). LFO polynomial `1 - (0.6 |p| + 0.4 p^2)`.
use super::bbd::Bbd;
use super::common::{Kind, design, rom, table};
use crate::{Effect, Values, catalog::*, zoom::curve};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_super_cho",
    name: "SuperCho",
    category: Category::Modulation,
    zdl: "SUPERCHO.ZDL",
    description: "CH-1 style chorus with EQ and stereo dry/wet split",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "E.LVL",
            max: 120,
            default: 50,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Rate",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "EQ",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Mode",
            max: 1,
            default: 0,
            display: MONO_STEREO,
        },
    ],
};

/// `GetString_MonoStereo`: "MONO" / "STR".
pub const MONO_STEREO: Display = Display::labels(&["Mono", "Stereo"]);

/// SUPERCHO 0x80000528: EQ low-pass corners, one per 10 steps.
const EQ_HZ: [f32; 11] = [
    500.0, 800.0, 1000.0, 2000.0, 3000.0, 4800.0, 6000.0, 8000.0, 10000.0, 12000.0, 16000.0,
];

struct SuperCho {
    sample_rate: f32,
    core: Bbd,
    last: Values,
    fresh: bool,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut core = Bbd::new(sample_rate, 10.0, 0.6, 0.4, None);
    core.tone[0].set(rom(0.996_38, -0.996_38, 0.0, 0.992_76, 0.0, sample_rate));
    core.tone[1].set(rom(
        0.261_436_67,
        0.522_873_3,
        0.261_436_67,
        0.178_092_06,
        -0.223_838_76,
        sample_rate,
    ));
    let mut effect = SuperCho {
        sample_rate,
        core,
        last: [f32::NAN; crate::MAX_PARAMS],
        fresh: true,
    };
    effect.set(&crate::Model::SuperCho.defaults(), 120.0);
    Box::new(effect)
}

/// Rate editor shared with SilkyCho: (Q32 increment at 44.1 kHz, c55).
pub fn rate_words(rate: f32) -> (f32, f32) {
    let p = (rate.clamp(0.0, 100.0) / 100.0).powf(1.62);
    (22_595.0 + 355_284.0 * p, 1.0 - 0.17 * p)
}

/// Depth editor shared with SilkyCho: (center ms, amplitude ms).
pub fn depth_ms(depth: f32) -> (f32, f32) {
    (
        curve(4.84, 3.96, 101, depth, 0),
        curve(2.64, 0.16, 101, depth, 0),
    )
}

impl Effect for SuperCho {
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
            let (center, amplitude) = depth_ms(values[2]);
            self.core.lfo.set_targets(center, amplitude);
        }
        if changed(3, &self.last) {
            let hz = table(&EQ_HZ, values[3], 100.0);
            self.core.tone[2].set(design(Kind::LowPass1, self.sample_rate, hz, 0.0, 0.78));
        }
        if changed(4, &self.last) {
            self.core.set_stereo(values[4].round() >= 1.0, jump);
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
    use crate::Model;

    #[test]
    fn rate_spans_the_rom_range() {
        let hz = |rate: f32| rate_words(rate).0 * 44_100.0 / 4_294_967_296.0;
        assert!((hz(0.0) - 0.232).abs() < 0.002);
        assert!((hz(100.0) - 3.880).abs() < 0.01);
        assert!((rate_words(100.0).1 - 0.83).abs() < 1e-6);
    }

    #[test]
    fn mono_mode_keeps_the_dry_sum_and_stereo_splits_it() {
        let mut effect = create(48_000.0);
        let mut values = Model::SuperCho.defaults();
        values[0] = 0.0;
        effect.set(&values, 120.0);
        for _ in 0..4800 {
            effect.process([0.5, 0.5]);
        }
        let out = effect.process([0.5, 0.5]);
        assert!((out[0] - 0.5).abs() < 1e-3 && (out[1] - 0.5).abs() < 1e-3);
        values[4] = 1.0;
        effect.set(&values, 120.0);
        let mut out = [0.0; 2];
        for _ in 0..4800 {
            out = effect.process([0.5, 0.5]);
        }
        assert!(out[0].abs() < 1e-3, "{out:?}");
        assert!((out[1] - 0.5).abs() < 1e-3, "{out:?}");
    }

    #[test]
    fn wet_is_a_delayed_copy_within_the_depth_range() {
        // Impulse in, look for the wet arrival: center 3.96..4.84 ms plus
        // amplitude, so 4.0..7.5 ms at the default depth.
        let mut effect = create(44_100.0);
        let mut values = Model::SuperCho.defaults();
        values[0] = 100.0;
        values[3] = 100.0;
        effect.set(&values, 120.0);
        let mut peak = (0, 0.0f32);
        for i in 0..2000 {
            let x = if i == 0 { 1.0 } else { 0.0 };
            let y = effect.process([x, x])[0];
            if i > 0 && y.abs() > peak.1 {
                peak = (i, y.abs());
            }
        }
        let ms = peak.0 as f32 / 44.1;
        assert!((4.0..7.6).contains(&ms), "{ms} ms");
    }
}
