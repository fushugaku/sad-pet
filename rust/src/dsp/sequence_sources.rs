//! Prepared stateful source backends for MainPitch voices. Each pool slot owns
//! its analyzer/carriers or 303 circuit; lookup tables and the outer FX stay shared.
use super::{
    SampleBuffer,
    filter_slots::{Bank, Instance},
    resynth, tb303,
};
use crate::Stereo;
use std::ops::Range;

struct Slot<T> {
    runtime: T,
    generation: Option<u32>,
}

pub(super) struct SequenceSources {
    rate: u32,
    resynth: Vec<Slot<resynth::Resynth>>,
    tb303: Vec<Slot<tb303::Runtime>>,
}

impl std::fmt::Debug for SequenceSources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SequenceSources")
            .field("rate", &self.rate)
            .field("resynth_voices", &self.resynth.len())
            .field("tb303_voices", &self.tb303.len())
            .finish()
    }
}

impl SequenceSources {
    /// Call on the preparation thread only. Render methods never grow banks.
    pub(super) fn new(
        rate: u32,
        voices: usize,
        resynth_enabled: bool,
        bass: Option<&crate::tb303::Settings>,
    ) -> Self {
        let mut resynth = Vec::new();
        if resynth_enabled && voices > 0 {
            resynth.reserve_exact(voices);
            resynth.push(Slot {
                runtime: resynth::Resynth::new(rate),
                generation: None,
            });
            for _ in 1..voices {
                let runtime = resynth[0].runtime.sibling();
                resynth.push(Slot {
                    runtime,
                    generation: None,
                });
            }
            for (index, slot) in resynth.iter_mut().enumerate() {
                slot.runtime
                    .set_filter_instance(Instance::OscillatorVoice(index));
            }
        }
        let tb303 = bass.map_or_else(Vec::new, |settings| {
            (0..voices)
                .map(|_| Slot {
                    runtime: tb303::Runtime::new(rate, settings.controls),
                    generation: None,
                })
                .collect()
        });
        Self {
            rate,
            resynth,
            tb303,
        }
    }

    /// Keep sounding histories on scalar/routing edits. The newly prepared
    /// allocations move back to `old` for disposal outside the callback.
    pub(super) fn inherit(&mut self, old: &mut Self) {
        if self.rate != old.rate {
            return;
        }
        if self.resynth.len() == old.resynth.len() {
            std::mem::swap(&mut self.resynth, &mut old.resynth);
        }
        if self.tb303.len() == old.tb303.len() {
            std::mem::swap(&mut self.tb303, &mut old.tb303);
        }
    }

    pub(super) fn configure_resynth_interactions(
        &mut self,
        index: usize,
        generation: u32,
        inputs: [super::interaction::Input; 4],
        demand: u8,
    ) {
        let Some(slot) = self.resynth.get_mut(index) else {
            return;
        };
        if slot.generation != Some(generation) {
            slot.runtime.reset_sequence_voice();
            slot.generation = Some(generation);
        }
        slot.runtime.configure_interactions(
            inputs.map(|input| [input; crate::resynth::BANDS]),
            std::array::from_fn(|i| if demand & (1 << i) != 0 { u16::MAX } else { 0 }),
        );
    }

    pub(super) fn resynth_meter(&self, index: usize) -> Option<crate::resynth::Meter> {
        self.resynth.get(index).map(|slot| crate::resynth::Meter {
            bands: slot.runtime.carrier_levels(),
            frequencies: slot.runtime.frequencies(),
        })
    }

    /// Raw band taps; callers aggregate them into this party's graph lane.
    pub(super) fn resynth_frames(
        &self,
        index: usize,
    ) -> Option<&[[super::wavetable::mixed::SlotFrame; crate::resynth::BANDS]; 4]> {
        self.resynth
            .get(index)
            .map(|slot| slot.runtime.interaction_frames())
    }

    pub(super) fn configure_tb303_interactions(
        &mut self,
        index: usize,
        generation: u32,
        vco: super::interaction::Input,
        layers: [super::interaction::Input; 4],
        demand: u8,
    ) {
        let Some(slot) = self.tb303.get_mut(index) else {
            return;
        };
        if slot.generation != Some(generation) {
            slot.runtime.reset_sequence_voice(generation);
            slot.generation = Some(generation);
        }
        slot.runtime.configure_interactions(vco, layers, demand);
    }

    pub(super) fn set_tb303_drone(&mut self, index: usize, drone: bool) {
        if let Some(slot) = self.tb303.get_mut(index) {
            slot.runtime.set_sequence_drone(drone);
        }
    }

