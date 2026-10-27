//! The voices of a Sampler timbre. Notes pick pads by key, pads pick zones by
//! key and velocity, and every zone plays its sample region with windowed-sinc
//! interpolation from the antialiasing mips, through the pad's envelope and
//! filter. Pads with a Pad FX timbre write into their own send instead of the
//! kit's mix; the host feeds each send to its timbre.
use super::{SampleBuffer, fast};
use crate::{
    Stereo,
    sampler::{
        FilterKind, LayerMode, LoopMode, MAX_PADS, MAX_VOICES, Pad, Settings, Trigger, Zone,
    },
    tables::Random,
};
use std::{collections::BTreeMap, sync::Arc};

/// Samples by pool id, shared by the voices that play them.
pub type Pool = BTreeMap<String, Arc<SampleBuffer>>;

/// A kit's runtime built off the audio thread from its settings and pool,
/// which the engine takes over in `exchange_preset`; the one it replaces
/// retires with the outgoing preset.
#[derive(Default)]
pub struct Prepared(pub Option<Box<Runtime>>);
impl Clone for Prepared {
    fn clone(&self) -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for Prepared {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreparedSampler")
            .field("runtime", &self.0.is_some())
            .finish()
    }
}
impl Prepared {
    pub fn prepare(&mut self, settings: Option<&Settings>, pool: &Pool, routes: &[u32], rate: u32) {
        self.0 = settings.map(|s| Runtime::prepared(s, pool, routes, rate));
    }
}

/// What an editor shows of a kit, in storage the audio thread fills
/// without allocating: each sounding voice's pad id, zone and frame, and
/// the notes played per pad id.
#[derive(Clone, Copy)]
pub struct Snapshot {
    pub voices: [(u32, u32, f64); MAX_VOICES],
    pub voice_count: usize,
    pub hits: [(u32, u32); MAX_PADS],
    pub pad_count: usize,
}
impl Default for Snapshot {
    fn default() -> Self {
        Self {
            voices: [(0, 0, 0.0); MAX_VOICES],
            voice_count: 0,
            hits: [(0, 0); MAX_PADS],
            pad_count: 0,
        }
    }
}
impl Snapshot {
    /// The editor's view, with the pool ids it holds.
    pub fn view(&self, loaded: Vec<String>) -> crate::sampler::View {
        crate::sampler::View {
            voices: self.voices[..self.voice_count]
                .iter()
                .map(|&(pad, zone, frame)| (pad, zone as usize, frame))
                .collect(),
            hits: self.hits[..self.pad_count].to_vec(),
            loaded,
        }
    }
}

/// Control-rate updates: pitch, gains and filter coefficients.
const CONTROL: u32 = 16;
/// A choked or stolen voice fades out over this many seconds.
const KILL_SECONDS: f32 = 0.005;
/// Envelopes end once they fall below −80 dB.
const SILENT: f32 = 1e-4;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum Stage {
    #[default]
    Idle,
    Attack,
    Hold,
    Decay,
    Sustain,
    Release,
}

/// Frames for a time in milliseconds, at least one.
fn frames(ms: f32, rate: f32) -> f32 {
    (ms * 0.001 * rate).max(1.0)
}
/// The per-frame factor that falls by 80 dB in `ms`.
fn falloff(ms: f32, rate: f32) -> f32 {
    (SILENT.ln() / frames(ms, rate)).exp()
}

#[derive(Clone, Copy, Debug, Default)]
struct Amp {
    stage: Stage,
    level: f32,
    attack: f32,
    hold: u32,
    decay: f32,
    sustain: f32,
    release: f32,
}
impl Amp {
    fn start(&mut self, envelope: &crate::sampler::Envelope, attack_scale: f32, rate: f32) {
        *self = Self {
            stage: Stage::Attack,
            level: 0.0,
            attack: 1.0 / frames(envelope.attack * attack_scale, rate),
            hold: (envelope.hold * 0.001 * rate) as u32,
            decay: falloff(envelope.decay, rate),
            sustain: envelope.sustain,
            release: falloff(envelope.release, rate),
        };
        if envelope.attack * attack_scale <= 0.0 {
            self.level = 1.0;
            self.stage = Stage::Hold;
        }
    }
    fn release(&mut self) {
        if self.stage != Stage::Idle {
            self.stage = Stage::Release;
        }
    }
    #[inline]
    fn next(&mut self) -> f32 {
        match self.stage {
            Stage::Idle => return 0.0,
            Stage::Attack => {
                self.level += self.attack;
                if self.level >= 1.0 {
                    self.level = 1.0;
                    self.stage = Stage::Hold;
                }
            }
            Stage::Hold => {
                if self.hold == 0 {
                    self.stage = Stage::Decay;
                } else {
                    self.hold -= 1;
                }
            }
            Stage::Decay => {
                self.level = self.sustain + (self.level - self.sustain) * self.decay;
                if (self.level - self.sustain).abs() < SILENT {
                    self.level = self.sustain;
                    self.stage = Stage::Sustain;
                }
            }
            Stage::Sustain => {}
            Stage::Release => self.level *= self.release,
        }
        if self.level < SILENT && matches!(self.stage, Stage::Release | Stage::Sustain) {
            self.stage = Stage::Idle;
            self.level = 0.0;
        }
        self.level
    }
}

