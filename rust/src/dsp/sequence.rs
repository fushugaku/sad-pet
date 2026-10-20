//! The actual Neuropolis sequencer, with SAD PET destinations and sample clock.
//! Prepared off-thread; ticks, edits, feedback and state adoption never allocate.
use super::Modulated;
use crate::{
    midi::Message,
    parameters,
    sequence::{Edit, Target, Track, TrackView, View},
};
use metropolix_core::sequencer::{PPQN, Sequencer, field};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Binding {
    MainPitch,
    Oscillator(usize),
    Parameter(usize),
    Notes,
    Unbound,
}
#[derive(Clone, Copy)]
pub(crate) enum Action {
    Midi(Message),
    Legato { channel: u8, note: u8, velocity: u8 },
}
#[derive(Clone, Copy, Default, Debug)]
struct Glide {
    from: f32,
    to: f32,
    start: f64,
    duration: f64,
}
impl Glide {
    fn value(self, now: f64) -> f32 {
        let t = if self.duration <= 0.0 {
            1.0
        } else {
            ((now - self.start) / self.duration).clamp(0.0, 1.0) as f32
        };
        self.from + (self.to - self.from) * t
    }
}
#[derive(Clone, Debug)]
struct Lane {
    id: u32,
    binding: Binding,
    route: Option<usize>,
    amount: f32,
    absolute: bool,
    gate_output: bool,
    root: u8,
    velocity: [u8; 16],
    held: Option<(u8, u8)>,
    pitch: Glide,
}
pub(crate) struct Runtime {
    core: Sequencer,
    lanes: Vec<Lane>,
    indices: Vec<(u32, usize)>,
    routes: Vec<usize>,
    oscillator_routes: Vec<usize>,
    phase: f64,
    started: bool,
    waiting_grid: bool,
    enabled: bool,
    pub(crate) view_offset: usize,
}
impl std::fmt::Debug for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Neuropolis")
            .field("tracks", &self.lanes.len())
            .field("ticks", &self.core.ticks)
            .finish()
    }
}
impl Runtime {
    pub fn new(tracks: &[Track]) -> Self {
        let mut core = Sequencer::default();
        core.resize_tracks(tracks.len().max(1));
        // Modulation comes from SAD PET's dynamic tracks; do not run hidden lanes.
        for lane in &mut core.mods {
            lane.config[9] = 1;
        }
        let lanes: Vec<Lane> = tracks
            .iter()
            .enumerate()
            .map(|(i, t)| {
                for (field, value) in t.config.iter().enumerate() {
                    core.set_track(i, field as u32, *value);
                }
                for (stage, s) in t.steps.iter().enumerate() {
                    for field in 0..15 {
                        core.set_step(i, stage, field as u32, s.get(field) as i32);
                    }
                }
                let binding = match &t.target {
                    Target::MainPitch => Binding::MainPitch,
                    Target::Oscillator { index } => Binding::Oscillator(*index as usize),
                    Target::Notes => Binding::Notes,
                    Target::Parameter { parameter } => parameters::specs()
                        .iter()
                        .position(|s| s.id == *parameter && s.can_sequence())
                        .map_or(Binding::Unbound, Binding::Parameter),
                };
                let route = match binding {
                    Binding::MainPitch => parameters::specs()
                        .iter()
                        .position(|s| s.id == "sample_tune_semis"),
                    Binding::Oscillator(index) => {
                        let id = format!("resynth_osc{}_transpose", index + 1);
                        parameters::specs().iter().position(|s| s.id == id)
                    }
                    Binding::Parameter(index) => Some(index),
                    _ => None,
                };
                Lane {
                    id: t.id,
                    binding,
                    route,
                    amount: t.amount,
                    absolute: t.absolute,
                    gate_output: t.gate_output,
                    root: t.root_note,
                    velocity: t.steps.map(|s| s.velocity),
                    held: None,
                    pitch: Glide::default(),
                }
            })
            .collect();
        let mut indices: Vec<_> = tracks.iter().enumerate().map(|(i, t)| (t.id, i)).collect();
        indices.sort_unstable_by_key(|p| p.0);
        let mut routes: Vec<_> = (0..lanes.len())
            .filter(|&i| lanes[i].route.is_some())
            .collect();
        routes.sort_unstable_by_key(|&i| lanes[i].route);
        let oscillator_routes = (0..lanes.len())
            .filter(|&i| matches!(lanes[i].binding, Binding::Oscillator(_)))
            .collect();
        Self {
            core,
            lanes,
            indices,
            routes,
            oscillator_routes,
            phase: 0.0,
            started: false,
            waiting_grid: false,
            enabled: false,
            view_offset: 0,
        }
    }
    pub fn index(&self, id: u32) -> Option<usize> {
        self.indices
            .binary_search_by_key(&id, |p| p.0)
            .ok()
            .map(|i| self.indices[i].1)
    }
    pub fn inherit(&mut self, old: &mut Self) {
        // Move the vectors aside so inherit_playback transfers the core's private
        // random/transport state without tying playheads to changing row indices.
        let mut next_tracks = std::mem::take(&mut self.core.tracks);
        let mut old_tracks = std::mem::take(&mut old.core.tracks);
        self.core.inherit_playback(&mut old.core);
        for (i, lane) in self.lanes.iter_mut().enumerate() {
            if let Ok(pos) = old.indices.binary_search_by_key(&lane.id, |p| p.0) {
                let j = old.indices[pos].1;
                let old_lane = &old.lanes[j];
                if lane.binding == old_lane.binding {
                    let config = next_tracks[i].config;
                    let steps = next_tracks[i].steps;
                    std::mem::swap(&mut next_tracks[i], &mut old_tracks[j]);
                    next_tracks[i].config = config;
                    next_tracks[i].steps = steps;
                    lane.held = old_lane.held;
                    lane.pitch = old_lane.pitch;
                }
            }
        }
        self.core.tracks = next_tracks;
        old.core.tracks = old_tracks;
        self.phase = old.phase;
        self.started = old.started;
        self.waiting_grid = old.waiting_grid;
        self.enabled = old.enabled;
        self.view_offset = old.view_offset.min(self.lanes.len().saturating_sub(1));
    }
    pub fn retained_note(&self, id: u32) -> bool {
        self.index(id)
            .is_some_and(|i| self.lanes[i].binding == Binding::Notes)
    }
    pub fn edit(&mut self, index: usize, track: &Track, edit: Edit) {
        if edit.stage.is_none() && edit.field >= 47 {
            self.core.reset_track(index, edit.field == 48);
            if edit.field == 47 {
                self.lanes[index].held = None;
            }
            return;
        }
        if let Some(stage) = edit.stage {
            if edit.field < 15 {
                self.core.set_step(
                    index,
                    stage as usize,
                    edit.field as u32,
                    track.steps[stage as usize].get(edit.field) as i32,
                );
            } else {
                self.lanes[index].velocity[stage as usize] = track.steps[stage as usize].velocity;
            }
        } else if (edit.field as usize) < field::COUNT {
            self.core
                .set_track(index, edit.field as u32, track.config[edit.field as usize]);
        } else {
            let l = &mut self.lanes[index];
            l.amount = track.amount;
            l.absolute = track.absolute;
            l.gate_output = track.gate_output;
            l.root = track.root_note;
        }
    }
    pub fn restart(&mut self) {
        self.core.restart();
        self.phase = 0.0;
        self.started = false;
        self.waiting_grid = false;
        for l in &mut self.lanes {
            l.held = None;
            l.pitch = Glide::default();
        }
    }
    pub fn start(&mut self, enabled: bool) {
        if self.enabled == enabled && !self.started && !self.waiting_grid {
            return;
        }
        self.enabled = enabled;
        self.restart();
    }
    pub fn seek(&mut self, beat: f64) {
        if self.started {
            self.restart();
        }
        self.phase = (beat * PPQN as f64).rem_euclid(1.0);
        self.waiting_grid = self.phase > 1e-7 && self.phase < 1.0 - 1e-7;
        if !self.waiting_grid {
            self.phase = 0.0;
        }
    }
    pub fn tick(&mut self, sr: u32, bpm: f32, enabled: bool) -> bool {
        if self.enabled != enabled {
            self.restart();
            self.enabled = enabled;
        }
        if !enabled || self.lanes.is_empty() {
            return false;
        }
        let step = bpm as f64 * PPQN as f64 / (60.0 * sr as f64);
        if self.waiting_grid {
            if self.phase < 1.0 {
                self.phase += step;
                return false;
            }
            self.phase -= 1.0;
            self.waiting_grid = false;
        }
        let tick = !self.started || self.phase >= 1.0;
        if tick {
            if self.started {
                self.phase -= 1.0;
            }
            self.started = true;
            self.core.set_global(0, (bpm * 1000.0).round() as i32);
            self.core.tick();
            let now = self.core.ticks.saturating_sub(1) as f64;
            for (i, l) in self.lanes.iter_mut().enumerate() {
                let target = self.core.cents(i) as f32 * 0.01;
                if target != l.pitch.to || self.core.track_events(i) & 1 != 0 {
                    // Neuropolis web/audio.js glideTime: Analog is distance
                    // dependent; Tempo follows a sixteenth; Acid keeps its tie.
                    let amount = self.core.slide_amount(i) as f64 / 100.0;
                    let duration = if self.core.slide(i) && amount > 0.0 {
                        let seconds = match self.core.slide_type(i) {
                            1 => 60.0 / bpm as f64 / 4.0 * amount * 2.0,
                            2 => 0.003 + amount * amount * 0.5,
                            _ => {
                                0.003
                                    + amount
                                        * amount
                                        * 0.6
                                        * ((target - l.pitch.to).abs() as f64 / 12.0).max(0.08)
                            }
                        };
                        seconds * bpm as f64 * PPQN as f64 / 60.0
                    } else {
                        0.0
                    };
                    l.pitch = Glide {
                        from: if now == 0.0 {
                            target
                        } else {
                            l.pitch.value(now)
                        },
                        to: target,
                        start: now,
                        duration,
                    };
                }
            }
        }
        self.phase += step;
        tick
    }
    pub fn len(&self) -> usize {
        self.lanes.len()
    }
    pub fn actions(&mut self, i: usize) -> (u32, [Option<Action>; 2]) {
        let lane = &mut self.lanes[i];
        let mut actions = [None; 2];
        if lane.binding != Binding::Notes {
            return (lane.id, actions);
        }
        let on = self.enabled && self.core.tracks[i].config[field::MUTE] == 0 && self.core.gate(i);
        let flags = self.core.track_events(i);
        let channel = (self.core.tracks[i].config[field::MIDI_CHANNEL] - 1).max(0) as u8;
        let raw = self.core.note(i).unwrap_or(60) as f32;
        let note = (lane.root as f32 + (raw - lane.root as f32) * lane.amount)
            .round()
            .clamp(0.0, 127.0) as u8;
        let stage = self.core.stage(i).unwrap_or(0) as usize;
        let velocity =
            (self.core.velocity(i) * lane.velocity[stage] as i32 / 127).clamp(1, 127) as u8;
        if on && flags & 1 == 0 && lane.held.is_some_and(|key| key != (channel, note)) {
            lane.held = Some((channel, note));
            return (
                lane.id,
                [
                    Some(Action::Legato {
                        channel,
                        note,
                        velocity,
                    }),
                    None,
                ],
            );
        }
        if lane
            .held
            .is_some_and(|key| !on || key != (channel, note) || flags & 1 != 0)
        {
            let (channel, note) = lane.held.take().unwrap();
            actions[0] = Some(Action::Midi(Message::NoteOff {
                channel,
                note,
                velocity: 0,
            }));
        }
        if on && lane.held.is_none() {
            lane.held = Some((channel, note));
            actions[1] = Some(Action::Midi(Message::NoteOn {
                channel,
                note,
                velocity,
            }));
        }
        (lane.id, actions)
    }
    pub fn apply(&self, m: &mut Modulated) {
        if !self.enabled || !self.started {
            return;
        }
        let now = self.core.ticks.saturating_sub(1) as f64 + self.phase.min(1.0);
        // Group compiled destinations once, off-thread. Sum contributions before
        // clamping so opposite lanes cancel, independent of their list order.
        let mut cursor = 0;
        while cursor < self.routes.len() {
            let id = self.lanes[self.routes[cursor]].route.unwrap();
            let spec = &parameters::specs()[id];
            let base = spec.frame_get(m);
            let mut delta = 0.0;
            while cursor < self.routes.len() && self.lanes[self.routes[cursor]].route == Some(id) {
                let i = self.routes[cursor];
                cursor += 1;
                let l = &self.lanes[i];
                if self.core.tracks[i].config[field::MUTE] != 0 {
                    continue;
                }
                delta += if matches!(l.binding, Binding::MainPitch | Binding::Oscillator(_)) {
                    (l.pitch.value(now) - l.root as f32) * l.amount
                } else {
                    let cv = self.core.cv(i) as f32 / 5000.0;
                    if l.absolute {
                        let unit = if self.core.tracks[i].config[field::CV_TYPE] == 0 {
                            (cv + 1.0) * 0.5
                        } else {
                            cv
                        };
                        (spec.min + unit * (spec.max - spec.min) - base.unwrap_or(0.0)) * l.amount
                    } else {
                        cv * l.amount * (spec.max - spec.min)
                    }
                };
            }
            if let Some(base) = base {
                spec.frame_set(m, base + delta);
            }
        }
        for &i in &self.oscillator_routes {
            let l = &self.lanes[i];
            if l.gate_output
                && self.core.tracks[i].config[field::MUTE] == 0
                && !self.core.gate(i)
                && let Binding::Oscillator(index) = l.binding
            {
                m.resynth.oscillators[index].level = 0.0;
            }
        }
    }
    pub fn view(&self) -> View {
        let mut v = View {
            offset: self.view_offset,
            total: self.lanes.len(),
            ..View::default()
        };
        for (i, l) in self
            .lanes
            .iter()
            .enumerate()
            .skip(v.offset)
            .take(crate::sequence::VIEW_TRACKS)
        {
            v.tracks[v.count] = TrackView {
                id: l.id,
                step: self.core.stage(i).unwrap_or(0),
                gate: self.enabled && self.core.gate(i),
                pulse: self.core.track_events(i) & 1 != 0,
                pitch: self.core.cents(i) as f32 * 0.01,
                cv: self.core.cv(i) as f32 / 5000.0,
                value: if !self.started {
                    0.0
                } else if matches!(l.binding, Binding::Parameter(_)) {
                    self.core.cv(i) as f32 / 5000.0
                } else {
                    (l.pitch.to - l.root as f32) * l.amount
                },
            };
            v.count += 1;
        }
        v
    }
}
