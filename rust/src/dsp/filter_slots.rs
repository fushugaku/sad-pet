//! Filters 3 and up at user-chosen points of the signal path.
//! Every slot owns a filter per voice, one for the shared bus and one at the
//! Resynth oscillator rate, so position, model and response can change from a
//! control without allocating. Banks are prepared off the audio thread and
//! keep their histories across preset updates that retain the slot.
use super::filter::{ShapedFilter as Filter, tracked_cutoff};
use crate::{
    Stereo,
    color::{ColorSlot, MAX_COLOR_SLOTS},
    preset::{FilterPosition, FilterSlot},
};

pub(crate) const POSITIONS: usize = FilterPosition::ALL.len();
pub(crate) type FxMasks = [[u16; MAX_COLOR_SLOTS]; 2];

pub(crate) struct Bank {
    id: u32,
    voices: Box<[Filter]>,
    shared: Filter,
    oscillator: Filter,
    oscillator_voices: Box<[Filter]>,
    voice_send_decimators: [super::resynth::Decimator; super::VOICES],
    voice_send_phases: [bool; super::VOICES],
    send_decimator: super::resynth::Decimator,
    oversampled: bool,
    send_phase: bool,
    gated_shared: bool,
    voice_gain: f32,
}
impl Bank {
    fn new(id: u32, rate: u32) -> Box<Self> {
        Box::new(Self {
            id,
            voices: (0..super::VOICES)
                .map(|_| Filter::new(rate))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            shared: Filter::new(rate),
            oscillator: Filter::new(rate * super::resynth::oversample(rate)),
            oscillator_voices: (0..super::VOICES)
                .map(|_| Filter::new(rate * super::resynth::oversample(rate)))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            voice_send_decimators: std::array::from_fn(|_| super::resynth::Decimator::new()),
            voice_send_phases: [false; super::VOICES],
            send_decimator: super::resynth::Decimator::new(),
            oversampled: super::resynth::oversample(rate) == 2,
            send_phase: false,
            gated_shared: false,
            voice_gain: 1.0,
        })
    }
    pub(crate) fn clear(&mut self) {
        self.voices.iter_mut().for_each(Filter::clear);
        self.shared.clear();
        self.oscillator.clear();
        self.oscillator_voices.iter_mut().for_each(Filter::clear);
        self.voice_send_decimators
            .iter_mut()
            .for_each(super::resynth::Decimator::clear);
        self.voice_send_phases.fill(false);
        self.send_decimator.clear();
        self.send_phase = false;
    }
    pub(crate) fn clear_voice(&mut self, voice: usize) {
        self.voices[voice].clear();
        self.oscillator_voices[voice].clear();
        self.voice_send_decimators[voice].clear();
        self.voice_send_phases[voice] = false;
    }
}

/// Banks in slot order, prepared on a control thread for the audio owner.
#[derive(Default)]
pub struct Prepared(pub(crate) Vec<Box<Bank>>);
impl Clone for Prepared {
    fn clone(&self) -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for Prepared {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreparedFilterSlots")
            .field("banks", &self.0.len())
            .finish()
    }
}
impl Prepared {
    pub fn prepare(&mut self, slots: &[FilterSlot], rate: u32) {
        self.0 = slots.iter().map(|slot| Bank::new(slot.id, rate)).collect();
    }
    /// Synchronous owners take the running banks of retained slots instead
    /// of allocating them again; only new slots get fresh banks.
    pub(crate) fn prepare_reusing(
        &mut self,
        slots: &[FilterSlot],
        rate: u32,
        running: &mut Vec<Box<Bank>>,
    ) {
        self.0 = slots
            .iter()
            .map(
                |slot| match running.iter().position(|bank| bank.id == slot.id) {
                    Some(index) => running.swap_remove(index),
                    None => Bank::new(slot.id, rate),
                },
            )
            .collect();
    }
}

/// Give retained slots their running banks back, so an update keeps their
/// filter histories; the prepared ones they replace are retired instead.
pub(crate) fn inherit(next: &mut [Box<Bank>], running: &mut [Box<Bank>]) {
    for bank in next {
        if let Some(old) = running.iter_mut().find(|old| old.id == bank.id) {
            std::mem::swap(bank, old);
        }
    }
}

/// Enabled slots with a bank, as one bit mask per position in list order.
pub(crate) fn masks(slots: &[FilterSlot], banks: &[Box<Bank>]) -> [u16; POSITIONS] {
    let mut masks = [0; POSITIONS];
    for (i, (slot, bank)) in slots.iter().zip(banks).enumerate() {
        if slot.enabled && slot.id == bank.id {
            masks[slot.position as usize] |= 1 << i;
        }
    }
    masks
}

