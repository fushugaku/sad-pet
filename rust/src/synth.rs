//! Synth engine timbre: the granular engine in reverse order. Rustias
//! oscillators play the engine's note voices before their filters, envelopes
//! and VCA; an optional granular stage then reads the played history.
use crate::resynth::{CrossInput, Oscillator, Waveform};
use serde::{Deserialize, Serialize};

pub const MAX_OSCILLATORS: usize = crate::resynth::MAX_OSCILLATORS;
/// Ring/Sync references 0 and 1 are Resynth's source and grain taps; a Synth
/// oscillator references another oscillator (2..5 = OSC 1..4).
pub const FIRST_REFERENCE: u8 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub count: u8,
    pub oscillators: [Oscillator; MAX_OSCILLATORS],
    pub granular: bool,
    pub buffer_seconds: f32,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            count: 1,
            oscillators: std::array::from_fn(|i| Oscillator {
                waveform: if i == 0 {
                    Waveform::Saw
                } else {
                    Waveform::default()
                },
                interaction_source: if i == 0 {
                    FIRST_REFERENCE + 1
                } else {
                    FIRST_REFERENCE
                },
                ..Oscillator::default()
            }),
            granular: false,
            buffer_seconds: 8.,
        }
    }
}
impl Settings {
    pub fn sanitize(&mut self) {
        self.count = self.count.clamp(1, MAX_OSCILLATORS as u8);
        self.buffer_seconds = if self.buffer_seconds.is_finite() {
            self.buffer_seconds.clamp(0.125, 32.)
        } else {
            8.
        };
        for o in &mut self.oscillators {
            o.sanitize();
            // A synth voice has no analysis or external audio input.
            if o.waveform == Waveform::AudioIn {
                o.waveform = Waveform::Sine;
            }
            o.interaction_source = o.interaction_source.max(FIRST_REFERENCE);
            o.cross_input = CrossInput::Analysis;
        }
    }
    pub fn history(&self) -> History {
        History {
            granular: self.granular,
            buffer_seconds: self.buffer_seconds,
        }
    }
}

/// The windowed live history shared by Drum and Synth timbres.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct History {
    pub granular: bool,
    pub buffer_seconds: f32,
}
impl History {
    pub fn buffer_frames(self, rate: u32) -> usize {
        (self.buffer_seconds.clamp(0.125, 32.0) * rate as f32).round() as usize
    }
    pub fn grain_seconds(self, size: f32) -> f32 {
        let seconds = crate::tables::grain_duration(size);
        if self.buffer_seconds >= 2.01 {
            seconds
        } else {
            0.01 + (seconds - 0.01) * ((self.buffer_seconds - 0.01).max(0.0) / 2.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn history_matches_the_drum_buffer_rules() {
        for seconds in [0.125, 0.5, 1.0, 2.0, 2.01, 8.0, 32.0] {
            let drums = crate::drums::Settings {
                buffer_seconds: seconds,
                ..Default::default()
            };
            let history = History {
                granular: true,
                buffer_seconds: seconds,
            };
            for rate in [44_100, 48_000, 96_000] {
                assert_eq!(history.buffer_frames(rate), drums.buffer_frames(rate));
            }
            for size in [0.0, 0.3, 0.7, 1.0] {
                assert_eq!(
                    history.grain_seconds(size).to_bits(),
                    drums.grain_seconds(size).to_bits()
                );
            }
        }
    }
    #[test]
    fn sanitize_removes_inputs_a_voice_cannot_hear() {
        let mut settings = Settings {
            count: 9,
            buffer_seconds: f32::NAN,
            ..Default::default()
        };
        settings.oscillators[1].waveform = Waveform::AudioIn;
        settings.oscillators[2].interaction_source = 0;
        settings.oscillators[3].cross_input = CrossInput::Grains;
        settings.sanitize();
        assert_eq!(settings.count, 4);
        assert_eq!(settings.buffer_seconds, 8.);
        assert_eq!(settings.oscillators[1].waveform, Waveform::Sine);
        assert_eq!(settings.oscillators[2].interaction_source, FIRST_REFERENCE);
        assert_eq!(settings.oscillators[3].cross_input, CrossInput::Analysis);
        let restored: Settings =
            serde_json::from_value(serde_json::to_value(settings).unwrap()).unwrap();
        assert_eq!(restored, settings);
    }
}
