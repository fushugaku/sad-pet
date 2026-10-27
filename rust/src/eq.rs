//! A Rack's EQ modules: parametric equalizers of eight bands, as many as
//! `MAX_EQS` in one rack, each standing wherever the rack puts it. A band is
//! a bell, a shelf, a cut, a notch or a band pass, on both channels, one of
//! them, or the mid or side. See `crate::dsp::eq` for how they sound.
use crate::preset::finite_clamp;
use serde::{Deserialize, Serialize};

/// EQ modules a rack can hold; each has its number, 1 to this, for good.
pub const MAX_EQS: usize = 8;
pub const BANDS: usize = 8;
pub const MIN_FREQUENCY: f32 = 20.0;
pub const MAX_FREQUENCY: f32 = 20_000.0;
/// A bell's or shelf's gain, and the output's, either way in dB.
pub const MAX_GAIN: f32 = 24.0;
pub const MIN_Q: f32 = 0.1;
pub const MAX_Q: f32 = 20.0;
/// A cut's steepness in dB per octave.
pub const SLOPES: [u8; 6] = [6, 12, 18, 24, 36, 48];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    #[default]
    Bell,
    LowShelf,
    HighShelf,
    /// A Butterworth high pass; Q adds a resonance at its corner.
    LowCut,
    /// A Butterworth low pass; Q adds a resonance at its corner.
    HighCut,
    Notch,
    BandPass,
}
impl Kind {
    /// Gain shapes the band; a cut, notch or band pass has none.
    pub fn has_gain(self) -> bool {
        matches!(self, Kind::Bell | Kind::LowShelf | Kind::HighShelf)
    }
    pub fn is_cut(self) -> bool {
        matches!(self, Kind::LowCut | Kind::HighCut)
    }
}

/// Which signal a band filters.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Placement {
    #[default]
    Stereo,
    Left,
    Right,
    /// The sum of the channels.
    Mid,
    /// Their difference.
    Side,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Band {
    pub enabled: bool,
    pub kind: Kind,
    /// Hz: a bell's centre, a shelf's midpoint, a cut's −3 dB corner.
    pub frequency: f32,
    /// dB, for a bell or shelf.
    pub gain: f32,
    pub q: f32,
    /// dB per octave, for a cut: one of `SLOPES`.
    pub slope: u8,
    pub placement: Placement,
}
impl Default for Band {
    fn default() -> Self {
        Self {
            enabled: true,
            kind: Kind::Bell,
            frequency: 1000.0,
            gain: 0.0,
            q: 1.0,
            slope: 24,
            placement: Placement::Stereo,
        }
    }
}
impl Band {
    fn sanitize(&mut self) {
        finite_clamp(&mut self.frequency, MIN_FREQUENCY, MAX_FREQUENCY, 1000.0);
        finite_clamp(&mut self.gain, -MAX_GAIN, MAX_GAIN, 0.0);
        finite_clamp(&mut self.q, MIN_Q, MAX_Q, 1.0);
        self.slope = *SLOPES
            .iter()
            .min_by_key(|&&s| s.abs_diff(self.slope))
            .expect("slopes");
    }
    /// One of `FIELDS`, as its host parameter holds it.
    pub fn field(&self, field: usize) -> f32 {
        match field {
            0 => f32::from(self.enabled),
            1 => frequency_to_unit(self.frequency),
            2 => self.gain,
            _ => q_to_unit(self.q),
        }
    }
    pub fn set_field(&mut self, field: usize, value: f32) {
        match field {
            0 => self.enabled = value >= 0.5,
            1 => self.frequency = unit_to_frequency(value),
            2 => self.gain = value,
            _ => self.q = unit_to_q(value),
        }
    }
}

/// A band's host controls in host order: id, name, min, max. Frequency and
/// Q go 0–1 along their logarithmic scales, so modulation sweeps them evenly.
pub const FIELDS: [(&str, &str, f32, f32); 4] = [
    ("on", "on", 0.0, 1.0),
    ("frequency", "frequency", 0.0, 1.0),
    ("gain", "gain", -MAX_GAIN, MAX_GAIN),
    ("q", "Q", 0.0, 1.0),
];
/// Host controls of one EQ: each band's `FIELDS`, then the output gain.
pub const CONTROLS: usize = BANDS * FIELDS.len() + 1;

