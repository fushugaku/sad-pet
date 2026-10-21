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
        let sample = self.engine.source_sample();
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
            waveform: if changed || self.engine.live_input.is_some() {
                &self.view.waveform[..self.view.waveform_len]
            } else {
                &[]
            },
            configuration: changed.then_some(Configuration {
                preset: &self.engine.preset,
                name: &sample.name,
                id: &sample.library_id,
                seconds: sample.duration(),
                frames: sample.len(),
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

    pub(super) fn import_snapshot(&mut self) -> Result<Value, String> {
        let packet: OwnedPacket = rmp_serde::from_slice(&self.input).map_err(|e| e.to_string())?;
        if let Some(config) = packet.configuration {
            self.engine.preset = config.preset;
            self.engine.region = config.region;
            self.snapshot_state = json!({
                "version":env!("CARGO_PKG_VERSION"),"instance_id":"browser",
                "startup_preset_index":sad_pet::startup::INDEX,
                "sample_rate":self.rate,"audio_device":"Web Audio","audio_error":null,"audio_enabled":false,
                "sample_name":config.name,"sample_id":config.id,"sample_seconds":config.seconds,"sample_frames":config.frames,
                "region":config.region,"fades":config.fades,"input":{"active":config.live},
                "recording":{"active":false,"id":null,"seconds":0,"available":false,"error":null},
                "cpu_percent":0,"underruns":0,
            });
        }
        let mut v = packet.view;
        self.engine.preset.sequencer.edit(&packet.sequence_edits);
        if let Some(drums) = &mut self.engine.preset.drums {
            drums.edit(&packet.drum_edits);
        }
        let parameters = sad_pet::parameters::parse(&packet.parameter_edits)?;
        sad_pet::parameters::apply(&mut self.engine.preset, &parameters);
        if v.grain_count > sad_pet::dsp::MAX_GRAINS
            || v.color_len > sad_pet::color::MAX_COLOR_SLOTS
            || v.pre_color_len > sad_pet::color::MAX_COLOR_SLOTS
            || v.note_count > 8
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
        self.engine.preset.performance.chord.clear();
        self.engine
            .preset
            .performance
            .chord
            .extend_from_slice(&v.chord[..v.chord_len]);
        for (i, step) in v.step_edits.iter().enumerate() {
            if let Some(step) = step {
                self.engine.preset.performance.steps[i] = *step;
            }
        }
        let mut state = self.snapshot_state.clone();
        if !packet.waveform.is_empty() {
            state["waveform"] = json!(packet.waveform);
        }
        state["input"]["seconds"] = json!(v.input_seconds);
        state["input"]["peak"] = json!(v.input_peak);
        state["playing"] = json!(v.playing);
        state["midi_gate"] = json!(v.midi_gate);
        state["frozen"] = json!(v.frozen);
        state["bypass"] = json!(v.bypass);
        state["preset"] = json!(self.engine.preset);
        state["center"] = json!(v.center);
        state["grains"] = json!(&v.grains[..v.grain_count]);
        state["active_grains"] = json!(v.grain_count);
        state["notes"] = json!(&v.notes[..v.note_count]);
        state["step_entry"] = json!(v.step_entry);
        state["step_edits"] = json!(v.step_edits);
        state["performance"] = json!({"mode":self.engine.preset.performance.mode,"step":v.step,"arp_index":v.arp_index,"chord":&v.chord[..v.chord_len],"gate":v.gate,"beat":v.beat});
        state["sequencer"] = json!(v.sequencer);
        state["timbres"] = json!(packet.context);
        state["source_generation"] = json!(packet.source_generation);
        state["drums"] = json!(v.drums);
        if self.engine.preset.drums.is_some() {
            state["playing"] = json!(v.drums.playing);
        }
        state["lfos"] = json!(v.lfos);
        state["envs"] = json!(v.envs);
        state["matrix_values"] = json!(v.matrix);
        state["knobs"] = json!(self.engine.knob_states_from_view(&v));
        state["peak"] = json!(v.peak);
        state["resynth"] = json!({"bands":v.resynth_bands,"frequencies":v.resynth_frequencies});
        state["source_fx"] = json!({"pending":v.source_fx_pending,"recipe":sad_pet::dsp::source_fx::Recipe::from_engine(&self.engine),"error":if v.source_fx_error {Some("Could not prepare Source FX")}else{None}});
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
        self.snapshot_state = state.clone();
        Ok(json!({"revision":packet.revision,"state":state}))
    }
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
            r.parts[index].host.import_snapshot()
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
