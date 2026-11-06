//! Loading a sound into an existing timbre, independently of its host.
use crate::{preset::Preset, timbres::Kind};
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum TempoPolicy {
    /// The first timbre may load the tempo stored with its sound.
    FromPreset,
    /// Other timbres follow the session rather than their imported sound.
    Session(f32),
}

#[derive(Clone, Copy, Debug)]
pub struct Target {
    pub kind: Kind,
    pub tempo: TempoPolicy,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LoadError {
    InvalidSessionTempo,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSessionTempo => {
                f.write_str("Session tempo must be between 20 and 300 BPM")
            }
        }
    }
}

impl std::error::Error for LoadError {}

/// Resolve a decoded preset against its destination before preparing DSP.
///
/// A routing timbre keeps its role, Vocal and Sampler timbres keep their content
/// when the incoming sound omits it, and Rack module edits do the same. Explicit
/// incoming content wins; removing a module drops its settings in `sanitize`.
/// Both missing and null optional content mean inheritance at the wire boundary.
///
/// The current model is borrowed throughout. Hosts prepare the result and commit
/// it only after their update has been accepted; this function neither prepares
/// audio resources nor publishes an edit.
pub fn load_into_timbre(
    current: &Preset,
    mut incoming: Preset,
    target: Target,
) -> Result<Preset, LoadError> {
    if let TempoPolicy::Session(tempo) = target.tempo
        && (!tempo.is_finite() || !(20.0..=300.0).contains(&tempo))
    {
        return Err(LoadError::InvalidSessionTempo);
    }

    if target.kind.keeps_role() {
        incoming.control = None;
        incoming.tb303 = None;
        incoming.rack = None;
        // Mixed oscillators would otherwise create a Rack during normalization.
        // Forget their graph references as when a source is removed in the editor.
        if let Some(oscillators) = incoming.oscillators.take() {
            for slot in oscillators.slots.iter().filter(|s| s.id != 0) {
                incoming
                    .interactions
                    .remove_node(crate::interaction::Node::Osc { id: slot.id });
            }
        }
        incoming.wavetable = None;
        if incoming.bus.is_none() {
            incoming.bus = current.bus.clone();
        }
        if let Some(bus) = &mut incoming.bus {
            bus.master = target.kind == Kind::Master;
        }
    } else {
        incoming.bus = None;
    }

    let rack = incoming.rack.is_some();
    if target.kind == Kind::Vocal || rack {
        if incoming.vocal.is_none() {
            incoming.vocal = current.vocal.clone();
        }
    } else {
        incoming.vocal = None;
    }
    if target.kind == Kind::Sampler || rack {
        if incoming.sampler.is_none() {
            incoming.sampler = current.sampler.clone();
        }
    } else {
        incoming.sampler = None;
    }

    incoming.sanitize();
    if let TempoPolicy::Session(tempo) = target.tempo {
        incoming.params.bpm = tempo;
    }
    Ok(incoming)
}