/// Attack/decay envelope of the filter cutoff, 0–1.
#[derive(Clone, Copy, Debug, Default)]
struct Sweep {
    level: f32,
    attack: f32,
    decay: f32,
    rising: bool,
}
impl Sweep {
    fn start(&mut self, filter: &crate::sampler::Filter, rate: f32) {
        self.attack = 1.0 / frames(filter.attack, rate);
        self.decay = falloff(filter.decay, rate);
        self.rising = filter.attack > 0.0;
        self.level = if self.rising { 0.0 } else { 1.0 };
    }
    /// Advances `count` frames at once, at control rate.
    fn advance(&mut self, count: u32) {
        for _ in 0..count {
            if self.rising {
                self.level += self.attack;
                if self.level >= 1.0 {
                    self.level = 1.0;
                    self.rising = false;
                }
            } else {
                self.level = fast::flush(self.level * self.decay);
            }
        }
    }
}

/// Topology-preserving state-variable filter, stereo.
#[derive(Clone, Copy, Debug, Default)]
struct Svf {
    kind: u8,
    k: f32,
    a1: f32,
    a2: f32,
    a3: f32,
    s1: Stereo,
    s2: Stereo,
}
impl Svf {
    fn configure(&mut self, kind: FilterKind, cutoff: f32, resonance: f32, rate: f32) {
        self.kind = kind as u8;
        if kind == FilterKind::Off {
            return;
        }
        let cutoff = cutoff.clamp(16.0, rate * 0.45);
        let g = (std::f32::consts::PI * cutoff / rate).tan();
        self.k = 2.0 - 1.96 * resonance;
        self.a1 = 1.0 / (1.0 + g * (g + self.k));
        self.a2 = g * self.a1;
        self.a3 = g * self.a2;
    }
    #[inline]
    fn process(&mut self, input: Stereo) -> Stereo {
        if self.kind == FilterKind::Off as u8 {
            return input;
        }
        std::array::from_fn(|c| {
            let v3 = input[c] - self.s2[c];
            let v1 = self.a1 * self.s1[c] + self.a2 * v3;
            let v2 = self.s2[c] + self.a2 * self.s1[c] + self.a3 * v3;
            self.s1[c] = fast::flush(2.0 * v1 - self.s1[c]);
            self.s2[c] = fast::flush(2.0 * v2 - self.s2[c]);
            match self.kind {
                1 => v2,
                2 => input[c] - self.k * v1 - v2,
                3 => v1 * self.k,
                _ => input[c] - self.k * v1,
            }
        })
    }
}

