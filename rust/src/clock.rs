//! MIDI beat clock: following another instrument's tempo, song position and
//! transport (24 clocks per quarter note, Start, Continue, Stop and the Song
//! Position Pointer) the way a plug-in follows its DAW.
//!
//! A host stamps each message with the frame of its own sample clock at which
//! the master's beat sounds, and asks [`Follower::block`] what to play at each
//! block. A delay-locked loop filters the ticks, and the engine's beat is
//! pulled onto the clock with small tempo changes, so jitter on a network or
//! Bluetooth link never makes the playhead jump.
use crate::midi::Message;

/// Clocks per quarter note.
pub const TICKS_PER_BEAT: f64 = 24.0;
const MIN_BPM: f64 = 20.0;
const MAX_BPM: f64 = 300.0;
/// A tick further than this share of a tick from where the loop expected it
/// means the tempo jumped: the loop starts again from the raw ticks.
const RELOCK: f64 = 0.5;
/// Loop bandwidth per tick: wide while it locks, narrow once it has.
const LOCKING: f64 = 0.35;
const LOCKED: f64 = 0.05;
const LOCKING_TICKS: u32 = 48;
/// A locked loop takes a gap of one or two ticks for ticks lost on the link,
/// though not more than twice in a row (that is a slower tempo).
const MISSED_TICKS: f64 = 2.0;
const MISSED_IN_A_ROW: u32 = 2;
/// The engine is pulled onto the clock over this time, at most `SLEW` away
/// from the clock's tempo; further off than `JUMP_SECONDS` it moves at once.
const PULL_SECONDS: f64 = 0.5;
const SLEW: f64 = 0.02;
const JUMP_SECONDS: f64 = 0.04;
/// The engine's tempo changes only by more than this, so that effects synced
/// to it are not retuned every block; the pull keeps the beat on the clock.
const HYSTERESIS_BPM: f64 = 0.05;
/// Without any message for this long, the master is gone.
const TIMEOUT_SECONDS: f64 = 0.5;
/// The tempo to show is measured over this many beats of filtered ticks.
const SHOWN_BEATS: usize = 16;

/// What the engine plays from a block's first frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub bpm: f32,
    /// Quarter notes since the song's start.
    pub beat: f64,
    pub playing: bool,
    /// The beat moved (Start, Continue, a song position, or too far off to
    /// pull): parts that read the beat seek to it.
    pub restart: bool,
}

#[derive(Clone, Debug)]
pub struct Follower {
    rate: f64,
    /// Frames to play ahead of the clock, for a link with a known delay.
    lead: f64,
    /// Frames per tick as the loop has it; 0 before two ticks.
    period: f64,
    /// Where the loop expects the next tick.
    expected: Option<f64>,
    /// The last tick as it arrived.
    last_tick: Option<f64>,
    /// Ticks since the loop last locked.
    ticks: u32,
    missed_in_a_row: u32,
    /// The loop's tick at each of the last beats since it locked, oldest
    /// first, for a tempo steadier than one tick's period.
    beats: [f64; SHOWN_BEATS + 1],
    beat_count: usize,
    /// The last message of any kind.
    heard: Option<f64>,
    running: bool,
    /// The beat the next tick sounds while running; where Continue resumes.
    next_beat: f64,
    /// A frame and the beat it sounds: the latest counted tick, or Start.
    anchor: (f64, f64),
    /// Start, Continue or a song position moved the beat; the engine has not.
    moved: bool,
    /// The engine's beat at the next block, its tempo and whether it plays.
    beat: f64,
    bpm: f32,
    playing: bool,
}

