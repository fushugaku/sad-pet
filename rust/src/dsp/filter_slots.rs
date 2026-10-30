//! Filters 3 and up at user-chosen points of the signal path.
//! Every slot owns a filter per voice, one for the shared bus and one at the
//! Resynth oscillator rate, so position, model and response can change from a
//! control without allocating. Banks are prepared off the audio thread and
//! keep their histories across preset updates that retain the slot.
use super::filter::{Filter, tracked_cutoff};
use crate::{
    Stereo,
    preset::{FilterPosition, FilterSlot},
};

pub(crate) const POSITIONS: usize = FilterPosition::ALL.len();

pub(crate) struct Bank {
    id: u32,
    voices: [Filter; super::VOICES],
    shared: Filter,
    oscillator: Filter,
}
impl Bank {
    fn new(id: u32, rate: u32) -> Box<Self> {
        Box::new(Self {
            id,
            voices: std::array::from_fn(|_| Filter::new(rate)),
            shared: Filter::new(rate),
            oscillator: Filter::new(rate * super::resynth::oversample(rate)),
        })
    }
    pub(crate) fn clear(&mut self) {
        self.voices.iter_mut().for_each(Filter::clear);
        self.shared.clear();
        self.oscillator.clear();
    }
    pub(crate) fn clear_voice(&mut self, voice: usize) {
        self.voices[voice].clear();
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
        if slot.id != bank.id || !slot.position.per_voice() {
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
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Instance {
    Voice(usize),
    Shared,
    Oscillator,
}

/// Run one position's filters in list order; an empty mask is a no-op.
#[inline]
pub(crate) fn chain(
    banks: &mut [Box<Bank>],
    mut mask: u16,
    mut frame: Stereo,
    instance: Instance,
) -> Stereo {
    while mask != 0 {
        let i = mask.trailing_zeros() as usize;
        mask &= mask - 1;
        let Some(bank) = banks.get_mut(i) else {
            break;
        };
        frame = match instance {
            Instance::Voice(voice) => bank.voices[voice].process(frame),
            Instance::Shared => bank.shared.process(frame),
            Instance::Oscillator => bank.oscillator.process(frame),
        };
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
}
