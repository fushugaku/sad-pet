//! RADIAS Drive/WS port from radias-emulator's shared Rustias synthesis source.
//! `kernels` and `controller` retain its fixed-point sample and DEPTH arithmetic.
//! This stereo FX adapter supplies SAD PET's latest MIDI pitch, host sample rate,
//! four-sample control slew and firmware-free sub-oscillator edge slopes.
pub mod controller;
mod fixed;
pub mod kernels;

use crate::Stereo;
use controller::{ShaperControl, WaveshaperType as NativeType};
use kernels::{
    DriveCoefficients, ShaperCoefficients, ShaperSignal, ShaperTables, SubOscillatorCoefficients,
    SubOscillatorWaveform, Waveshaper,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sample(pub i32);
#[derive(Clone, Copy, Debug)]
pub struct PhaseIncrement(pub u32);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WaveshaperMode {
    #[default]
    Drive,
    HardClip,
    Decimator,
    OctSaw,
    MultiTriangle,
    MultiSine,
    SubSaw,
    SubSquare,
    SubTriangle,
    SubSine,
    Pickup,
    LevelBoost,
}
impl WaveshaperMode {
    pub const ALL: [Self; 12] = [
        Self::Drive,
        Self::HardClip,
        Self::Decimator,
        Self::OctSaw,
        Self::MultiTriangle,
        Self::MultiSine,
        Self::SubSaw,
        Self::SubSquare,
        Self::SubTriangle,
        Self::SubSine,
        Self::Pickup,
        Self::LevelBoost,
    ];

    fn coefficients(self, depth: f32, pitch: u16) -> ShaperCoefficients {
        // Retain fractional DEPTH modulation through the original manual offset.
        let raw = (depth.clamp(0.0, 1.0) * 127.0 * 256.0).round() as u16;
        let control = ShaperControl {
            depth: (raw / 256) as u8,
            manual_offset: (raw % 256) as i16,
            modulation: 0,
        };
        let native = match self {
            Self::Drive => {
                return ShaperCoefficients::Drive(DriveCoefficients {
                    depth: control.drive_depth(),
                    normalization: 23920,
                    feedback_gain: 0,
                    threshold: 19660,
                    curves: [26214, -26214],
                });
            }
            Self::HardClip => NativeType::HardClip,
            Self::Decimator => NativeType::Decimator,
            Self::OctSaw => NativeType::OctSaw,
            Self::MultiTriangle => NativeType::MultiTriangle,
            Self::MultiSine => NativeType::MultiSine,
            Self::SubSaw => NativeType::SubSaw,
            Self::SubSquare => NativeType::SubSquare,
            Self::SubTriangle => NativeType::SubTriangle,
            Self::SubSine => NativeType::SubSine,
            Self::Pickup => NativeType::Pickup,
            Self::LevelBoost => NativeType::LevelBoost,
        };
        let depth = control.waveshaper_depth(native);
        match self {
            Self::Drive => unreachable!(),
            Self::HardClip => ShaperCoefficients::HardClip { depth },
            Self::Decimator => ShaperCoefficients::Decimator { depth },
            Self::OctSaw => ShaperCoefficients::OctSaw { depth },
            Self::MultiTriangle => ShaperCoefficients::MultiTriangle { depth },
            Self::MultiSine => ShaperCoefficients::MultiSine { depth },
            Self::Pickup => ShaperCoefficients::Pickup {
                depth,
                pitch_current: pitch as i16,
            },
            Self::LevelBoost => ShaperCoefficients::LevelBoost { depth },
            _ => ShaperCoefficients::SubOscillator(SubOscillatorCoefficients {
                waveform: match self {
                    Self::SubSaw => SubOscillatorWaveform::Saw,
                    Self::SubSquare => SubOscillatorWaveform::Square,
                    Self::SubTriangle => SubOscillatorWaveform::Triangle,
                    _ => SubOscillatorWaveform::Sine,
                },
                depth,
                target_depth: depth,
                gain_current: 0,
            }),
        }
    }
}

pub struct RadiasWaveshaper {
    states: [Waveshaper; 2],
    tables: ShaperTables,
    mode: WaveshaperMode,
    current: ShaperCoefficients,
    target: ShaperCoefficients,
    pitch: u16,
    increment: PhaseIncrement,
    sample_rate: f64,
    frame: u8,
}
impl RadiasWaveshaper {
    pub fn new(sample_rate: u32) -> Self {
        let mut result = Self {
            states: [Waveshaper::default(); 2],
            tables: ShaperTables {
                sub_edges: std::array::from_fn(|i| {
                    16384usize.checked_div(i).unwrap_or(32767) as i16
                }),
            },
            mode: WaveshaperMode::Drive,
            current: WaveshaperMode::Drive.coefficients(0.5, 60 * 256),
            target: WaveshaperMode::Drive.coefficients(0.5, 60 * 256),
            pitch: 60 * 256,
            increment: PhaseIncrement(0),
            sample_rate: sample_rate as f64,
            frame: 0,
        };
        result.set_pitch(60.0);
        result
    }
    pub fn clear(&mut self) {
        self.states = [Waveshaper::default(); 2];
        self.current = self.target;
        if !matches!(self.current, ShaperCoefficients::Pickup { .. }) {
            self.current.set_gain_current(0);
        }
        self.frame = 0;
    }
    pub fn configure(&mut self, mode: WaveshaperMode, depth: f32) {
        self.target = mode.coefficients(depth, self.pitch);
        if self.mode != mode {
            self.mode = mode;
            self.clear();
        }
        if let (
            ShaperCoefficients::SubOscillator(current),
            ShaperCoefficients::SubOscillator(target),
        ) = (&mut self.current, self.target)
        {
            current.target_depth = target.target_depth;
        }
    }
    pub(super) fn pitch_code(&self) -> u16 {
        self.pitch
    }
    pub fn set_pitch(&mut self, note: f32) {
        let note = note.clamp(0.0, 127.996);
        self.pitch = (note * 256.0).round().clamp(0.0, 32767.0) as u16;
        let hz = 440.0 * 2.0_f64.powf((note as f64 - 69.0) / 12.0);
        self.increment = PhaseIncrement(
            (hz * 4294967296.0 / self.sample_rate)
                .round()
                .clamp(0.0, i32::MAX as f64) as u32,
        );
    }
    pub fn process(&mut self, input: Stereo) -> Stereo {
        self.process_with_voice_gain(input, 1.0)
    }
    pub fn process_with_voice_gain(&mut self, input: Stereo, voice_gain: f32) -> Stereo {
        let mut coefficients = self.current;
        if let ShaperCoefficients::SubOscillator(c) = &mut coefficients {
            // A post-filter FX sits after the voice VCAs. Keep its added SubOSC
            // under the same note envelope instead of producing a permanent tone.
            c.gain_current = (c.gain_current as f32 * voice_gain.clamp(0.0, 1.0)).round() as i16;
        }
        let result = std::array::from_fn(|channel| {
            let sample = if input[channel].is_finite() {
                (input[channel].clamp(-1.0, 1.0) as f64 * 2147483648.0)
                    .clamp(i32::MIN as f64, i32::MAX as f64) as i32
            } else {
                0
            };
            self.states[channel]
                .process(
                    ShaperSignal {
                        input: Sample(sample),
                        primary_pitch_code: self.pitch,
                        primary_increment: self.increment,
                    },
                    coefficients,
                    &self.tables,
                )
                .0 as f32
                / 2147483648.0
        });
        // Same four-frame publication and staged Q15 rounding as VoiceRenderer.
        if self.frame & 3 == 3 {
            let gain = self.current.gain_target(self.pitch);
            self.current
                .set_depth(slew(self.current.depth(), self.target.depth()));
            if let (Some(current), Some(target)) = (self.current.gain_current(), gain) {
                self.current.set_gain_current(slew(current, target));
            }
        }
        self.frame = self.frame.wrapping_add(1);
        result
    }
}
fn slew(current: i16, target: i16) -> i16 {
    (fixed::saturate(2 * (target as i64 * 0x1d4 + current as i64 * 0x7e2d)) >> 16) as i16
}