impl Follower {
    pub fn new(rate: u32) -> Self {
        Self {
            rate: rate.max(1) as f64,
            lead: 0.0,
            period: 0.0,
            expected: None,
            last_tick: None,
            ticks: 0,
            missed_in_a_row: 0,
            beats: [0.0; SHOWN_BEATS + 1],
            beat_count: 0,
            heard: None,
            running: false,
            next_beat: 0.0,
            anchor: (0.0, 0.0),
            moved: false,
            beat: 0.0,
            bpm: 0.0,
            playing: false,
        }
    }
    /// Forgets the master, keeping the lead.
    pub fn reset(&mut self) {
        *self = Self {
            lead: self.lead,
            ..Self::new(self.rate as u32)
        };
    }
    /// Plays this many seconds ahead of the clock (behind when negative).
    pub fn set_lead(&mut self, seconds: f64) {
        self.lead = if seconds.is_finite() {
            seconds.clamp(-1.0, 1.0) * self.rate
        } else {
            0.0
        };
    }
    pub fn lead(&self) -> f64 {
        self.lead / self.rate
    }
    /// The master's tempo, once two ticks have arrived.
    pub fn bpm(&self) -> Option<f64> {
        (self.period > 0.0).then(|| 60.0 * self.rate / (self.period * TICKS_PER_BEAT))
    }
    /// The master's tempo measured over the last beats, steadier than
    /// [`Self::bpm`] when the link jitters: the tempo to show.
    pub fn tempo(&self) -> Option<f64> {
        if self.beat_count < 3 {
            return self.bpm();
        }
        let span = self.beats[self.beat_count - 1] - self.beats[0];
        (span > 0.0).then(|| 60.0 * self.rate * (self.beat_count - 1) as f64 / span)
    }
    /// The master plays (after Start or Continue, until Stop).
    pub fn running(&self) -> bool {
        self.running
    }
    /// A message arrived recently enough to follow.
    pub fn present(&self, frame: f64) -> bool {
        self.heard
            .is_some_and(|heard| frame - heard <= self.timeout())
    }
    fn timeout(&self) -> f64 {
        (TIMEOUT_SECONDS * self.rate).max(6.0 * self.period)
    }
    fn periods(&self) -> std::ops::RangeInclusive<f64> {
        let period = |bpm: f64| 60.0 * self.rate / (bpm * TICKS_PER_BEAT);
        period(MAX_BPM)..=period(MIN_BPM)
    }

    /// Takes a clock message sounding at `frame`; other messages are ignored.
    /// Returns whether the message belonged to the clock.
    pub fn event(&mut self, frame: f64, message: Message) -> bool {
        if !frame.is_finite() {
            return false;
        }
        match message {
            Message::Clock => self.tick(frame),
            Message::Start => {
                self.next_beat = 0.0;
                self.start(frame);
            }
            Message::Continue => self.start(frame),
            Message::Stop => {
                self.running = false;
                self.moved = false;
            }
            Message::SongPosition { sixteenths } => {
                self.next_beat = sixteenths as f64 / 4.0;
                if self.running {
                    self.start(frame);
                }
            }
            _ => return false,
        }
        self.heard = Some(frame);
        true
    }
    /// The first tick after Start or Continue sounds `next_beat`. A master
    /// restarts its ticks there, so the loop keeps only the period.
    fn start(&mut self, frame: f64) {
        self.running = true;
        self.moved = true;
        self.anchor = (frame, self.next_beat);
        self.expected = None;
        self.last_tick = None;
    }
    fn tick(&mut self, frame: f64) {
        let periods = self.periods();
        let mut counted = 1.0;
        let tick = match (self.expected, self.last_tick) {
            (Some(mut expected), _) => {
                let mut error = frame - expected;
                // A locked loop counts a gap of one or two ticks as lost ones.
                let missed = (error / self.period).round();
                if self.ticks >= LOCKING_TICKS
                    && (1.0..=MISSED_TICKS).contains(&missed)
                    && (error - missed * self.period).abs() <= 0.25 * self.period
                    && self.missed_in_a_row < MISSED_IN_A_ROW
                {
                    self.missed_in_a_row += 1;
                    self.beat_count = 0;
                    counted += missed;
                    expected += missed * self.period;
                    error = frame - expected;
                } else {
                    self.missed_in_a_row = 0;
                }
                if error.abs() <= RELOCK * self.period {
                    let width = if self.ticks < LOCKING_TICKS {
                        LOCKING
                    } else {
                        LOCKED
                    };
                    self.period = (self.period + width * width * error)
                        .clamp(*periods.start(), *periods.end());
                    self.ticks = self.ticks.saturating_add(1);
                    let tick = expected + std::f64::consts::SQRT_2 * width * error;
                    if self.ticks.is_multiple_of(TICKS_PER_BEAT as u32) {
                        if self.beat_count == self.beats.len() {
                            self.beats.copy_within(1.., 0);
                            self.beat_count -= 1;
                        }
                        self.beats[self.beat_count] = tick;
                        self.beat_count += 1;
                    }
                    tick
                } else {
                    self.relock(frame)
                }
            }
            (None, _) => self.relock(frame),
        };
        self.last_tick = Some(frame);
        self.expected = (self.period > 0.0).then_some(tick + self.period);
        if self.running {
            self.next_beat += (counted - 1.0) / TICKS_PER_BEAT;
            self.anchor = (tick, self.next_beat);
            self.next_beat += 1.0 / TICKS_PER_BEAT;
        }
    }
    /// Starts the loop again from the interval since the last tick, or keeps
    /// the old period across a gap too long to measure one.
    fn relock(&mut self, frame: f64) -> f64 {
        self.ticks = 0;
        self.missed_in_a_row = 0;
        self.beat_count = 0;
        if let Some(interval) = self.last_tick.map(|last| frame - last)
            && self.periods().contains(&interval)
        {
            self.period = interval;
        }
        frame
    }

