//! Stage taps: every section of a timbre's chain hands its output on down the
//! chain and, at its own level, to one Bus.
use crate::bus::MAX_BUSES;
use serde::{Deserialize, Serialize};

pub const STAGES: usize = 6;

/// The sections of a timbre's chain, in the order the page shows them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage {
    /// Source FX: the source as the grain engine hears it.
    Source,
    /// The grain engine with its Dry/Wet blend; after the filters in a Synth.
    Grains,
    Resynth,
    PreFx,
    /// Filter 1 and 2 with the filters added around them.
    Filters,
    PostFx,
}
impl Stage {
    pub const ALL: [Stage; STAGES] = [
        Stage::Source,
        Stage::Grains,
        Stage::Resynth,
        Stage::PreFx,
        Stage::Filters,
        Stage::PostFx,
    ];
    pub fn key(self) -> &'static str {
        match self {
            Stage::Source => "source",
            Stage::Grains => "grains",
            Stage::Resynth => "resynth",
            Stage::PreFx => "pre_fx",
            Stage::Filters => "filters",
            Stage::PostFx => "post_fx",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Stage::Source => "Source FX",
            Stage::Grains => "Grain engine",
            Stage::Resynth => "Resynth",
            Stage::PreFx => "Pre-filter FX",
            Stage::Filters => "Filters",
            Stage::PostFx => "Post-filter FX",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Tap {
    /// Share of the stage's output that goes on down the chain; the rest is
    /// the stage's input, so at 0 the chain skips the stage.
    pub chain: f32,
    /// Share of the stage's output sent to `bus`, whatever `chain` is.
    pub send: f32,
    /// 0 = none; 1–8 = the Bus that receives `send`.
    pub bus: u8,
}
impl Default for Tap {
    fn default() -> Self {
        Self {
            chain: 1.0,
            send: 0.0,
            bus: 0,
        }
    }
}
impl Tap {
    pub fn sanitize(&mut self) {
        self.chain = if self.chain.is_finite() {
            self.chain.clamp(0.0, 1.0)
        } else {
            1.0
        };
        self.send = if self.send.is_finite() {
            self.send.clamp(0.0, 1.0)
        } else {
            0.0
        };
        self.bus = self.bus.min(MAX_BUSES as u8);
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Taps {
    pub source: Tap,
    pub grains: Tap,
    pub resynth: Tap,
    pub pre_fx: Tap,
    pub filters: Tap,
    pub post_fx: Tap,
}
impl Taps {
    pub fn get(&self, stage: Stage) -> &Tap {
        match stage {
            Stage::Source => &self.source,
            Stage::Grains => &self.grains,
            Stage::Resynth => &self.resynth,
            Stage::PreFx => &self.pre_fx,
            Stage::Filters => &self.filters,
            Stage::PostFx => &self.post_fx,
        }
    }
    pub fn get_mut(&mut self, stage: Stage) -> &mut Tap {
        match stage {
            Stage::Source => &mut self.source,
            Stage::Grains => &mut self.grains,
            Stage::Resynth => &mut self.resynth,
            Stage::PreFx => &mut self.pre_fx,
            Stage::Filters => &mut self.filters,
            Stage::PostFx => &mut self.post_fx,
        }
    }
    pub fn sanitize(&mut self) {
        for stage in Stage::ALL {
            self.get_mut(stage).sanitize();
        }
    }
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}
