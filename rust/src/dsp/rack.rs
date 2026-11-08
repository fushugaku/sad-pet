//! A Rack timbre's frame: its modules in the order they stand, top to bottom.
//! See `crate::rack` for what each module does. The notes, modulators,
//! sequencer and output stage are the engine's own, as in any other timbre.
use super::sample::LiveInput;
use super::{
    CONTROL_INTERVAL, Engine, Stereo, VOICES, bits, fast,
    filter_slots::{self, Instance},
};
use crate::{
    preset::FilterPosition,
    rack::{Chain, Module, Plan, STRIP_SOURCES as SOURCES, STRIPS, Settings, Strip, Strips},
    taps::Stage,
};

/// The Grain history allocated off the audio thread for a Rack.
/// The engine takes it over in `exchange_preset`.
#[derive(Default)]
pub struct Prepared(pub Option<LiveInput>);

impl std::fmt::Debug for Prepared {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreparedRack")
            .field("history", &self.0.is_some())
            .finish()
    }
}

fn strip_index(source: Module) -> usize {
    SOURCES.iter().position(|&s| s == source).unwrap_or(0)
}

/// What the frame loop keeps of a Rack between frames.
#[derive(Clone, Copy)]
pub(super) struct Runtime {
    plan: Plan,
    /// The Grain engine is on and in some chain.
    pub(super) grains: bool,
    /// The Grain engine granulates the Sample's file, note by note.
    pub(super) reads_file: bool,
    /// The history's upstream source already rendered each note's pitch.
    pub(super) history_pitched: bool,
    /// Filter 1 and 2 run a pair per note; otherwise voice 0's pair filters
    /// the chain as one.
    pub(super) filters_per_note: bool,
    /// The strips' gains ramp to their targets so a fader never clicks.
    targets: [Stereo; STRIPS],
    gains: [Stereo; STRIPS],
    /// Each chain's loudest sample since the host last read it.
    peaks: [f32; STRIPS],
    capture: [f32; crate::oscillators::SLOTS],
    capture_initialized: bool,
    /// The Sampler and Vocal modules are in the rack.
    kit: bool,
    vocal: bool,
    /// The EQs that play: on and under a source, bit n for EQ n + 1.
    pub(super) equalizers: u16,
}
impl Runtime {
    pub(super) fn new(settings: &Settings, previous: Option<&Runtime>) -> Self {
        let plan = settings.plan();
        let targets = Self::targets(&plan, &settings.strips);
        let in_chain = |module| {
            plan.iter()
                .any(|step| step.module == module && step.enabled && step.chain != Chain::None)
        };
        Self {
            plan,
            grains: in_chain(Module::Grains),
            reads_file: plan.iter().any(|step| step.reads_file),
            history_pitched: plan
                .iter()
                .find(|step| step.module == Module::Grains && step.enabled && !step.reads_file)
                .is_some_and(|step| {
                    matches!(
                        step.chain,
                        Chain::Source(
                            Module::Synth | Module::Wavetable | Module::Oscillators | Module::Tb303
                        )
                    )
                }),
            filters_per_note: plan
                .iter()
                .any(|step| step.module == Module::Filters && step.per_note),
            targets,
            gains: previous.map_or(targets, |p| p.gains),
            peaks: previous.map_or([0.0; STRIPS], |p| p.peaks),
            capture: previous.map_or([1.0; crate::oscillators::SLOTS], |p| p.capture),
            capture_initialized: previous.is_some_and(|p| {
                p.capture_initialized && p.plan.iter().any(|s| s.module == Module::Oscillators)
            }),
            kit: settings.has(Module::Sampler),
            vocal: settings.has(Module::Vocal),
            equalizers: plan
                .iter()
                .filter(|s| s.module == Module::Eq && s.enabled && s.chain != Chain::None)
                .filter_map(|s| s.id.checked_sub(1))
                .fold(0, |mask, index| mask | 1 << index),
        }
    }
    /// A solo keeps the soloed chains among those that reach the Mixer.
    fn targets(plan: &Plan, strips: &Strips) -> [Stereo; STRIPS] {
        let soloing = SOURCES
            .iter()
            .any(|&s| strips.get(s).solo && plan.iter().any(|step| step.module == s && step.mixed));
        SOURCES.map(|s| Strip::gains(strips.get(s), soloing))
    }
    /// The strips as modulated in this control block.
    pub(super) fn retarget(&mut self, strips: &Strips) {
        self.targets = Self::targets(&self.plan, strips);
    }
}

/// One chain on its way down: the notes' own signals while they are apart,
/// then their sum.
struct Flow {
    chain: Chain,
    /// The chain's source feeds the Mixer under it.
    mixed: bool,
    per_note: bool,
    notes: [Stereo; VOICES],
    /// A legacy source that remains in the chain alongside sequenced notes.
    shared: Stereo,
    /// A routed shared source/cloud exists, even at a sample zero crossing.
    /// Merely reserving its DSP bank does not activate this signal branch.
    shared_active: bool,
    signal: Stereo,
    /// The chain's source, for Resynth's Source input.
    source: Stereo,
    source_notes: [Stereo; VOICES],
    /// The chain's grains, for Resynth's Grains input.
    grains: Option<Stereo>,
    grain_notes: Option<[Stereo; VOICES]>,
    /// Sub oscillators follow the branch that still carries each note;
    /// a prepared filter bank alone must not introduce another source.
    note_sub_gain: [f32; VOICES],
    shared_sub_gain: f32,
    bypass_notes: [Stereo; VOICES],
    split: bool,
}

impl Engine {
    /// The loudest sample of each source's chain (`STRIP_SOURCES`) since the
    /// peaks were last cleared, after their Mixer strips.
    pub fn rack_peaks(&self) -> Option<[f32; STRIPS]> {
        self.rack.as_ref().map(|rack| rack.peaks)
    }
    pub fn clear_rack_peaks(&mut self) {
        if let Some(rack) = &mut self.rack {
            rack.peaks = [0.0; STRIPS];
        }
    }
    /// What a Rack's Pad FX timbres return, for its Sampler in the next frame.
    pub fn return_pads(&mut self, frame: Stereo) {
        self.pad_return = frame;
    }

