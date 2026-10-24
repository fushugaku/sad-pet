//! Static description of every model and its user parameters, shared with the UI.
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Dynamics,
    Filter,
    Eq,
    Drive,
    Modulation,
    Pitch,
    Synth,
    Delay,
    Reverb,
    Utility,
}

impl Category {
    pub const ALL: [Self; 10] = [
        Self::Dynamics,
        Self::Filter,
        Self::Eq,
        Self::Drive,
        Self::Modulation,
        Self::Pitch,
        Self::Synth,
        Self::Delay,
        Self::Reverb,
        Self::Utility,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Dynamics => "Dynamics",
            Self::Filter => "Filter",
            Self::Eq => "EQ",
            Self::Drive => "Drive",
            Self::Modulation => "Modulation",
            Self::Pitch => "Pitch",
            Self::Synth => "Synth",
            Self::Delay => "Delay",
            Self::Reverb => "Reverb",
            Self::Utility => "Utility",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ModelInfo {
    /// Stable identifier, also the SAD PET slot kind: `mooz_<name>`.
    pub key: &'static str,
    /// The original MS-70CDR effect name.
    pub name: &'static str,
    pub category: Category,
    /// The source module in the 2.10 file system.
    pub zdl: &'static str,
    /// One line for menus and tooltips.
    pub description: &'static str,
    pub params: &'static [ParamInfo],
}

#[derive(Debug, Serialize)]
pub struct ParamInfo {
    /// The original label, as the pedal shows it.
    pub name: &'static str,
    /// Values run over `0..=max` pedal units.
    pub max: u16,
    pub default: u16,
    pub display: Display,
}

/// How the UI prints a value `v` (pedal units, rounded to an integer).
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Display {
    /// `v * scale + offset`, printed with `decimals` and followed by `unit`.
    Number {
        offset: f32,
        scale: f32,
        decimals: u8,
        unit: &'static str,
    },
    /// One label per value.
    Labels { labels: &'static [&'static str] },
    /// `0..=threshold` is `v + offset` ms (most models show `v + 1`); larger
    /// values are the tempo notes [`crate::zoom::DELAY_NOTE_LABELS`]`[v - threshold - 1]`.
    DelayTime { threshold: u16, offset: i16 },
    /// `0..=threshold` prints like `Number`; larger values are the tempo notes
    /// [`crate::zoom::RATE_NOTE_LABELS`]`[v - threshold - 1]`.
    SyncRate {
        threshold: u16,
        offset: f32,
        scale: f32,
        decimals: u8,
        unit: &'static str,
    },
}

impl Display {
    /// The plain integer the pedal shows.
    pub const INT: Self = Self::number(0.0, 1.0, 0, "");
    /// Off / On switch.
    pub const OFF_ON: Self = Self::Labels {
        labels: &["Off", "On"],
    };

    pub const fn number(offset: f32, scale: f32, decimals: u8, unit: &'static str) -> Self {
        Self::Number {
            offset,
            scale,
            decimals,
            unit,
        }
    }

    /// Integer shown with an offset, e.g. a 0..29 Decay shown as 1..30.
    pub const fn offset(offset: f32) -> Self {
        Self::number(offset, 1.0, 0, "")
    }

    pub const fn labels(labels: &'static [&'static str]) -> Self {
        Self::Labels { labels }
    }

    pub const fn sync_rate(threshold: u16, offset: f32) -> Self {
        Self::SyncRate {
            threshold,
            offset,
            scale: 1.0,
            decimals: 0,
            unit: "",
        }
    }
}
