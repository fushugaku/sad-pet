//! Vocal timbres sing a score with the `sad-pet-voice` engine. Phrases render
//! off the audio thread (a native thread, or a browser Worker) into clips on
//! the song timeline; the audio thread only mixes the clips at the song
//! position and feeds the result through the timbre's chain, which works like
//! a Bus: everything starts off and the voice passes through unchanged.
pub use sad_pet_voice as voice;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use voice::bank::Voicebank;
use voice::render::{self, Phrase, Voice};
use voice::score::{Score, Tempo, Timeline};

/// Voicebanks that come with SAD PET, by id and language. The plug-in and
/// the web build carry their files, from `voice/packs`.
pub const PACKS: [(&str, voice::Language); 3] = [
    ("kasane-teto", voice::Language::Ja),
    ("kasane-teto-en", voice::Language::En),
    ("kumi-ru", voice::Language::Ru),
];

/// The voicebank that comes with SAD PET for a language.
pub fn pack(language: voice::Language) -> &'static str {
    PACKS
        .iter()
        .find(|(_, l)| *l == language)
        .map_or("", |(id, _)| id)
}

/// What a Vocal timbre saves with its preset.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub score: Score,
}
impl Settings {
    /// A new timbre's score, sung by the voicebank that comes with SAD PET
    /// for its language.
    pub fn new() -> Self {
        let mut score = Score::default();
        score.voice = pack(score.language).to_string();
        Self { score }
    }
    pub fn sanitize(&mut self) {
        self.score.sanitize();
    }
}

/// A rendered phrase on the song timeline, in frames of the engine's rate.
/// In a looping song a phrase may start before 0: its lead-in sounds at the
/// end of the pass before.
#[derive(Clone, Debug)]
pub struct Clip {
    pub key: u64,
    pub start: i64,
    pub samples: Arc<[f32]>,
}
impl Clip {
    pub fn end(&self) -> i64 {
        self.start + self.samples.len() as i64
    }
}

/// A looping song's region, in frames: playback goes from `end` back to
/// `start`. `lead` bounds how early a phrase starts before its first note.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Loop {
    pub start: i64,
    pub end: i64,
    pub lead: i64,
}

/// The frames a score loops over, if it does.
pub fn looping(score: &Score, host: Option<&[Tempo]>, rate: u32) -> Option<Loop> {
    if score.loop_ticks == 0 {
        return None;
    }
    let timeline = score.timeline(host);
    let frames = |tick: u32| (timeline.seconds(tick as f64) * rate as f64).round() as i64;
    let (start, end) = (frames(score.loop_start), frames(score.loop_ticks));
    let lead = (render::LEAD * rate as f64).ceil() as i64;
    (end > start).then_some(Loop { start, end, lead })
}

