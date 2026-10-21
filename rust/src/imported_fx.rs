//! Original KekLocator / Dayverb DSP. Heavy instances are prepared on the control
//! thread, moved into the racks, and retired with the previous preset.
use crate::{Stereo, color::ColorSlot, preset::SaturatorType};

pub struct Core {
    engine: Algorithm,
    failed: bool,
}
// Core is already boxed; keep the small 24-frame I/O buffers inline.
#[allow(clippy::large_enum_variant)]
enum Algorithm {
    SuperEcho(Box<sad_pet_superecho::Engine>),
    AlphaVerb {
        engine: Box<sad_pet_alphaverb::Nightverb>,
        input: [f32; 48],
        output: [f32; 48],
        index: usize,
        read: usize,
        available: usize,
    },
}
impl Core {
    pub fn new(kind: SaturatorType) -> Option<Box<Self>> {
        let engine = match kind {
            SaturatorType::SuperEcho => Algorithm::SuperEcho(Box::new(
                sad_pet_superecho::Engine::new().expect("Pinned SuperEcho DSP initialization"),
            )),
            SaturatorType::AlphaVerb => Algorithm::AlphaVerb {
                engine: Box::new(
                    sad_pet_alphaverb::Nightverb::new()
                        .expect("Pinned AlphaVerb DSP initialization"),
                ),
                input: [0.0; 48],
                output: [0.0; 48],
                index: 0,
                read: 0,
                available: 0,
            },
            _ => return None,
        };
        Some(Box::new(Self {
            engine,
            failed: false,
        }))
    }
    pub fn configure(&mut self, slot: ColorSlot, bpm: f32) {
        match &mut self.engine {
            Algorithm::SuperEcho(engine) => {
                let mut values = slot.super_echo_values();
                if slot.settings.super_echo_sync >= 0.5 {
                    // Firmware selects floor(time * 16.99), rather than rounding.
                    let note = (slot.settings.super_echo_note * 16.0).round();
                    values[3] = (note + 0.01) / 16.99;
                }
                self.failed |= engine
                    .set_sync(slot.settings.super_echo_sync >= 0.5, bpm.clamp(20.0, 400.0))
                    .is_err();
                for (id, value) in values.into_iter().enumerate() {
                    if engine.values[id] != value {
                        self.failed |= engine.set_parameter(id, value).is_err();
                    }
                }
            }
            Algorithm::AlphaVerb { engine, .. } => {
                for (id, mut value) in slot.alpha_verb_values().into_iter().enumerate() {
                    value = match id {
                        0 | 21 => f32::from(value >= 0.5),
                        13 => (value * 3.0).round() / 3.0,
                        _ => value,
                    };
                    if engine.value(id) != Some(value) {
                        self.failed |= engine.set_parameter(id, value).is_err();
                    }
                }
            }
        }
    }
    pub fn clear(&mut self) {
        self.failed = match &mut self.engine {
            Algorithm::SuperEcho(engine) => engine.reset().is_err(),
            Algorithm::AlphaVerb {
                engine,
                input,
                output,
                index,
                read,
                available,
            } => {
                input.fill(0.0);
                output.fill(0.0);
                *index = 0;
                *read = 0;
                *available = 0;
                engine.clear().is_err()
            }
        };
    }
    pub fn process(&mut self, frame: Stereo) -> Stereo {
        if self.failed {
            return [0.0; 2];
        }
        match &mut self.engine {
            Algorithm::SuperEcho(engine) => {
                let mut output = [[0.0; 2]];
                self.failed |= engine.process(&[frame], &mut output).is_err();
                output[0]
            }
            Algorithm::AlphaVerb {
                engine,
                input,
                output,
                index,
                read,
                available,
            } => {
                input[*index * 2..*index * 2 + 2].copy_from_slice(&frame);
                *index += 1;
                if *index == 24 {
                    self.failed |= engine.process(input).is_err();
                    output.copy_from_slice(input);
                    *index = 0;
                    *read = 0;
                    *available = 24;
                }
                if *available == 0 {
                    [0.0; 2]
                } else {
                    let result = [output[*read * 2], output[*read * 2 + 1]];
                    *read += 1;
                    *available -= 1;
                    result
                }
            }
        }
    }
}

struct Entry {
    id: u32,
    kind: SaturatorType,
    core: Option<Box<Core>>,
}
#[derive(Default)]
pub struct PreparedEffects {
    entries: Vec<Entry>,
}
impl Clone for PreparedEffects {
    fn clone(&self) -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for PreparedEffects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreparedEffects")
            .field("instances", &self.entries.len())
            .finish()
    }
}
impl PreparedEffects {
    /// Called only while preparing a preset, never from exchange_preset/process.
    pub fn prepare<'a>(&mut self, slots: impl Iterator<Item = &'a ColorSlot>) {
        self.entries = slots
            .filter(|s| matches!(s.kind, SaturatorType::SuperEcho | SaturatorType::AlphaVerb))
            .map(|s| Entry {
                id: s.id,
                kind: s.kind,
                core: Core::new(s.kind),
            })
            .collect();
    }
    pub fn inherit(&mut self, old: &mut Self) {
        for entry in &mut self.entries {
            if let Some(previous) = old
                .entries
                .iter_mut()
                .find(|e| e.id == entry.id && e.kind == entry.kind)
            {
                std::mem::swap(&mut entry.core, &mut previous.core);
            }
        }
    }
    pub fn store(&mut self, id: u32, core: &mut Option<Box<Core>>) {
        if core.is_some() {
            let entry = self
                .entries
                .iter_mut()
                .find(|e| e.id == id)
                .expect("Prepared effect identity");
            debug_assert!(entry.core.is_none());
            std::mem::swap(&mut entry.core, core);
        }
    }
    pub fn take(&mut self, id: u32) -> Option<Box<Core>> {
        self.entries
            .iter_mut()
            .find(|e| e.id == id)
            .and_then(|e| e.core.take())
    }
}
