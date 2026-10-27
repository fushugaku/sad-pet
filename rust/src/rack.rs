//! Rack timbres: an instrument built from modules placed in any order. Sound
//! runs from the top module down. Each source starts a chain of its own, the
//! modules under it process that chain, and the chains meet in the Mixer,
//! whose sum the modules under it process in turn. The sequencer and the
//! modulators are the timbre's own and stay under every module.
use crate::dsp::sample::LiveInput;
use crate::taps::Stage;
use serde::{Deserialize, Serialize};

/// A Grain engine reading the chain keeps at most this much of it.
pub const MAX_BUFFER_SECONDS: f32 = 32.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Module {
    /// The loaded file, played at its cursor through the notes' VCA. A Grain
    /// engine right under it granulates the file itself, note by note, as a
    /// Granular timbre does.
    Sample,
    /// Up to four oscillators per note, through the notes' VCA.
    Synth,
    /// The drum machine with its own pattern; it plays without notes.
    Drums,
    /// A kit of sample pads, played by the notes and by pad tracks. Its pads
    /// have their own envelopes, so the chain is one stream, as the Drums'.
    Sampler,
    /// Sings its score with the song transport; its phrases render off the
    /// audio thread and the host hands the voice in.
    Vocal,
    /// The one-effect Source FX slot.
    SourceFx,
    /// Grains of the Sample right above it, or of a live history of its chain.
    Grains,
    Resynth,
    /// The first effect rack (Pre-filter FX of other timbres).
    PreFx,
    /// Filter 1 and 2 with the filters added around them. Right under a Synth
    /// or a Sample (and its grains) every note has its own; elsewhere one
    /// pair filters the chain.
    Filters,
    /// The second effect rack (Post-filter FX of other timbres).
    PostFx,
    /// Where the chains above it meet, each on a strip of its own.
    Mixer,
}
impl Module {
    pub const ALL: [Module; 12] = [
        Module::Sample,
        Module::Synth,
        Module::Drums,
        Module::Sampler,
        Module::Vocal,
        Module::SourceFx,
        Module::Grains,
        Module::Resynth,
        Module::PreFx,
        Module::Filters,
        Module::PostFx,
        Module::Mixer,
    ];
    pub fn source(self) -> bool {
        matches!(
            self,
            Module::Sample | Module::Synth | Module::Drums | Module::Sampler | Module::Vocal
        )
    }
    /// The section tap (Output row) this module ends in, if any.
    pub fn stage(self) -> Option<Stage> {
        match self {
            Module::SourceFx => Some(Stage::Source),
            Module::Grains => Some(Stage::Grains),
            Module::Resynth => Some(Stage::Resynth),
            Module::PreFx => Some(Stage::PreFx),
            Module::Filters => Some(Stage::Filters),
            Module::PostFx => Some(Stage::PostFx),
            _ => None,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Module::Sample => "Sample",
            Module::Synth => "Synth",
            Module::Drums => "Drums",
            Module::Sampler => "Sampler",
            Module::Vocal => "Vocal",
            Module::SourceFx => "Insert FX",
            Module::Grains => "Grain engine",
            Module::Resynth => "Resynth",
            Module::PreFx => "FX A",
            Module::Filters => "Filters",
            Module::PostFx => "FX B",
            Module::Mixer => "Mixer",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Slot {
    pub module: Module,
    /// Off, a source is silent and any other module passes its input on.
    pub enabled: bool,
}
impl Default for Slot {
    fn default() -> Self {
        Self {
            module: Module::Mixer,
            enabled: true,
        }
    }
}

/// One chain's strip in the Mixer.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Strip {
    /// 0–2, unity at 1.
    pub level: f32,
    /// 0 left, 0.5 centre, 1 right; a balance that keeps the centre at unity.
    pub pan: f32,
    pub muted: bool,
    pub solo: bool,
}
impl Default for Strip {
    fn default() -> Self {
        Self {
            level: 1.0,
            pan: 0.5,
            muted: false,
            solo: false,
        }
    }
}
impl Strip {
    fn sanitize(&mut self) {
        self.level = if self.level.is_finite() {
            self.level.clamp(0.0, 2.0)
        } else {
            1.0
        };
        self.pan = if self.pan.is_finite() {
            self.pan.clamp(0.0, 1.0)
        } else {
            0.5
        };
    }
    /// One of `STRIP_FIELDS`.
    pub fn field(&self, field: usize) -> f32 {
        match field {
            0 => self.level,
            1 => self.pan,
            2 => f32::from(self.muted),
            _ => f32::from(self.solo),
        }
    }
    pub fn set_field(&mut self, field: usize, value: f32) {
        match field {
            0 => self.level = value,
            1 => self.pan = value,
            2 => self.muted = value >= 0.5,
            _ => self.solo = value >= 0.5,
        }
    }
    /// Left and right gains, or none while muted or outside a solo.
    pub fn gains(self, soloing: bool) -> crate::Stereo {
        if self.muted || (soloing && !self.solo) {
            return [0.0; 2];
        }
        let left = (2.0 * (1.0 - self.pan)).min(1.0);
        let right = (2.0 * self.pan).min(1.0);
        [self.level * left, self.level * right]
    }
}

/// The chains with a Mixer strip, in strip order.
pub const STRIP_SOURCES: [Module; STRIPS] = [
    Module::Sample,
    Module::Synth,
    Module::Drums,
    Module::Sampler,
    Module::Vocal,
];
pub const STRIPS: usize = 5;
/// A strip's numeric controls, in host order.
pub const STRIP_FIELDS: [(&str, &str, f32, f32, f32); 4] = [
    ("level", "level", 0.0, 2.0, 1.0),
    ("pan", "pan", 0.0, 1.0, 0.5),
    ("mute", "mute", 0.0, 1.0, 0.0),
    ("solo", "solo", 0.0, 1.0, 0.0),
];

/// The Mixer strips of the source chains.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Strips {
    pub sample: Strip,
    pub synth: Strip,
    pub drums: Strip,
    pub sampler: Strip,
    pub vocal: Strip,
}
impl Strips {
    pub fn get(&self, source: Module) -> Strip {
        match source {
            Module::Sample => self.sample,
            Module::Synth => self.synth,
            Module::Drums => self.drums,
            Module::Sampler => self.sampler,
            Module::Vocal => self.vocal,
            _ => Strip::default(),
        }
    }
    /// By position in `STRIP_SOURCES`.
    pub fn at(&self, index: usize) -> Strip {
        self.get(STRIP_SOURCES[index])
    }
    pub fn at_mut(&mut self, index: usize) -> &mut Strip {
        match index {
            0 => &mut self.sample,
            1 => &mut self.synth,
            2 => &mut self.drums,
            3 => &mut self.sampler,
            _ => &mut self.vocal,
        }
    }
    pub fn get_mut(&mut self, source: Module) -> Option<&mut Strip> {
        match source {
            Module::Sample => Some(&mut self.sample),
            Module::Synth => Some(&mut self.synth),
            Module::Drums => Some(&mut self.drums),
            Module::Sampler => Some(&mut self.sampler),
            Module::Vocal => Some(&mut self.vocal),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Top to bottom; each module at most once.
    pub modules: Vec<Slot>,
    pub strips: Strips,
    /// Seconds of its chain a Grain engine keeps when it does not read the
    /// Sample right above it.
    pub buffer_seconds: f32,
}
impl Default for Settings {
    /// Drums and a filtered Synth meeting in the Mixer, then the FX B rack.
    fn default() -> Self {
        Self {
            modules: [
                Module::Drums,
                Module::Synth,
                Module::Filters,
                Module::Mixer,
                Module::PostFx,
            ]
            .map(|module| Slot {
                module,
                enabled: true,
            })
            .to_vec(),
            strips: Strips::default(),
            buffer_seconds: 8.0,
        }
    }
}
impl Settings {
    pub fn sanitize(&mut self) {
        let mut seen = Vec::with_capacity(Module::ALL.len());
        self.modules.retain(|slot| {
            let first = !seen.contains(&slot.module);
            seen.push(slot.module);
            first
        });
        for source in STRIP_SOURCES {
            if let Some(strip) = self.strips.get_mut(source) {
                strip.sanitize();
            }
        }
        self.buffer_seconds = if self.buffer_seconds.is_finite() {
            self.buffer_seconds.clamp(0.125, MAX_BUFFER_SECONDS)
        } else {
            8.0
        };
    }
    pub fn has(&self, module: Module) -> bool {
        self.modules.iter().any(|slot| slot.module == module)
    }
    fn enabled(&self, module: Module) -> bool {
        self.modules
            .iter()
            .any(|slot| slot.module == module && slot.enabled)
    }
    /// A Grain engine granulates the file when it sits right under the Sample
    /// and both are on.
    pub fn grains_read_file(&self) -> bool {
        self.modules.windows(2).any(|pair| {
            pair[0].module == Module::Sample
                && pair[1].module == Module::Grains
                && pair[0].enabled
                && pair[1].enabled
        })
    }
    /// The live history a Grain engine reads when it does not read the file.
    pub fn history(&self) -> Option<crate::synth::History> {
        (self.enabled(Module::Grains) && !self.grains_read_file()).then_some(
            crate::synth::History {
                granular: true,
                buffer_seconds: self.buffer_seconds,
            },
        )
    }
    pub fn plan(&self) -> Plan {
        Plan::new(&self.modules)
    }
}

/// Which chain a module processes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chain {
    /// Above every source: there is nothing to process yet.
    None,
    /// The chain this source starts.
    Source(Module),
    /// The Mixer's sum.
    Mixed,
}

/// One module as the frame loop runs it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Step {
    pub module: Module,
    pub enabled: bool,
    pub chain: Chain,
    /// Every note of the chain still has a signal of its own here: a Synth or
    /// Sample, the Grain engine granulating that Sample, and Filters right
    /// under them. The notes' VCA follows the last such module.
    pub per_note: bool,
    /// For a Grain engine: it granulates the file of the Sample above it.
    pub reads_file: bool,
    /// For a source: its chain ends in the Mixer rather than at the output.
    pub mixed: bool,
}

/// The modules in order, with what each one processes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Plan {
    pub steps: [Option<Step>; Module::ALL.len()],
}
impl Plan {
    pub fn new(modules: &[Slot]) -> Self {
        let mut steps = [None; Module::ALL.len()];
        let mut chain = Chain::None;
        let mut per_note = false;
        let mut previous: Option<Slot> = None;
        let mixer = modules.iter().position(|s| s.module == Module::Mixer);
        for (i, slot) in modules.iter().take(steps.len()).enumerate() {
            let mut step = Step {
                module: slot.module,
                enabled: slot.enabled,
                chain,
                per_note: false,
                reads_file: false,
                mixed: false,
            };
            match slot.module {
                Module::Sample
                | Module::Synth
                | Module::Drums
                | Module::Sampler
                | Module::Vocal => {
                    chain = Chain::Source(slot.module);
                    step.chain = chain;
                    // Off, a voiced source has no notes for the modules under it;
                    // the stream sources have none of the notes' own.
                    per_note =
                        matches!(slot.module, Module::Sample | Module::Synth) && slot.enabled;
                    step.per_note = per_note;
                    step.mixed = mixer.is_some_and(|m| m > i);
                }
                Module::Mixer => {
                    chain = Chain::Mixed;
                    step.chain = chain;
                    per_note = false;
                }
                Module::Grains => {
                    step.reads_file = slot.enabled
                        && previous.is_some_and(|p| p.module == Module::Sample && p.enabled);
                    per_note = step.reads_file;
                    step.per_note = per_note;
                }
                // A filter that is off keeps the notes apart for the next one.
                Module::Filters => step.per_note = per_note,
                _ => per_note = false,
            }
            steps[i] = Some(step);
            previous = Some(*slot);
        }
        Self { steps }
    }
    pub fn iter(&self) -> impl Iterator<Item = Step> + '_ {
        self.steps.iter().map_while(|s| *s)
    }
}