/// Every clip of one timbre, sorted by start.
#[derive(Clone, Debug, Default)]
pub struct Song {
    pub clips: Vec<Clip>,
    /// The score's loop, in frames.
    pub looping: Option<Loop>,
}
impl Song {
    pub fn new(mut clips: Vec<Clip>) -> Self {
        clips.sort_by_key(|c| c.start);
        Self {
            clips,
            looping: None,
        }
    }
    /// Replaces the clips and keeps the loop.
    pub fn set_clips(&mut self, clips: Vec<Clip>) {
        let looping = self.looping;
        *self = Self::new(clips);
        self.looping = looping;
    }
    /// Where a song position falls in the score.
    pub fn wrap(&self, position: i64) -> i64 {
        match self.looping {
            Some(l) if position >= l.end => l.start + (position - l.start) % (l.end - l.start),
            _ => position,
        }
    }
    /// Frames of lead-in before the song's start (a first note's consonants),
    /// for playback from 0 to begin with.
    pub fn lead_in(&self) -> u64 {
        self.clips.first().map_or(0, |c| (-c.start).max(0) as u64)
    }
    /// Mixes the song into `out`, starting at `position`. A looping song
    /// plays up to its loop's end, then the loop over and over: what rings
    /// past the end sounds over the start of the next pass, and the lead-in
    /// of the loop's first phrase over the end of the pass before.
    pub fn fill(&self, position: i64, out: &mut [f32]) {
        out.fill(0.0);
        let Some(l) = self.looping else {
            self.mix(position, out, |_| true);
            return;
        };
        let length = l.end - l.start;
        let mut done = 0;
        while done < out.len() {
            let at = position + done as i64;
            if at < l.start {
                // Before the loop: the intro, or the lead-in before 0.
                let count = ((l.start - at) as usize).min(out.len() - done);
                self.mix(at, &mut out[done..done + count], |_| true);
                done += count;
                continue;
            }
            let from = self.wrap(at);
            let count = ((l.end - from) as usize).min(out.len() - done);
            let block = &mut out[done..done + count];
            self.mix(from, block, |_| true);
            if at >= l.end {
                // The pass before rings on.
                self.mix(from + length, block, |c| c.start < l.end);
            }
            // The next pass's first phrase leads in.
            self.mix(from - length, block, |c| {
                c.start < l.start && c.start >= l.start - l.lead && c.end() > l.start
            });
            done += count;
        }
    }
    /// Adds the clips `take` accepts, over `out.len()` frames from
    /// `position`, into `out`.
    fn mix(&self, position: i64, out: &mut [f32], take: impl Fn(&Clip) -> bool) {
        let end = position + out.len() as i64;
        // Clips are short and few; skip those that end before the block.
        let first = self.clips.partition_point(|c| c.end() < position);
        for clip in self.clips[first..].iter().take_while(|c| c.start < end) {
            if clip.end() <= position || !take(clip) {
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
    /// The host's tempo map as the audio thread has learned it (empty without
    /// a host transport), and a count of its changes.
    tempo: Mutex<Vec<Tempo>>,
    tempo_version: std::sync::atomic::AtomicU64,
    /// The tempo version whose song was posted with every phrase rendered;
    /// `u64::MAX` while phrases are missing. Offline bounces wait for it.
    complete: std::sync::atomic::AtomicU64,
}
impl Mailbox {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            garbage: Mutex::new(Vec::with_capacity(8)),
            tempo: Mutex::new(Vec::with_capacity(TempoMap::CAPACITY)),
            complete: u64::MAX.into(),
            ..Default::default()
        })
    }
    /// Audio side: hands over a newer tempo map. Never blocks or allocates;
    /// false when the render side holds the map (try again next block).
    pub fn publish_tempo(&self, map: &TempoMap) -> bool {
        let Ok(mut tempo) = self.tempo.try_lock() else {
            return false;
        };
        tempo.clear();
        let room = tempo.capacity();
        tempo.extend(map.entries.iter().take(room));
        self.tempo_version.fetch_add(1, Ordering::AcqRel);
        self.complete.store(u64::MAX, Ordering::Release);
        true
    }
    /// Render side: the host tempo map, if any, and its version.
    pub fn tempo(&self) -> (Option<Arc<[Tempo]>>, u64) {
        let Ok(tempo) = self.tempo.lock() else {
            return (None, 0);
        };
        let version = self.tempo_version.load(Ordering::Acquire);
        (
            (!tempo.is_empty()).then(|| tempo.as_slice().into()),
            version,
        )
    }
    /// Render side: the song just posted has every phrase of the plan made
    /// with tempo `version` (or misses some, with `None`).
    pub fn set_complete(&self, version: Option<u64>) {
        self.complete
            .store(version.unwrap_or(u64::MAX), Ordering::Release);
    }
    /// Audio side: whether the latest song has every phrase for the tempo
    /// map the audio thread published last.
    pub fn is_complete(&self) -> bool {
        self.complete.load(Ordering::Acquire) == self.tempo_version.load(Ordering::Acquire)
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
    /// Takes the newest song and tells whether it has every phrase of the
    /// latest score and tempo map: an offline bounce waits for that.
    pub fn complete(&mut self) -> bool {
        let complete = self.mailbox.as_ref().is_none_or(|m| m.is_complete());
        self.update();
        complete
            && self
                .mailbox
                .as_ref()
                .is_none_or(|m| matches!(m.incoming.try_lock(), Ok(slot) if slot.is_none()))
            && self.pending_garbage.is_none()
    }
    /// Fills one block at a song position and reports where in the song
    /// that is.
    pub fn fill(&mut self, position: i64, out: &mut [f32]) {
        self.update();
        self.song.fill(position, out);
        if let Some(mailbox) = &self.mailbox {
            mailbox.position.store(
                self.song.wrap(position).max(0) as u64,
                std::sync::atomic::Ordering::Relaxed,
            );
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
    /// Frames at the engine rate; below 0 while a first note leads in.
    pub position: i64,
    /// Where Play starts.
    pub start: u64,
}
impl Clock {
    /// Starts or stops. From the song's start, playback begins `lead_in`
    /// frames early, so a first note's consonants sound on the first pass.
    pub fn play(&mut self, playing: bool, lead_in: u64) {
        if playing && !self.playing {
            self.position = self.start as i64 - if self.start == 0 { lead_in as i64 } else { 0 };
        }
        self.playing = playing;
    }
    pub fn advance(&mut self, frames: usize) {
        if self.playing {
            self.position += frames as i64;
        }
    }
}

/// A host's tempo as the audio thread meets it: block by block it notes
/// where the tempo differs from what it knew, so phrases render on the same
/// map the song position is read with. Fixed capacity: it never allocates
/// after `new`.
#[derive(Clone, Debug)]
pub struct TempoMap {
    entries: Vec<Tempo>,
    version: u64,
}
impl Default for TempoMap {
    fn default() -> Self {
        Self::new()
    }
}
impl TempoMap {
    pub const CAPACITY: usize = 512;
    pub fn new() -> Self {
        Self {
            entries: Vec::with_capacity(Self::CAPACITY),
            version: 0,
        }
    }
    /// Counts the changes, so each listener can tell it has the latest.
    pub fn version(&self) -> u64 {
        self.version
    }
    /// Notes the tempo at a beat. Tempo changes land on a 1/16 grid and
    /// differences under 0.05 BPM are ignored, so ramps stay a few entries.
    pub fn observe(&mut self, beat: f64, bpm: f32) {
        if !(bpm.is_finite() && bpm > 0.0 && beat.is_finite()) {
            return;
        }
        let step = voice::score::TICKS_PER_BEAT / 4;
        let tick = (beat.max(0.0) * 4.0).floor() as u32 * step;
        let known = self.entries.iter().rev().find(|t| t.tick <= tick);
        if known.is_some_and(|t| (t.bpm - bpm).abs() < 0.05) {
            return;
        }
        if self.entries.is_empty() {
            // Before the first beat heard, the tempo was the same.
            self.entries.push(Tempo { tick: 0, bpm });
        } else {
            self.entries.retain(|t| t.tick < tick);
            if self.entries.len() < Self::CAPACITY {
                self.entries.push(Tempo { tick, bpm });
            }
        }
        self.version += 1;
    }
    /// Seconds at a beat on this map (the given tempo when it is empty).
    pub fn seconds(&self, beat: f64, bpm: f32) -> f64 {
        let tick = beat * voice::score::TICKS_PER_BEAT as f64;
        let per_tick =
            |bpm: f32| 60.0 / (bpm.max(1.0) as f64 * voice::score::TICKS_PER_BEAT as f64);
        let Some(first) = self.entries.first() else {
            return tick * per_tick(bpm);
        };
        let mut seconds = 0.0;
        let mut at = (0.0, first.bpm);
        for t in &self.entries[1..] {
            if t.tick as f64 >= tick {
                break;
            }
            seconds += (t.tick as f64 - at.0) * per_tick(at.1);
            at = (t.tick as f64, t.bpm);
        }
        seconds + (tick - at.0) * per_tick(at.1)
    }
}

/// What a render job needs to know about one timbre.
#[derive(Clone, Debug)]
pub struct Job {
    pub settings: Settings,
    pub rate: u32,
    /// The host's tempo map (in ticks) replaces the score's when the score
    /// follows it.
    pub host_tempo: Option<Arc<[Tempo]>>,
}

/// A planned song: its phrases and timeline.
#[derive(Clone, Debug)]
pub struct Plan {
    pub phrases: Vec<Phrase>,
    pub timeline: Timeline,
    pub rate: u32,
    pub voice_id: String,
    /// What the phrases sing: one pass of a looping score.
    pub score: Score,
    pub looping: Option<Loop>,
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
        let score = job.settings.score.one_pass().into_owned();
        let timeline = score.timeline(job.host_tempo.as_deref());
        let voice_id = if self.voices.contains_key(&job.settings.score.voice) {
            job.settings.score.voice.clone()
        } else {
            String::new()
        };
        Plan {
            phrases: render::plan(
                &score,
                &timeline,
                &voice_id,
                job.rate,
                self.dictionary.as_ref(),
            ),
            timeline,
            rate: job.rate,
            voice_id,
            looping: looping(&score, job.host_tempo.as_deref(), job.rate),
            score,
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
    pub fn render_next(&mut self, plan: &Plan, near: u64) -> bool {
        let near = near as f64 / plan.rate as f64;
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
        let samples = render::render(&plan.score, &plan.timeline, phrase, &voice, plan.rate);
        self.cache.insert(phrase.key, samples.into());
        true
    }
    pub fn render_all(&mut self, plan: &Plan) {
        while self.render_next(plan, 0) {}
    }
    /// The song of every rendered phrase of a plan.
    pub fn song(&self, plan: &Plan) -> Song {
        let mut song = Song::new(
            plan.phrases
                .iter()
                .filter_map(|p| {
                    let samples = self.cache.get(&p.key)?;
                    Some(Clip {
                        key: p.key,
                        start: (p.start * plan.rate as f64).round() as i64,
                        samples: samples.clone(),
                    })
                })
                .collect(),
        );
        song.looping = plan.looping;
        song
    }
    /// For each note of a plan (by id): the voicebank units it is sung with
    /// and others that could sing it. Nothing for the built-in voice.
    pub fn units(&self, plan: &Plan) -> Vec<(u32, voice::concat::NoteUnits)> {
        let voice = self.voice(&plan.voice_id);
        plan.phrases
            .iter()
            .flat_map(|p| render::units(&plan.score, &plan.timeline, p, &voice))
            .map(|(n, units)| (plan.score.notes[n].id, units))
            .collect()
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
            host_tempo: None,
        };
        let mut renderer = Renderer::new();
        let plan = renderer.plan(&job);
        assert_eq!(plan.phrases.len(), 3);
        assert!(renderer.render_next(&plan, 24_000 * 3));
        // The phrase at 2.5 s (beat 5) renders first from a position of 3 s.
        let song = renderer.song(&plan);
        assert_eq!(song.clips.len(), 1);
        assert!(song.clips[0].start > 24_000);
        renderer.render_all(&plan);
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
    fn the_tempo_map_learns_changes_and_reads_seconds_on_them() {
        let mut map = TempoMap::new();
        assert_eq!(map.seconds(4.0, 120.0), 2.0, "empty: the tempo given");
        map.observe(2.3, 120.0);
        map.observe(3.0, 120.02);
        assert_eq!(map.version(), 1, "tiny wobbles are not changes");
        // From beat 8 on, 60 BPM; heard a little after the beat.
        map.observe(8.1, 60.0);
        assert_eq!(map.version(), 2);
        assert!((map.seconds(10.0, 60.0) - (4.0 + 2.0)).abs() < 1e-9);
        assert!(
            (map.seconds(6.0, 60.0) - 3.0).abs() < 1e-9,
            "earlier beats keep their time"
        );
        // Played again from the start, the map is unchanged.
        map.observe(0.0, 120.0);
        assert_eq!(map.version(), 2);
        // A different tempo earlier forgets what came after it.
        map.observe(4.0, 90.0);
        assert!((map.seconds(10.0, 90.0) - (2.0 + 6.0 * 60.0 / 90.0)).abs() < 1e-9);
        // The render side gets the map in ticks; the song waits for it.
        let mailbox = Mailbox::new();
        mailbox.set_complete(Some(0));
        assert!(mailbox.is_complete());
        assert!(mailbox.publish_tempo(&map));
        assert!(!mailbox.is_complete(), "a new map needs a new render");
        let (tempo, version) = mailbox.tempo();
        assert_eq!(
            tempo.unwrap()[1],
            Tempo {
                tick: 1920,
                bpm: 90.0
            }
        );
        mailbox.set_complete(Some(version));
        assert!(mailbox.is_complete());
    }

    #[test]
    fn a_looping_song_repeats_and_rings_into_the_next_pass() {
        let mut song = Song::new(vec![Clip {
            key: 1,
            start: 6,
            samples: vec![1.0; 6].into(),
        }]);
        song.looping = Some(Loop {
            start: 0,
            end: 10,
            lead: 5,
        });
        let mut out = [9.0; 30];
        song.fill(0, &mut out);
        let expected = [
            0., 0., 0., 0., 0., 0., 1., 1., 1., 1., // first pass
            1., 1., 0., 0., 0., 0., 1., 1., 1., 1., // the tail, then again
            1., 1., 0., 0., 0., 0., 1., 1., 1., 1.,
        ];
        assert_eq!(out, expected);
        assert_eq!(song.wrap(23), 3);
        // A lead-in before 0 sounds at the end of every pass, the next one's.
        song.set_clips(vec![Clip {
            key: 2,
            start: -3,
            samples: vec![1.0; 5].into(),
        }]);
        assert_eq!(song.looping.map(|l| l.end), Some(10));
        song.fill(0, &mut out);
        let expected = [
            1., 1., 0., 0., 0., 0., 0., 1., 1., 1., // the rest of its own, then the next
            1., 1., 0., 0., 0., 0., 0., 1., 1., 1., //
            1., 1., 0., 0., 0., 0., 0., 1., 1., 1.,
        ];
        assert_eq!(out, expected);
        // Played from 0, the clock starts early enough for the first pass
        // to lead in too.
        assert_eq!(song.lead_in(), 3);
        let mut clock = Clock::default();
        clock.play(true, song.lead_in());
        assert_eq!(clock.position, -3);
        let mut first = [9.0; 5];
        song.fill(clock.position, &mut first);
        assert_eq!(first, [1.0; 5]);
    }

    #[test]
    fn an_intro_plays_once_before_the_loop() {
        // Intro clip at 0..4, the loop 10..20 with its phrase leading in from
        // 8 and ringing on to 22.
        let mut song = Song::new(vec![
            Clip {
                key: 1,
                start: 0,
                samples: vec![0.5; 4].into(),
            },
            Clip {
                key: 2,
                start: 8,
                samples: vec![1.0; 14].into(),
            },
        ]);
        song.looping = Some(Loop {
            start: 10,
            end: 20,
            lead: 3,
        });
        let mut out = [9.0; 40];
        song.fill(0, &mut out);
        let mut expected = [0.0f32; 40];
        expected[..4].fill(0.5); // the intro, once
        expected[8..20].fill(1.0); // lead-in and first pass
        expected[20..22].fill(2.0); // the tail over the next pass's start
        expected[22..30].fill(1.0);
        // The lead-in of each next pass over the last frames of the one before.
        expected[18..20].fill(2.0);
        expected[28..30].fill(2.0);
        expected[30..32].fill(2.0);
        expected[32..40].fill(1.0);
        expected[38..40].fill(2.0);
        assert_eq!(out, expected);
        assert_eq!(song.wrap(25), 15);
        assert_eq!(song.wrap(5), 5);
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