    /// The notes' signals summed through their VCA.
    #[inline]
    fn rack_collapse(&self, flow: &mut Flow) {
        if flow.per_note {
            let mut sum = flow.shared;
            for i in bits(self.sounding) {
                let value = self.voices[i].value;
                if value <= 0.0 || flow.note_sub_gain[i] <= 0.0 {
                    continue;
                }
                flow.shared_active = true;
                for (total, x) in sum.iter_mut().zip(flow.notes[i]) {
                    *total += x * value;
                }
            }
            flow.signal = sum;
            flow.shared = [0.0; 2];
            flow.per_note = false;
        }
    }
    fn rack_merge_bypass(&self, flow: &mut Flow) {
        if !flow.split {
            return;
        }
        for i in bits(self.sounding) {
            let gain = self.voices[i].value.max(0.0);
            flow.shared_active |= gain > 0.0;
            for c in 0..2 {
                flow.signal[c] += flow.bypass_notes[i][c] * gain;
            }
        }
        flow.split = false;
        flow.bypass_notes.fill([0.0; 2]);
    }

    /// The notes' signals as one, each weighed by its VCA and the sum by
    /// all of them: what a classic timbre's Resynth and Pre-filter FX hear.
    #[inline]
    fn rack_mean(&self, flow: &Flow) -> (Stereo, f32) {
        let mut sum = flow.shared;
        let mut weight = f32::from(flow.shared_active);
        for i in bits(self.sounding) {
            let value = self.voices[i].value;
            if value <= 0.0 || flow.note_sub_gain[i] <= 0.0 {
                continue;
            }
            for (total, x) in sum.iter_mut().zip(flow.notes[i]) {
                *total += x * value;
            }
            weight += value;
        }
        if weight > 0.0 {
            sum = sum.map(|x| x / weight);
        }
        (sum, weight)
    }
    /// Each note takes what a stage did to the notes' `mean`.
    #[inline]
    fn rack_share(&self, flow: &mut Flow, mean: Stereo, output: Stereo) {
        let change = [output[0] - mean[0], output[1] - mean[1]];
        if flow.shared_active {
            for (shared, change) in flow.shared.iter_mut().zip(change) {
                *shared += change;
            }
        }
        for i in bits(self.sounding) {
            if self.voices[i].value <= 0.0 || flow.note_sub_gain[i] <= 0.0 {
                continue;
            }
            for (x, d) in flow.notes[i].iter_mut().zip(change) {
                *x += d;
            }
        }
    }

