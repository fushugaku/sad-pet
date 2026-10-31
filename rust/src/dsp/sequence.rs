//! The actual Neuropolis sequencer, with SAD PET destinations and sample clock.
//! Prepared off-thread; ticks, edits, feedback and state adoption never allocate.
use super::Modulated;
use crate::{
    midi::Message,
    parameters,
    sequence::{Edit, PAGES, STEPS, Stage, Target, Track, TrackView, View},
};
use metropolix_core::{
    STAGES,
    order::{Direction, OrderMode, PlayOrder},
    random::FirmwareRandom,
    sequencer::{
        CLOCK_DIVISION_HALF, CLOCK_DIVISION_QUARTER, GateMode, PPQN, Sequencer, Step,
        Track as Playhead, field,
    },
};
use std::sync::OnceLock;

const _: () = assert!(crate::sequence::STAGES == STAGES);
/// The core's user scales follow its factory scales.
const USER_SCALE: usize = metropolix_core::scale::FACTORY_SCALES.len();

/// Config fields that move a track's stages and pulses in time.
const TIMING: [usize; 8] = [
    field::ORDER,
    field::LENGTH,
    field::REVERSE,
    field::DIVISION,
    field::SWING,
    field::OFFSET,
    field::PULSES_LENGTH,
    field::PULSE_OVERRIDE,
];
/// Shadow ticks per audio frame, ~85 ns each natively: a bar of sixteenths
/// relocks within 24 frames, ten minutes of an order without a plain period
/// within a sixth of a second.
const REPLAY_TICKS: u32 = 16;

type Timing = ([i32; TIMING.len()], u8, [(u8, bool); STEPS]);
/// What places a track's stages and pulses in time.
fn timing(config: &[i32; field::COUNT], lane: &Lane) -> Timing {
    (
        TIMING.map(|f| config[f]),
        lane.pages,
        lane.stages.map(|s| (s.pulses, s.skip)),
    )
}
/// metropolix-core's clock_division_ticks.
fn division_ticks(c: &[i32; field::COUNT]) -> u32 {
    match c[field::DIVISION] {
        CLOCK_DIVISION_HALF => 12,
        CLOCK_DIVISION_QUARTER => 6,
        division => 24 * division as u32,
    }
}
/// metropolix-core's pulse_duration: swing lengthens even pulses.
fn pulse_ticks(c: &[i32; field::COUNT], serial: u32) -> u32 {
    let base = division_ticks(c);
    let long = 2 * base * c[field::SWING] as u32 / 100;
    if serial.is_multiple_of(2) {
        long
    } else {
        2 * base - long
    }
}
/// Ticks after which a playhead repeats from its origin, where its settings
/// make that plain: Pulses length restarts the timeline and a Linear order
/// repeats every pass of its pages. Other orders replay from the origin.
fn period(c: &[i32; field::COUNT], lane: &Lane) -> Option<u64> {
    // metropolix-core's pulse_duration.
    let base = u64::from(division_ticks(c));
    let long = 2 * base * c[field::SWING] as u64 / 100;
    let ticks = |pulses: u64| pulses / 2 * 2 * base + pulses % 2 * long;
    if c[field::PULSES_LENGTH] > 0 {
        // Every restart of the timeline turns the page.
        return Some(u64::from(lane.pages) * ticks(c[field::PULSES_LENGTH] as u64));
    }
    if c[field::ORDER] != 0 {
        return None;
    }
    let pulses: u64 = lane
        .stages
        .chunks(STAGES)
        .take(lane.pages.into())
        .flat_map(|page| {
            (0..c[field::LENGTH])
                .map(|position| page[((position + c[field::OFFSET]) as usize) % STAGES])
        })
        .filter(|step| !step.skip)
        .map(|step| match c[field::PULSE_OVERRIDE] {
            0 => step.pulses as u64,
            pulses => pulses as u64,
        })
        .sum();
    // Swing alternates long and short pulses: an odd pass repeats after two.
    let pulses = if pulses % 2 == 1 && long != base {
        2 * pulses
    } else {
        pulses
    };
    (pulses > 0).then(|| ticks(pulses))
}

