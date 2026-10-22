//! Streaming 16-band excitation of four Rustias carrier layers.
//! All histories and tables are allocated before rendering; scalar edits keep
//! phases, envelope histories and the original OSC1 four-frame control slew.
use super::{
    SampleBuffer,
    filter_slots::{self, Bank, Instance},
    radias_oscillator, radias_wavefolder,
};
use crate::{
    Stereo,
    resynth::{BANDS, CrossInput, Input, MAX_OSCILLATORS, Mode, Model, Settings, Waveform},
};
use radias_synth_application::primary::PrimaryProgram;
use radias_synth_domain::{
    Sample,
    controller_primary::PrimaryControl,
    pitch::{PhaseIncrement, PitchCode},
    primary_oscillator::PrimaryOscillator,
    vocoder::BandEnvelopes,
    waveform::WaveformTable,
};
use std::{f32::consts::PI, ops::Range};

/// Sixteen analysis band-pass filters of one input, stored by field so each
/// band is one vector lane. Every lane runs the per-band filter arithmetic
/// unchanged; only the memory layout differs.
#[derive(Clone, Copy)]
struct Bands {
    g: [f32; BANDS],
    a: [f32; BANDS],
    /// [channel][state][band]
    z: [[[f32; BANDS]; 2]; 2],
}

impl Bands {
    fn new(frequencies: &[f32; BANDS], sr: f32) -> Self {
        let g = frequencies.map(|hz| (PI * hz / sr).tan());
        Self {
            g,
            a: g.map(|g| 1.0 / (1.0 + g * (g + 1.0 / 2.8))),
            z: [[[0.0; BANDS]; 2]; 2],
        }
    }
    fn clear(&mut self) {
        self.z = [[[0.0; BANDS]; 2]; 2];
    }
    #[inline]
    fn next(&mut self, input: Stereo) -> [Stereo; BANDS] {
        let mut out = [[0.0; 2]; BANDS];
        for (c, [z0, z1]) in self.z.iter_mut().enumerate() {
            for i in 0..BANDS {
                let v3 = input[c] - z1[i];
                let v1 = self.a[i] * (z0[i] + self.g[i] * v3);
                let v2 = z1[i] + self.g[i] * v1;
                z0[i] = super::fast::flush(2.0 * v1 - z0[i]);
                z1[i] = super::fast::flush(2.0 * v2 - z1[i]);
                out[i][c] = v1 / 2.8;
            }
        }
        out
    }
}

#[derive(Clone, Copy, Default)]
pub(super) struct Lowpass {
    b: [f32; 3],
    a: [f32; 2],
    z: [[f32; 2]; 2],
}
impl Lowpass {
    /// The four sections of the 2x decimator, in order.
    pub(super) const QS: [f32; 4] = [0.5097956, 0.6013449, 0.8999762, 2.5629154];
    pub(super) fn new(q: f32) -> Self {
        // Eighth-order minimum-phase anti-alias filter at 0.45 * host rate.
        let w = 2.0 * PI * 0.225;
        let alpha = w.sin() / (2.0 * q);
        let scale = 1.0 / (1.0 + alpha);
        let b = (1.0 - w.cos()) * 0.5 * scale;
        Self {
            b: [b, 2.0 * b, b],
            a: [-2.0 * w.cos() * scale, (1.0 - alpha) * scale],
            ..Self::default()
        }
    }
    pub(super) fn next(&mut self, input: Stereo) -> Stereo {
        std::array::from_fn(|c| {
            let y = self.b[0] * input[c] + self.z[c][0];
            self.z[c] = [
                super::fast::flush(self.b[1] * input[c] - self.a[0] * y + self.z[c][1]),
                super::fast::flush(self.b[2] * input[c] - self.a[1] * y),
            ];
            y
        })
    }
}

struct Carrier {
    controller: radias_oscillator::Controller,
    oscillator: PrimaryOscillator,
    increment: PhaseIncrement,
    code: PitchCode,
    pcm_phase: f64,
    pcm_rate: f64,
    pcm_level: usize,
    audible: bool,
    folder: radias_wavefolder::Wavefolder,
    phase_initialized: bool,
}