    /// The tempo and beat to play from `frame`, the block's first frame;
    /// None while no master is present. `fallback_bpm` plays until two ticks
    /// give the master's tempo.
    pub fn block(&mut self, frame: f64, frames: usize, fallback_bpm: f32) -> Option<Position> {
        if !self.present(frame) {
            self.running = false;
            self.moved = false;
            self.playing = false;
            return None;
        }
        let clock_bpm = self
            .bpm()
            .unwrap_or(fallback_bpm as f64)
            .clamp(MIN_BPM, MAX_BPM);
        let mut bpm = clock_bpm;
        let mut restart = false;
        let at = frame + self.lead;
        if !self.running {
            self.playing = false;
        } else if self.moved && at < self.anchor.0 {
            // Start sounds later than this block begins.
            self.playing = false;
        } else {
            let beats_per_frame = clock_bpm / (60.0 * self.rate);
            let target = self.anchor.1 + (at - self.anchor.0) * beats_per_frame;
            let error = target - self.beat;
            if self.moved || !self.playing || error.abs() > JUMP_SECONDS * clock_bpm / 60.0 {
                self.beat = target.max(0.0);
                self.playing = true;
                self.moved = false;
                restart = true;
            } else {
                let pull = error / PULL_SECONDS / (clock_bpm / 60.0);
                bpm = clock_bpm * (1.0 + pull.clamp(-SLEW, SLEW));
            }
        }
        let bpm = bpm.clamp(MIN_BPM, MAX_BPM);
        if restart || (bpm - self.bpm as f64).abs() > HYSTERESIS_BPM {
            self.bpm = bpm as f32;
        }
        let bpm = self.bpm;
        let position = Position {
            bpm,
            beat: self.beat,
            playing: self.playing,
            restart,
        };
        if self.playing {
            // The engine advances on the same single-precision tempo.
            self.beat += frames as f64 * bpm as f64 / (60.0 * self.rate);
        }
        Some(position)
    }
}

const TICK: u8 = 0xf8;
const START: u8 = 0xfa;
const CONTINUE: u8 = 0xfb;
const STOP: u8 = 0xfc;
const SONG_POSITION: u8 = 0xf2;

/// The scene's transport at a block's first frame, for clock out; `epoch`
/// changes whenever it starts, stops or moves.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transport {
    pub beat: f64,
    pub bpm: f32,
    pub playing: bool,
    pub epoch: u32,
}

