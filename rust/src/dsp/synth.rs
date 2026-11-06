//! Synth engine voices: up to four Rustias OSC1/Wavefolder layers on each
//! engine voice, before that voice's filters, envelopes and VCA. Settings and
//! tables are shared; every voice owns its phases, slews and decimator. The
//! voice sum enters the live history like a drum machine, so the granular
//! stage follows the VCA. Rendering allocates nothing.
use super::{
    SampleBuffer, VOICES, radias_oscillator, radias_wavefolder,
    resynth::{Decimator, PcmLoop},
    sample::{LiveInput, Mip},
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
use std::{f32::consts::PI, ops::Range, sync::OnceLock};

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
    pcm_mip: Mip,
    phase_slew: super::interaction::PhaseSlew,
    tap_decimator: Decimator,
    transport_phase: f64,
    phase_correction: Stereo,
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
            pcm_mip: Mip::default(),
            phase_slew: Default::default(),
            tap_decimator: Decimator::new(),
            transport_phase: 0.0,
            phase_correction: [0.0; 2],
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
    /// Each note can route its layers independently without muting operators
    /// that another layer uses as a modulation source.
    selected: [f32; MAX_OSCILLATORS],
    selection_ready: bool,
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
            selected: [1.0; MAX_OSCILLATORS],
            selection_ready: false,
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

struct State<const N: usize> {
    sample_rate: f32,
    oversample: usize,
    voices: [Voice; N],
    layers: [Layer; MAX_OSCILLATORS],
    /// A layer renders when audible or referenced by an audible layer.
    needed: [bool; MAX_OSCILLATORS],
    tables: &'static radias_oscillator::Tables,
    waveform: &'static WaveformTable,
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
    /// Optional unmixed generator outside this bank (the 303 VCO).
    external: Option<(f32, bool)>,
    interaction_frames: [super::wavetable::mixed::SlotFrame; MAX_OSCILLATORS],
}

pub struct VoiceBank<const N: usize> {
    state: Box<State<N>>,
    /// Prepared off audio. Also retires a previous live input on a type change.
    pub live: Option<LiveInput>,
}
pub type Runtime = VoiceBank<VOICES>;
pub(super) type Mono = VoiceBank<1>;

impl<const N: usize> VoiceBank<N> {
    pub fn new(sample_rate: u32) -> Self {
        let sample_rate = sample_rate.clamp(8_000, 384_000);
        // Every bank has independent oscillator state, but these tables never
        // change. A polyphonic 303 can therefore prepare many mono banks without
        // duplicating the immutable data.
        static TABLES: OnceLock<radias_oscillator::Tables> = OnceLock::new();
        static WAVEFORM: OnceLock<WaveformTable> = OnceLock::new();
        Self {
            state: Box::new(State {
                sample_rate: sample_rate as f32,
                oversample: if sample_rate < 96_000 { 2 } else { 1 },
                voices: std::array::from_fn(|_| Voice::default()),
                layers: [Layer::default(); MAX_OSCILLATORS],
                needed: [false; MAX_OSCILLATORS],
                tables: TABLES.get_or_init(radias_oscillator::Tables::new),
                waveform: WAVEFORM.get_or_init(radias_oscillator::waveform_table),
                settings: None,
                transpose: 0.0,
                pcm_source: (0, 0, 0, 0),
                root_hz: 261.6255653005986,
                epoch: 0,
                frame: 0,
                current: 0,
                lfo: 0,
                output: [0.0; 2],
                external: None,
                interaction_frames: [super::wavetable::mixed::SlotFrame::default();
                    MAX_OSCILLATORS],
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
    pub(super) fn set_source(&mut self, sample: f32, wrapped: bool) {
        self.state.external = Some((sample, wrapped));
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
            o.interaction_source = o.interaction_source.clamp(
                if s.external.is_some() {
                    0
                } else {
                    FIRST_REFERENCE
                },
                FIRST_REFERENCE + MAX_OSCILLATORS as u8 - 1,
            );
            o.cross_source = o.cross_source.min(MAX_OSCILLATORS as u8);
            o
        });
        let count = (settings.count as usize).min(MAX_OSCILLATORS);
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
                        .then(|| {
                            o.interaction_source
                                .checked_sub(FIRST_REFERENCE)
                                .map(usize::from)
                        })
                        .flatten(),
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
    /// Optional mono layers can fade all the way out when their bank is empty.
    pub(super) fn active(&self) -> bool {
        self.state.needed.iter().any(|&on| on)
            || self.state.layers.iter().any(|layer| layer.gain != 0.0)
    }
    /// Render one engine voice before its filter. `generation` changes with
    /// every note-on of that voice; `note` includes the voice's pitch bend.
    #[cfg(test)]
    pub(super) fn render(
        &mut self,
        index: usize,
        generation: u32,
        note: f32,
        sample: &SampleBuffer,
        range: &Range<usize>,
    ) -> Stereo {
        self.render_interacting(index, generation, note, sample, range, None, 0, None)
    }
    pub(super) fn interaction_frames(
        &self,
    ) -> [super::wavetable::mixed::SlotFrame; MAX_OSCILLATORS] {
        self.state.interaction_frames
    }
    #[cfg(test)]
    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_interacting(
        &mut self,
        index: usize,
        generation: u32,
        note: f32,
        sample: &SampleBuffer,
        range: &Range<usize>,
        inputs: Option<[super::interaction::Input; MAX_OSCILLATORS]>,
        demand: u8,
        transport: Option<(&mut super::phase_transport::Runtime, usize, usize, u64)>,
    ) -> Stereo {
        self.render_interacting_masked(
            index,
            generation,
            note,
            sample,
            range,
            inputs,
            demand,
            transport,
            u8::MAX,
        )
    }

    /// `mask` selects the audible layers of this note; their unmixed signals
    /// still run for internal and external interaction routes.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_interacting_masked(
        &mut self,
        index: usize,
        generation: u32,
        note: f32,
        sample: &SampleBuffer,
        range: &Range<usize>,
        inputs: Option<[super::interaction::Input; MAX_OSCILLATORS]>,
        demand: u8,
        mut transport: Option<(&mut super::phase_transport::Runtime, usize, usize, u64)>,
        mask: u8,
    ) -> Stereo {
        let s = &mut *self.state;
        s.interaction_frames
            .fill(super::wavetable::mixed::SlotFrame::default());
        let voice = &mut s.voices[index];
        if voice.generation != generation {
            voice.reset(generation);
        }
        let step = 1.0 / (s.sample_rate * 0.005).max(1.0);
        for (i, selected) in voice.selected.iter_mut().enumerate() {
            let target = f32::from(mask & (1 << i) != 0);
            if voice.selection_ready {
                *selected += (target - *selected).clamp(-step, step);
            } else {
                *selected = target;
            }
        }
        voice.selection_ready = true;
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
                let graph_needed = demand & (1 << id) != 0 && layer.settings.enabled;
                if (!s.needed[id] && layer.gain == 0.0 && !graph_needed) || !carrier.audible {
                    voice.taps[id] = 0.0;
                    voice.wraps[id] = false;
                    continue;
                }
                let o = layer.settings;
                let (reference, wrapped) = o
                    .interaction_source
                    .checked_sub(FIRST_REFERENCE)
                    .map_or_else(
                        || s.external.unwrap_or_default(),
                        |id| (voice.taps[id as usize], voice.wraps[id as usize]),
                    );
                let interaction = inputs.map(|i| i[id]).unwrap_or_default();
                let event = interaction.playback.event_in(sub, s.oversample);
                let external_sync = interaction.playback.mode == crate::interaction::SyncMode::Hard
                    && event.is_some();
                let sync = if interaction.playback.mode != crate::interaction::SyncMode::Off {
                    external_sync
                } else {
                    o.interaction.sync() && wrapped
                };
                let transporting = o.model == Model::Radias
                    && matches!(
                        o.waveform,
                        Waveform::Noise | Waveform::AudioIn | Waveform::Formant
                    )
                    && (o.transport.enabled
                        || interaction.playback.mode != crate::interaction::SyncMode::Off)
                    && transport
                        .as_ref()
                        .is_some_and(|(p, base, lane, _)| p.contains(*base + id, *lane));
                let pcm_loop =
                    (o.waveform == Waveform::Pcm).then(|| PcmLoop::new(sample, range, o));
                let phase = if transporting {
                    carrier.transport_phase
                } else if let Some(pcm) = &pcm_loop {
                    carrier.pcm_phase / pcm.length.max(1.0)
                } else if o.model == Model::Wavefolder {
                    carrier.folder.phase
                } else {
                    radias_oscillator::phase(&carrier.oscillator)
                };
                if event.is_some() {
                    carrier.phase_slew.correct(
                        phase,
                        interaction.playback,
                        s.sample_rate as f64 * s.oversample as f64,
                    );
                }
                let correction = carrier.phase_slew.next();
                let corrected_phase = (phase + correction).rem_euclid(1.0);
                if correction != 0.0 && !transporting {
                    if let Some(pcm) = &pcm_loop {
                        carrier.pcm_phase = corrected_phase * pcm.length;
                    } else if o.model == Model::Wavefolder {
                        carrier.folder.phase = corrected_phase;
                    } else {
                        let offset = (-correction * 4294967296.0) as i64 as u32;
                        carrier.oscillator.phase.0 =
                            carrier.oscillator.phase.0.wrapping_add(offset);
                        carrier.oscillator.modulated_phase.0 =
                            carrier.oscillator.modulated_phase.0.wrapping_add(offset);
                        for phase in &mut carrier.oscillator.unison.phases {
                            phase.0 = phase.0.wrapping_add(offset);
                        }
                    }
                }
                let modulator = if o.cross_source == 0 {
                    if o.cross_input == crate::resynth::CrossInput::Source {
                        s.external.unwrap_or_default().0
                    } else {
                        0.0
                    }
                } else {
                    voice.taps[o.cross_source as usize - 1]
                };
                let (mut raw, mut wrapped) = match o.waveform {
                    Waveform::Pcm => {
                        if sync && !external_sync {
                            carrier.pcm_phase = 0.0;
                        }
                        let pcm = PcmLoop::new(sample, range, o);
                        let wrapped = carrier.pcm_phase + carrier.pcm_rate >= pcm.length;
                        let x = if external_sync || carrier.phase_correction != [0.0; 2] {
                            let (value, end) = super::interaction::read_phase(
                                &mut carrier.phase_correction,
                                carrier.pcm_phase,
                                carrier.pcm_rate,
                                external_sync.then(|| {
                                    (event.unwrap(), interaction.playback.phase * pcm.length)
                                }),
                                |p| {
                                    let mut position = p.rem_euclid(pcm.length.max(1.0));
                                    pcm.read_at(&mut position, 0.0, carrier.pcm_mip, sample, range)
                                },
                            );
                            carrier.pcm_phase = end.rem_euclid(pcm.length.max(1.0));
                            value
                        } else {
                            pcm.read_at(
                                &mut carrier.pcm_phase,
                                carrier.pcm_rate,
                                carrier.pcm_mip,
                                sample,
                                range,
                            )
                        };
                        (x, wrapped)
                    }
                    _ if o.model == Model::Wavefolder => {
                        let phase = carrier.folder.phase;
                        let x = if external_sync || carrier.phase_correction != [0.0; 2] {
                            carrier.folder.next_interacting(
                                carrier.increment.0 as f64 / 4294967296.0,
                                o.waveform,
                                o.fold_shape,
                                layer.fold_controls,
                                external_sync.then(|| (event.unwrap(), interaction.playback.phase)),
                                &mut carrier.phase_correction,
                            )
                        } else {
                            carrier.folder.next(
                                carrier.increment.0 as f64 / 4294967296.0,
                                o.waveform,
                                o.fold_shape,
                                layer.fold_controls,
                                sync && !external_sync,
                            )
                        };
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
                            .next_with_fractional_reset(
                                &mut carrier.oscillator,
                                layer.program,
                                carrier.increment,
                                carrier.code,
                                lfo,
                                [0; 2],
                                modulator,
                                s.tables,
                                s.waveform,
                                frame,
                                index * MAX_OSCILLATORS + id,
                                sync && !transporting,
                                s.epoch,
                                if external_sync {
                                    radias_oscillator::reset_word(interaction.playback.phase)
                                } else {
                                    0
                                },
                                if external_sync && !transporting {
                                    event
                                } else {
                                    None
                                },
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
                if let Some((pool, base, lane, stamp)) = &mut transport {
                    let scale = if o.model == Model::Radias
                        && !matches!(o.waveform, Waveform::AudioIn | Waveform::Pcm)
                    {
                        0.5
                    } else {
                        1.0
                    };
                    if sub + 1 == s.oversample {
                        pool.capture(*base + id, *lane, raw.map(|x| x * scale), *stamp);
                    }
                    if transporting {
                        let hz = carrier.increment.0 as f64 / 4294967296.0
                            * s.sample_rate as f64
                            * s.oversample as f64;
                        let advance = carrier.increment.0 as f64 / 4294967296.0
                            * o.transport.rate as f64
                            + correction;
                        let (value, end) = super::interaction::read_phase(
                            &mut carrier.phase_correction,
                            carrier.transport_phase,
                            advance,
                            external_sync.then(|| (event.unwrap(), interaction.playback.phase)),
                            |p| {
                                pool.read(
                                    *base + id,
                                    *lane,
                                    p,
                                    hz,
                                    s.sample_rate as f64 * s.oversample as f64,
                                    o.waveform,
                                )
                                .unwrap_or([0.0; 2])
                            },
                        );
                        raw = value.map(|x| x / scale);
                        wrapped = end >= 1.0;
                        carrier.transport_phase = end.rem_euclid(1.0);
                    }
                }
                voice.wraps[id] = voice.initialized[id] && (sync || wrapped);
                voice.initialized[id] = true;
                let tap_scale = if o.waveform == Waveform::Pcm || o.model == Model::Wavefolder {
                    0.5
                } else {
                    0.25
                };
                let generation = (raw[0] + raw[1]) * tap_scale;
                if o.interaction.ring() && interaction.depth == 0.0 {
                    let reference = reference.clamp(-1.0, 1.0);
                    raw = raw.map(|x| x * reference);
                }
                if inputs.is_some() {
                    raw = raw.map(|x| x * interaction.factor_at(sub, s.oversample));
                    let tap = [generation, (raw[0] + raw[1]) * tap_scale];
                    let reduced = if s.oversample == 2 {
                        carrier.tap_decimator.next(tap)
                    } else {
                        tap
                    };
                    s.interaction_frames[id].generation =
                        [if o.enabled { reduced[0] } else { 0.0 }; 2];
                    s.interaction_frames[id].output = [if o.enabled { reduced[1] } else { 0.0 }; 2];
                    let step = if let Some(pcm) = pcm_loop {
                        carrier.pcm_rate / pcm.length.max(1.0)
                    } else {
                        carrier.increment.0 as f64 / 4294967296.0
                            * if transporting {
                                o.transport.rate as f64
                            } else {
                                1.0
                            }
                    } + correction;
                    if !sync
                        && o.enabled
                        && phase + step >= 1.0
                        && step > 0.0
                        && (o.waveform != Waveform::AudioIn || transporting)
                    {
                        s.interaction_frames[id].wrap =
                            Some((sub as f64 + (1.0 - phase) / step) / s.oversample as f64);
                    }
                }
                voice.taps[id] = (raw[0] + raw[1]) * 0.5;
                for c in 0..2 {
                    wet[c] += raw[c] * layer.gain * layer.pan[c] * OUTPUT * voice.selected[id];
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
        carrier.pcm_mip = sample.mip_for_rate(carrier.pcm_rate, true);
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

#[cfg(test)]
mod source_tests {
    use super::*;
    use crate::resynth::{CrossInput, Interaction};

    #[test]
    fn mask_is_per_voice_and_keeps_hidden_ring_source_running() {
        let sample = SampleBuffer::new(String::new(), 48_000, vec![[0.0; 2]]).unwrap();
        let mut settings = Settings {
            count: 2,
            ..Settings::default()
        };
        settings.oscillators[0].interaction = Interaction::Ring;
        settings.oscillators[0].interaction_source = FIRST_REFERENCE + 1;
        settings.oscillators[1].transpose = 7.0;
        // Matching bank/voice indices keep the four-frame control schedule the
        // same across the independent reference renders.
        let mut banks = std::array::from_fn::<_, 4, _>(|_| Mono::new(48_000));
        assert!(std::ptr::eq(banks[0].state.tables, banks[1].state.tables));
        assert!(std::ptr::eq(
            banks[0].state.waveform,
            banks[1].state.waveform
        ));
        for bank in &mut banks {
            bank.configure(&settings, 0.0, &sample, 60.0);
        }
        let mut energy = 0.0;
        let mut hidden_source = 0.0_f32;
        for _ in 0..1024 {
            let render = |bank: &mut Mono, mask| {
                bank.begin_frame(0.0);
                bank.render_interacting_masked(0, 1, 60.0, &sample, &(0..1), None, 0, None, mask)
            };
            let a = render(&mut banks[0], 1);
            let b = render(&mut banks[1], 2);
            let both = render(&mut banks[2], 3);
            let neither = render(&mut banks[3], 0);
            for c in 0..2 {
                assert!(
                    (both[c] - (a[c] + b[c])).abs() < 2e-6,
                    "combined {} vs {} + {}",
                    both[c],
                    a[c],
                    b[c]
                );
                assert_eq!(neither[c], 0.0);
            }
            energy += a[0].abs();
            hidden_source = hidden_source.max(banks[0].state.voices[0].taps[1].abs());
        }
        assert!(energy > 1.0, "the selected ring-modulated layer was silent");
        assert!(hidden_source > 0.01, "the unselected ring source stopped");

        let bank = &mut banks[0];
        bank.begin_frame(0.0);
        bank.render_interacting_masked(0, 1, 60.0, &sample, &(0..1), None, 0, None, 0);
        assert!(bank.state.voices[0].selected[0] > 0.99);
        for _ in 0..400 {
            bank.begin_frame(0.0);
            bank.render_interacting_masked(0, 1, 60.0, &sample, &(0..1), None, 0, None, 0);
        }
        assert_eq!(bank.state.voices[0].selected[0], 0.0);
        let mut poly = VoiceBank::<2>::new(48_000);
        poly.configure(&settings, 0.0, &sample, 60.0);
        poly.begin_frame(0.0);
        poly.render_interacting_masked(0, 1, 60.0, &sample, &(0..1), None, 0, None, 1);
        poly.render_interacting_masked(1, 1, 60.0, &sample, &(0..1), None, 0, None, 2);
        assert_eq!(poly.state.voices[0].selected[..2], [1.0, 0.0]);
        assert_eq!(poly.state.voices[1].selected[..2], [0.0, 1.0]);
    }

    #[test]
    fn external_source_resets_radias_phase_and_multiplies_the_unmixed_wave() {
        let sample = SampleBuffer::new(String::new(), 192000, vec![[0.0; 2]]).unwrap();
        let mut settings = Settings::default();
        settings.oscillators[0].interaction = Interaction::RingSync;
        settings.oscillators[0].interaction_source = 0;
        let mut bank = Mono::new(192000);
        bank.set_source(1.0, false);
        bank.configure(&settings, 0.0, &sample, 60.0);
        for _ in 0..256 {
            bank.begin_frame(0.0);
            bank.render(0, 0, 52.0, &sample, &(0..1));
        }
        let render = |bank: &mut Mono, source, wrap| {
            bank.set_source(source, wrap);
            bank.begin_frame(0.0);
            bank.render(0, 0, 52.0, &sample, &(0..1))
        };
        let full = render(&mut bank, 1.0, true);
        let reset_phase = bank.state.voices[0].carriers[0].oscillator.phase;
        for _ in 0..37 {
            render(&mut bank, 1.0, false);
        }
        assert_ne!(
            bank.state.voices[0].carriers[0].oscillator.phase,
            reset_phase
        );
        let half = render(&mut bank, 0.5, true);
        assert_eq!(
            bank.state.voices[0].carriers[0].oscillator.phase,
            reset_phase
        );
        assert_eq!(half, full.map(|v| v * 0.5));
        assert_ne!(full, [0.0; 2]);
        assert_eq!(render(&mut bank, 0.0, true), [0.0; 2]);
    }

    #[test]
    fn external_cross_input_changes_radias_without_an_internal_modulator() {
        let sample = SampleBuffer::new(String::new(), 192000, vec![[0.0; 2]]).unwrap();
        let mut settings = Settings::default();
        settings.oscillators[0].mode = Mode::Cross;
        settings.oscillators[0].control1 = 1.0;
        settings.oscillators[0].cross_source = 0;
        settings.oscillators[0].cross_input = CrossInput::Source;
        let mut plain = Mono::new(192000);
        let mut cross = Mono::new(192000);
        for bank in [&mut plain, &mut cross] {
            bank.set_source(0.0, false);
            bank.configure(&settings, 0.0, &sample, 60.0);
        }
        let mut difference = 0.0_f32;
        for i in 0..4096 {
            plain.begin_frame(0.0);
            cross.set_source((i as f32 * 0.13).sin(), false);
            cross.begin_frame(0.0);
            let a = plain.render(0, 0, 52.0, &sample, &(0..1));
            let b = cross.render(0, 0, 52.0, &sample, &(0..1));
            difference += (a[0] - b[0]).abs();
        }
        assert!(difference > 1.0);
    }
}