/// Resolve stable FX IDs at control rate. Last shares the final effect's
/// point, preserving filter list order there; an empty rack uses point 0.
pub(crate) fn fx_masks(slots: &[FilterSlot], mut mask: u16, chains: [&[ColorSlot]; 2]) -> FxMasks {
    let mut masks = [[0; MAX_COLOR_SLOTS]; 2];
    while mask != 0 {
        let i = mask.trailing_zeros() as usize;
        mask &= mask - 1;
        let slot = &slots[i];
        let chain = slot.fx_chain as usize;
        let effects = chains[chain];
        let point = slot
            .fx_slot
            .and_then(|id| effects.iter().position(|fx| fx.id == id))
            .unwrap_or_else(|| effects.len().saturating_sub(1));
        masks[chain][point] |= 1 << i;
    }
    masks
}

/// The slot's cutoff after its envelope and key tracking.
fn cutoff<const N: usize>(slot: &FilterSlot, note: f32, envs: [f32; N]) -> f32 {
    tracked_cutoff(
        slot.model,
        slot.kind,
        slot.cutoff,
        slot.env.value(&envs),
        slot.env_depth,
        note,
        slot.keytrack,
    )
}

/// Control-rate coefficients for the shared and oscillator-rate filters.
/// Without a note of their own they follow each envelope's loudest voice, as
/// an ENV routed to a destination does, and track the last note played.
/// `every` also configures the shared filter of per-voice positions, which a
/// Rack runs on a whole chain.
pub(crate) fn configure_shared<const N: usize>(
    slots: &[FilterSlot],
    banks: &mut [Box<Bank>],
    note: f32,
    envs: [f32; N],
    every: bool,
) {
    for (slot, bank) in slots.iter().zip(banks) {
        if slot.id != bank.id || (slot.position.per_voice() && !every) {
            continue;
        }
        bank.gated_shared = every
            || (slot.position == FilterPosition::AfterFx
                && slot.fx_chain == crate::preset::FilterFxChain::B);
        let filter = if slot.position.oscillator().is_some() {
            &mut bank.oscillator
        } else {
            &mut bank.shared
        };
        filter.configure(
            slot.model,
            slot.kind,
            cutoff(slot, note, envs),
            slot.resonance,
            slot.drive,
            slot.morph,
        );
        filter.configure_shapers(slot.shapers, note, &envs);
    }
}

