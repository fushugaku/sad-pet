//! What CoronaCho (CRN_CHO.ZDL) and CoronaTri (CRN_TRI.ZDL) share: the two
//! ROM front biquads every input passes (dry included), the TONE editor (a
//! first-order high shelf at 22 kHz whose cut moves its pole down to
//! `22 kHz * G`, i.e. a gentle low-pass with a floor), the DRY switch
//! (`KillDry_edit`: dry gain ramps to 1 when DRY is on, to 0 when off) and the
//! output sum `dry * front + FxLVL * tone(wet)`.
use super::common::{Filter, Kind, design, rom, table};
use crate::dsp::Coefficients;
use crate::zoom::{DEFAULT_STEP, Ramp};

/// CRN_CHO c36..c45 / CRN_TRI c47..c56: peaking -0.5 dB at 2.8 kHz and
/// -0.5 dB at 10 kHz (Q ~0.78), applied to both channels before everything.
pub fn front_filters(sample_rate: f32) -> [Coefficients; 2] {
    [
        rom(
            0.988_557_4,
            -1.465_968_5,
            0.602_333_96,
            1.465_968_5,
            -0.590_891_36,
            sample_rate,
        ),
        rom(
            0.977_861_2,
            -0.175_855_47,
            0.230_607_84,
            0.175_855_47,
            -0.208_469_06,
            sample_rate,
        ),
    ]
}

/// One channel's front pair.
#[derive(Debug, Clone, Copy)]
pub struct Front([Filter; 2]);
impl Front {
    pub fn new(coefficients: [Coefficients; 2]) -> Self {
        let mut filters = [Filter::default(); 2];
        filters[0].set(coefficients[0]);
        filters[1].set(coefficients[1]);
        Self(filters)
    }
    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let [a, b] = &mut self.0;
        b.process(a.process(x))
    }
    pub fn clear(&mut self) {
        self.0.iter_mut().for_each(Filter::clear);
    }
}

/// Output stage: tone shelves, FxLVL and DRY gains.
#[derive(Debug, Clone)]
pub struct Output {
    sample_rate: f32,
    tone: [Filter; 2],
    fx: Ramp,
    dry: Ramp,
}

impl Output {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            tone: [Filter::default(); 2],
            fx: Ramp::new(0.0, DEFAULT_STEP, sample_rate),
            dry: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
        }
    }

    /// TONE through the model's 11-point dB table.
    pub fn set_tone(&mut self, decibels: &[f32; 11], value: f32) {
        let gain = table(decibels, value, 100.0);
        let c = design(Kind::HighShelf1, self.sample_rate, 22_000.0, gain, 1.0);
        self.tone.iter_mut().for_each(|f| f.set(c));
    }

    pub fn set_levels(&mut self, fx: f32, dry_on: bool, jump: bool) {
        let dry = if dry_on { 1.0 } else { 0.0 };
        if jump {
            self.fx.jump(fx);
            self.dry.jump(dry);
        } else {
            self.fx.set(fx);
            self.dry.set(dry);
        }
    }

    #[inline]
    pub fn mix(&mut self, front: [f32; 2], wet: [f32; 2]) -> [f32; 2] {
        let fx = self.fx.tick();
        let dry = self.dry.tick();
        [
            dry * front[0] + fx * self.tone[0].process(wet[0]),
            dry * front[1] + fx * self.tone[1].process(wet[1]),
        ]
    }

    pub fn clear(&mut self) {
        self.tone.iter_mut().for_each(Filter::clear);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsp::gain_to_db;

    #[test]
    fn front_filters_are_half_db_dips() {
        let [a, b] = front_filters(48_000.0);
        assert!(gain_to_db(a.magnitude(48_000.0, 100.0)).abs() < 0.02);
        assert!((gain_to_db(a.magnitude(48_000.0, 2805.0)) + 0.5).abs() < 0.03);
        assert!((gain_to_db(b.magnitude(48_000.0, 10_000.0)) + 0.5).abs() < 0.05);
    }

    #[test]
    fn tone_cut_is_a_low_pass_with_a_floor() {
        let mut output = Output::new(44_100.0);
        output.set_tone(&[-60.0; 11], 0.0);
        let c = output.tone[0].biquad.coefficients;
        // Pole at 22 kHz * 0.001 = 22 Hz: -20 dB a decade above it.
        assert!(gain_to_db(c.magnitude(44_100.0, 2200.0)) < -38.0);
        assert!(gain_to_db(c.magnitude(44_100.0, 5.0)).abs() < 0.5);
    }
}
