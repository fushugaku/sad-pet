//! Vocal timbres sing a score with the `sad-pet-voice` engine. Phrases render
//! off the audio thread (a native thread, or a browser Worker) into clips on
//! the song timeline; the audio thread only mixes the clips at the song
//! position and feeds the result through the timbre's chain, which works like
//! a Bus: everything starts off and the voice passes through unchanged.
pub use sad_pet_voice as voice;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use voice::bank::Voicebank;
use voice::render::{self, Phrase, Voice};
use voice::score::{Score, Timeline};

/// What a Vocal timbre saves with its preset.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub score: Score,
}
impl Settings {
    pub fn sanitize(&mut self) {
        self.score.sanitize();
    }
}

/// A rendered phrase on the song timeline, in frames of the engine's rate.
#[derive(Clone, Debug)]
pub struct Clip {
    pub key: u64,
    pub start: u64,
    pub samples: Arc<[f32]>,
}
impl Clip {
    pub fn end(&self) -> u64 {
        self.start + self.samples.len() as u64
    }
}

/// Every clip of one timbre, sorted by start.
#[derive(Clone, Debug, Default)]
pub struct Song {
    pub clips: Vec<Clip>,
}
impl Song {
    pub fn new(mut clips: Vec<Clip>) -> Self {
        clips.sort_by_key(|c| c.start);
        Self { clips }
    }
    /// Mixes the song into `out`, starting at `position`.
    pub fn fill(&self, position: u64, out: &mut [f32]) {
        out.fill(0.0);
        let end = position + out.len() as u64;
        // Clips are short and few; skip those that end before the block.
        let first = self
            .clips
            .partition_point(|c| c.start + (c.samples.len() as u64) < position);
        for clip in self.clips[first..].iter().take_while(|c| c.start < end) {
            if clip.end() <= position {
                continue;
            }
            let from = position.max(clip.start);
            let to = end.min(clip.end());
            let src = &clip.samples[(from - clip.start) as usize..(to - clip.start) as usize];
            let dst = &mut out[(from - position) as usize..(to - position) as usize];
            for (d, s) in dst.iter_mut().zip(src) {
                *d += s;
            }
        }
    }
}

/// Hands songs from a render thread to the audio thread without blocking
/// either: the audio thread only ever `try_lock`s, and the songs it replaces
/// go back to be dropped elsewhere.
#[derive(Debug, Default)]
pub struct Mailbox {
    incoming: Mutex<Option<Song>>,
    garbage: Mutex<Vec<Song>>,
    /// Song position the audio thread last played, so rendering starts there.
    pub position: std::sync::atomic::AtomicU64,
    /// The host's tempo as `f32` bits, 0 without a host transport.
    pub host_bpm: std::sync::atomic::AtomicU32,
}
impl Mailbox {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            garbage: Mutex::new(Vec::with_capacity(8)),
            ..Default::default()
        })
    }
    /// Render side: offers the newest song, replacing one not taken yet.
    pub fn post(&self, song: Song) {
        let previous = self.incoming.lock().map(|mut slot| slot.replace(song));
        drop(previous);
        self.collect();
    }
    /// Render side: drops what the audio thread gave back.
    pub fn collect(&self) {
        let old = self.garbage.lock().map(|mut g| std::mem::take(&mut *g));
        if let Ok(old) = old {
            drop(old);
            if let Ok(mut g) = self.garbage.lock() {
                g.reserve(8);
            }
        }
    }
}