pub(super) struct PcmLoop {
    start: f64,
    pub(super) length: f64,
    fade: f64,
}
impl PcmLoop {
    pub(super) fn new(
        sample: &SampleBuffer,
        range: &Range<usize>,
        o: crate::resynth::Oscillator,
    ) -> Self {
        let length = range.len().max(1) as f64;
        let offset = o.control1 as f64 * (length - 1.0);
        let length = (length - offset).max(1.0);
        let fade = o.control2 as f64 * (length * 0.1).min(sample.sample_rate as f64 * 0.1);
        Self {
            start: range.start as f64 + offset,
            length,
            fade: if fade > 1.0 { fade } else { 0.0 },
        }
    }
    fn read(&self, carrier: &mut Carrier, sample: &SampleBuffer, range: &Range<usize>) -> Stereo {
        self.read_at(
            &mut carrier.pcm_phase,
            carrier.pcm_rate,
            carrier.pcm_level,
            sample,
            range,
        )
    }
    pub(super) fn read_at(
        &self,
        phase: &mut f64,
        rate: f64,
        level: usize,
        sample: &SampleBuffer,
        range: &Range<usize>,
    ) -> Stereo {
        let p = if *phase < self.length {
            *phase
        } else {
            phase.rem_euclid(self.length)
        };
        let mut x = sample.read_region_level(self.start + p, level, range);
        if self.fade > 0.0 && p > self.length - self.fade {
            let y =
                sample.read_region_level(self.start + p - self.length + self.fade, level, range);
            let t = ((p - self.length + self.fade) / self.fade) as f32;
            let mix = t * t * (3.0 - 2.0 * t);
            x = std::array::from_fn(|c| x[c] + (y[c] - x[c]) * mix);
        }
        let next = p + rate;
        *phase = if next < self.length {
            next
        } else if self.fade > 0.0 {
            // The head overlap was already played during the crossfade. Resume
            // after it, including when a high pitch crosses several periods.
            self.fade + (next - self.length).rem_euclid(self.length - self.fade)
        } else {
            next.rem_euclid(self.length)
        };
        x
    }
}
impl Default for Carrier {
    fn default() -> Self {
        Self {
            controller: Default::default(),
            oscillator: Default::default(),
            increment: PhaseIncrement(0),
            code: PitchCode::new(60 * 256).unwrap(),
            pcm_phase: 0.0,
            pcm_rate: 1.0,
            pcm_level: 0,
            audible: true,
            folder: Default::default(),
            phase_initialized: false,
        }
    }
}
struct Layer {
    carriers: [Carrier; BANDS],
    settings: crate::resynth::Oscillator,
    program: PrimaryProgram,
    pan: Stereo,
    gain: f32,
    target_gain: f32,
    fold_controls: [f64; 2],
}
impl Default for Layer {
    fn default() -> Self {
        Self {
            carriers: std::array::from_fn(|_| Carrier::default()),
            settings: Default::default(),
            program: Default::default(),
            pan: [1.0; 2],
            gain: 0.0,
            target_gain: 0.0,
            fold_controls: [0.0; 2],
        }
    }
}

