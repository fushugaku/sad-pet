use serde::{Deserialize, Serialize};

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
            if byte >= 0xf0 {
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

#[derive(Clone, Debug, Serialize)]
pub struct Port {
    pub id: usize,
    pub name: String,
}

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
pub struct Connection {
    _input: midir::MidiInputConnection<Parser>,
    pub name: String,
}
#[cfg(not(target_arch = "wasm32"))]
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
        let connection = input
            .connect(
                port,
                "SAD PET input",
                move |_, bytes, parser| {
                    for &byte in bytes {
                        if let Some(message) = parser.feed(byte) {
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
        })
    }
}
