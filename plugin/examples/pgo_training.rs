//! Profile-guided optimization training run for the native engine (see
//! plugin/scripts/pgo.py). It drives the C ABI that the JUCE wrapper calls
//! through plug-in, effect and multi-timbre app scenes, then sweeps every
//! filter model, Color FX type, modulator and oscillator shape: code the run
//! never reaches is optimized as cold, so every DSP path should appear here.
//! The input is libm-free (a triangle plus LCG noise; `f32::sin` differs
//! between arm64 and x86_64), so every target trains on the same signal. Each
//! line ends with a hash of the rendered PCM, which the build compares between
//! the instrumented and the optimized engine of one target (arm64 and x86_64
//! renders of these scenes still differ from each other).
//!   cargo run -p sad-pet-plugin --release --example pgo_training
use sad_pet::{
    color::{ColorParameter, ColorSlot},
    performance::{Direction, Mode as PlayMode},
    preset::{
        Destination, EnvMode, FilterModel, FilterPosition, FilterRouting, FilterSlot, FilterType,
        Lfo, Preset, ReverbType, SaturatorType, Waveform,
    },
    resynth::{self, CrossInput, FoldShape, Interaction, Model, Oscillator},
};
use sad_pet_plugin::*;
use serde_json::{Value, json};
use std::time::Instant;

const BLOCK: usize = 256;
/// Host parameters moved every few blocks, like DAW automation lanes.
const AUTOMATED: [&str; 8] = [
    "position",
    "grains",
    "pitch",
    "filter",
    "filter_resonance",
    "cloud",
    "mix",
    "stereo_spread",
];
const COLOR_TYPES: [SaturatorType; 13] = [
    SaturatorType::Distortion,
    SaturatorType::Reducer,
    SaturatorType::Crusher,
    SaturatorType::Flanger,
    SaturatorType::Chorus,
    SaturatorType::Binaural,
    SaturatorType::Reverb,
    SaturatorType::LcrDelay,
    SaturatorType::ModDelay,
    SaturatorType::Compressor,
    SaturatorType::Waveshaper,
    SaturatorType::SuperEcho,
    SaturatorType::AlphaVerb,
];
const DESTINATIONS: [Destination; 17] = [
    Destination::Position,
    Destination::Pitch,
    Destination::GrainSize,
    Destination::Grains,
    Destination::Filter,
    Destination::Mix,
    Destination::Pan,
    Destination::Reverb,
    Destination::GrainShape,
    Destination::Detune,
    Destination::Cloud,
    Destination::StereoSpread,
    Destination::FilterResonance,
    Destination::Filter2,
    Destination::Filter2Resonance,
    Destination::Saturation,
    Destination::Volume,
];

#[derive(Clone, Copy, PartialEq)]
enum Wrapper {
    Instrument,
    Effect,
    App,
}

/// One wrapper instance as a host runs it: controller, audio side and the
/// host's parameter values.
struct Host {
    control: *mut Controller,
    audio: *mut Audio,
    values: Vec<f32>,
    automated: Vec<usize>,
    rate: u32,
    standalone: bool,
    input: bool,
    notes: bool,
    frame: usize,
    blocks: usize,
    beat: f64,
    rng: u32,
    hash: u64,
    seconds: f64,
}

impl Host {
    fn new(wrapper: Wrapper, rate: u32) -> Self {
        let control = match wrapper {
            Wrapper::Instrument => sad_control_new(false),
            Wrapper::Effect => sad_control_new(true),
            Wrapper::App => sad_control_new_standalone(),
        };
        let specs = sad_pet::parameters::specs();
        let mut host = Self {
            control,
            audio: unsafe { sad_control_prepare(control, rate) },
            values: vec![0.0; specs.len()],
            automated: AUTOMATED
                .iter()
                .filter_map(|id| specs.iter().position(|s| s.id == *id))
                .collect(),
            rate,
            standalone: wrapper == Wrapper::App,
            input: wrapper != Wrapper::Instrument,
            notes: wrapper != Wrapper::Effect,
            frame: 0,
            blocks: 0,
            beat: 0.0,
            rng: 0x1234_5678,
            hash: 0xcbf2_9ce4_8422_2325,
            seconds: 0.0,
        };
        assert_eq!(host.automated.len(), AUTOMATED.len());
        if wrapper == Wrapper::App {
            host.request("/api/transport", json!({"playing":true,"midi_gate":false}));
        }
        host.refresh();
        host
    }

