//! DXM-088 / LXR sound engine. Editor data and prepared audio state are separate.
use crate::{Stereo, dsp::sample::LiveInput};
use sad_pet_dxm088::{Engine, params, sequencer as seq};
use serde::{Deserialize, Serialize, ser::SerializeSeq};

pub const TRACKS: usize = params::TRACKS;
pub const STEPS: usize = seq::STEPS;
pub const NAMES: [&str; TRACKS] = [
    "Drum 1",
    "Drum 2",
    "Drum 3",
    "Snare",
    "Clap / Cym",
    "Closed HH",
    "Open HH",
];
#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct View {
    pub positions: [i32; TRACKS],
    pub levels: [f32; TRACKS],
    pub playing: bool,
}
pub const PARAM_NAMES: [&str; params::PARAMS] = [
    "Pitch",
    "Waveform",
    "Decay",
    "Attack",
    "Slope",
    "Pitch sweep",
    "Pitch decay",
    "FM ratio",
    "FM amount",
    "FM waveform",
    "Noise mix",
    "Click level",
    "Filter 1 cutoff",
    "Filter 1 resonance",
    "LXR type",
    "Drive",
    "Distortion",
    "Reduction",
    "Pan",
    "Voice level",
    "LFO rate",
    "LFO depth",
    "LFO target",
    "LFO wave",
    "FM ratio 2",
    "FM gain 2",
    "Pulse width",
    "Transient",
    "Repeat",
    "Open decay",
    "Velocity target",
    "Velocity amount",
    "OSC mode",
    "LFO retrigger",
    "Transient rate",
    "Noise color",
    "Filter 1 drive",
    "Pitch slope",
    "FM wave 2",
    "Filter env attack",
    "Filter env decay",
    "Filter env curve",
    "Filter 1 env amount",
    "Assignable attack",
    "Assignable decay",
    "Assignable curve",
    "Filter 2 type",
    "Filter 2 cutoff",
    "Filter 2 resonance",
    "Filter 2 drive",
    "Filter 2 env amount",
    "Filter routing",
    "Filter balance",
    "Filter 1 morph",
    "Filter 2 morph",
    "Filter 1 model",
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Controls(pub [[f32; params::PARAMS]; params::VOICES]);
impl Default for Controls {
    fn default() -> Self {
        Self([[0.; params::PARAMS]; params::VOICES])
    }
}
impl Serialize for Controls {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut rows = s.serialize_seq(Some(params::VOICES))?;
        for row in &self.0 {
            rows.serialize_element(row.as_slice())?;
        }
        rows.end()
    }
}
impl<'de> Deserialize<'de> for Controls {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let rows = Vec::<Vec<f32>>::deserialize(d)?;
        if rows.len() != params::VOICES || rows.iter().any(|r| r.len() != params::PARAMS) {
            return Err(serde::de::Error::custom("Invalid drum controls"));
        }
        let mut result = Self::default();
        for (to, from) in result.0.iter_mut().zip(rows) {
            to.copy_from_slice(&from);
        }
        Ok(result)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Step {
    pub velocity: f32,
    pub probability: f32,
    pub note: i8,
    pub ratchet: u8,
    pub micro: f32,
    pub condition: u8,
    pub every: u8,
    pub phase: u8,
    pub locks: [Lock; 2],
}
impl Default for Step {
    fn default() -> Self {
        Self {
            velocity: 0.,
            probability: 1.,
            note: 0,
            ratchet: 1,
            micro: 0.,
            condition: 0,
            every: 2,
            phase: 0,
            locks: [Lock::default(); 2],
        }
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Lock {
    pub param: u8,
    pub value: f32,
}
impl Default for Lock {
    fn default() -> Self {
        Self {
            param: 255,
            value: 0.,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Track {
    pub steps: Vec<Step>,
    pub length: usize,
    pub division: u8,
    pub delay: f32,
    pub groove: u8,
    pub groove_amount: f32,
    pub swing: f32,
    pub level: f32,
    pub muted: bool,
    pub solo: bool,
    pub midi_note: u8,
}
impl Default for Track {
    fn default() -> Self {
        Self {
            steps: vec![Step::default(); STEPS],
            length: 16,
            division: 4,
            delay: 0.,
            groove: 0,
            groove_amount: 0.,
            swing: 0.,
            level: 0.85,
            muted: false,
            solo: false,
            midi_note: 36,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub params: Controls,
    pub tracks: Vec<Track>,
    pub granular: bool,
    pub buffer_seconds: f32,
    pub swing: f32,
    pub master: f32,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            params: Controls(params::kit(0)),
            tracks: (0..TRACKS)
                .map(|i| Track {
                    midi_note: [36, 41, 43, 38, 39, 42, 46][i],
                    ..Track::default()
                })
                .collect(),
            granular: false,
            buffer_seconds: 8.,
            swing: 0.,
            master: 0.8,
        }
    }
}
fn finite(v: f32, min: f32, max: f32, fallback: f32) -> f32 {
    if v.is_finite() {
        v.clamp(min, max)
    } else {
        fallback
    }
}
impl Settings {
    pub fn buffer_frames(&self, rate: u32) -> usize {
        (self.buffer_seconds.clamp(0.125, 32.0) * rate as f32).round() as usize
    }
    pub fn grain_seconds(&self, size: f32) -> f32 {
        let seconds = crate::tables::grain_duration(size);
        if self.buffer_seconds >= 2.01 {
            seconds
        } else {
            0.01 + (seconds - 0.01) * ((self.buffer_seconds - 0.01).max(0.0) / 2.0)
        }
    }
    pub fn sanitize(&mut self) {
        self.buffer_seconds = finite(self.buffer_seconds, 0.125, 32., 8.);
        for row in &mut self.params.0 {
            for (p, v) in row.iter_mut().enumerate() {
                *v = params::constrain(p, *v).unwrap_or(params::MIN[p]);
            }
        }
        self.tracks.truncate(TRACKS);
        self.tracks.resize_with(TRACKS, Track::default);
        self.swing = finite(self.swing, 0., 0.5, 0.);
        self.master = finite(self.master, 0., 1.25, 0.8);
        for track in &mut self.tracks {
            track.length = track.length.clamp(1, STEPS);
            track.division = track
                .division
                .min((sad_pet_dxm088::performance::DIVISIONS.len() - 1) as u8);
            track.delay = finite(track.delay, -0.49, 0.49, 0.);
            track.groove = track.groove.min(4);
            track.groove_amount = finite(track.groove_amount, 0., 1., 0.);
            track.swing = finite(track.swing, 0., 0.5, 0.);
            track.level = finite(track.level, 0., 1.25, 0.85);
            track.midi_note = track.midi_note.min(127);
            track.steps.truncate(STEPS);
            track.steps.resize(STEPS, Step::default());
            for s in &mut track.steps {
                s.velocity = finite(s.velocity, 0., 1., 0.);
                s.probability = finite(s.probability, 0., 1., 1.);
                s.note = s.note.clamp(-36, 36);
                s.ratchet = s.ratchet.clamp(1, 4);
                s.micro = finite(s.micro, -0.49, 0.49, 0.);
                s.condition = s.condition.min(7);
                s.every = s.every.clamp(1, 16);
                s.phase %= s.every;
                for lock in &mut s.locks {
                    if let Some(v) = params::constrain(lock.param as usize, lock.value) {
                        lock.value = v;
                    } else {
                        *lock = Lock::default();
                    }
                }
            }
        }
    }
}

pub struct Runtime {
    pub engine: Engine,
    /// Prepared off audio. Also retires a previous live input on a type change.
    pub live: Option<LiveInput>,
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
        f.debug_struct("PreparedDrums")
            .field("ready", &self.0.is_some())
            .finish()
    }
}
impl Prepared {
    pub fn prepare(&mut self, settings: Option<&Settings>, rate: u32) {
        self.0 = settings.map(|s| {
            let mut engine = Engine::new(rate as f32);
            engine.params = s.params.0;
            engine.master = s.master;
            engine.swing = s.swing;
            for (i, track) in s.tracks.iter().enumerate() {
                copy_track(&mut engine, i, track);
            }
            engine.configure_params(&s.params.0);
            let mut input = LiveInput::for_drums(rate, !s.granular, s.buffer_seconds);
            input.sample.name = "Drum machine".into();
            Box::new(Runtime {
                engine,
                live: Some(input),
            })
        });
    }
}
fn copy_track(engine: &mut Engine, i: usize, track: &Track) {
    let out = &mut engine.patterns[0].tracks[i];
    out.length = track.length;
    out.swing = track.swing;
    for (to, from) in out.steps.iter_mut().zip(&track.steps) {
        *to = seq::Step {
            velocity: from.velocity,
            probability: from.probability,
            note: from.note,
            ratchet: from.ratchet,
            micro: from.micro,
            condition: from.condition,
            every: from.every,
            phase: from.phase,
            locks: from.locks.map(|l| seq::Lock {
                param: l.param,
                value: l.value,
            }),
        };
    }
    engine.track_settings(
        0,
        i,
        track.division,
        track.delay,
        track.groove,
        track.groove_amount,
    );
    engine.levels[i] = track.level;
    engine.muted[i] = track.muted;
    engine.solo[i] = track.solo;
}
impl Runtime {
    pub fn configure(&mut self, settings: &Settings, controls: Controls, bpm: f32, transpose: f32) {
        if self.engine.params != controls.0 {
            self.engine.configure_params(&controls.0);
        }
        self.engine.tune(transpose);
        self.engine.master = settings.master;
        self.engine.tempo(bpm, settings.swing);
    }
    pub fn frame(&mut self) -> Stereo {
        let output = self.engine.render(1);
        let frame = [output[0], output[1]];
        self.engine.clear_hits();
        frame
    }
    pub fn sync_tracks(&mut self, settings: &Settings) {
        for (i, track) in settings.tracks.iter().enumerate() {
            copy_track(&mut self.engine, i, track);
        }
    }
}

pub fn catalog() -> serde_json::Value {
    serde_json::json!({"tracks":NAMES,"steps":STEPS,"divisions":sad_pet_dxm088::performance::DIVISIONS,"defaults":Controls(params::kit(0)),"controls":PARAM_NAMES.iter().enumerate().map(|(i,n)| serde_json::json!({"name":n,"min":params::MIN[i],"max":params::MAX[i],"discrete":params::DISCRETE.contains(&i)})).collect::<Vec<_>>()})
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Edit {
    pub track: usize,
    pub step: Option<usize>,
    pub field: u8,
    pub value: f32,
}
pub fn parse_edits(data: &serde_json::Value) -> Result<Vec<Edit>, String> {
    let edits: Vec<Edit> = serde_json::from_value(data.clone()).map_err(|e| e.to_string())?;
    if edits.len() > TRACKS * STEPS * 12
        || edits.iter().any(|e| {
            e.track >= TRACKS
                || e.step.is_some_and(|s| s >= STEPS)
                || e.field > if e.step.is_some() { 11 } else { 9 }
                || !e.value.is_finite()
        })
    {
        return Err("Invalid drum edit".into());
    }
    Ok(edits)
}
impl Settings {
    pub fn edit(&mut self, edits: &[Edit]) {
        for e in edits {
            let Some(track) = self.tracks.get_mut(e.track) else {
                continue;
            };
            let n = e.value.round();
            if let Some(i) = e.step {
                let Some(s) = track.steps.get_mut(i) else {
                    continue;
                };
                match e.field {
                    0 => s.velocity = e.value.clamp(0., 1.),
                    1 => s.probability = e.value.clamp(0., 1.),
                    2 => s.note = n.clamp(-36., 36.) as i8,
                    3 => s.ratchet = n.clamp(1., 4.) as u8,
                    4 => s.micro = e.value.clamp(-0.49, 0.49),
                    5 => s.condition = n.clamp(0., 7.) as u8,
                    6 => s.every = n.clamp(1., 16.) as u8,
                    7 => s.phase = n.clamp(0., 15.) as u8,
                    8 | 10 => {
                        s.locks[(e.field as usize - 8) / 2].param =
                            if n >= 56. { 255 } else { n.max(0.) as u8 }
                    }
                    9 | 11 => s.locks[(e.field as usize - 9) / 2].value = e.value,
                    _ => {}
                }
                for lock in &mut s.locks {
                    if let Some(value) = params::constrain(lock.param as usize, lock.value) {
                        lock.value = value;
                    } else {
                        *lock = Lock::default();
                    }
                }
            } else {
                match e.field {
                    0 => track.length = n.clamp(1., STEPS as f32) as usize,
                    1 => track.division = n.clamp(0., 12.) as u8,
                    2 => track.delay = e.value.clamp(-0.49, 0.49),
                    3 => track.groove = n.clamp(0., 4.) as u8,
                    4 => track.groove_amount = e.value.clamp(0., 1.),
                    5 => track.swing = e.value.clamp(0., 0.5),
                    6 => track.level = e.value.clamp(0., 1.25),
                    7 => track.muted = e.value >= 0.5,
                    8 => track.solo = e.value >= 0.5,
                    9 => track.midi_note = n.clamp(0., 127.) as u8,
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn factory_tones_and_original_stereo_dsp_are_preserved() {
        let mut settings = Settings::default();
        let original = settings.params;
        settings.sanitize();
        assert_eq!(
            settings.params, original,
            "Validation changed the upstream factory sound"
        );
        for spec in crate::parameters::specs()
            .iter()
            .filter(|s| s.id.starts_with("drum"))
        {
            assert!(
                (spec.min..=spec.max).contains(&spec.default),
                "Invalid host default: {}",
                spec.id
            );
        }
        let mut prepared = Prepared::default();
        prepared.prepare(Some(&settings), 48_000);
        let runtime = prepared.0.as_mut().unwrap();
        let mut reference = Engine::new(48_000.);
        reference.params = settings.params.0;
        reference.configure_params(&settings.params.0);
        for track in 0..TRACKS {
            runtime.engine.trigger(track, 0.83, 0);
            reference.trigger(track, 0.83, 0);
            for _ in 0..2048 {
                let expected = reference.render(1);
                assert_eq!(runtime.frame(), [expected[0], expected[1]]);
            }
        }
    }
}
