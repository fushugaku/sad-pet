//! Synth engine voices: up to four Rustias OSC1/Wavefolder layers on each
//! engine voice, before that voice's filters, envelopes and VCA. Settings and
//! tables are shared; every voice owns its phases, slews and decimator. The
//! voice sum enters the live history like a drum machine, so the granular
//! stage follows the VCA. Rendering allocates nothing.
use super::{
    SampleBuffer, VOICES, radias_oscillator, radias_wavefolder,
    resynth::{Decimator, PcmLoop},
    sample::LiveInput,
};
use crate::{
    Stereo,
    resynth::{Mode, Model, Oscillator, Waveform},
    synth::{FIRST_REFERENCE, MAX_OSCILLATORS, Settings},
};
use radias_synth_application::primary::PrimaryProgram;
use radias_synth_domain::{
    Sample,
    controller_primary::PrimaryControl,
    pitch::{PhaseIncrement, PitchCode},
    primary_oscillator::PrimaryOscillator,
    waveform::WaveformTable,
};
use std::{f32::consts::PI, ops::Range};

/// Rustias samples are Q30, so a full-scale wave reaches 2.0. One oscillator
/// at level 1.0, centred, peaks near 0.5.
const OUTPUT: f32 = 0.25;

struct Carrier {
    controller: radias_oscillator::Controller,
    oscillator: PrimaryOscillator,
    folder: radias_wavefolder::Wavefolder,
    increment: PhaseIncrement,
    code: PitchCode,
    audible: bool,
    pcm_phase: f64,
    pcm_rate: f64,
    pcm_level: usize,
}
impl Default for Carrier {
    fn default() -> Self {
        Self {
            controller: Default::default(),
            oscillator: Default::default(),
            folder: Default::default(),
            increment: PhaseIncrement(0),
            code: PitchCode::new(60 * 256).unwrap(),
            audible: true,
            pcm_phase: 0.0,
            pcm_rate: 1.0,
            pcm_level: 0,
        }
    }
}

struct Voice {
    carriers: [Carrier; MAX_OSCILLATORS],
    /// Unmixed outputs before level and pan, for Ring/Sync and Cross links.
    /// Earlier layers give the current sample, self/later the previous one.
    taps: [f32; MAX_OSCILLATORS],
    wraps: [bool; MAX_OSCILLATORS],
    initialized: [bool; MAX_OSCILLATORS],
    decimator: Decimator,
    /// The engine voice's note-on count these carriers were reset for.
    generation: u32,
    /// Bits of the semitone pitch the carriers are tuned to.
    pitch: Option<u32>,
}
impl Default for Voice {
    fn default() -> Self {
        Self {
            carriers: std::array::from_fn(|_| Carrier::default()),
            taps: [0.0; MAX_OSCILLATORS],
            wraps: [false; MAX_OSCILLATORS],
            initialized: [false; MAX_OSCILLATORS],
            decimator: Decimator::new(),
            generation: 0,
            pitch: None,
        }
    }
}
impl Voice {
    /// A new note starts every layer from its initial phase and slew.
    fn reset(&mut self, generation: u32) {
        *self = Self {
            generation,
            ..Self::default()
        };
    }
}

#[derive(Clone, Copy)]
struct Layer {
    settings: Oscillator,
    program: PrimaryProgram,
    fold_controls: [f64; 2],
    pan: Stereo,
    gain: f32,
    target_gain: f32,
}
impl Default for Layer {
    fn default() -> Self {
        Self {
            settings: Oscillator::default(),
            program: PrimaryProgram::default(),
            fold_controls: [0.0; 2],
            pan: [1.0; 2],
            gain: 0.0,
            target_gain: 0.0,
        }
    }
}

struct State {
    sample_rate: f32,
    oversample: usize,
    voices: [Voice; VOICES],
    layers: [Layer; MAX_OSCILLATORS],
    /// A layer renders when audible or referenced by an audible layer.
    needed: [bool; MAX_OSCILLATORS],
    tables: radias_oscillator::Tables,
    waveform: WaveformTable,
    settings: Option<Settings>,
    /// Engine transposition (semitones) added to every voice.
    transpose: f32,
    pcm_source: (usize, u32, usize, u32),
    root_hz: f64,
    /// Advances with every change of carrier controls, including pitch.
    epoch: u64,
    frame: u64,
    current: u64,
    lfo: i16,
    output: Stereo,
}

