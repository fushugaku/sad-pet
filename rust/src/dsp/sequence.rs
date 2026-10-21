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
    pitch_oscillators: [bool; 4],
    velocity: [u8; 16],
    held: Option<(u8, u8)>,
    pitch: Glide,
}
#[derive(Clone, Copy)]
struct Route {
    parameter: usize,
    lane: Option<usize>,
    oscillator: Option<usize>,
}
pub(crate) struct Runtime {
    core: Sequencer,
    lanes: Vec<Lane>,
    indices: Vec<(u32, usize)>,
    routes: Vec<Route>,
    main_pitch_routes: Vec<usize>,
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
                    pitch_oscillators: t.pitch_oscillators,
                    velocity: t.steps.map(|s| s.velocity),
                    held: None,
                    pitch: Glide::default(),
                }
            })
            .collect();
        let mut indices: Vec<_> = tracks.iter().enumerate().map(|(i, t)| (t.id, i)).collect();
        indices.sort_unstable_by_key(|p| p.0);
        let oscillator_parameters: [usize; 4] = std::array::from_fn(|index| {
            let id = format!("resynth_osc{}_transpose", index + 1);
            parameters::specs().iter().position(|s| s.id == id).unwrap()
        });
        let mut routes = Vec::new();
        for (i, lane) in lanes.iter().enumerate() {
            if let Some(parameter) = lane.route {
                routes.push(Route {
                    parameter,
                    lane: Some(i),
                    oscillator: oscillator_parameters.iter().position(|&id| id == parameter),
                });
            }
        }
        // Include each carrier once even when it has no lane of its own. Main
        // pitch masks can then change in place, without four routes per track.
        for (oscillator, &parameter) in oscillator_parameters.iter().enumerate() {
            routes.push(Route {
                parameter,
                lane: None,
                oscillator: Some(oscillator),
            });
        }
        routes.sort_unstable_by_key(|route| route.parameter);
        let main_pitch_routes = (0..lanes.len())
            .filter(|&i| lanes[i].binding == Binding::MainPitch)
            .collect();
        let oscillator_routes = (0..lanes.len())
            .filter(|&i| matches!(lanes[i].binding, Binding::Oscillator(_)))
            .collect();
        Self {
            core,
            lanes,
            indices,
            routes,
            main_pitch_routes,
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
        if edit.stage.is_none() && matches!(edit.field, 47 | 48) {
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
            l.pitch_oscillators = track.pitch_oscillators;
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
        let mut oscillator_pitch = [0.0; 4];
        for &i in &self.main_pitch_routes {
            let lane = &self.lanes[i];
            if self.core.tracks[i].config[field::MUTE] == 0
                && lane.pitch_oscillators.iter().any(|&on| on)
            {
                let delta = (lane.pitch.value(now) - lane.root as f32) * lane.amount;
                for (value, selected) in oscillator_pitch.iter_mut().zip(lane.pitch_oscillators) {
                    if selected {
                        *value += delta;
                    }
                }
            }
        }
        // Group compiled destinations once, off-thread. Sum contributions before
        // clamping so opposite lanes cancel, independent of their list order.
        let mut cursor = 0;
        while cursor < self.routes.len() {
            let id = self.routes[cursor].parameter;
            let spec = &parameters::specs()[id];
            let base = spec.frame_get(m);
            let mut delta = self.routes[cursor]
                .oscillator
                .map_or(0.0, |index| oscillator_pitch[index]);
            while cursor < self.routes.len() && self.routes[cursor].parameter == id {
                let route = self.routes[cursor];
                cursor += 1;
                let Some(i) = route.lane else {
                    continue;
                };
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{preset::Preset, sequence::Stage};

    fn lane(id: u32, target: Target, pitch: u8) -> Track {
        Track {
            id,
            target,
            steps: [Stage {
                pitch,
                ..Stage::default()
            }; 16],
            ..Track::default()
        }
    }
    fn current(runtime: &Runtime, preset: &Preset) -> Modulated {
        let mut m = Modulated::from_preset(preset);
        runtime.apply(&mut m);
        m
    }

    #[test]
    fn main_pitch_routes_to_selected_carriers_and_oscillator_offsets_are_additive() {
        let mut main = lane(1, Target::MainPitch, 72);
        main.pitch_oscillators = [true, false, true, false];
        let tracks = [
            main,
            lane(2, Target::Oscillator { index: 0 }, 55),
            lane(3, Target::Oscillator { index: 2 }, 67),
        ];
        let mut r = Runtime::new(&tracks);
        r.tick(8000, 120.0, true);
        let mut p = Preset::default();
        for (osc, transpose) in p.resynth.oscillators.iter_mut().zip([3.0, -2.0, 5.0, -8.0]) {
            osc.transpose = transpose;
        }
        let m = current(&r, &p);
        assert_eq!(m.params.sample_tune_semis, 12.0);
        assert_eq!(
            m.resynth.oscillators.map(|o| o.transpose),
            [10.0, -2.0, 24.0, -8.0]
        );
        assert_eq!(p.resynth.oscillators[0].transpose, 3.0);

        let mut main = tracks[0].clone();
        let ticks = r.core.ticks;
        let stage = r.core.stage(0);
        for (field, value) in [(49, 0.0), (50, 1.0)] {
            let edit = Edit {
                track: 1,
                stage: None,
                field,
                value,
            };
            main.edit(edit);
            r.edit(0, &main, edit);
        }
        assert_eq!(r.core.ticks, ticks, "Routing reset the transport");
        assert_eq!(r.core.stage(0), stage);
        assert_eq!(
            current(&r, &p).resynth.oscillators.map(|o| o.transpose),
            [-2.0, 10.0, 24.0, -8.0]
        );
        let edit = Edit {
            track: 1,
            stage: None,
            field: 8,
            value: 1.0,
        };
        main.edit(edit);
        r.edit(0, &main, edit);
        let m = current(&r, &p);
        assert_eq!(m.params.sample_tune_semis, 0.0);
        assert_eq!(
            m.resynth.oscillators.map(|o| o.transpose),
            [-2.0, -2.0, 12.0, -8.0]
        );
    }

    #[test]
    fn carrier_pitch_contributions_cancel_before_clamping_and_keep_independent_clocks() {
        let mut a = lane(1, Target::MainPitch, 84);
        a.pitch_oscillators = [true; 4];
        let mut b = a.clone();
        b.id = 2;
        b.steps.fill(Stage {
            pitch: 36,
            ..Stage::default()
        });
        let mut p = Preset::default();
        p.resynth.oscillators[0].transpose = 47.0;
        for tracks in [[a.clone(), b.clone()], [b, a]] {
            let mut r = Runtime::new(&tracks);
            r.tick(8000, 120.0, true);
            assert_eq!(current(&r, &p).resynth.oscillators[0].transpose, 47.0);
        }

        let mut main = lane(1, Target::MainPitch, 72);
        main.pitch_oscillators[0] = true;
        main.config[field::DIVISION] = 2;
        main.steps[1].pitch = 74;
        let mut osc = lane(2, Target::Oscillator { index: 0 }, 60);
        osc.steps[1].pitch = 67;
        let mut r = Runtime::new(&[main, osc]);
        for _ in 0..1001 {
            r.tick(8000, 120.0, true);
        }
        assert_eq!(r.core.stage(0), Some(0));
        assert_eq!(r.core.stage(1), Some(1));
        assert_eq!(
            current(&r, &Preset::default()).resynth.oscillators[0].transpose,
            19.0
        );
    }
}
