//! Fixed-capacity interaction snapshots. Every edge reads the previous frame.
//! Audio owners never resolve IDs, allocate, or recursively render a source.
use super::VOICES;
use crate::{
    Stereo,
    interaction::{self as model, Bridge, Event, Scope, SyncMode, Tap},
};

pub const BANDS: usize = crate::resynth::BANDS;
pub const GLOBAL: usize = VOICES + BANDS;
pub const LANES: usize = GLOBAL + 1;
const ABSENT: f32 = -1.0;
const EVENT_DELAY: usize = 64;
// Polyphonic sources have voice lanes, spectral sources have band lanes,
// and mono sources/groups use GLOBAL. Their unused cross-domain lanes never
// receive natural events, so they do not need 64-sample delay lines.
const POLY_DELAY_LINES: usize = 16 * (VOICES + 1);
const SPECTRAL_DELAY_LINES: usize = 4 * (BANDS + 1);
const DELAY_LINES: usize = POLY_DELAY_LINES + SPECTRAL_DELAY_LINES + model::NODES - 20;
fn delay_line(node: usize, lane: usize) -> Option<usize> {
    if node < 16 {
        let lane = if lane < VOICES {
            lane
        } else if lane == GLOBAL {
            VOICES
        } else {
            return None;
        };
        Some(node * (VOICES + 1) + lane)
    } else if node < 20 {
        (VOICES..=GLOBAL)
            .contains(&lane)
            .then(|| POLY_DELAY_LINES + (node - 16) * (BANDS + 1) + lane - VOICES)
    } else {
        (node < model::NODES && lane == GLOBAL)
            .then_some(POLY_DELAY_LINES + SPECTRAL_DELAY_LINES + node - 20)
    }
}
#[derive(Clone, Copy, Default)]
struct Scheduled {
    stamp: u64,
    time: f32,
    forced: bool,
}

#[derive(Clone, Copy)]
struct Frame {
    stamp: u64,
    generation: Stereo,
    output: Stereo,
    events: [f32; 4],
    output_cross: f32,
    prior: [f32; 2],
    mix_weight: f32,
    forced: u8,
}
impl Default for Frame {
    fn default() -> Self {
        Self {
            stamp: 0,
            generation: [0.0; 2],
            output: [0.0; 2],
            events: [ABSENT; 4],
            output_cross: ABSENT,
            prior: [0.0; 2],
            mix_weight: 0.0,
            forced: 0,
        }
    }
}
#[derive(Clone, Copy, Default)]
struct Detector {
    last: [f32; 2],
    armed: [bool; 2],
}

#[derive(Clone, Copy, Debug)]
pub struct PhaseCommand {
    pub mode: SyncMode,
    pub event: f64,
    pub phase: f64,
    pub strength: f64,
    pub response: f64,
}
impl Default for PhaseCommand {
    fn default() -> Self {
        Self {
            mode: SyncMode::Off,
            event: -1.0,
            phase: 0.0,
            strength: 0.0,
            response: 0.005,
        }
    }
}
impl PhaseCommand {
    pub fn event_in(self, sub: usize, factor: usize) -> Option<f64> {
        if self.mode == SyncMode::Off || self.event < 0.0 || self.strength == 0.0 {
            return None;
        }
        let at = self.event.clamp(0.0, 1.0 - f64::EPSILON) * factor as f64;
        (at.floor() as usize == sub).then(|| at.fract())
    }
}