pub struct Runtime {
    state: Box<State>,
    /// Prepared off audio. Also retires a previous live input on a type change.
    pub live: Option<LiveInput>,
}
impl Runtime {
    pub fn new(sample_rate: u32) -> Self {
        let sample_rate = sample_rate.clamp(8_000, 384_000);
        Self {
            state: Box::new(State {
                sample_rate: sample_rate as f32,
                oversample: if sample_rate < 96_000 { 2 } else { 1 },
                voices: std::array::from_fn(|_| Voice::default()),
                layers: [Layer::default(); MAX_OSCILLATORS],
                needed: [false; MAX_OSCILLATORS],
                tables: radias_oscillator::Tables::new(),
                waveform: radias_oscillator::waveform_table(),
                settings: None,
                transpose: 0.0,
                pcm_source: (0, 0, 0, 0),
                root_hz: 261.6255653005986,
                epoch: 0,
                frame: 0,
                current: 0,
                lfo: 0,
                output: [0.0; 2],
            }),
            live: None,
        }
    }
    /// A replacement preset keeps sounding voices, slews and history.
    pub(super) fn inherit(&mut self, old: &mut Self) {
        if self.state.sample_rate == old.state.sample_rate {
            std::mem::swap(&mut self.state, &mut old.state);
        }
        std::mem::swap(&mut self.live, &mut old.live);
    }
    /// The post-VCA voice sum of the previous frame, the history's next input.
    pub(super) fn output(&self) -> Stereo {
        self.state.output
    }
    pub(super) fn set_output(&mut self, output: Stereo) {
        self.state.output = output;
    }
    /// Control-rate configuration; equal settings leave every carrier alone.
    pub(super) fn configure(
        &mut self,
        settings: &Settings,
        transpose: f32,
        sample: &SampleBuffer,
        root: f32,
    ) {
        let s = &mut *self.state;
        let pcm_source = (
            sample as *const SampleBuffer as usize,
            sample.sample_rate,
            sample.len(),
            root.to_bits(),
        );
        if s.settings.as_ref() == Some(settings)
            && s.transpose == transpose
            && s.pcm_source == pcm_source
        {
            return;
        }
        let retune = s.transpose != transpose
            || s.pcm_source != pcm_source
            || s.settings.is_none_or(|old| {
                old.oscillators
                    .iter()
                    .zip(&settings.oscillators)
                    .any(|(a, b)| a.transpose != b.transpose || a.fine != b.fine)
            });
        // Modulation can reach values the editor never stores.
        let oscillators = settings.oscillators.map(|mut o| {
            if o.waveform == Waveform::AudioIn {
                o.waveform = Waveform::Sine;
            }
            o.interaction_source = o
                .interaction_source
                .clamp(FIRST_REFERENCE, FIRST_REFERENCE + MAX_OSCILLATORS as u8 - 1);
            o.cross_source = o.cross_source.min(MAX_OSCILLATORS as u8);
            o
        });
        let count = (settings.count as usize).clamp(1, MAX_OSCILLATORS);
        let enabled = oscillators[..count]
            .iter()
            .filter(|o| o.enabled)
            .count()
            .max(1) as f32;
        for (i, layer) in s.layers.iter_mut().enumerate() {
            let o = oscillators[i];
            layer.target_gain = if i < count && o.enabled {
                o.level / enabled.sqrt()
            } else {
                0.0
            };
            let selection = if o.waveform.analog() {
                o.waveform.code() + o.mode.code() * 16
            } else {
                o.waveform.code()
            };
            layer.program = PrimaryProgram {
                selection,
                control: PrimaryControl {
                    control1: (o.control1 * 127.0).round() as u8,
                    control2: (o.control2 * 127.0).round() as u8,
                    ..Default::default()
                },
            };
            layer.fold_controls = [
                (o.control1 as f64 * 127.0).round() / 127.0,
                (((o.control2 as f64 * 127.0).round() - 64.0) / 64.0).clamp(-1.0, 1.0),
            ];
            layer.pan = [
                (o.pan * PI * 0.5).cos() * std::f32::consts::SQRT_2,
                (o.pan * PI * 0.5).sin() * std::f32::consts::SQRT_2,
            ];
            layer.settings = o;
        }
        s.needed = std::array::from_fn(|i| {
            i < count
                && oscillators[i].enabled
                && (oscillators[i].level > 0.0 || s.layers[i].gain > 0.0)
        });
        // Keep an unmixed layer running while another one uses its signal or
        // phase. The fixed number of passes also closes forward links/cycles.
        for _ in 0..MAX_OSCILLATORS {
            for i in 0..MAX_OSCILLATORS {
                if !s.needed[i] {
                    continue;
                }
                let o = oscillators[i];
                let references = [
                    (o.interaction != crate::resynth::Interaction::Off)
                        .then(|| (o.interaction_source - FIRST_REFERENCE) as usize),
                    (o.model == Model::Radias && o.mode == Mode::Cross && o.cross_source > 0)
                        .then(|| o.cross_source as usize - 1),
                ];
                for reference in references.into_iter().flatten() {
                    if reference < count && oscillators[reference].enabled {
                        s.needed[reference] = true;
                    }
                }
            }
        }
        if retune {
            for voice in &mut s.voices {
                voice.pitch = None;
            }
        }
        s.root_hz = 440.0 * 2.0_f64.powf((root as f64 - 69.0) / 12.0);
        s.settings = Some(*settings);
        s.transpose = transpose;
        s.pcm_source = pcm_source;
        s.epoch = s.epoch.wrapping_add(1);
    }
    /// Advance shared slews once per host frame, before rendering voices.
    pub(super) fn begin_frame(&mut self, lfo: f32) {
        let s = &mut *self.state;
        if s.frame.is_multiple_of(32) {
            s.lfo = (lfo.clamp(-1.0, 1.0) * 32767.0) as i16;
        }
        for layer in &mut s.layers {
            layer.gain += (layer.target_gain - layer.gain).clamp(-1.0 / 128.0, 1.0 / 128.0);
        }
        s.current = s.frame;
        s.frame = s.frame.wrapping_add(1);
    }
    /// Render one engine voice before its filter. `generation` changes with
    /// every note-on of that voice; `note` includes the voice's pitch bend.
    pub(super) fn render(
        &mut self,
        index: usize,
        generation: u32,
        note: f32,
        sample: &SampleBuffer,
        range: &Range<usize>,
    ) -> Stereo {
        let s = &mut *self.state;
        let voice = &mut s.voices[index];
        if voice.generation != generation {
            voice.reset(generation);
        }
        let pitch = note + s.transpose;
        if voice.pitch != Some(pitch.to_bits()) {
            tune(
                voice,
                &s.layers,
                pitch,
                s.sample_rate,
                s.oversample,
                sample,
                s.root_hz,
            );
            voice.pitch = Some(pitch.to_bits());
            s.epoch = s.epoch.wrapping_add(1);
        }
        let mut output = [0.0; 2];
        for sub in 0..s.oversample {
            let mut wet = [0.0; 2];
            // Slew once per four host frames, including when carriers run at 2x.
            let frame = if s.current & 3 == 3 && sub + 1 == s.oversample {
                s.current
            } else {
                s.current & !3
            };
            for id in 0..MAX_OSCILLATORS {
                let layer = &s.layers[id];
                let carrier = &mut voice.carriers[id];
                if (!s.needed[id] && layer.gain == 0.0) || !carrier.audible {
                    voice.taps[id] = 0.0;
                    voice.wraps[id] = false;
                    continue;
                }
                let o = layer.settings;
                let reference = (o.interaction_source - FIRST_REFERENCE) as usize;
                let sync = o.interaction.sync() && voice.wraps[reference];
                let modulator = if o.cross_source == 0 {
                    0.0
                } else {
                    voice.taps[o.cross_source as usize - 1]
                };
                let (mut raw, wrapped) = match o.waveform {
                    Waveform::Pcm => {
                        if sync {
                            carrier.pcm_phase = 0.0;
                        }
                        let pcm = PcmLoop::new(sample, range, o);
                        let wrapped = carrier.pcm_phase + carrier.pcm_rate >= pcm.length;
                        let x = pcm.read_at(
                            &mut carrier.pcm_phase,
                            carrier.pcm_rate,
                            carrier.pcm_level,
                            sample,
                            range,
                        );
                        (x, wrapped)
                    }
                    _ if o.model == Model::Wavefolder => {
                        let phase = carrier.folder.phase;
                        let x = carrier.folder.next(
                            carrier.increment.0 as f64 / 4294967296.0,
                            o.waveform,
                            o.fold_shape,
                            layer.fold_controls,
                            sync,
                        );
                        ([x; 2], carrier.folder.phase < phase)
                    }
                    _ => {
                        let lfo = if o.waveform.analog()
                            && matches!(o.mode, Mode::Wave | Mode::Cross)
                            && o.control2 != 0.0
                        {
                            s.lfo
                        } else {
                            0
                        };
                        let modulator = Sample(
                            (modulator as f64 * i32::MAX as f64)
                                .clamp(i32::MIN as f64, i32::MAX as f64)
                                as i32,
                        );
                        let phase = carrier.oscillator.phase;
                        let x = carrier
                            .controller
                            .next_with_sync(
                                &mut carrier.oscillator,
                                layer.program,
                                carrier.increment,
                                carrier.code,
                                lfo,
                                [0; 2],
                                modulator,
                                &s.tables,
                                &s.waveform,
                                frame,
                                index * MAX_OSCILLATORS + id,
                                sync,
                                s.epoch,
                            )
                            .0 as f32
                            / 1073741824.0;
                        let wrapped = radias_synth_domain::secondary_control::SecondaryModulation {
                            ring: false,
                            sync: true,
                        }
                        .wraps(carrier.oscillator.phase, phase);
                        ([x; 2], wrapped)
                    }
                };
                voice.wraps[id] = voice.initialized[id] && (sync || wrapped);
                voice.initialized[id] = true;
                if o.interaction.ring() {
                    let reference = voice.taps[reference].clamp(-1.0, 1.0);
                    raw = raw.map(|x| x * reference);
                }
                voice.taps[id] = (raw[0] + raw[1]) * 0.5;
                for c in 0..2 {
                    wet[c] += raw[c] * layer.gain * layer.pan[c] * OUTPUT;
                }
            }
            if s.oversample == 2 {
                wet = voice.decimator.next(wet);
            }
            output = wet;
        }
        output
    }
}