pub struct Resynth {
    sample_rate: f32,
    oversample: usize,
    frequencies: [f32; BANDS],
    bands: Bands,
    source_bands: Bands,
    grain_bands: Bands,
    source_signs: [i8; BANDS],
    grain_signs: [i8; BANDS],
    analysis_signs: [i8; BANDS],
    envelopes: BandEnvelopes,
    levels: [f32; BANDS],
    gates: [bool; BANDS],
    gains: [f32; BANDS],
    feedback: [[f32; BANDS]; MAX_OSCILLATORS],
    taps: [[f32; BANDS]; MAX_OSCILLATORS],
    wraps: [[bool; BANDS]; MAX_OSCILLATORS],
    needed: [bool; MAX_OSCILLATORS],
    external_taps: [bool; 2],
    layers: [Layer; MAX_OSCILLATORS],
    tables: Box<radias_oscillator::Tables>,
    waveform: Box<WaveformTable>,
    lowpass: [Lowpass; 4],
    settings: Settings,
    configured: bool,
    pcm_source: (usize, u32, usize, u32),
    attack: i32,
    release: i32,
    sensitivity: f32,
    threshold: f32,
    wet: f32,
    wet_target: f32,
    frame: u64,
    lfo: i16,
    /// Advances with every configuration that can change carrier controls.
    control_epoch: u64,
}
impl Resynth {
    pub fn new(sr: u32) -> Self {
        let frequencies = std::array::from_fn(|i| {
            60.0 * ((12000.0_f32.min(sr as f32 * 0.4) / 60.0).powf(i as f32 / (BANDS - 1) as f32))
        });
        Self {
            sample_rate: sr as f32,
            oversample: oversample(sr) as usize,
            frequencies,
            bands: Bands::new(&frequencies, sr as f32),
            source_bands: Bands::new(&frequencies, sr as f32),
            grain_bands: Bands::new(&frequencies, sr as f32),
            source_signs: [0; BANDS],
            grain_signs: [0; BANDS],
            analysis_signs: [0; BANDS],
            envelopes: Default::default(),
            levels: [0.0; BANDS],
            gates: [false; BANDS],
            gains: [0.0; BANDS],
            feedback: [[0.0; BANDS]; MAX_OSCILLATORS],
            taps: [[0.0; BANDS]; MAX_OSCILLATORS],
            wraps: [[false; BANDS]; MAX_OSCILLATORS],
            needed: [false; MAX_OSCILLATORS],
            external_taps: [false; 2],
            layers: std::array::from_fn(|_| Layer::default()),
            tables: Box::new(radias_oscillator::Tables::new()),
            waveform: Box::new(radias_oscillator::waveform_table()),
            lowpass: Lowpass::QS.map(Lowpass::new),
            settings: Settings::default(),
            configured: false,
            pcm_source: (0, 0, 0, 0),
            attack: 0,
            release: 0,
            sensitivity: 1.0,
            threshold: 0.0,
            wet: 0.0,
            wet_target: 0.0,
            frame: 0,
            lfo: 0,
            control_epoch: 0,
        }
    }
    pub fn clear(&mut self) {
        self.bands.clear();
        self.source_bands.clear();
        self.grain_bands.clear();
        self.envelopes.levels.fill(0);
        self.levels.fill(0.0);
        self.gains.fill(0.0);
        self.gates.fill(false);
        self.feedback.fill([0.0; BANDS]);
        self.taps.fill([0.0; BANDS]);
        self.wraps.fill([false; BANDS]);
        self.source_signs.fill(0);
        self.grain_signs.fill(0);
        self.analysis_signs.fill(0);
        for filter in &mut self.lowpass {
            filter.z = [[0.0; 2]; 2];
        }
        for layer in &mut self.layers {
            for carrier in &mut layer.carriers {
                carrier.pcm_phase = 0.0;
                carrier.phase_initialized = false;
            }
        }
    }
    pub fn configure(&mut self, settings: Settings, sample: &SampleBuffer, root: f32) {
        let pcm_source = (
            sample as *const SampleBuffer as usize,
            sample.sample_rate,
            sample.len(),
            root.to_bits(),
        );
        if self.configured && self.settings == settings && self.pcm_source == pcm_source {
            return;
        }
        let source_changed = self.pcm_source != pcm_source;
        let rate = self.sample_rate * self.oversample as f32;
        let gain = |seconds: f32| {
            ((-1.0_f64 / (seconds as f64 * self.sample_rate as f64)).exp() * i32::MAX as f64)
                .round() as i32
        };
        if !self.configured || settings.attack != self.settings.attack {
            self.attack = gain(settings.attack_seconds());
        }
        if !self.configured || settings.release != self.settings.release {
            self.release = gain(settings.release_seconds());
        }
        self.sensitivity = settings.sensitivity_gain();
        self.threshold = settings.threshold_gain();
        self.wet_target = if settings.enabled { settings.mix } else { 0.0 };
        let enabled = settings.oscillators[..settings.count as usize]
            .iter()
            .filter(|o| o.enabled)
            .count()
            .max(1) as f32;
        self.needed = std::array::from_fn(|i| {
            i < settings.count as usize
                && settings.oscillators[i].enabled
                && (settings.oscillators[i].level > 0.0 || self.layers[i].gain > 0.0)
        });
        // Keep an unmixed source running when another oscillator uses its
        // signal or phase. Fixed-size closure also handles forward links/cycles.
        for _ in 0..MAX_OSCILLATORS {
            for i in 0..MAX_OSCILLATORS {
                if !self.needed[i] {
                    continue;
                }
                let o = settings.oscillators[i];
                let references = [
                    if o.interaction_source >= 2
                        && o.interaction != crate::resynth::Interaction::Off
                    {
                        Some(o.interaction_source as usize - 2)
                    } else {
                        None
                    },
                    if o.model == Model::Radias && o.mode == Mode::Cross && o.cross_source > 0 {
                        Some(o.cross_source as usize - 1)
                    } else {
                        None
                    },
                ];
                for reference in references.into_iter().flatten() {
                    if reference < settings.count as usize
                        && settings.oscillators[reference].enabled
                    {
                        self.needed[reference] = true;
                    }
                }
            }
        }
        self.external_taps = [false; 2];
        for (i, o) in settings.oscillators.iter().enumerate() {
            if !self.needed[i] {
                continue;
            }
            if o.interaction != crate::resynth::Interaction::Off && o.interaction_source < 2 {
                self.external_taps[o.interaction_source as usize] = true;
            }
            if o.model == Model::Radias && o.mode == Mode::Cross && o.cross_source == 0 {
                match o.cross_input {
                    CrossInput::Source => self.external_taps[0] = true,
                    CrossInput::Grains => self.external_taps[1] = true,
                    _ => {}
                }
            }
        }
        for (i, layer) in self.layers.iter_mut().enumerate() {
            let o = settings.oscillators[i];
            layer.target_gain = if i < settings.count as usize && o.enabled {
                o.level / enabled.sqrt()
            } else {
                0.0
            };
            if !self.configured || layer.settings != o || source_changed {
                if layer.settings.model != o.model
                    || layer.settings.waveform != o.waveform
                    || layer.settings.mode != o.mode
                {
                    for carrier in &mut layer.carriers {
                        carrier.phase_initialized = false;
                    }
                }
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
                let pitch_changed = !self.configured
                    || layer.settings.transpose != o.transpose
                    || layer.settings.fine != o.fine
                    || source_changed;
                if pitch_changed {
                    let ratio = 2.0_f64.powf((o.transpose as f64 + o.fine as f64 * 0.01) / 12.0);
                    let root_hz = 440.0 * 2.0_f64.powf((root as f64 - 69.0) / 12.0);
                    for (band, carrier) in layer.carriers.iter_mut().enumerate() {
                        let hz = self.frequencies[band] as f64 * ratio;
                        carrier.audible = hz < self.sample_rate as f64 * 0.45;
                        carrier.increment = PhaseIncrement(
                            (hz.min(rate as f64 * 0.45) / rate as f64 * 4294967296.0).round()
                                as u32,
                        );
                        carrier.code = PitchCode::new(
                            ((69.0 + 12.0 * (hz / 440.0).log2()).clamp(0.0, 127.996) * 256.0)
                                .round()
                                .min(32767.0) as u16,
                        )
                        .unwrap();
                        carrier.pcm_rate = hz / root_hz * sample.sample_rate as f64 / rate as f64;
                        carrier.pcm_level = sample.level_for_rate(carrier.pcm_rate, true);
                    }
                }
                layer.settings = o;
            }
        }
        self.settings = settings;
        self.pcm_source = pcm_source;
        self.configured = true;
        self.control_epoch += 1;
    }
    pub fn active(&self) -> bool {
        self.wet > 0.0 || self.wet_target > 0.0
    }
    pub fn levels(&self) -> [f32; BANDS] {
        self.levels
    }
    pub fn frequencies(&self) -> [f32; BANDS] {
        self.frequencies
    }
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn process(
        &mut self,
        dry: Stereo,
        source: Stereo,
        lfo: f32,
        voice_scale: f32,
        sample: &SampleBuffer,
        range: &Range<usize>,
    ) -> Stereo {
        self.process_with_grains(dry, source, dry, lfo, voice_scale, sample, range, true)
    }
    /// `running` is the transport: stopped and fully silent, carriers pause.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn process_with_grains(
        &mut self,
        dry: Stereo,
        source: Stereo,
        grains: Stereo,
        lfo: f32,
        voice_scale: f32,
        sample: &SampleBuffer,
        range: &Range<usize>,
        running: bool,
    ) -> Stereo {
        self.process_with_filters(
            dry,
            source,
            grains,
            lfo,
            voice_scale,
            sample,
            range,
            running,
            [0; MAX_OSCILLATORS],
            &mut [],
        )
    }
    /// As `process_with_grains`, with added filters after oscillators: one
    /// mask of filter slots per oscillator. A filtered oscillator sums its
    /// carriers on its own before the filters; the sync and ring taps were
    /// taken from the carriers already, so filters never change them.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn process_with_filters(
        &mut self,
        dry: Stereo,
        source: Stereo,
        grains: Stereo,
        lfo: f32,
        voice_scale: f32,
        sample: &SampleBuffer,
        range: &Range<usize>,
        running: bool,
        layer_filters: [u16; MAX_OSCILLATORS],
        banks: &mut [Box<Bank>],
    ) -> Stereo {
        if !self.active() {
            return dry;
        }
        self.wet += (self.wet_target - self.wet).clamp(-1.0 / 128.0, 1.0 / 128.0);
        if !self.active() {
            self.clear();
            return dry;
        }
        if self.frame.is_multiple_of(32) {
            self.lfo = (lfo.clamp(-1.0, 1.0) * 32767.0) as i16;
        }
        let input = if self.settings.input == Input::Source {
            source
        } else {
            grains
        };
        let analyzed = self.bands.next(input);
        let source_bands = if self.settings.input == Input::Source {
            analyzed
        } else if self.external_taps[0] {
            self.source_bands.next(source)
        } else {
            [[0.0; 2]; BANDS]
        };
        let grain_bands = if self.settings.input == Input::Grains {
            analyzed
        } else if self.external_taps[1] {
            self.grain_bands.next(grains)
        } else {
            [[0.0; 2]; BANDS]
        };
        let source_wraps = zero_crossings(&source_bands, &mut self.source_signs);
        let grain_wraps = zero_crossings(&grain_bands, &mut self.grain_signs);
        let analysis_wraps = zero_crossings(&analyzed, &mut self.analysis_signs);
        let excitation = analyzed.map(|x| {
            (x[0].abs().max(x[1].abs()) as f64 * self.sensitivity as f64 * i32::MAX as f64)
                .clamp(0.0, i32::MAX as f64) as i32
        });
        self.envelopes.update(excitation, self.attack, self.release);
        for band in 0..BANDS {
            let level = self.envelopes.levels[band] as i16 as f32 / 32768.0;
            self.levels[band] = level;
            if level > self.threshold {
                self.gates[band] = true;
            } else if level < self.threshold * 0.5 {
                self.gates[band] = false;
            }
            let gain = if self.gates[band] {
                level * std::f32::consts::FRAC_PI_2
            } else {
                0.0
            };
            self.gains[band] =
                super::fast::flush(self.gains[band] + (gain - self.gains[band]) * 0.0625);
        }
        for layer in &mut self.layers {
            layer.gain += (layer.target_gain - layer.gain).clamp(-1.0 / 128.0, 1.0 / 128.0);
        }
        // With every band gain exactly zero, each carrier adds an exact zero.
        // While stopped they pause there instead of rendering silence, and later
        // resume from their own phases. During playback they always run, so
        // rendered audio is unchanged.
        let paused = !running && self.gains.iter().all(|&gain| gain == 0.0);
        let mut wet = [0.0; 2];
        for sub in 0..self.oversample {
            wet = [0.0; 2];
            // Slew once per four host frames, including when carriers run at 2x.
            let frame = if self.frame & 3 == 3 && sub + 1 == self.oversample {
                self.frame
            } else {
                self.frame & !3
            };
            for (layer_id, &filters) in layer_filters.iter().enumerate() {
                let layer = &mut self.layers[layer_id];
                if paused
                    || (!self.needed[layer_id] && layer.target_gain == 0.0 && layer.gain == 0.0)
                {
                    self.feedback[layer_id].fill(0.0);
                    self.taps[layer_id].fill(0.0);
                    self.wraps[layer_id].fill(false);
                    continue;
                }
                let o = layer.settings;
                let mut layer_wet = [0.0; 2];
                let pcm_loop =
                    (o.waveform == Waveform::Pcm).then(|| PcmLoop::new(sample, range, o));
                let lfo = if o.waveform.analog()
                    && matches!(o.mode, Mode::Wave | Mode::Cross)
                    && o.control2 != 0.0
                {
                    self.lfo
                } else {
                    0
                };
                for (band, analyzed_band) in analyzed.iter().enumerate() {
                    let carrier = &mut layer.carriers[band];
                    if !carrier.audible && o.waveform != Waveform::AudioIn {
                        self.feedback[layer_id][band] = 0.0;
                        self.taps[layer_id][band] = 0.0;
                        self.wraps[layer_id][band] = false;
                        continue;
                    }
                    let (reference, wrap) = match o.interaction_source {
                        0 => (mono(source_bands[band]), sub == 0 && source_wraps[band]),
                        1 => (mono(grain_bands[band]), sub == 0 && grain_wraps[band]),
                        id => (
                            self.taps[id as usize - 2][band],
                            self.wraps[id as usize - 2][band],
                        ),
                    };
                    let sync = o.interaction.sync() && wrap;
                    let modulator = if o.cross_source == 0 {
                        mono(match o.cross_input {
                            CrossInput::Analysis => *analyzed_band,
                            CrossInput::Source => source_bands[band],
                            CrossInput::Grains => grain_bands[band],
                        })
                    } else {
                        self.feedback[o.cross_source as usize - 1][band]
                    };
                    let (mut raw, wrapped) = match o.waveform {
                        Waveform::AudioIn => (
                            analyzed_band.map(|x| x * self.sensitivity),
                            sub == 0 && analysis_wraps[band],
                        ),
                        Waveform::Pcm => {
                            if sync {
                                carrier.pcm_phase = 0.0;
                            }
                            let pcm_loop = pcm_loop.as_ref().unwrap();
                            let wrapped = carrier.pcm_phase + carrier.pcm_rate >= pcm_loop.length;
                            (pcm_loop.read(carrier, sample, range), wrapped)
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
                                    &self.tables,
                                    &self.waveform,
                                    frame,
                                    layer_id * BANDS + band,
                                    sync,
                                    self.control_epoch,
                                )
                                .0 as f32
                                / 1073741824.0;
                            let wrapped =
                                radias_synth_domain::secondary_control::SecondaryModulation {
                                    ring: false,
                                    sync: true,
                                }
                                .wraps(carrier.oscillator.phase, phase);
                            ([x; 2], wrapped)
                        }
                    };
                    self.wraps[layer_id][band] = carrier.phase_initialized && (sync || wrapped);
                    carrier.phase_initialized = true;
                    if o.interaction.ring() {
                        raw = raw.map(|x| x * reference.clamp(-1.0, 1.0));
                    }
                    // Interaction taps precede envelope, oscillator level and
                    // pan. Ordered current/previous taps permit feedback links.
                    self.taps[layer_id][band] = mono(raw);
                    let stereo = raw.map(|x| x * self.gains[band]);
                    self.feedback[layer_id][band] = (stereo[0] + stereo[1]) * 0.5;
                    let sum = if filters == 0 {
                        &mut wet
                    } else {
                        &mut layer_wet
                    };
                    for c in 0..2 {
                        sum[c] += stereo[c] * layer.gain * layer.pan[c] * 0.6;
                    }
                }
                if filters != 0 {
                    let filtered =
                        filter_slots::chain(banks, filters, layer_wet, Instance::Oscillator);
                    for c in 0..2 {
                        wet[c] += filtered[c];
                    }
                }
            }
            if self.oversample == 2 {
                for filter in &mut self.lowpass {
                    wet = filter.next(wet);
                }
            }
        }
        self.frame = self.frame.wrapping_add(1);
        std::array::from_fn(|c| {
            dry[c]
                + (wet[c]
                    * if self.settings.input == Input::Source {
                        voice_scale
                    } else {
                        1.0
                    }
                    - dry[c])
                    * self.wet
        })
    }
}

