//! Fixed-buffer publication on the worklet; JSON and knob metadata on the worker.
use super::*;
use sad_pet::dsp::telemetry::RealtimeView;
struct Edits<'a>(&'a [(u64, sad_pet::sequence::Edit)]);
struct Parameters<'a>(&'a [(u64, String, f32)]);
impl Serialize for Parameters<'_> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = s.serialize_map(Some(self.0.len()))?;
        for (_, key, value) in self.0 {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl Serialize for Edits<'_> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut list = s.serialize_seq(Some(self.0.len()))?;
        for (_, edit) in self.0 {
            list.serialize_element(edit)?;
        }
        list.end()
    }
}

/// A Rack's EQs as modulated: those it holds (bit n for EQ n + 1) only.
struct Equalizers<'a>(&'a [sad_pet::eq::Equalizer; sad_pet::eq::MAX_EQS], u16);
impl Serialize for Equalizers<'_> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut list = s.serialize_seq(Some(self.1.count_ones() as usize))?;
        for (i, eq) in self.0.iter().enumerate() {
            if self.1 & 1 << i != 0 {
                list.serialize_element(eq)?;
            }
        }
        list.end()
    }
}

struct DrumEdits<'a>(&'a [(u64, sad_pet::drums::Edit)]);
impl Serialize for DrumEdits<'_> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut list = s.serialize_seq(Some(self.0.len()))?;
        for (_, edit) in self.0 {
            list.serialize_element(edit)?;
        }
        list.end()
    }
}
#[derive(Serialize)]
struct Configuration<'a> {
    preset: &'a Preset,
    name: &'a str,
    id: &'a Option<String>,
    seconds: f64,
    frames: usize,
    region: sad_pet::dsp::Region,
    fades: sad_pet::dsp::Fades,
    live: bool,
}
#[derive(Deserialize)]
struct OwnedConfiguration {
    preset: Preset,
    name: String,
    id: Option<String>,
    seconds: f64,
    frames: usize,
    region: sad_pet::dsp::Region,
    fades: sad_pet::dsp::Fades,
    live: bool,
}
#[derive(Serialize)]
struct Packet<'a> {
    source_generation: u32,
    context: &'a sad_pet::timbres::Context,
    parameter_edits: Parameters<'a>,
    sequence_edits: Edits<'a>,
    drum_edits: DrumEdits<'a>,
    revision: u64,
    view: &'a RealtimeView,
    waveform: &'a [[f32; 2]],
    configuration: Option<Configuration<'a>>,
    grains: &'a [sad_pet::dsp::GrainView],
    color: &'a [sad_pet::color::ColorSlot],
    pre_color: &'a [sad_pet::color::ColorSlot],
    rack_eqs: Equalizers<'a>,
}
#[derive(Deserialize)]
struct OwnedPacket {
    #[serde(default)]
    source_generation: u32,
    #[serde(default)]
    context: sad_pet::timbres::Context,
    parameter_edits: Value,
    sequence_edits: Vec<sad_pet::sequence::Edit>,
    #[serde(default)]
    drum_edits: Vec<sad_pet::drums::Edit>,
    revision: u64,
    view: RealtimeView,
    waveform: Vec<[f32; 2]>,
    configuration: Option<OwnedConfiguration>,
    grains: Vec<sad_pet::dsp::GrainView>,
    color: Vec<sad_pet::color::ColorSlot>,
    pre_color: Vec<sad_pet::color::ColorSlot>,
    #[serde(default)]
    rack_eqs: Vec<sad_pet::eq::Equalizer>,
}

