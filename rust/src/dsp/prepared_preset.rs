//! Control-side preparation and ownership of an audio update. Preset documents
//! contain settings; this module owns the resources installed or retired with them.
use super::{Engine, sample::LiveInputPreparation};
use crate::{preset::Preset, wavetable::assets::Pool};

/// A non-Rack bus forces all note gains except voice 0 to zero.
pub(super) fn bus_mono(preset: &Preset) -> bool {
    preset.bus.is_some() && preset.rack.is_none()
}

pub(super) fn resynth_voice_count(preset: &Preset) -> usize {
    if bus_mono(preset) { 1 } else { super::VOICES }
}

/// An independent grain return needs its own main-filter history,
/// separate from a free voice's release in filter bank 0. Reserve by physical
/// module presence so bypass, routing edits, and removal of the last MainPitch
/// binding leave already-playing shared grains intact. Classic file playback
/// always has Grains and Filters; generated instruments and buses do not.
pub(super) fn needs_shared_grain_filter(preset: &Preset) -> bool {
    if let Some(rack) = &preset.rack {
        return [crate::rack::Module::Grains, crate::rack::Module::Filters]
            .into_iter()
            .all(|module| rack.modules.iter().any(|slot| slot.module == module));
    }
    preset.bus.is_none()
        && preset.synth.is_none()
        && preset.wavetable.is_none()
        && preset.drums.is_none()
        && preset.sampler.is_none()
}

/// Full banks made off-thread for a possible mono-bus to instrument change.
#[derive(Default)]
pub(super) struct FilterUpgrades(pub Vec<(usize, super::filter::FilterBank)>);

impl std::fmt::Debug for FilterUpgrades {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FilterUpgrades")
            .field(&self.0.len())
            .finish()
    }
}

#[derive(Default)]
pub(super) struct Resources {
    pub oscillator_runtime: super::oscillators::Prepared,
    pub phase_runtime: super::phase_transport::Prepared,
    pub drum_runtime: super::drums::Prepared,
    pub synth_runtime: super::synth::Prepared,
    pub wavetable_runtime: super::wavetable::Prepared,
    pub tb303_runtime: super::tb303::Prepared,
    pub bus_runtime: super::bus::Prepared,
    pub sampler_runtime: super::sampler::Prepared,
    pub rack_runtime: super::rack::Prepared,
    pub imported_effects: crate::imported_fx::PreparedEffects,
    pub filter_banks: super::filter_slots::Prepared,
    pub sequence: Option<Box<super::sequence::Runtime>>,
    pub sequence_sources: Option<Box<super::sequence_sources::SequenceSources>>,
    /// An immediate update can transfer the compatible running pool at exchange.
    pub reuse_sequence_sources: bool,
    pub sequence_histories: Option<Box<super::sequence_histories::SequenceHistories>>,
    pub filter_upgrades: FilterUpgrades,
    /// Prepared independently, inherited by swap, and retired with this owner.
    /// It must never share DSP state with the note-owned filter bank 0.
    pub shared_grain_filter: Option<super::filter::FilterBank>,
}

impl std::fmt::Debug for Resources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Resources")
            .field("oscillator_runtime", &self.oscillator_runtime)
            .field("phase_runtime", &self.phase_runtime)
            .field("drum_runtime", &self.drum_runtime)
            .field("synth_runtime", &self.synth_runtime)
            .field("wavetable_runtime", &self.wavetable_runtime)
            .field("tb303_runtime", &self.tb303_runtime)
            .field("bus_runtime", &self.bus_runtime)
            .field("sampler_runtime", &self.sampler_runtime)
            .field("rack_runtime", &self.rack_runtime)
            .field("imported_effects", &self.imported_effects)
            .field("filter_banks", &self.filter_banks)
            .field("sequence", &self.sequence)
            .field("sequence_sources", &self.sequence_sources)
            .field("reuse_sequence_sources", &self.reuse_sequence_sources)
            .field("sequence_histories", &self.sequence_histories)
            .field("filter_upgrades", &self.filter_upgrades)
            .field("shared_grain_filter", &self.shared_grain_filter.is_some())
            .finish()
    }
}