/// Carriers run at twice the host rate below 96 kHz.
pub(crate) fn oversample(sample_rate: u32) -> u32 {
    if sample_rate < 96000 { 2 } else { 1 }
}

fn mono(x: Stereo) -> f32 {
    (x[0] + x[1]) * 0.5
}
/// Band signs cross zero at the band frequency, so this stays branch-free:
/// above +1e-5 the sign becomes 1, below -1e-5 it becomes -1, otherwise
/// (including NaN) it holds; a wrap is a rise from -1 to 1.
fn zero_crossings(input: &[Stereo; BANDS], signs: &mut [i8; BANDS]) -> [bool; BANDS] {
    let mut wraps = [false; BANDS];
    for ((x, sign), wrap) in input.iter().zip(signs.iter_mut()).zip(&mut wraps) {
        let sample = mono(*x);
        let (up, down) = (sample > 1e-5, sample < -1e-5);
        let previous = *sign;
        *sign = up as i8 - down as i8 + previous * !(up | down) as i8;
        *wrap = up & (previous == -1);
    }
    wraps
}

#[cfg(test)]
mod tests {
    use super::*;
    fn tone(n: usize, hz: f32, sr: u32) -> Stereo {
        [(n as f32 * 2.0 * PI * hz / sr as f32).sin() * 0.2; 2]
    }
    fn setup(sr: u32) -> (Resynth, SampleBuffer, Settings) {
        let sample = SampleBuffer::new(
            "PCM".into(),
            sr,
            (0..4096).map(|n| tone(n, 261.6256, sr)).collect(),
        )
        .unwrap();
        let mut settings = Settings {
            enabled: true,
            mix: 1.0,
            ..Settings::default()
        };
        settings.oscillators[0].control1 = 0.37;
        settings.oscillators[0].control2 = 0.41;
        (Resynth::new(sr), sample, settings)
    }
    fn render_interaction(
        sr: u32,
        model: Model,
        interaction: crate::resynth::Interaction,
        reference: u8,
    ) -> Vec<f32> {
        let (mut engine, sample, mut settings) = setup(sr);
        settings.count = 4;
        for (i, o) in settings.oscillators.iter_mut().enumerate() {
            o.model = model;
            o.level = if i == 0 { 0.7 } else { 0.0 };
            o.transpose = [0.0, 7.0, -12.0, 12.0][i];
            o.control1 = 0.4;
            o.control2 = 0.5;
        }
        settings.oscillators[0].interaction = interaction;
        settings.oscillators[0].interaction_source = reference;
        engine.configure(settings, &sample, 60.0);
        (0..6000)
            .map(|n| {
                engine.process_with_grains(
                    [0.0; 2],
                    tone(n, 211.0, sr),
                    tone(n, 733.0, sr),
                    0.0,
                    1.0,
                    &sample,
                    &(0..sample.len()),
                    true,
                )[0]
            })
            .collect()
    }
    #[test]
    fn ring_and_sync_use_separate_source_grain_and_unmixed_oscillator_taps() {
        use crate::resynth::Interaction;
        let difference = |a: &[f32], b: &[f32]| {
            a.iter()
                .zip(b)
                .skip(1000)
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f32>()
        };
        for sr in [44100, 48000, 96000] {
            for model in [Model::Radias, Model::Wavefolder] {
                let plain = render_interaction(sr, model, Interaction::Off, 0);
                for interaction in [Interaction::Ring, Interaction::Sync, Interaction::RingSync] {
                    let source = render_interaction(sr, model, interaction, 0);
                    let grains = render_interaction(sr, model, interaction, 1);
                    assert!(source.iter().chain(&grains).all(|x| x.is_finite()));
                    assert!(
                        difference(&plain, &source) > 1e-6,
                        "inactive {model:?}/{interaction:?}/{sr}"
                    );
                    assert!(
                        difference(&source, &grains) > 1e-6,
                        "identical external taps {model:?}/{interaction:?}/{sr}"
                    );
                }
            }
        }
        for model in [Model::Radias, Model::Wavefolder] {
            for interaction in [Interaction::Ring, Interaction::Sync, Interaction::RingSync] {
                let plain = render_interaction(48000, model, Interaction::Off, 0);
                let references: Vec<_> = (3..=5)
                    .map(|reference| render_interaction(48000, model, interaction, reference))
                    .collect();
                for reference in &references {
                    assert!(
                        reference.iter().any(|x| x.abs() > 1e-5),
                        "zero-level source stopped its carrier"
                    );
                    assert!(difference(&plain, reference) > 1e-6);
                }
                assert!(difference(&references[0], &references[1]) > 1e-6);
                assert!(difference(&references[1], &references[2]) > 1e-6);
            }
        }
    }
    #[test]
    fn all_osc4_shapes_and_both_controls_reach_every_layer() {
        use crate::resynth::FoldShape;
        for sr in [44100, 48000, 96000] {
            for shape in FoldShape::ALL {
                let (mut engine, sample, mut settings) = setup(sr);
                settings.count = 4;
                for (i, o) in settings.oscillators.iter_mut().enumerate() {
                    o.model = Model::Wavefolder;
                    o.waveform = Waveform::ALL[i];
                    o.fold_shape = shape;
                    o.control1 = 0.2;
                    o.control2 = 0.5;
                }
                engine.configure(settings, &sample, 60.0);
                let mut energy = 0.0;
                for n in 0..4096 {
                    let out = engine.process(
                        [0.0; 2],
                        tone(n, engine.frequencies[7], sr),
                        0.0,
                        1.0,
                        &sample,
                        &(0..sample.len()),
                    );
                    assert!(out.iter().all(|x| x.is_finite() && x.abs() < 8.0));
                    energy += out[0] * out[0];
                }
                assert!(energy > 1e-7, "silent OSC4 {shape:?}/{sr}");
                for i in 0..4 {
                    let phase = engine.layers[i].carriers[7].folder.phase;
                    settings.oscillators[i].control1 = 0.9;
                    settings.oscillators[i].control2 = 0.1;
                    engine.configure(settings, &sample, 60.0);
                    assert_eq!(
                        engine.layers[i].carriers[7].folder.phase, phase,
                        "control edit reset OSC4"
                    );
                    assert!(engine.layers[i].fold_controls[0] > 0.89);
                    assert!(engine.layers[i].fold_controls[1] < -0.7);
                }
            }
        }
    }
    #[test]
    fn all_radias_types_and_modes_render_at_host_rates() {
        for sr in [44100, 48000, 96000] {
            let (mut engine, sample, mut settings) = setup(sr);
            for waveform in Waveform::ALL {
                for mode in Mode::ALL {
                    if !waveform.analog() && mode != Mode::Wave {
                        continue;
                    }
                    settings.oscillators[0].waveform = waveform;
                    settings.oscillators[0].mode = mode;
                    engine.configure(settings, &sample, 60.0);
                    let mut energy = 0.0;
                    for n in 0..4096 {
                        let source = tone(n, engine.frequencies[7], sr);
                        let out =
                            engine.process([0.0; 2], source, 0.3, 1.0, &sample, &(0..sample.len()));
                        assert!(
                            out.iter().all(|x| x.is_finite() && x.abs() < 16.0),
                            "{sr} {waveform:?} {mode:?}"
                        );
                        energy += out[0] * out[0];
                    }
                    assert!(energy > 1e-7, "silent {sr} {waveform:?} {mode:?}: {energy}");
                }
            }
        }
    }
    #[test]
    fn analysis_selects_bands_and_envelopes_release_to_silence() {
        let (mut engine, sample, mut settings) = setup(48000);
        settings.release = 0.0;
        settings.attack = 0.0;
        engine.configure(settings, &sample, 60.0);
        for n in 0..24000 {
            engine.process(
                [0.0; 2],
                tone(n, engine.frequencies[8], 48000),
                0.0,
                1.0,
                &sample,
                &(0..sample.len()),
            );
        }
        let levels = engine.levels();
        let peak = levels
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(b.1))
            .unwrap()
            .0;
        assert_eq!(peak, 8);
        assert!(levels[8] > levels[2] * 10.0 && levels[8] > levels[14] * 10.0);
        for _ in 0..48000 {
            engine.process([0.0; 2], [0.0; 2], 0.0, 1.0, &sample, &(0..sample.len()));
        }
        assert!(engine.levels().iter().all(|x| *x < 0.0001));
        assert!(engine.gains.iter().all(|x| *x < 0.0001));
    }
    #[test]
    fn scalar_edits_keep_phase_and_disabled_processing_is_exact() {
        let (mut engine, sample, mut settings) = setup(48000);
        engine.configure(settings, &sample, 60.0);
        for n in 0..1024 {
            engine.process(
                [0.0; 2],
                tone(n, 440.0, 48000),
                0.0,
                1.0,
                &sample,
                &(0..sample.len()),
            );
        }
        let phases = engine.layers[0]
            .carriers
            .each_ref()
            .map(|c| c.oscillator.phase);
        settings.oscillators[0].control1 = 0.8;
        settings.oscillators[0].transpose = 12.0;
        settings.attack = 0.2;
        engine.configure(settings, &sample, 60.0);
        assert_eq!(
            phases,
            engine.layers[0]
                .carriers
                .each_ref()
                .map(|c| c.oscillator.phase)
        );
        settings.enabled = false;
        engine.configure(settings, &sample, 60.0);
        for _ in 0..129 {
            engine.process([0.0; 2], [0.0; 2], 0.0, 1.0, &sample, &(0..sample.len()));
        }
        assert!(!engine.active());
        let dry = [f32::from_bits(0x3e723456), -0.0];
        let out = engine.process(dry, [1.0; 2], 0.0, 1.0, &sample, &(0..sample.len()));
        assert_eq!(dry.map(f32::to_bits), out.map(f32::to_bits));
    }
    #[test]
    fn filters_after_an_oscillator_leave_sync_ring_and_feedback_alone() {
        use crate::{
            preset::{FilterPosition, FilterSlot},
            resynth::Interaction,
        };
        let (mut plain, sample, mut settings) = setup(48000);
        let (mut filtered, _, _) = setup(48000);
        settings.count = 4;
        for (i, o) in settings.oscillators.iter_mut().enumerate() {
            o.waveform = [
                Waveform::Saw,
                Waveform::Sine,
                Waveform::Triangle,
                Waveform::Pulse,
            ][i];
            // OSC 2..4 sync and ring from OSC 1, which also cross-modulates OSC 2.
            if i > 0 {
                o.interaction = Interaction::RingSync;
                o.interaction_source = 2;
            }
            o.cross_source = u8::from(i == 1);
        }
        plain.configure(settings, &sample, 60.0);
        filtered.configure(settings, &sample, 60.0);
        let slots = [FilterSlot {
            id: 1,
            position: FilterPosition::AfterOsc1,
            cutoff: 0.05,
            ..FilterSlot::default()
        }];
        let mut banks = crate::dsp::filter_slots::Prepared::default();
        banks.prepare(&slots, 48000);
        crate::dsp::filter_slots::configure_shared(
            &slots,
            &mut banks.0,
            60.0,
            [0.0; crate::preset::MODULATORS],
        );
        let range = 0..sample.len();
        let mut changed = false;
        for n in 0..12_000 {
            let input = tone(n, 440.0, 48000);
            let a =
                plain.process_with_grains([0.0; 2], input, input, 0.0, 1.0, &sample, &range, true);
            let b = filtered.process_with_filters(
                [0.0; 2],
                input,
                input,
                0.0,
                1.0,
                &sample,
                &range,
                true,
                [1, 0, 0, 0],
                &mut banks.0,
            );
            changed |= a != b;
            let bits = |r: &Resynth| {
                (
                    r.taps.map(|t| t.map(f32::to_bits)),
                    r.feedback.map(|t| t.map(f32::to_bits)),
                    r.wraps,
                )
            };
            assert_eq!(bits(&plain), bits(&filtered), "frame {n}");
        }
        assert!(changed, "the filter after OSC 1 never shaped the output");
    }

    #[test]
    fn stopped_carriers_pause_in_silence_without_changing_the_output() {
        let (mut playing, sample, mut settings) = setup(48000);
        let (mut stopping, _, _) = setup(48000);
        settings.count = 4;
        for (i, o) in settings.oscillators.iter_mut().enumerate() {
            o.waveform = [
                Waveform::Saw,
                Waveform::Sine,
                Waveform::Formant,
                Waveform::Triangle,
            ][i];
            if i == 1 {
                o.model = Model::Wavefolder;
            }
        }
        playing.configure(settings, &sample, 60.0);
        stopping.configure(settings, &sample, 60.0);
        let range = 0..sample.len();
        let phases = |r: &Resynth| {
            (
                r.layers[0].carriers[3].oscillator.phase.0,
                r.layers[1].carriers[3].folder.phase,
            )
        };
        let mut paused = 0;
        // Play a tone, stop at frame 12000 and stay silent: the stopped copy
        // pauses once every band has closed, sample for sample identical.
        for n in 0..60_000 {
            let input = if n < 12_000 {
                tone(n, 440.0, 48000)
            } else {
                [0.0; 2]
            };
            let before = phases(&stopping);
            let a = playing.process_with_grains(
                [0.1, -0.2],
                input,
                input,
                0.0,
                1.0,
                &sample,
                &range,
                true,
            );
            let b = stopping.process_with_grains(
                [0.1, -0.2],
                input,
                input,
                0.0,
                1.0,
                &sample,
                &range,
                n < 12_000,
            );
            assert_eq!(a.map(f32::to_bits), b.map(f32::to_bits), "frame {n}");
            if n >= 12_000 && stopping.gains.iter().all(|&g| g == 0.0) {
                assert_eq!(
                    phases(&stopping),
                    before,
                    "carriers advanced while paused at {n}"
                );
                paused += 1;
            }
        }
        assert!(paused > 30_000, "paused for only {paused} frames");
        assert_ne!(phases(&playing), phases(&stopping));
        // Both resume normally; only their oscillator phases differ.
        let mut energy = [0.0; 2];
        for n in 0..12_000 {
            let input = tone(n, 440.0, 48000);
            for (r, e) in [&mut playing, &mut stopping].into_iter().zip(&mut energy) {
                let out =
                    r.process_with_grains([0.0; 2], input, input, 0.0, 1.0, &sample, &range, true);
                assert!(out.iter().all(|x| x.is_finite()));
                *e += out[0] * out[0];
            }
        }
        assert!(energy.iter().all(|&e| e > 0.001), "{energy:?}");
    }

    #[test]
    fn four_independent_layers_cross_feedback_and_pcm_trim_stay_bounded() {
        let (mut engine, sample, mut settings) = setup(48000);
        settings.count = 4;
        for (i, o) in settings.oscillators.iter_mut().enumerate() {
            o.waveform = [
                Waveform::Saw,
                Waveform::Pcm,
                Waveform::Formant,
                Waveform::Sine,
            ][i];
            o.mode = if o.waveform.analog() {
                Mode::Cross
            } else {
                Mode::Wave
            };
            o.cross_source = ((i + 1) % 4 + 1) as u8;
            o.control1 = 0.99;
            o.control2 = 1.0;
            o.transpose = i as f32 * 12.0 - 12.0;
        }
        engine.configure(settings, &sample, 60.0);
        let mut energy = 0.0;
        for n in 0..12000 {
            let out = engine.process(
                [0.0; 2],
                tone(n, 440.0, 48000),
                0.8,
                1.0,
                &sample,
                &(17..19),
            );
            assert!(out.iter().all(|x| x.is_finite() && x.abs() < 16.0));
            energy += out[0] * out[0];
        }
        assert!(energy > 0.001);
    }

    #[test]
    fn shared_source_carriers_do_not_multiply_with_a_midi_chord() {
        use crate::{dsp::Engine, midi::Message, preset::Preset};
        use std::sync::Arc;
        let sample = Arc::new(
            SampleBuffer::new(
                "Tone".into(),
                8000,
                (0..8000).map(|n| tone(n, 440.0, 8000)).collect(),
            )
            .unwrap(),
        );
        let render = |count: usize| {
            let mut p = Preset::default();
            p.color_chain.clear();
            p.params.filter_env_depth = 0.0;
            p.params.filter_resonance = 0.0;
            p.envs[0].attack = 0.0;
            p.envs[0].decay = 0.0;
            p.envs[0].sustain = 1.0;
            p.resynth.enabled = true;
            p.resynth.mix = 1.0;
            let mut e = Engine::new(8000, sample.clone(), p);
            e.set_midi_gate(true);
            e.set_playing(true);
            for note in 0..count {
                e.midi(Message::NoteOn {
                    channel: 0,
                    note: 48 + note as u8 * 3,
                    velocity: 127,
                });
            }
            let mut energy = 0.0;
            for n in 0..6000 {
                let x = e.process_frame();
                if n > 2000 {
                    energy += x[0] * x[0];
                }
            }
            energy
        };
        let single = render(1);
        let chord = render(8);
        assert!(single > 0.01);
        assert!(
            (chord / single - 1.0).abs() < 0.02,
            "one={single}, eight={chord}"
        );
    }
    #[test]
    fn pcm_overlap_has_a_continuous_seam_and_handles_large_steps_and_tiny_trims() {
        // Deliberately different endpoints expose restarting the consumed head.
        let sample = SampleBuffer::new(
            "Ramp".into(),
            48000,
            (0..1024).map(|i| [i as f32 / 1024.0; 2]).collect(),
        )
        .unwrap();
        let o = crate::resynth::Oscillator {
            waveform: Waveform::Pcm,
            control2: 1.0,
            ..Default::default()
        };
        let pcm = PcmLoop::new(&sample, &(0..1024), o);
        let mut carrier = Carrier {
            pcm_phase: 1023.0,
            ..Default::default()
        };
        let tail = pcm.read(&mut carrier, &sample, &(0..1024));
        let head = pcm.read(&mut carrier, &sample, &(0..1024));
        assert!(
            (tail[0] - head[0]).abs() < 0.002,
            "PCM overlap seam: {tail:?} -> {head:?}"
        );
        for rate in [0.1, 37.3, 10_000.75, 1_000_000.1] {
            carrier.pcm_rate = rate;
            for _ in 0..16 {
                assert!(
                    pcm.read(&mut carrier, &sample, &(0..1024))
                        .iter()
                        .all(|x| x.is_finite())
                );
                assert!((pcm.fade..pcm.length).contains(&carrier.pcm_phase));
            }
        }
        let tiny = PcmLoop::new(&sample, &(17..18), o);
        assert_eq!(tiny.fade, 0.0);
        for _ in 0..16 {
            assert!(
                tiny.read(&mut carrier, &sample, &(17..18))
                    .iter()
                    .all(|x| x.is_finite())
            );
        }
        let inactive = PcmLoop::new(
            &sample,
            &(0..1024),
            crate::resynth::Oscillator {
                control2: 0.001,
                ..o
            },
        );
        assert_eq!(inactive.fade, 0.0);
        carrier.pcm_phase = 1023.0;
        carrier.pcm_rate = 1.0;
        inactive.read(&mut carrier, &sample, &(0..1024));
        assert_eq!(
            carrier.pcm_phase, 0.0,
            "Sub-sample overlap changed the loop period"
        );
    }
}