    /// One editor bridge request, answered as the WebView sees it.
    fn request(&mut self, path: &str, data: Value) -> Value {
        let method = if data.is_null() { "GET" } else { "POST" };
        let body = serde_json::to_vec(&json!({"path":path,"method":method,"data":data})).unwrap();
        let bytes = unsafe { sad_control_request(self.control, body.as_ptr(), body.len()) };
        let slice = unsafe { std::slice::from_raw_parts(bytes.data, bytes.len) };
        let value: Value = serde_json::from_slice(slice).unwrap();
        unsafe { sad_bytes_free(bytes.data, bytes.len) };
        assert_eq!(value["status"], 200, "{path}: {value}");
        value["data"].clone()
    }

    /// Reads the host parameters back after an editor change, as the wrapper does.
    fn refresh(&mut self) {
        let len = self.values.len();
        unsafe { sad_control_get_parameters(self.control, self.values.as_mut_ptr(), len) };
    }

    fn preset(&mut self, preset: &Preset) {
        self.request("/api/preset", serde_json::to_value(preset).unwrap());
        self.refresh();
    }

    fn active_preset(&mut self) -> Preset {
        serde_json::from_value(self.request("/api/preset", Value::Null)).unwrap()
    }

    fn midi(&mut self, bytes: &[u8]) {
        unsafe { sad_audio_midi(self.audio, bytes.as_ptr(), bytes.len()) };
    }

    fn random(&mut self) -> f32 {
        self.rng = self.rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (self.rng >> 8) as f32 / (1u32 << 24) as f32
    }

    fn block(&mut self, frames: usize) {
        let n = self.blocks;
        self.blocks += 1;
        let (mut left, mut right) = ([0.0; 1024], [0.0; 1024]);
        for i in 0..frames {
            let t = ((self.frame + i) % 200) as f32 / 100.0;
            let triangle = if t < 1.0 { t } else { 2.0 - t } - 0.5;
            let noise = self.random() - 0.5;
            let v = if self.input {
                triangle * 0.3 + noise * 0.02
            } else {
                0.0
            };
            left[i] = v;
            right[i] = v * 0.9;
        }
        self.frame += frames;
        let note = 48 + (n / 47 % 5) as u8 * 3;
        if self.notes && n.is_multiple_of(47) {
            self.midi(&[0x90, note, 100, 0x90, note + 7, 90, 0x90, note + 12, 70]);
        }
        if self.notes && n % 47 == 30 {
            self.midi(&[0x80, note, 0, 0x80, note + 7, 0, 0x80, note + 12, 0]);
        }
        if n.is_multiple_of(61) {
            let bend = (n * 13 % 16384) as u16;
            let (low, high) = ((bend & 0x7f) as u8, (bend >> 7) as u8);
            self.midi(&[0xe0, low, high, 0xb0, 1, (n % 128) as u8]);
        }
        if n.is_multiple_of(16) {
            let specs = sad_pet::parameters::specs();
            for (k, &index) in self.automated.iter().enumerate() {
                let phase = ((n / 16 + k * 5) % 24) as f32 / 12.0;
                let shape = if phase < 1.0 { phase } else { 2.0 - phase };
                let spec = &specs[index];
                self.values[index] = spec.min + (spec.max - spec.min) * (0.2 + 0.6 * shape);
            }
        }
        let start = Instant::now();
        unsafe {
            sad_audio_begin(
                self.audio,
                self.values.as_ptr(),
                self.values.len(),
                120.0,
                self.beat,
                !self.standalone,
                true,
                frames,
            );
            sad_audio_process(
                self.audio,
                left.as_ptr(),
                right.as_ptr(),
                left.as_mut_ptr(),
                right.as_mut_ptr(),
                frames,
            );
            sad_audio_end(self.audio, frames);
        }
        self.seconds += start.elapsed().as_secs_f64();
        self.beat += frames as f64 / self.rate as f64 * 2.0;
        for i in 0..frames {
            for byte in [left[i], right[i]]
                .into_iter()
                .flat_map(|v| v.to_bits().to_le_bytes())
            {
                self.hash = (self.hash ^ byte as u64).wrapping_mul(0x0100_0000_01b3);
            }
        }
        // The editor polls about 30 times a second; polling drains the feedback queues.
        if n.is_multiple_of(8) {
            self.request("/api/timbres", Value::Null);
        }
        if n.is_multiple_of(64) {
            self.request("/api/state", Value::Null);
        }
    }