/// A SAD PET stage as the core plays it. `Stage::set` keeps every field in
/// the ranges `Sequencer::set_step` accepts.
fn core_step(s: &Stage) -> Step {
    Step {
        pitch: s.pitch.into(),
        pulses: s.pulses,
        gate: [
            GateMode::Rest,
            GateMode::Single,
            GateMode::Multiple,
            GateMode::Hold,
        ][usize::from(s.gate.min(3))],
        ratchet: s.ratchet,
        probability: s.probability,
        slide: s.slide,
        skip: s.skip,
        accumulate: s.accumulate,
        pitch_override: s.pitch_override,
        gate_override: s.gate_override,
        probability_target: s.probability_target,
        accumulate_trigger: s.accumulate_trigger,
        cv: s.cv,
        cv_ramp: s.cv_ramp,
        cv_gate: s.cv_gate,
    }
}
fn page(stages: &[Stage; STEPS], page: u8) -> &[Stage] {
    &stages[usize::from(page) * STAGES..][..STAGES]
}
/// Put one page in the core, the `masked` stages skipped.
fn load(playhead: &mut Playhead, stages: &[Stage; STEPS], index: u8, masked: u16) {
    for (slot, (step, stage)) in playhead
        .steps
        .iter_mut()
        .zip(page(stages, index))
        .enumerate()
    {
        *step = core_step(stage);
        step.skip |= masked >> slot & 1 != 0;
    }
}
fn play_order(mode: usize, length: u8, reverse: bool, random: &mut FirmwareRandom) -> PlayOrder {
    let direction = if reverse {
        Direction::Reverse
    } else {
        Direction::Forward
    };
    PlayOrder::new(OrderMode::ALL[mode], length, direction, random).expect("1..=16 stages")
}
/// Visits after which each play order repeats from its reset, by order,
/// length and direction. Zero for the orders that draw random numbers,
/// Shuffle too: its permutation comes from the core's random stream.
fn cycles() -> &'static [[[u8; 2]; STAGES]; 21] {
    static CYCLES: OnceLock<[[[u8; 2]; STAGES]; 21]> = OnceLock::new();
    CYCLES.get_or_init(|| {
        // Growing and Shrinking over sixteen stages repeat after 136 visits.
        const LONGEST: usize = 136;
        let mut table = [[[0; 2]; STAGES]; 21];
        for (mode, lengths) in table
            .iter_mut()
            .enumerate()
            .take(OrderMode::Brownian as usize)
        {
            for (length, directions) in lengths.iter_mut().enumerate() {
                for (reverse, cycle) in directions.iter_mut().enumerate() {
                    let mut random = FirmwareRandom::new(0, 0);
                    let mut order = play_order(mode, length as u8 + 1, reverse == 1, &mut random);
                    let mut visits = vec![order.current()];
                    visits.extend((0..2 * LONGEST).map(|_| order.advance(&mut random)));
                    *cycle = (1..=LONGEST)
                        .find(|&p| (0..=LONGEST).all(|i| visits[i] == visits[i + p]))
                        .expect("deterministic orders repeat") as u8;
                }
            }
        }
        table
    })
}
/// One pass of a page: its stage starts in one cycle of the play order, and
/// the stages that cycle still visits, all skipped, after its last start.
/// Random orders play as many stages as Length leaves unskipped.
fn pass(c: &[i32; field::COUNT], page: &[Stage]) -> (u16, u16) {
    let length = c[field::LENGTH].clamp(1, STAGES as i32) as u8;
    let slot =
        |position: u8| (i32::from(position) + c[field::OFFSET]).rem_euclid(STAGES as i32) as usize;
    let mode = c[field::ORDER].clamp(0, 20) as usize;
    let reverse = c[field::REVERSE] != 0;
    let cycle = cycles()[mode][usize::from(length) - 1][usize::from(reverse)];
    if cycle == 0 {
        let starts = (0..length).filter(|&p| !page[slot(p)].skip).count();
        return ((starts as u16).max(1), 0);
    }
    // Deterministic orders draw nothing.
    let mut random = FirmwareRandom::new(0, 0);
    let mut order = play_order(mode, length, reverse, &mut random);
    let (mut starts, mut trailing) = (0, 0_u16);
    for visit in 0..cycle {
        if visit > 0 {
            order.advance(&mut random);
        }
        let slot = slot(order.current());
        if page[slot].skip {
            trailing |= 1 << slot;
        } else {
            starts += 1;
            trailing = 0;
        }
    }
    if starts == 0 {
        (1, 0)
    } else {
        (starts, trailing)
    }
}