/// Clock out: the scene's transport as the messages a slave follows, at the
/// frames of a block they sound on. While it plays, a tick every 24th of a
/// beat after Start, or after a song position and Continue; Stop when it
/// stops. Ticks carry on at the tempo while stopped, so a slave keeps it.
#[derive(Clone, Debug)]
pub struct Sender {
    rate: f64,
    /// The transport's epoch at the last block; None before the first.
    epoch: Option<u32>,
    playing: bool,
    /// The next tick: its beat while playing, its frame while stopped.
    beat: f64,
    frame: f64,
    /// The sixteenth the transport starts on, announced with the next tick.
    announce: Option<u16>,
    /// The frame of the last tick sent: the next comes a tick after it.
    last: f64,
}
impl Sender {
    pub fn new(rate: u32) -> Self {
        Self {
            rate: rate.max(1) as f64,
            epoch: None,
            playing: false,
            beat: 0.0,
            frame: 0.0,
            announce: None,
            last: f64::NEG_INFINITY,
        }
    }
    /// Forgets the transport: the next block starts the clock afresh.
    pub fn reset(&mut self) {
        *self = Self::new(self.rate as u32);
    }
    /// The messages of the block that starts at `frame` and lasts `frames`,
    /// with the transport there. Each message goes to `send` with its offset
    /// in the block, in order.
    pub fn block(
        &mut self,
        frame: f64,
        frames: usize,
        transport: Transport,
        mut send: impl FnMut(usize, &[u8]),
    ) {
        let Transport {
            beat,
            bpm,
            playing,
            epoch,
        } = transport;
        if frames == 0 {
            return;
        }
        let frames_per_beat = 60.0 * self.rate / (bpm as f64).clamp(MIN_BPM, MAX_BPM);
        let period = frames_per_beat / TICKS_PER_BEAT;
        let frame_of = |b: f64| frame + (b - beat) * frames_per_beat;
        let offset = |at: f64| ((at - frame).round().max(0.0) as usize).min(frames - 1);
        if self.epoch != Some(epoch) {
            if self.playing {
                send(0, &[STOP]);
            }
            if playing {
                // A slave counts every tick after Start or Continue, so the
                // song resumes on the first sixteenth after what went out.
                let mut sixteenth = (beat * 4.0 - 1e-6).ceil().max(0.0);
                while frame_of(sixteenth / 4.0) <= self.last {
                    sixteenth += 1.0;
                }
                self.beat = sixteenth / 4.0;
                self.announce = Some(sixteenth.min(16_383.0) as u16);
            } else if self.epoch.is_none() || self.playing {
                self.frame = frame.max(self.last + period);
            }
            self.epoch = Some(epoch);
            self.playing = playing;
        }
        let end = frame + frames as f64;
        loop {
            let at = if self.playing {
                frame_of(self.beat)
            } else {
                self.frame
            };
            if at >= end - 0.5 {
                break;
            }
            let offset = offset(at);
            if let Some(sixteenth) = self.announce.take() {
                if sixteenth == 0 {
                    send(offset, &[START]);
                } else {
                    let low = (sixteenth & 0x7f) as u8;
                    send(offset, &[SONG_POSITION, low, (sixteenth >> 7) as u8]);
                    send(offset, &[CONTINUE]);
                }
            }
            send(offset, &[TICK]);
            self.last = at;
            if self.playing {
                self.beat += 1.0 / TICKS_PER_BEAT;
            } else {
                self.frame = at.max(frame) + period;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: u32 = 48_000;
    const BLOCK: usize = 128;

    /// A master at `bpm` whose ticks reach the follower up to `jitter / 2`
    /// frames early or late, deterministically.
    struct Master {
        bpm: f64,
        beat: f64,
        frame: f64,
        jitter: f64,
        seed: u64,
    }
    impl Master {
        fn new(bpm: f64, jitter: f64) -> Self {
            Self {
                bpm,
                beat: 0.0,
                frame: 0.0,
                jitter,
                seed: 0x2545_f491_4f6c_dd1d,
            }
        }
        fn period(&self) -> f64 {
            60.0 * RATE as f64 / (self.bpm * TICKS_PER_BEAT)
        }
        fn noise(&mut self) -> f64 {
            self.seed ^= self.seed << 13;
            self.seed ^= self.seed >> 7;
            self.seed ^= self.seed << 17;
            ((self.seed >> 11) as f64 / (1u64 << 53) as f64 - 0.5) * self.jitter
        }
        /// Sends the ticks that sound before `frame`.
        fn run(&mut self, follower: &mut Follower, until: f64) {
            while self.frame < until {
                let late = self.noise();
                follower.event(self.frame + late, Message::Clock);
                self.frame += self.period();
                self.beat += 1.0 / TICKS_PER_BEAT;
            }
        }
    }

    /// Runs blocks from `start` until `end`, ticking the master in between;
    /// returns the last position, the engine's beat error at each block and
    /// how often its tempo changed.
    fn play(
        follower: &mut Follower,
        master: &mut Master,
        start: f64,
        end: f64,
    ) -> (Position, Vec<f64>, usize) {
        let mut frame = start;
        let mut errors = Vec::new();
        let mut last: Option<Position> = None;
        let mut changes = 0;
        let running_from = master.frame - master.beat * master.period() * TICKS_PER_BEAT;
        while frame < end {
            // Ticks up to and including the block's first frame.
            master.run(follower, frame + 1.0);
            let position = follower.block(frame, BLOCK, 120.0).expect("a master");
            if position.playing {
                let expected = (frame - running_from) / (master.period() * TICKS_PER_BEAT);
                errors.push(position.beat - expected);
            }
            changes += last.is_some_and(|l| l.bpm != position.bpm) as usize;
            last = Some(position);
            frame += BLOCK as f64;
        }
        (last.unwrap(), errors, changes)
    }

    fn seconds(frames: f64) -> f64 {
        frames * RATE as f64
    }

    #[test]
    fn parser_reads_song_position_and_keeps_running_status_apart() {
        let mut parser = crate::midi::Parser::default();
        let bytes = [0x90, 60, 100, 0xf2, 0x10, 0x02, 62, 100, 0xf8];
        let messages: Vec<_> = bytes.iter().filter_map(|&b| parser.feed(b)).collect();
        assert_eq!(
            messages,
            [
                Message::NoteOn {
                    channel: 0,
                    note: 60,
                    velocity: 100
                },
                Message::SongPosition {
                    sixteenths: 2 << 7 | 0x10
                },
                Message::Clock,
            ],
            "Song Position ends running status: 62 100 alone is no note"
        );
    }

    #[test]
    fn follows_tempo_and_beat_through_jitter() {
        let mut follower = Follower::new(RATE);
        assert_eq!(follower.block(0.0, BLOCK, 120.0), None, "no master yet");
        let mut master = Master::new(128.0, 0.003 * RATE as f64);
        // The master's clock runs while it is stopped: tempo only.
        let (stopped, ..) = play(&mut follower, &mut master, 0.0, seconds(2.0));
        assert!(!stopped.playing);
        assert!((stopped.bpm - 128.0).abs() < 0.2, "{}", stopped.bpm);
        // Start: the next tick is beat 0.
        follower.event(master.frame, Message::Start);
        master.beat = 0.0;
        let begin = master.frame;
        let first = follower.block(master.frame.ceil(), BLOCK, 120.0).unwrap();
        assert!(
            first.playing && first.restart && first.beat < 0.01,
            "{first:?}"
        );
        let (last, errors, changes) = play(
            &mut follower,
            &mut master,
            begin + BLOCK as f64,
            begin + seconds(20.0),
        );
        assert!(last.playing && !last.restart);
        let settled = &errors[errors.len() / 4..];
        let worst = settled.iter().fold(0.0f64, |m, e| m.max(e.abs()));
        // 3 ms of jitter; the engine stays within a millisecond of the master.
        assert!(worst * 60.0 / 128.0 < 0.001, "worst {worst} beats");
        assert!(
            (follower.bpm().unwrap() - 128.0).abs() < 0.2,
            "{:?}",
            follower.bpm()
        );
        assert!(
            (follower.tempo().unwrap() - 128.0).abs() < 0.02,
            "{:?}",
            follower.tempo()
        );
        assert!((last.bpm - 128.0).abs() < 0.2, "{}", last.bpm);
        // The engine's tempo moves now and then, not every block.
        assert!(changes < errors.len() / 10, "{changes} of {}", errors.len());
    }

    #[test]
    fn pulls_small_offsets_and_jumps_large_ones() {
        let mut follower = Follower::new(RATE);
        let mut master = Master::new(120.0, 0.0);
        play(&mut follower, &mut master, 0.0, seconds(1.0));
        follower.event(master.frame, Message::Start);
        master.beat = 0.0;
        let begin = master.frame;
        play(&mut follower, &mut master, begin, begin + seconds(1.0));
        // Playing 20 ms behind: the engine slides back by small tempo
        // changes, never seeking.
        follower.set_lead(-0.02);
        let mut frame = begin + seconds(1.0);
        let mut restarts = 0;
        let mut slowest = 120.0f32;
        while frame < begin + seconds(4.0) {
            master.run(&mut follower, frame);
            let p = follower.block(frame, BLOCK, 120.0).unwrap();
            restarts += p.restart as u32;
            slowest = slowest.min(p.bpm);
            frame += BLOCK as f64;
        }
        assert_eq!(restarts, 0);
        assert!(slowest < 119.0, "{slowest}");
        assert!(slowest >= 120.0 * (1.0 - SLEW as f32) - 0.01, "{slowest}");
        // 80 ms ahead is too far to pull: the engine seeks.
        follower.set_lead(0.08);
        master.run(&mut follower, frame);
        let p = follower.block(frame, BLOCK, 120.0).unwrap();
        assert!(p.restart, "{p:?}");
        assert!(
            (p.beat - ((frame - begin) / seconds(0.5) + 0.16)).abs() < 0.002,
            "{p:?}"
        );
    }

    #[test]
    fn continue_resumes_at_the_song_position_and_stop_holds() {
        let mut follower = Follower::new(RATE);
        let mut master = Master::new(100.0, 0.0);
        play(&mut follower, &mut master, 0.0, seconds(1.0));
        follower.event(master.frame, Message::SongPosition { sixteenths: 32 });
        follower.event(master.frame, Message::Continue);
        let at = master.frame.ceil();
        let p = follower.block(at, BLOCK, 120.0).unwrap();
        assert!(p.playing && p.restart);
        assert!((p.beat - 8.0).abs() < 0.01, "{p:?}");
        master.run(&mut follower, at + seconds(1.0));
        let p = follower.block(at + seconds(1.0), BLOCK, 120.0).unwrap();
        // 100 BPM for a second is 1⅔ beats past bar 3.
        assert!((p.beat - (8.0 + 100.0 / 60.0)).abs() < 0.01, "{p:?}");
        follower.event(master.frame, Message::Stop);
        let stopped = follower
            .block(at + seconds(1.0) + BLOCK as f64, BLOCK, 120.0)
            .unwrap();
        assert!(!stopped.playing && !stopped.restart);
        assert!(!follower.running());
    }

    #[test]
    fn lost_ticks_keep_the_count_and_silence_lets_go() {
        let mut follower = Follower::new(RATE);
        let mut master = Master::new(120.0, 0.0);
        play(&mut follower, &mut master, 0.0, seconds(1.0));
        follower.event(master.frame, Message::Start);
        master.beat = 0.0;
        let begin = master.frame;
        play(&mut follower, &mut master, begin, begin + seconds(3.0));
        // One tick never arrives.
        master.frame += master.period();
        master.beat += 1.0 / TICKS_PER_BEAT;
        let (p, errors, _) = play(
            &mut follower,
            &mut master,
            begin + seconds(3.0),
            begin + seconds(4.0),
        );
        assert!(p.playing && !p.restart);
        assert!(errors.last().unwrap().abs() < 0.002, "{:?}", errors.last());
        // The master goes away without Stop.
        let quiet = master.frame + seconds(TIMEOUT_SECONDS) + BLOCK as f64;
        assert_eq!(follower.block(quiet, BLOCK, 120.0), None);
        assert!(!follower.running());
    }

    #[test]
    fn a_tempo_change_relocks_and_a_lead_plays_ahead() {
        let mut follower = Follower::new(RATE);
        follower.set_lead(0.01);
        let mut master = Master::new(90.0, 0.0);
        play(&mut follower, &mut master, 0.0, seconds(1.0));
        follower.event(master.frame, Message::Start);
        master.beat = 0.0;
        let begin = master.frame;
        let (_, errors, _) = play(&mut follower, &mut master, begin, begin + seconds(2.0));
        let lead = 0.01 * 90.0 / 60.0;
        assert!(
            (errors.last().unwrap() - lead).abs() < 0.001,
            "{:?}",
            errors.last()
        );
        master.bpm = 140.0;
        let mut frame = begin + seconds(2.0);
        let mut p = None;
        while frame < begin + seconds(4.0) {
            master.run(&mut follower, frame);
            p = follower.block(frame, BLOCK, 120.0);
            frame += BLOCK as f64;
        }
        assert!((p.unwrap().bpm - 140.0).abs() < 0.2, "{p:?}");
    }

    /// Every message a Sender gives over `blocks` blocks of BLOCK frames,
    /// as (frame, bytes), with the transport `at` each block's first frame.
    fn sent(
        sender: &mut Sender,
        start: u64,
        blocks: u64,
        at: impl Fn(u64) -> (f64, f32, bool, u32),
    ) -> Vec<(u64, Vec<u8>)> {
        let mut out = Vec::new();
        for block in start..start + blocks {
            let frame = block * BLOCK as u64;
            let (beat, bpm, playing, epoch) = at(frame);
            let transport = Transport {
                beat,
                bpm,
                playing,
                epoch,
            };
            sender.block(frame as f64, BLOCK, transport, |offset, bytes| {
                out.push((frame + offset as u64, bytes.to_vec()))
            });
        }
        out
    }

    #[test]
    fn the_sender_ticks_24_times_a_beat_from_start_and_stops() {
        let mut sender = Sender::new(RATE);
        // 120 BPM from beat 0: a tick every 1000 frames, Start before the first.
        let beat = |frame: u64| frame as f64 / 24_000.0;
        let out = sent(&mut sender, 0, 375, |f| (beat(f), 120.0, true, 1));
        assert_eq!(out[0], (0, vec![START]));
        let ticks: Vec<u64> = out.iter().filter(|m| m.1 == [TICK]).map(|m| m.0).collect();
        assert_eq!(ticks.len(), 48);
        assert!(ticks.iter().enumerate().all(|(i, &f)| f == i as u64 * 1000));
        // Stopped: Stop at once, then ticks go on at the tempo.
        let out = sent(&mut sender, 375, 100, |_| (2.0, 120.0, false, 2));
        assert_eq!(out[0], (48_000, vec![STOP]));
        let ticks: Vec<u64> = out.iter().filter(|m| m.1 == [TICK]).map(|m| m.0).collect();
        assert_eq!(ticks[0], 48_000);
        assert!(ticks.windows(2).all(|w| w[1] - w[0] == 1000));
        // Continuing at beat 2.3 resumes on the next sixteenth, 2.5, with its
        // song position (10 sixteenths) and Continue.
        let from = 475 * BLOCK as u64;
        let at = |f: u64| 2.3 + (f - from) as f64 / 24_000.0;
        let out = sent(&mut sender, 475, 50, |f| (at(f), 120.0, true, 3));
        let resume = from + 4_800;
        assert_eq!(out[0], (resume, vec![SONG_POSITION, 10, 0]));
        assert_eq!(out[1], (resume, vec![CONTINUE]));
        assert_eq!(out[2], (resume, vec![TICK]));
    }

    #[test]
    fn a_follower_locks_onto_a_senders_clock() {
        let mut sender = Sender::new(RATE);
        let mut follower = Follower::new(RATE);
        let beat = |frame: u64| 1.0 + frame as f64 * 97.0 / (60.0 * RATE as f64);
        let mut position = None;
        for block in 0..3_000_u64 {
            let frame = block * BLOCK as u64;
            let transport = Transport {
                beat: beat(frame),
                bpm: 97.0,
                playing: true,
                epoch: 1,
            };
            sender.block(frame as f64, BLOCK, transport, |offset, bytes| {
                let mut parser = crate::midi::Parser::default();
                for &byte in bytes {
                    if let Some(message) = parser.feed(byte) {
                        follower.event((frame + offset as u64) as f64, message);
                    }
                }
            });
            position = follower.block(frame as f64, BLOCK, 120.0);
        }
        let position = position.unwrap();
        assert!(position.playing);
        assert!((position.bpm - 97.0).abs() < 0.1, "{}", position.bpm);
        let end = 3_000 * BLOCK as u64;
        assert!(
            (position.beat - beat(end)).abs() < 0.01,
            "{} vs {}",
            position.beat,
            beat(end)
        );
    }
}