    /// `input` is the voice of a Vocal module, which the host renders.
    pub(super) fn process_rack_frame(&mut self, input: Stereo) -> Stereo {
        self.taps.begin_frame();
        let Some(rack) = &mut self.rack else {
            return [0.0; 2];
        };
        for (gain, target) in rack.gains.iter_mut().zip(rack.targets) {
            for c in 0..2 {
                // Settled, a muted strip is silent rather than nearly so.
                let next = gain[c] + (target[c] - gain[c]) * 0.002;
                gain[c] = if (target[c] - next).abs() < 1e-6 {
                    target[c]
                } else {
                    next
                };
            }
        }
        let streams = (rack.kit, rack.vocal);
        if self.playing {
            self.tick_events();
        }
        self.track_tb303_note();
        self.sync_history_window();
        let input_gain = self.input_gain;
        let bypass = self.bypass;
        let source_rate = self.sample.sample_rate as f64 / self.sample_rate as f64;
        // The sources render first; the drums keep their clock while off,
        // and the kit's voices play on.
        let drums = self.drums.as_mut().map_or([0.0; 2], |d| d.frame());
        let kit = if streams.0 {
            let pads = self.render_sampler();
            let returned = std::mem::take(&mut self.pad_return);
            std::array::from_fn(|c| pads[c] + returned[c])
        } else {
            [0.0; 2]
        };
        let vocal = if streams.1 {
            input.map(|x| {
                if x.is_finite() {
                    x.clamp(-16.0, 16.0)
                } else {
                    0.0
                }
            })
        } else {
            [0.0; 2]
        };
        // A live input plays in the Sample's place and fills the history its
        // Grain engine reads, as a Granular timbre's does; grain feedback
        // enters that history.
        let live = self.rack_input();
        let file = if live {
            let clean = input.map(|x| {
                if x.is_finite() {
                    x.clamp(-16.0, 16.0)
                } else {
                    0.0
                }
            });
            let fed = self.grain_feedback.output;
            let recorded = if fed == [0.0; 2] {
                clean
            } else {
                std::array::from_fn(|c| clean[c] + fed[c])
            };
            let frozen = self.frozen;
            if let Some(history) = &mut self.live_input {
                history.push_with_meter(recorded, input, frozen);
            }
            clean
        } else if self
            .preset
            .rack
            .as_ref()
            .is_some_and(|r| r.has(Module::Sample))
        {
            let fade = self
                .fades
                .gain(self.cursor, self.sample.sample_rate, &self.file_range);
            let mip = self.sample.mip_for_rate(source_rate, true);
            self.sample
                .read_region_mip(self.cursor, mip, &self.file_range)
                .map(|v| v * fade)
        } else {
            [0.0; 2]
        };
        let mut level = (file[0].abs().max(file[1].abs())).max(drums[0].abs().max(drums[1].abs()));
        if streams != (false, false) {
            for frame in [kit, vocal] {
                level = level.max(frame[0].abs().max(frame[1].abs()));
            }
        }
        self.follower = fast::flush(
            self.follower
                + (level - self.follower)
                    * if level > self.follower {
                        0.006
                    } else {
                        0.00015
                    },
        );
        let sr = self.sample_rate as f32;
        let (loudest_voice, voice_sum) = self.advance_modulators(sr);
        if self.frame.is_multiple_of(CONTROL_INTERVAL) {
            self.update_modulation();
        }
        filter_slots::set_voice_gain(&mut self.filter_banks, loudest_voice);
        let bass = self.render_tb303();
        let Some(mut rack) = self.rack else {
            return [0.0; 2];
        };
        // A live input for effects only plays without grains.
        let effects_only = live && self.live_input.as_ref().is_some_and(|i| i.effects_only);
        self.schedule_grains(rack.grains && !bypass && !effects_only);
        self.smooth_mix =
            fast::flush(self.smooth_mix + (self.modulated.params.mix - self.smooth_mix) * 0.002);
        let dry_scale = if voice_sum > 0.0 {
            loudest_voice / voice_sum
        } else {
            0.0
        };
        let masks = self.filter_masks;
        let mut flow = Flow {
            chain: Chain::None,
            mixed: false,
            per_note: false,
            notes: [[0.0; 2]; VOICES],
            shared: [0.0; 2],
            shared_active: false,
            signal: [0.0; 2],
            source: [0.0; 2],
            source_notes: [[0.0; 2]; VOICES],
            grains: None,
            grain_notes: None,
            note_sub_gain: [1.0; VOICES],
            shared_sub_gain: 0.0,
            bypass_notes: [[0.0; 2]; VOICES],
            split: false,
        };
        let mut mixer = [0.0; 2];
        let mut mixer_active = false;
        let mut out = [0.0; 2];
        let mut peaks = rack.peaks;
        for step in rack.plan.iter() {
            // Bypass leaves only the sources and the Mixer.
            let on =
                step.enabled && (!bypass || step.module.source() || step.module == Module::Mixer);
            if step.module.source() || step.module == Module::Mixer {
                self.rack_collapse(&mut flow);
                self.rack_merge_bypass(&mut flow);
                match flow.chain {
                    Chain::Source(source) => {
                        let index = strip_index(source);
                        let sent = if flow.mixed {
                            let gains = rack.gains[index];
                            std::array::from_fn(|c| flow.signal[c] * gains[c])
                        } else {
                            flow.signal
                        };
                        peaks[index] = peaks[index].max(sent[0].abs().max(sent[1].abs()));
                        let into = if flow.mixed { &mut mixer } else { &mut out };
                        if flow.mixed {
                            mixer_active |= flow.shared_active;
                        }
                        for c in 0..2 {
                            into[c] += sent[c];
                        }
                    }
                    Chain::Mixed => {
                        for (total, x) in out.iter_mut().zip(flow.signal) {
                            *total += x;
                        }
                    }
                    Chain::None => {}
                }
                flow.chain = step.chain;
                flow.mixed = step.mixed;
                flow.signal = [0.0; 2];
                flow.shared = [0.0; 2];
                flow.shared_active = false;
                flow.source_notes.fill([0.0; 2]);
                flow.grains = None;
                flow.grain_notes = None;
                flow.note_sub_gain.fill(1.0);
                flow.shared_sub_gain = 0.0;
            }
            if step.chain == Chain::None {
                // Above every source there is nothing to process.
                continue;
            }
            match step.module {
                Module::Connections => {}
                Module::Sample => {
                    let source = file.map(|x| x * input_gain);
                    flow.source = source;
                    if on {
                        // Shared by all notes without multiplying its level
                        // when a chord adds voices, as a Granular source is.
                        flow.per_note = true;
                        let note = source.map(|x| x * dry_scale);
                        for i in bits(self.sounding) {
                            flow.notes[i] = note;
                            flow.source_notes[i] = source;
                        }
                    }
                }
                Module::Synth => {
                    flow.source = [0.0; 2];
                    let Some(synth) = &mut self.oscillator_engine.synth else {
                        continue;
                    };
                    synth.begin_frame(self.lfo_values[0]);
                    if !on {
                        continue;
                    }
                    flow.per_note = true;
                    for i in bits(self.sounding) {
                        let gain = self.voices[i].value;
                        if gain <= 0.0 {
                            continue;
                        }
                        let frame = self.render_synth_voice(i);
                        flow.source_notes[i] = frame.map(|x| x * input_gain);
                        flow.notes[i] = frame.map(|x| x * input_gain);
                        for c in 0..2 {
                            flow.source[c] += flow.notes[i][c] * gain;
                        }
                    }
                }
                Module::Wavetable => {
                    flow.source = [0.0; 2];
                    let Some(wavetable) = &mut self.oscillator_engine.wavetable else {
                        continue;
                    };
                    wavetable.begin_frame(on);
                    if !wavetable.active()
                        && (self.oscillator_engine.graph.demanded
                            | self.oscillator_engine.graph.targets)
                            & (15 << 12)
                            == 0
                    {
                        continue;
                    }
                    flow.per_note = true;
                    for i in bits(self.sounding) {
                        let gain = self.voices[i].value;
                        if gain <= 0.0 {
                            continue;
                        }
                        let frame = self.render_wavetable_voice(i);
                        flow.source_notes[i] = frame.map(|x| x * input_gain);
                        flow.notes[i] = frame.map(|x| x * input_gain);
                        for c in 0..2 {
                            flow.source[c] += flow.notes[i][c] * gain;
                        }
                    }
                }
                Module::Oscillators => {
                    flow.source = [0.0; 2];
                    let split = on
                        && !bypass
                        && self.preset.grain_capture.mode == crate::interaction::CaptureMode::Osc
                        && rack.plan.iter().any(|s| {
                            s.module == Module::Grains && s.enabled && s.chain == step.chain
                        });
                    let selected = self.preset.grain_capture.slots;
                    for slot in 0..crate::oscillators::SLOTS {
                        let target = f32::from(!split || selected & (1 << slot) != 0);
                        if rack.capture_initialized {
                            rack.capture[slot] += (target - rack.capture[slot]).clamp(
                                -1.0 / (self.sample_rate as f32 * 0.005),
                                1.0 / (self.sample_rate as f32 * 0.005),
                            );
                        } else {
                            rack.capture[slot] = target;
                        }
                    }
                    rack.capture_initialized = true;
                    let owner = &mut self.oscillator_engine;
                    let Some(mixed) = &mut owner.mixed else {
                        continue;
                    };
                    mixed.begin_frame(on);
                    flow.per_note = true;
                    flow.split = split || rack.capture.iter().any(|&v| v < 1.0);
                    for i in bits(self.sounding) {
                        let voice = &self.voices[i];
                        if voice.value <= 0.0 {
                            continue;
                        }
                        let note = voice.pitch()
                            + self.bend[voice.channel as usize]
                                * self.modulated.params.midi_pitchbend_range;
                        if voice.main_pitch {
                            owner.graph.prepare_sequence_voice(i, voice.generation);
                        } else {
                            owner.graph.prepare_voice(i, voice.generation);
                        }
                        let inputs = std::array::from_fn(|slot| owner.graph.input(slot, i));
                        let (frames, into, bypass) = mixed.render_masked(
                            i,
                            voice.generation,
                            note,
                            &voice.env_values,
                            &self.voice_lfo_values[i],
                            inputs,
                            (owner.graph.demanded | owner.graph.targets) as u8,
                            owner.graph.demanded as u8,
                            rack.capture,
                            &self.sample,
                            &self.file_range,
                            self.modulated.params.sample_root_note,
                            owner.phase_adapters.as_deref_mut(),
                            self.frame,
                            voice.oscillator_mask() as u8,
                        );
                        for (slot, frame) in frames.into_iter().enumerate() {
                            owner.graph.publish(
                                slot,
                                i,
                                frame.generation,
                                frame.output,
                                voice.value,
                            );
                            if let Some(at) = frame.wrap {
                                owner.graph.natural_event(slot, i, at, frame.latency);
                            }
                        }
                        flow.notes[i] = into.map(|x| x * input_gain);
                        flow.source_notes[i] = flow.notes[i];
                        flow.bypass_notes[i] = bypass.map(|x| x * input_gain);
                        flow.split |= bypass != [0.0; 2];
                        // Source follows the selected branch until the Granular
                        // merge. Processors between them must not hear bypass.
                        for (source, sample) in flow.source.iter_mut().zip(into) {
                            *source += sample * input_gain * voice.value;
                        }
                    }
                }
                Module::Tb303 if self.main_pitch_sequence() => {
                    // The original monophonic keyboard/pattern stays in this
                    // chain along with the independent sequencer parties.
                    flow.shared = if on {
                        bass.map(|x| x * input_gain)
                    } else {
                        [0.0; 2]
                    };
                    flow.source = flow.shared;
                    flow.shared_sub_gain = if on { self.tb303_view().envelope } else { 0.0 };
                    flow.shared_active = on;
                    if on {
                        flow.per_note = true;
                        for i in bits(self.sounding) {
                            if !self.voices[i].main_pitch {
                                continue;
                            }
                            let frame = self.render_main_tb303(i).map(|x| x * input_gain);
                            flow.source_notes[i] = frame;
                            flow.notes[i] = frame;
                            for c in 0..2 {
                                flow.source[c] += flow.notes[i][c] * self.voices[i].value;
                            }
                        }
                    }
                }
                Module::Drums | Module::Sampler | Module::Vocal | Module::Tb303 => {
                    let stream = match step.module {
                        Module::Drums => drums,
                        Module::Sampler => kit,
                        Module::Tb303 => bass,
                        _ => vocal,
                    };
                    flow.source = stream.map(|x| x * input_gain);
                    if on {
                        flow.signal = flow.source;
                        flow.shared_active = true;
                    }
                }
                Module::Mixer => {
                    flow.signal = mixer;
                    flow.source = mixer;
                    flow.shared_active = mixer_active;
                }
                _ if !on => {}
                Module::Grains if step.reads_file => {
                    self.rack_file_grains(&mut flow, file, dry_scale);
                }
                Module::Grains => self.rack_history_grains(&mut flow),
                Module::Filters => {
                    if flow.per_note {
                        let sends = self.taps.sends(Stage::Filters);
                        let mut send = [0.0; 2];
                        for i in bits(self.sounding) {
                            let value = self.voices[i].value;
                            if value <= 0.0 {
                                continue;
                            }
                            let unfiltered = flow.notes[i];
                            let activity = f32::from(flow.note_sub_gain[i] > 0.0);
                            // A note routed into the shared cloud may never
                            // use its private return. Defer that filter chain
                            // until its first active frame. Once used, keep
                            // clocking it through gaps so tails and delays
                            // cannot resume from a frozen history.
                            if activity == 0.0
                                && !self.filters[i].has_processed()
                                && !filter_slots::voice_has_processed(
                                    &self.filter_banks,
                                    masks[FilterPosition::BeforeFilters as usize]
                                        | masks[FilterPosition::AfterFilters as usize],
                                    i,
                                )
                            {
                                flow.notes[i] = [0.0; 2];
                                continue;
                            }
                            let voice = filter_slots::route_chain_with_voice_gain(
                                &mut self.filter_banks,
                                masks[FilterPosition::BeforeFilters as usize],
                                unfiltered,
                                Instance::Voice(i),
                                Some((&mut self.taps, value * activity)),
                                Some(flow.note_sub_gain[i]),
                            );
                            let voice = self.filters[i].process_tapped_with_voice_gain(
                                voice,
                                &mut self.taps,
                                value * activity,
                                flow.note_sub_gain[i],
                            );
                            let voice = filter_slots::route_chain_with_voice_gain(
                                &mut self.filter_banks,
                                masks[FilterPosition::AfterFilters as usize],
                                voice,
                                Instance::Voice(i),
                                Some((&mut self.taps, value * activity)),
                                Some(flow.note_sub_gain[i]),
                            );
                            if sends {
                                for c in 0..2 {
                                    send[c] += voice[c] * value * activity;
                                }
                            }
                            let extra_latency = filter_slots::latency_samples(
                                &self.filter_banks,
                                masks[FilterPosition::BeforeFilters as usize]
                                    | masks[FilterPosition::AfterFilters as usize],
                                Instance::Voice(i),
                            );
                            let aligned =
                                self.filters[i].align_chain_input(unfiltered, extra_latency);
                            flow.notes[i] = self
                                .taps
                                .mix(Stage::Filters, aligned, voice)
                                .map(|x| x * activity);
                        }
                        let use_shared_bank = self
                            .preset_runtime
                            .shared_grain_filter
                            .as_ref()
                            .is_some_and(|filter| filter.is_prepared());
                        if flow.shared_active || use_shared_bank {
                            let input = flow.shared;
                            let activity = f32::from(flow.shared_active);
                            let voice = filter_slots::route_chain_with_voice_gain(
                                &mut self.filter_banks,
                                masks[FilterPosition::BeforeFilters as usize],
                                input,
                                Instance::Shared,
                                Some((&mut self.taps, activity)),
                                Some(flow.shared_sub_gain),
                            );
                            let voice = if use_shared_bank {
                                self.preset_runtime
                                    .shared_grain_filter
                                    .as_mut()
                                    .unwrap()
                                    .process_tapped_with_voice_gain(
                                        voice,
                                        &mut self.taps,
                                        activity,
                                        flow.shared_sub_gain,
                                    )
                            } else {
                                self.filters[0].process_tapped(voice, &mut self.taps, activity)
                            };
                            let voice = filter_slots::route_chain_with_voice_gain(
                                &mut self.filter_banks,
                                masks[FilterPosition::AfterFilters as usize],
                                voice,
                                Instance::Shared,
                                Some((&mut self.taps, activity)),
                                Some(flow.shared_sub_gain),
                            );
                            if sends {
                                for c in 0..2 {
                                    send[c] += voice[c] * activity;
                                }
                            }
                            let extra_latency = filter_slots::latency_samples(
                                &self.filter_banks,
                                masks[FilterPosition::BeforeFilters as usize]
                                    | masks[FilterPosition::AfterFilters as usize],
                                Instance::Shared,
                            );
                            let aligned = if use_shared_bank {
                                self.preset_runtime
                                    .shared_grain_filter
                                    .as_mut()
                                    .unwrap()
                                    .align_chain_input(input, extra_latency)
                            } else {
                                self.filters[0].align_chain_input(input, extra_latency)
                            };
                            flow.shared = self
                                .taps
                                .mix(Stage::Filters, aligned, voice)
                                .map(|x| x * activity);
                        }
                        if sends {
                            self.taps.send(Stage::Filters, send);
                        }
                    } else {
                        let input = flow.signal;
                        let activity = f32::from(flow.shared_active);
                        let use_shared_bank = self
                            .preset_runtime
                            .shared_grain_filter
                            .as_ref()
                            .is_some_and(|filter| filter.is_prepared());
                        let output = filter_slots::route_chain(
                            &mut self.filter_banks,
                            masks[FilterPosition::BeforeFilters as usize],
                            input,
                            Instance::Shared,
                            Some((&mut self.taps, activity)),
                        );
                        let output = if use_shared_bank {
                            self.preset_runtime
                                .shared_grain_filter
                                .as_mut()
                                .unwrap()
                                .process_tapped_with_voice_gain(
                                    output,
                                    &mut self.taps,
                                    activity,
                                    loudest_voice,
                                )
                        } else {
                            self.filters[0].process_tapped_with_voice_gain(
                                output,
                                &mut self.taps,
                                activity,
                                loudest_voice,
                            )
                        };
                        let output = filter_slots::route_chain(
                            &mut self.filter_banks,
                            masks[FilterPosition::AfterFilters as usize],
                            output,
                            Instance::Shared,
                            Some((&mut self.taps, activity)),
                        );
                        let extra_latency = filter_slots::latency_samples(
                            &self.filter_banks,
                            masks[FilterPosition::BeforeFilters as usize]
                                | masks[FilterPosition::AfterFilters as usize],
                            Instance::Shared,
                        );
                        let aligned = if use_shared_bank {
                            self.preset_runtime
                                .shared_grain_filter
                                .as_mut()
                                .unwrap()
                                .align_chain_input(input, extra_latency)
                        } else {
                            self.filters[0].align_chain_input(input, extra_latency)
                        };
                        flow.signal = self.tap(
                            Stage::Filters,
                            aligned.map(|x| x * activity),
                            output.map(|x| x * activity),
                        );
                    }
                }
                Module::Resynth if flow.per_note => {
                    if self.main_pitch_sequence() {
                        for i in bits(self.sounding) {
                            let grains = flow.grain_notes.as_ref().map_or(flow.notes[i], |g| g[i]);
                            flow.notes[i] = self.render_main_resynth(
                                i,
                                flow.notes[i],
                                flow.source_notes[i],
                                grains,
                                matches!(
                                    flow.chain,
                                    Chain::Source(
                                        Module::Synth
                                            | Module::Wavetable
                                            | Module::Oscillators
                                            | Module::Tb303
                                    )
                                ),
                            );
                            // This module can introduce a selected carrier even
                            // when Grains moved the note's dry stream to shared.
                            if self.modulated.resynth.enabled
                                && self.voices[i].oscillator_mask() & (15 << 16) != 0
                            {
                                flow.note_sub_gain[i] = 1.0;
                            }
                        }
                        if flow.shared_active {
                            let input = flow.shared;
                            let output = self.oscillator_engine.resynth.process_interacting(
                                input,
                                input,
                                input,
                                self.lfo_values[0],
                                1.0,
                                &self.sample,
                                &self.file_range,
                                self.playing,
                                [0; 4],
                                &mut self.filter_banks,
                                None,
                                self.oscillator_engine.phase_adapters.as_deref_mut(),
                                self.frame,
                            );
                            flow.shared = self.tap(Stage::Resynth, input, output);
                        }
                        continue;
                    }
                    // As a classic timbre's: on the notes' mean, each note
                    // then taking what it did to that mean.
                    let (mean, weight) = self.rack_mean(&flow);
                    let grains = flow.grains.map_or(mean, |g| {
                        if weight > 0.0 {
                            g.map(|x| x / weight)
                        } else {
                            g
                        }
                    });
                    let output = self.oscillator_engine.resynth.process_interacting(
                        mean,
                        flow.source,
                        grains,
                        self.lfo_values[0],
                        dry_scale,
                        &self.sample,
                        &self.file_range,
                        self.playing,
                        [
                            masks[FilterPosition::AfterOsc1 as usize],
                            masks[FilterPosition::AfterOsc2 as usize],
                            masks[FilterPosition::AfterOsc3 as usize],
                            masks[FilterPosition::AfterOsc4 as usize],
                        ],
                        &mut self.filter_banks,
                        Some((&mut self.taps, weight)),
                        self.oscillator_engine.phase_adapters.as_deref_mut(),
                        self.frame,
                    );
                    let output = filter_slots::route_chain(
                        &mut self.filter_banks,
                        masks[FilterPosition::AfterResynth as usize],
                        output,
                        Instance::Shared,
                        Some((&mut self.taps, weight)),
                    );
                    if self.taps.sends(Stage::Resynth) {
                        self.taps.send(Stage::Resynth, output.map(|x| x * weight));
                    }
                    let output = self.taps.mix(Stage::Resynth, mean, output);
                    self.rack_share(&mut flow, mean, output);
                }
                Module::PreFx if flow.per_note => {
                    let (mean, weight) = self.rack_mean(&flow);
                    let output = self.pre_color_rack.process_with_filters(
                        mean,
                        1.0,
                        &self.filter_fx_masks[0],
                        &mut self.filter_banks,
                        &mut self.taps,
                        weight,
                    );
                    if self.taps.sends(Stage::PreFx) {
                        self.taps.send(Stage::PreFx, output.map(|x| x * weight));
                    }
                    let output =
                        self.taps
                            .mix(Stage::PreFx, self.pre_color_rack.aligned_input(), output);
                    self.rack_share(&mut flow, mean, output);
                }
                Module::Resynth => {
                    self.rack_collapse(&mut flow);
                    let input = flow.signal;
                    if self.main_pitch_sequence() {
                        let weight: f32 = bits(self.sounding).map(|i| self.voices[i].value).sum();
                        let dry = if weight > 0.0 {
                            input.map(|x| x / weight)
                        } else {
                            [0.0; 2]
                        };
                        flow.signal = [0.0; 2];
                        for i in bits(self.sounding) {
                            let output = self.render_main_resynth(
                                i,
                                dry,
                                flow.source,
                                flow.grain_notes
                                    .as_ref()
                                    .map_or(flow.grains.unwrap_or(dry), |g| g[i]),
                                matches!(
                                    flow.chain,
                                    Chain::Source(
                                        Module::Synth
                                            | Module::Wavetable
                                            | Module::Oscillators
                                            | Module::Tb303
                                    )
                                ),
                            );
                            for (signal, output) in flow.signal.iter_mut().zip(output) {
                                *signal += output * self.voices[i].value;
                            }
                        }
                        continue;
                    }
                    let output = self.oscillator_engine.resynth.process_interacting(
                        input,
                        flow.source,
                        flow.grains.unwrap_or(input),
                        self.lfo_values[0],
                        1.0,
                        &self.sample,
                        &self.file_range,
                        self.playing,
                        [
                            masks[FilterPosition::AfterOsc1 as usize],
                            masks[FilterPosition::AfterOsc2 as usize],
                            masks[FilterPosition::AfterOsc3 as usize],
                            masks[FilterPosition::AfterOsc4 as usize],
                        ],
                        &mut self.filter_banks,
                        Some((&mut self.taps, 1.0)),
                        self.oscillator_engine.phase_adapters.as_deref_mut(),
                        self.frame,
                    );
                    let output = filter_slots::route_chain(
                        &mut self.filter_banks,
                        masks[FilterPosition::AfterResynth as usize],
                        output,
                        Instance::Shared,
                        Some((&mut self.taps, 1.0)),
                    );
                    flow.signal = self.tap(Stage::Resynth, input, output);
                }
                Module::Eq => {
                    if flow.per_note && self.main_pitch_sequence() {
                        let (mean, _) = self.rack_mean(&flow);
                        if let Some(index) = step.id.checked_sub(1) {
                            let output = self.equalizers.process(usize::from(index), mean);
                            self.rack_share(&mut flow, mean, output);
                        }
                    } else {
                        self.rack_collapse(&mut flow);
                        if let Some(index) = step.id.checked_sub(1) {
                            flow.signal = self.equalizers.process(usize::from(index), flow.signal);
                        }
                    }
                }
                Module::SourceFx => {
                    if flow.per_note && self.main_pitch_sequence() {
                        let (mean, weight) = self.rack_mean(&flow);
                        let output = self.source_color.process(mean);
                        if self.taps.sends(Stage::Source) {
                            self.taps.send(Stage::Source, output.map(|x| x * weight));
                        }
                        let output = self.taps.mix(Stage::Source, mean, output);
                        self.rack_share(&mut flow, mean, output);
                    } else {
                        self.rack_collapse(&mut flow);
                        let input = flow.signal;
                        let output = self.source_color.process(input);
                        flow.signal = self.tap(Stage::Source, input, output);
                    }
                }
                Module::PreFx => {
                    self.rack_collapse(&mut flow);
                    let input = flow.signal;
                    let output = self.pre_color_rack.process_with_filters(
                        input,
                        1.0,
                        &self.filter_fx_masks[0],
                        &mut self.filter_banks,
                        &mut self.taps,
                        1.0,
                    );
                    flow.signal =
                        self.tap(Stage::PreFx, self.pre_color_rack.aligned_input(), output);
                }
                Module::PostFx => {
                    if flow.per_note && self.main_pitch_sequence() {
                        let (mean, weight) = self.rack_mean(&flow);
                        let output = self.color_rack.process_with_filters(
                            mean,
                            loudest_voice,
                            &self.filter_fx_masks[1],
                            &mut self.filter_banks,
                            &mut self.taps,
                            weight,
                        );
                        if self.taps.sends(Stage::PostFx) {
                            self.taps.send(Stage::PostFx, output.map(|x| x * weight));
                        }
                        let output = self.taps.mix(Stage::PostFx, mean, output);
                        self.rack_share(&mut flow, mean, output);
                    } else {
                        self.rack_collapse(&mut flow);
                        let input = flow.signal;
                        let output = self.color_rack.process_with_filters(
                            input,
                            loudest_voice,
                            &self.filter_fx_masks[1],
                            &mut self.filter_banks,
                            &mut self.taps,
                            1.0,
                        );
                        flow.signal =
                            self.tap(Stage::PostFx, self.color_rack.aligned_input(), output);
                    }
                }
            }
        }
        // The last chain ends at the output.
        self.rack_collapse(&mut flow);
        self.rack_merge_bypass(&mut flow);
        match flow.chain {
            Chain::Source(source) => {
                let index = strip_index(source);
                peaks[index] = peaks[index].max(flow.signal[0].abs().max(flow.signal[1].abs()));
                for (total, x) in out.iter_mut().zip(flow.signal) {
                    *total += x;
                }
            }
            Chain::Mixed => {
                for (total, x) in out.iter_mut().zip(flow.signal) {
                    *total += x;
                }
            }
            Chain::None => {}
        }
        if let Some(active) = &mut self.rack {
            active.peaks = peaks;
            active.capture = rack.capture;
            active.capture_initialized = rack.capture_initialized;
        }
        self.finish_frame(out, None, input_gain, source_rate)
    }