/// A finalized update, constructed outside the audio callback. Exchange reuses
/// this envelope for the old settings and resources, which its owner retires.
/// It deliberately has no Clone or serialization implementation.
#[derive(Debug)]
pub struct PreparedPreset {
    pub(super) preset: Preset,
    pub(super) resources: Resources,
    pub(super) sample_rate: u32,
    pub(super) ready: bool,
}

impl PreparedPreset {
    pub fn prepare(preset: Preset, assets: &Pool, sample_rate: u32) -> Result<Self, String> {
        let mut preparation = PresetPreparation::new(preset, sample_rate);
        while !preparation.step(assets, None)? {}
        preparation.finish()
    }

    pub fn preset(&self) -> &Preset {
        &self.preset
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Attach a host's canonical kit and Pad FX routes before publication.
    pub fn with_sampler(mut self, pool: &super::sampler::Pool, routes: &[u32]) -> Self {
        self.resources.sampler_runtime.prepare(
            self.preset.sampler.as_deref(),
            pool,
            routes,
            self.sample_rate,
        );
        self
    }

    /// Tempo changes no resource layout; a staged instrument adopts the latest
    /// session tempo immediately before it is installed.
    pub fn set_tempo(&mut self, bpm: f32) {
        if bpm.is_finite() && (20.0..=300.0).contains(&bpm) {
            self.preset.params.bpm = bpm;
        }
    }
}

/// Incremental preparation. Only `finish` can produce an installable update,
/// and the model is read-only while its resource banks are being constructed.
pub struct PresetPreparation {
    preset: Preset,
    resources: Resources,
    sample_rate: u32,
    stage: u8,
    history_reserve: usize,
    live: Option<LiveInputPreparation>,
    filter_cursor: usize,
}

impl PresetPreparation {
    const STAGES: u8 = 13;

    pub fn new(mut preset: Preset, sample_rate: u32) -> Self {
        preset.sanitize();
        Self {
            preset,
            resources: Resources::default(),
            sample_rate,
            stage: 0,
            history_reserve: 32,
            live: None,
            filter_cursor: 1,
        }
    }

    /// Browser instrument creation reserves its compact history in chunks;
    /// synchronous and queued native preparation retain the full reserve.
    pub fn with_history_reserve(mut self, seconds: usize) -> Self {
        self.history_reserve = seconds.clamp(8, 32);
        self
    }

    pub fn preset(&self) -> &Preset {
        &self.preset
    }

    pub fn set_tempo(&mut self, bpm: f32) {
        if bpm.is_finite() && (20.0..=300.0).contains(&bpm) {
            self.preset.params.bpm = bpm;
        }
    }