/// One voice's filters, with its note and envelopes as for Filter 1 and 2.
pub(crate) fn configure_voice<const N: usize>(
    slots: &[FilterSlot],
    banks: &mut [Box<Bank>],
    voice: usize,
    note: f32,
    envs: [f32; N],
) {
    for (slot, bank) in slots.iter().zip(banks) {
        if slot.id != bank.id {
            continue;
        }
        bank.voices[voice].configure(
            slot.model,
            slot.kind,
            cutoff(slot, note, envs),
            slot.resonance,
            slot.drive,
            slot.morph,
        );
        bank.voices[voice].configure_shapers(slot.shapers, note, &envs);
        if slot.position.oscillator().is_some() {
            bank.oscillator_voices[voice].configure(
                slot.model,
                slot.kind,
                cutoff(slot, note, envs),
                slot.resonance,
                slot.drive,
                slot.morph,
            );
            bank.oscillator_voices[voice].configure_shapers(slot.shapers, note, &envs);
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Instance {
    Voice(usize),
    Shared,
    Oscillator,
    OscillatorVoice(usize),
}

/// This is note activity, separate from tap/send gain. Shared filters after
/// note summing gate only the extra Sub oscillator, leaving audio tails intact.
pub(crate) fn set_voice_gain(banks: &mut [Box<Bank>], gain: f32) {
    for bank in banks {
        bank.voice_gain = if bank.gated_shared {
            gain.clamp(0.0, 1.0)
        } else {
            1.0
        };
    }
}

pub(crate) fn latency_samples(banks: &[Box<Bank>], mut mask: u16, instance: Instance) -> usize {
    let mut latency = 0;
    while mask != 0 {
        let i = mask.trailing_zeros() as usize;
        mask &= mask - 1;
        let Some(bank) = banks.get(i) else { break };
        let filter = match instance {
            Instance::Voice(voice) => &bank.voices[voice],
            Instance::Shared => &bank.shared,
            Instance::Oscillator => &bank.oscillator,
            Instance::OscillatorVoice(voice) => &bank.oscillator_voices[voice],
        };
        latency += filter.latency_samples();
    }
    latency
}

/// Run one position's filters in list order; an empty mask is a no-op.
#[inline]
#[cfg(test)]
pub(crate) fn chain(
    banks: &mut [Box<Bank>],
    mask: u16,
    frame: Stereo,
    instance: Instance,
) -> Stereo {
    route_chain(banks, mask, frame, instance, None)
}

/// A tap is taken at each filter, before its Chain blend. `gain` is the
/// surrounding voice's VCA/normalization; timbre volume is applied at finish.
#[inline]
pub(crate) fn route_chain(
    banks: &mut [Box<Bank>],
    mask: u16,
    frame: Stereo,
    instance: Instance,
    route: Option<(&mut super::taps::Runtime, f32)>,
) -> Stereo {
    route_chain_with_voice_gain(banks, mask, frame, instance, route, None)
}

/// Override the generated Sub component for a split note/shared branch.
/// Filter histories and the incoming audio continue at their original gain.
#[inline]
pub(crate) fn route_chain_with_voice_gain(
    banks: &mut [Box<Bank>],
    mut mask: u16,
    mut frame: Stereo,
    instance: Instance,
    mut route: Option<(&mut super::taps::Runtime, f32)>,
    voice_gain: Option<f32>,
) -> Stereo {
    while mask != 0 {
        let i = mask.trailing_zeros() as usize;
        mask &= mask - 1;
        let Some(bank) = banks.get_mut(i) else {
            break;
        };
        let filter = match instance {
            Instance::Voice(voice) => &mut bank.voices[voice],
            Instance::Shared => &mut bank.shared,
            Instance::Oscillator => &mut bank.oscillator,
            Instance::OscillatorVoice(voice) => &mut bank.oscillator_voices[voice],
        };
        let gain = if let Some(gain) = voice_gain {
            gain
        } else if matches!(instance, Instance::Shared | Instance::Oscillator) {
            bank.voice_gain
        } else {
            1.0
        };
        let output = filter.process_with_voice_gain(frame, gain);
        let aligned = filter.aligned_input();
        if let Some((taps, gain)) = route.as_mut() {
            if taps.slot_sends(i) {
                let (sent, emit) = if let Instance::OscillatorVoice(voice) = instance {
                    if bank.oversampled {
                        bank.voice_send_phases[voice] = !bank.voice_send_phases[voice];
                        (
                            bank.voice_send_decimators[voice].next(output),
                            !bank.voice_send_phases[voice],
                        )
                    } else {
                        (output, true)
                    }
                } else if matches!(instance, Instance::Oscillator) && bank.oversampled {
                    bank.send_phase = !bank.send_phase;
                    (bank.send_decimator.next(output), !bank.send_phase)
                } else {
                    (output, true)
                };
                if emit {
                    taps.send_slot(i, sent.map(|x| x * *gain));
                }
            } else {
                if let Instance::OscillatorVoice(voice) = instance {
                    bank.voice_send_decimators[voice].clear();
                    bank.voice_send_phases[voice] = false;
                } else {
                    bank.send_decimator.clear();
                    bank.send_phase = false;
                }
            }
            frame = taps.mix_slot(i, aligned, output);
        } else {
            frame = output;
        }
    }
    frame
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preset::FilterSlot;

    fn slot(id: u32) -> FilterSlot {
        FilterSlot {
            id,
            ..FilterSlot::default()
        }
    }
    fn addresses(banks: &[Box<Bank>]) -> Vec<(u32, *const Bank)> {
        banks.iter().map(|b| (b.id, &**b as *const Bank)).collect()
    }

    #[test]
    fn retained_slots_keep_their_banks_across_updates() {
        let mut running = Prepared::default();
        running.prepare(&[slot(1), slot(2), slot(3)], 48000);
        let before = addresses(&running.0);
        // Queued update: freshly prepared banks give way to the running ones.
        let mut next = Prepared::default();
        next.prepare(&[slot(3), slot(4), slot(1)], 48000);
        inherit(&mut next.0, &mut running.0);
        let after = addresses(&next.0);
        assert_eq!(after[0], before[2]);
        assert_eq!(after[2], before[0]);
        assert!(!before.contains(&after[1]));
        // Synchronous update: retained slots are taken without allocating.
        let mut reused = Prepared::default();
        reused.prepare_reusing(&[slot(4), slot(1)], 48000, &mut next.0);
        assert_eq!(addresses(&reused.0), vec![after[1], after[2]]);
        assert_eq!(addresses(&next.0), vec![after[0]]);
    }

    #[test]
    fn masks_follow_list_order_and_skip_bypassed_or_missing_banks() {
        let mut slots = [slot(1), slot(2), slot(3), slot(4)];
        slots[1].enabled = false;
        slots[2].position = FilterPosition::AfterFx;
        let mut banks = Prepared::default();
        banks.prepare(&slots[..3], 48000);
        let masks = masks(&slots, &banks.0);
        assert_eq!(masks[FilterPosition::AfterFilters as usize], 0b0001);
        assert_eq!(masks[FilterPosition::AfterFx as usize], 0b0100);
        assert!(masks.iter().all(|m| m & 0b1010 == 0));
    }

    #[test]
    fn spectral_parties_keep_independent_oversampled_filter_histories() {
        let mut slot = slot(1);
        slot.position = FilterPosition::AfterOsc1;
        slot.cutoff = 800.0;
        let mut banks = Prepared::default();
        banks.prepare(&[slot], 48_000);
        configure_voice(&[slot], &mut banks.0, 1, 60.0, [0.0; 8]);
        configure_voice(&[slot], &mut banks.0, 2, 60.0, [0.0; 8]);
        let mut energy = 0.0;
        for frame in 0..128 {
            let impulse = if frame == 0 { [1.0; 2] } else { [0.0; 2] };
            let output = chain(&mut banks.0, 1, impulse, Instance::OscillatorVoice(1));
            energy += output[0].abs();
            assert_eq!(
                chain(&mut banks.0, 1, [0.0; 2], Instance::OscillatorVoice(2)),
                [0.0; 2]
            );
        }
        assert!(energy > 0.01);
    }
}