/// The history a Grain engine reads, allocated off the audio thread; the
/// engine takes it over in `exchange_preset`.
#[derive(Default)]
pub struct Prepared(pub Option<LiveInput>);
impl Clone for Prepared {
    fn clone(&self) -> Self {
        Self::default()
    }
}
impl std::fmt::Debug for Prepared {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreparedRack")
            .field("history", &self.0.is_some())
            .finish()
    }
}
impl Prepared {
    pub fn prepare(&mut self, settings: Option<&Settings>, rate: u32) {
        self.0 = settings
            .and_then(Settings::history)
            .map(|history| LiveInput::for_rack(rate, history.buffer_seconds));
    }
    /// A synchronous owner keeps the engine's running history when it fits.
    pub fn prepare_reusing(&mut self, settings: Option<&Settings>, rate: u32, kept: bool) {
        if kept {
            self.0 = None;
        } else {
            self.prepare(settings, rate);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slots(modules: &[Module]) -> Vec<Slot> {
        modules
            .iter()
            .map(|&module| Slot {
                module,
                enabled: true,
            })
            .collect()
    }

    #[test]
    fn each_source_starts_a_chain_that_meets_the_others_in_the_mixer() {
        use Module::*;
        let plan = Plan::new(&slots(&[
            PostFx, Drums, Filters, Synth, Filters, Mixer, PreFx, Sample,
        ]));
        let steps: Vec<_> = plan.iter().collect();
        assert_eq!(steps[0].chain, Chain::None);
        assert_eq!(steps[1].chain, Chain::Source(Drums));
        assert!(steps[1].mixed && !steps[1].per_note);
        // Filters under Drums filter the chain as one.
        assert_eq!(steps[2].chain, Chain::Source(Drums));
        assert!(!steps[2].per_note);
        assert_eq!(steps[3].chain, Chain::Source(Synth));
        assert!(steps[3].per_note && steps[4].per_note);
        assert_eq!(steps[5].chain, Chain::Mixed);
        assert_eq!(steps[6].chain, Chain::Mixed);
        // A source under the Mixer goes to the output beside its sum.
        assert_eq!(steps[7].chain, Chain::Source(Sample));
        assert!(!steps[7].mixed);
    }

    #[test]
    fn grains_read_the_file_only_right_under_the_sample() {
        use Module::*;
        let mut settings = Settings {
            modules: slots(&[Sample, Grains, Filters, Mixer]),
            ..Settings::default()
        };
        assert!(settings.grains_read_file());
        assert_eq!(settings.history(), None);
        let steps: Vec<_> = settings.plan().iter().collect();
        assert!(steps[1].reads_file && steps[1].per_note && steps[2].per_note);
        settings.modules = slots(&[Sample, Filters, Grains, Mixer]);
        assert!(!settings.grains_read_file());
        assert!(settings.history().is_some());
        let steps: Vec<_> = settings.plan().iter().collect();
        assert!(steps[1].per_note && !steps[2].reads_file && !steps[2].per_note);
        // Off, the Sample leaves the Grain engine its chain's history.
        settings.modules = slots(&[Sample, Grains]);
        settings.modules[0].enabled = false;
        assert!(settings.history().is_some());
    }

    #[test]
    fn sanitize_keeps_the_first_of_each_module_and_clamps_the_mixer() {
        use Module::*;
        let mut settings = Settings {
            modules: slots(&[Synth, Mixer, Synth, Filters, Mixer]),
            buffer_seconds: 1e9,
            ..Settings::default()
        };
        settings.strips.synth.level = f32::NAN;
        settings.strips.drums.pan = 3.0;
        settings.sanitize();
        let modules: Vec<_> = settings.modules.iter().map(|s| s.module).collect();
        assert_eq!(modules, [Synth, Mixer, Filters]);
        assert_eq!(settings.strips.synth.level, 1.0);
        assert_eq!(settings.strips.drums.pan, 1.0);
        assert_eq!(settings.buffer_seconds, MAX_BUFFER_SECONDS);
    }

    #[test]
    fn strips_balance_mute_and_solo() {
        let strip = Strip::default();
        assert_eq!(strip.gains(false), [1.0, 1.0]);
        let left = Strip {
            pan: 0.0,
            level: 0.5,
            ..Strip::default()
        };
        assert_eq!(left.gains(false), [0.5, 0.0]);
        assert_eq!(strip.gains(true), [0.0, 0.0]);
        let solo = Strip {
            solo: true,
            ..Strip::default()
        };
        assert_eq!(solo.gains(true), [1.0, 1.0]);
        let muted = Strip {
            muted: true,
            ..solo
        };
        assert_eq!(muted.gains(true), [0.0, 0.0]);
    }
}