/// The audio thread's view of one Vocal timbre.
#[derive(Debug, Default)]
pub struct Track {
    pub song: Song,
    pending_garbage: Option<Song>,
    pub mailbox: Option<Arc<Mailbox>>,
}
impl Track {
    pub fn new(mailbox: Arc<Mailbox>) -> Self {
        Self {
            mailbox: Some(mailbox),
            ..Default::default()
        }
    }
    /// Takes a newer song if one is waiting. Never blocks or frees memory.
    pub fn update(&mut self) {
        let Some(mailbox) = &self.mailbox else { return };
        if let Some(old) = self.pending_garbage.take()
            && let Err(old) = give_back(mailbox, old)
        {
            self.pending_garbage = Some(old);
            return;
        }
        let new = match mailbox.incoming.try_lock() {
            Ok(mut slot) => slot.take(),
            Err(_) => None,
        };
        if let Some(new) = new {
            let old = std::mem::replace(&mut self.song, new);
            if let Err(old) = give_back(mailbox, old) {
                self.pending_garbage = Some(old);
            }
        }
    }
    /// Fills one block at a song position and reports the position.
    pub fn fill(&mut self, position: u64, out: &mut [f32]) {
        self.update();
        self.song.fill(position, out);
        if let Some(mailbox) = &self.mailbox {
            mailbox
                .position
                .store(position, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

fn give_back(mailbox: &Mailbox, song: Song) -> Result<(), Song> {
    match mailbox.garbage.try_lock() {
        Ok(mut g) if g.len() < g.capacity() => {
            g.push(song);
            Ok(())
        }
        _ => Err(song),
    }
}

/// The song clock of a scene without a host transport (browser, app).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Clock {
    pub playing: bool,
    /// Frames at the engine rate.
    pub position: u64,
    /// Where Play starts.
    pub start: u64,
}
impl Clock {
    pub fn play(&mut self, playing: bool) {
        if playing && !self.playing {
            self.position = self.start;
        }
        self.playing = playing;
    }
    pub fn advance(&mut self, frames: usize) {
        if self.playing {
            self.position += frames as u64;
        }
    }
}

/// What a render job needs to know about one timbre.
#[derive(Clone, Debug)]
pub struct Job {
    pub settings: Settings,
    pub rate: u32,
    /// The host's tempo replaces the score's when the score follows it.
    pub host_bpm: Option<f32>,
}

/// A planned song: its phrases and timeline.
#[derive(Clone, Debug)]
pub struct Plan {
    pub phrases: Vec<Phrase>,
    pub timeline: Timeline,
    pub rate: u32,
    pub voice_id: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Progress {
    pub phrases: usize,
    pub rendered: usize,
    /// The score asks for a voicebank that is not loaded; the built-in voice sings.
    pub missing_voice: bool,
}

/// Plans and renders phrases with a cache shared by every timbre, so edits
/// and tempo changes only render what changed. No threads of its own: a
/// native service thread or a browser Worker drives it.
#[derive(Default)]
pub struct Renderer {
    cache: HashMap<u64, Arc<[f32]>>,
    voices: HashMap<String, Arc<Voicebank>>,
    dictionary: Option<voice::g2p::en::Dictionary>,
}
impl Renderer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_voice(&mut self, id: &str, bank: Arc<Voicebank>) {
        self.voices.insert(id.to_string(), bank);
    }
    pub fn remove_voice(&mut self, id: &str) {
        self.voices.remove(id);
    }
    pub fn has_voice(&self, id: &str) -> bool {
        self.voices.contains_key(id)
    }
    pub fn voices(&self) -> impl Iterator<Item = (&String, &Arc<Voicebank>)> {
        self.voices.iter()
    }
    pub fn set_dictionary(&mut self, dictionary: Option<voice::g2p::en::Dictionary>) {
        self.dictionary = dictionary;
    }
    fn voice(&self, id: &str) -> Voice {
        self.voices
            .get(id)
            .map_or(Voice::Formant, |b| Voice::Bank(b.clone()))
    }
    pub fn plan(&self, job: &Job) -> Plan {
        let timeline = job.settings.score.timeline(job.host_bpm);
        let voice_id = if self.voices.contains_key(&job.settings.score.voice) {
            job.settings.score.voice.clone()
        } else {
            String::new()
        };
        Plan {
            phrases: render::plan(
                &job.settings.score,
                &timeline,
                &voice_id,
                job.rate,
                self.dictionary.as_ref(),
            ),
            timeline,
            rate: job.rate,
            voice_id,
        }
    }
    pub fn progress(&self, job: &Job, plan: &Plan) -> Progress {
        Progress {
            phrases: plan.phrases.len(),
            rendered: plan
                .phrases
                .iter()
                .filter(|p| self.cache.contains_key(&p.key))
                .count(),
            missing_voice: !job.settings.score.voice.is_empty() && plan.voice_id.is_empty(),
        }
    }
    /// Renders the missing phrase nearest after `near` (frames), else the
    /// nearest before it. Returns false when nothing is missing.
    pub fn render_next(&mut self, job: &Job, plan: &Plan, near: u64) -> bool {
        let rate = plan.rate as f64;
        let near = near as f64 / rate;
        let next = plan
            .phrases
            .iter()
            .filter(|p| !self.cache.contains_key(&p.key))
            .min_by(|a, b| {
                let score = |p: &&Phrase| {
                    if p.end >= near {
                        p.start - near
                    } else {
                        1e6 + near - p.end
                    }
                };
                score(a).total_cmp(&score(b))
            });
        let Some(phrase) = next else { return false };
        let voice = self.voice(&plan.voice_id);
        let samples = render::render(
            &job.settings.score,
            &plan.timeline,
            phrase,
            &voice,
            plan.rate,
        );
        self.cache.insert(phrase.key, samples.into());
        let _ = rate;
        true
    }
    pub fn render_all(&mut self, job: &Job, plan: &Plan) {
        while self.render_next(job, plan, 0) {}
    }
    /// The song of every rendered phrase of a plan.
    pub fn song(&self, plan: &Plan) -> Song {
        Song::new(
            plan.phrases
                .iter()
                .filter_map(|p| {
                    let samples = self.cache.get(&p.key)?;
                    Some(Clip {
                        key: p.key,
                        start: (p.start * plan.rate as f64).round() as u64,
                        samples: samples.clone(),
                    })
                })
                .collect(),
        )
    }
    /// Forgets phrases no current plan uses.
    pub fn retain(&mut self, plans: &[&Plan]) {
        let keep: HashSet<u64> = plans
            .iter()
            .flat_map(|p| p.phrases.iter().map(|p| p.key))
            .collect();
        self.cache.retain(|k, _| keep.contains(k));
    }
    pub fn cached(&self, key: u64) -> Option<&Arc<[f32]>> {
        self.cache.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use voice::score::{Note, TICKS_PER_BEAT};

    fn settings() -> Settings {
        let mut score = Score::default();
        for (i, lyric) in ["ra", "ra", "ra"].iter().enumerate() {
            score.notes.push(Note {
                id: i as u32 + 1,
                start: TICKS_PER_BEAT * (1 + i as u32 * 4),
                length: TICKS_PER_BEAT,
                key: 64,
                lyric: lyric.to_string(),
                ..Default::default()
            });
        }
        Settings { score }
    }

    #[test]
    fn renders_phrases_into_a_song_and_reuses_them() {
        let job = Job {
            settings: settings(),
            rate: 24_000,
            host_bpm: None,
        };
        let mut renderer = Renderer::new();
        let plan = renderer.plan(&job);
        assert_eq!(plan.phrases.len(), 3);
        assert!(renderer.render_next(&job, &plan, 24_000 * 3));
        // The phrase at 2.5 s (beat 5) renders first from a position of 3 s.
        let song = renderer.song(&plan);
        assert_eq!(song.clips.len(), 1);
        assert!(song.clips[0].start > 24_000);
        renderer.render_all(&job, &plan);
        assert_eq!(renderer.progress(&job, &plan).rendered, 3);
        // Moving one note re-renders only its phrase.
        let mut moved = job.clone();
        moved.settings.score.notes[2].key = 67;
        let plan2 = renderer.plan(&moved);
        assert_eq!(renderer.progress(&moved, &plan2).rendered, 2);
        renderer.retain(&[&plan2]);
        assert_eq!(renderer.cache.len(), 2);
    }

    #[test]
    fn songs_mix_clips_at_the_position() {
        let song = Song::new(vec![
            Clip {
                key: 1,
                start: 10,
                samples: vec![1.0; 10].into(),
            },
            Clip {
                key: 2,
                start: 15,
                samples: vec![0.5; 10].into(),
            },
        ]);
        let mut out = [9.0; 20];
        song.fill(5, &mut out);
        assert_eq!(out[4], 0.0);
        assert_eq!(out[5], 1.0);
        assert_eq!(out[10], 1.5);
        assert_eq!(out[19], 0.5);
    }

    #[test]
    fn the_audio_thread_swaps_songs_without_dropping_them() {
        let mailbox = Mailbox::new();
        let mut track = Track::new(mailbox.clone());
        mailbox.post(Song::new(vec![Clip {
            key: 1,
            start: 0,
            samples: vec![1.0; 4].into(),
        }]));
        let mut out = [0.0; 4];
        track.fill(0, &mut out);
        assert_eq!(out, [1.0; 4]);
        mailbox.post(Song::default());
        track.fill(0, &mut out);
        assert_eq!(out, [0.0; 4]);
        // The replaced song waits in the garbage for the render side.
        assert_eq!(mailbox.garbage.lock().unwrap().len(), 1);
        mailbox.collect();
        assert!(mailbox.garbage.lock().unwrap().is_empty());
        assert_eq!(
            mailbox.position.load(std::sync::atomic::Ordering::Relaxed),
            0
        );
    }
}
