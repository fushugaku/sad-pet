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
    scope: u8,
    rate: u32,
    core: Option<Box<Core>>,
    buffers: Buffers,
}

/// Large delay histories exist only for slots which use them. Like imported
/// firmware instances, they are prepared and retired by the control owner.
#[derive(Default)]
pub(crate) struct Buffers {
    pub reverb: Option<crate::dsp::effects::Storage>,
    pub delay: Option<Vec<Stereo>>,
}
impl Buffers {
    fn new(kind: SaturatorType, rate: u32) -> Self {
        Self {
            reverb: (kind == SaturatorType::Reverb)
                .then(|| crate::dsp::effects::Effects::new(rate).take_storage()),
            delay: matches!(kind, SaturatorType::LcrDelay | SaturatorType::ModDelay)
                .then(|| crate::dsp::delay::RadiasDelay::new(rate).take_storage()),
        }
    }
    fn is_empty(&self) -> bool {
        self.reverb.is_none() && self.delay.is_none()
    }
}
#[derive(Default)]
pub struct PreparedEffects {
    entries: Vec<Entry>,
    rate: Option<u32>,
}
impl Clone for PreparedEffects {
    fn clone(&self) -> Self {
        // Keep identities so an edit to a sequencer-only clone can inherit its
        // existing histories without preparing or allocating audio buffers.
        Self {
            rate: self.rate,
            entries: self
                .entries
                .iter()
                .map(|entry| Entry {
                    id: entry.id,
                    kind: entry.kind,
                    scope: entry.scope,
                    rate: entry.rate,
                    core: None,
                    buffers: Buffers::default(),
                })
                .collect(),
        }
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
        self.prepare_for(slots, 48_000);
    }
    pub fn prepare_for<'a>(&mut self, slots: impl Iterator<Item = &'a ColorSlot>, rate: u32) {
        self.prepare_scoped(slots.map(|slot| (0, slot)), rate, |_, _| (false, false));
    }
    pub(crate) fn prepare_racks(&mut self, racks: [&[ColorSlot]; 3], rate: u32) {
        self.prepare_scoped(
            racks
                .into_iter()
                .enumerate()
                .flat_map(|(scope, slots)| slots.iter().map(move |slot| (scope as u8, slot))),
            rate,
            |_, _| (false, false),
        );
    }
    pub(crate) fn prepare_racks_reusing(
        &mut self,
        racks: [&[ColorSlot]; 3],
        rate: u32,
        previous: &Self,
        previous_racks: [&[ColorSlot]; 3],
    ) {
        self.prepare_scoped(
            racks
                .into_iter()
                .enumerate()
                .flat_map(|(scope, slots)| slots.iter().map(move |s| (scope as u8, s))),
            rate,
            |scope, slot| {
                let old = previous.entries.iter().find(|e| {
                    e.id == slot.id
                        && e.kind == slot.kind
                        && e.rate == rate
                        && previous_racks[e.scope as usize]
                            .iter()
                            .any(|s| s.id == slot.id && s.kind == slot.kind)
                });
                (old.is_some(), old.is_some_and(|e| e.scope == scope))
            },
        );
    }
    fn prepare_scoped<'a>(
        &mut self,
        slots: impl Iterator<Item = (u8, &'a ColorSlot)>,
        rate: u32,
        mut reuse: impl FnMut(u8, &ColorSlot) -> (bool, bool),
    ) {
        self.rate = Some(rate);
        self.entries = slots
            .map(|(scope, s)| {
                let (core, buffers) = reuse(scope, s);
                Entry {
                    id: s.id,
                    kind: s.kind,
                    scope,
                    rate,
                    core: if core { None } else { Core::new(s.kind) },
                    buffers: if buffers {
                        Buffers::default()
                    } else {
                        Buffers::new(s.kind, rate)
                    },
                }
            })
            .collect();
    }
    pub fn inherit(&mut self, old: &mut Self) {
        if self.rate.is_none() {
            self.rate = old.rate;
        }
        for entry in &mut self.entries {
            if let Some(previous) = old
                .entries
                .iter_mut()
                .find(|e| e.id == entry.id && e.kind == entry.kind && e.rate == entry.rate)
            {
                std::mem::swap(&mut entry.core, &mut previous.core);
                // Legacy racks reset a newly placed processor. Only histories
                // whose identity stays in the same rack are retained.
                if entry.scope == previous.scope {
                    std::mem::swap(&mut entry.buffers, &mut previous.buffers);
                }
            }
        }
    }
    pub(crate) fn inherit_matching<'a>(
        &mut self,
        old: &mut Self,
        slots: impl Iterator<Item = &'a ColorSlot>,
    ) {
        // A controller may enqueue an unprepared copy for a metadata-only
        // change. Reuse the existing owner and its storage without allocating
        // or retiring the histories on the render thread.
        let mut slots = slots.peekable();
        if self.entries.is_empty()
            && slots.peek().is_some()
            && slots.all(|slot| {
                old.entries
                    .iter()
                    .any(|e| e.id == slot.id && e.kind == slot.kind)
            })
        {
            self.rate = old.rate;
            std::mem::swap(&mut self.entries, &mut old.entries);
        } else {
            self.inherit(old);
        }
    }
    pub(crate) fn sample_rate(&self) -> Option<u32> {
        self.rate
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
    pub(crate) fn store_buffers(&mut self, id: u32, buffers: &mut Buffers) {
        if !buffers.is_empty() {
            let entry = self
                .entries
                .iter_mut()
                .find(|e| e.id == id)
                .expect("Prepared effect buffer identity");
            debug_assert!(entry.buffers.is_empty());
            std::mem::swap(&mut entry.buffers, buffers);
        }
    }
    pub(crate) fn take_buffers(&mut self, id: u32) -> Buffers {
        self.entries
            .iter_mut()
            .find(|e| e.id == id)
            .map(|e| std::mem::take(&mut e.buffers))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn moving_a_legacy_effect_to_another_rack_uses_fresh_history() {
        let slots = [ColorSlot {
            id: 7,
            kind: SaturatorType::Reverb,
            ..Default::default()
        }];
        let mut old = PreparedEffects::default();
        old.prepare_racks([&slots, &[], &[]], 8000);
        old.entries[0].buffers.reverb.as_mut().unwrap()[0][0] = [0.75, -0.25];
        let mut moved = PreparedEffects::default();
        moved.prepare_racks([&[], &slots, &[]], 8000);
        moved.inherit(&mut old);
        assert_eq!(
            moved.entries[0].buffers.reverb.as_ref().unwrap()[0][0],
            [0.; 2]
        );
        assert_eq!(
            old.entries[0].buffers.reverb.as_ref().unwrap()[0][0],
            [0.75, -0.25]
        );
    }
    #[test]
    fn removing_all_effects_retires_histories_with_the_previous_preset() {
        let slots = [ColorSlot {
            id: 7,
            kind: SaturatorType::Reverb,
            ..Default::default()
        }];
        let mut old = PreparedEffects::default();
        old.prepare_for(slots.iter(), 8000);
        let mut empty = PreparedEffects::default();
        empty.inherit_matching(&mut old, [].iter());
        assert!(empty.entries.is_empty());
        assert!(old.entries[0].buffers.reverb.is_some());
    }
    #[test]
    fn alphaverb_mode_labels_match_original_compressor_and_gate_flags() {
        let manifest: serde_json::Value =
            serde_json::from_str(include_str!("../tests/fixtures/erika-fx/manifest.json")).unwrap();
        let mut core = Core::new(SaturatorType::AlphaVerb).unwrap();
        let mut slot = ColorSlot {
            kind: SaturatorType::AlphaVerb,
            ..Default::default()
        };
        for mode in manifest["alpha_verb"]["dynamics_modes"].as_array().unwrap() {
            slot.settings.alpha_verb_dynamics = mode["normalized"].as_f64().unwrap() as f32;
            core.configure(slot, 120.0);
            let Algorithm::AlphaVerb { engine, .. } = &core.engine else {
                unreachable!()
            };
            let bytes = engine.state_bytes();
            let flags =
                u16::from_le_bytes(bytes[0x6b0 + 0x1384..0x6b0 + 0x1386].try_into().unwrap());
            assert_eq!(
                u64::from(flags),
                mode["flags"].as_u64().unwrap(),
                "{}",
                mode["label"]
            );
        }
    }
}