fn tune(
    voice: &mut Voice,
    layers: &[Layer; MAX_OSCILLATORS],
    pitch: f32,
    sample_rate: f32,
    oversample: usize,
    sample: &SampleBuffer,
    root_hz: f64,
) {
    let rate = sample_rate as f64 * oversample as f64;
    for (layer, carrier) in layers.iter().zip(&mut voice.carriers) {
        let o = layer.settings;
        let note = pitch as f64 + o.transpose as f64 + o.fine as f64 * 0.01;
        let hz = 440.0 * 2.0_f64.powf((note - 69.0) / 12.0);
        carrier.audible = hz < sample_rate as f64 * 0.45;
        carrier.increment =
            PhaseIncrement((hz.min(rate * 0.45) / rate * 4294967296.0).round() as u32);
        carrier.code =
            PitchCode::new((note.clamp(0.0, 127.996) * 256.0).round().min(32767.0) as u16).unwrap();
        carrier.pcm_rate = hz / root_hz * sample.sample_rate as f64 / rate;
        carrier.pcm_level = sample.level_for_rate(carrier.pcm_rate, true);
    }
}

#[derive(Default)]
pub struct Prepared(pub Option<Box<Runtime>>);
impl Clone for Prepared {
    fn clone(&self) -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for Prepared {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreparedSynth")
            .field("ready", &self.0.is_some())
            .finish()
    }
}
impl Prepared {
    pub fn prepare(&mut self, settings: Option<&Settings>, rate: u32) {
        self.prepare_engine(settings, rate);
        self.prepare_input(settings, rate);
    }
    /// Web hosts can spread construction over separate render quanta.
    pub fn prepare_engine(&mut self, settings: Option<&Settings>, rate: u32) {
        self.0 = settings.map(|_| Box::new(Runtime::new(rate)));
    }
    pub fn prepare_input(&mut self, settings: Option<&Settings>, rate: u32) {
        if let (Some(runtime), Some(s)) = (&mut self.0, settings) {
            let mut input = LiveInput::for_drums(rate, !s.granular, s.buffer_seconds);
            input.sample.name = NAME.into();
            runtime.live = Some(input);
        }
    }
}
pub const NAME: &str = "Synth engine";