    /// Prepare one independent bank, or at most one history allocation/256 KiB
    /// initialization chunk. `previous` must be the engine that will apply this
    /// update before it renders again; queued updates pass None instead.
    pub fn step(&mut self, assets: &Pool, previous: Option<&mut Engine>) -> Result<bool, String> {
        if !(8_000..=384_000).contains(&self.sample_rate) {
            return Err("Invalid preset sample rate".into());
        }
        let previous = previous.filter(|e| e.sample_rate == self.sample_rate);
        let p = &self.preset;
        let r = &mut self.resources;
        let rate = self.sample_rate;
        match self.stage {
            0 => r.sequence = Some(Box::new(super::sequence::Runtime::new(&p.sequencer.tracks))),
            11 => {
                let parties = p
                    .sequencer
                    .tracks
                    .iter()
                    .filter(|t| t.target == crate::sequence::Target::MainPitch)
                    .count();
                let tails = previous
                    .as_ref()
                    .is_some_and(|e| e.voices.iter().any(|v| v.main_pitch && v.active()));
                if parties > 0 || tails {
                    // Resynth can be enabled by numeric automation without a
                    // resource exchange, so its per-note states must be ready.
                    r.reuse_sequence_sources = previous
                        .as_ref()
                        .and_then(|e| e.preset_runtime.sequence_sources.as_ref())
                        .is_some_and(|sources| {
                            sources.supports_counts(
                                rate,
                                resynth_voice_count(p),
                                super::VOICES,
                                p.tb303.is_some(),
                            )
                        });
                    if !r.reuse_sequence_sources {
                        r.sequence_sources = Some(Box::new(
                            super::sequence_sources::SequenceSources::new_with_resynth_count(
                                rate,
                                super::VOICES,
                                resynth_voice_count(p),
                                p.tb303.as_deref(),
                            ),
                        ));
                    }
                    let history = if let Some(rack) = &p.rack {
                        rack.history().is_some()
                    } else {
                        p.synth.is_some()
                    };
                    if history {
                        let old = previous
                            .as_ref()
                            .and_then(|e| e.preset_runtime.sequence_histories.as_deref());
                        r.sequence_histories = Some(Box::new(
                            super::sequence_histories::SequenceHistories::new(rate, parties, old),
                        ));
                    }
                }
            }
            1 => r.wavetable_runtime.prepare_reusing(
                p.wavetable.as_deref(),
                assets,
                rate,
                previous
                    .as_ref()
                    .and_then(|e| e.oscillator_engine.wavetable.as_deref()),
            )?,
            2 => r.oscillator_runtime.prepare_reusing(
                p.oscillators.as_deref(),
                assets,
                rate,
                previous
                    .as_ref()
                    .and_then(|e| e.oscillator_engine.mixed.as_deref()),
            )?,
            3 => r.phase_runtime.prepare(p, rate)?,
            4 => {
                if p.needs_wavetable_interactions()
                    && let Some(wavetable) = &mut r.wavetable_runtime.0
                {
                    wavetable.prepare_interactions();
                }
            }
            5 => r.tb303_runtime.prepare(p.tb303.as_deref(), rate),
            6 => r.drum_runtime.prepare_engine(p.drums.as_deref(), rate),
            7 => r.synth_runtime.prepare_engine(p.synth.as_deref(), rate),
            8 => {
                let keep_rack = p.rack.is_some()
                    && previous.as_ref().is_some_and(|e| {
                        e.rack.is_some() && e.live_input.is_some() && !e.rack_input()
                    });
                let keep_bus =
                    p.bus.is_some() && previous.as_ref().is_some_and(|e| e.bus_history());
                let history = if keep_rack || keep_bus {
                    None
                } else if let Some(rack) = &p.rack {
                    rack.history().map(|h| (h, self.history_reserve, "Rack"))
                } else if let Some(bus) = &p.bus {
                    bus.granular.then(|| (bus.history(), 8, "Bus input"))
                } else if let Some(drums) = &p.drums {
                    Some((
                        crate::synth::History {
                            granular: drums.granular,
                            buffer_seconds: drums.buffer_seconds,
                        },
                        self.history_reserve,
                        "Drum machine",
                    ))
                } else {
                    p.synth
                        .as_ref()
                        .map(|s| (s.history(), self.history_reserve, super::synth::NAME))
                };
                if let Some((history, reserve, name)) = history {
                    let live = self.live.get_or_insert_with(|| {
                        LiveInputPreparation::for_history(
                            rate,
                            !history.granular,
                            reserve,
                            history.buffer_seconds,
                            name,
                        )
                    });
                    if !live.step() {
                        return Ok(false);
                    }
                    let live = self.live.take().unwrap().finish().unwrap();
                    if p.rack.is_some() {
                        r.rack_runtime.0 = Some(live);
                    } else if p.bus.is_some() {
                        r.bus_runtime.0 = Some(live);
                    } else if let Some(drums) = &mut r.drum_runtime.0 {
                        drums.live = Some(live);
                    } else if let Some(synth) = &mut r.synth_runtime.0 {
                        synth.live = Some(live);
                    }
                }
            }
            9 => {
                let racks: [&[crate::color::ColorSlot]; 3] =
                    [&p.color_chain, &p.pre_color_chain, &p.source_color_chain];
                if let Some(previous) = previous {
                    r.imported_effects.prepare_racks_reusing(
                        racks,
                        rate,
                        &previous.preset_runtime.imported_effects,
                        [
                            &previous.preset.color_chain,
                            &previous.preset.pre_color_chain,
                            &previous.preset.source_color_chain,
                        ],
                    );
                    // Destructive reuse is last, after every fallible bank was
                    // prepared successfully. No rejected update can take live filters.
                    r.filter_banks.prepare_reusing(
                        &p.filter_slots,
                        rate,
                        &mut previous.filter_banks,
                    );
                } else {
                    r.imported_effects.prepare_racks(racks, rate);
                    r.filter_banks.prepare(&p.filter_slots, rate);
                }
            }
            10 => {
                if let Some(settings) = p.sampler.as_deref() {
                    r.sampler_runtime.0 = Some(match previous.and_then(|e| e.sampler.as_deref()) {
                        Some(current) => current.prepare_update(settings, rate),
                        None => super::sampler::Runtime::prepared(
                            settings,
                            &Default::default(),
                            &[],
                            rate,
                        ),
                    });
                }
            }
            12 => {
                if !bus_mono(p) {
                    // The first bank exists even in a mono engine. Independent
                    // updates prepare every other bank; immediate updates only
                    // prepare banks the current engine is missing. The loop
                    // creates at most one bank per preparation step.
                    while self.filter_cursor < super::VOICES
                        && previous
                            .as_ref()
                            .is_some_and(|engine| engine.filters[self.filter_cursor].is_prepared())
                    {
                        self.filter_cursor += 1;
                    }
                    if self.filter_cursor < super::VOICES {
                        r.filter_upgrades
                            .0
                            .push((self.filter_cursor, super::filter::FilterBank::new(rate)));
                        self.filter_cursor += 1;
                        return Ok(false);
                    }
                }
                if needs_shared_grain_filter(p) {
                    r.shared_grain_filter = Some(super::filter::FilterBank::new(rate));
                }
            }
            _ => return Ok(true),
        }
        self.stage += 1;
        Ok(self.stage == Self::STAGES)
    }

