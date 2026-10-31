//! Original factory instruments: portable Rack settings over immutable tables.
//! No third-party patches, sample paths, private caches or generated DSP state.
use crate::{
    preset::{Destination, Preset},
    rack::Module,
    timbres::Kind,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

pub struct Instrument {
    pub name: &'static str,
    pub category: &'static str,
    pub a: &'static str,
    pub b: &'static str,
    pub extra: Option<[&'static str; 2]>,
}
macro_rules! instrument {
    ($name:literal,$category:literal,$a:literal,$b:literal) => {
        Instrument {
            name: $name,
            category: $category,
            a: $a,
            b: $b,
            extra: None,
        }
    };
}
macro_rules! complex_instrument {
    ($name:literal,$category:literal,$a:literal,$b:literal,$c:literal,$d:literal) => {
        Instrument {
            name: $name,
            category: $category,
            a: $a,
            b: $b,
            extra: Some([$c, $d]),
        }
    };
}
pub static INSTRUMENTS: &[Instrument] = &[
    instrument!("Rounded Current", "Bass", "rounded-square", "basic-morph"),
    instrument!("Vowel Rubber", "Bass", "vowel-sweep", "rounded-square"),
    instrument!("Hollow Engine", "Bass", "hollow-pulse", "odd-even"),
    instrument!("Comb Pressure", "Bass", "comb-sweep", "basic-morph"),
    instrument!("Odd Circuit", "Bass", "odd-even", "pulse-width"),
    instrument!("Glass Sub", "Bass", "additive-glass", "triangle-bloom"),
    instrument!("Talking Circuit", "Bass", "talking-bass", "saw-brass"),
    instrument!("Dust Floor", "Bass", "dust-contours", "rounded-square"),
    instrument!("Prism Needle", "Lead", "prism-fold", "triangle-bloom"),
    instrument!("Brass Wire", "Lead", "saw-brass", "odd-even"),
    instrument!("Formant Line", "Lead", "nasal-reed", "basic-morph"),
    instrument!("Pulse Flight", "Lead", "pulse-width", "octave-stack"),
    instrument!("Spiral Singer", "Lead", "phase-spiral", "choir-colors"),
    instrument!("Even Ray", "Lead", "harmonic-shift", "rounded-square"),
    instrument!("Razor Ribbon", "Lead", "razor-stack", "spectral-peaks"),
    instrument!("Liquid Reed", "Lead", "digital-reeds", "throat-bloom"),
    instrument!("Slow Choir", "Pad", "choir-colors", "vowel-sweep"),
    instrument!("Harmonic Horizon", "Pad", "harmonic-shift", "octave-stack"),
    instrument!("Warm Geometry", "Pad", "basic-morph", "prism-fold"),
    instrument!("Frozen Canopy", "Pad", "frozen-spark", "rounded-square"),
    instrument!("Vowel Clouds", "Pad", "vowel-sweep", "noise-cloud"),
    instrument!("Octave Mist", "Pad", "octave-stack", "triangle-bloom"),
    instrument!("Soft Prism", "Pad", "prism-fold", "organ-blend"),
    instrument!("Amber Organ", "Pad", "organ-blend", "hollow-pulse"),
    instrument!(
        "Breathing Glass",
        "Pad",
        "breath-formants",
        "additive-glass"
    ),
    instrument!("Air Garden", "Pad", "fractured-air", "choir-colors"),
    instrument!(
        "Additive Tines",
        "Keys & Pluck",
        "additive-glass",
        "triangle-bloom"
    ),
    instrument!(
        "Spectral Marimba",
        "Keys & Pluck",
        "spectral-peaks",
        "harmonic-shift"
    ),
    instrument!("Hollow Keys", "Keys & Pluck", "hollow-pulse", "organ-blend"),
    instrument!(
        "Glass Chord",
        "Keys & Pluck",
        "additive-glass",
        "frozen-spark"
    ),
    instrument!(
        "Rounded Pluck",
        "Keys & Pluck",
        "rounded-square",
        "pulse-width"
    ),
    instrument!(
        "Bell River",
        "Keys & Pluck",
        "bell-metals",
        "harmonic-steps"
    ),
    instrument!(
        "Pulse Harp",
        "Keys & Pluck",
        "pulse-width",
        "additive-glass"
    ),
    instrument!("Reed Bells", "Keys & Pluck", "digital-reeds", "bell-metals"),
    instrument!("Analog Steps", "Sequence", "basic-morph", "pulse-width"),
    instrument!("Metal Carousel", "Sequence", "metallic-scan", "bell-metals"),
    instrument!("Talking Grid", "Sequence", "talking-bass", "nasal-reed"),
    instrument!("Harmonic Lattice", "Sequence", "harmonic-steps", "odd-even"),
    instrument!("Prism Clock", "Sequence", "prism-fold", "bit-shapes"),
    instrument!("Pulse Conveyor", "Sequence", "pulse-width", "hollow-pulse"),
    instrument!("Comb Machine", "Sequence", "comb-sweep", "spectral-peaks"),
    instrument!(
        "Fractured Steps",
        "Sequence",
        "fractured-air",
        "sync-contours"
    ),
    instrument!("Frozen Dust", "Texture", "dust-contours", "frozen-spark"),
    instrument!("Shifting Air", "Texture", "noise-cloud", "fractured-air"),
    instrument!("Spectral Ash", "Texture", "spectral-peaks", "dust-contours"),
    instrument!("Breath Atlas", "Texture", "breath-formants", "throat-bloom"),
    instrument!("Phase Weather", "Texture", "phase-spiral", "metallic-scan"),
    instrument!("Fractured Prism", "Texture", "bit-shapes", "razor-stack"),
    // Original four-wave instruments for the complex content expansion.
    complex_instrument!(
        "Steel Cascade",
        "Lead",
        "cascade-steel",
        "ratio-bloom",
        "modal-bronze",
        "spectral-weave"
    ),
    complex_instrument!(
        "Operator Crystal",
        "Keys & Pluck",
        "glass-operators",
        "nested-bells",
        "hollow-ceramic",
        "harmonic-eclipse"
    ),
    complex_instrument!(
        "Folded Alloy Bass",
        "Bass",
        "folded-carrier",
        "electric-alloy",
        "wire-chamber",
        "oddity-layers"
    ),
    complex_instrument!(
        "Feedback Glass",
        "Texture",
        "reed-operators",
        "crystal-feedback",
        "prism-chimes",
        "moving-cavities"
    ),
    complex_instrument!(
        "Vowel Labyrinth Pad",
        "Pad",
        "vowel-labyrinth",
        "choir-machine",
        "phase-maze",
        "fourfold-motion"
    ),
    complex_instrument!(
        "Liquid Choir Lead",
        "Lead",
        "liquid-throat",
        "shifting-mouths",
        "windowed-sync",
        "alloy-garden"
    ),
    complex_instrument!(
        "Whisper Weather",
        "Texture",
        "nasal-orbit",
        "whisper-glass",
        "mirrored-pulse",
        "neon-orchestra"
    ),
    complex_instrument!(
        "Syllable Steps",
        "Sequence",
        "formant-weather",
        "synthetic-syllables",
        "phase-fracture",
        "hybrid-tesseract"
    ),
    complex_instrument!(
        "Bronze Vessel",
        "Keys & Pluck",
        "modal-bronze",
        "glass-vessel",
        "spectral-weave",
        "fractal-bloom"
    ),
    complex_instrument!(
        "Bowed Porcelain",
        "Pad",
        "hollow-ceramic",
        "bowed-metal",
        "harmonic-eclipse",
        "frozen-current"
    ),
    complex_instrument!(
        "Tubular Prisms",
        "Keys & Pluck",
        "wire-chamber",
        "tubular-flux",
        "oddity-layers",
        "spectral-foam"
    ),
    complex_instrument!(
        "Reed Chamber",
        "Texture",
        "prism-chimes",
        "reed-cavity",
        "moving-cavities",
        "vapor-lattice"
    ),
    complex_instrument!(
        "Lattice Runner",
        "Lead",
        "phase-maze",
        "bend-lattice",
        "fourfold-motion",
        "byte-labyrinth"
    ),
    complex_instrument!(
        "Fold Circuit Sequence",
        "Sequence",
        "windowed-sync",
        "fold-cascade",
        "alloy-garden",
        "folding-circuit"
    ),
    complex_instrument!(
        "Mirrored Helix Bass",
        "Bass",
        "mirrored-pulse",
        "helix-carrier",
        "neon-orchestra",
        "segmented-glass"
    ),
    complex_instrument!(
        "Fractured Reed",
        "Pad",
        "phase-fracture",
        "twisted-reed",
        "hybrid-tesseract",
        "serrated-orbit"
    ),
    complex_instrument!(
        "Spectral Weave Pad",
        "Pad",
        "spectral-weave",
        "peak-migration",
        "fractal-bloom",
        "cascade-steel"
    ),
    complex_instrument!(
        "Eclipse Constellation",
        "Sequence",
        "harmonic-eclipse",
        "comb-constellation",
        "frozen-current",
        "glass-operators"
    ),
    complex_instrument!(
        "Oddity Inversion",
        "Lead",
        "oddity-layers",
        "spectral-inversion",
        "spectral-foam",
        "folded-carrier"
    ),
    complex_instrument!(
        "Cavity Origami",
        "Texture",
        "moving-cavities",
        "harmonic-origami",
        "vapor-lattice",
        "reed-operators"
    ),
    complex_instrument!(
        "Fourfold Orchestra",
        "Pad",
        "fourfold-motion",
        "string-glass-choir",
        "byte-labyrinth",
        "vowel-labyrinth"
    ),
    complex_instrument!(
        "Alloy Reed Garden",
        "Pad",
        "alloy-garden",
        "reed-to-steel",
        "folding-circuit",
        "liquid-throat"
    ),
    complex_instrument!(
        "Neon Ember Sequence",
        "Sequence",
        "neon-orchestra",
        "ember-prism",
        "segmented-glass",
        "nasal-orbit"
    ),
    complex_instrument!(
        "Polar Tesseract",
        "Texture",
        "hybrid-tesseract",
        "polar-currents",
        "serrated-orbit",
        "formant-weather"
    ),
    complex_instrument!(
        "Fractal Cathedral",
        "Pad",
        "fractal-bloom",
        "dust-cathedral",
        "cascade-steel",
        "modal-bronze"
    ),
    complex_instrument!(
        "Frozen Prism Pad",
        "Pad",
        "frozen-current",
        "prismatic-noise",
        "glass-operators",
        "hollow-ceramic"
    ),
    complex_instrument!(
        "Quantum Foam",
        "Texture",
        "spectral-foam",
        "quantum-grain",
        "folded-carrier",
        "wire-chamber"
    ),
    complex_instrument!(
        "Vapor Choir Steps",
        "Sequence",
        "vapor-lattice",
        "particle-choir",
        "reed-operators",
        "prism-chimes"
    ),
    complex_instrument!(
        "Bytework Bass",
        "Bass",
        "byte-labyrinth",
        "clockwork-formants",
        "vowel-labyrinth",
        "phase-maze"
    ),
    complex_instrument!(
        "Circuit Gears",
        "Sequence",
        "folding-circuit",
        "spectral-gears",
        "liquid-throat",
        "windowed-sync"
    ),
    complex_instrument!(
        "Segmented Logic Keys",
        "Keys & Pluck",
        "segmented-glass",
        "logic-choir",
        "nasal-orbit",
        "mirrored-pulse"
    ),
    complex_instrument!(
        "Terminal Orbit Bass",
        "Bass",
        "serrated-orbit",
        "terminal-bloom",
        "formant-weather",
        "phase-fracture"
    ),
];
impl Instrument {
    pub fn id(&self) -> String {
        format!(
            "{:x}",
            Sha256::digest(format!("SAD PET Wavetable Instruments v1/{}", self.name).as_bytes())
        )[..32]
            .into()
    }
    pub fn record(&self) -> Value {
        json!({"id":self.id(),"name":self.name,"category":format!("Wavetable / {}",self.category),
        "kind":"rack","factory":true,"created_at":0,"modified_at":0,"codec":"gzip"})
    }
    pub fn preset(&self) -> Preset {
        let index = INSTRUMENTS
            .iter()
            .position(|i| i.name == self.name)
            .unwrap();
        let variant = (index % 8) as f32;
        let mut p = Kind::Wavetable.preset().unwrap();
        p.name = self.name.into();
        for slot in &mut p.rack.as_mut().unwrap().modules {
            if slot.module == Module::Filters {
                slot.enabled = true;
            }
        }
        p.params.filter = 0.65 + variant * 0.025;
        p.params.filter_resonance = 0.12;
        p.params.master_volume = 0.8;
        p.params.sample_tune_semis = 0.0;
        p.envs[0].attack = 0.0;
        p.envs[0].decay = 0.28;
        p.envs[0].sustain = 0.8;
        p.envs[0].release = 0.18;
        p.envs[1].attack = 0.0;
        p.envs[1].decay = 0.38;
        p.envs[1].sustain = 0.15;
        p.envs[1].release = 0.18;
        let settings = p.wavetable.as_mut().unwrap();
        settings.count = if self.extra.is_some() { 4 } else { 2 };
        settings.seed = 0x5754_5052_4553_0000 + index as u64;
        let extra = self.extra.unwrap_or(["basic-morph"; 2]);
        let waves = [self.a, self.b, extra[0], extra[1]];
        for (i, osc) in settings
            .oscillators
            .iter_mut()
            .enumerate()
            .take(settings.count as usize)
        {
            osc.asset_id = format!("factory:wt-{}:v1", waves[i]);
            let c = &mut osc.controls;
            c.enabled = true;
            c.level = [0.8, 0.32, 0.24, 0.18][i];
            c.position = if i == 0 {
                0.12 + variant * 0.06
            } else if i == 1 {
                0.7 - variant * 0.05
            } else if i == 2 {
                0.37
            } else {
                0.83
            };
            c.phase_random = 1.0;
            c.detune = 5.0 + variant;
            c.spread = 0.65;
            c.position_env = 2;
            c.position_env_depth = [0.28, -0.18, 0.19, -0.13][i];
        }
        match self.category {
            "Bass" => {
                p.params.master_volume = 0.48;
                p.params.filter = 0.52 + variant * 0.035;
                p.envs[0].sustain = 0.72;
                p.envs[0].release = 0.10;
                for osc in &mut settings.oscillators {
                    osc.controls.transpose = -12.0;
                    osc.controls.phase_random = 0.0;
                }
                settings.oscillators[1].controls.fine = 3.0;
            }
            "Lead" => {
                p.params.master_volume = 0.66;
                settings.oscillators[0].controls.unison = 3;
                settings.oscillators[1].controls.transpose =
                    if index % 2 == 0 { 12.0 } else { 0.0 };
                p.envs[0].sustain = 0.9;
                p.envs[0].release = 0.25;
            }
            "Pad" => {
                // These spectra place energy above a low-pass's usual init
                // cutoff. Keep the harmonics audible before the Tone macro.
                p.params.filter = 0.96;
                settings.oscillators[0].controls.level = 1.0;
                settings.oscillators[1].controls.level = 0.48;
                p.envs[0].attack = 0.48 + variant * 0.025;
                p.envs[0].release = 0.6;
                p.envs[0].sustain = 0.8;
                p.envs[1].attack = 0.58;
                p.envs[1].sustain = 0.7;
                for osc in &mut settings.oscillators {
                    osc.controls.unison = 4;
                    osc.controls.detune = 6.0;
                }
                settings.oscillators[1].controls.fine = -7.0;
            }
            "Keys & Pluck" => {
                p.envs[0].decay = 0.32 + variant * 0.03;
                p.envs[0].sustain = 0.12;
                p.envs[0].release = 0.16;
                p.envs[1].decay = 0.25;
                p.envs[1].sustain = 0.0;
                settings.oscillators[1].controls.transpose = 12.0;
                settings.oscillators[0].controls.position_env_depth = 0.55;
            }
            "Sequence" => {
                use metropolix_core::sequencer::field;
                p.envs[0].decay = 0.18;
                p.envs[0].sustain = 0.35;
                p.envs[0].release = 0.08;
                p.sequencer.enabled = true;
                let mut notes = crate::sequence::Track {
                    target: crate::sequence::Target::Notes,
                    name: "Notes".into(),
                    ..Default::default()
                };
                notes.config[field::LENGTH] = 8;
                for (i, step) in notes.steps.iter_mut().enumerate() {
                    step.pitch = 48 + [0, 7, 12, 3, 10, 7, 15, 12][(i + index) % 8];
                    step.velocity = 80 + (i % 4) as u8 * 10;
                }
                let mut scan = crate::sequence::Track {
                    id: 2,
                    name: "Table motion".into(),
                    target: crate::sequence::Target::Parameter {
                        parameter: "wt_a_position".into(),
                    },
                    amount: 0.24,
                    ..Default::default()
                };
                scan.config[field::LENGTH] = 8;
                for (i, step) in scan.steps.iter_mut().enumerate() {
                    step.cv = ((i + index) % 8) as i32 * 1200 - 4200;
                }
                p.sequencer.tracks = vec![notes, scan];
            }
            "Texture" => {
                p.params.filter = 0.94;
                p.envs[0].attack = 0.32;
                p.envs[0].release = 0.68;
                for osc in &mut settings.oscillators {
                    osc.controls.unison = 3;
                    osc.controls.detune = 14.0;
                }
                settings.oscillators[1].controls.transpose = 7.0;
            }
            _ => {}
        }
        if self.extra.is_some() {
            // Four independently phased spectra have lower crest/RMS than
            // the old coherent basic waves; use measured chord headroom.
            p.params.out_gain = 6.0;
            p.params.filter = match self.category {
                "Bass" => 0.84,
                "Lead" => 0.90,
                "Keys & Pluck" => 0.94,
                "Sequence" => 0.92,
                _ => 0.98,
            };
            for osc in &mut settings.oscillators {
                osc.controls.level *= 0.60;
            }
            settings.oscillators[2].controls.pan = 0.27;
            settings.oscillators[3].controls.pan = 0.73;
            settings.oscillators[2].controls.fine = 8.0;
            settings.oscillators[3].controls.fine = -11.0;
            settings.oscillators[3].controls.position_env = 3;
            settings.oscillators[3].controls.position_env_depth = 0.24;
            p.envs[2].attack = if self.category == "Pad" { 0.68 } else { 0.02 };
            p.envs[2].decay = 0.70;
            p.envs[2].sustain = 0.45;
            p.envs[2].release = 0.55;
            p.lfos[1].rate_hz = 0.08 + variant * 0.017;
            p.lfos[1].depth = 0.13;
            p.lfos[1].destination = control("wt_c_position");
        }
        // Motion is a shared timbre route. Position ENV above remains per note.
        p.lfos[0].rate_hz = if self.category == "Pad" {
            0.05 + variant * 0.012
        } else {
            0.14 + variant * 0.07
        };
        p.lfos[0].depth = if matches!(self.category, "Pad" | "Texture") {
            0.14
        } else {
            0.035
        };
        p.lfos[0].destination = control("wt_a_position");
        p.macros = vec![
            macro_knob(
                "Scan",
                vec![
                    (control("wt_a_position"), 0.4),
                    (control("wt_b_position"), -0.3),
                ],
            ),
            macro_knob(
                "Width",
                vec![
                    (control("wt_a_pan"), -0.25),
                    (control("wt_b_pan"), 0.25),
                    (control("wt_a_detune"), 0.08),
                ],
            ),
            macro_knob(
                "Tone",
                vec![
                    (Destination::Filter, 0.2),
                    (Destination::FilterResonance, 0.1),
                ],
            ),
            macro_knob(
                "Tail",
                vec![(
                    Destination::Env {
                        index: 0,
                        parameter: crate::matrix::EnvParameter::Release,
                    },
                    0.2,
                )],
            ),
        ];
        if self.extra.is_some() {
            p.macros[0].targets.extend(
                macro_knob(
                    "",
                    vec![
                        (control("wt_c_position"), 0.3),
                        (control("wt_d_position"), -0.25),
                    ],
                )
                .targets,
            );
            p.macros[1].targets.extend(
                macro_knob(
                    "",
                    vec![(control("wt_c_pan"), -0.2), (control("wt_d_pan"), 0.2)],
                )
                .targets,
            );
        }
        p.sanitize();
        p
    }
    pub fn document(&self) -> Result<Value, String> {
        use base64::Engine as _;
        Ok(
            json!({"version":1,"meta":{"id":0,"name":self.name,"kind":"rack"},"preset":self.preset(),
            "sample_name":"Wavetable","sample_id":null,"sample_wav":base64::engine::general_purpose::STANDARD.encode(crate::wav::bytes(48000,&[[0.0;2];2])?),
            "source":"file","region":crate::dsp::Region::default(),"fades":crate::dsp::Fades::default(),
            "sync":true,"frozen":false,"bypass":false,"midi_gate":true,"playing":false,"timbres":[],"active_timbre":0}),
        )
    }
}
fn control(id: &str) -> Destination {
    Destination::Control {
        index: crate::parameters::specs()
            .iter()
            .position(|s| s.id == id)
            .unwrap() as u16,
    }
}
fn macro_knob(name: &str, targets: Vec<(Destination, f32)>) -> crate::macros::Macro {
    crate::macros::Macro {
        name: name.into(),
        value: 0.0,
        targets: targets
            .into_iter()
            .map(|(destination, depth)| crate::macros::Target { destination, depth })
            .collect(),
    }
}
pub fn find(id: &str) -> Option<&'static Instrument> {
    INSTRUMENTS.iter().find(|p| p.id() == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn factory_instruments_are_complete_portable_and_cover_every_table() {
        use std::collections::{BTreeMap, BTreeSet};
        assert_eq!(INSTRUMENTS.len(), 80);
        let mut categories = BTreeMap::new();
        let mut ids = BTreeSet::new();
        let mut tables = BTreeSet::new();
        for instrument in INSTRUMENTS {
            *categories.entry(instrument.category).or_insert(0) += 1;
            assert!(ids.insert(instrument.id()));
            let p = instrument.preset();
            assert_eq!(p.macros.len(), 4);
            assert_eq!(Kind::of(&p), Kind::Rack);
            for osc in &p.wavetable.as_ref().unwrap().oscillators {
                assert!(super::super::factory::find(&osc.asset_id).is_some());
                tables.insert(osc.asset_id.clone());
            }
            let state = instrument.document().unwrap();
            let recalled: Preset = serde_json::from_value(state["preset"].clone()).unwrap();
            assert_eq!(p.wavetable, recalled.wavetable);
        }
        assert_eq!(
            categories,
            BTreeMap::from([
                ("Bass", 12),
                ("Lead", 12),
                ("Pad", 18),
                ("Keys & Pluck", 12),
                ("Sequence", 14),
                ("Texture", 12)
            ])
        );
        assert_eq!(tables.len(), 96);
        assert_eq!(INSTRUMENTS.iter().filter(|p| p.extra.is_some()).count(), 32);
    }
}
