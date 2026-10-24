use crate::{
    preset::{Env, EnvMode, Lfo, Waveform},
    tables::{self, Random},
};
use radias_synth_domain::{
    amp_envelope::{AmpEnvelopeParameters, EnvelopeStage},
    envelope_segment::{EnvelopeCurves, EnvelopeTiming, EnvelopeTimingTables},
    mod_envelope::ModEnvelope,
    q_envelope,
};
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Stage {
    #[default]
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

/// Rustias EG controllers tick at 2 kHz; levels are 0..=32766.
const CONTROLLER_HZ: u32 = 2000;
const FULL_SCALE: f32 = 32766.0;
/// Rustias per-sample gain smoothing coefficient 0x1d4 at 48 kHz.
const SMOOTHING_48K: f32 = 468.0 / 32768.0;

struct RustiasTables {
    curves: EnvelopeCurves,
    timing: EnvelopeTimingTables,
}

/// Firmware-free RADIAS EG tables, as Rustias `standalone_tables::controllers`
/// builds them (that crate is not a SAD PET dependency). Built once, in place.
fn rustias_tables() -> &'static RustiasTables {
    static TABLES: OnceLock<RustiasTables> = OnceLock::new();
    TABLES.get_or_init(|| {
        let seconds = |value: usize| 0.003 * 2000.0f64.powf(value as f64 / 127.0);
        let curve = |kind: usize, x: f64| match kind {
            0 => x.powi(3),
            1 => x * x,
            2 => x * x * (3.0 - 2.0 * x),
            4 => x.sqrt(),
            6 => 1.0 - (1.0 - x).powi(3),
            7 => x.powf(1.5),
            _ => x,
        };
        RustiasTables {
            curves: EnvelopeCurves {
                values: std::array::from_fn(|c| {
                    std::array::from_fn(|i| (curve(c, i as f64 / 256.0) * 65536.0) as u16)
                }),
            },
            timing: EnvelopeTimingTables {
                increments: std::array::from_fn(|_| {
                    std::array::from_fn(|i| {
                        (0x00ff_ffff as f64 / (2000.0 * seconds(i))).round() as u32
                    })
                }),
                scale: std::array::from_fn(|i| {
                    (256.0 * 2.0f64.powf((i as f64 - 64.0) / 32.0)).round() as u16
                }),
                key_tracking: std::array::from_fn(|i| ((i as i32 - 64) * 256) as i16),
            },
        }
    })
}

