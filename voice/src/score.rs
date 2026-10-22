//! The score a Vocal timbre sings: notes with lyrics, a tempo map, parameter
//! curves with VOCALOID5's ranges, and a style. Positions are in ticks of a
//! quarter note, so the score follows tempo changes of the host.
use crate::phoneme::Language;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const TICKS_PER_BEAT: u32 = 480;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Param {
    Dynamics,
    Breathiness,
    Brightness,
    Clearness,
    Character,
    Exciter,
    Growl,
    Air,
    Mouth,
    PortamentoTiming,
    PitchBend,
    PitchBendSensitivity,
}
impl Param {
    pub const ALL: [Self; 12] = [
        Self::Dynamics,
        Self::Breathiness,
        Self::Brightness,
        Self::Clearness,
        Self::Character,
        Self::Exciter,
        Self::Growl,
        Self::Air,
        Self::Mouth,
        Self::PortamentoTiming,
        Self::PitchBend,
        Self::PitchBendSensitivity,
    ];
    /// (minimum, maximum, default) in VOCALOID5's units.
    pub fn range(self) -> (f32, f32, f32) {
        match self {
            Self::Dynamics => (0.0, 127.0, 64.0),
            Self::Breathiness => (0.0, 127.0, 0.0),
            Self::Brightness => (0.0, 127.0, 64.0),
            Self::Clearness => (0.0, 127.0, 0.0),
            Self::Character => (-64.0, 63.0, 0.0),
            Self::Exciter => (-64.0, 63.0, 0.0),
            Self::Growl => (0.0, 127.0, 0.0),
            Self::Air => (0.0, 127.0, 0.0),
            Self::Mouth => (-127.0, 0.0, 0.0),
            Self::PortamentoTiming => (0.0, 127.0, 64.0),
            Self::PitchBend => (-8192.0, 8191.0, 0.0),
            Self::PitchBendSensitivity => (0.0, 24.0, 2.0),
        }
    }
    pub fn key(self) -> &'static str {
        match self {
            Self::Dynamics => "dynamics",
            Self::Breathiness => "breathiness",
            Self::Brightness => "brightness",
            Self::Clearness => "clearness",
            Self::Character => "character",
            Self::Exciter => "exciter",
            Self::Growl => "growl",
            Self::Air => "air",
            Self::Mouth => "mouth",
            Self::PortamentoTiming => "portamento_timing",
            Self::PitchBend => "pitch_bend",
            Self::PitchBendSensitivity => "pitch_bend_sensitivity",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub tick: u32,
    pub value: f32,
}

/// A curve is linear between points and holds its ends; an empty curve is
/// the parameter's default.
pub type Curve = Vec<Point>;

pub fn curve_value(curve: &[Point], tick: f32, default: f32) -> f32 {
    match curve.binary_search_by(|p| (p.tick as f32).total_cmp(&tick)) {
        Ok(i) => curve[i].value,
        Err(0) => curve.first().map_or(default, |p| p.value),
        Err(i) if i >= curve.len() => curve[curve.len() - 1].value,
        Err(i) => {
            let (a, b) = (curve[i - 1], curve[i]);
            let t = (tick - a.tick as f32) / (b.tick - a.tick).max(1) as f32;
            a.value + (b.value - a.value) * t
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Attack {
    #[default]
    None,
    /// Rises into the note from below.
    Scoop,
    /// Falls into the note from above.
    Bend,
    /// Fades in.
    Soft,
    /// Louder and brighter start.
    Accent,
    /// Starts on breath.
    Breathy,
    /// A quick turn below the note (kobushi).
    Turn,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Release {
    #[default]
    None,
    /// Drops in pitch.
    Fall,
    /// Rises in pitch.
    Rise,
    /// Fades out.
    Soft,
    /// Ends on breath.
    Breathy,
    /// Vibrato that widens towards the end.
    Swell,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Effect<K> {
    pub kind: K,
    /// 0–1.
    pub amount: f32,
}

/// Per-note vibrato; values left at `None` come from the style.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Vibrato {
    /// Peak deviation in cents.
    pub depth: Option<f32>,
    pub rate: Option<f32>,
    /// Seconds from the note start.
    pub delay: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Note {
    pub id: u32,
    pub start: u32,
    pub length: u32,
    /// MIDI note number.
    pub key: u8,
    pub lyric: String,
    /// Space-separated phonemes that replace the lyric.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonemes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    /// VOCALOID's VEL: higher is a shorter, sharper consonant.
    pub velocity: u8,
    pub vibrato: Vibrato,
    pub attack: Effect<Attack>,
    pub release: Effect<Release>,
}
impl Default for Note {
    fn default() -> Self {
        Self {
            id: 0,
            start: 0,
            length: TICKS_PER_BEAT,
            key: 60,
            lyric: String::new(),
            phonemes: None,
            language: None,
            velocity: 64,
            vibrato: Vibrato::default(),
            attack: Effect::default(),
            release: Effect::default(),
        }
    }
}
impl Note {
    pub fn end(&self) -> u32 {
        self.start + self.length
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tempo {
    pub tick: u32,
    pub bpm: f32,
}

/// VOCALOID5's Style: a singing skill (how the pitch moves) plus voice-color
/// offsets added to the curves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Style {
    pub name: String,
    /// Seconds a note-to-note pitch transition takes.
    pub transition: f32,
    /// 0–1: how far the pitch overshoots a new note.
    pub overshoot: f32,
    /// 0–1: the small dip against the direction before a transition.
    pub preparation: f32,
    /// Cents below the note that a phrase starts from.
    pub scoop: f32,
    /// Cents the pitch drops at the end of a phrase.
    pub fall: f32,
    pub vibrato_depth: f32,
    pub vibrato_rate: f32,
    pub vibrato_delay: f32,
    /// Notes shorter than this (seconds) get no vibrato.
    pub vibrato_min_length: f32,
    /// Slow pitch wander in cents.
    pub drift: f32,
    /// Steps between notes with no fluctuation.
    pub robot: bool,
    /// Breath before phrases that follow a rest.
    pub auto_breath: bool,
    pub breath_level: f32,
    /// Offsets added to the curves, in the curves' units.
    pub color: BTreeMap<Param, f32>,
}
impl Default for Style {
    fn default() -> Self {
        Self {
            name: "Natural".into(),
            transition: 0.09,
            overshoot: 0.35,
            preparation: 0.2,
            scoop: 40.0,
            fall: 30.0,
            vibrato_depth: 35.0,
            vibrato_rate: 5.6,
            vibrato_delay: 0.35,
            vibrato_min_length: 0.45,
            drift: 8.0,
            robot: false,
            auto_breath: true,
            breath_level: 0.5,
            color: BTreeMap::new(),
        }
    }
}
impl Style {
    pub fn presets() -> Vec<Self> {
        let base = Self::default();
        let color = |pairs: &[(Param, f32)]| pairs.iter().copied().collect::<BTreeMap<_, _>>();
        vec![
            base.clone(),
            Self {
                name: "Pop".into(),
                transition: 0.07,
                overshoot: 0.45,
                scoop: 60.0,
                vibrato_depth: 25.0,
                vibrato_rate: 6.0,
                vibrato_delay: 0.4,
                color: color(&[(Param::Brightness, 10.0), (Param::Clearness, 15.0)]),
                ..base.clone()
            },
            Self {
                name: "Ballad".into(),
                transition: 0.13,
                overshoot: 0.2,
                preparation: 0.3,
                scoop: 70.0,
                fall: 60.0,
                vibrato_depth: 45.0,
                vibrato_rate: 5.2,
                vibrato_delay: 0.3,
                color: color(&[(Param::Breathiness, 25.0), (Param::Brightness, -8.0)]),
                ..base.clone()
            },
            Self {
                name: "Rock".into(),
                transition: 0.06,
                overshoot: 0.6,
                scoop: 90.0,
                fall: 80.0,
                vibrato_depth: 30.0,
                vibrato_rate: 6.2,
                color: color(&[
                    (Param::Growl, 35.0),
                    (Param::Exciter, 20.0),
                    (Param::Dynamics, 10.0),
                ]),
                ..base.clone()
            },
            Self {
                name: "Whisper".into(),
                vibrato_depth: 10.0,
                color: color(&[
                    (Param::Breathiness, 110.0),
                    (Param::Air, 60.0),
                    (Param::Dynamics, -10.0),
                ]),
                ..base.clone()
            },
            Self {
                name: "Opera".into(),
                transition: 0.12,
                overshoot: 0.25,
                vibrato_depth: 70.0,
                vibrato_rate: 5.4,
                vibrato_delay: 0.15,
                vibrato_min_length: 0.3,
                color: color(&[
                    (Param::Clearness, 40.0),
                    (Param::Mouth, 0.0),
                    (Param::Character, -10.0),
                ]),
                ..base.clone()
            },
            Self {
                name: "Robot".into(),
                transition: 0.02,
                overshoot: 0.0,
                preparation: 0.0,
                scoop: 0.0,
                fall: 0.0,
                vibrato_depth: 0.0,
                drift: 0.0,
                robot: true,
                auto_breath: false,
                ..base
            },
        ]
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Score {
    pub notes: Vec<Note>,
    pub tempo: Vec<Tempo>,
    /// Use the host's tempo instead of `tempo` (plug-ins).
    pub follow_host: bool,
    pub language: Language,
    /// Sung on notes without a lyric.
    pub default_lyric: String,
    pub curves: BTreeMap<Param, Curve>,
    pub style: Style,
    /// Voicebank id; empty is the built-in voice.
    pub voice: String,
    /// Semitones added to every note.
    pub transpose: i8,
    /// Length in ticks after which the score starts over; 0 sings it once.
    pub loop_ticks: u32,
}
impl Default for Score {
    fn default() -> Self {
        Self {
            notes: Vec::new(),
            tempo: vec![Tempo {
                tick: 0,
                bpm: 120.0,
            }],
            follow_host: true,
            language: Language::Ja,
            default_lyric: String::new(),
            curves: BTreeMap::new(),
            style: Style::default(),
            voice: String::new(),
            transpose: 0,
            loop_ticks: 0,
        }
    }
}

impl Score {
    pub fn default_lyric(&self) -> &str {
        if !self.default_lyric.trim().is_empty() {
            return &self.default_lyric;
        }
        match self.language {
            Language::Ja => "ら",
            Language::En => "la",
            Language::Ru => "ла",
        }
    }

    /// Keeps the score playable: sorted notes, unique ids, sane values.
    pub fn sanitize(&mut self) {
        self.notes.retain(|n| n.length > 0);
        self.notes.sort_by_key(|n| (n.start, n.key));
        let mut next = self.notes.iter().map(|n| n.id).max().unwrap_or(0) + 1;
        let mut seen = std::collections::HashSet::new();
        for note in &mut self.notes {
            if !seen.insert(note.id) || note.id == 0 {
                note.id = next;
                seen.insert(next);
                next += 1;
            }
            note.key = note.key.min(127);
            note.velocity = note.velocity.min(127);
            note.attack.amount = note.attack.amount.clamp(0.0, 1.0);
            note.release.amount = note.release.amount.clamp(0.0, 1.0);
            note.lyric = note
                .lyric
                .chars()
                .filter(|c| !c.is_control())
                .take(64)
                .collect();
        }
        self.loop_ticks = self.loop_ticks.min(TICKS_PER_BEAT * 4 * 1024);
        self.tempo.retain(|t| t.bpm.is_finite() && t.bpm > 0.0);
        self.tempo.sort_by_key(|t| t.tick);
        self.tempo.dedup_by_key(|t| t.tick);
        if self.tempo.first().is_none_or(|t| t.tick != 0) {
            let bpm = self.tempo.first().map_or(120.0, |t| t.bpm);
            self.tempo.insert(0, Tempo { tick: 0, bpm });
        }
        for t in &mut self.tempo {
            t.bpm = t.bpm.clamp(10.0, 999.0);
        }
        for (param, curve) in &mut self.curves {
            let (lo, hi, _) = param.range();
            curve.retain(|p| p.value.is_finite());
            curve.sort_by_key(|p| p.tick);
            curve.dedup_by_key(|p| p.tick);
            for p in curve.iter_mut() {
                p.value = p.value.clamp(lo, hi);
            }
        }
        self.curves.retain(|_, c| !c.is_empty());
    }

    /// What one pass of a looping score sings: the notes that start before
    /// the loop's end, cut at it. A score that does not loop stays whole.
    pub fn one_pass(&self) -> std::borrow::Cow<'_, Score> {
        let end = self.loop_ticks;
        if end == 0 || self.notes.iter().all(|n| n.start + n.length <= end) {
            return std::borrow::Cow::Borrowed(self);
        }
        let mut score = self.clone();
        score.notes.retain(|n| n.start < end);
        for n in &mut score.notes {
            n.length = n.length.min(end - n.start);
        }
        std::borrow::Cow::Owned(score)
    }

    /// Tempo map with an optional host tempo replacing it.
    pub fn timeline(&self, host_bpm: Option<f32>) -> Timeline {
        match host_bpm.filter(|b| self.follow_host && b.is_finite() && *b > 0.0) {
            Some(bpm) => Timeline::constant(bpm),
            None => Timeline::new(&self.tempo),
        }
    }

    /// The value of a parameter at a tick: its curve plus the style's offset.
    pub fn param(&self, param: Param, tick: f32) -> f32 {
        let (lo, hi, default) = param.range();
        let base = self
            .curves
            .get(&param)
            .map_or(default, |c| curve_value(c, tick, default));
        (base + self.style.color.get(&param).copied().unwrap_or(0.0)).clamp(lo, hi)
    }
}

/// Converts between ticks and seconds.
#[derive(Clone, Debug, PartialEq)]
pub struct Timeline {
    /// (tick, seconds at that tick, seconds per tick)
    segments: Vec<(f64, f64, f64)>,
}
impl Timeline {
    pub fn constant(bpm: f32) -> Self {
        Self::new(&[Tempo { tick: 0, bpm }])
    }
    pub fn new(tempo: &[Tempo]) -> Self {
        let mut segments = Vec::new();
        let mut seconds = 0.0;
        let mut previous: Option<(f64, f64)> = None;
        for t in tempo {
            let per_tick = 60.0 / (t.bpm.max(1.0) as f64 * TICKS_PER_BEAT as f64);
            if let Some((tick, rate)) = previous {
                seconds += (t.tick as f64 - tick) * rate;
            }
            segments.push((t.tick as f64, seconds, per_tick));
            previous = Some((t.tick as f64, per_tick));
        }
        if segments.is_empty() {
            segments.push((0.0, 0.0, 60.0 / (120.0 * TICKS_PER_BEAT as f64)));
        }
        Self { segments }
    }
    pub fn seconds(&self, tick: f64) -> f64 {
        let i = self
            .segments
            .partition_point(|s| s.0 <= tick)
            .saturating_sub(1);
        let (t0, s0, rate) = self.segments[i];
        s0 + (tick - t0) * rate
    }
    pub fn tick(&self, seconds: f64) -> f64 {
        let i = self
            .segments
            .partition_point(|s| s.1 <= seconds)
            .saturating_sub(1);
        let (t0, s0, rate) = self.segments[i];
        t0 + (seconds - s0) / rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn timeline_round_trips() {
        let t = Timeline::new(&[
            Tempo {
                tick: 0,
                bpm: 120.0,
            },
            Tempo {
                tick: 960,
                bpm: 60.0,
            },
        ]);
        assert!((t.seconds(480.0) - 0.5).abs() < 1e-9);
        assert!((t.seconds(960.0) - 1.0).abs() < 1e-9);
        assert!((t.seconds(1440.0) - 2.0).abs() < 1e-9);
        for tick in [0.0, 100.0, 960.0, 2000.0] {
            assert!((t.tick(t.seconds(tick)) - tick).abs() < 1e-6);
        }
    }
    #[test]
    fn curves_and_style_offsets() {
        let mut score = Score::default();
        score.curves.insert(
            Param::Dynamics,
            vec![
                Point {
                    tick: 0,
                    value: 0.0,
                },
                Point {
                    tick: 100,
                    value: 100.0,
                },
            ],
        );
        assert_eq!(score.param(Param::Dynamics, 50.0), 50.0);
        assert_eq!(score.param(Param::Dynamics, 500.0), 100.0);
        assert_eq!(score.param(Param::Brightness, 0.0), 64.0);
        score.style.color.insert(Param::Brightness, 100.0);
        assert_eq!(score.param(Param::Brightness, 0.0), 127.0);
    }
    #[test]
    fn one_pass_cuts_the_score_at_the_loop() {
        let note = |start, length| Note {
            start,
            length,
            ..Default::default()
        };
        let mut score = Score {
            notes: vec![note(0, 480), note(1440, 960), note(1920, 480)],
            ..Default::default()
        };
        assert_eq!(score.one_pass().notes.len(), 3);
        score.loop_ticks = 1920;
        let pass = score.one_pass();
        assert_eq!(pass.notes.len(), 2);
        assert_eq!(pass.notes[1].length, 480);
    }

    #[test]
    fn sanitize_orders_and_numbers_notes() {
        let mut score = Score {
            notes: vec![
                Note {
                    id: 5,
                    start: 960,
                    ..Default::default()
                },
                Note {
                    id: 5,
                    start: 0,
                    ..Default::default()
                },
                Note {
                    length: 0,
                    ..Default::default()
                },
            ],
            tempo: Vec::new(),
            ..Default::default()
        };
        score.sanitize();
        assert_eq!(score.notes.len(), 2);
        assert_eq!(score.notes[0].start, 0);
        assert_ne!(score.notes[0].id, score.notes[1].id);
        assert_eq!(score.tempo[0].tick, 0);
        let json = serde_json::to_string(&score).unwrap();
        let back: Score = serde_json::from_str(&json).unwrap();
        assert_eq!(back, score);
    }
}
