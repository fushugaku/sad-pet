//! Eight additive modulation routes. Sources are sampled together at the control
//! tick; feedback between modulation generators advances on the next tick.
use crate::preset::{Destination, finite_clamp};
use serde::{Deserialize, Serialize};

pub const MAX_ROUTES: usize = 8;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    #[default]
    Lfo1,
    Lfo2,
    Lfo3,
    Env1,
    Env2,
    Env3,
    Velocity,
    KeyTrack,
    ModWheel,
    PitchBend,
    Aftertouch,
    Expression,
    RandomNote,
    Gate,
    Follower,
    Lfo4,
    Lfo5,
    Lfo6,
    Lfo7,
    Lfo8,
    Env4,
    Env5,
    Env6,
    Env7,
    Env8,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Route {
    pub source: Source,
    pub destination: Destination,
    pub depth: f32,
    pub enabled: bool,
}
impl Default for Route {
    fn default() -> Self {
        Self {
            source: Source::Lfo1,
            destination: Destination::None,
            depth: 0.25,
            enabled: true,
        }
    }
}
impl Route {
    pub fn sanitize(&mut self) {
        finite_clamp(&mut self.depth, -1.0, 1.0, 0.0);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Parameter {
    FilterEnvDepth,
    FilterKeytrack,
    Filter2EnvDepth,
    Filter2Keytrack,
    Tempo,
    InputGain,
    OutputGain,
    PitchRange,
    SampleTuneSemis,
    SampleTuneCents,
    PitchBendRange,
    MinGrains,
    FilterDrive,
    FilterMorph,
    Filter2Drive,
    Filter2Morph,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvParameter {
    Attack,
    Decay,
    Sustain,
    Release,
    Depth,
    ExtraDepth,
    Depth3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LfoParameter {
    Rate,
    Depth,
    Division,
    Depth2,
    Depth3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PerformanceParameter {
    ArpRate,
    ArpGate,
    ArpOctaves,
    SequenceRate,
    SequenceGate,
    SequenceLength,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepParameter {
    Note,
    Velocity,
}

pub fn valid(destination: Destination) -> bool {
    match destination {
        Destination::Env { index, .. } | Destination::Lfo { index, .. } => {
            (index as usize) < crate::preset::MODULATORS
        }
        Destination::Control { index } => crate::parameters::specs()
            .get(index as usize)
            .is_some_and(|s| s.can_sequence()),
        Destination::Step { index, .. } => index < 16,
        _ => true,
    }
}

pub fn add(target: &mut f32, value: f32, min: f32, max: f32) {
    *target = (*target + value * (max - min)).clamp(min, max);
}
pub fn add_log(target: &mut f32, value: f32, min: f32, max: f32) {
    let ratio = max / min;
    let unit = (*target / min).ln() / ratio.ln();
    *target = min * ratio.powf((unit + value).clamp(0.0, 1.0));
}