#[derive(Default)]
struct DecodedPart {
    preset: Preset,
    region: sad_pet::dsp::Region,
    state: Value,
}
struct Decoder {
    rate: u32,
    revision: u64,
    parts: std::collections::BTreeMap<u32, DecodedPart>,
    input: Vec<u8>,
    output: Vec<u8>,
}
impl Decoder {
    fn new(rate: u32) -> Self {
        Self {
            rate: rate.clamp(8_000, 384_000),
            revision: 0,
            parts: [(0, DecodedPart::default())].into(),
            input: Vec::new(),
            output: Vec::new(),
        }
    }
    fn import(&mut self, delta: bool) -> Result<Value, String> {
        let packet: OwnedPacket = rmp_serde::from_slice(&self.input).map_err(|e| e.to_string())?;
        let context = &packet.context;
        if !context.timbres.is_empty() && context.revision >= self.revision {
            if !context.timbres.iter().any(|part| part.id == context.active) {
                return Err("Unknown timbre".into());
            }
            self.parts
                .retain(|id, _| context.timbres.iter().any(|part| part.id == *id));
            for part in &context.timbres {
                self.parts.entry(part.id).or_default();
            }
            self.revision = context.revision;
        }
        let part = self
            .parts
            .get_mut(&context.active)
            .ok_or("Obsolete timbre snapshot")?;
        if part.state.is_null() && packet.configuration.is_none() {
            return Err("Telemetry configuration is missing".into());
        }
        decode_snapshot(
            packet,
            delta,
            self.rate,
            &mut part.preset,
            &mut part.region,
            &mut part.state,
        )
    }
}
thread_local! {
    static DECODER: RefCell<Option<Decoder>> = const { RefCell::new(None) };
}
fn decoder<T>(f: impl FnOnce(&mut Decoder) -> T) -> T {
    DECODER.with(|d| {
        f(d.borrow_mut()
            .as_mut()
            .expect("Call sad_telemetry_init first"))
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_telemetry_init(rate: u32) {
    DECODER.with(|d| *d.borrow_mut() = Some(Decoder::new(rate)));
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_telemetry_input(length: usize) -> *mut u8 {
    decoder(|d| {
        if length > MAX_INPUT {
            return std::ptr::null_mut();
        }
        d.input.resize(length, 0);
        d.input.as_mut_ptr()
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_telemetry_output() -> *const u8 {
    decoder(|d| d.output.as_ptr())
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_telemetry_output_len() -> usize {
    decoder(|d| d.output.len())
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_telemetry_import(delta: u32) -> u32 {
    decoder(|d| {
        let result = d
            .import(delta != 0)
            .and_then(|value| serde_json::to_vec(&value).map_err(|e| e.to_string()));
        match result {
            Ok(bytes) => {
                d.output = bytes;
                200
            }
            Err(error) => {
                d.output = serde_json::to_vec(&json!({"error":error})).unwrap();
                400
            }
        }
    })
}

impl Host {
    pub(super) fn publish_snapshot(&mut self) -> usize {
        let Some(slot) = self.unacked_peaks.iter().position(Option::is_none) else {
            return 0;
        };
        self.engine.realtime_view(&mut self.view);
        for (_, peak, input_peak) in self.unacked_peaks.iter().flatten() {
            for c in 0..2 {
                self.view.peak[c] = self.view.peak[c].max(peak[c]);
                self.view.input_peak[c] = self.view.input_peak[c].max(input_peak[c]);
            }
        }
        let sample = self.engine.shown_sample();
        let changed = self.sent_revision != Some(self.config_revision);
        let packet = Packet {
            source_generation: self.file_generation,
            context: &self.context,
            parameter_edits: Parameters(&self.parameter_edits),
            sequence_edits: Edits(&self.sequence_edits),
            drum_edits: DrumEdits(&self.drum_edits),
            revision: self.config_revision,
            view: &self.view,
            grains: &self.view.grains[..self.view.grain_count],
            color: &self.view.color[..self.view.color_len],
            pre_color: &self.view.pre_color[..self.view.pre_color_len],
            rack_eqs: Equalizers(
                &self.view.modulated_rack_eqs,
                self.engine.preset.rack.as_ref().map_or(0, |r| {
                    r.equalizers.iter().fold(0, |mask, eq| {
                        mask | 1u16
                            .checked_shl(u32::from(eq.id).wrapping_sub(1))
                            .unwrap_or(0)
                    })
                }),
            ),
            waveform: if changed || self.engine.live_input.is_some() {
                &self.view.waveform[..self.view.waveform_len]
            } else {
                &[]
            },
            configuration: changed.then_some(Configuration {
                preset: &self.engine.preset,
                // A Rack shows the file, or the history its Grain engine reads.
                name: if self.engine.preset.rack.is_some() {
                    &sample.name
                } else if self.engine.preset.drums.is_some() {
                    "Drum machine"
                } else if self.engine.preset.synth.is_some() {
                    sad_pet::dsp::synth::NAME
                } else {
                    &sample.name
                },
                id: &sample.library_id,
                seconds: self.engine.shown_duration(),
                frames: self.engine.shown_frames(),
                region: self.engine.region,
                fades: self.engine.fades,
                live: self.engine.live_input.is_some(),
            }),
        };
        let mut cursor = std::io::Cursor::new(&mut self.snapshot_buffer[..]);
        match packet.serialize(&mut rmp_serde::Serializer::new(&mut cursor).with_struct_map()) {
            Ok(()) => {
                self.sent_revision = Some(self.config_revision);
                self.snapshot_sequence = self.snapshot_sequence.wrapping_add(1).max(1);
                self.unacked_peaks[slot] =
                    Some((self.snapshot_sequence, self.view.peak, self.view.input_peak));
                self.sent_edit_revisions[slot] = Some((self.snapshot_sequence, self.edit_revision));
                cursor.position() as usize
            }
            Err(_) => {
                // Keep unreported peaks if a future schema exceeds the buffer.
                for c in 0..2 {
                    self.engine.peak[c] = self.engine.peak[c].max(self.view.peak[c]);
                }
                if let Some(input) = &mut self.engine.live_input {
                    for c in 0..2 {
                        input.peak[c] = input.peak[c].max(self.view.input_peak[c]);
                    }
                }
                0
            }
        }
    }

    pub(super) fn import_snapshot(&mut self, delta: bool) -> Result<Value, String> {
        let packet = rmp_serde::from_slice(&self.input).map_err(|e| e.to_string())?;
        decode_snapshot(
            packet,
            delta,
            self.rate,
            &mut self.engine.preset,
            &mut self.engine.region,
            &mut self.snapshot_state,
        )
    }
}

// The decoder only needs preset metadata and the published view. Audio buffers,
// filters, effect histories and synthesis state stay in the audio owner.
fn decode_snapshot(
    packet: OwnedPacket,
    delta: bool,
    rate: u32,
    preset: &mut Preset,
    region: &mut sad_pet::dsp::Region,
    state_cache: &mut Value,
) -> Result<Value, String> {
    let configuration_changed = packet.configuration.is_some();
    let mut preset_changed = configuration_changed
        || !packet.sequence_edits.is_empty()
        || !packet.drum_edits.is_empty()
        || packet
            .parameter_edits
            .as_object()
            .is_some_and(|v| !v.is_empty());
    if let Some(mut config) = packet.configuration {
        // Imported UI data must not discard the ownership metadata for
        // this host's prepared DSP buffers, even if it is reused later.
        std::mem::swap(
            &mut config.preset.imported_effects,
            &mut preset.imported_effects,
        );
        *preset = config.preset;
        *region = config.region;
        *state_cache = json!({
            "version":env!("CARGO_PKG_VERSION"),"instance_id":"browser",
            "startup_preset_index":sad_pet::startup::INDEX,
            "sample_rate":rate,"audio_device":"Web Audio","audio_error":null,"audio_enabled":false,
            "sample_name":config.name,"sample_id":config.id,"sample_seconds":config.seconds,"sample_frames":config.frames,
            "region":config.region,"fades":config.fades,"input":{"active":config.live},
            "recording":{"active":false,"id":null,"seconds":0,"available":false,"error":null},
            "cpu_percent":0,"underruns":0,
        });
    }
    let mut v = packet.view;
    preset.sequencer.edit(&packet.sequence_edits);
    if let Some(drums) = &mut preset.drums {
        drums.edit(&packet.drum_edits);
    }
    let parameters = sad_pet::parameters::parse(&packet.parameter_edits)?;
    sad_pet::parameters::apply(preset, &parameters);
    if v.grain_count > sad_pet::dsp::MAX_GRAINS
        || v.color_len > sad_pet::color::MAX_COLOR_SLOTS
        || v.pre_color_len > sad_pet::color::MAX_COLOR_SLOTS
        || v.note_count > v.notes.len()
        || v.chord_len > 32
        || v.waveform_len > 640
    {
        return Err("Invalid telemetry counts".into());
    }
    if packet.grains.len() != v.grain_count
        || packet.color.len() != v.color_len
        || packet.pre_color.len() != v.pre_color_len
    {
        return Err("Invalid telemetry lengths".into());
    }
    v.grains[..v.grain_count].copy_from_slice(&packet.grains);
    v.color[..v.color_len].copy_from_slice(&packet.color);
    v.pre_color[..v.pre_color_len].copy_from_slice(&packet.pre_color);
    for eq in packet.rack_eqs {
        if let Some(to) = v
            .modulated_rack_eqs
            .get_mut(usize::from(eq.id).wrapping_sub(1))
        {
            *to = eq;
        }
    }
    if v.chord_len > 32 {
        return Err("Invalid chord count".into());
    }
    preset_changed |= preset.performance.chord.as_slice() != &v.chord[..v.chord_len];
    preset.performance.chord.clear();
    preset
        .performance
        .chord
        .extend_from_slice(&v.chord[..v.chord_len]);
    for (i, step) in v.step_edits.iter().enumerate() {
        if let Some(step) = step {
            preset_changed |= preset.performance.steps[i] != *step;
            preset.performance.steps[i] = *step;
        }
    }
    if preset_changed {
        state_cache["preset"] = json!(preset);
    }
    if !packet.waveform.is_empty() {
        state_cache["waveform"] = json!(packet.waveform);
    }
    // Cache only configuration, never clone the full preset on a meter tick.
    let mut state = if !delta || configuration_changed {
        state_cache.clone()
    } else {
        json!({})
    };
    if delta && !configuration_changed {
        if preset_changed {
            state["preset"] = state_cache["preset"].clone();
        }
        if !packet.waveform.is_empty() {
            state["waveform"] = state_cache["waveform"].clone();
        }
    }
    state["input"]["seconds"] = json!(v.input_seconds);
    state["input"]["peak"] = json!(v.input_peak);
    state["playing"] = json!(v.playing);
    state["midi_gate"] = json!(v.midi_gate);
    state["frozen"] = json!(v.frozen);
    state["bypass"] = json!(v.bypass);
    state["center"] = json!(v.center);
    state["grains"] = json!(&v.grains[..v.grain_count]);
    state["active_grains"] = json!(v.grain_count);
    state["notes"] = json!(&v.notes[..v.note_count]);
    state["step_entry"] = json!(v.step_entry);
    state["step_edits"] = json!(v.step_edits);
    state["performance"] = json!({"mode":preset.performance.mode,"step":v.step,"arp_index":v.arp_index,"chord":&v.chord[..v.chord_len],"gate":v.gate,"beat":v.beat});
    state["sequencer"] = json!(v.sequencer);
    state["timbres"] = json!(packet.context);
    state["source_generation"] = json!(packet.source_generation);
    state["drums"] = json!(v.drums);
    state["rack_peaks"] = json!(v.rack_peaks);
    if let Some(history) = preset
        .rack
        .as_ref()
        .filter(|r| !r.has(sad_pet::rack::Module::Sample))
        .and_then(|r| r.history())
    {
        let frames = history.buffer_frames(rate);
        state["sample_frames"] = json!(frames);
        state["sample_seconds"] = json!(frames as f64 / rate as f64);
    } else if preset.rack.is_some() {
        // A Rack with a Sample shows its file.
    } else if let Some(drums) = &preset.drums {
        state["playing"] = json!(v.drums.playing);
        // The drum history length follows its Buffer setting, not the file.
        let frames = drums.buffer_frames(rate);
        state["sample_frames"] = json!(frames);
        state["sample_seconds"] = json!(frames as f64 / rate as f64);
    }
    if let Some(synth) = preset.synth.as_ref().filter(|_| preset.rack.is_none()) {
        let frames = synth.history().buffer_frames(rate);
        state["sample_frames"] = json!(frames);
        state["sample_seconds"] = json!(frames as f64 / rate as f64);
    }
    state["lfos"] = json!(v.lfos);
    state["envs"] = json!(v.envs);
    state["matrix_values"] = json!(v.matrix);
    let knobs = json!(sad_pet::dsp::telemetry::knob_states_from_view(preset, &v));
    if delta && !configuration_changed {
        let mut changes = serde_json::Map::new();
        let previous = state_cache["knobs"].as_object();
        for (key, value) in knobs.as_object().unwrap() {
            if previous.and_then(|p| p.get(key)) != Some(value) {
                changes.insert(key.clone(), value.clone());
            }
        }
        if let Some(previous) = previous {
            for key in previous.keys() {
                if knobs.get(key).is_none() {
                    changes.insert(key.clone(), Value::Null);
                }
            }
        }
        state["knob_changes"] = Value::Object(changes);
    } else {
        state["knobs"] = knobs.clone();
    }
    state_cache["knobs"] = knobs;
    state["peak"] = json!(v.peak);
    state["resynth"] = json!({"bands":v.resynth_bands,"frequencies":v.resynth_frequencies});
    state["source_fx"] = json!({"pending":v.source_fx_pending,"recipe":sad_pet::dsp::source_fx::Recipe::new(*region, preset.source_color_chain.first().copied(), preset.params.bpm, preset.params.sample_root_note),"error":if v.source_fx_error {Some("Could not prepare Source FX")}else{None}});
    state["gain_reduction"] = json!(
        v.gain_reduction
            .iter()
            .filter(|(id, _)| *id != 0)
            .map(|(id, db)| (id.to_string(), *db))
            .collect::<std::collections::BTreeMap<_, _>>()
    );
    if v.source_gain_reduction.0 != 0 {
        state["gain_reduction"][v.source_gain_reduction.0.to_string()] =
            json!(v.source_gain_reduction.1);
    }
    Ok(json!({"revision":packet.revision,"delta":delta,"state":state}))
}

#[unsafe(no_mangle)]
pub extern "C" fn sad_snapshot() -> usize {
    registry(|r| {
        let Some(slot) = r.unacked.iter().position(Option::is_none) else {
            return 0;
        };
        let index = r.selected;
        let length = r.parts[index].host.publish_snapshot();
        if length > 0 {
            r.sequence = r.sequence.wrapping_add(1).max(1);
            r.unacked[slot] = Some((
                r.sequence,
                r.parts[index].meta.id,
                r.parts[index].host.snapshot_sequence,
            ));
            r.snapshot_part = index;
        }
        length
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_snapshot_pointer() -> *const u8 {
    registry(|r| {
        r.parts[r.snapshot_part.min(r.parts.len() - 1)]
            .host
            .snapshot_buffer
            .as_ptr()
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_snapshot_capacity() -> usize {
    registry(|r| {
        r.parts
            .iter()
            .map(|p| p.host.snapshot_buffer.len())
            .max()
            .unwrap_or(131072)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_snapshot_import() -> u32 {
    import_snapshot(false)
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_snapshot_import_delta() -> u32 {
    import_snapshot(true)
}
fn import_snapshot(delta: bool) -> u32 {
    registry(|r| {
        let index = r.index();
        let bytes = std::mem::take(&mut r.parts[index].host.input);
        #[derive(Deserialize)]
        struct Envelope {
            #[serde(default)]
            context: sad_pet::timbres::Context,
        }
        let result = (|| {
            let envelope: Envelope = rmp_serde::from_slice(&bytes).map_err(|e| e.to_string())?;
            let id = envelope.context.active;
            r.adopt_context(envelope.context)?;
            let index = r
                .parts
                .iter()
                .position(|p| p.meta.id == id)
                .ok_or("Obsolete timbre snapshot")?;
            r.parts[index].host.input = bytes;
            r.parts[index].host.import_snapshot(delta)
        })()
        .and_then(|v| serde_json::to_vec(&v).map_err(|e| e.to_string()));
        let index = r.index();
        r.parts[index].host.finish(result)
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_snapshot_sequence() -> u32 {
    registry(|r| r.sequence)
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_snapshot_ack(sequence: u32, success: u32) {
    registry(|r| {
        if let Some(index) = r
            .unacked
            .iter()
            .position(|e| e.is_some_and(|e| e.0 == sequence))
        {
            let (_, id, local) = r.unacked[index].take().unwrap();
            if let Some(part) = r.parts.iter_mut().find(|p| p.meta.id == id) {
                part.host.ack_snapshot(local, success);
            }
        }
    })
}
impl Host {
    pub(super) fn ack_snapshot(&mut self, sequence: u32, success: u32) {
        if let Some(index) = self
            .unacked_peaks
            .iter()
            .position(|p| p.is_some_and(|p| p.0 == sequence))
        {
            let (_, peak, input_peak) = self.unacked_peaks[index].take().unwrap();
            if let Some((_, revision)) = self.sent_edit_revisions[index].take()
                && success != 0
            {
                self.sequence_edits.retain(|(r, _)| *r > revision);
                self.drum_edits.retain(|(r, _)| *r > revision);
                self.parameter_edits.retain(|(r, _, _)| *r > revision);
            }
            if success == 0 {
                self.sent_revision = None;
                for (c, value) in peak.iter().enumerate() {
                    self.engine.peak[c] = self.engine.peak[c].max(*value);
                }
                if let Some(input) = &mut self.engine.live_input {
                    for (c, value) in input_peak.iter().enumerate() {
                        input.peak[c] = input.peak[c].max(*value);
                    }
                }
            }
        }
    }
}
