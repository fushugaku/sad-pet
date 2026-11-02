//! A directed interaction graph shared by every oscillator and grain group.
use serde::{Deserialize, Serialize};

pub const VERSION: u32 = 1;
pub const MAX_ROUTES: usize = 96;
pub const NODES: usize = 30;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Transport {
    pub enabled: bool,
    pub window_seconds: f32,
    pub rolling: bool,
    pub seed: u32,
    pub rate: f32,
}
impl Default for Transport {
    fn default() -> Self {
        Self {
            enabled: false,
            window_seconds: 0.04,
            rolling: true,
            seed: 0x53414450,
            rate: 1.0,
        }
    }
}
impl Transport {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
    pub fn sanitize(&mut self) {
        crate::preset::finite_clamp(&mut self.window_seconds, 0.005, 0.25, 0.04);
        crate::preset::finite_clamp(&mut self.rate, 0.125, 8.0, 1.0);
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Node {
    #[default]
    None,
    Osc {
        id: u32,
    },
    Synth {
        index: u8,
    },
    Wavetable {
        index: u8,
    },
    Resynth {
        index: u8,
    },
    Tb303 {
        index: u8,
    },
    Tb303Vco,
    Group {
        index: u8,
    },
    GrainWet,
}
impl Node {
    pub fn resolve(self, oscillators: Option<&crate::oscillators::Settings>) -> Option<usize> {
        match self {
            Self::None => None,
            Self::Osc { id } => oscillators.and_then(|o| o.index(id)),
            Self::Synth { index: i @ 0..=3 } => Some(8 + i as usize),
            Self::Wavetable { index: i @ 0..=3 } => Some(12 + i as usize),
            Self::Resynth { index: i @ 0..=3 } => Some(16 + i as usize),
            Self::Tb303 { index: i @ 0..=3 } => Some(20 + i as usize),
            Self::Tb303Vco => Some(24),
            Self::Group { index: i @ 0..=3 } => Some(25 + i as usize),
            Self::GrainWet => Some(29),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncMode {
    #[default]
    Off,
    Soft,
    Hard,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    #[default]
    Playback,
    Clock,
    Both,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tap {
    #[default]
    Generation,
    Output,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    #[default]
    NaturalWrap,
    AudioZeroCross,
    GroupClockWrap,
    GrainLaunch,
    Auto,
}
impl Event {
    pub fn index(self) -> usize {
        if self == Self::Auto { 0 } else { self as usize }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Bridge {
    #[default]
    Auto,
    SameVoice,
    Mix,
    Voice {
        index: u8,
    },
    Band {
        index: u8,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Route {
    pub source: Node,
    pub target: Node,
    pub enabled: bool,
    pub tap: Tap,
    pub event: Event,
    pub bridge: Bridge,
    pub ring: f32,
    pub sync: SyncMode,
    pub scope: Scope,
    pub strength: f32,
    /// Milliseconds of phase correction for Soft Sync.
    pub response: f32,
    pub reset_phase: f32,
    pub divide: u8,
    pub include_forced: bool,
}
impl Default for Route {
    fn default() -> Self {
        Self {
            source: Node::None,
            target: Node::None,
            enabled: true,
            tap: Tap::Generation,
            event: Event::Auto,
            bridge: Bridge::Auto,
            ring: 0.0,
            sync: SyncMode::Off,
            scope: Scope::Playback,
            strength: 1.0,
            response: 5.0,
            reset_phase: 0.0,
            divide: 1,
            include_forced: false,
        }
    }
}
impl Route {
    pub fn needs_group_clock(self) -> bool {
        self.enabled
            && self.sync != SyncMode::Off
            && ((matches!(self.target, Node::Group { .. }) && self.scope != Scope::Playback)
                || (matches!(self.source, Node::Group { .. })
                    && (self.event == Event::GroupClockWrap
                        || (self.event == Event::Auto && self.tap == Tap::Generation))))
    }
    pub fn sanitize(&mut self) {
        use crate::preset::finite_clamp;
        finite_clamp(&mut self.ring, 0.0, 1.0, 0.0);
        finite_clamp(&mut self.strength, 0.0, 1.0, 1.0);
        finite_clamp(&mut self.response, 0.1, 1000.0, 5.0);
        finite_clamp(&mut self.reset_phase, 0.0, 1.0, 0.0);
        self.divide = self.divide.clamp(1, 32);
    }
}

/// Editing a master replaces only the overlapping domain. Route slots stay
/// fixed because host automation addresses them by index.
pub fn replace_masters(routes: &mut [Route], edited: usize) {
    let Some(mut incoming) = routes.get(edited).copied() else {
        return;
    };
    if !matches!(incoming.target, Node::Group { .. }) {
        incoming.scope = Scope::Playback;
        routes[edited].scope = Scope::Playback;
    }
    if !incoming.enabled {
        return;
    }
    for (i, other) in routes.iter_mut().enumerate() {
        if i == edited || !other.enabled || other.target != incoming.target {
            continue;
        }
        if incoming.ring > 0.0 {
            other.ring = 0.0;
        }
        if incoming.sync == SyncMode::Off || other.sync == SyncMode::Off {
            continue;
        }
        match (incoming.scope, other.scope) {
            (Scope::Playback, Scope::Both) => other.scope = Scope::Clock,
            (Scope::Clock, Scope::Both) => other.scope = Scope::Playback,
            (Scope::Playback, Scope::Clock) | (Scope::Clock, Scope::Playback) => {}
            _ => other.sync = SyncMode::Off,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub version: u32,
    pub routes: Vec<Route>,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            version: VERSION,
            routes: Vec::new(),
        }
    }
}
impl Settings {
    pub fn is_default(&self) -> bool {
        self.routes.is_empty()
    }
    pub fn validate(
        &self,
        oscillators: Option<&crate::oscillators::Settings>,
    ) -> Result<(), String> {
        if self.version != VERSION {
            return Err("Unsupported interaction graph version".into());
        }
        if self.routes.len() > MAX_ROUTES {
            return Err("Too many oscillator interactions".into());
        }
        let mut ring = [false; NODES];
        let mut playback = [false; NODES];
        let mut clock = [false; NODES];
        for r in &self.routes {
            if !r.enabled {
                continue;
            }
            let target = r
                .target
                .resolve(oscillators)
                .ok_or("Interaction target is missing")?;
            r.source
                .resolve(oscillators)
                .ok_or("Interaction source is missing")?;
            if r.target == Node::GrainWet {
                return Err("Choose an individual grain group as target".into());
            }
            if r.sync != SyncMode::Off
                && !matches!(r.target, Node::Group { .. })
                && r.scope != Scope::Playback
            {
                return Err("Clock scope is available on grain groups".into());
            }
            if r.ring > 0.0 {
                if ring[target] {
                    return Err("An audio target has one Ring master".into());
                }
                ring[target] = true;
            }
            if r.sync != SyncMode::Off {
                let p = r.scope != Scope::Clock;
                let c = r.scope != Scope::Playback;
                if (p && playback[target]) || (c && clock[target]) {
                    return Err("A phase domain has one Sync master".into());
                }
                playback[target] |= p;
                clock[target] |= c;
            }
        }
        Ok(())
    }
    pub fn sanitize(&mut self) {
        self.routes.truncate(MAX_ROUTES);
        for route in &mut self.routes {
            route.sanitize();
        }
    }
    pub fn remove_node(&mut self, node: Node) {
        for r in &mut self.routes {
            if r.source == node || r.target == node {
                *r = Route {
                    enabled: false,
                    ..Default::default()
                };
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Controls {
    pub routes: [Route; MAX_ROUTES],
    pub count: usize,
}
impl Default for Controls {
    fn default() -> Self {
        Self {
            routes: [Route::default(); MAX_ROUTES],
            count: 0,
        }
    }
}
impl Controls {
    pub fn from_settings(settings: &Settings) -> Self {
        let mut result = Self {
            count: settings.routes.len().min(MAX_ROUTES),
            ..Self::default()
        };
        result.routes[..result.count].copy_from_slice(&settings.routes[..result.count]);
        result
    }
}
impl Serialize for Controls {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.routes[..self.count.min(MAX_ROUTES)].serialize(s)
    }
}
impl<'de> Deserialize<'de> for Controls {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let routes = Vec::<Route>::deserialize(d)?;
        if routes.len() > MAX_ROUTES {
            return Err(serde::de::Error::custom("Too many interaction controls"));
        }
        Ok(Self::from_settings(&Settings {
            routes,
            ..Default::default()
        }))
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureMode {
    #[default]
    Chain,
    Osc,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Capture {
    pub mode: CaptureMode,
    /// Physical slot bits: toggling selection never changes slot identities.
    pub slots: u8,
}
impl Capture {
    pub fn is_default(&self) -> bool {
        self.mode == CaptureMode::Chain
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scheduler {
    #[default]
    Legacy,
    Independent,
}
impl Scheduler {
    pub fn is_legacy(&self) -> bool {
        *self == Self::Legacy
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_directed_pairs_accept_the_same_modes() {
        let oscs = crate::oscillators::Settings::default();
        let nodes = [
            Node::Osc { id: 1 },
            Node::Synth { index: 0 },
            Node::Wavetable { index: 3 },
            Node::Resynth { index: 2 },
            Node::Tb303Vco,
            Node::Group { index: 0 },
            Node::Group { index: 3 },
        ];
        for source in nodes {
            for target in nodes {
                for sync in [SyncMode::Off, SyncMode::Soft, SyncMode::Hard] {
                    for ring in [0.0, 1.0] {
                        let s = Settings {
                            routes: vec![Route {
                                source,
                                target,
                                sync,
                                ring,
                                ..Default::default()
                            }],
                            ..Default::default()
                        };
                        assert!(s.validate(Some(&oscs)).is_ok(), "{source:?} → {target:?}");
                    }
                }
            }
        }
    }
    #[test]
    fn a_group_has_one_audio_master_and_independent_phase_masters() {
        let target = Node::Group { index: 0 };
        let mut s = Settings {
            routes: vec![
                Route {
                    source: Node::Group { index: 1 },
                    target,
                    ring: 1.0,
                    sync: SyncMode::Hard,
                    ..Default::default()
                },
                Route {
                    source: Node::Group { index: 2 },
                    target,
                    sync: SyncMode::Soft,
                    scope: Scope::Clock,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert!(s.validate(None).is_ok());
        s.routes[1].ring = 0.5;
        assert!(s.validate(None).is_err());
        s.routes[1].ring = 0.0;
        s.routes[1].scope = Scope::Both;
        assert!(s.validate(None).is_err());
    }
}