/// A track's place among its pages. It mirrors the core's pulse timeline to
/// see each stage end a tick ahead: the next page is in the core before the
/// stage that starts it. The play order keeps walking across pages.
#[derive(Clone, Copy, Debug, Default)]
struct Pager {
    page: u8,
    /// Stage starts on this page, and how many make its pass.
    played: u16,
    pass: u16,
    /// Stages this page's pass still visits, skipped, after its last start.
    trailing: u16,
    /// The last page's trailing stages, held skipped in the core until the
    /// order has walked past them into this page.
    masked: u16,
    /// The core timeline as of the last tick: ticks into the pulse, pulse
    /// in the stage, pulse serial (for swing) and pulses since a restart.
    elapsed: u32,
    pulse: u32,
    serial: u32,
    cycle: u32,
}
impl Pager {
    fn new(config: &[i32; field::COUNT], stages: &[Stage; STEPS]) -> Self {
        let (pass, trailing) = pass(config, page(stages, 0));
        Self {
            pass,
            trailing,
            ..Self::default()
        }
    }
    /// Follow a core tick, counting as Timeline::prepare and finish do.
    fn follow(&mut self, playhead: &Playhead) {
        if playhead.reset_pulse {
            (self.elapsed, self.pulse, self.serial, self.cycle) = (0, 0, 0, 0);
        } else if playhead.pulse {
            self.elapsed = 0;
            self.pulse = if playhead.changed { 0 } else { self.pulse + 1 };
            self.serial = self.serial.wrapping_add(1);
            self.cycle = self.cycle.saturating_add(1);
        } else {
            self.elapsed = self.elapsed.saturating_add(1);
        }
        if playhead.changed {
            self.played = self.played.saturating_add(1);
        }
    }
    /// After `follow`: whether the pulse the last tick played ends with it,
    /// whether its stage does, and whether Pulses length restarts the
    /// timeline at the next tick.
    fn ends(&self, playhead: &Playhead, stage: u8) -> (bool, bool, bool) {
        let c = &playhead.config;
        if self.elapsed + 1 < pulse_ticks(c, self.serial) {
            return (false, false, false);
        }
        let limit = c[field::PULSES_LENGTH];
        let restart = limit > 0 && self.cycle + 1 >= limit as u32;
        let pulses = match c[field::PULSE_OVERRIDE] {
            pulses if pulses > 0 => pulses as u32,
            _ => u32::from(playhead.steps[usize::from(stage)].pulses),
        };
        (true, restart || self.pulse + 1 >= pulses, restart)
    }
    /// The page's pass ends with the last tick: the stage that one played
    /// is its last, or Pulses length restarts the timeline.
    fn turns(&self, playhead: &Playhead, stage: u8) -> bool {
        let (_, stage_ends, restart) = self.ends(playhead, stage);
        if playhead.config[field::PULSES_LENGTH] > 0 {
            restart
        } else {
            stage_ends && self.played >= self.pass
        }
    }
    /// After each core tick: let the last page's trailing stages play again
    /// once the order is past them, and turn the page when its pass ends.
    fn tick(&mut self, pages: u8, stages: &[Stage; STEPS], playhead: &mut Playhead, stage: u8) {
        if playhead.changed && self.masked != 0 {
            self.masked = 0;
            load(playhead, stages, self.page, 0);
        }
        self.follow(playhead);
        // A page past a lowered count still turns, back to the first.
        if (pages > 1 || self.page > 0) && self.turns(playhead, stage) {
            self.page = if self.page + 1 >= pages {
                0
            } else {
                self.page + 1
            };
            // A restarted timeline begins the order afresh.
            self.masked = if playhead.config[field::PULSES_LENGTH] > 0 {
                0
            } else {
                self.trailing
            };
            (self.pass, self.trailing) = pass(&playhead.config, page(stages, self.page));
            self.played = 0;
            load(playhead, stages, self.page, self.masked);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Binding {
    MainPitch,
    Oscillator(usize),
    SynthOscillator(usize),
    Tb303Oscillator(usize),
    WavetableOscillator(usize),
    Parameter(usize),
    Notes,
    /// Each gate starts this LFO over.
    LfoReset(usize),
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
    /// The LFO each step destination resets, on gates of steps set to 1.
    resets: [Option<usize>; 3],
    amount: f32,
    absolute: bool,
    gate_output: bool,
    env_gate: bool,
    root: u8,
    pitch_oscillators: [bool; 4],
    /// Every page's stages; the core holds the page playing.
    stages: Box<[Stage; STEPS]>,
    pages: u8,
    pager: Pager,
    held: Option<(u8, u8)>,
    pitch: Glide,
    /// Core tick where this playhead started: the transport start, or its
    /// own Restart track.
    origin: u64,
    /// A timing edit moved the playhead off the place its settings, played
    /// from its origin, give it now: a replay is queued.
    relock: bool,
    /// Added while playing: silent until its replay has found its place.
    waiting: bool,
}
impl Lane {
    /// A stage of the page playing.
    fn stage(&self, slot: u8) -> &Stage {
        &page(&self.stages, self.pager.page)[usize::from(slot)]
    }
}
#[derive(Clone, Copy)]
struct Route {
    parameter: usize,
    lane: Option<usize>,
    oscillator: Option<usize>,
    destination: Option<usize>,
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
    /// One track replayed from its origin faster than real time. When it has
    /// caught up with the live tick, its playhead replaces the one a timing
    /// edit moved, so every lane stays on the grid of the transport start.
    shadow: Sequencer,
    shadow_pager: Pager,
    replay: Option<usize>,
    /// The live tick where the shadow's replay starts.
    anchor: u64,
    relocks: usize,
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
                let stages = Box::new(t.steps);
                load(&mut core.tracks[i], &stages, 0, 0);
                let pager = Pager::new(&core.tracks[i].config, &stages);
                let binding = match &t.target {
                    Target::MainPitch => Binding::MainPitch,
                    Target::Oscillator { index } => Binding::Oscillator(*index as usize),
                    Target::SynthOscillator { index } => Binding::SynthOscillator(*index as usize),
                    Target::Tb303Oscillator { index } => Binding::Tb303Oscillator(*index as usize),
                    Target::WavetableOscillator { index } => Binding::WavetableOscillator(
                        (*index as usize).min(crate::wavetable::OSCILLATORS - 1),
                    ),
                    Target::Notes | Target::Pad { .. } => Binding::Notes,
                    Target::Parameter { parameter } => crate::sequence::lfo_reset(parameter)
                        .map_or_else(
                            || {
                                parameters::specs()
                                    .iter()
                                    .position(|s| s.id == *parameter && s.can_sequence())
                                    .map_or(Binding::Unbound, Binding::Parameter)
                            },
                            Binding::LfoReset,
                        ),
                };
                let route = match binding {
                    Binding::MainPitch => parameters::specs()
                        .iter()
                        .position(|s| s.id == "sample_tune_semis"),
                    Binding::Oscillator(index) => {
                        let id = format!("resynth_osc{}_transpose", index + 1);
                        parameters::specs().iter().position(|s| s.id == id)
                    }
                    Binding::SynthOscillator(index) => {
                        let id = format!("synth_osc{}_transpose", index + 1);
                        parameters::specs().iter().position(|s| s.id == id)
                    }
                    Binding::Tb303Oscillator(index) => {
                        let id = format!("tb303_osc{}_transpose", index + 1);
                        parameters::specs().iter().position(|s| s.id == id)
                    }
                    Binding::WavetableOscillator(index) => {
                        let id = format!("wt_{}_transpose", ["a", "b", "c", "d"][index]);
                        parameters::specs().iter().position(|s| s.id == id)
                    }
                    Binding::Parameter(index) => Some(index),
                    _ => None,
                };
                Lane {
                    id: t.id,
                    binding,
                    route,
                    resets: std::array::from_fn(|d| {
                        t.destinations[d]
                            .as_deref()
                            .and_then(crate::sequence::lfo_reset)
                    }),
                    amount: t.amount,
                    absolute: t.absolute,
                    gate_output: t.gate_output,
                    env_gate: t.env_gate,
                    root: t.root_note,
                    pitch_oscillators: t.pitch_oscillators,
                    stages,
                    pages: t.pages.clamp(1, PAGES as u8),
                    pager,
                    held: None,
                    pitch: Glide::default(),
                    origin: 0,
                    relock: false,
                    waiting: false,
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
                    destination: None,
                });
            }
            for (destination, target) in tracks[i].destinations.iter().enumerate() {
                if let Some(parameter) = target.as_ref().and_then(|id| {
                    parameters::specs()
                        .iter()
                        .position(|s| s.id == *id && s.can_sequence())
                }) {
                    routes.push(Route {
                        parameter,
                        lane: Some(i),
                        oscillator: oscillator_parameters.iter().position(|&id| id == parameter),
                        destination: Some(destination),
                    });
                }
            }
        }
        // Include each carrier once even when it has no lane of its own. Main
        // pitch masks can then change in place, without four routes per track.
        for (oscillator, &parameter) in oscillator_parameters.iter().enumerate() {
            routes.push(Route {
                parameter,
                lane: None,
                oscillator: Some(oscillator),
                destination: None,
            });
        }
        routes.sort_unstable_by_key(|route| route.parameter);
        let main_pitch_routes = (0..lanes.len())
            .filter(|&i| lanes[i].binding == Binding::MainPitch)
            .collect();
        let oscillator_routes = (0..lanes.len())
            .filter(|&i| {
                matches!(
                    lanes[i].binding,
                    Binding::Oscillator(_)
                        | Binding::SynthOscillator(_)
                        | Binding::Tb303Oscillator(_)
                        | Binding::WavetableOscillator(_)
                )
            })
            .collect();
        let mut shadow = Sequencer::default();
        shadow.resize_tracks(1);
        for lane in &mut shadow.mods {
            lane.config[9] = 1;
        }
        let mut runtime = Self {
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
            shadow,
            shadow_pager: Pager::default(),
            replay: None,
            anchor: 0,
            relocks: 0,
        };
        for (i, t) in tracks.iter().enumerate() {
            runtime.scale(i, t);
        }
        runtime
    }
    /// The core quantizes to the track's Scale, or to the notes picked for it,
    /// held in a user scale that tracks with the same notes share.
    fn scale(&mut self, i: usize, track: &Track) {
        let mut index = track.config[field::SCALE];
        if track.notes != 0 {
            let (core, root) = (&self.core, track.config[field::ROOT]);
            let users = 0..core.user_roots.len();
            let slot = users
                .clone()
                .find(|&k| {
                    core.scales[USER_SCALE + k].bits() == track.notes
                        && i32::from(core.user_roots[k]) == root
                })
                .or_else(|| {
                    users.clone().find(|&k| {
                        core.tracks.iter().enumerate().all(|(j, t)| {
                            j == i || t.config[field::SCALE] != (USER_SCALE + k) as i32
                        })
                    })
                });
            // With every user scale holding other tracks' notes, it keeps its Scale.
            if let Some(k) = slot {
                let mask = i32::from(track.notes);
                self.core.set_scale(USER_SCALE + k, mask, root);
                self.shadow.set_scale(USER_SCALE + k, mask, root);
                index = (USER_SCALE + k) as i32;
            }
        }
        self.core.set_track(i, field::SCALE as u32, index);
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
                let moved =
                    timing(&next_tracks[i].config, lane) != timing(&old_tracks[j].config, old_lane);
                let config = next_tracks[i].config;
                std::mem::swap(&mut next_tracks[i], &mut old_tracks[j]);
                next_tracks[i].config = config;
                // The page playing stays in place, with the new stages.
                lane.pager = old_lane.pager;
                (lane.pager.pass, lane.pager.trailing) =
                    pass(&config, page(&lane.stages, lane.pager.page));
                load(
                    &mut next_tracks[i],
                    &lane.stages,
                    lane.pager.page,
                    lane.pager.masked,
                );
                lane.origin = old_lane.origin;
                lane.waiting = old_lane.waiting;
                lane.relock = old_lane.relock || old.replay == Some(j) || moved;
                // A new destination keeps the playhead; held notes and
                // glides belong to the old one.
                if lane.binding == old_lane.binding {
                    lane.held = old_lane.held;
                    lane.pitch = old_lane.pitch;
                }
            } else {
                // Added while playing: it joins the grid of the others.
                lane.relock = true;
                lane.waiting = true;
            }
            lane.relock &= old.started;
            lane.waiting &= old.started;
        }
        self.relocks = self.lanes.iter().filter(|l| l.relock).count();
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
                // Restart track starts a grid of its own for this track.
                self.lanes[index].origin = self.core.ticks;
                self.first_page(index);
                self.settle(index);
            }
            return;
        }
        let before = timing(&self.core.tracks[index].config, &self.lanes[index]);
        self.set(index, track, edit);
        if timing(&self.core.tracks[index].config, &self.lanes[index]) != before {
            self.relock(index);
        }
    }
    /// Back to the first page, where a reset core playhead starts.
    fn first_page(&mut self, i: usize) {
        let (lane, playhead) = (&mut self.lanes[i], &mut self.core.tracks[i]);
        lane.pager = Pager::new(&playhead.config, &lane.stages);
        load(playhead, &lane.stages, 0, 0);
    }
    fn set(&mut self, index: usize, track: &Track, edit: Edit) {
        if let Some(stage) = edit.stage.map(usize::from) {
            let lane = &mut self.lanes[index];
            lane.stages[stage] = track.steps[stage];
            if stage / STAGES == usize::from(lane.pager.page) {
                let slot = stage % STAGES;
                let step = &mut self.core.tracks[index].steps[slot];
                *step = core_step(&track.steps[stage]);
                step.skip |= lane.pager.masked >> slot & 1 != 0;
            }
        } else if edit.field == 54 {
            // New pages may have been filled from the last one.
            let lane = &mut self.lanes[index];
            *lane.stages = track.steps;
            lane.pages = track.pages.clamp(1, PAGES as u8);
        } else if (edit.field as usize) < field::COUNT {
            self.core
                .set_track(index, edit.field as u32, track.config[edit.field as usize]);
            if matches!(edit.field as usize, field::SCALE | field::ROOT) {
                self.scale(index, track);
            }
        } else if edit.field == 55 {
            self.scale(index, track);
        } else {
            let l = &mut self.lanes[index];
            l.amount = track.amount;
            l.absolute = track.absolute;
            l.gate_output = track.gate_output;
            l.env_gate = track.env_gate;
            l.root = track.root_note;
            l.pitch_oscillators = track.pitch_oscillators;
        }
    }
    /// Queue a replay that moves this playhead to where its settings, played
    /// from its origin, put it now.
    fn relock(&mut self, i: usize) {
        if !self.started {
            return;
        }
        if self.replay == Some(i) {
            self.replay = None;
        }
        if !self.lanes[i].relock {
            self.lanes[i].relock = true;
            self.relocks += 1;
        }
    }
    /// The playhead is where it belongs: drop its queued or running replay.
    fn settle(&mut self, i: usize) {
        if self.replay == Some(i) {
            self.replay = None;
        }
        if self.lanes[i].relock {
            self.lanes[i].relock = false;
            self.relocks -= 1;
        }
        self.lanes[i].waiting = false;
    }
    /// Advance the shadow; right after the live tick it has caught up with,
    /// its playhead takes over. Edits keep applying to the live track.
    fn replay(&mut self, live_tick: bool) {
        let i = match self.replay {
            Some(i) => i,
            None if self.relocks == 0 => return,
            None => {
                let i = self.lanes.iter().position(|l| l.relock).unwrap();
                self.lanes[i].relock = false;
                self.relocks -= 1;
                let (live, lane) = (&self.core.tracks[i], &self.lanes[i]);
                let origin = lane.origin;
                let elapsed = self.core.ticks - origin;
                self.anchor = origin + period(&live.config, lane).map_or(0, |p| elapsed / p * p);
                let shadow = &mut self.shadow.tracks[0];
                shadow.config = live.config;
                load(shadow, &lane.stages, 0, 0);
                self.shadow_pager = Pager::new(&live.config, &lane.stages);
                self.shadow.restart();
                // Drop the accumulators of the playhead it held last, even
                // ones set never to reset.
                self.shadow.reset_track(0, true);
                self.replay = Some(i);
                i
            }
        };
        let target = self.core.ticks - self.anchor;
        for _ in 0..REPLAY_TICKS {
            if self.shadow.ticks >= target {
                break;
            }
            self.shadow.tick();
            let (lane, stage) = (&self.lanes[i], self.shadow.stage(0).unwrap_or(0));
            self.shadow_pager
                .tick(lane.pages, &lane.stages, &mut self.shadow.tracks[0], stage);
        }
        if live_tick && self.shadow.ticks == target {
            let live = &mut self.core.tracks[i];
            let config = live.config;
            std::mem::swap(live, &mut self.shadow.tracks[0]);
            live.config = config;
            let lane = &mut self.lanes[i];
            // Edits during the replay reached the lane's stages.
            lane.pager = self.shadow_pager;
            load(live, &lane.stages, lane.pager.page, lane.pager.masked);
            if lane.waiting {
                // A new lane starts at its pitch instead of gliding from 0.
                let pitch = self.core.cents(i) as f32 * 0.01;
                lane.pitch = Glide {
                    from: pitch,
                    to: pitch,
                    ..Glide::default()
                };
                lane.waiting = false;
            }
            self.replay = None;
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
            l.origin = 0;
            l.relock = false;
            l.waiting = false;
        }
        for i in 0..self.lanes.len() {
            self.first_page(i);
        }
        self.replay = None;
        self.relocks = 0;
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
            for i in 0..self.lanes.len() {
                let (stage, lane) = (self.core.stage(i).unwrap_or(0), &mut self.lanes[i]);
                lane.pager
                    .tick(lane.pages, &lane.stages, &mut self.core.tracks[i], stage);
            }
            self.replay(true);
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
        } else {
            self.replay(false);
        }
        self.phase += step;
        tick
    }
    pub fn len(&self) -> usize {
        self.lanes.len()
    }
    /// Unmuted and placed on the grid.
    fn sounding(&self, i: usize) -> bool {
        self.core.tracks[i].config[field::MUTE] == 0 && !self.lanes[i].waiting
    }
    pub fn actions(&mut self, i: usize) -> (u32, [Option<Action>; 2]) {
        let mut actions = [None; 2];
        if self.lanes[i].binding != Binding::Notes {
            return (self.lanes[i].id, actions);
        }
        let on = self.enabled && self.sounding(i) && self.core.gate(i);
        let lane = &mut self.lanes[i];
        let flags = self.core.track_events(i);
        let channel = (self.core.tracks[i].config[field::MIDI_CHANNEL] - 1).max(0) as u8;
        let raw = self.core.note(i).unwrap_or(60) as f32;
        let note = (lane.root as f32 + (raw - lane.root as f32) * lane.amount)
            .round()
            .clamp(0.0, 127.0) as u8;
        let stage = lane.stage(self.core.stage(i).unwrap_or(0));
        let velocity =
            (self.core.velocity(i) * i32::from(stage.velocity) / 127).clamp(1, 127) as u8;
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
    /// Unmuted Main pitch lanes that gate ENV, read right after a clock tick:
    /// whether one of them triggered a new gate and whether any gate is open.
    /// Tied steps keep the gate open without a retrigger, like a legato note.
    pub fn env_gate(&self) -> Option<(bool, bool)> {
        let mut gate = None;
        for &i in &self.main_pitch_routes {
            if self.lanes[i].env_gate && self.sounding(i) {
                let open = self.core.gate(i);
                let (trigger, any) = gate.get_or_insert((false, false));
                *trigger |= open && self.core.track_events(i) & 1 != 0;
                *any |= open;
            }
        }
        gate
    }
    /// LFOs a step started over, read right after a clock tick: one bit each.
    /// A reset lane's every gate resets; a step destination's, where the
    /// step is set to 1.
    pub fn lfo_resets(&self) -> u8 {
        let mut mask = 0;
        for (i, lane) in self.lanes.iter().enumerate() {
            if !self.sounding(i) || self.core.track_events(i) & 1 == 0 {
                continue;
            }
            if let Binding::LfoReset(lfo) = lane.binding {
                mask |= 1 << lfo;
            }
            let stage = lane.stage(self.core.stage(i).unwrap_or(0));
            for (d, lfo) in lane.resets.iter().enumerate() {
                if let Some(lfo) = lfo
                    && stage.get(16 + d as u8) >= 0.5
                {
                    mask |= 1 << lfo;
                }
            }
        }
        mask
    }
    pub fn apply(&self, m: &mut Modulated) {
        if !self.enabled || !self.started {
            return;
        }
        let now = self.core.ticks.saturating_sub(1) as f64 + self.phase.min(1.0);
        let mut oscillator_pitch = [0.0; 4];
        for &i in &self.main_pitch_routes {
            let lane = &self.lanes[i];
            if self.sounding(i) && lane.pitch_oscillators.iter().any(|&on| on) {
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
                if !self.sounding(i) {
                    continue;
                }
                delta += if let Some(destination) = route.destination {
                    self.core.stage(i).map_or(0.0, |stage| {
                        l.stage(stage).get(16 + destination as u8) * (spec.max - spec.min)
                    })
                } else if matches!(
                    l.binding,
                    Binding::MainPitch
                        | Binding::Oscillator(_)
                        | Binding::SynthOscillator(_)
                        | Binding::Tb303Oscillator(_)
                        | Binding::WavetableOscillator(_)
                ) {
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
            if l.gate_output && self.sounding(i) && !self.core.gate(i) {
                // Layer gains slew towards zero, so a closed gate never clicks.
                match l.binding {
                    Binding::Oscillator(index) => m.resynth.oscillators[index].level = 0.0,
                    Binding::SynthOscillator(index) => m.synth.oscillators[index].level = 0.0,
                    Binding::WavetableOscillator(index) => m.wavetable[index].level = 0.0,
                    Binding::Tb303Oscillator(index) => {
                        m.tb303_radias.oscillators[index].level = 0.0
                    }
                    _ => {}
                }
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
                // Counted across pages.
                step: l.pager.page * STAGES as u8 + self.core.stage(i).unwrap_or(0),
                gate: self.enabled && !l.waiting && self.core.gate(i),
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
            }; STEPS],
            ..Track::default()
        }
    }
    fn current(runtime: &Runtime, preset: &Preset) -> Modulated {
        let mut m = Modulated::from_preset(preset);
        runtime.apply(&mut m);
        m
    }

    #[test]
    fn picked_notes_share_user_scales_and_free_them_for_other_sets() {
        let with = |id, notes, root| {
            let mut t = lane(id, Target::Notes, 61);
            t.notes = notes;
            t.config[field::ROOT] = root;
            t
        };
        let mut tracks = [
            with(1, 0x091, 0),
            with(2, 0x091, 0),
            with(3, 0x091, 4),
            with(4, 0, 0),
        ];
        let mut r = Runtime::new(&tracks);
        let scales = |r: &Runtime| [0, 1, 2, 3].map(|i| r.core.tracks[i].config[field::SCALE]);
        // Same notes and root share one; the shadow has them all too.
        assert_eq!(scales(&r), [49, 49, 50, 0]);
        assert_eq!((r.core.scales[50].bits(), r.core.user_roots[1]), (0x091, 4));
        assert_eq!(
            (r.shadow.scales, r.shadow.user_roots),
            (r.core.scales, r.core.user_roots)
        );
        let mut edit = |r: &mut Runtime, i: usize, field: u8, value: f32| {
            let e = Edit {
                track: i as u32 + 1,
                stage: None,
                field,
                value,
            };
            tracks[i].edit(e);
            r.edit(i, &tracks[i], e);
        };
        // A set only one track used makes room for that track's next one.
        edit(&mut r, 2, 55, 0x0a5 as f32);
        assert_eq!(scales(&r), [49, 49, 50, 0]);
        assert_eq!(r.core.scales[50].bits(), 0x0a5);
        // One still in use stays; a root change is a set of its own.
        edit(&mut r, 0, 5, 7.0);
        assert_eq!(scales(&r), [51, 49, 50, 0]);
        assert_eq!((r.core.scales[49].bits(), r.core.user_roots[0]), (0x091, 0));
        assert_eq!(r.core.user_roots[2], 7);
        edit(&mut r, 3, 55, 0x091 as f32);
        assert_eq!(scales(&r)[3], 49);
        // A Scale picked by name leaves the user scales.
        edit(&mut r, 1, field::SCALE as u8, 5.0);
        assert_eq!(scales(&r), [51, 5, 50, 49]);
        assert_eq!(
            (r.shadow.scales, r.shadow.user_roots),
            (r.core.scales, r.core.user_roots)
        );

        // Past a hundred different sets, the rest keep their Scale.
        let many: Vec<_> = (1..=101).map(|i| with(i, i as u16, 0)).collect();
        let r = Runtime::new(&many);
        assert_eq!(r.core.tracks[99].config[field::SCALE], 148);
        assert_eq!(r.core.tracks[100].config[field::SCALE], 0);
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
    fn synth_oscillator_lanes_offset_and_gate_synth_layers_only() {
        let mut osc = lane(1, Target::SynthOscillator { index: 1 }, 67);
        osc.gate_output = true;
        let mut r = Runtime::new(&[osc]);
        r.tick(8000, 120.0, true);
        let mut p = Preset::default();
        let mut synth = crate::synth::Settings::default();
        synth.oscillators[1].transpose = -5.0;
        synth.oscillators[1].level = 0.8;
        p.synth = Some(Box::new(synth));
        p.resynth.oscillators[1].transpose = 2.0;
        let m = current(&r, &p);
        assert_eq!(m.synth.oscillators[1].transpose, 2.0);
        assert_eq!(m.synth.oscillators[1].level, 0.8);
        assert_eq!(m.resynth.oscillators[1].transpose, 2.0);
        // The step's gate closes after its gate time; then the layer mutes.
        for _ in 0..8000 {
            if !r.core.gate(0) {
                break;
            }
            r.tick(8000, 120.0, true);
        }
        assert!(!r.core.gate(0));
        let m = current(&r, &p);
        assert_eq!(m.synth.oscillators[1].level, 0.0);
        assert_eq!(
            m.resynth.oscillators[1].level,
            p.resynth.oscillators[1].level
        );
    }

    #[test]
    fn wavetable_pitch_lanes_only_offset_and_gate_their_own_oscillator() {
        for index in 0..crate::wavetable::OSCILLATORS {
            let mut osc = lane(1, Target::WavetableOscillator { index: index as u8 }, 67);
            osc.gate_output = true;
            let mut r = Runtime::new(&[osc]);
            r.tick(8000, 120.0, true);
            let mut p = crate::timbres::Kind::Wavetable.preset().unwrap();
            p.wavetable.as_mut().unwrap().count = 4;
            for oscillator in &mut p.wavetable.as_mut().unwrap().oscillators {
                oscillator.controls.enabled = true;
            }
            p.wavetable.as_mut().unwrap().oscillators[index]
                .controls
                .transpose = -5.0;
            p.wavetable.as_mut().unwrap().oscillators[index]
                .controls
                .level = 0.8;
            let m = current(&r, &p);
            for (i, oscillator) in m.wavetable.iter().enumerate() {
                assert_eq!(oscillator.transpose, if i == index { 2.0 } else { 0.0 });
            }
            assert_eq!(m.wavetable[index].level, 0.8);
            assert_eq!(m.params.sample_tune_semis, 0.0);
            for _ in 0..8000 {
                if !r.core.gate(0) {
                    break;
                }
                r.tick(8000, 120.0, true);
            }
            assert!(!r.core.gate(0));
            let m = current(&r, &p);
            for (i, oscillator) in m.wavetable.iter().enumerate() {
                assert_eq!(oscillator.level, if i == index { 0.0 } else { 1.0 });
            }
        }
    }

    #[test]
    fn tb303_radias_pitch_and_gates_do_not_edit_other_banks() {
        let mut osc = lane(1, Target::Tb303Oscillator { index: 2 }, 67);
        osc.gate_output = true;
        let main = lane(2, Target::MainPitch, 72);
        let mut r = Runtime::new(&[osc, main]);
        r.tick(8000, 120.0, true);
        let mut p = Preset {
            tb303: Some(Box::default()),
            ..Default::default()
        };
        p.tb303.as_mut().unwrap().oscillators[2].transpose = -12.0;
        let m = current(&r, &p);
        assert_eq!(m.params.sample_tune_semis, 12.0);
        assert_eq!(m.tb303_radias.oscillators[2].transpose, -5.0);
        assert_eq!(m.synth.oscillators[2].transpose, 0.0);
        assert_eq!(m.resynth.oscillators[2].transpose, 0.0);
        for _ in 0..8000 {
            if !r.core.gate(0) {
                break;
            }
            r.tick(8000, 120.0, true);
        }
        assert_eq!(current(&r, &p).tb303_radias.oscillators[2].level, 0.0);
        assert!(current(&r, &p).tb303.level > 0.0);
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

    #[test]
    fn restart_track_keeps_its_own_grid_and_added_lanes_wait_silently_for_theirs() {
        let mut tracks = vec![lane(1, Target::Notes, 60), lane(2, Target::Notes, 60)];
        let mut r = Runtime::new(&tracks);
        let run = |r: &mut Runtime, ticks: u64| {
            let end = r.core.ticks + ticks;
            while r.core.ticks < end {
                r.tick(8000, 120.0, true);
            }
        };
        // Sixteenths from a playhead's origin, as of the last tick.
        let stage = |r: &Runtime, origin: u64| Some(((r.core.ticks - 1 - origin) / 24 % 16) as u8);
        run(&mut r, 1010);
        let restart = Edit {
            track: 2,
            stage: None,
            field: 47,
            value: 1.0,
        };
        r.edit(1, &tracks[1], restart);
        let origin = r.core.ticks;
        run(&mut r, 100);
        // A timing edit and its undo relock to the restart, not the transport.
        for value in [2.0, 1.0] {
            let edit = Edit {
                track: 2,
                stage: None,
                field: 3,
                value,
            };
            tracks[1].edit(edit);
            r.edit(1, &tracks[1], edit);
            run(&mut r, 50);
        }
        assert_eq!(r.core.stage(0), stage(&r, 0));
        assert_eq!(r.core.stage(1), stage(&r, origin));
        assert_ne!(r.core.stage(0), r.core.stage(1));

        tracks.push(lane(3, Target::Notes, 60));
        let mut next = Runtime::new(&tracks);
        next.inherit(&mut r);
        let mut waited = 0;
        loop {
            let ticked = next.tick(8000, 120.0, true);
            if !next.lanes[2].waiting {
                break;
            }
            if ticked {
                assert!(next.actions(2).1.iter().all(Option::is_none));
            }
            waited += 1;
        }
        assert!(waited > 0);
        assert_eq!(next.core.stage(2), next.core.stage(0));
        assert_eq!(next.core.stage(1), stage(&next, origin));
    }

    /// `pages` of steps whose CV is ten times their place across pages.
    fn paged(pages: u8) -> Track {
        let mut t = lane(1, Target::Notes, 60);
        t.pages = pages;
        for (i, s) in t.steps.iter_mut().enumerate() {
            s.cv = 10 * i as i32;
        }
        t
    }
    /// The first `count` stage starts, as (step across pages, CV).
    fn starts(r: &mut Runtime, count: usize) -> Vec<(u8, i32)> {
        let mut out = Vec::new();
        while out.len() < count {
            if r.tick(8000, 120.0, true) && r.core.tracks[0].changed {
                out.push((r.view().tracks[0].step, r.core.cv(0)));
            }
        }
        out
    }
    fn steps(starts: &[(u8, i32)]) -> Vec<u8> {
        for &(step, cv) in starts {
            assert_eq!(cv, 10 * i32::from(step), "step {step} played another page");
        }
        starts.iter().map(|s| s.0).collect()
    }
    fn cycle(pattern: impl IntoIterator<Item = u8>, count: usize) -> Vec<u8> {
        pattern
            .into_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .cycle()
            .take(count)
            .collect()
    }

    #[test]
    fn pages_play_in_turn_for_one_pass_of_the_order_each() {
        let mut r = Runtime::new(&[paged(3)]);
        assert_eq!(steps(&starts(&mut r, 100)), cycle(0..48, 100));

        // The middle page ends early and the last starts late: the order
        // walks over the skipped stages of both into the next page.
        let mut t = paged(3);
        for stage in (28..32).chain(32..35) {
            t.steps[stage].skip = true;
        }
        let mut r = Runtime::new(&[t]);
        assert_eq!(steps(&starts(&mut r, 90)), cycle((0..28).chain(35..48), 90));

        // Ping Pong plays its whole figure on each page.
        let mut t = paged(2);
        t.config[field::ORDER] = 1;
        t.config[field::LENGTH] = 4;
        let mut r = Runtime::new(&[t]);
        assert_eq!(
            steps(&starts(&mut r, 40)),
            cycle([0, 1, 2, 3, 3, 2, 1, 0, 16, 17, 18, 19, 19, 18, 17, 16], 40)
        );

        // Pulses length restarts the timeline, and each restart turns the page.
        let mut t = paged(2);
        t.config[field::PULSES_LENGTH] = 6;
        let mut r = Runtime::new(&[t]);
        assert_eq!(steps(&starts(&mut r, 30)), cycle((0..6).chain(16..22), 30));

        // A random order plays as many stages as each page leaves unskipped.
        let mut t = paged(2);
        t.config[field::ORDER] = 20;
        for stage in [17, 20, 25, 31] {
            t.steps[stage].skip = true;
        }
        let mut r = Runtime::new(&[t]);
        let pages: Vec<u8> = steps(&starts(&mut r, 56)).iter().map(|s| s / 16).collect();
        assert_eq!(pages, cycle([[0; 16].as_slice(), &[1; 12]].concat(), 56));
    }

    #[test]
    fn the_pager_sees_every_pulse_and_stage_end_one_tick_ahead() {
        let mut seed = 0x9e37_79b9_u32;
        let mut next = |n: u32| {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            seed % n
        };
        for _ in 0..300 {
            let mut core = Sequencer::default();
            core.resize_tracks(1);
            for lane in &mut core.mods {
                lane.config[9] = 1;
            }
            let config = [
                (field::ORDER, next(21) as i32),
                (field::LENGTH, 1 + next(16) as i32),
                (field::OFFSET, next(16) as i32),
                (field::REVERSE, next(2) as i32),
                (field::DIVISION, [1, 2, 3, 65, 66][next(5) as usize]),
                (field::SWING, 50 + next(29) as i32),
                (
                    field::PULSE_OVERRIDE,
                    [0, 0, 1 + next(8) as i32][next(3) as usize],
                ),
                (
                    field::PULSES_LENGTH,
                    [0, 0, 1 + next(64) as i32][next(3) as usize],
                ),
            ];
            for (field, value) in config {
                assert!(core.set_track(0, field as u32, value));
            }
            for stage in 0..STAGES {
                core.set_step(0, stage, 1, 1 + next(8) as i32);
                core.set_step(0, stage, 6, i32::from(next(5) == 0));
            }
            core.restart();
            let (mut pager, mut expected) = (Pager::default(), None);
            for _ in 0..4000 {
                core.tick();
                let playhead = &core.tracks[0];
                if let Some(ends) = expected {
                    assert_eq!((playhead.pulse, playhead.changed), ends, "{config:?}");
                }
                pager.follow(playhead);
                let (pulse, stage, _) = pager.ends(playhead, core.stage(0).unwrap());
                expected = Some((pulse, stage));
            }
        }
    }

    #[test]
    fn pages_reach_the_core_as_set_step_writes_them() {
        let mut seed = 0x2545_f491_u32;
        let mut core = Sequencer::default();
        for _ in 0..200 {
            let mut stage = Stage::default();
            for field in 0..15 {
                seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                stage.set(field, (seed >> 8) as f32 % 10_001.0 - 5000.0);
                core.set_step(0, 0, field as u32, stage.get(field) as i32);
            }
            assert_eq!(
                format!("{:?}", core_step(&stage)),
                format!("{:?}", core.tracks[0].steps[0])
            );
        }
    }

    #[test]
    fn adopted_lanes_keep_their_page_and_page_counts_relock_to_the_grid() {
        let tracks = [paged(3)];
        let mut r = Runtime::new(&tracks);
        let played = starts(&mut r, 20);
        assert_eq!(played.last().unwrap().0, 19);
        let mut next = Runtime::new(&tracks);
        next.inherit(&mut r);
        assert_eq!(
            steps(&starts(&mut next, 30)),
            (20..48).chain(0..2).collect::<Vec<_>>()
        );

        // Down to one page while the third plays: the replay puts it where
        // one page, played from the transport start, is now.
        let mut track = tracks[0].clone();
        let edit = Edit {
            track: 1,
            stage: None,
            field: 54,
            value: 1.0,
        };
        track.edit(edit);
        next.edit(0, &track, edit);
        let played = steps(&starts(&mut next, 40));
        assert!(played[20..].iter().all(|&step| step < 16), "{played:?}");
        let sixteenth = |r: &Runtime| ((r.core.ticks - 1) / 24 % 16) as u8;
        assert_eq!(next.view().tracks[0].step, sixteenth(&next));
    }
}
