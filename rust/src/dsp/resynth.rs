//! Streaming 16-band excitation of four Rustias carrier layers.
//! All histories and tables are allocated before rendering; scalar edits keep
//! phases, envelope histories and the original OSC1 four-frame control slew.
use super::{SampleBuffer, radias_oscillator};
use crate::{
    Stereo,
    resynth::{BANDS, Input, MAX_OSCILLATORS, Mode, Settings, Waveform},
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

#[derive(Clone, Copy, Default)]
struct Band {
    g: f32,
    a: f32,
    z: [[f32; 2]; 2],
}

impl Band {
    fn new(hz: f32, sr: f32) -> Self {
        let g = (PI * hz / sr).tan();
        Self {
            g,
            a: 1.0 / (1.0 + g * (g + 1.0 / 2.8)),
            ..Self::default()
        }
    }
    fn next(&mut self, input: Stereo) -> Stereo {
        std::array::from_fn(|c| {
            let v3 = input[c] - self.z[c][1];
            let v1 = self.a * (self.z[c][0] + self.g * v3);
            let v2 = self.z[c][1] + self.g * v1;
            self.z[c] = [2.0 * v1 - self.z[c][0], 2.0 * v2 - self.z[c][1]];
            for z in &mut self.z[c] {
                if z.abs() < 1e-20 {
                    *z = 0.0;
                }
            }
            v1 / 2.8
        })
    }
}

#[derive(Clone, Copy, Default)]
struct Lowpass {
    b: [f32; 3],
    a: [f32; 2],
    z: [[f32; 2]; 2],
}
impl Lowpass {
    fn new(q: f32) -> Self {
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
    fn next(&mut self, input: Stereo) -> Stereo {
        std::array::from_fn(|c| {
            let y = self.b[0] * input[c] + self.z[c][0];
            self.z[c] = [
                self.b[1] * input[c] - self.a[0] * y + self.z[c][1],
                self.b[2] * input[c] - self.a[1] * y,
            ];
            for z in &mut self.z[c] {
                if z.abs() < 1e-20 {
                    *z = 0.0;
                }
            }
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
}

struct PcmLoop {
    start: f64,
    length: f64,
    fade: f64,
}
impl PcmLoop {
    fn new(sample: &SampleBuffer, range: &Range<usize>, o: crate::resynth::Oscillator) -> Self {
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
        let p = if carrier.pcm_phase < self.length {
            carrier.pcm_phase
        } else {
            carrier.pcm_phase.rem_euclid(self.length)
        };
        let mut x = sample.read_region_level(self.start + p, carrier.pcm_level, range);
        if self.fade > 0.0 && p > self.length - self.fade {
            let y = sample.read_region_level(
                self.start + p - self.length + self.fade,
                carrier.pcm_level,
                range,
            );
            let t = ((p - self.length + self.fade) / self.fade) as f32;
            let mix = t * t * (3.0 - 2.0 * t);
            x = std::array::from_fn(|c| x[c] + (y[c] - x[c]) * mix);
        }
        let next = p + carrier.pcm_rate;
        carrier.pcm_phase = if next < self.length {
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
        }
    }
}

pub struct Resynth {
    sample_rate: f32,
    oversample: usize,
    frequencies: [f32; BANDS],
    bands: [Band; BANDS],
    envelopes: BandEnvelopes,
    levels: [f32; BANDS],
    gates: [bool; BANDS],
    gains: [f32; BANDS],
    feedback: [[f32; BANDS]; MAX_OSCILLATORS],
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
}
impl Resynth {
    pub fn new(sr: u32) -> Self {
        let frequencies = std::array::from_fn(|i| {
            60.0 * ((12000.0_f32.min(sr as f32 * 0.4) / 60.0).powf(i as f32 / (BANDS - 1) as f32))
        });
        Self {
            sample_rate: sr as f32,
            oversample: if sr < 96000 { 2 } else { 1 },
            frequencies,
            bands: frequencies.map(|hz| Band::new(hz, sr as f32)),
            envelopes: Default::default(),
            levels: [0.0; BANDS],
            gates: [false; BANDS],
            gains: [0.0; BANDS],
            feedback: [[0.0; BANDS]; MAX_OSCILLATORS],
            layers: std::array::from_fn(|_| Layer::default()),
            tables: Box::new(radias_oscillator::Tables::new()),
            waveform: Box::new(radias_oscillator::waveform_table()),
            lowpass: [0.5097956, 0.6013449, 0.8999762, 2.5629154].map(Lowpass::new),
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
        }
    }
    pub fn clear(&mut self) {
        for band in &mut self.bands {
            band.z = [[0.0; 2]; 2];
        }
        self.envelopes.levels.fill(0);
        self.levels.fill(0.0);
        self.gains.fill(0.0);
        self.gates.fill(false);
        self.feedback.fill([0.0; BANDS]);
        for filter in &mut self.lowpass {
            filter.z = [[0.0; 2]; 2];
        }
        for layer in &mut self.layers {
            for carrier in &mut layer.carriers {
                carrier.pcm_phase = 0.0;
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
        for (i, layer) in self.layers.iter_mut().enumerate() {
            let o = settings.oscillators[i];
            layer.target_gain = if i < settings.count as usize && o.enabled {
                o.level / enabled.sqrt()
            } else {
                0.0
            };
            if !self.configured || layer.settings != o || source_changed {
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
            dry
        };
        let analyzed: [Stereo; BANDS] = std::array::from_fn(|i| self.bands[i].next(input));
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
            self.gains[band] += (gain - self.gains[band]) * 0.0625;
        }
        for layer in &mut self.layers {
            layer.gain += (layer.target_gain - layer.gain).clamp(-1.0 / 128.0, 1.0 / 128.0);
        }
        let mut wet = [0.0; 2];
        for sub in 0..self.oversample {
            wet = [0.0; 2];
            // Slew once per four host frames, including when carriers run at 2x.
            let frame = if self.frame & 3 == 3 && sub + 1 == self.oversample {
                self.frame
            } else {
                self.frame & !3
            };
            for layer_id in 0..MAX_OSCILLATORS {
                let layer = &mut self.layers[layer_id];
                if layer.target_gain == 0.0 && layer.gain == 0.0 {
                    self.feedback[layer_id].fill(0.0);
                    continue;
                }
                let o = layer.settings;
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
                        continue;
                    }
                    let modulator = if o.cross_source == 0 {
                        (analyzed_band[0] + analyzed_band[1]) * 0.5
                    } else {
                        self.feedback[o.cross_source as usize - 1][band]
                    };
                    let stereo = match o.waveform {
                        Waveform::AudioIn => {
                            analyzed_band.map(|x| x * self.sensitivity * self.gains[band])
                        }
                        Waveform::Pcm => pcm_loop
                            .as_ref()
                            .unwrap()
                            .read(carrier, sample, range)
                            .map(|x| x * self.gains[band]),
                        _ => {
                            let modulator = Sample(
                                (modulator as f64 * i32::MAX as f64)
                                    .clamp(i32::MIN as f64, i32::MAX as f64)
                                    as i32,
                            );
                            let x = carrier
                                .controller
                                .next(
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
                                )
                                .0 as f32
                                / 1073741824.0;
                            [x * self.gains[band]; 2]
                        }
                    };
                    self.feedback[layer_id][band] = (stereo[0] + stereo[1]) * 0.5;
                    for c in 0..2 {
                        wet[c] += stereo[c] * layer.gain * layer.pan[c] * 0.6;
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
