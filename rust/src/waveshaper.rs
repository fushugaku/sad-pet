//! Persisted controls for the extended waveshaper.

use crate::color::WaveshaperMode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Quality {
    #[default]
    Legacy,
    Clean2x,
    Clean4x,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Curve {
    #[default]
    Radias,
    SoftClip,
    Tanh,
    Diode,
    Rectify,
    Custom,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Options {
    pub quality: Quality,
    pub curve: Curve,
    pub curve_b: Curve,
    pub morph: f32,
    pub input: f32,
    pub output: f32,
    pub knee: f32,
    pub bias: f32,
    pub pre_tone: f32,
    pub post_tone: f32,
    pub bass: f32,
    pub bass_frequency: f32,
    pub envelope: f32,
    pub attack: f32,
    pub release: f32,
    pub dc_block: bool,
    pub auto_gain: bool,
    pub points: [f32; 17],
}

impl Default for Options {
    fn default() -> Self {
        Self {
            quality: Quality::Legacy,
            curve: Curve::Radias,
            curve_b: Curve::SoftClip,
            morph: 0.0,
            input: 0.5,
            output: 0.5,
            knee: 0.5,
            bias: 0.5,
            pre_tone: 1.0,
            post_tone: 1.0,
            bass: 0.0,
            bass_frequency: 0.5,
            envelope: 0.5,
            attack: 0.2,
            release: 0.5,
            dc_block: false,
            auto_gain: false,
            points: std::array::from_fn(|i| i as f32 / 8.0 - 1.0),
        }
    }
}

impl Options {
    /// Clamp persisted or host-provided controls to their documented ranges.
    pub fn sanitize(mut self) -> Self {
        self.morph = finite_unit(self.morph, 0.0);
        self.input = finite_unit(self.input, 0.5);
        self.output = finite_unit(self.output, 0.5);
        self.knee = finite_unit(self.knee, 0.5);
        self.bias = finite_unit(self.bias, 0.5);
        self.pre_tone = finite_unit(self.pre_tone, 1.0);
        self.post_tone = finite_unit(self.post_tone, 1.0);
        self.bass = finite_unit(self.bass, 0.0);
        self.bass_frequency = finite_unit(self.bass_frequency, 0.5);
        self.envelope = finite_unit(self.envelope, 0.5);
        self.attack = finite_unit(self.attack, 0.2);
        self.release = finite_unit(self.release, 0.5);
        for point in &mut self.points {
            *point = if point.is_finite() {
                point.clamp(-1.0, 1.0)
            } else {
                0.0
            };
        }
        self
    }

    pub fn is_legacy_default(self) -> bool {
        self == Self::default()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Stage {
    pub enabled: bool,
    pub mode: WaveshaperMode,
    pub depth: f32,
    pub mix: f32,
    pub options: Options,
}

impl Default for Stage {
    fn default() -> Self {
        Self {
            enabled: false,
            mode: WaveshaperMode::Drive,
            depth: 0.5,
            mix: 1.0,
            options: Options::default(),
        }
    }
}

impl Stage {
    pub fn sanitize(mut self) -> Self {
        self.depth = finite_unit(self.depth, 0.5);
        self.mix = finite_unit(self.mix, 1.0);
        self.options = self.options.sanitize();
        self
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FilterShapers {
    pub before: Stage,
    pub after: Stage,
}

impl FilterShapers {
    pub fn sanitize(mut self) -> Self {
        self.before = self.before.sanitize();
        self.after = self.after.sanitize();
        self
    }
}

fn finite_unit(value: f32, default: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 1.0)
    } else {
        default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_round_trip_and_partial_json_keeps_neutral_values() {
        let defaults = Options::default();
        let encoded = serde_json::to_string(&defaults).unwrap();
        let decoded: Options = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, defaults);

        let partial: Options = serde_json::from_str(r#"{"quality":"clean4x"}"#).unwrap();
        assert_eq!(partial.quality, Quality::Clean4x);
        assert_eq!(partial.input, 0.5);
        assert_eq!(partial.points, defaults.points);
    }

    #[test]
    fn sanitize_clamps_controls_and_repairs_non_finite_custom_points() {
        let options = Options {
            input: f32::INFINITY,
            bass: -4.0,
            points: [
                f32::NAN,
                2.0,
                -2.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            ],
            ..Options::default()
        }
        .sanitize();
        assert_eq!(options.input, 0.5);
        assert_eq!(options.bass, 0.0);
        assert_eq!(options.points[0], 0.0);
        assert_eq!(options.points[1], 1.0);
        assert_eq!(options.points[2], -1.0);
    }
}
