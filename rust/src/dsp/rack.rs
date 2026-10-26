//! A Rack timbre's frame: its modules in the order they stand, top to bottom.
//! See `crate::rack` for what each module does. The notes, modulators,
//! sequencer and output stage are the engine's own, as in any other timbre.
use super::{
    CONTROL_INTERVAL, Engine, Stereo, VOICES, bits, fast,
    filter_slots::{self, Instance},
    tables,
};
use crate::{
    preset::FilterPosition,
    rack::{Chain, Module, Plan, STRIP_SOURCES as SOURCES, Settings, Strip, Strips},
    taps::Stage,
};

fn strip_index(source: Module) -> usize {
    SOURCES.iter().position(|&s| s == source).unwrap_or(0)
}

/// What the frame loop keeps of a Rack between frames.
#[derive(Clone, Copy)]
pub(super) struct Runtime {
    plan: Plan,
    /// The Grain engine is on and in some chain.
    grains: bool,
    /// The Grain engine granulates the Sample's file, note by note.
    pub(super) reads_file: bool,
    /// Filter 1 and 2 run a pair per note; otherwise voice 0's pair filters
    /// the chain as one.
    pub(super) filters_per_note: bool,
    /// The strips' gains ramp to their targets so a fader never clicks.
    targets: [Stereo; 3],
    gains: [Stereo; 3],
    /// Each chain's loudest sample since the host last read it.
    peaks: [f32; 3],
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
            filters_per_note: plan
                .iter()
                .any(|step| step.module == Module::Filters && step.per_note),
            targets,
            gains: previous.map_or(targets, |p| p.gains),
            peaks: previous.map_or([0.0; 3], |p| p.peaks),
        }
    }
    /// A solo keeps the soloed chains among those that reach the Mixer.
    fn targets(plan: &Plan, strips: &Strips) -> [Stereo; 3] {
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
    signal: Stereo,
    /// The chain's source, for Resynth's Source input.
    source: Stereo,
    /// The chain's grains, for Resynth's Grains input.
    grains: Option<Stereo>,
}

impl Engine {
    /// The loudest sample of the Sample, Synth and Drums chains since the
    /// peaks were last cleared, after their Mixer strips.
    pub fn rack_peaks(&self) -> Option<[f32; 3]> {
        self.rack.as_ref().map(|rack| rack.peaks)
    }
    pub fn clear_rack_peaks(&mut self) {
        if let Some(rack) = &mut self.rack {
            rack.peaks = [0.0; 3];
        }
    }

    /// The notes' signals summed through their VCA.
    #[inline]
    fn rack_collapse(&self, flow: &mut Flow) {
        if flow.per_note {
            let mut sum = [0.0; 2];
            for i in bits(self.sounding) {
                let value = self.voices[i].value;
                if value <= 0.0 {
                    continue;
                }
                for (total, x) in sum.iter_mut().zip(flow.notes[i]) {
                    *total += x * value;
                }
            }
            flow.signal = sum;
            flow.per_note = false;
        }
    }

