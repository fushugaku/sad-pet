//! Four independently configured Rustias carriers, excited by sixteen bands.
use serde::{Deserialize, Serialize};

pub const BANDS: usize = 16;
pub const MAX_OSCILLATORS: usize = 4;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Waveform {
    Saw,
    Pulse,
    Triangle,
    #[default]
    Sine,
    Formant,
    Noise,
    AudioIn,
    Pcm,
}
impl Waveform {
    pub const ALL: [Self; 8] = [
        Self::Saw,
        Self::Pulse,
        Self::Triangle,
        Self::Sine,
        Self::Formant,
        Self::Noise,
        Self::AudioIn,
        Self::Pcm,
    ];
    pub fn code(self) -> u8 {
        self as u8
    }
    pub fn analog(self) -> bool {
        self.code() < 4
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    #[default]
    Wave,
    Cross,
    Unison,
    Vpm,
}
impl Mode {
    pub const ALL: [Self; 4] = [Self::Wave, Self::Cross, Self::Unison, Self::Vpm];
    pub fn code(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Input {
    #[default]
    Source,
    Grains,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Oscillator {
    pub enabled: bool,
    pub waveform: Waveform,
    pub mode: Mode,
    pub control1: f32,
    pub control2: f32,
    pub transpose: f32,
    pub fine: f32,
    pub level: f32,
    pub pan: f32,
    /// 0 = analysis input; 1..4 = another carrier (feedback uses its previous sample).
    pub cross_source: u8,
}
impl Default for Oscillator {
    fn default() -> Self {
        Self {
            enabled: true,
            waveform: Waveform::Sine,
            mode: Mode::Wave,
            control1: 0.0,
            control2: 0.0,
            transpose: 0.0,
            fine: 0.0,
            level: 0.7,
            pan: 0.5,
            cross_source: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub enabled: bool,
    pub input: Input,
    pub mix: f32,
    pub sensitivity: f32,
    pub attack: f32,
    pub release: f32,
    pub threshold: f32,
    pub count: u8,
    pub oscillators: [Oscillator; MAX_OSCILLATORS],
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: false,
            input: Input::Source,
            mix: 0.8,
            sensitivity: 0.5,
            attack: 0.15,
            release: 0.3,
            threshold: 0.2,
            count: 1,
            oscillators: [Oscillator::default(); MAX_OSCILLATORS],
        }
    }
}
fn finite(value: &mut f32, low: f32, high: f32, fallback: f32) {
    *value = if value.is_finite() {
        value.clamp(low, high)
    } else {
        fallback
    };
}
impl Settings {
    pub fn sanitize(&mut self) {
        let d = Self::default();
        finite(&mut self.mix, 0.0, 1.0, d.mix);
        finite(&mut self.sensitivity, 0.0, 1.0, d.sensitivity);
        finite(&mut self.attack, 0.0, 1.0, d.attack);
        finite(&mut self.release, 0.0, 1.0, d.release);
        finite(&mut self.threshold, 0.0, 1.0, d.threshold);
        self.count = self.count.clamp(1, MAX_OSCILLATORS as u8);
        for o in &mut self.oscillators {
            let d = Oscillator::default();
            finite(&mut o.control1, 0.0, 1.0, d.control1);
            finite(&mut o.control2, 0.0, 1.0, d.control2);
            finite(&mut o.transpose, -48.0, 48.0, d.transpose);
            finite(&mut o.fine, -100.0, 100.0, d.fine);
            finite(&mut o.level, 0.0, 1.0, d.level);
            finite(&mut o.pan, 0.0, 1.0, d.pan);
            o.cross_source = o.cross_source.min(4);
            if !o.waveform.analog() {
                o.mode = Mode::Wave;
            }
        }
    }
    pub fn sensitivity_gain(self) -> f32 {
        10.0_f32.powf((self.sensitivity * 48.0 - 24.0) / 20.0)
    }
    pub fn attack_seconds(self) -> f32 {
        0.001 + self.attack.powi(3) * 0.5
    }
    pub fn release_seconds(self) -> f32 {
        0.005 + self.release.powi(3) * 2.0
    }
    pub fn threshold_gain(self) -> f32 {
        10.0_f32.powf((-84.0 + self.threshold * 60.0) / 20.0)
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Meter {
    pub bands: [f32; BANDS],
    pub frequencies: [f32; BANDS],
}
