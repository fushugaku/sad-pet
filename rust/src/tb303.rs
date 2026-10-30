//! A monophonic 303 voice and its sixteen-step pattern. DSP and fidelity
//! boundaries are documented in docs/TB303.md.
use serde::{Deserialize, Serialize};

pub const STEPS: usize = 16;
pub const FIELDS: [&str; 9] = [
    "tuning",
    "cutoff",
    "resonance",
    "env_mod",
    "decay",
    "accent",
    "waveform",
    "level",
    "transpose",
];

pub fn range(field: usize) -> (f32, f32) {
    match field {
        0 => (-7.0, 7.0),
        8 => (-48.0, 48.0),
        _ => (0.0, 1.0),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Controls {
    /// Original tuning range, in semitones.
    pub tuning: f32,
    pub cutoff: f32,
    pub resonance: f32,
    pub env_mod: f32,
    pub decay: f32,
    pub accent: f32,
    /// 0 = saw, 1 = square; numeric so it can use host automation.
    pub waveform: f32,
    pub level: f32,
    pub transpose: f32,
}
impl Default for Controls {
    fn default() -> Self {
        Self {
            tuning: 0.0,
            cutoff: 0.45,
            resonance: 0.72,
            env_mod: 0.55,
            decay: 0.35,
            accent: 0.65,
            waveform: 0.0,
            level: 1.0,
            transpose: 0.0,
        }
    }
}
impl Controls {
    pub fn get(&self, field: usize) -> f32 {
        match field {
            0 => self.tuning,
            1 => self.cutoff,
            2 => self.resonance,
            3 => self.env_mod,
            4 => self.decay,
            5 => self.accent,
            6 => self.waveform,
            7 => self.level,
            _ => self.transpose,
        }
    }
    pub fn set(&mut self, field: usize, value: f32) {
        match field {
            0 => self.tuning = value,
            1 => self.cutoff = value,
            2 => self.resonance = value,
            3 => self.env_mod = value,
            4 => self.decay = value,
            5 => self.accent = value,
            6 => self.waveform = value.round(),
            7 => self.level = value,
            _ => self.transpose = value,
        }
    }
    pub fn sanitize(&mut self) {
        let default = Self::default();
        for field in 0..FIELDS.len() {
            let value = self.get(field);
            let (min, max) = range(field);
            self.set(
                field,
                if value.is_finite() {
                    value.clamp(min, max)
                } else {
                    default.get(field)
                },
            );
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Step {
    pub note: u8,
    pub gate: bool,
    pub accent: bool,
    /// Connects this step to the following note; the pitch slews on arrival.
    pub slide: bool,
}
impl Default for Step {
    fn default() -> Self {
        Self {
            note: 36,
            gate: true,
            accent: false,
            slide: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    #[serde(flatten)]
    pub controls: Controls,
    pub pattern: bool,
    pub length: u8,
    pub steps: [Step; STEPS],
    /// Optional RADIAS layers mixed before the mono ladder and VCA.
    pub count: u8,
    pub oscillators: [crate::resynth::Oscillator; crate::synth::MAX_OSCILLATORS],
}
impl Default for Settings {
    fn default() -> Self {
        let notes = [
            36, 36, 48, 36, 43, 36, 39, 46, 36, 48, 43, 39, 36, 43, 48, 34,
        ];
        Self {
            controls: Controls::default(),
            pattern: true,
            length: 16,
            steps: std::array::from_fn(|i| Step {
                note: notes[i],
                gate: !matches!(i, 3 | 7 | 11),
                accent: matches!(i, 0 | 6 | 10 | 14),
                slide: matches!(i, 1 | 8 | 12),
            }),
            count: 0,
            oscillators: crate::synth::Settings::default().oscillators.map(|mut o| {
                o.interaction_source = 0;
                o
            }),
        }
    }
}
impl Settings {
    pub fn sanitize(&mut self) {
        self.controls.sanitize();
        self.length = self.length.clamp(1, STEPS as u8);
        for step in &mut self.steps {
            step.note = step.note.min(127);
        }
        self.count = self.count.min(crate::synth::MAX_OSCILLATORS as u8);
        for osc in &mut self.oscillators {
            osc.sanitize();
            osc.model = crate::resynth::Model::Radias;
            osc.pan = 0.5;
            if osc.interaction_source < crate::synth::FIRST_REFERENCE {
                osc.interaction_source = 0;
            }
            if osc.cross_input == crate::resynth::CrossInput::Grains {
                osc.cross_input = crate::resynth::CrossInput::Source;
            }
            if matches!(
                osc.waveform,
                crate::resynth::Waveform::Pcm | crate::resynth::Waveform::AudioIn
            ) {
                osc.waveform = crate::resynth::Waveform::Sine;
            }
        }
    }
    pub fn radias(&self) -> crate::synth::Settings {
        crate::synth::Settings {
            count: self.count,
            oscillators: self.oscillators,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct View {
    pub step: Option<u8>,
    pub gate: bool,
    pub accent: bool,
    pub slide: bool,
    pub note: f32,
    pub envelope: f32,
    pub accent_sweep: f32,
}

#[derive(Default, Debug)]
pub struct Prepared(pub Option<Box<crate::dsp::tb303::Runtime>>);
impl Clone for Prepared {
    fn clone(&self) -> Self {
        Self::default()
    }
}
impl Prepared {
    pub fn prepare(&mut self, settings: Option<&Settings>, rate: u32) {
        self.0 = settings.map(|s| Box::new(crate::dsp::tb303::Runtime::new(rate, s.controls)));
    }
}
