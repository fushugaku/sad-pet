use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Message {
    NoteOn {
        channel: u8,
        note: u8,
        velocity: u8,
    },
    NoteOff {
        channel: u8,
        note: u8,
        velocity: u8,
    },
    Control {
        channel: u8,
        controller: u8,
        value: u8,
    },
    PitchBend {
        channel: u8,
        value: u16,
    },
    Pressure {
        channel: u8,
        value: u8,
    },
    PolyPressure {
        channel: u8,
        note: u8,
        value: u8,
    },
    Program {
        channel: u8,
        value: u8,
    },
    Clock,
    Start,
    Continue,
    Stop,
    Reset,
    /// Song Position Pointer: where Continue resumes, in sixteenth notes.
    SongPosition {
        sixteenths: u16,
    },
}

/// Byte stream parser corresponding to the framing seen at c00361ac.
/// Realtime messages may occur between the two data bytes of a channel message.
#[derive(Default, Debug)]
pub struct Parser {
    status: u8,
    count: usize,
    data: [u8; 2],
    sysex: bool,
}
impl Parser {
    pub fn feed(&mut self, byte: u8) -> Option<Message> {
        if byte >= 0xf8 {
            return match byte {
                0xf8 => Some(Message::Clock),
                0xfa => Some(Message::Start),
                0xfb => Some(Message::Continue),
                0xfc => Some(Message::Stop),
                0xff => Some(Message::Reset),
                _ => None,
            };
        }
        if byte & 0x80 != 0 {
            self.count = 0;
            if byte == 0xf2 {
                self.status = byte;
                self.sysex = false;
            } else if byte >= 0xf0 {
                self.status = 0;
                self.sysex = byte == 0xf0;
            } else {
                self.status = byte;
                self.sysex = false;
            }
            return None;
        }
        if self.sysex || self.status == 0 {
            return None;
        }
        self.data[self.count] = byte;
        self.count += 1;
        let kind = self.status >> 4;
        let length = if kind == 0xc || kind == 0xd { 1 } else { 2 };
        if self.count < length {
            return None;
        }
        self.count = 0;
        let channel = self.status & 0xf;
        let a = self.data[0];
        let b = self.data[1];
        if self.status == 0xf2 {
            // System common messages have no running status.
            self.status = 0;
            return Some(Message::SongPosition {
                sixteenths: (b as u16) << 7 | a as u16,
            });
        }
        Some(match kind {
            8 => Message::NoteOff {
                channel,
                note: a,
                velocity: b,
            },
            9 => {
                if b == 0 {
                    Message::NoteOff {
                        channel,
                        note: a,
                        velocity: 0,
                    }
                } else {
                    Message::NoteOn {
                        channel,
                        note: a,
                        velocity: b,
                    }
                }
            }
            0xa => Message::PolyPressure {
                channel,
                note: a,
                value: b,
            },
            0xb => Message::Control {
                channel,
                controller: a,
                value: b,
            },
            0xc => Message::Program { channel, value: a },
            0xd => Message::Pressure { channel, value: a },
            0xe => Message::PitchBend {
                channel,
                value: (b as u16) << 7 | a as u16,
            },
            _ => return None,
        })
    }
}

const CONTROLLERS: usize = 16 * 128;

/// The latest value of every controller on every channel, for the editor's
/// MIDI learn. The audio or MIDI thread records without waiting or
/// allocating; the editor's state poll collects the controllers that moved.
pub struct Controls {
    latest: Box<[AtomicU32]>,
    seen: Box<[AtomicU32]>,
}
impl Default for Controls {
    fn default() -> Self {
        let slots = || (0..CONTROLLERS).map(|_| AtomicU32::new(0)).collect();
        Self {
            latest: slots(),
            seen: slots(),
        }
    }
}
impl Controls {
    /// A slot keeps a write count above the 7-bit value, so a controller
    /// moved away and back between two polls still reads as moved.
    pub fn record(&self, message: Message) {
        if let Message::Control {
            channel,
            controller,
            value,
        } = message
            && channel < 16
            && controller < 128
        {
            let slot = &self.latest[channel as usize * 128 + controller as usize];
            let count = (slot.load(Relaxed) >> 7).wrapping_add(1);
            slot.store(count << 7 | value.min(127) as u32, Relaxed);
        }
    }
    /// [channel, controller, value] of each controller moved since the last call.
    pub fn changes(&self) -> Vec<[u8; 3]> {
        let mut changes = Vec::new();
        for (i, (latest, seen)) in self.latest.iter().zip(&self.seen).enumerate() {
            let word = latest.load(Relaxed);
            if seen.swap(word, Relaxed) != word {
                changes.push([(i / 128) as u8, (i % 128) as u8, (word & 127) as u8]);
            }
        }
        changes
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Port {
    pub id: usize,
    pub name: String,
}

#[cfg(all(feature = "desktop", not(target_arch = "wasm32")))]
pub fn ports() -> Result<Vec<Port>, String> {
    let input = midir::MidiInput::new("SAD PET ports").map_err(|e| e.to_string())?;
    input
        .ports()
        .iter()
        .enumerate()
        .map(|(id, port)| {
            input
                .port_name(port)
                .map(|name| Port { id, name })
                .map_err(|e| e.to_string())
        })
        .collect()
}

#[cfg(all(feature = "desktop", not(target_arch = "wasm32")))]
pub struct Connection {
    _input: midir::MidiInputConnection<Parser>,
    pub name: String,
    /// Controllers moved on this input, for the editor's MIDI learn.
    pub controls: std::sync::Arc<Controls>,
}
#[cfg(all(feature = "desktop", not(target_arch = "wasm32")))]
impl Connection {
    pub fn open(
        id: usize,
        commands: std::sync::mpsc::SyncSender<crate::audio::Command>,
    ) -> Result<Self, String> {
        let mut input = midir::MidiInput::new("SAD PET").map_err(|e| e.to_string())?;
        input.ignore(midir::Ignore::None);
        let ports = input.ports();
        let port = ports.get(id).ok_or("MIDI port is no longer available")?;
        let name = input.port_name(port).map_err(|e| e.to_string())?;
        let controls = std::sync::Arc::new(Controls::default());
        let moved = controls.clone();
        let connection = input
            .connect(
                port,
                "SAD PET input",
                move |_, bytes, parser| {
                    for &byte in bytes {
                        if let Some(message) = parser.feed(byte) {
                            moved.record(message);
                            let _ = commands.try_send(crate::audio::Command::Midi(message));
                        }
                    }
                },
                Parser::default(),
            )
            .map_err(|e| e.to_string())?;
        Ok(Self {
            _input: connection,
            name,
            controls,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn control(channel: u8, controller: u8, value: u8) -> Message {
        Message::Control {
            channel,
            controller,
            value,
        }
    }

    #[test]
    fn controls_report_each_moved_controller_once_with_its_latest_value() {
        let controls = Controls::default();
        assert!(controls.changes().is_empty());
        controls.record(control(0, 74, 10));
        controls.record(control(0, 74, 90));
        controls.record(control(15, 1, 127));
        controls.record(Message::NoteOn {
            channel: 0,
            note: 74,
            velocity: 100,
        });
        assert_eq!(controls.changes(), [[0, 74, 90], [15, 1, 127]]);
        assert!(controls.changes().is_empty());
        // Away and back between two polls is still a move.
        controls.record(control(0, 74, 20));
        controls.record(control(0, 74, 90));
        assert_eq!(controls.changes(), [[0, 74, 90]]);
        controls.record(control(16, 1, 1));
        controls.record(control(0, 128, 1));
        assert!(controls.changes().is_empty());
    }
}