/// Soft correction is a finite trajectory, never a restart of a voice.
#[derive(Clone, Copy, Debug, Default)]
pub struct PhaseSlew {
    remaining: f64,
    samples: u32,
}
impl PhaseSlew {
    pub fn is_active(&self) -> bool {
        self.samples != 0
    }
    pub fn correct(&mut self, phase: f64, command: PhaseCommand, rate: f64) {
        if command.mode == SyncMode::Hard && command.event >= 0.0 && command.strength > 0.0 {
            self.clear();
        }
        if command.mode == SyncMode::Soft && command.event >= 0.0 && command.strength > 0.0 {
            let error = (command.phase - phase + 0.5).rem_euclid(1.0) - 0.5;
            self.remaining = error * command.strength;
            self.samples = (command.response * rate).round().max(1.0) as u32;
        }
    }
    pub fn next(&mut self) -> f64 {
        if self.samples == 0 {
            return 0.0;
        }
        let step = self.remaining / self.samples as f64;
        self.remaining -= step;
        self.samples -= 1;
        step
    }
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Input {
    pub ring: f32,
    pub ring_start: f32,
    pub depth: f32,
    pub playback: PhaseCommand,
    pub clock: PhaseCommand,
}
impl Default for Input {
    fn default() -> Self {
        Self {
            ring: 0.0,
            ring_start: 0.0,
            depth: 0.0,
            playback: PhaseCommand::default(),
            clock: PhaseCommand::default(),
        }
    }
}
impl Input {
    pub fn factor(self) -> f32 {
        (1.0 - self.depth) + self.depth * self.ring
    }
    pub fn factor_at(self, sub: usize, factor: usize) -> f32 {
        let t = (sub + 1) as f32 / factor as f32;
        (1.0 - self.depth) + self.depth * (self.ring_start + (self.ring - self.ring_start) * t)
    }
    pub fn engaged(self) -> bool {
        self.depth > 0.0 || self.playback.mode != SyncMode::Off || self.clock.mode != SyncMode::Off
    }
}

/// The same fractional value/slope correction used by the WT readhead,
/// applied to PCM and captured windows without changing their source buffers.
pub fn read_phase(
    correction: &mut Stereo,
    position: f64,
    step: f64,
    reset: Option<(f64, f64)>,
    mut read: impl FnMut(f64) -> Stereo,
) -> (Stereo, f64) {
    let mut value = read(position);
    for c in 0..2 {
        value[c] += correction[c];
    }
    *correction = [0.0; 2];
    if let Some((at, phase)) = reset {
        let f = at.clamp(0.0, 1.0) as f32;
        let left = 1.0 - f;
        let before = read(position + step * at);
        let after = read(phase);
        let epsilon = step / 1024.0;
        let a = read(position + step * at - epsilon);
        let b = read(position + step * at + epsilon);
        let c = read(phase - epsilon);
        let d = read(phase + epsilon);
        for i in 0..2 {
            let jump = after[i] - before[i];
            let slope = ((d[i] - c[i]) - (b[i] - a[i])) * 512.0;
            value[i] += jump * 0.5 * left * left + slope * left * left * left / 6.0;
            correction[i] = -jump * 0.5 * f * f + slope * f * f * f / 6.0;
        }
        (value, phase + step * (1.0 - at))
    } else {
        (value, position + step)
    }
}

#[derive(Clone, Copy)]
struct Compiled {
    route: model::Route,
    source: usize,
    target: usize,
    depth: f32,
    counters: [u8; LANES],
    permitted: [bool; LANES],
}
impl Default for Compiled {
    fn default() -> Self {
        Self {
            route: Default::default(),
            source: 0,
            target: 0,
            depth: 0.0,
            counters: [0; LANES],
            permitted: [false; LANES],
        }
    }
}

pub struct Runtime {
    rate: f64,
    previous: Box<[[Frame; LANES]]>,
    current: Box<[[Frame; LANES]]>,
    detectors: Box<[[Detector; LANES]]>,
    scheduled: Box<[Scheduled]>,
    routes: [Compiled; model::MAX_ROUTES],
    count: usize,
    frame: u64,
    voice_generations: [u32; VOICES],
    pub demanded: u32,
    pub targets: u32,
    pub dropped_births: u64,
}
impl Runtime {
    pub fn new(rate: u32) -> Self {
        Self {
            rate: rate as f64,
            previous: vec![[Frame::default(); LANES]; model::NODES].into_boxed_slice(),
            current: vec![[Frame::default(); LANES]; model::NODES].into_boxed_slice(),
            detectors: vec![[Detector::default(); LANES]; model::NODES].into_boxed_slice(),
            scheduled: vec![Scheduled::default(); DELAY_LINES * EVENT_DELAY].into_boxed_slice(),
            routes: [Compiled::default(); model::MAX_ROUTES],
            count: 0,
            frame: 1,
            demanded: 0,
            targets: 0,
            dropped_births: 0,
            voice_generations: [u32::MAX; VOICES],
        }
    }
    pub fn active(&self) -> bool {
        self.count != 0
    }
    #[cfg(test)]
    pub fn configure(
        &mut self,
        settings: &model::Settings,
        oscs: Option<&crate::oscillators::Settings>,
    ) {
        self.configure_routes(&settings.routes, oscs, u32::MAX);
    }
    pub fn configure_routes(
        &mut self,
        routes: &[model::Route],
        oscs: Option<&crate::oscillators::Settings>,
        phase_capable: u32,
    ) {
        let mut count = 0;
        self.demanded = 0;
        self.targets = 0;
        for route in routes.iter().filter(|r| r.enabled).take(model::MAX_ROUTES) {
            let (Some(source), Some(target)) =
                (route.source.resolve(oscs), route.target.resolve(oscs))
            else {
                continue;
            };
            let mut effective = *route;
            if effective.event == Event::Auto {
                let single = if source < 16 {
                    matches!(route.bridge, Bridge::Voice { .. })
                        || (target < 16 && matches!(route.bridge, Bridge::Auto | Bridge::SameVoice))
                } else if source < 20 {
                    matches!(route.bridge, Bridge::Band { .. })
                        || ((16..20).contains(&target) && route.bridge == Bridge::Auto)
                } else {
                    source < 25
                };
                effective.event = if route.tap == Tap::Output {
                    Event::AudioZeroCross
                } else if (25..29).contains(&source) {
                    Event::GroupClockWrap
                } else if single && phase_capable & (1 << source) != 0 {
                    Event::NaturalWrap
                } else {
                    Event::AudioZeroCross
                };
            }
            let route = &effective;
            if route.ring == 0.0 && route.sync == SyncMode::Off {
                continue;
            }
            let old = self.routes[count];
            let same = old.source == source
                && old.target == target
                && old.route.source == route.source
                && old.route.target == route.target
                && old.route.event == route.event
                && old.route.bridge == route.bridge
                && old.route.tap == route.tap;
            self.routes[count] = Compiled {
                route: *route,
                source,
                target,
                depth: if same { old.depth } else { 0.0 },
                counters: if same { old.counters } else { [0; LANES] },
                permitted: [false; LANES],
            };
            self.demanded |= 1 << source;
            self.targets |= 1 << target;
            count += 1;
        }
        if count == 0 && self.count != 0 {
            self.clear();
        }
        self.count = count;
    }
    pub fn clear(&mut self) {
        for node in self.previous.iter_mut() {
            node.fill(Frame::default());
        }
        for node in self.current.iter_mut() {
            node.fill(Frame::default());
        }
        for node in self.detectors.iter_mut() {
            node.fill(Detector::default());
        }
        self.scheduled.fill(Scheduled::default());
        self.voice_generations.fill(u32::MAX);
        for r in &mut self.routes {
            r.counters.fill(0);
            r.permitted.fill(false);
        }
    }
    pub fn prepare_voice(&mut self, lane: usize, generation: u32) {
        if !self.active() || lane >= VOICES || self.voice_generations[lane] == generation {
            return;
        }
        self.voice_generations[lane] = generation;
        for node in 0..model::NODES {
            self.previous[node][lane] = Frame::default();
            self.current[node][lane] = Frame::default();
            self.detectors[node][lane] = Detector::default();
            if let Some(line) = delay_line(node, lane) {
                let start = line * EVENT_DELAY;
                self.scheduled[start..start + EVENT_DELAY].fill(Scheduled::default());
            }
        }
    }
    pub fn invalidate_node(&mut self, node: usize) {
        self.previous[node].fill(Frame::default());
        self.current[node].fill(Frame::default());
        self.detectors[node].fill(Detector::default());
        for lane in 0..LANES {
            if let Some(line) = delay_line(node, lane) {
                let start = line * EVENT_DELAY;
                self.scheduled[start..start + EVENT_DELAY].fill(Scheduled::default());
            }
        }
        for r in self.routes.iter_mut().take(self.count) {
            if r.source == node || r.target == node {
                r.counters.fill(0);
            }
        }
    }
    pub fn begin_frame(&mut self) {
        if !self.active() {
            return;
        }
        std::mem::swap(&mut self.previous, &mut self.current);
        self.frame = self.frame.wrapping_add(1).max(2);
        for node in 0..model::NODES {
            if !self.needed(node) {
                continue;
            }
            for lane in 0..LANES {
                let Some(line) = delay_line(node, lane) else {
                    continue;
                };
                let index = line * EVENT_DELAY + self.frame as usize % EVENT_DELAY;
                let event = self.scheduled[index];
                if event.stamp == self.frame {
                    self.event(
                        node,
                        lane,
                        Event::NaturalWrap,
                        event.time as f64,
                        event.forced,
                    );
                    self.scheduled[index] = Scheduled::default();
                }
            }
        }
        let step = (1.0 / (self.rate * 0.005)) as f32;
        for route in self.routes.iter_mut().take(self.count) {
            route.depth += (route.route.ring - route.depth).clamp(-step, step);
            for lane in 0..LANES {
                let frame = self.previous[route.source][lane];
                let event = if route.route.event == Event::AudioZeroCross
                    && route.route.tap == Tap::Output
                {
                    frame.output_cross
                } else {
                    frame.events[route.route.event.index()]
                };
                let forced = frame.forced & (1 << route.route.event.index()) != 0;
                route.permitted[lane] = false;
                if frame.stamp + 1 == self.frame
                    && event >= 0.0
                    && (!forced || route.route.include_forced)
                {
                    route.counters[lane] += 1;
                    if route.counters[lane] >= route.route.divide {
                        route.counters[lane] = 0;
                        route.permitted[lane] = true;
                    }
                }
            }
        }
    }
    pub fn needed(&self, node: usize) -> bool {
        node < model::NODES && (self.demanded | self.targets) & (1 << node) != 0
    }
    fn source_lane(route: &Compiled, target_lane: usize) -> usize {
        match route.route.bridge {
            Bridge::SameVoice => {
                if route.source < 16 && target_lane < VOICES {
                    target_lane
                } else {
                    GLOBAL
                }
            }
            Bridge::Mix => GLOBAL,
            Bridge::Voice { index } => {
                if route.source < 16 {
                    usize::from(index).min(VOICES - 1)
                } else {
                    GLOBAL
                }
            }
            Bridge::Band { index } => {
                if (16..20).contains(&route.source) {
                    VOICES + usize::from(index).min(BANDS - 1)
                } else {
                    GLOBAL
                }
            }
            Bridge::Auto => {
                if (route.source < 16 && target_lane < VOICES)
                    || ((16..20).contains(&route.source) && (VOICES..GLOBAL).contains(&target_lane))
                {
                    target_lane
                } else {
                    GLOBAL
                }
            }
        }
    }
    pub fn input(&self, node: usize, lane: usize) -> Input {
        let mut input = Input::default();
        if !self.active() {
            return input;
        }
        for route in self
            .routes
            .iter()
            .take(self.count)
            .filter(|r| r.target == node)
        {
            let source_lane = Self::source_lane(route, lane);
            let frame = self.previous[route.source][source_lane];
            let live = frame.stamp + 1 == self.frame;
            if route.depth > 0.0 {
                let audio = if route.route.tap == Tap::Generation {
                    frame.generation
                } else {
                    frame.output
                };
                input.ring = if live {
                    ((audio[0] + audio[1]) * 0.5).clamp(-2.0, 2.0)
                } else {
                    0.0
                };
                input.ring_start = if live {
                    frame.prior[usize::from(route.route.tap == Tap::Output)].clamp(-2.0, 2.0)
                } else {
                    0.0
                };
                input.depth = route.depth;
            }
            if route.route.sync != SyncMode::Off {
                let event = if live && route.permitted[source_lane] {
                    if route.route.event == Event::AudioZeroCross && route.route.tap == Tap::Output
                    {
                        frame.output_cross as f64
                    } else {
                        frame.events[route.route.event.index()] as f64
                    }
                } else {
                    -1.0
                };
                let command = PhaseCommand {
                    mode: route.route.sync,
                    event,
                    phase: route.route.reset_phase as f64,
                    strength: route.route.strength as f64,
                    response: route.route.response as f64 * 0.001,
                };
                if route.route.scope != Scope::Clock {
                    input.playback = command;
                }
                if route.route.scope != Scope::Playback {
                    input.clock = command;
                }
            }
        }
        input
    }
    pub fn publish(
        &mut self,
        node: usize,
        lane: usize,
        generation: Stereo,
        output: Stereo,
        weight: f32,
    ) {
        if !self.active() || node >= model::NODES || lane >= LANES || !self.needed(node) {
            return;
        }
        let at = &mut self.current[node][lane];
        if at.stamp != self.frame {
            *at = Frame {
                stamp: self.frame,
                ..Default::default()
            };
        }
        at.generation = generation;
        at.output = output;
        if lane != GLOBAL {
            let all = &mut self.current[node][GLOBAL];
            if all.stamp != self.frame {
                *all = Frame {
                    stamp: self.frame,
                    ..Default::default()
                };
            }
            for c in 0..2 {
                all.generation[c] += generation[c] * weight;
                all.output[c] += output[c] * weight;
            }
            all.mix_weight += weight.abs();
        }
    }
    pub fn event(&mut self, node: usize, lane: usize, kind: Event, time: f64, forced: bool) {
        if !self.active() || !self.needed(node) {
            return;
        }
        let at = &mut self.current[node][lane];
        if at.stamp != self.frame {
            *at = Frame {
                stamp: self.frame,
                ..Default::default()
            };
        }
        let i = kind.index();
        let time = time.clamp(0.0, 1.0) as f32;
        if at.events[i] < 0.0 || time < at.events[i] {
            at.events[i] = time;
            if forced {
                at.forced |= 1 << i;
            } else {
                at.forced &= !(1 << i);
            }
        }
    }
    /// Align a raw phase event with a carrier's linear-phase tap reduction.
    /// AudioZeroCross is detected on the reduced audio and is never delayed twice.
    pub fn natural_event(&mut self, node: usize, lane: usize, time: f64, latency: f64) {
        if latency <= 0.0 {
            self.event(node, lane, Event::NaturalWrap, time, false);
            return;
        }
        if !self.active() || !self.needed(node) {
            return;
        }
        let time = time + latency;
        let whole = time.floor() as u64;
        let stamp = self.frame + whole.min((EVENT_DELAY - 1) as u64);
        if stamp == self.frame {
            self.event(node, lane, Event::NaturalWrap, time.fract(), false);
            return;
        }
        let Some(line) = delay_line(node, lane) else {
            return;
        };
        let index = line * EVENT_DELAY + stamp as usize % EVENT_DELAY;
        let event = &mut self.scheduled[index];
        let fraction = time.fract() as f32;
        if event.stamp != stamp || fraction < event.time {
            *event = Scheduled {
                stamp,
                time: fraction,
                forced: false,
            };
        }
    }
    pub fn finish_frame(&mut self) {
        if !self.active() {
            return;
        }
        for node in 0..model::NODES {
            if !self.needed(node) {
                continue;
            }
            for lane in 0..LANES {
                let frame = &mut self.current[node][lane];
                if lane == GLOBAL && frame.mix_weight > 1.0 {
                    for c in 0..2 {
                        frame.generation[c] /= frame.mix_weight;
                        frame.output[c] /= frame.mix_weight;
                    }
                }
                let d = &mut self.detectors[node][lane];
                frame.prior = d.last;
                let values = if frame.stamp == self.frame {
                    [
                        (frame.generation[0] + frame.generation[1]) * 0.5,
                        (frame.output[0] + frame.output[1]) * 0.5,
                    ]
                } else {
                    [0.0; 2]
                };
                // Both stages keep detector history; the route chooses the stage.
                for (i, x) in values.into_iter().enumerate() {
                    if x < -1e-6 {
                        d.armed[i] = true;
                    }
                    if d.armed[i] && x > 1e-6 {
                        let time = if x != d.last[i] {
                            (-d.last[i] / (x - d.last[i])).clamp(0.0, 1.0)
                        } else {
                            0.0
                        };
                        if i == 0 {
                            frame.events[Event::AudioZeroCross.index()] = time;
                        } else {
                            frame.output_cross = time;
                        }
                        d.armed[i] = false;
                    }
                    d.last[i] = x;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use model::{Node, Route, Settings};
    #[test]
    fn compact_delay_lines_keep_voice_band_and_mono_events_independent() {
        let mut used = vec![false; DELAY_LINES];
        for node in 0..model::NODES {
            for lane in 0..LANES {
                if let Some(line) = delay_line(node, lane) {
                    assert!(!used[line], "delay-line alias at {node}/{lane}");
                    used[line] = true;
                }
            }
        }
        assert!(used.into_iter().all(|v| v));
        assert!(DELAY_LINES * EVENT_DELAY * std::mem::size_of::<Scheduled>() <= 640 * 1024);
        let mut r = Runtime::new(48000);
        r.configure(
            &Settings {
                routes: [
                    (Node::Synth { index: 0 }, Node::Synth { index: 1 }),
                    (Node::Resynth { index: 0 }, Node::Resynth { index: 1 }),
                    (Node::Tb303Vco, Node::Group { index: 0 }),
                ]
                .map(|(source, target)| Route {
                    source,
                    target,
                    event: Event::NaturalWrap,
                    sync: SyncMode::Hard,
                    ..Route::default()
                })
                .to_vec(),
                ..Settings::default()
            },
            None,
        );
        let lanes = [
            (8, 0, 9),
            (8, 1, 9),
            (16, VOICES, 17),
            (16, VOICES + 1, 17),
            (24, GLOBAL, 25),
        ];
        r.begin_frame();
        for (i, &(source, lane, _)) in lanes.iter().enumerate() {
            r.natural_event(source, lane, 0.05 + i as f64 * 0.1, 39.125);
        }
        r.finish_frame();
        let mut seen: [Vec<f64>; 5] = Default::default();
        for _ in 0..44 {
            r.begin_frame();
            for (i, &(_, lane, target)) in lanes.iter().enumerate() {
                let event = r.input(target, lane).playback.event;
                if event >= 0.0 {
                    seen[i].push(event);
                }
            }
            r.finish_frame();
        }
        for (i, events) in seen.iter().enumerate() {
            assert_eq!(events.len(), 1, "lane {i}");
            assert!((events[0] - (0.175 + i as f64 * 0.1)).abs() < 1e-6);
        }
    }
    #[test]
    fn directed_edges_and_reciprocal_ring_read_the_same_previous_snapshot() {
        let mut r = Runtime::new(48000);
        let settings = Settings {
            routes: vec![
                Route {
                    source: Node::Group { index: 0 },
                    target: Node::Group { index: 1 },
                    ring: 1.0,
                    ..Default::default()
                },
                Route {
                    source: Node::Group { index: 1 },
                    target: Node::Group { index: 0 },
                    ring: 1.0,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        r.configure(&settings, None);
        for _ in 0..300 {
            r.begin_frame();
            let a = r.input(25, GLOBAL);
            let b = r.input(26, GLOBAL);
            r.publish(26, GLOBAL, [0.25; 2], [0.25 * b.factor(); 2], 1.0);
            assert_eq!(r.input(25, GLOBAL).ring, a.ring);
            r.publish(25, GLOBAL, [0.5; 2], [0.5 * a.factor(); 2], 1.0);
            r.finish_frame();
        }
        r.begin_frame();
        assert_eq!(r.input(25, GLOBAL).ring, 0.25);
        assert_eq!(r.input(26, GLOBAL).ring, 0.5);
        assert_eq!(r.input(25, GLOBAL).depth, 1.0);
    }
    #[test]
    fn forced_events_are_not_natural_wraps_and_soft_is_a_finite_trajectory() {
        let mut r = Runtime::new(48000);
        let settings = Settings {
            routes: vec![Route {
                source: Node::Group { index: 0 },
                target: Node::Group { index: 1 },
                sync: SyncMode::Hard,
                event: Event::GrainLaunch,
                ..Default::default()
            }],
            ..Default::default()
        };
        r.configure(&settings, None);
        r.begin_frame();
        r.publish(25, GLOBAL, [1.0; 2], [1.0; 2], 1.0);
        r.event(25, GLOBAL, Event::GrainLaunch, 0.25, true);
        r.finish_frame();
        r.begin_frame();
        assert_eq!(r.input(26, GLOBAL).playback.event, -1.0);
        r.publish(25, GLOBAL, [1.0; 2], [1.0; 2], 1.0);
        r.event(25, GLOBAL, Event::GrainLaunch, 0.5, false);
        r.finish_frame();
        r.begin_frame();
        assert_eq!(r.input(26, GLOBAL).playback.event, 0.5);
        let mut slew = PhaseSlew::default();
        slew.correct(
            0.25,
            PhaseCommand {
                mode: SyncMode::Soft,
                event: 0.0,
                strength: 1.0,
                response: 0.001,
                ..Default::default()
            },
            48000.0,
        );
        let first = slew.next();
        assert!(first.abs() < 0.01);
        let total = first + (0..47).map(|_| slew.next()).sum::<f64>();
        assert!((total + 0.25).abs() < 1e-12);
        assert_eq!(slew.next(), 0.0);
    }
    #[test]
    fn delayed_natural_events_keep_fractional_carry_and_origin_is_atomic() {
        let mut r = Runtime::new(48000);
        r.configure(
            &Settings {
                routes: vec![Route {
                    source: Node::Group { index: 0 },
                    target: Node::Group { index: 1 },
                    sync: SyncMode::Hard,
                    event: Event::NaturalWrap,
                    ..Default::default()
                }],
                ..Default::default()
            },
            None,
        );
        r.begin_frame();
        r.natural_event(25, GLOBAL, 0.9, 39.125);
        r.finish_frame();
        for _ in 0..40 {
            r.begin_frame();
            assert_eq!(r.input(26, GLOBAL).playback.event, -1.0);
            r.finish_frame();
        }
        r.begin_frame();
        assert!((r.input(26, GLOBAL).playback.event - 0.025).abs() < 1e-6);
        r.event(25, GLOBAL, Event::NaturalWrap, 0.2, false);
        r.event(25, GLOBAL, Event::NaturalWrap, 0.3, true);
        assert_eq!(r.current[25][GLOBAL].forced, 0);
    }
}