    pub(super) fn process_rack_frame(&mut self) -> Stereo {
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
        if self.playing {
            self.tick_events();
        }
        self.sync_history_window();
        let input_gain = self.input_gain;
        let bypass = self.bypass;
        let source_rate = self.sample.sample_rate as f64 / self.sample_rate as f64;
        // The sources render first; the drums keep their clock while off.
        let drums = self.drums.as_mut().map_or([0.0; 2], |d| d.frame());
        let file = if self
            .preset
            .rack
            .as_ref()
            .is_some_and(|r| r.has(Module::Sample))
        {
            let fade = self
                .fades
                .gain(self.cursor, self.sample.sample_rate, &self.file_range);
            let level = self.sample.level_for_rate(source_rate, true);
            self.sample
                .read_region_level(self.cursor, level, &self.file_range)
                .map(|v| v * fade)
        } else {
            [0.0; 2]
        };
        let level = (file[0].abs().max(file[1].abs())).max(drums[0].abs().max(drums[1].abs()));
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
        let Some(rack) = self.rack else {
            return [0.0; 2];
        };
        self.grain_clock += self.grain_step;
        if rack.grains && !bypass && self.grain_density > 0.0 && self.grain_clock >= 1.0 {
            self.grain_clock = self.grain_clock.fract();
            self.spawn();
        }
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
            signal: [0.0; 2],
            source: [0.0; 2],
            grains: None,
        };
        let mut mixer = [0.0; 2];
        let mut out = [0.0; 2];
        let mut peaks = rack.peaks;
        for step in rack.plan.iter() {
            // Bypass leaves only the sources and the Mixer.
            let on =
                step.enabled && (!bypass || step.module.source() || step.module == Module::Mixer);
            if step.module.source() || step.module == Module::Mixer {
                self.rack_collapse(&mut flow);
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
                flow.grains = None;
            }
            if step.chain == Chain::None {
                // Above every source there is nothing to process.
                continue;
            }
            match step.module {
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
                        }
                    }
                }
                Module::Synth => {
                    flow.source = [0.0; 2];
                    let Some(synth) = &mut self.synth else {
                        continue;
                    };
                    synth.begin_frame(self.lfo_values[0]);
                    if !on {
                        continue;
                    }
                    flow.per_note = true;
                    for i in bits(self.sounding) {
                        let voice = &self.voices[i];
                        if voice.value <= 0.0 {
                            continue;
                        }
                        let note = voice.note as f32
                            + self.bend[voice.channel as usize]
                                * self.modulated.params.midi_pitchbend_range;
                        let frame =
                            synth.render(i, voice.generation, note, &self.sample, &self.file_range);
                        flow.notes[i] = frame.map(|x| x * input_gain);
                        for c in 0..2 {
                            flow.source[c] += flow.notes[i][c] * voice.value;
                        }
                    }
                }
                Module::Drums => {
                    flow.source = drums.map(|x| x * input_gain);
                    if on {
                        flow.signal = flow.source;
                    }
                }
                Module::Mixer => {
                    flow.signal = mixer;
                    flow.source = mixer;
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
                            let voice = filter_slots::chain(
                                &mut self.filter_banks,
                                masks[FilterPosition::BeforeFilters as usize],
                                unfiltered,
                                Instance::Voice(i),
                            );
                            let voice = self.filters[i].process(voice);
                            let voice = filter_slots::chain(
                                &mut self.filter_banks,
                                masks[FilterPosition::AfterFilters as usize],
                                voice,
                                Instance::Voice(i),
                            );
                            if sends {
                                for c in 0..2 {
                                    send[c] += voice[c] * value;
                                }
                            }
                            flow.notes[i] = self.taps.mix(Stage::Filters, unfiltered, voice);
                        }
                        if sends {
                            self.taps.send(Stage::Filters, send);
                        }
                    } else {
                        let input = flow.signal;
                        let output = filter_slots::chain(
                            &mut self.filter_banks,
                            masks[FilterPosition::BeforeFilters as usize],
                            input,
                            Instance::Shared,
                        );
                        let output = self.filters[0].process(output);
                        let output = filter_slots::chain(
                            &mut self.filter_banks,
                            masks[FilterPosition::AfterFilters as usize],
                            output,
                            Instance::Shared,
                        );
                        flow.signal = self.tap(Stage::Filters, input, output);
                    }
                }
                Module::Resynth => {
                    self.rack_collapse(&mut flow);
                    let input = flow.signal;
                    let output = self.resynth.process_with_filters(
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
                    );
                    let output = filter_slots::chain(
                        &mut self.filter_banks,
                        masks[FilterPosition::AfterResynth as usize],
                        output,
                        Instance::Shared,
                    );
                    flow.signal = self.tap(Stage::Resynth, input, output);
                }
                Module::SourceFx => {
                    self.rack_collapse(&mut flow);
                    let input = flow.signal;
                    let output = self.source_color.process(input);
                    flow.signal = self.tap(Stage::Source, input, output);
                }
                Module::PreFx => {
                    self.rack_collapse(&mut flow);
                    let input = flow.signal;
                    let output = self.pre_color_rack.process(input);
                    flow.signal = self.tap(Stage::PreFx, input, output);
                }
                Module::PostFx => {
                    self.rack_collapse(&mut flow);
                    let input = flow.signal;
                    let output = self
                        .color_rack
                        .process_with_voice_gain(input, loudest_voice);
                    let output = filter_slots::chain(
                        &mut self.filter_banks,
                        masks[FilterPosition::AfterFx as usize],
                        output,
                        Instance::Shared,
                    );
                    flow.signal = self.tap(Stage::PostFx, input, output);
                }
            }
        }
        // The last chain ends at the output.
        self.rack_collapse(&mut flow);
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
        if let Some(rack) = &mut self.rack {
            rack.peaks = peaks;
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
        let (voice_frames, group_sent) = self.render_grains(false);
        let norm = self.grain_norm();
        let input_gain = self.input_gain;
        let feedback = self.modulated.params.grain_feedback;
        if feedback > 0.0 || self.grain_feedback.output != [0.0; 2] {
            let mut grains = [0.0; 2];
            for i in bits(self.sounding) {
                let weight = self.voices[i].value.max(0.0);
                for c in 0..2 {
                    grains[c] += voice_frames[i][c] * weight;
                }
            }
            self.grain_feedback.feed(grains.map(|x| x * norm), feedback);
            if feedback > 0.0 && !self.frozen {
                let center = self.center();
                self.grain_feedback.write(center);
            }
        }
        self.send_groups(group_sent, norm * input_gain);
        let (dry_amount, grain_amount) =
            (dry_scale * (1.0 - self.smooth_mix), norm * self.smooth_mix);
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
        let source = file;
        let masks = self.filter_masks;
        for i in bits(self.sounding) {
            let value = self.voices[i].value;
            if value <= 0.0 {
                continue;
            }
            let frame = filter_slots::chain(
                &mut self.filter_banks,
                masks[FilterPosition::AfterGrains as usize],
                voice_frames[i],
                Instance::Voice(i),
            );
            flow.notes[i] = std::array::from_fn(|c| {
                (source[c] * chain_dry + frame[c] * chain_grain) * input_gain
            });
            for c in 0..2 {
                tap[c] += frame[c] * norm * input_gain * value;
            }
            if sends {
                for c in 0..2 {
                    send[c] +=
                        (source[c] * dry_amount + frame[c] * grain_amount) * input_gain * value;
                }
            }
        }
        if sends {
            self.taps.send(Stage::Grains, send);
        }
        flow.per_note = true;
        flow.grains = Some(tap);
    }

    /// A Grain engine anywhere else: grains of a live history of its chain,
    /// as a Synth's Grain engine reads its voices.
    fn rack_history_grains(&mut self, flow: &mut Flow) {
        self.rack_collapse(flow);
        let input = flow.signal;
        let Some(live) = &mut self.live_input else {
            return;
        };
        let epoch = live.range_epoch();
        // Grain feedback enters the history the grains read, as Clouds does.
        let fed = self.grain_feedback.output;
        let recorded = if fed == [0.0; 2] {
            input
        } else {
            std::array::from_fn(|c| input[c] + fed[c])
        };
        live.push_with_meter(recorded, input, self.frozen);
        let shift = (live.range_epoch() - epoch) as f64;
        if shift != 0.0 {
            self.frozen_center -= shift;
            for slot in self.live_grain_slots() {
                self.grains[slot].position -= shift;
            }
        }
        let (voice_frames, group_sent) = self.render_grains(true);
        let norm = self.grain_norm();
        let feedback = self.modulated.params.grain_feedback;
        if feedback > 0.0 || self.grain_feedback.output != [0.0; 2] {
            self.grain_feedback
                .feed(voice_frames[0].map(|x| x * norm), feedback);
        }
        self.send_groups(group_sent, norm);
        let grains = filter_slots::chain(
            &mut self.filter_banks,
            self.filter_masks[FilterPosition::AfterGrains as usize],
            voice_frames[0],
            Instance::Shared,
        );
        let mix = self.smooth_mix;
        let output = std::array::from_fn(|c| input[c] * (1.0 - mix) + grains[c] * norm * mix);
        flow.grains = Some(grains.map(|x| x * norm));
        flow.signal = self.tap(Stage::Grains, input, output);
    }

    /// The grains' level at the current overlap.
    #[inline]
    fn grain_norm(&self) -> f32 {
        if self.overlap > 2.0 {
            tables::inverse_sqrt(self.overlap - 1.0)
        } else {
            1.0
        }
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