#[derive(Clone, Default)]
struct Voice {
    sample: Option<Arc<SampleBuffer>>,
    active: bool,
    pad: usize,
    pad_id: u32,
    zone: usize,
    owner: u32,
    channel: u8,
    note: u8,
    velocity: f32,
    held: bool,
    sustained: bool,
    age: u64,
    /// Playback position: the frame itself, or mirrored when reversed.
    position: f64,
    forward: bool,
    /// Region and loop in playback positions.
    start: f64,
    end: f64,
    loop_start: f64,
    loop_end: f64,
    loop_mode: LoopMode,
    crossfade: f64,
    /// Reversed voices read frame `mirror - position`.
    mirror: Option<f64>,
    /// Region and fades in sample frames.
    region: [f64; 2],
    fades: [f64; 2],
    curves: [f32; 2],
    /// Frames per output frame before pitch modulation.
    base: f64,
    step: f64,
    level: usize,
    zone_gain: f32,
    zone_pan: f32,
    gains: Stereo,
    amp: Amp,
    sweep: Sweep,
    filter: Svf,
    pitch: f32,
    pitch_decay: f32,
    kill: f32,
    killing: bool,
    countdown: u32,
}
impl Voice {
    #[inline]
    fn frame_of(&self, position: f64) -> f64 {
        self.mirror.map_or(position, |m| m - position)
    }
    fn looping(&self) -> bool {
        match self.loop_mode {
            LoopMode::Off => false,
            LoopMode::Sustain => self.held || self.sustained,
            LoopMode::Forward | LoopMode::Alternate => true,
        }
    }
    fn start_kill(&mut self) {
        self.killing = true;
    }
    /// Pitch, gains and filter from the pad as it is now.
    fn control(&mut self, pad: &Pad, bend: f32, rate: f32) {
        let bounced = pad.bounce.is_some();
        let semitones = pad.tune + pad.fine * 0.01 + bend + if bounced { 0.0 } else { self.pitch };
        self.step = self.base * fast::exp2(semitones / 12.0) as f64;
        if let Some(sample) = &self.sample {
            self.level = sample.level_for_rate(self.step, true);
        }
        let gain = fast::gain_db(pad.gain + self.zone_gain);
        let pan = (pad.pan + self.zone_pan).clamp(-1.0, 1.0);
        self.gains = [gain * (1.0 - pan).min(1.0), gain * (1.0 + pan).min(1.0)];
        let filter = &pad.filter;
        if !bounced && filter.kind != FilterKind::Off {
            let octaves = filter.envelope * self.sweep.level
                + filter.keytrack * (self.note as f32 - 60.0) / 12.0
                - filter.velocity * (1.0 - self.velocity);
            self.filter.configure(
                filter.kind,
                filter.cutoff * fast::exp2(octaves.clamp(-12.0, 12.0)),
                filter.resonance,
                rate,
            );
        } else {
            self.filter.configure(FilterKind::Off, 0.0, 0.0, rate);
        }
    }
    /// One frame of this voice, or `None` once it has ended.
    #[inline]
    fn render(&mut self, kill_step: f32) -> Option<Stereo> {
        let sample = self.sample.as_ref()?;
        let amp = self.amp.next();
        if self.amp.stage == Stage::Idle {
            return None;
        }
        if self.killing {
            self.kill -= kill_step;
            if self.kill <= 0.0 {
                return None;
            }
        }
        let frame = self.frame_of(self.position);
        let mut value = sample.read_level(frame, self.level);
        let looping = self.looping();
        if looping && self.crossfade > 0.0 && self.loop_mode != LoopMode::Alternate {
            let from = self.loop_end - self.crossfade;
            if self.position >= from {
                // The last frame before the wrap is all the audio before the loop.
                let t = ((self.position - from + 1.0) / self.crossfade) as f32;
                let length = self.loop_end - self.loop_start;
                let other = sample.read_level(self.frame_of(self.position - length), self.level);
                let (out, into) = ((1.0 - t).max(0.0).sqrt(), t.min(1.0).sqrt());
                value = std::array::from_fn(|c| value[c] * out + other[c] * into);
            }
        }
        let mut gain = amp * self.kill;
        let [fade_in, fade_out] = self.fades;
        if fade_in > 0.0 && frame - self.region[0] < fade_in {
            gain *= curve(((frame - self.region[0]) / fade_in) as f32, self.curves[0]);
        }
        if fade_out > 0.0 && self.region[1] - frame < fade_out {
            gain *= curve(((self.region[1] - frame) / fade_out) as f32, self.curves[1]);
        }
        let value = self.filter.process(value);
        let output = [
            value[0] * gain * self.gains[0],
            value[1] * gain * self.gains[1],
        ];
        // Advance, then loop or end.
        if self.forward {
            self.position += self.step;
        } else {
            self.position -= self.step;
        }
        if looping {
            let length = self.loop_end - self.loop_start;
            match self.loop_mode {
                LoopMode::Alternate => {
                    if self.forward && self.position >= self.loop_end {
                        self.position = (2.0 * self.loop_end - self.position).max(self.loop_start);
                        self.forward = false;
                    } else if !self.forward && self.position < self.loop_start {
                        self.position = (2.0 * self.loop_start - self.position).min(self.loop_end);
                        self.forward = true;
                    }
                }
                _ => {
                    if self.position >= self.loop_end {
                        self.position =
                            self.loop_start + (self.position - self.loop_end).rem_euclid(length);
                    }
                }
            }
        } else if self.position >= self.end || self.position < self.start {
            // This frame still sounds; the voice ends on the next.
            self.amp.stage = Stage::Idle;
        }
        Some(output)
    }
}

/// Fade gain for `x` from 0 (silent) to 1 (full): `x^(4^curve)`.
#[inline]
fn curve(x: f32, curve: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    if curve == 0.0 || x == 0.0 {
        x
    } else {
        fast::exp2(fast::exp2(2.0 * curve) * fast::log2(x))
    }
}

struct PadMap {
    id: u32,
    /// The playing zones' samples, `None` where the pool lacks one.
    samples: Vec<Option<Arc<SampleBuffer>>>,
    routed: bool,
    cycle: usize,
}

