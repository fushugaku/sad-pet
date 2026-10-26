//! MIDI clock in the browser: the scene follows a master's tempo, song
//! position and transport, and keeps the beat that clock out sends.
use super::*;
use sad_pet::clock::{Follower, Position};

pub(super) struct MidiClock {
    follow: bool,
    follower: Follower,
    /// The master's tempo while the last block followed it.
    followed: Option<f64>,
    rate: f64,
    /// Frames rendered so far: the first frame of the next block. Clock
    /// messages are stamped on this count.
    frame: u64,
    /// The scene's transport at `frame`, for clock out.
    beat: f64,
    bpm: f32,
    playing: bool,
    /// Counts the times the transport started, stopped or moved.
    epoch: u32,
    /// The state as numbers, read by the worklet without a JSON answer.
    view: [f64; VIEW],
}
/// frame, beat, bpm, playing, epoch, follow, present, running, the master's
/// tempo (0 while unknown), and the lead in seconds.
pub const VIEW: usize = 10;
impl MidiClock {
    pub fn new(rate: u32) -> Self {
        Self {
            follow: false,
            follower: Follower::new(rate),
            followed: None,
            rate: rate as f64,
            frame: 0,
            beat: 0.0,
            bpm: 120.0,
            playing: false,
            epoch: 0,
            view: [0.0; VIEW],
        }
    }
    pub fn epoch(&self) -> u32 {
        self.epoch
    }
    pub fn view(&mut self) -> *const f64 {
        let present = self.follow && self.follower.present(self.frame as f64);
        self.view = [
            self.frame as f64,
            self.beat,
            self.bpm as f64,
            self.playing as u8 as f64,
            self.epoch as f64,
            self.follow as u8 as f64,
            present as u8 as f64,
            (present && self.follower.running()) as u8 as f64,
            self.follower.tempo().filter(|_| present).unwrap_or(0.0),
            self.follower.lead(),
        ];
        self.view.as_ptr()
    }
    pub fn frame(&self) -> u64 {
        self.frame
    }
    /// After a block: the frame and beat the next one starts at.
    pub fn advance(&mut self, frames: usize) {
        self.frame += frames as u64;
        if self.playing {
            self.beat += frames as f64 * self.bpm as f64 / (60.0 * self.rate);
        }
    }
}

impl timbres::Registry {
    /// A clock message stamped at `frame`. Without Follow, Start, Continue
    /// and Stop still play and stop the timbres that take MIDI.
    pub fn clock_event(&mut self, frame: f64, message: Message) {
        if self.midi_clock.follow {
            self.midi_clock.follower.event(frame, message);
            return;
        }
        if !matches!(message, Message::Start | Message::Continue | Message::Stop) {
            return;
        }
        for part in &mut self.parts {
            if part.meta.kind.takes_midi() && part.meta.mix.accepts(message) {
                part.host.engine.midi(message);
            }
        }
    }
    /// `/api/clock`: sets Follow and the lead, and answers the clock's state
    /// with the scene's transport at the next block.
    pub fn clock_command(&mut self, data: &Value) -> Result<Value, String> {
        let clock = &mut self.midi_clock;
        if let Some(follow) = data["follow"].as_bool()
            && follow != clock.follow
        {
            clock.follow = follow;
            clock.follower.reset();
        }
        if let Some(ms) = data["lead_ms"].as_f64() {
            clock.follower.set_lead(ms / 1000.0);
        }
        let present = clock.follow && clock.follower.present(clock.frame as f64);
        Ok(json!({
            "follow": clock.follow,
            "lead_ms": clock.follower.lead() * 1000.0,
            "present": present,
            "running": present && clock.follower.running(),
            "tempo": clock.follower.tempo().filter(|_| present),
            "frame": clock.frame,
            "beat": clock.beat,
            "bpm": clock.bpm,
            "playing": clock.playing,
            "epoch": clock.epoch,
        }))
    }
    /// Before a block: with Follow, every timbre plays the master's tempo and
    /// beat as it would a DAW's; the scene's transport is noted for clock out.
    pub fn clock_block(&mut self, count: usize) {
        let fallback = self.parts[0].host.engine.preset.params.bpm;
        let clock = &mut self.midi_clock;
        let followed = if clock.follow {
            clock.follower.block(clock.frame as f64, count, fallback)
        } else {
            None
        };
        let released = clock.followed.filter(|_| followed.is_none());
        clock.followed = followed.and(clock.follower.tempo().or(Some(fallback as f64)));
        if let Some(tempo) = released {
            self.release_clock(tempo);
        }
        let (playing, bpm, moved) = match followed {
            Some(position) => {
                self.follow_clock(position);
                self.midi_clock.beat = position.beat;
                (position.playing, position.bpm, position.restart)
            }
            None => {
                let engine = &self.parts[0].host.engine;
                if engine.playing && !self.midi_clock.playing {
                    // Play starts the scene at its top.
                    self.midi_clock.beat = 0.0;
                }
                (engine.playing, engine.preset.params.bpm, false)
            }
        };
        let clock = &mut self.midi_clock;
        if moved || playing != clock.playing {
            clock.epoch = clock.epoch.wrapping_add(1);
        }
        clock.playing = playing;
        clock.bpm = bpm;
    }
    /// The master is gone or no longer followed: the timbres keep its tempo,
    /// to a tenth of a beat per minute, and the editor hears of it.
    fn release_clock(&mut self, tempo: f64) {
        let bpm = ((tempo * 10.0).round() / 10.0).clamp(20.0, 300.0) as f32;
        for part in &mut self.parts {
            let engine = &mut part.host.engine;
            engine.preset.params.bpm = bpm;
            engine.refresh_parameters();
            part.host.config_revision = part.host.config_revision.wrapping_add(1);
        }
    }
    fn follow_clock(&mut self, position: Position) {
        let lead_in = self.vocal_lead_in();
        for part in &mut self.parts {
            let engine = &mut part.host.engine;
            let started = position.playing && !engine.playing;
            if position.playing != engine.playing {
                engine.set_playing(position.playing);
            }
            let restart = position.restart || started;
            let sequencer = engine.preset.sequencer.enabled && restart;
            let before = engine.preset.params.bpm;
            engine.host_transport(position.bpm, Some(position.beat), restart, sequencer);
            if engine.preset.params.bpm != before {
                engine.refresh_parameters();
            }
        }
        if position.playing != self.clock.playing {
            self.clock.play(position.playing, lead_in);
        }
    }
}
