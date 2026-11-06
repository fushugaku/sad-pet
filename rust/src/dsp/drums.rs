//! Drum machine runtime and control-side preparation for DXM-088.
use super::sample::LiveInput;
use crate::{
    Stereo,
    drums::{Controls, STEPS, Settings, TRACKS, Track},
};
use sad_pet_dxm088::{Engine, sequencer as seq};

pub struct Runtime {
    pub engine: Engine,
    /// Prepared off audio. Also retires a previous live input on a type change.
    pub live: Option<LiveInput>,
    /// The note shift each engine track was copied with.
    pub shifts: [i32; TRACKS],
}

/// A prepared drum engine and optional granular input, built off the audio thread.
#[derive(Default)]
pub struct Prepared(pub Option<Box<Runtime>>);

impl std::fmt::Debug for Prepared {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PreparedDrums")
            .field("ready", &self.0.is_some())
            .finish()
    }
}

impl Prepared {
    pub fn prepare(&mut self, settings: Option<&Settings>, rate: u32) {
        self.prepare_engine(settings, rate);
        self.prepare_input(settings, rate);
    }

    /// Web hosts can spread construction over separate render quanta.
    pub fn prepare_engine(&mut self, settings: Option<&Settings>, rate: u32) {
        self.0 = settings.map(|s| {
            let mut engine = Engine::new(rate as f32);
            engine.params = s.params.0;
            engine.master = s.master;
            engine.swing = s.swing;
            let mut shifts = [0; TRACKS];
            for (i, track) in s.tracks.iter().enumerate() {
                shifts[i] = track.shift as i32;
                copy_track(&mut engine, i, track, shifts[i]);
            }
            engine.configure_params(&s.params.0);
            Box::new(Runtime {
                engine,
                live: None,
                shifts,
            })
        });
    }

    pub fn prepare_input(&mut self, settings: Option<&Settings>, rate: u32) {
        if let (Some(runtime), Some(s)) = (&mut self.0, settings) {
            let mut input = LiveInput::for_drums(rate, !s.granular, s.buffer_seconds);
            input.sample.name = "Drum machine".into();
            runtime.live = Some(input);
        }
    }
}

/// The engine plays step `s` of a shifted track at `s + shift`, wrapped inside
/// the length; steps past the length keep their places.
fn copy_track(engine: &mut Engine, i: usize, track: &Track, shift: i32) {
    let out = &mut engine.patterns[0].tracks[i];
    out.length = track.length;
    out.swing = track.swing;
    let length = track.length.clamp(1, STEPS);
    for (s, from) in track.steps.iter().enumerate().take(STEPS) {
        let to = if s < length {
            (s as i64 + shift as i64).rem_euclid(length as i64) as usize
        } else {
            s
        };
        out.steps[to] = seq::Step {
            velocity: from.velocity,
            probability: from.probability,
            note: from.note,
            ratchet: from.ratchet,
            micro: from.micro,
            condition: from.condition,
            every: from.every,
            phase: from.phase,
            locks: from.locks.map(|l| seq::Lock {
                param: l.param,
                value: l.value,
            }),
        };
    }
    engine.track_settings(
        0,
        i,
        track.division,
        track.delay,
        track.groove,
        track.groove_amount,
    );
    engine.levels[i] = track.level;
    engine.muted[i] = track.muted;
    engine.solo[i] = track.solo;
}

impl Runtime {
    pub fn configure(&mut self, settings: &Settings, controls: Controls, bpm: f32, transpose: f32) {
        if self.engine.params != controls.0 {
            self.engine.configure_params(&controls.0);
        }
        self.engine.tune(transpose);
        self.engine.master = settings.master;
        self.engine.tempo(bpm, settings.swing);
    }

    pub fn frame(&mut self) -> Stereo {
        let output = self.engine.render(1);
        let frame = [output[0], output[1]];
        self.engine.clear_hits();
        frame
    }

    pub fn sync_tracks(&mut self, settings: &Settings) {
        for (i, track) in settings.tracks.iter().enumerate() {
            copy_track(&mut self.engine, i, track, self.shifts[i]);
        }
    }

    /// Recopies a track whose modulated note shift moved to another step.
    pub fn shift(&mut self, settings: &Settings, shifts: [f32; TRACKS]) {
        for (i, track) in settings.tracks.iter().enumerate() {
            let shift = shifts[i].round() as i32;
            if self.shifts[i] != shift {
                self.shifts[i] = shift;
                copy_track(&mut self.engine, i, track, shift);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factory_tones_and_original_stereo_dsp_are_preserved() {
        let mut settings = Settings::default();
        let original = settings.params;
        settings.sanitize();
        assert_eq!(
            settings.params, original,
            "Validation changed the upstream factory sound"
        );
        for spec in crate::parameters::specs()
            .iter()
            .filter(|s| s.id.starts_with("drum"))
        {
            assert!(
                (spec.min..=spec.max).contains(&spec.default),
                "Invalid host default: {}",
                spec.id
            );
        }
        let mut prepared = Prepared::default();
        prepared.prepare(Some(&settings), 48_000);
        let runtime = prepared.0.as_mut().unwrap();
        let mut reference = Engine::new(48_000.);
        reference.params = settings.params.0;
        reference.configure_params(&settings.params.0);
        for track in 0..TRACKS {
            runtime.engine.trigger(track, 0.83, 0);
            reference.trigger(track, 0.83, 0);
            for _ in 0..2048 {
                let expected = reference.render(1);
                assert_eq!(runtime.frame(), [expected[0], expected[1]]);
            }
        }
    }
}