/// A Rustias EG type driven by its own 2 kHz controller clock. The RADIAS
/// type is the EG1/EG3 `ModEnvelope`; the Q types are `q_envelope::Envelope`.
#[derive(Clone, Copy, Debug)]
struct Rustias {
    mode: EnvMode,
    parameters: AmpEnvelopeParameters,
    settings: q_envelope::Settings,
    radias: ModEnvelope,
    q: q_envelope::Envelope,
    held: bool,
    rate: u32,
    clock: u32,
    smoothing: f32,
    /// Smoothed output, like the Rustias per-sample amplifier gain.
    level: f32,
}
impl Default for Rustias {
    fn default() -> Self {
        let p = Env::default();
        Self {
            mode: EnvMode::Legacy,
            parameters: Self::parameters(&p),
            settings: Self::settings(&p),
            radias: ModEnvelope::default(),
            q: q_envelope::Envelope::default(),
            held: false,
            rate: 48_000,
            clock: 0,
            smoothing: SMOOTHING_48K,
            level: 0.0,
        }
    }
}
impl Rustias {
    /// Velocity and key time scaling stay neutral: SAD PET has no such controls.
    fn parameters(p: &Env) -> AmpEnvelopeParameters {
        AmpEnvelopeParameters {
            attack: Env::rustias_control(p.attack),
            decay: Env::rustias_control(p.decay),
            sustain: Env::rustias_control(p.sustain),
            release: Env::rustias_control(p.release),
            curve: p.curve & 7,
            velocity_sensitivity: 64,
            key_tracking: 64,
            velocity: 64,
            note: 60,
        }
    }
    fn settings(p: &Env) -> q_envelope::Settings {
        q_envelope::Settings {
            mode: p.mode.rustias_mode().unwrap_or(0),
            attack_level: Env::rustias_control(p.attack_level),
            decay2: Env::rustias_control(p.decay2),
            sustain2: Env::rustias_control(p.sustain2),
        }
    }
    fn configure(&mut self, sample_rate: f32, p: &Env) {
        // Engine::new configures every envelope: build the tables there, not
        // on the first note.
        rustias_tables();
        let rate = (sample_rate.round() as u32).max(1);
        if rate != self.rate {
            self.rate = rate;
            self.clock = 0;
            self.smoothing = 1.0 - (1.0 - SMOOTHING_48K).powf(48_000.0 / rate as f32);
        }
        let parameters = Self::parameters(p);
        let settings = Self::settings(p);
        if parameters == self.parameters && settings == self.settings {
            return;
        }
        // Rustias retargets a running envelope from its current level.
        if self.running() {
            let timing = &rustias_tables().timing;
            if self.mode == EnvMode::Radias {
                self.radias.edit(self.parameters, parameters, timing);
            } else {
                self.q.edit(self.parameters, parameters, settings, timing);
            }
        }
        self.parameters = parameters;
        self.settings = settings;
    }
    fn running(&self) -> bool {
        match self.mode {
            EnvMode::Legacy => false,
            EnvMode::Radias => !matches!(
                self.radias.envelope.stage,
                EnvelopeStage::Disabled | EnvelopeStage::Finished
            ),
            _ => self.q.stage != q_envelope::Stage::Finished,
        }
    }
    fn output(&self) -> u16 {
        if self.mode == EnvMode::Radias {
            self.radias.envelope.segment.level
        } else {
            self.q.level
        }
    }
    fn on(&mut self) {
        let tables = rustias_tables();
        self.held = true;
        self.clock = 0;
        if self.mode == EnvMode::Radias {
            self.radias
                .note_on(self.parameters, &tables.curves, &tables.timing, 0);
        } else {
            self.q
                .note_on(self.settings, self.parameters, &tables.timing);
        }
    }
    fn off(&mut self) {
        self.held = false;
        if !self.running() {
            return;
        }
        let timing = &rustias_tables().timing;
        if self.mode == EnvMode::Radias {
            self.radias.release(self.parameters, timing);
        } else {
            self.q.release(self.parameters, timing);
        }
    }
    fn kill(&mut self) {
        self.radias = ModEnvelope::default();
        self.q = q_envelope::Envelope::default();
        self.held = false;
        self.clock = 0;
        self.level = 0.0;
    }
    /// Continue a sounding envelope of another type from its current level,
    /// as Rustias does when an EG type changes under a held or released note.
    fn adopt(&mut self, level: f32, held: bool) {
        let tables = rustias_tables();
        let value = (level.clamp(0.0, 1.0) * FULL_SCALE).round() as u16;
        self.held = held;
        self.level = level;
        if self.mode == EnvMode::Radias {
            let p = self.parameters;
            let e = &mut self.radias.envelope;
            e.stage = if held {
                EnvelopeStage::Attack
            } else {
                EnvelopeStage::Release
            };
            e.dirty = true;
            e.increment_flags = 0;
            e.divider = 0;
            e.target = 0;
            e.segment.level = value;
            let increment = if held {
                tables.timing.increments[3][usize::from(p.attack & 127)]
            } else {
                tables.timing.increment(EnvelopeTiming {
                    curve: p.curve,
                    time: p.release,
                    velocity: p.velocity,
                    velocity_sensitivity: p.velocity_sensitivity,
                    note: p.note,
                    key_tracking: p.key_tracking,
                })
            };
            e.segment.begin(if held { 32766 } else { 0 }, increment);
        } else {
            self.q
                .adopt(self.settings, value, held, self.parameters, &tables.timing);
        }
    }
    #[inline(never)]
    fn next_sample(&mut self) -> f32 {
        if self.running() {
            self.clock += CONTROLLER_HZ;
            if self.clock >= self.rate {
                let tables = rustias_tables();
                while self.clock >= self.rate {
                    self.clock -= self.rate;
                    if self.mode == EnvMode::Radias {
                        self.radias
                            .tick(self.parameters, &tables.curves, &tables.timing);
                    } else {
                        self.q.tick(self.parameters, &tables.timing);
                    }
                }
            }
        } else if self.level == 0.0 {
            return 0.0;
        }
        let target = f32::from(self.output()) / FULL_SCALE;
        let next = self.level + (target - self.level) * self.smoothing;
        // Settle exactly once the step stops moving, and never into denormals.
        self.level = if next == self.level
            || (target - next).abs() < 1.0e-6
            || (!self.running() && next < 0.0001)
        {
            target
        } else {
            next
        };
        self.level
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Envelope {
    stage: Stage,
    value: f32,
    attack_step: f32,
    decay_coeff: f32,
    release_coeff: f32,
    sustain: f32,
    configured: Option<(f32, [f32; 4])>,
    /// Rustias EG types; unused while the type is `EnvMode::Legacy`.
    rustias: Rustias,
}
impl Envelope {
    pub fn configure(&mut self, sample_rate: f32, p: &Env) {
        if p.mode != self.rustias.mode {
            let (active, held, level) = if self.rustias.mode == EnvMode::Legacy {
                (
                    self.stage != Stage::Idle,
                    matches!(self.stage, Stage::Attack | Stage::Decay | Stage::Sustain),
                    self.value,
                )
            } else {
                (self.active(), self.rustias.held, self.rustias.level)
            };
            self.stage = Stage::Idle;
            self.value = 0.0;
            self.rustias.kill();
            self.rustias.mode = p.mode;
            self.rustias.configure(sample_rate, p);
            if active && p.mode == EnvMode::Legacy {
                self.value = level;
                self.stage = if held { Stage::Attack } else { Stage::Release };
            } else if active {
                self.rustias.adopt(level, held);
            }
        } else {
            self.rustias.configure(sample_rate, p);
        }
        let settings = (sample_rate, [p.attack, p.decay, p.sustain, p.release]);
        if self.configured == Some(settings) {
            return;
        }
        self.configured = Some(settings);
        self.attack_step = 1.0 / (p.attack_seconds() * sample_rate);
        self.decay_coeff = (-6.0 / (p.decay_seconds() * sample_rate)).exp();
        self.release_coeff = (-9.0 / (p.release_seconds() * sample_rate)).exp();
        self.sustain = p.sustain;
    }
    pub fn on(&mut self) {
        if self.rustias.mode != EnvMode::Legacy {
            return self.rustias.on();
        }
        // A retrigger attacks from the level still sounding: a jump to zero
        // under a held or releasing note clicks.
        self.stage = Stage::Attack;
    }
    pub fn off(&mut self) {
        if self.rustias.mode != EnvMode::Legacy {
            return self.rustias.off();
        }
        if self.stage != Stage::Idle {
            self.stage = Stage::Release;
        }
    }
    pub fn kill(&mut self) {
        self.rustias.kill();
        self.stage = Stage::Idle;
        self.value = 0.0;
    }
    pub fn active(&self) -> bool {
        if self.rustias.mode != EnvMode::Legacy {
            return self.rustias.running() || self.rustias.level != 0.0;
        }
        self.stage != Stage::Idle
    }
    /// Inlined into the per-frame voice loop: an idle or legacy stage costs a
    /// few instructions, while the Rustias types run out of line.
    #[inline]
    pub fn next_sample(&mut self) -> f32 {
        if self.rustias.mode != EnvMode::Legacy {
            return self.rustias.next_sample();
        }
        match self.stage {
            Stage::Idle => self.value = 0.0,
            Stage::Attack => {
                self.value += self.attack_step;
                if self.value >= 1.0 {
                    self.value = 1.0;
                    self.stage = Stage::Decay;
                }
            }
            Stage::Decay => {
                self.value = self.sustain + (self.value - self.sustain) * self.decay_coeff;
                if (self.value - self.sustain).abs() < 0.001 {
                    self.stage = Stage::Sustain;
                }
            }
            Stage::Sustain => self.value = self.sustain,
            Stage::Release => {
                self.value *= self.release_coeff;
                if self.value < 0.0001 {
                    self.kill();
                }
            }
        }
        self.value
    }
}

#[derive(Clone, Debug)]
pub struct Oscillator {
    pub phase: f64,
    from: f32,
    to: f32,
    random: Random,
}
impl Oscillator {
    pub fn new(seed: u64) -> Self {
        let mut random = Random(seed);
        let from = random.bipolar();
        let to = random.bipolar();
        Self {
            phase: 0.0,
            from,
            to,
            random,
        }
    }
    pub fn next_sample(&mut self, lfo: &Lfo, sample_rate: f32, bpm: f32, envelope: f32) -> f32 {
        let p = self.phase as f32;
        let result = match lfo.waveform {
            Waveform::Sine => tables::sine(self.phase),
            Waveform::Triangle => 1.0 - 4.0 * (p - 0.5).abs(),
            Waveform::Saw => p * 2.0 - 1.0,
            Waveform::Square => {
                if p < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Random => self.to,
            Waveform::SmoothRandom => self.from + (self.to - self.from) * (p * p * (3.0 - 2.0 * p)),
            Waveform::Envelope => envelope * 2.0 - 1.0,
        };
        let hz = if lfo.sync {
            bpm / 60.0 / lfo.division
        } else {
            lfo.rate_hz
        };
        self.phase += hz as f64 / sample_rate as f64;
        if self.phase >= 1.0 {
            self.phase = self.phase.rem_euclid(1.0);
            self.from = self.to;
            self.to = self.random.bipolar();
        }
        result
    }
}