/// The audio thread's view of a Sampler timbre.
pub struct Runtime {
    rate: f32,
    pub pool: Pool,
    pads: Vec<PadMap>,
    voices: Vec<Voice>,
    polyphony: usize,
    sends: Vec<Stereo>,
    routes: Vec<u32>,
    random: Random,
    clock: u64,
    solo: bool,
    /// Counts each pad's notes, so an editor can flash it.
    pub hits: Vec<u32>,
}
impl Runtime {
    pub fn new(rate: u32) -> Self {
        Self {
            rate: rate as f32,
            pool: Pool::new(),
            pads: Vec::new(),
            voices: vec![Voice::default(); MAX_VOICES],
            polyphony: 32,
            sends: Vec::new(),
            routes: Vec::new(),
            random: Random(0x5341_4d50_4c45_5231),
            clock: 0,
            solo: false,
            hits: Vec::new(),
        }
    }
    /// A runtime for `settings` built off the audio thread, with its own
    /// copy of the pool and the pads that go to Pad FX timbres.
    pub fn prepared(settings: &Settings, pool: &Pool, routes: &[u32], rate: u32) -> Box<Self> {
        let mut runtime = Box::new(Self::new(rate));
        runtime.pool = pool.clone();
        runtime.routes = routes.to_vec();
        runtime.configure(settings);
        runtime
    }
    /// Takes over the sounding voices of the runtime this one replaces,
    /// without allocating: by pad identity, as `configure` keeps them. A
    /// voice whose pad is gone, or whose sample this pool no longer holds,
    /// stops and stays with `old`, so no sample is freed on the audio thread.
    pub fn inherit(&mut self, old: &mut Runtime) {
        std::mem::swap(&mut self.voices, &mut old.voices);
        for (voice, spare) in self.voices.iter_mut().zip(&mut old.voices) {
            let pad = self.pads.iter().position(|p| p.id == voice.pad_id);
            let held = voice
                .sample
                .as_ref()
                .is_some_and(|s| self.pool.values().any(|p| Arc::ptr_eq(p, s)));
            match pad {
                Some(index) if voice.active && held => voice.pad = index,
                _ => std::mem::swap(voice, spare),
            }
        }
        for (map, hits) in self.pads.iter_mut().zip(&mut self.hits) {
            if let Some(i) = old.pads.iter().position(|p| p.id == map.id) {
                map.cycle = old.pads[i].cycle;
                *hits = old.hits.get(i).copied().unwrap_or(0);
            }
        }
        std::mem::swap(&mut self.random, &mut old.random);
        self.clock = old.clock;
    }
    /// Fills `view` with what an editor shows, without allocating.
    pub fn snapshot(&self, view: &mut Snapshot) {
        view.voice_count = 0;
        for voice in self.voices.iter().filter(|v| v.active) {
            view.voices[view.voice_count] = (
                voice.pad_id,
                voice.zone as u32,
                voice.frame_of(voice.position),
            );
            view.voice_count += 1;
        }
        view.pad_count = self.pads.len().min(view.hits.len());
        for ((slot, map), hits) in view.hits.iter_mut().zip(&self.pads).zip(&self.hits) {
            *slot = (map.id, *hits);
        }
    }
    /// Follows edited settings or pool: voices keep sounding when their pad
    /// still exists, by identity, and stop when it is gone.
    pub fn configure(&mut self, settings: &Settings) {
        let previous: Vec<u32> = self.pads.iter().map(|p| p.id).collect();
        let hits = std::mem::take(&mut self.hits);
        self.pads = settings
            .pads
            .iter()
            .map(|pad| PadMap {
                id: pad.id,
                samples: pad
                    .playing_zones()
                    .iter()
                    .map(|z| self.pool.get(&z.sample).cloned())
                    .collect(),
                // A bounced pad plays into the mix, as `set_routes` has it.
                routed: self.routes.contains(&pad.id) && pad.bounce.is_none(),
                cycle: 0,
            })
            .collect();
        self.hits = self
            .pads
            .iter()
            .map(|p| {
                previous
                    .iter()
                    .position(|id| *id == p.id)
                    .and_then(|i| hits.get(i).copied())
                    .unwrap_or(0)
            })
            .collect();
        self.sends = vec![[0.0; 2]; self.pads.len()];
        self.polyphony = settings.polyphony.clamp(1, MAX_VOICES as u8) as usize;
        self.solo = settings.pads.iter().any(|p| p.solo);
        for voice in &mut self.voices {
            if !voice.active {
                continue;
            }
            match self.pads.iter().position(|p| p.id == voice.pad_id) {
                Some(index) => voice.pad = index,
                None => *voice = Voice::default(),
            }
        }
    }
    /// The pads whose voices go to a Pad FX timbre.
    pub fn set_routes(&mut self, routes: &[u32], settings: &Settings) {
        if self.routes != routes {
            self.routes = routes.to_vec();
            for (map, pad) in self.pads.iter_mut().zip(&settings.pads) {
                map.routed = self.routes.contains(&pad.id) && pad.bounce.is_none();
            }
        }
    }
    pub fn pad_index(&self, id: u32) -> Option<usize> {
        self.pads.iter().position(|p| p.id == id)
    }
    /// This frame's voices of a routed pad.
    pub fn send(&self, pad: u32) -> Stereo {
        self.pad_index(pad)
            .and_then(|i| self.sends.get(i))
            .copied()
            .unwrap_or_default()
    }
    pub fn idle(&self) -> bool {
        self.voices.iter().all(|v| !v.active)
    }
    pub fn panic(&mut self) {
        for voice in &mut self.voices {
            *voice = Voice::default();
        }
    }
    pub fn release_all(&mut self) {
        for voice in self.voices.iter_mut().filter(|v| v.active) {
            voice.held = false;
            voice.sustained = false;
            voice.amp.release();
        }
    }
    /// Pads mapped to the note play it.
    pub fn note_on(
        &mut self,
        settings: &Settings,
        channel: u8,
        note: u8,
        velocity: u8,
        owner: u32,
    ) {
        for index in 0..settings.pads.len().min(self.pads.len()) {
            let pad = &settings.pads[index];
            if (pad.keys[0]..=pad.keys[1]).contains(&note) {
                self.trigger(settings, index, channel, note, velocity, owner);
            }
        }
    }
    /// Plays one pad whatever its keys, for sequencer tracks and audition.
    pub fn trigger(
        &mut self,
        settings: &Settings,
        index: usize,
        channel: u8,
        note: u8,
        velocity: u8,
        owner: u32,
    ) {
        let (Some(pad), Some(map)) = (settings.pads.get(index), self.pads.get(index)) else {
            return;
        };
        if pad.muted || (self.solo && !pad.solo) {
            return;
        }
        let zones = pad.playing_zones();
        let count = zones
            .iter()
            .zip(&map.samples)
            .filter(|(z, s)| s.is_some() && z.matches(note, velocity))
            .count();
        if count == 0 {
            return;
        }
        if let Some(hits) = self.hits.get_mut(index) {
            *hits = hits.wrapping_add(1);
        }
        // Mono pads and mute groups cut what they replace.
        for voice in self.voices.iter_mut().filter(|v| v.active && !v.killing) {
            let other = &settings.pads[voice.pad.min(settings.pads.len() - 1)];
            if (pad.mono && voice.pad == index)
                || (pad.choke != 0 && voice.pad != index && other.choke == pad.choke)
            {
                voice.start_kill();
            }
        }
        let pick = match pad.layers {
            LayerMode::Velocity => None,
            LayerMode::Cycle => {
                let map = &mut self.pads[index];
                let turn = map.cycle % count;
                map.cycle = map.cycle.wrapping_add(1);
                Some(turn)
            }
            LayerMode::Random => Some(self.random.next_u32() as usize % count),
        };
        let mut matching = 0;
        for (zone_index, zone) in zones.iter().enumerate() {
            let Some(sample) = self.pads[index].samples[zone_index].clone() else {
                continue;
            };
            if !zone.matches(note, velocity) {
                continue;
            }
            matching += 1;
            if pick.is_some_and(|p| p + 1 != matching) {
                continue;
            }
            let slot = self.allocate();
            self.start(
                slot, pad, index, zone_index, sample, channel, note, velocity, owner,
            );
        }
    }
    /// A free voice, or the quietest released one, or the oldest.
    fn allocate(&mut self) -> usize {
        let live = self
            .voices
            .iter()
            .filter(|v| v.active && !v.killing)
            .count();
        if live >= self.polyphony {
            let steal = (0..self.voices.len())
                .filter(|&i| self.voices[i].active && !self.voices[i].killing)
                .min_by(|&a, &b| {
                    let (a, b) = (&self.voices[a], &self.voices[b]);
                    (a.held, a.amp.level, a.age)
                        .partial_cmp(&(b.held, b.amp.level, b.age))
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap_or(0);
            self.voices[steal].start_kill();
        }
        self.voices
            .iter()
            .position(|v| !v.active)
            .unwrap_or_else(|| {
                (0..self.voices.len())
                    .min_by_key(|&i| self.voices[i].age)
                    .unwrap_or(0)
            })
    }
    #[allow(clippy::too_many_arguments)]
    fn start(
        &mut self,
        slot: usize,
        pad: &Pad,
        index: usize,
        zone_index: usize,
        sample: Arc<SampleBuffer>,
        channel: u8,
        note: u8,
        velocity: u8,
        owner: u32,
    ) {
        let rate = self.rate;
        let bounced = pad.bounce.is_some();
        let zone: &Zone = &pad.playing_zones()[zone_index];
        let length = sample.len() as f64;
        let start = (zone.start as f64).min(length - 1.0);
        let end = if zone.end == 0 {
            length
        } else {
            (zone.end as f64).clamp(start + 1.0, length)
        };
        let source = sample.sample_rate as f32;
        let ms = |value: f32| (value * 0.001 * source) as f64;
        let loop_frame = (zone.loop_start as f64).clamp(start, end - 1.0);
        let mirror = zone.reverse.then_some(start + end - 1.0);
        // Reversed, the loop keeps its frames, which then come first and
        // have nothing before them to crossfade from.
        let (loop_start, loop_end, crossfade) = match mirror {
            Some(_) => (start, start + (end - loop_frame), 0.0),
            None => (
                loop_frame,
                end,
                ms(zone.crossfade)
                    .min(loop_frame - start)
                    .min(end - loop_frame),
            ),
        };
        let velocity_unit = velocity.max(1) as f32 / 127.0;
        let keytrack = pad.keytrack || bounced;
        let offset = if keytrack {
            note as f32 - zone.root as f32
        } else {
            0.0
        };
        let semitones = offset + zone.tune + zone.fine * 0.01;
        let base = (source / rate) as f64 * fast::exp2(semitones / 12.0) as f64;
        let late = if bounced {
            0.0
        } else {
            pad.velocity.start * (1.0 - velocity_unit)
        };
        let position = start + (end - start) * late as f64;
        let level = 1.0 - pad.velocity.level * (1.0 - velocity_unit * velocity_unit);
        let mut voice = Voice {
            sample: Some(sample),
            active: true,
            pad: index,
            pad_id: pad.id,
            zone: zone_index,
            owner,
            channel,
            note,
            velocity: velocity_unit,
            held: true,
            sustained: false,
            age: self.clock,
            position: position.min(end - 1.0),
            forward: true,
            start,
            end,
            loop_start,
            loop_end,
            loop_mode: zone.loop_mode,
            crossfade,
            mirror,
            region: [start, end],
            fades: [ms(zone.fade_in), ms(zone.fade_out)],
            curves: zone.fade_curve,
            base,
            step: base,
            level: 0,
            zone_gain: zone.gain + fast::db(level.max(1e-6)),
            zone_pan: zone.pan,
            gains: [1.0; 2],
            kill: 1.0,
            killing: false,
            countdown: 0,
            ..Voice::default()
        };
        if bounced {
            let envelope = crate::sampler::Envelope {
                attack: 0.0,
                hold: 0.0,
                decay: 1.0,
                sustain: 1.0,
                release: pad.envelope.release,
            };
            voice.amp.start(&envelope, 1.0, rate);
        } else {
            let scale = 1.0 + 3.0 * pad.velocity.attack * (1.0 - velocity_unit);
            voice.amp.start(&pad.envelope, scale, rate);
            voice.sweep.start(&pad.filter, rate);
            voice.pitch = pad.pitch.amount;
            voice.pitch_decay = falloff(pad.pitch.decay, rate);
        }
        self.clock += 1;
        self.voices[slot] = voice;
    }
    /// Ends the notes this message names; sustained ones wait for the pedal.
    pub fn note_off(
        &mut self,
        settings: &Settings,
        channel: u8,
        note: u8,
        owner: u32,
        sustain: bool,
    ) {
        for voice in &mut self.voices {
            if voice.active
                && voice.held
                && voice.note == note
                && voice.channel == channel
                && voice.owner == owner
            {
                voice.held = false;
                voice.sustained = sustain;
                if !sustain {
                    release(voice, settings);
                }
            }
        }
    }
    /// The sustain pedal came up.
    pub fn release_sustained(&mut self, settings: &Settings, channel: u8) {
        for voice in &mut self.voices {
            if voice.active && voice.sustained && voice.channel == channel {
                voice.sustained = false;
                release(voice, settings);
            }
        }
    }
    /// Ends every note a sequencer track holds.
    pub fn release_sequenced(&mut self, settings: &Settings) {
        for voice in &mut self.voices {
            if voice.active && voice.held && voice.owner != 0 {
                voice.held = false;
                voice.sustained = false;
                release(voice, settings);
            }
        }
    }
    /// Ends the notes of one sequencer track or one pad audition.
    pub fn release_owner(&mut self, settings: &Settings, owner: u32) {
        for voice in &mut self.voices {
            if voice.active && voice.held && voice.owner == owner {
                voice.held = false;
                voice.sustained = false;
                release(voice, settings);
            }
        }
    }
    /// One frame of the kit's mix; routed pads go to their sends.
    pub fn render(&mut self, settings: &Settings, bend: &[f32; 16]) -> Stereo {
        let mut mix = [0.0; 2];
        for (send, pad) in self.sends.iter_mut().zip(&self.pads) {
            if pad.routed {
                *send = [0.0; 2];
            }
        }
        let kill_step = 1.0 / (KILL_SECONDS * self.rate);
        let solo = self.solo;
        for voice in &mut self.voices {
            if !voice.active {
                continue;
            }
            let Some(pad) = settings.pads.get(voice.pad) else {
                *voice = Voice::default();
                continue;
            };
            if (pad.muted || (solo && !pad.solo)) && !voice.killing {
                voice.start_kill();
            }
            if voice.countdown == 0 {
                voice.countdown = CONTROL;
                if voice.pitch != 0.0 {
                    voice.pitch = fast::flush(voice.pitch * voice.pitch_decay.powi(CONTROL as i32));
                }
                voice.control(pad, bend[voice.channel as usize & 15], self.rate);
                voice.sweep.advance(CONTROL);
            }
            voice.countdown -= 1;
            match voice.render(kill_step) {
                Some(frame) => {
                    let target = match self.pads.get(voice.pad) {
                        Some(map) if map.routed => &mut self.sends[voice.pad],
                        _ => &mut mix,
                    };
                    target[0] += frame[0];
                    target[1] += frame[1];
                }
                None => {
                    voice.active = false;
                }
            }
        }
        mix
    }
    /// Where each sounding voice plays, for the editor: (pad, zone, frame).
    pub fn positions(&self) -> Vec<(u32, usize, f64)> {
        self.voices
            .iter()
            .filter(|v| v.active)
            .map(|v| (v.pad_id, v.zone, v.frame_of(v.position)))
            .collect()
    }
    pub fn pad_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.pads.iter().map(|p| p.id)
    }
}

fn release(voice: &mut Voice, settings: &Settings) {
    let Some(pad) = settings.pads.get(voice.pad) else {
        voice.start_kill();
        return;
    };
    // One-shot pads ignore note-off unless they loop forever.
    let looping = matches!(voice.loop_mode, LoopMode::Forward | LoopMode::Alternate);
    if pad.trigger == Trigger::NoteOn || looping {
        voice.amp.release();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sampler::{Settings, Zone};

    fn ramp(frames: usize, rate: u32) -> Arc<SampleBuffer> {
        Arc::new(
            SampleBuffer::new(
                "ramp".into(),
                rate,
                (0..frames).map(|i| [i as f32 / frames as f32; 2]).collect(),
            )
            .unwrap(),
        )
    }
    fn kit(zone: Zone) -> (Settings, Runtime) {
        let mut settings = Settings::new();
        settings.pads[0].zones.push(zone);
        settings.sanitize();
        let mut runtime = Runtime::new(48_000);
        runtime.pool.insert("ramp".into(), ramp(4_800, 48_000));
        runtime.configure(&settings);
        (settings, runtime)
    }
    fn run(settings: &Settings, runtime: &mut Runtime, frames: usize) -> Vec<Stereo> {
        (0..frames)
            .map(|_| runtime.render(settings, &[0.0; 16]))
            .collect()
    }

    #[test]
    fn a_one_shot_plays_its_region_once_at_the_root() {
        let (settings, mut runtime) = kit(Zone {
            start: 1_000,
            end: 2_000,
            ..Zone::new("ramp")
        });
        runtime.note_on(&settings, 0, 36, 127, 0);
        runtime.note_off(&settings, 0, 36, 0, false);
        let out = run(&settings, &mut runtime, 1_200);
        assert!(
            (out[0][0] - 1_000.0 / 4_800.0).abs() < 1e-3,
            "{}",
            out[0][0]
        );
        assert!((out[500][0] - 1_500.0 / 4_800.0).abs() < 1e-3);
        assert!((out[999][0] - 1_999.0 / 4_800.0).abs() < 1e-3);
        assert!(out[1_000..].iter().all(|f| f[0] == 0.0));
        assert!(runtime.idle());
    }
    #[test]
    fn keytracked_pads_play_faster_above_the_root() {
        let (mut settings, _) = kit(Zone::new("ramp"));
        settings.pads[0].keytrack = true;
        settings.pads[0].keys = [0, 127];
        settings.pads[0].zones[0].root = 60;
        let mut runtime = Runtime::new(48_000);
        runtime.pool.insert("ramp".into(), ramp(4_800, 48_000));
        runtime.configure(&settings);
        runtime.note_on(&settings, 0, 72, 127, 0);
        let out = run(&settings, &mut runtime, 2_500);
        // An octave up reads two frames per output frame.
        assert!(
            (out[100][0] - 200.0 / 4_800.0).abs() < 1e-3,
            "{}",
            out[100][0]
        );
        assert!(runtime.idle());
    }
    #[test]
    fn forward_loops_wrap_to_the_loop_start_until_released() {
        let (mut settings, _) = kit(Zone {
            loop_mode: LoopMode::Forward,
            loop_start: 2_400,
            ..Zone::new("ramp")
        });
        settings.pads[0].trigger = Trigger::NoteOn;
        settings.pads[0].envelope.release = 5.0;
        let mut runtime = Runtime::new(48_000);
        runtime.pool.insert("ramp".into(), ramp(4_800, 48_000));
        runtime.configure(&settings);
        runtime.note_on(&settings, 0, 36, 127, 0);
        let out = run(&settings, &mut runtime, 4_800 + 100);
        assert!((out[4_800 + 50][0] - 2_450.0 / 4_800.0).abs() < 1e-3);
        runtime.note_off(&settings, 0, 36, 0, false);
        run(&settings, &mut runtime, 4_800);
        assert!(runtime.idle());
    }
    #[test]
    fn crossfades_blend_the_audio_before_the_loop_start_into_the_end() {
        let (settings, mut runtime) = kit(Zone {
            loop_mode: LoopMode::Forward,
            loop_start: 2_400,
            // 480 frames at 48 kHz.
            crossfade: 10.0,
            ..Zone::new("ramp")
        });
        runtime.note_on(&settings, 0, 36, 127, 0);
        let out = run(&settings, &mut runtime, 4_800);
        // Halfway through the crossfade both sides weigh sqrt(0.5).
        let at = 4_800 - 240;
        let expected = (at as f32 + (at - 2_400) as f32) / 4_800.0 * 0.5_f32.sqrt();
        assert!(
            (out[at][0] - expected).abs() < 2e-3,
            "{} {expected}",
            out[at][0]
        );
    }
    #[test]
    fn reversed_zones_play_from_the_end() {
        let (settings, mut runtime) = kit(Zone {
            start: 1_000,
            end: 2_000,
            reverse: true,
            ..Zone::new("ramp")
        });
        runtime.note_on(&settings, 0, 36, 127, 0);
        let out = run(&settings, &mut runtime, 1_100);
        assert!(
            (out[0][0] - 1_999.0 / 4_800.0).abs() < 1e-3,
            "{}",
            out[0][0]
        );
        assert!((out[999][0] - 1_000.0 / 4_800.0).abs() < 1e-3);
        assert_eq!(out[1_050], [0.0; 2]);
    }
    #[test]
    fn mute_groups_cut_the_other_pads() {
        let (mut settings, _) = kit(Zone::new("ramp"));
        settings.pads[1].zones.push(Zone::new("ramp"));
        settings.pads[0].choke = 1;
        settings.pads[1].choke = 1;
        let mut runtime = Runtime::new(48_000);
        runtime.pool.insert("ramp".into(), ramp(48_000, 48_000));
        runtime.configure(&settings);
        runtime.note_on(&settings, 0, 36, 127, 0);
        run(&settings, &mut runtime, 100);
        runtime.note_on(&settings, 0, 37, 127, 0);
        run(&settings, &mut runtime, 1_000);
        let pads: Vec<u32> = runtime.positions().iter().map(|p| p.0).collect();
        assert_eq!(pads, vec![2]);
    }
    #[test]
    fn fades_follow_their_curves() {
        assert_eq!(curve(0.5, 0.0), 0.5);
        assert!((curve(0.5, 1.0) - 0.0625).abs() < 1e-4);
        assert!((curve(0.5, -1.0) - 0.5_f32.powf(0.25)).abs() < 1e-4);
        let (settings, mut runtime) = kit(Zone {
            fade_in: 10.0,
            ..Zone::new("ramp")
        });
        runtime.note_on(&settings, 0, 36, 127, 0);
        let out = run(&settings, &mut runtime, 1_000);
        assert!(out[240][0] < 241.0 / 4_800.0 * 0.51);
        assert!((out[600][0] - 600.0 / 4_800.0).abs() < 1e-3);
    }
    #[test]
    fn velocity_layers_pick_their_zone() {
        let (mut settings, _) = kit(Zone {
            velocities: [1, 63],
            gain: -60.0,
            ..Zone::new("ramp")
        });
        settings.pads[0].zones.push(Zone {
            velocities: [64, 127],
            ..Zone::new("ramp")
        });
        let mut runtime = Runtime::new(48_000);
        runtime.pool.insert("ramp".into(), ramp(4_800, 48_000));
        runtime.configure(&settings);
        runtime.note_on(&settings, 0, 36, 100, 0);
        assert_eq!(runtime.positions().len(), 1);
        assert_eq!(runtime.positions()[0].1, 1);
    }
    #[test]
    fn routed_pads_fill_their_send() {
        let (settings, mut runtime) = kit(Zone::new("ramp"));
        runtime.set_routes(&[1], &settings);
        runtime.note_on(&settings, 0, 36, 127, 0);
        run(&settings, &mut runtime, 10);
        let mix = runtime.render(&settings, &[0.0; 16]);
        assert_eq!(mix, [0.0; 2]);
        assert!(runtime.send(1)[0] > 0.0);
    }
}