    /// A module's output after its Output row: the Chain share of it goes on
    /// down, and the Send goes to its bus.
    #[inline]
    fn tap(&mut self, stage: Stage, input: Stereo, output: Stereo) -> Stereo {
        if self.taps.sends(stage) {
            self.taps.send(stage, output);
        }
        self.taps.mix(stage, input, output)
    }

    /// The Grain engine right under the Sample: grains of the file itself,
    /// note by note, blended with the file by Dry/Wet as a Granular timbre's.
    fn rack_file_grains(&mut self, flow: &mut Flow, file: Stereo, dry_scale: f32) {
        let shared_grains_active = self.live_grain_slots().any(|slot| self.grains[slot].shared);
        let (voice_frames, group_sent, voice_owned, shared_frame) = self.render_grains(false);
        // A live input fills a history rather than a file; for effects only
        // it plays dry.
        let live = self.rack_input();
        let effects_only = live && self.live_input.as_ref().is_some_and(|i| i.effects_only);
        let norm = self.grain_norm(effects_only);
        let input_gain = self.input_gain;
        let shared_fallback = self.file_grain_fallback();
        let feedback = self.modulated.params.grain_feedback;
        if feedback > 0.0 || self.grain_feedback.output != [0.0; 2] {
            let mut grains = shared_frame;
            for i in bits(self.sounding) {
                let weight = self.voices[i].value.max(0.0);
                for c in 0..2 {
                    grains[c] += voice_frames[i][c] * weight;
                }
            }
            self.grain_feedback.feed(grains.map(|x| x * norm), feedback);
            if !live && feedback > 0.0 && !self.frozen {
                let center = self.center();
                self.grain_feedback.write(center);
            }
        }
        self.send_groups(group_sent, norm * input_gain);
        let (dry_amount, grain_amount) = if effects_only {
            (dry_scale, 0.0)
        } else {
            (dry_scale * (1.0 - self.smooth_mix), norm * self.smooth_mix)
        };
        let grain_chain = self.taps.chain(Stage::Grains);
        let (chain_dry, chain_grain) = if grain_chain == 1.0 {
            (dry_amount, grain_amount)
        } else {
            (
                dry_scale + (dry_amount - dry_scale) * grain_chain,
                grain_amount * grain_chain,
            )
        };
        let sends = self.taps.sends(Stage::Grains);
        let mut send = [0.0; 2];
        let mut tap = [0.0; 2];
        let mut grain_notes = [[0.0; 2]; VOICES];
        let source = file;
        let masks = self.filter_masks;
        flow.shared_active = shared_fallback || shared_grains_active;
        flow.shared_sub_gain = if flow.shared_active {
            self.voices.iter().map(|v| v.value).fold(0.0, f32::max)
        } else {
            0.0
        };
        let shared_activity = f32::from(flow.shared_active);
        let shared_aftergrains = filter_slots::route_chain_with_voice_gain(
            &mut self.filter_banks,
            masks[FilterPosition::AfterGrains as usize],
            shared_frame,
            Instance::Shared,
            Some((&mut self.taps, norm * input_gain * shared_activity)),
            Some(flow.shared_sub_gain),
        )
        .map(|x| x * shared_activity);
        for i in bits(self.sounding) {
            let value = self.voices[i].value;
            if value <= 0.0 {
                continue;
            }
            flow.note_sub_gain[i] =
                f32::from(self.voices[i].granular() || voice_owned & (1 << i) != 0);
            let frame = filter_slots::route_chain_with_voice_gain(
                &mut self.filter_banks,
                masks[FilterPosition::AfterGrains as usize],
                if voice_owned & (1 << i) != 0 {
                    voice_frames[i]
                } else {
                    [0.0; 2]
                },
                Instance::Voice(i),
                Some((
                    &mut self.taps,
                    norm * input_gain * value * flow.note_sub_gain[i],
                )),
                Some(flow.note_sub_gain[i]),
            )
            .map(|x| x * flow.note_sub_gain[i]);
            let independent_file_lane =
                shared_fallback && self.voices[i].main_pitch && !self.voices[i].granular;
            let dry = if independent_file_lane {
                0.0
            } else if self.voices[i].granular() {
                chain_dry
            } else {
                0.0
            };
            grain_notes[i] = if voice_owned & (1 << i) != 0 {
                frame.map(|x| x * norm * input_gain)
            } else {
                [0.0; 2]
            };
            flow.notes[i] =
                std::array::from_fn(|c| (source[c] * dry + frame[c] * chain_grain) * input_gain);
            for c in 0..2 {
                tap[c] += frame[c] * norm * input_gain * value;
            }
            if sends && !independent_file_lane {
                for c in 0..2 {
                    send[c] +=
                        (source[c] * dry_amount + frame[c] * grain_amount) * input_gain * value;
                }
            }
        }
        if sends {
            self.taps.send(Stage::Grains, send);
        }
        for c in 0..2 {
            tap[c] += shared_aftergrains[c] * norm * input_gain;
        }
        if shared_fallback {
            let shared_input = file.map(|x| x * input_gain);
            let shared_output = std::array::from_fn(|c| {
                shared_input[c] * (1.0 - self.smooth_mix)
                    + shared_aftergrains[c] * norm * input_gain * self.smooth_mix
            });
            flow.shared = self.tap(Stage::Grains, shared_input, shared_output);
        } else if shared_frame != [0.0; 2] || shared_aftergrains != [0.0; 2] {
            let shared = flow.shared;
            let output = std::array::from_fn(|c| {
                shared[c] + shared_aftergrains[c] * norm * input_gain * self.smooth_mix
            });
            flow.shared = self.tap(Stage::Grains, shared, output);
        }
        flow.per_note = true;
        flow.grains = Some(tap);
        flow.grain_notes = Some(grain_notes);
    }