    pub(super) fn tb303_view(&self, index: usize) -> Option<crate::tb303::View> {
        self.tb303.get(index).map(|slot| slot.runtime.view())
    }

    pub(super) fn tb303_frames(
        &self,
        index: usize,
    ) -> Option<(
        super::wavetable::mixed::SlotFrame,
        [super::wavetable::mixed::SlotFrame; 4],
    )> {
        self.tb303
            .get(index)
            .map(|slot| slot.runtime.interaction_frames())
    }

    /// `dry` and `grains` belong to this voice, never the already summed bus.
    /// `pitch_offset` is relative to the preset/sample root in semitones.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_resynth(
        &mut self,
        index: usize,
        generation: u32,
        mut settings: crate::resynth::Settings,
        pitch_offset: f32,
        mask: u8,
        dry: Stereo,
        source: Stereo,
        grains: Stereo,
        lfo: f32,
        voice_scale: f32,
        sample: &SampleBuffer,
        range: &Range<usize>,
        root: f32,
        running: bool,
        layer_filters: [u16; crate::resynth::MAX_OSCILLATORS],
        banks: &mut [Box<Bank>],
        route: Option<(&mut super::taps::Runtime, f32)>,
    ) -> Stereo {
        let Some(slot) = self.resynth.get_mut(index) else {
            return dry;
        };
        if slot.generation != Some(generation) {
            slot.runtime.reset_sequence_voice();
            slot.generation = Some(generation);
        }
        for oscillator in &mut settings.oscillators {
            oscillator.transpose += pitch_offset;
        }
        // Selection controls audibility only: an unselected layer can still
        // drive another layer or publish a graph modulation tap.
        let audible = settings.oscillators[..settings.count as usize]
            .iter()
            .enumerate()
            .any(|(i, o)| o.enabled && mask & (1 << i) != 0);
        if !audible && !slot.runtime.interactions_active() {
            return dry;
        }
        slot.runtime.set_voice_mask(mask);
        slot.runtime.configure(settings, sample, root);
        let output = slot.runtime.process_with_filters(
            dry,
            source,
            grains,
            lfo,
            voice_scale,
            sample,
            range,
            running,
            layer_filters,
            banks,
            route,
        );
        if audible { output } else { dry }
    }

    /// Mask bits 0..3 select RADIAS layers; bit 4 selects the original VCO.
    /// This private runtime ignores the legacy 303 pattern and MIDI stack.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_tb303(
        &mut self,
        index: usize,
        generation: u32,
        settings: &crate::tb303::Settings,
        controls: crate::tb303::Controls,
        radias: &crate::synth::Settings,
        pitch: f32,
        gate: bool,
        velocity: u8,
        mask: u8,
        lfo: f32,
        bpm: f32,
    ) -> Stereo {
        let Some(slot) = self.tb303.get_mut(index) else {
            return [0.0; 2];
        };
        if slot.generation != Some(generation) {
            slot.runtime.reset_sequence_voice(generation);
            slot.generation = Some(generation);
        }
        slot.runtime.sequence_frame(
            settings, controls, radias, pitch, gate, velocity, mask, lfo, bpm,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn independent_303_voices_preserve_fractional_pitch_and_release() {
        let settings = crate::tb303::Settings::default();
        let radias = crate::synth::Settings::default();
        let mut bank = SequenceSources::new(48_000, 2, false, Some(&settings));
        let mut reference = SequenceSources::new(48_000, 1, false, Some(&settings));
        let mut energy = 0.0;
        for frame in 0..1536 {
            let gate = frame < 768;
            let first = bank.render_tb303(
                0,
                1,
                &settings,
                settings.controls,
                &radias,
                48.25,
                gate,
                80,
                16,
                0.0,
                120.0,
            );
            let second = bank.render_tb303(
                1,
                1,
                &settings,
                settings.controls,
                &radias,
                60.75,
                true,
                110,
                16,
                0.0,
                120.0,
            );
            let expected = reference.render_tb303(
                0,
                1,
                &settings,
                settings.controls,
                &radias,
                48.25,
                gate,
                80,
                16,
                0.0,
                120.0,
            );
            assert_eq!(first, expected, "another party changed this circuit");
            assert!(second.iter().all(|v| v.is_finite()));
            energy += first[0].abs();
        }
        assert!(energy > 0.01);
        assert!((bank.tb303[0].runtime.view().note - 48.25).abs() < 0.001);
        assert!((bank.tb303[1].runtime.view().note - 60.75).abs() < 0.001);
        assert!(!bank.tb303[0].runtime.view().gate);
        assert!(bank.tb303[1].runtime.view().gate);
    }

    #[test]
    fn resynth_parties_do_not_share_carrier_or_analysis_history() {
        let sample = SampleBuffer::new("test".into(), 16_000, vec![[0.0; 2]; 32]).unwrap();
        let mut settings = crate::resynth::Settings::default();
        settings.enabled = true;
        settings.mix = 1.0;
        settings.threshold = 0.0;
        settings.attack = 0.0;
        let mut bank = SequenceSources::new(16_000, 2, true, None);
        let mut reference = SequenceSources::new(16_000, 1, true, None);
        let mut energy = 0.0;
        for frame in 0..512 {
            let input = [((frame as f32) * 0.23).sin() * 0.5; 2];
            let render = |bank: &mut SequenceSources, index, pitch, input| {
                bank.render_resynth(
                    index,
                    1,
                    settings,
                    pitch,
                    15,
                    [0.0; 2],
                    input,
                    [0.0; 2],
                    0.0,
                    1.0,
                    &sample,
                    &(0..32),
                    60.0,
                    true,
                    [0; 4],
                    &mut [],
                    None,
                )
            };
            let first = render(&mut bank, 0, 0.25, input);
            let second = render(&mut bank, 1, 12.75, [0.25; 2]);
            let expected = render(&mut reference, 0, 0.25, input);
            assert_eq!(
                first, expected,
                "another party changed this analyzer/carrier"
            );
            assert!(second.iter().all(|v| v.is_finite()));
            energy += first[0].abs();
        }
        assert!(energy > 0.001);
    }

    #[test]
    fn unselected_resynth_layer_still_modulates_selected_layer() {
        let sample = SampleBuffer::new("mask test".into(), 16_000, vec![[0.0; 2]; 32]).unwrap();
        let mut settings = crate::resynth::Settings::default();
        settings.enabled = true;
        settings.count = 2;
        settings.mix = 1.0;
        settings.threshold = 0.0;
        settings.attack = 0.0;
        settings.oscillators[1].enabled = true;
        settings.oscillators[1].interaction = crate::resynth::Interaction::Ring;
        settings.oscillators[1].interaction_source = 2; // First carrier layer.
        let mut reference_settings = settings;
        reference_settings.oscillators[0].level = 0.0;
        let mut selected = SequenceSources::new(16_000, 1, true, None);
        let mut reference = SequenceSources::new(16_000, 1, true, None);
        let mut energy = 0.0;
        for frame in 0..512 {
            let input = [((frame as f32) * 0.23).sin() * 0.5; 2];
            let render = |bank: &mut SequenceSources, settings, mask| {
                bank.render_resynth(
                    0,
                    1,
                    settings,
                    0.0,
                    mask,
                    [0.0; 2],
                    input,
                    [0.0; 2],
                    0.0,
                    1.0,
                    &sample,
                    &(0..32),
                    60.0,
                    true,
                    [0; 4],
                    &mut [],
                    None,
                )
            };
            let actual = render(&mut selected, settings, 2);
            let expected = render(&mut reference, reference_settings, 3);
            assert_eq!(actual, expected, "mask disabled a hidden modulation source");
            energy += actual[0].abs();
        }
        assert!(energy > 0.001);
    }

    #[test]
    fn deselected_sources_are_silent_and_slot_reuse_is_fresh() {
        let settings = crate::tb303::Settings::default();
        let radias = crate::synth::Settings::default();
        let mut bank = SequenceSources::new(48_000, 1, false, Some(&settings));
        let mut fresh = SequenceSources::new(48_000, 1, false, Some(&settings));
        for _ in 0..128 {
            let silent = bank.render_tb303(
                0,
                1,
                &settings,
                settings.controls,
                &radias,
                48.0,
                true,
                80,
                0,
                0.0,
                120.0,
            );
            assert_eq!(silent, [0.0; 2]);
        }
        for _ in 0..256 {
            let reused = bank.render_tb303(
                0,
                2,
                &settings,
                settings.controls,
                &radias,
                60.5,
                true,
                80,
                16,
                0.0,
                120.0,
            );
            let expected = fresh.render_tb303(
                0,
                2,
                &settings,
                settings.controls,
                &radias,
                60.5,
                true,
                80,
                16,
                0.0,
                120.0,
            );
            assert_eq!(reused, expected);
        }
    }
}