pub fn frequency_to_unit(hz: f32) -> f32 {
    (hz / MIN_FREQUENCY).ln() / (MAX_FREQUENCY / MIN_FREQUENCY).ln()
}
pub fn unit_to_frequency(unit: f32) -> f32 {
    MIN_FREQUENCY * (MAX_FREQUENCY / MIN_FREQUENCY).powf(unit.clamp(0.0, 1.0))
}
pub fn q_to_unit(q: f32) -> f32 {
    (q / MIN_Q).ln() / (MAX_Q / MIN_Q).ln()
}
pub fn unit_to_q(unit: f32) -> f32 {
    MIN_Q * (MAX_Q / MIN_Q).powf(unit.clamp(0.0, 1.0))
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Equalizer {
    /// Its number in the rack, 1 to `MAX_EQS`; its module slot carries it too.
    pub id: u8,
    pub bands: [Band; BANDS],
    /// dB after every band.
    pub output: f32,
}
impl Default for Equalizer {
    /// Cuts at either end, off, and six bands at 0 dB between them, ready
    /// to be pulled up or down.
    fn default() -> Self {
        let band = |kind, frequency, q, enabled| Band {
            enabled,
            kind,
            frequency,
            q,
            ..Band::default()
        };
        Self {
            id: 1,
            bands: [
                band(Kind::LowCut, 30.0, std::f32::consts::FRAC_1_SQRT_2, false),
                band(Kind::LowShelf, 90.0, std::f32::consts::FRAC_1_SQRT_2, true),
                band(Kind::Bell, 250.0, 1.0, true),
                band(Kind::Bell, 800.0, 1.0, true),
                band(Kind::Bell, 2500.0, 1.0, true),
                band(Kind::Bell, 6000.0, 1.0, true),
                band(
                    Kind::HighShelf,
                    10_000.0,
                    std::f32::consts::FRAC_1_SQRT_2,
                    true,
                ),
                band(
                    Kind::HighCut,
                    18_000.0,
                    std::f32::consts::FRAC_1_SQRT_2,
                    false,
                ),
            ],
            output: 0.0,
        }
    }
}
impl Equalizer {
    pub fn numbered(id: u8) -> Self {
        Self {
            id,
            ..Self::default()
        }
    }
    pub fn sanitize(&mut self) {
        for band in &mut self.bands {
            band.sanitize();
        }
        finite_clamp(&mut self.output, -MAX_GAIN, MAX_GAIN, 0.0);
    }
    /// One of `CONTROLS`, as its host parameter holds it.
    pub fn control(&self, control: usize) -> f32 {
        match self.bands.get(control / FIELDS.len()) {
            Some(band) => band.field(control % FIELDS.len()),
            None => self.output,
        }
    }
    pub fn set_control(&mut self, control: usize, value: f32) {
        match self.bands.get_mut(control / FIELDS.len()) {
            Some(band) => band.set_field(control % FIELDS.len(), value),
            None => self.output = value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_controls_round_trip_along_their_scales() {
        let mut eq = Equalizer::default();
        eq.set_control(FIELDS.len() * 2 + 1, 0.5);
        let middle = (MIN_FREQUENCY * MAX_FREQUENCY).sqrt();
        assert!((eq.bands[2].frequency - middle).abs() < 0.01 * middle);
        assert!((eq.control(FIELDS.len() * 2 + 1) - 0.5).abs() < 1e-5);
        eq.set_control(3, q_to_unit(4.0));
        assert!((eq.bands[0].q - 4.0).abs() < 1e-3);
        eq.set_control(CONTROLS - 1, -6.0);
        assert_eq!(eq.output, -6.0);
        eq.set_control(FIELDS.len() * 7, 1.0);
        assert!(eq.bands[7].enabled);
    }

    #[test]
    fn sanitize_clamps_and_picks_the_nearest_slope() {
        let mut eq = Equalizer::default();
        eq.bands[0].frequency = 1e9;
        eq.bands[1].gain = f32::NAN;
        eq.bands[2].q = 0.0;
        eq.bands[3].slope = 30;
        eq.output = 99.0;
        eq.sanitize();
        assert_eq!(eq.bands[0].frequency, MAX_FREQUENCY);
        assert_eq!(eq.bands[1].gain, 0.0);
        assert_eq!(eq.bands[2].q, MIN_Q);
        assert!([24, 36].contains(&eq.bands[3].slope));
        assert_eq!(eq.output, MAX_GAIN);
    }
}