    fn run(&mut self, seconds: f32) {
        for _ in 0..(seconds * self.rate as f32) as usize / BLOCK {
            self.block(BLOCK);
        }
    }

    fn report(self, name: &str, hashed: bool) {
        let hash = if hashed {
            format!("{:016x}", self.hash)
        } else {
            "timing-dependent".into()
        };
        let block_us = self.seconds * 1e6 / self.blocks as f64;
        println!(
            "{name}: rate={} blocks={} block_us={block_us:.2} hash={hash}",
            self.rate, self.blocks
        );
        unsafe {
            sad_audio_free(self.audio);
            sad_control_free(self.control);
        }
    }
}

fn timbre_id(context: &Value, kind: &str) -> u64 {
    let timbres = context["timbres"].as_array().unwrap();
    timbres.iter().find(|t| t["kind"] == kind).unwrap()["id"]
        .as_u64()
        .unwrap()
}

/// A factory preset with the grain engine fully open and a note-following filter.
fn grains(index: usize) -> Preset {
    let mut p = Preset::builtin(index % 5);
    p.params.grains = 1.0;
    p.params.grain_size = 0.9;
    p.params.cloud = 0.6;
    p.params.filter_routing = FilterRouting::Serial;
    p.params.vcf_type = FilterType::BandPass;
    p.params.filter = 0.65;
    p.params.filter2 = 0.8;
    p.params.in_gain = 3.1;
    p.params.out_gain = -1.3;
    p.lfos[0].destination = Destination::Filter;
    p.lfos[0].depth = 0.3;
    p
}

/// A Color FX slot with every algorithm setting at a seeded position.
fn color(kind: SaturatorType, id: u32, seed: u32) -> ColorSlot {
    let mut slot = ColorSlot {
        kind,
        id,
        mix: 0.6,
        ..ColorSlot::default()
    };
    let mut rng = seed.wrapping_mul(0x9e37_79b9) | 1;
    for &parameter in &ColorParameter::ALL[1..] {
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;
        *slot.parameter_mut(parameter) = (rng >> 8) as f32 / (1u32 << 24) as f32;
    }
    // Freeze holds the buffer forever; leave it to the dedicated passes.
    slot.settings.super_echo_freeze = 0.0;
    slot.settings.alpha_verb_freeze = 0.0;
    slot.settings.reverb_type = [
        ReverbType::PlateS,
        ReverbType::PlateL,
        ReverbType::Room,
        ReverbType::Reverse,
    ][seed as usize % 4];
    slot
}

