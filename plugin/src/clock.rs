//! MIDI clock in the app: the scene follows a master's tempo, song position
//! and transport from the app's MIDI inputs as the plug-in follows a DAW,
//! and sends its own clock, which the app delivers to a MIDI output.
use sad_pet::{
    clock::{Follower, Position, Sender, Transport},
    midi::Message,
};

/// One message of clock out at its frame in the block, as the app reads it.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClockEvent {
    pub offset: u32,
    pub size: u8,
    pub bytes: [u8; 3],
}

/// What the editor shows of the clock, from the audio side.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct View {
    /// A followed master sent a message recently.
    pub present: bool,
    pub running: bool,
    pub tempo: Option<f64>,
}

/// One block's clock out: 300 BPM ticks fill 22 of these in 8192 frames at
/// 44.1 kHz.
const EVENTS: usize = 128;

pub(crate) struct AppClock {
    follow: bool,
    follower: Follower,
    /// The master's tempo while the last block followed it.
    followed: Option<f64>,
    rate: f64,
    /// The block's first frame, and the frames rendered in it so far, which
    /// stamp the clock messages that arrive.
    frame: u64,
    cursor: usize,
    /// The scene's transport at `frame`, for clock out; the epoch counts
    /// the times it started, stopped or moved.
    beat: f64,
    bpm: f32,
    playing: bool,
    epoch: u32,
    sender: Sender,
    events: [ClockEvent; EVENTS],
    len: usize,
}
impl AppClock {
    pub fn new(rate: u32, follow: bool, lead_seconds: f64) -> Box<Self> {
        let mut clock = Box::new(Self {
            follow,
            follower: Follower::new(rate),
            followed: None,
            rate: rate as f64,
            frame: 0,
            cursor: 0,
            beat: 0.0,
            bpm: 120.0,
            playing: false,
            epoch: 0,
            sender: Sender::new(rate),
            events: [ClockEvent::default(); EVENTS],
            len: 0,
        });
        clock.follower.set_lead(lead_seconds);
        clock
    }
    pub fn configure(&mut self, follow: bool, lead_seconds: f64) {
        if follow != self.follow {
            self.follow = follow;
            self.follower.reset();
        }
        self.follower.set_lead(lead_seconds);
    }
    /// A message from the app's MIDI input: with Follow, the clock's go to
    /// the follower, stamped with the frame they arrived at, and the engine
    /// does not play them (returns true). Without, Start, Continue and Stop
    /// still play and stop the timbres.
    pub fn event(&mut self, message: Message) -> bool {
        if !self.follow
            || !matches!(
                message,
                Message::Clock
                    | Message::Start
                    | Message::Continue
                    | Message::Stop
                    | Message::SongPosition { .. }
            )
        {
            return false;
        }
        let frame = (self.frame + self.cursor as u64) as f64;
        self.follower.event(frame, message);
        true
    }
    pub fn rendered(&mut self, frames: usize) {
        self.cursor += frames;
    }
    /// Before a block: what a followed master plays from its first frame,
    /// and the tempo of a master that has just gone, to keep.
    pub fn begin(&mut self, frames: usize, fallback_bpm: f32) -> (Option<Position>, Option<f32>) {
        self.cursor = 0;
        let followed = if self.follow {
            self.follower.block(self.frame as f64, frames, fallback_bpm)
        } else {
            None
        };
        // Kept to a tenth of a beat per minute, as the browser does.
        let released = self
            .followed
            .filter(|_| followed.is_none())
            .map(|tempo| ((tempo * 10.0).round() / 10.0).clamp(20.0, 300.0) as f32);
        self.followed = followed.and(self.follower.tempo().or(Some(fallback_bpm as f64)));
        (followed, released)
    }
    /// The scene's transport at the block's first frame: the master's beat
    /// while followed, else its own count from where Play started it.
    pub fn transport(&mut self, playing: bool, bpm: f32, beat: Option<f64>, moved: bool) {
        match beat {
            Some(beat) => self.beat = beat,
            // Play starts the scene at its top.
            None if playing && !self.playing => self.beat = 0.0,
            None => {}
        }
        if moved || playing != self.playing {
            self.epoch = self.epoch.wrapping_add(1);
        }
        self.playing = playing;
        self.bpm = bpm;
    }
    /// After a block: its clock out, then the frame and beat the next starts at.
    pub fn end(&mut self, frames: usize) {
        let (events, len) = (&mut self.events, &mut self.len);
        *len = 0;
        let transport = Transport {
            beat: self.beat,
            bpm: self.bpm,
            playing: self.playing,
            epoch: self.epoch,
        };
        self.sender
            .block(self.frame as f64, frames, transport, |offset, bytes| {
                if *len < EVENTS {
                    let mut event = ClockEvent {
                        offset: offset as u32,
                        size: bytes.len() as u8,
                        bytes: [0; 3],
                    };
                    event.bytes[..bytes.len()].copy_from_slice(bytes);
                    events[*len] = event;
                    *len += 1;
                }
            });
        self.frame += frames as u64;
        if self.playing {
            self.beat += frames as f64 * self.bpm as f64 / (60.0 * self.rate);
        }
    }
    /// The clock out of the block that just ended.
    pub fn events(&self) -> &[ClockEvent] {
        &self.events[..self.len]
    }
    pub fn view(&self) -> View {
        let present = self.follow && self.follower.present(self.frame as f64);
        View {
            present,
            running: present && self.follower.running(),
            tempo: self.follower.tempo().filter(|_| present),
        }
    }
}