    /// A Grain engine anywhere else: grains of a live history of its chain,
    /// as a Synth's Grain engine reads its voices.
    fn rack_history_grains(&mut self, flow: &mut Flow) {
        let separate = flow.per_note && self.main_pitch_sequence();
        let individual = separate && self.individual_history();
        let notes = flow.notes;
        let shared = flow.shared;
        let shared_was_active = flow.shared_active;
        let shared_sub_gain = flow.shared_sub_gain;
        let mut legacy = shared;
        let mut legacy_active = shared_was_active;
        if separate {
            for i in bits(self.sounding) {
                let voice = self.voices[i];
                if (!voice.main_pitch || !voice.granular) && voice.value > 0.0 {
                    legacy_active = true;
                    for c in 0..2 {
                        legacy[c] += notes[i][c] * voice.value;
                    }
                }
            }
        }
        self.rack_collapse(flow);
        let input = flow.signal;
        let Some(live) = &mut self.live_input else {
            return;
        };
        let epoch = live.range_epoch();
        // Grain feedback enters the history the grains read, as Clouds does.
        let fed = self.grain_feedback.output;
        let history_input = if individual { legacy } else { input };
        let recorded = if fed == [0.0; 2] {
            history_input
        } else {
            std::array::from_fn(|c| history_input[c] + fed[c])
        };
        live.push_with_display(recorded, input, input, self.frozen);
        let shift = (live.range_epoch() - epoch) as f64;
        if shift != 0.0 {
            self.frozen_center -= shift;
            for slot in self.live_grain_slots() {
                self.grains[slot].position -= shift;
                self.grains[slot].anchor -= shift;
            }
        }
        let shared_grains_active = self.live_grain_slots().any(|slot| self.grains[slot].shared);
        let (voice_frames, group_sent, voice_owned, shared_frame) = self.render_grains(true);
        let history_grains = self.mixed_grains(&voice_frames, shared_frame, true);
        let norm = self.grain_norm(false);
        if individual {
            self.capture_sequence_histories(&notes, &voice_frames, norm);
        }
        let feedback = self.modulated.params.grain_feedback;
        if feedback > 0.0 || self.grain_feedback.output != [0.0; 2] {
            self.grain_feedback.feed(
                if individual {
                    shared_frame
                } else {
                    history_grains
                }
                .map(|x| x * norm),
                feedback,
            );
        }
        self.send_groups(group_sent, norm);
        if separate {
            let mut grain_notes = [[0.0; 2]; VOICES];
            let mut sent = [0.0; 2];
            let sends = self.taps.sends(Stage::Grains);
            flow.shared_active =
                legacy_active || self.main_pitch_grain_bindings().2 || shared_grains_active;
            let shared_activity = f32::from(flow.shared_active);
            flow.shared_sub_gain = self
                .voices
                .iter()
                .filter(|voice| !voice.main_pitch || !voice.granular)
                .map(|voice| voice.value)
                .fold(shared_sub_gain, f32::max);
            let shared_grain = filter_slots::route_chain_with_voice_gain(
                &mut self.filter_banks,
                self.filter_masks[FilterPosition::AfterGrains as usize],
                shared_frame,
                Instance::Shared,
                Some((&mut self.taps, norm * shared_activity)),
                Some(flow.shared_sub_gain),
            )
            .map(|x| x * norm * shared_activity);
            let shared_output = std::array::from_fn(|c| {
                legacy[c] * (1.0 - self.smooth_mix) + shared_grain[c] * self.smooth_mix
            });
            let mut shared_bypass = [0.0; 2];
            for i in bits(self.sounding) {
                let value = self.voices[i].value;
                if value <= 0.0 {
                    continue;
                }
                let owned = voice_owned & (1 << i) != 0;
                let private = self.voices[i].main_pitch && self.voices[i].granular;
                let activity = f32::from(owned || private);
                flow.note_sub_gain[i] = activity;
                let frame = if owned { voice_frames[i] } else { [0.0; 2] };
                let grain = filter_slots::route_chain_with_voice_gain(
                    &mut self.filter_banks,
                    self.filter_masks[FilterPosition::AfterGrains as usize],
                    frame,
                    Instance::Voice(i),
                    Some((&mut self.taps, norm * value * activity)),
                    Some(activity),
                )
                .map(|x| x * norm * activity);
                grain_notes[i] = grain;
                let dry = if private { notes[i] } else { [0.0; 2] };
                let output = std::array::from_fn(|c| {
                    dry[c] * (1.0 - self.smooth_mix) + grain[c] * self.smooth_mix
                });
                if sends {
                    for c in 0..2 {
                        sent[c] += output[c] * value;
                    }
                }
                flow.notes[i] = self.taps.mix(Stage::Grains, dry, output);
                if flow.split {
                    for (c, bypass) in shared_bypass.iter_mut().enumerate() {
                        if private {
                            flow.notes[i][c] += flow.bypass_notes[i][c];
                        } else {
                            *bypass += flow.bypass_notes[i][c] * value;
                        }
                    }
                }
            }
            if sends {
                self.taps.send(Stage::Grains, sent);
            }
            // One shared grain cloud is already post-VCA. It must not be
            // copied into each note and multiplied by each note's envelope.
            flow.shared = self.tap(
                Stage::Grains,
                legacy.map(|x| x * shared_activity),
                shared_output.map(|x| x * shared_activity),
            );
            for (output, bypass) in flow.shared.iter_mut().zip(shared_bypass) {
                *output += bypass;
            }
            flow.per_note = true;
            flow.grain_notes = Some(grain_notes);
            flow.grains = Some(history_grains.map(|x| x * norm));
            flow.split = false;
            flow.bypass_notes.fill([0.0; 2]);
            return;
        }
        flow.shared_active |= shared_grains_active;
        let grains = filter_slots::route_chain(
            &mut self.filter_banks,
            self.filter_masks[FilterPosition::AfterGrains as usize],
            history_grains,
            Instance::Shared,
            Some((&mut self.taps, norm)),
        );
        let mix = self.smooth_mix;
        let output = std::array::from_fn(|c| input[c] * (1.0 - mix) + grains[c] * norm * mix);
        flow.grains = Some(grains.map(|x| x * norm));
        flow.signal = self.tap(Stage::Grains, input, output);
        self.rack_merge_bypass(flow);
    }

    /// Each advanced group's grains to its bus, before Dry/Wet.
    #[inline]
    fn send_groups(&mut self, sent: [Stereo; crate::grain_groups::GROUPS], gain: f32) {
        let sends = self.taps.group_sends();
        for (group, sent) in sent.iter().enumerate() {
            if sends & 1 << group != 0 {
                self.taps.send_group(group, sent.map(|x| x * gain));
            }
        }
    }
}