/// Four oscillator layers, one combination of shape, mode and interaction per pass.
fn oscillators(layers: &mut [Oscillator; 4], pass: usize, first_reference: u8) {
    for (j, o) in layers.iter_mut().enumerate() {
        o.enabled = true;
        o.waveform = resynth::Waveform::ALL[(pass * 4 + j) % 8];
        o.mode = resynth::Mode::ALL[(pass / 2 + j) % 4];
        o.model = if (pass + j) % 3 == 2 {
            Model::Wavefolder
        } else {
            Model::Radias
        };
        o.fold_shape = FoldShape::ALL[(pass + 2 * j) % 5];
        o.interaction = Interaction::ALL[(pass + j) % 4];
        o.interaction_source =
            first_reference + ((pass + j) % (6 - first_reference as usize)) as u8;
        o.cross_source = ((pass + 2 * j) % 5) as u8;
        o.cross_input = CrossInput::ALL[(pass + j) % 3];
        o.control1 = 0.2 + 0.2 * j as f32;
        o.control2 = 0.9 - 0.1 * pass as f32;
        o.transpose = (j as f32 - 1.5) * 7.0;
        o.fine = (pass as f32 - 3.5) * 5.0;
        o.level = 0.6;
        o.pan = 0.2 + 0.2 * j as f32;
    }
}

