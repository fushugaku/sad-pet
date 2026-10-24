//! PitchSHFT (PTCHSHFT.ZDL): two-tap delay-line pitch shifter.
//!
//! Callback: the recovered DETUNE/PTCHSHFT graph shared with Detune
//! ([`super::two_tap`]). Editors decoded from the disassembly:
//! - `shift_edit` (Shift and Fine): c5 from the 26×51 increment table at
//!   0x80000000 (reproduced by its formula, ratio `2^((shift + (fine-25)/100)/12)`
//!   over the c8 sweep of `PITCH_SHIFT_DEPTH[shift]`), c9 = 793 samples at 0
//!   semitones and 2 otherwise, c25 = 0 only at exact unison. The editor mutes
//!   c3 over 16 ms, changes the taps and fades c3 back in; so does the port.
//! - `tone_edit`: c19 = `TWO_TAP_TONE[tone]`, c20 = 1 - c19 (input one-pole).
//! - `bal_edit`: c16 dry = two-slope(100 - Bal), c15 wet = two-slope(Bal),
//!   `context[8]` ramps; `outLv_edit`: c2 = Level / 100.
//!
//! Approximation: at exact unison the original keeps whatever the ramp word
//! held (filtered dry after a patch load, a frozen delayed copy after a turn
//! from another setting); the port always crossfades to the filtered dry there.
use super::{
    common::{SHIFT_LABELS, one_pole, out_level, shift_ratio, step, two_slope},
    rom::{PITCH_SHIFT_DEPTH, TWO_TAP_TONE, f},
    two_tap::{Geometry, TwoTapShifter},
};
use crate::{
    Effect, Values,
    catalog::*,
    zoom::{DEFAULT_STEP, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_pitch_shft",
    name: "PitchSHFT",
    category: Category::Pitch,
    zdl: "PTCHSHFT.ZDL",
    description: "Pitch shifter, -1 to +2 octaves with fine tune",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Shift",
            max: 25,
            default: 19,
            display: Display::labels(SHIFT_LABELS),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 7,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Bal",
            max: 100,
            default: 40,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Fine",
            max: 50,
            default: 25,
            display: Display::offset(-25.0),
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

/// Sweep depth of a Shift step in 44.1 kHz samples (`c8 >> c11`, c11 = 18).
fn depth(shift: usize) -> f32 {
    PITCH_SHIFT_DEPTH[shift] as f32 / 262_144.0
}

struct PitchShift {
    sample_rate: f32,
    shifter: TwoTapShifter,
    taps: Option<(usize, usize)>,
    dry: Ramp,
    wet: Ramp,
    level: Ramp,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(PitchShift {
        sample_rate,
        // 2 + the +24 sweep (5058) is the longest distance.
        shifter: TwoTapShifter::new(sample_rate, 5_070.0),
        taps: None,
        dry: Ramp::new(two_slope(60.0), DEFAULT_STEP, sample_rate),
        wet: Ramp::new(two_slope(40.0), DEFAULT_STEP, sample_rate),
        level: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
    })
}

impl Effect for PitchShift {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let taps = (step(values[0], 25), step(values[3], 50));
        if self.taps != Some(taps) {
            let (shift, fine) = taps;
            let unison = shift == 12 && fine == 25;
            let base = if shift == 12 { 793.0 } else { 2.0 };
            let geometry = Geometry::new(
                shift_ratio(shift, fine),
                base,
                depth(shift),
                self.sample_rate,
                !unison,
            );
            self.shifter.request(geometry, true);
            self.taps = Some(taps);
        }
        let tone = f(TWO_TAP_TONE[step(values[1], 10)]);
        self.shifter.set_tone(one_pole(tone, self.sample_rate));
        self.dry.set(two_slope(100.0 - values[2]));
        self.wet.set(two_slope(values[2]));
        self.level.set(out_level(values[4]));
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let wet = self.shifter.process(0.5 * (left + right)) * self.wet.tick();
        let (dry, level) = (self.dry.tick(), self.level.tick());
        [level * (left * dry + wet), level * (right * dry + wet)]
    }

    fn clear(&mut self) {
        self.shifter.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::pitch::two_tap::tests::centroid;

    fn render(values: Values, sample_rate: f32, hz: f32, seconds: f32) -> Vec<f32> {
        let mut effect = create(sample_rate);
        effect.set(&values, 120.0);
        let n = (sample_rate * seconds) as usize;
        (0..n)
            .map(|i| {
                let x = 0.5 * (i as f32 * hz / sample_rate * std::f32::consts::TAU).sin();
                effect.process([x, x])[0]
            })
            .collect()
    }

    #[test]
    fn default_is_a_fifth_up_mixed_with_dry() {
        let mut values = crate::Model::PitchShft.defaults();
        values[2] = 100.0; // wet only
        let out = render(values, 48_000.0, 220.0, 1.0);
        let found = centroid(&out[24_000..], 48_000.0, 290.0, 370.0);
        assert!((found - 330.0).abs() < 2.0, "{found}");
    }

    #[test]
    fn octave_down_and_two_up() {
        let mut values = crate::Model::PitchShft.defaults();
        values[2] = 100.0;
        values[1] = 10.0;
        values[0] = 0.0;
        let out = render(values, 44_100.0, 427.25, 1.0);
        let found = centroid(&out[22_050..], 44_100.0, 190.0, 237.25);
        assert!((found - 213.63).abs() < 2.0, "{found}");
        values[0] = 25.0;
        let out = render(values, 44_100.0, 226.7, 1.0);
        let found = centroid(&out[22_050..], 44_100.0, 846.8, 966.8);
        assert!((found - 906.8).abs() < 4.0, "{found}");
    }

    #[test]
    fn unison_passes_the_filtered_dry() {
        let mut values = crate::Model::PitchShft.defaults();
        values[0] = 12.0;
        values[3] = 25.0;
        values[1] = 10.0;
        let out = render(values, 44_100.0, 100.0, 0.5);
        let peak = out[11_025..].iter().fold(0.0f32, |a, &x| a.max(x.abs()));
        // dry 0.72 + wet 0.52 of a 0.5 sine through a 8.6 kHz one-pole.
        assert!((peak - 0.5 * (0.72 + 0.52)).abs() < 0.02, "{peak}");
    }
}
