//! Fixed-buffer publication on the worklet; JSON and knob metadata on the worker.
use super::*;
use sad_pet::dsp::telemetry::RealtimeView;

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
                "sample_rate":self.rate,"audio_device":"Web Audio","audio_error":null,"audio_enabled":false,
                "sample_name":config.name,"sample_id":config.id,"sample_seconds":config.seconds,"sample_frames":config.frames,
                "region":config.region,"fades":config.fades,"input":{"active":config.live},
                "recording":{"active":false,"id":null,"seconds":0,"available":false,"error":null},
                "cpu_percent":0,"underruns":0,
            });
        }
        let mut v = packet.view;
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
    host(Host::publish_snapshot)
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_snapshot_pointer() -> *const u8 {
    host(|h| h.snapshot_buffer.as_ptr())
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_snapshot_import() -> u32 {
    host(|h| {
        let result = h
            .import_snapshot()
            .and_then(|v| serde_json::to_vec(&v).map_err(|e| e.to_string()));
        h.finish(result)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn sad_snapshot_sequence() -> u32 {
    host(|h| h.snapshot_sequence)
}
#[unsafe(no_mangle)]
pub extern "C" fn sad_snapshot_ack(sequence: u32, success: u32) {
    host(|h| {
        if let Some(index) = h
            .unacked_peaks
            .iter()
            .position(|p| p.is_some_and(|p| p.0 == sequence))
        {
            let (_, peak, input_peak) = h.unacked_peaks[index].take().unwrap();
            if success == 0 {
                h.sent_revision = None;
                for (c, value) in peak.iter().enumerate() {
                    h.engine.peak[c] = h.engine.peak[c].max(*value);
                }
                if let Some(input) = &mut h.engine.live_input {
                    for (c, value) in input_peak.iter().enumerate() {
                        input.peak[c] = input.peak[c].max(*value);
                    }
                }
            }
        }
    });
}