fn main() {
    // The three wrappers with their first-open scene.
    let mut h = Host::new(Wrapper::Instrument, 48_000);
    h.run(4.0);
    h.report("plugin_instrument", true);
    let mut h = Host::new(Wrapper::Effect, 44_100);
    h.run(2.0);
    for mode in ["effects", "grains"] {
        h.request("/api/input", json!({"active":true,"mode":mode}));
        h.run(1.5);
    }
    h.report("plugin_effect", true);
    let mut h = Host::new(Wrapper::App, 48_000);
    h.run(3.0);
    // Stopped: Resynth tails, then silence.
    h.request("/api/transport", json!({"playing":false}));
    h.run(1.5);
    h.report("app_default", true);

    // Drums, a synth and a bus fed by sends, then a saved and restored scene.
    let mut h = Host::new(Wrapper::App, 48_000);
    h.request("/api/timbres/add", json!({"kind":"drums"}));
    h.request("/api/timbres/add", json!({"kind":"synth"}));
    let context = h.request("/api/timbres/add", json!({"kind":"bus"}));
    let (drums, synth) = (timbre_id(&context, "drums"), timbre_id(&context, "synth"));
    let mut sends = [0.0; 8];
    sends[0] = 0.5;
    h.request("/api/timbres/update", json!({"id":0,"sends":sends}));
    sends[0] = 0.3;
    h.request("/api/timbres/update", json!({"id":synth,"sends":sends}));
    h.request("/api/timbres/select", json!({"id":synth}));
    h.run(4.0);
    h.request("/api/timbres/update", json!({"id":drums,"muted":true}));
    h.run(1.0);
    let saved = unsafe { sad_control_save(h.control) };
    let restored = unsafe { sad_control_restore(h.control, saved.data, saved.len) };
    unsafe { sad_bytes_free(saved.data, saved.len) };
    assert!(restored, "scene restore");
    h.run(1.0);
    h.report("app_multi", true);

    let mut h = Host::new(Wrapper::Instrument, 48_000);
    for index in 0..=sad_pet::startup::INDEX {
        h.preset(&Preset::builtin(index));
        h.run(1.0);
    }
    h.report("factory_presets", true);

    let mut h = Host::new(Wrapper::Instrument, 48_000);
    for (i, model) in FilterModel::ALL.into_iter().enumerate() {
        let mut p = grains(i);
        p.params.filter_model = model;
        p.params.filter2_model = FilterModel::ALL[(i + 7) % FilterModel::ALL.len()];
        p.params.filter_routing = [
            FilterRouting::Single,
            FilterRouting::Serial,
            FilterRouting::Parallel,
        ][i % 3];
        p.params.vcf_type = FilterType::ALL[i % 4];
        p.params.vcf2_type = FilterType::ALL[(i + 1) % 4];
        p.params.filter_drive = (i % 4) as f32 * 0.25;
        p.params.filter_morph = (i % 3) as f32 * 0.4;
        p.params.filter2_drive = 0.3;
        p.params.filter2_morph = 0.4;
        p.params.filter_keytrack = 0.5;
        h.preset(&p);
        h.run(0.5);
    }
    h.report("filters", true);

    // Added filters at every position, after Resynth layers.
    let mut h = Host::new(Wrapper::Instrument, 48_000);
    for i in 0..FilterPosition::ALL.len() {
        let mut p = sad_pet::startup::preset();
        p.filter_slots = (0..3)
            .map(|k| FilterSlot {
                id: k as u32 + 1,
                position: FilterPosition::ALL[(i + 3 * k) % FilterPosition::ALL.len()],
                model: FilterModel::ALL[(i * 3 + k) % FilterModel::ALL.len()],
                kind: FilterType::ALL[(i + k) % 4],
                drive: 0.3,
                morph: 0.5,
                env_depth: 0.4,
                keytrack: 0.5,
                ..FilterSlot::default()
            })
            .collect();
        h.preset(&p);
        h.run(0.5);
    }
    h.report("filter_slots", true);

    // Every Color FX type after and before the grains, then full racks.
    let mut h = Host::new(Wrapper::Instrument, 48_000);
    for (i, &kind) in COLOR_TYPES.iter().enumerate() {
        let mut p = grains(i);
        p.color_chain = vec![color(kind, 1, i as u32)];
        p.pre_color_chain = vec![color(COLOR_TYPES[(i + 5) % 13], 2, i as u32 + 40)];
        h.preset(&p);
        h.run(0.5);
    }
    for pass in 0..2 {
        let mut p = grains(pass + 3);
        p.color_chain = (0..11)
            .map(|i| color(COLOR_TYPES[i], i as u32 + 1, (pass * 11 + i) as u32))
            .collect();
        p.pre_color_chain = (0..11)
            .map(|i| color(COLOR_TYPES[10 - i], i as u32 + 101, (pass * 7 + i) as u32))
            .collect();
        h.preset(&p);
        h.run(1.0);
    }
    h.report("color_fx", true);

    // The imported firmware effects resample from other rates.
    for rate in [44_100, 96_000] {
        let mut h = Host::new(Wrapper::Effect, rate);
        for pass in 0..3 {
            let mut p = h.active_preset();
            let mut echo = color(SaturatorType::SuperEcho, 1, pass + 7);
            let mut verb = color(SaturatorType::AlphaVerb, 2, pass + 9);
            echo.settings.super_echo_freeze = (pass == 2) as u8 as f32;
            verb.settings.alpha_verb_dirty = (pass == 1) as u8 as f32;
            verb.settings.alpha_verb_freeze = (pass == 2) as u8 as f32;
            p.color_chain = vec![echo];
            p.pre_color_chain = vec![verb];
            h.preset(&p);
            h.run(1.0);
        }
        h.report(&format!("imported_fx_{rate}"), true);
    }

    // LFO shapes, envelope types and play modes on every destination.
    let mut h = Host::new(Wrapper::Instrument, 48_000);
    let shapes = [
        Waveform::Sine,
        Waveform::Triangle,
        Waveform::Saw,
        Waveform::Square,
        Waveform::Random,
        Waveform::SmoothRandom,
        Waveform::Envelope,
    ];
    let play = [
        PlayMode::Keys,
        PlayMode::Hold,
        PlayMode::Arp,
        PlayMode::Sequencer,
        PlayMode::SequencerArp,
    ];
    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::UpDown,
        Direction::Random,
        Direction::AsPlayed,
    ];
    for i in 0..EnvMode::ALL.len() {
        let mut p = grains(i);
        p.lfo_count = sad_pet::preset::MODULATORS as u8;
        p.env_count = sad_pet::preset::MODULATORS as u8;
        for k in 0..sad_pet::preset::MODULATORS {
            p.lfos[k] = Lfo {
                waveform: shapes[(i + k) % shapes.len()],
                rate_hz: 0.5 + k as f32,
                depth: 0.3,
                destination: DESTINATIONS[(i * 8 + k) % DESTINATIONS.len()],
                sync: k % 2 == 1,
                division: 0.5,
            };
            let env = &mut p.envs[k];
            env.mode = EnvMode::ALL[(i + k) % EnvMode::ALL.len()];
            env.destination = DESTINATIONS[(i * 8 + k + 3) % DESTINATIONS.len()];
            env.depth = 0.4;
            env.curve = k as u8;
        }
        p.performance.mode = play[i % play.len()];
        p.performance.direction = directions[i % directions.len()];
        p.performance.octaves = 2;
        p.sequencer.enabled = i % 2 == 0;
        h.preset(&p);
        h.run(0.6);
    }
    h.report("modulation", true);

    // Resynth layers in every shape, mode and interaction, on the app's input.
    let mut h = Host::new(Wrapper::App, 48_000);
    for pass in 0..8 {
        let mut p = sad_pet::startup::preset();
        p.resynth.enabled = true;
        p.resynth.count = 4;
        p.resynth.input = [resynth::Input::Source, resynth::Input::Grains][pass % 2];
        oscillators(&mut p.resynth.oscillators, pass, 0);
        h.preset(&p);
        h.run(0.5);
    }
    h.report("resynth", true);

    let mut h = Host::new(Wrapper::App, 48_000);
    h.request("/api/timbres/add", json!({"kind":"synth"}));
    for pass in 0..8 {
        let mut p = h.active_preset();
        let synth = p.synth.as_mut().unwrap();
        synth.count = 4;
        synth.granular = pass % 2 == 1;
        oscillators(
            &mut synth.oscillators,
            pass,
            sad_pet::synth::FIRST_REFERENCE,
        );
        p.params.filter_model = FilterModel::ALL[pass * 3 % FilterModel::ALL.len()];
        h.preset(&p);
        h.run(0.5);
    }
    h.report("synth", true);

    let mut h = Host::new(Wrapper::App, 48_000);
    h.request("/api/timbres/add", json!({"kind":"drums"}));
    for pass in 0..4 {
        let mut p = h.active_preset();
        let kit = p.drums.as_mut().unwrap();
        kit.granular = pass % 2 == 1;
        kit.swing = pass as f32 * 0.2;
        h.preset(&p);
        for b in 0..120 {
            if b % 9 == 0 {
                h.request(
                    "/api/drums/trigger",
                    json!({"track":b / 9 % 7,"velocity":0.8}),
                );
            }
            h.block(BLOCK);
        }
    }
    h.report("drums", true);

    // Transport and editing: loops, regions, freeze, bypass, time signatures
    // and the block sizes hosts use.
    let mut h = Host::new(Wrapper::Instrument, 96_000);
    let mut p = grains(2);
    p.params.loop_one_bar = true;
    h.preset(&p);
    h.request("/api/region", json!({"start":0.251,"end":0.613}));
    for b in 0..480 {
        let transport = match b {
            100 => json!({"frozen":true}),
            160 => json!({"frozen":false}),
            220 => json!({"bypass":true}),
            260 => json!({"bypass":false}),
            _ => Value::Null,
        };
        if !transport.is_null() {
            h.request("/api/transport", transport);
        }
        if b == 300 {
            h.request("/api/region", json!({"start":0.5,"end":0.5001}));
        }
        if b % 50 == 0 {
            let (numerator, denominator) = [(4, 4), (7, 8), (3, 4)][b / 50 % 3];
            unsafe { sad_audio_time_signature(h.audio, numerator, denominator) };
        }
        h.block([64, 256, 512, 1024, 32, 480][b % 6]);
    }
    h.report("transport", true);

    // Source FX render on a worker thread, so their timing varies; not hashed.
    let mut h = Host::new(Wrapper::Instrument, 48_000);
    for (i, kind) in [
        SaturatorType::Reverb,
        SaturatorType::Chorus,
        SaturatorType::Crusher,
    ]
    .into_iter()
    .enumerate()
    {
        let mut p = grains(i);
        p.source_color_chain = vec![color(kind, 1, i as u32 + 3)];
        h.preset(&p);
        for _ in 0..100 {
            h.block(BLOCK);
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
    h.report("source_fx", false);
}