    pub fn finish(self) -> Result<PreparedPreset, String> {
        if self.stage != Self::STAGES {
            return Err("Preset preparation is incomplete".into());
        }
        Ok(PreparedPreset {
            preset: self.preset,
            resources: self.resources,
            sample_rate: self.sample_rate,
            ready: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rack::{Module, Slot};
    use std::sync::Arc;

    fn grain_filter_rack(enabled: bool) -> Preset {
        let mut preset = crate::timbres::Kind::Oscillators.rack_instrument().unwrap();
        let rack = preset.rack.as_mut().unwrap();
        rack.buffer_seconds = 0.25;
        for (index, module) in [Module::Grains, Module::Filters].into_iter().enumerate() {
            rack.modules.insert(
                index + 1,
                Slot {
                    module,
                    enabled,
                    id: 0,
                },
            );
        }
        preset
    }

    #[test]
    fn shared_grain_filter_is_prepared_for_bypassed_modules_but_not_other_racks() {
        let assets = Pool::default();
        for enabled in [false, true] {
            let prepared =
                PreparedPreset::prepare(grain_filter_rack(enabled), &assets, 8_000).unwrap();
            assert!(needs_shared_grain_filter(prepared.preset()));
            assert!(prepared.resources.shared_grain_filter.is_some());
        }

        let wavetable = crate::timbres::Kind::Wavetable.rack_instrument().unwrap();
        assert!(!needs_shared_grain_filter(&wavetable));
        let bus = crate::timbres::Kind::Bus.preset().unwrap();
        assert!(!needs_shared_grain_filter(&bus));
        let prepared = PreparedPreset::prepare(bus, &assets, 8_000).unwrap();
        assert!(prepared.resources.shared_grain_filter.is_none());

        let mut without_filters = grain_filter_rack(false);
        without_filters
            .rack
            .as_mut()
            .unwrap()
            .modules
            .retain(|slot| slot.module != Module::Filters);
        let prepared = PreparedPreset::prepare(without_filters, &assets, 8_000).unwrap();
        assert!(prepared.resources.shared_grain_filter.is_none());

        // Classic file playback can switch the existing MainPitch grain
        // route during playback, including while the sequencer is stopped.
        for granular in [false, true] {
            let mut file = Preset::default();
            file.sequencer.enabled = false;
            let binding = file
                .sequencer
                .tracks
                .iter_mut()
                .find(|track| track.target == crate::sequence::Target::MainPitch)
                .expect("the default preset has a MainPitch binding");
            binding.granular = granular;
            let prepared = PreparedPreset::prepare(file, &assets, 8_000).unwrap();
            assert!(prepared.resources.shared_grain_filter.is_some());
        }
        let mut no_binding = Preset::default();
        no_binding.sequencer.tracks.clear();
        let prepared = PreparedPreset::prepare(no_binding, &assets, 8_000).unwrap();
        assert!(prepared.resources.shared_grain_filter.is_some());
        assert!(!needs_shared_grain_filter(&Preset {
            synth: Some(Box::default()),
            ..Preset::default()
        }));
        assert!(!needs_shared_grain_filter(&Preset {
            wavetable: Some(Box::default()),
            ..Preset::default()
        }));
    }

    #[test]
    fn shared_grain_filter_cancellation_exchange_and_retirement_preserve_histories() {
        let preset = grain_filter_rack(false);
        let mut engine = Engine::new(
            8_000,
            Arc::new(super::super::SampleBuffer::demo(8_000)),
            preset.clone(),
        );
        let marker = [0.25, -0.125];
        let bank = engine.preset_runtime.shared_grain_filter.as_mut().unwrap();
        assert_eq!(bank.align_source_input(marker, 1), [0.0; 2]);

        // Preparing and abandoning a document cannot take the sounding state.
        let cancelled = engine.prepare_preset_update(preset.clone()).unwrap();
        assert!(cancelled.resources.shared_grain_filter.is_some());
        drop(cancelled);

        // An independent queued document owns a fresh bank. Exchange retains
        // the running delay and sends the unused bank back for off-thread drop.
        let mut queued = PreparedPreset::prepare(preset.clone(), &Pool::default(), 8_000).unwrap();
        assert!(engine.exchange_preset(&mut queued));
        assert_eq!(
            engine
                .preset_runtime
                .shared_grain_filter
                .as_mut()
                .unwrap()
                .align_source_input([0.0; 2], 1),
            marker,
        );
        assert_eq!(
            queued
                .resources
                .shared_grain_filter
                .as_mut()
                .unwrap()
                .align_source_input([0.0; 2], 1),
            [0.0; 2],
        );

        engine
            .preset_runtime
            .shared_grain_filter
            .as_mut()
            .unwrap()
            .align_source_input(marker, 1);
        let mut without_grains = preset.clone();
        without_grains
            .rack
            .as_mut()
            .unwrap()
            .modules
            .retain(|slot| slot.module != Module::Grains);
        let mut remove = engine.prepare_preset_update(without_grains).unwrap();
        assert!(remove.resources.shared_grain_filter.is_none());
        assert!(engine.exchange_preset(&mut remove));
        assert!(engine.preset_runtime.shared_grain_filter.is_none());
        assert_eq!(
            remove
                .resources
                .shared_grain_filter
                .as_mut()
                .unwrap()
                .align_source_input([0.0; 2], 1),
            marker,
        );

        let mut restore = engine.prepare_preset_update(preset).unwrap();
        assert!(engine.exchange_preset(&mut restore));
        assert_eq!(
            engine
                .preset_runtime
                .shared_grain_filter
                .as_mut()
                .unwrap()
                .align_source_input([0.0; 2], 1),
            [0.0; 2],
        );
    }
}
