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
    #[cfg(test)]
    pub(super) fn new(
        rate: u32,
        voices: usize,
        resynth_enabled: bool,
        bass: Option<&crate::tb303::Settings>,
    ) -> Self {
        Self::new_with_resynth_count(rate, voices, if resynth_enabled { voices } else { 0 }, bass)
    }

    /// A Bus only renders its permanent voice. The logical voice count remains
    /// independent so a 303 pool, when requested, still covers every slot.
    pub(super) fn new_with_resynth_count(
        rate: u32,
        voices: usize,
        resynth_voices: usize,
        bass: Option<&crate::tb303::Settings>,
    ) -> Self {
        let mut resynth = Vec::new();
        let resynth_voices = resynth_voices.min(voices);
        if resynth_voices > 0 {
            resynth.reserve_exact(resynth_voices);
            resynth.push(Slot {
                runtime: resynth::Resynth::new(rate),
                generation: None,
            });
            for _ in 1..resynth_voices {
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

    /// Whether this pool can serve a prepared update without growing it.
    /// Bass presence must match: an absent 303 cannot expose a retired
    /// voice's filter envelope or resurrect its state on a later re-add.
    #[cfg(test)]
    pub(super) fn supports(&self, rate: u32, voices: usize, bass: bool) -> bool {
        self.supports_counts(rate, voices, voices, bass)
    }

    pub(super) fn supports_counts(
        &self,
        rate: u32,
        resynth_voices: usize,
        bass_voices: usize,
        bass: bool,
    ) -> bool {
        self.rate == rate
            && self.resynth.len() >= resynth_voices
            && if bass {
                self.tb303.len() >= bass_voices
            } else {
                self.tb303.is_empty()
            }
    }

    /// Keep sounding histories on scalar/routing edits. The newly prepared
    /// allocations move back to `old` for disposal outside the callback.
    pub(super) fn inherit(&mut self, old: &mut Self) {
        if self.rate != old.rate {
            return;
        }
        fn retain<T>(next: &mut Vec<Slot<T>>, old: &mut Vec<Slot<T>>) {
            if next.len() <= old.len() {
                std::mem::swap(next, old);
            } else {
                // Growing a mono pool preserves voice 0 while the additional
                // already-prepared slots remain in the new allocation.
                for (next, old) in next.iter_mut().zip(old.iter_mut()) {
                    std::mem::swap(next, old);
                }
            }
        }
        retain(&mut self.resynth, &mut old.resynth);
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
    use crate::{dsp::Engine, preset::Preset};
    use std::sync::Arc;

    #[test]
    fn preparation_reuses_only_a_compatible_source_pool() {
        let bass = crate::tb303::Settings::default();
        let pool = SequenceSources::new(8_000, 2, true, Some(&bass));
        assert!(pool.supports(8_000, 2, true));
        assert!(!pool.supports(8_000, 2, false));
        assert!(!pool.supports(8_001, 2, false));
        assert!(!pool.supports(8_000, 3, false));
        let no_bass = SequenceSources::new(8_000, 2, true, None);
        assert!(!no_bass.supports(8_000, 2, true));
    }

    #[test]
    fn mono_sources_expand_without_losing_voice_zero_or_the_303_bank() {
        let bass = crate::tb303::Settings::default();
        let mut mono = SequenceSources::new_with_resynth_count(8_000, 33, 1, Some(&bass));
        assert!(mono.supports_counts(8_000, 1, 33, true));
        assert!(!mono.supports_counts(8_000, 33, 33, true));
        mono.resynth[0].generation = Some(17);
        let bass_storage = mono.tb303.as_ptr();

        let mut full = SequenceSources::new_with_resynth_count(8_000, 33, 33, Some(&bass));
        full.inherit(&mut mono);
        assert_eq!(full.resynth.len(), 33);
        assert_eq!(full.resynth[0].generation, Some(17));
        assert_eq!(full.tb303.as_ptr(), bass_storage);

        let full_storage = full.resynth.as_ptr();
        let mut queued_bus = SequenceSources::new_with_resynth_count(8_000, 33, 1, None);
        queued_bus.inherit(&mut full);
        assert_eq!(queued_bus.resynth.as_ptr(), full_storage);
        assert_eq!(queued_bus.resynth[0].generation, Some(17));
        assert!(queued_bus.supports_counts(8_000, 33, 33, false));
        assert!(!queued_bus.supports_counts(8_000, 33, 33, true));
    }

    #[test]
    fn queued_full_filter_banks_retire_without_replacing_the_live_bank() {
        const RATE: u32 = 8_000;
        let preset = Preset::default();
        let mut engine = Engine::new(RATE, Arc::new(SampleBuffer::demo(RATE)), preset.clone());
        let mut reference = super::super::filter::FilterBank::new(RATE);
        let mut params = preset.params;
        params.filter = 0.37;
        params.filter_resonance = 0.5;
        let envs = [0.0; crate::preset::MODULATORS + 1];
        engine.filters[1].configure(&params, 60.0, envs);
        reference.configure(&params, 60.0, envs);
        let compare = |engine: &mut Engine, reference: &mut super::super::filter::FilterBank, n| {
            for i in 0..n {
                let sample = [((i as f32) * 0.17).sin() * 0.4; 2];
                assert_eq!(engine.filters[1].process(sample), reference.process(sample));
            }
        };
        compare(&mut engine, &mut reference, 128);
        for _ in 0..2 {
            let mut queued =
                super::super::PreparedPreset::prepare(preset.clone(), &Default::default(), RATE)
                    .unwrap();
            assert_eq!(queued.resources.filter_upgrades.0.len(), 32);
            assert!(engine.exchange_preset(&mut queued));
            assert!(
                engine
                    .preset_runtime
                    .filter_upgrades
                    .0
                    .iter()
                    .all(|(_, bank)| !bank.is_prepared())
            );
            assert_eq!(queued.resources.filter_upgrades.0.len(), 32);
            assert!(
                queued
                    .resources
                    .filter_upgrades
                    .0
                    .iter()
                    .all(|(_, bank)| bank.is_prepared())
            );
            compare(&mut engine, &mut reference, 128);
        }
    }

    #[test]
    fn removed_and_readded_303_does_not_expose_its_old_voice_envelope() {
        const RATE: u32 = 8_000;
        let mut preset = Preset::default();
        preset.tb303 = Some(Box::default());
        let mut engine = Engine::new(RATE, Arc::new(SampleBuffer::demo(RATE)), preset.clone());
        let settings = engine.preset.tb303.as_ref().unwrap().clone();
        let radias = crate::synth::Settings::default();
        engine.voices[1].main_pitch = true;
        engine.voices[1].env_gate = false;
        engine.voices[1].held = true;
        engine.voices[1].value = 1.0;
        for _ in 0..512 {
            engine
                .preset_runtime
                .sequence_sources
                .as_mut()
                .unwrap()
                .render_tb303(
                    1,
                    7,
                    &settings,
                    settings.controls,
                    &radias,
                    60.0,
                    true,
                    100,
                    16,
                    0.0,
                    120.0,
                );
        }
        assert!(engine.voice_tb303_envelope(1) > 0.0);
        assert!(engine.tb303_view().envelope > 0.0);

        engine.set_preset(Preset::default());
        assert!(engine.voices[1].active());
        assert_eq!(engine.voice_tb303_envelope(1), 0.0);
        assert_eq!(engine.tb303_view().envelope, 0.0);
        assert!(
            engine
                .preset_runtime
                .sequence_sources
                .as_ref()
                .unwrap()
                .tb303
                .is_empty()
        );

        engine.set_preset(preset);
        assert_eq!(engine.voice_tb303_envelope(1), 0.0);
        assert_eq!(engine.tb303_view().envelope, 0.0);
        let sources = engine.preset_runtime.sequence_sources.as_ref().unwrap();
        assert_eq!(sources.tb303.len(), super::super::VOICES);
        assert!(sources.tb303.iter().all(|slot| slot.generation.is_none()));
    }

    #[test]
    fn mono_filter_banks_expand_once_and_stale_updates_are_rejected() {
        const RATE: u32 = 8_000;
        let sample = Arc::new(SampleBuffer::demo(RATE));
        let bus = crate::timbres::Kind::Bus.preset().unwrap();
        let mut engine = Engine::new(RATE, sample.clone(), bus.clone());
        assert!(engine.filters[0].is_prepared());
        assert!(engine.filters[1..].iter().all(|bank| !bank.is_prepared()));
        assert_eq!(
            engine
                .preset_runtime
                .sequence_sources
                .as_ref()
                .unwrap()
                .resynth
                .len(),
            1
        );

        let instrument = Preset::default();
        let cancelled = engine.prepare_preset_update(instrument.clone()).unwrap();
        assert_eq!(cancelled.resources.filter_upgrades.0.len(), 32);
        drop(cancelled);
        assert!(engine.filters[1..].iter().all(|bank| !bank.is_prepared()));

        let mut update = engine.prepare_preset_update(instrument.clone()).unwrap();
        assert!(engine.exchange_preset(&mut update));
        assert!(engine.filters.iter().all(|bank| bank.is_prepared()));
        assert_eq!(
            engine
                .preset_runtime
                .sequence_sources
                .as_ref()
                .unwrap()
                .resynth
                .len(),
            33
        );
        let mut back = engine.prepare_preset_update(bus.clone()).unwrap();
        assert!(engine.exchange_preset(&mut back));
        assert!(engine.filters.iter().all(|bank| bank.is_prepared()));
        assert_eq!(
            engine
                .preset_runtime
                .sequence_sources
                .as_ref()
                .unwrap()
                .resynth
                .len(),
            33
        );

        // No sequence pool is required, so this exercises the independent
        // filter-bank preflight rather than the source-pool stale check.
        let mut no_tracks = instrument.clone();
        no_tracks.sequencer.tracks.clear();
        let mut stale = engine.prepare_preset_update(no_tracks).unwrap();
        assert!(stale.resources.filter_upgrades.0.is_empty());
        let mut other = Engine::new(RATE, sample.clone(), bus);
        assert!(!other.exchange_preset(&mut stale));
        assert!(other.preset.bus.is_some());
        assert!(other.filters[1..].iter().all(|bank| !bank.is_prepared()));

        // A standalone prepared instrument owns exactly the banks the fresh
        // engine will consume; construction must not build them a second time.
        let independent =
            super::super::PreparedPreset::prepare(instrument, &Default::default(), RATE).unwrap();
        assert_eq!(independent.resources.filter_upgrades.0.len(), 32);
        let from_prepared = Engine::from_prepared(sample, independent);
        assert!(from_prepared.filters.iter().all(|bank| bank.is_prepared()));
        assert!(
            from_prepared
                .preset_runtime
                .filter_upgrades
                .0
                .iter()
                .all(|(_, bank)| !bank.is_prepared())
        );
    }

    #[test]
    fn adding_303_prepares_missing_voices_and_keeps_existing_resynth_history() {
        let preset = Preset::default();
        let mut engine = Engine::new(8_000, Arc::new(SampleBuffer::demo(8_000)), preset.clone());
        let before = engine
            .preset_runtime
            .sequence_sources
            .as_ref()
            .unwrap()
            .resynth
            .as_ptr();
        let mut with_bass = preset.clone();
        with_bass.tb303 = Some(Box::default());
        let mut update = engine.prepare_preset_update(with_bass).unwrap();
        assert!(!update.resources.reuse_sequence_sources);
        assert!(update.resources.sequence_sources.is_some());
        assert!(engine.exchange_preset(&mut update));
        let sources = engine.preset_runtime.sequence_sources.as_ref().unwrap();
        assert_eq!(sources.resynth.as_ptr(), before);
        assert!(sources.supports(8_000, super::super::VOICES, true));

        let mut without_bass = engine.prepare_preset_update(preset).unwrap();
        assert!(!without_bass.resources.reuse_sequence_sources);
        assert!(without_bass.resources.sequence_sources.is_some());
        assert!(engine.exchange_preset(&mut without_bass));
        let sources = engine.preset_runtime.sequence_sources.as_ref().unwrap();
        assert_eq!(sources.resynth.as_ptr(), before);
        assert!(sources.supports(8_000, super::super::VOICES, false));
        assert!(!sources.supports(8_000, super::super::VOICES, true));
    }

    #[test]
    fn cancelled_and_repeated_updates_preserve_the_sounding_source_history() {
        const RATE: u32 = 8_000;
        let sample = Arc::new(
            SampleBuffer::new(
                "source history".into(),
                RATE,
                (0..RATE)
                    .map(|i| [((i as f32) * 0.19).sin() * 0.4; 2])
                    .collect(),
            )
            .unwrap(),
        );
        let mut preset = Preset::default();
        preset.sequencer.enabled = true;
        preset.resynth.enabled = true;
        preset.resynth.mix = 1.0;
        preset.resynth.attack = 0.0;
        preset.resynth.threshold = 0.0;
        preset.color_chain.clear();
        let mut engine = Engine::new(RATE, sample.clone(), preset.clone());
        let mut reference = Engine::new(RATE, sample, preset.clone());
        for engine in [&mut engine, &mut reference] {
            engine.set_midi_gate(true);
            engine.set_playing(true);
        }
        let mut energy = 0.0;
        for _ in 0..512 {
            let frame = engine.process_frame();
            assert_eq!(frame, reference.process_frame());
            energy += frame[0].abs() + frame[1].abs();
        }
        assert!(energy > 0.001, "the running source must be audible");
        let sources = engine.preset_runtime.sequence_sources.as_ref().unwrap();
        let voice_storage = sources.resynth.as_ptr();
        assert!(sources.resynth.iter().any(|slot| slot.generation.is_some()));

        let cancelled = engine.prepare_preset_update(preset.clone()).unwrap();
        assert!(cancelled.resources.reuse_sequence_sources);
        assert!(cancelled.resources.sequence_sources.is_none());
        drop(cancelled);
        assert_eq!(
            engine
                .preset_runtime
                .sequence_sources
                .as_ref()
                .unwrap()
                .resynth
                .as_ptr(),
            voice_storage
        );
        for _ in 0..128 {
            assert_eq!(engine.process_frame(), reference.process_frame());
        }

        let mut update = engine.prepare_preset_update(preset.clone()).unwrap();
        assert!(update.resources.reuse_sequence_sources);
        assert!(engine.exchange_preset(&mut update));
        assert_eq!(
            engine
                .preset_runtime
                .sequence_sources
                .as_ref()
                .unwrap()
                .resynth
                .as_ptr(),
            voice_storage
        );
        for _ in 0..512 {
            assert_eq!(engine.process_frame(), reference.process_frame());
        }

        // A queued update has its own prepared pool. Exchange inherits the
        // running voices and returns the unused pool to the retired envelope.
        let mut queued =
            super::super::PreparedPreset::prepare(preset, &Default::default(), RATE).unwrap();
        assert!(!queued.resources.reuse_sequence_sources);
        let unused_storage = queued
            .resources
            .sequence_sources
            .as_ref()
            .unwrap()
            .resynth
            .as_ptr();
        assert_ne!(unused_storage, voice_storage);
        assert!(engine.exchange_preset(&mut queued));
        assert_eq!(
            engine
                .preset_runtime
                .sequence_sources
                .as_ref()
                .unwrap()
                .resynth
                .as_ptr(),
            voice_storage
        );
        assert_eq!(
            queued
                .resources
                .sequence_sources
                .as_ref()
                .unwrap()
                .resynth
                .as_ptr(),
            unused_storage
        );
        for _ in 0..512 {
            assert_eq!(engine.process_frame(), reference.process_frame());
        }
    }

    #[test]
    fn stale_reuse_is_rejected_before_changing_either_owner() {
        let sample = Arc::new(SampleBuffer::demo(8_000));
        let preset = Preset::default();
        let mut engine = Engine::new(8_000, sample, preset.clone());
        let mut stale = engine.prepare_preset_update(preset.clone()).unwrap();
        assert!(stale.resources.reuse_sequence_sources);

        let mut no_sources = preset.clone();
        no_sources.sequencer.tracks.clear();
        let mut remove = engine.prepare_preset_update(no_sources).unwrap();
        assert!(engine.exchange_preset(&mut remove));
        assert!(engine.preset_runtime.sequence_sources.is_none());
        assert!(!engine.exchange_preset(&mut stale));
        assert!(stale.ready);
        assert!(engine.preset.sequencer.tracks.is_empty());

        let mut fresh = engine.prepare_preset_update(preset).unwrap();
        assert!(!fresh.resources.reuse_sequence_sources);
        assert!(fresh.resources.sequence_sources.is_some());
        assert!(engine.exchange_preset(&mut fresh));
        assert!(engine.preset_runtime.sequence_sources.is_some());
    }

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
