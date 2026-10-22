//! Scores from other tools: Standard MIDI files (with lyric events), UTAU
//! `.ust`, MusicXML (`.musicxml`, `.xml`, compressed `.mxl`) and VOCALOID5
//! `.vpr` projects. Each becomes notes at 480 ticks per quarter note, a tempo
//! map and, where the file has them, lyrics, phonemes and curves.
use crate::phoneme::Language;
use crate::score::{Note, Param, Point, Score, TICKS_PER_BEAT, Tempo};

/// Reads a score by file name and contents.
pub fn score(name: &str, bytes: &[u8], language: Language) -> Result<Score, String> {
    let lower = name.to_lowercase();
    let mut score = if bytes.starts_with(b"MThd") {
        midi(bytes)?
    } else if lower.ends_with(".vpr") {
        vpr(bytes)?
    } else if lower.ends_with(".mxl") {
        mxl(bytes)?
    } else if lower.ends_with(".ust") || bytes.windows(9).take(4096).any(|w| w == b"[#SETTING") {
        ust(&crate::utau::decode_text(bytes))?
    } else if lower.ends_with(".json") {
        serde_json::from_slice(bytes).map_err(|e| format!("Score: {e}"))?
    } else {
        musicxml(&String::from_utf8_lossy(bytes))?
    };
    if !lower.ends_with(".vpr") && !lower.ends_with(".json") {
        score.language = detect_language(&score, language);
    }
    score.sanitize();
    if score.notes.is_empty() {
        return Err("The file has no notes".into());
    }
    Ok(score)
}

fn detect_language(score: &Score, fallback: Language) -> Language {
    let text: String = score.notes.iter().map(|n| n.lyric.as_str()).collect();
    crate::g2p::detect(&text, fallback)
}

fn note(id: usize, start: u32, length: u32, key: u8, lyric: String) -> Note {
    Note {
        id: id as u32 + 1,
        start,
        length,
        key,
        lyric,
        ..Default::default()
    }
}

// --- MIDI --------------------------------------------------------------------

/// One MIDI track: notes as (start, end, key) and lyrics as (tick, text).
type Track = (Vec<(u32, u32, u8)>, Vec<(u32, String)>);

struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}
impl Reader<'_> {
    fn byte(&mut self) -> Result<u8, String> {
        let b = *self.bytes.get(self.at).ok_or("Truncated MIDI file")?;
        self.at += 1;
        Ok(b)
    }
    fn take(&mut self, n: usize) -> Result<&[u8], String> {
        let slice = self
            .bytes
            .get(self.at..self.at + n)
            .ok_or("Truncated MIDI file")?;
        self.at += n;
        Ok(slice)
    }
    fn variable(&mut self) -> Result<u32, String> {
        let mut value = 0u32;
        for _ in 0..4 {
            let b = self.byte()?;
            value = (value << 7) | (b & 0x7F) as u32;
            if b & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err("Invalid MIDI length".into())
    }
}

/// A Standard MIDI file: the track with the most notes (preferring one with
/// lyrics) is sung; lyric (FF 05) or text (FF 01) events at a note's start
/// become its lyric. Overlapping notes are cut so one voice sings them.
pub fn midi(bytes: &[u8]) -> Result<Score, String> {
    let mut r = Reader { bytes, at: 0 };
    if r.take(4)? != b"MThd" {
        return Err("Not a MIDI file".into());
    }
    let header_length = u32::from_be_bytes(r.take(4)?.try_into().unwrap()) as usize;
    let header = r.take(header_length)?;
    if header.len() < 6 {
        return Err("Invalid MIDI header".into());
    }
    let division = u16::from_be_bytes([header[4], header[5]]);
    if division & 0x8000 != 0 {
        return Err("SMPTE-timed MIDI files are not supported".into());
    }
    let scale = TICKS_PER_BEAT as f64 / division.max(1) as f64;
    let mut tempo: Vec<Tempo> = Vec::new();
    let mut tracks: Vec<Track> = Vec::new();
    while r.at + 8 <= bytes.len() {
        let id = r.take(4)?.to_vec();
        let length = u32::from_be_bytes(r.take(4)?.try_into().unwrap()) as usize;
        let end = (r.at + length).min(bytes.len());
        if id != b"MTrk" {
            r.at = end;
            continue;
        }
        let mut tick = 0u64;
        let mut status = 0u8;
        let mut open: [Option<u32>; 128] = [None; 128];
        let mut notes = Vec::new();
        let mut lyrics = Vec::new();
        while r.at < end {
            tick += r.variable()? as u64;
            let t = (tick as f64 * scale).round() as u32;
            let mut b = r.byte()?;
            if b == 0xFF {
                let kind = r.byte()?;
                let len = r.variable()? as usize;
                let data = r.take(len)?;
                match kind {
                    0x51 if len == 3 => {
                        let micros =
                            ((data[0] as u32) << 16) | ((data[1] as u32) << 8) | data[2] as u32;
                        tempo.push(Tempo {
                            tick: t,
                            bpm: 60_000_000.0 / micros.max(1) as f32,
                        });
                    }
                    0x05 | 0x01 => {
                        let text = crate::utau::decode_text(data).trim().to_string();
                        if !text.is_empty() && !text.starts_with('@') {
                            lyrics.push((t, text));
                        }
                    }
                    _ => {}
                }
                continue;
            }
            if b == 0xF0 || b == 0xF7 {
                let len = r.variable()? as usize;
                r.take(len)?;
                continue;
            }
            if b & 0x80 != 0 {
                status = b;
                b = r.byte()?;
            }
            let kind = status & 0xF0;
            let data1 = b;
            let data2 = if matches!(kind, 0xC0 | 0xD0) {
                0
            } else {
                r.byte()?
            };
            let key = (data1 & 0x7F) as usize;
            match kind {
                0x90 if data2 > 0 => open[key] = Some(t),
                0x80 | 0x90 => {
                    if let Some(start) = open[key].take()
                        && t > start
                    {
                        notes.push((start, t, key as u8));
                    }
                }
                _ => {}
            }
        }
        r.at = end;
        notes.sort();
        tracks.push((notes, lyrics));
    }
    let track = tracks
        .iter()
        .max_by_key(|(notes, lyrics)| (!lyrics.is_empty(), notes.len()))
        .ok_or("The MIDI file has no tracks")?;
    // Lyrics may live in another track (format 1 files often do that).
    let lyrics: Vec<(u32, String)> = if track.1.is_empty() {
        tracks.iter().flat_map(|t| t.1.clone()).collect()
    } else {
        track.1.clone()
    };
    let mut notes: Vec<Note> = Vec::new();
    for (i, &(start, end, key)) in track.0.iter().enumerate() {
        let lyric = lyrics
            .iter()
            .find(|(t, _)| t.abs_diff(start) <= 10)
            .map(|(_, l)| l.clone())
            .unwrap_or_default();
        notes.push(note(i, start, end - start, key, lyric));
    }
    for i in 1..notes.len() {
        let next = notes[i].start;
        let previous = &mut notes[i - 1];
        if previous.end() > next {
            previous.length = next.saturating_sub(previous.start).max(1);
        }
    }
    notes.dedup_by(|b, a| a.start == b.start);
    if tempo.is_empty() {
        tempo.push(Tempo {
            tick: 0,
            bpm: 120.0,
        });
    }
    Ok(Score {
        notes,
        tempo,
        ..Default::default()
    })
}

// --- UST ---------------------------------------------------------------------

/// UTAU sequence text: `[#SETTING]` with `Tempo`, then `[#0000]` notes with
/// `Length` (480 per quarter), `Lyric` (R is a rest) and `NoteNum`.
pub fn ust(text: &str) -> Result<Score, String> {
    let mut tempo = vec![Tempo {
        tick: 0,
        bpm: 120.0,
    }];
    let mut notes = Vec::new();
    let mut tick = 0u32;
    let mut section = String::new();
    let mut fields: Vec<(String, String)> = Vec::new();
    let mut flush = |section: &str, fields: &mut Vec<(String, String)>, tick: &mut u32| {
        let get = |k: &str| fields.iter().find(|(f, _)| f == k).map(|(_, v)| v.as_str());
        if section == "#SETTING" {
            if let Some(bpm) = get("Tempo").and_then(|v| v.replace(',', ".").parse::<f32>().ok()) {
                tempo[0].bpm = bpm;
            }
        } else if section.starts_with('#') && section[1..].chars().all(|c| c.is_ascii_digit()) {
            let length = get("Length")
                .and_then(|v| v.parse::<u32>().ok())
                .unwrap_or(0);
            if let Some(bpm) = get("Tempo").and_then(|v| v.replace(',', ".").parse::<f32>().ok())
                && tempo.last().is_none_or(|t| t.bpm != bpm)
            {
                tempo.push(Tempo { tick: *tick, bpm });
            }
            let lyric = get("Lyric").unwrap_or("").trim().to_string();
            let key = get("NoteNum")
                .and_then(|v| v.parse::<u8>().ok())
                .unwrap_or(60);
            let rest = lyric.is_empty() || matches!(lyric.as_str(), "R" | "r" | "rest");
            if !rest && length > 0 {
                // VCV lyrics such as "a か" are sung as their last syllable.
                let lyric = lyric.rsplit(' ').next().unwrap_or(&lyric).to_string();
                notes.push(note(notes.len(), *tick, length, key, lyric));
            }
            *tick += length;
        }
        fields.clear();
    };
    for line in text.lines() {
        let line = line.trim();
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            flush(&section, &mut fields, &mut tick);
            section = name.to_string();
        } else if let Some((key, value)) = line.split_once('=') {
            fields.push((key.trim().to_string(), value.trim().to_string()));
        }
    }
    flush(&section, &mut fields, &mut tick);
    tempo.sort_by_key(|t| t.tick);
    Ok(Score {
        notes,
        tempo,
        ..Default::default()
    })
}

// --- MusicXML ----------------------------------------------------------------

fn child<'a, 'input>(
    n: roxmltree::Node<'a, 'input>,
    name: &str,
) -> Option<roxmltree::Node<'a, 'input>> {
    n.children().find(|c| c.has_tag_name(name))
}
fn text_of<'a>(n: Option<roxmltree::Node<'a, '_>>) -> &'a str {
    n.and_then(|n| n.text()).map(str::trim).unwrap_or("")
}

fn mxl(bytes: &[u8]) -> Result<Score, String> {
    let files = crate::zip::unzip(bytes)?;
    let container = files
        .0
        .get("META-INF/container.xml")
        .map(|b| String::from_utf8_lossy(b).into_owned());
    let root = container.as_deref().and_then(|c| {
        let doc = roxmltree::Document::parse(c).ok()?;
        doc.descendants()
            .find(|n| n.has_tag_name("rootfile"))
            .and_then(|n| n.attribute("full-path"))
            .map(str::to_string)
    });
    let path = root
        .or_else(|| {
            files
                .0
                .keys()
                .find(|k| {
                    (k.ends_with(".xml") || k.ends_with(".musicxml")) && !k.starts_with("META-INF")
                })
                .cloned()
        })
        .ok_or("The .mxl has no score")?;
    let xml = files.0.get(&path).ok_or("The .mxl has no score")?;
    musicxml(&String::from_utf8_lossy(xml))
}

/// The first part's first voice: pitches, durations (ties joined), lyrics
/// with their syllabic hyphenation, and `<sound tempo>` changes.
pub fn musicxml(text: &str) -> Result<Score, String> {
    let doc = roxmltree::Document::parse(text).map_err(|e| format!("MusicXML: {e}"))?;
    let part = doc
        .descendants()
        .find(|n| n.has_tag_name("part"))
        .ok_or("The MusicXML file has no part")?;
    let mut divisions = 1.0f64;
    let mut tick = 0.0f64;
    let mut tempo: Vec<Tempo> = Vec::new();
    let mut notes: Vec<Note> = Vec::new();
    let mut tied = false;
    for measure in part.children().filter(|n| n.has_tag_name("measure")) {
        for item in measure.children().filter(|n| n.is_element()) {
            match item.tag_name().name() {
                "attributes" => {
                    if let Ok(d) = text_of(child(item, "divisions")).parse::<f64>() {
                        divisions = d.max(1.0);
                    }
                }
                "direction" | "sound" => {
                    let sound = if item.has_tag_name("sound") {
                        Some(item)
                    } else {
                        item.descendants().find(|n| n.has_tag_name("sound"))
                    };
                    if let Some(bpm) = sound
                        .and_then(|s| s.attribute("tempo"))
                        .and_then(|t| t.parse::<f32>().ok())
                    {
                        tempo.push(Tempo {
                            tick: tick.round() as u32,
                            bpm,
                        });
                    }
                }
                "backup" => {
                    let d: f64 = text_of(child(item, "duration")).parse().unwrap_or(0.0);
                    tick -= d / divisions * TICKS_PER_BEAT as f64;
                }
                "forward" => {
                    let d: f64 = text_of(child(item, "duration")).parse().unwrap_or(0.0);
                    tick += d / divisions * TICKS_PER_BEAT as f64;
                }
                "note" => {
                    let voice = text_of(child(item, "voice"));
                    if !voice.is_empty() && voice != "1" {
                        // Other voices are skipped; their time is undone by <backup>.
                        if child(item, "chord").is_none() {
                            let d: f64 = text_of(child(item, "duration")).parse().unwrap_or(0.0);
                            tick += d / divisions * TICKS_PER_BEAT as f64;
                        }
                        continue;
                    }
                    if child(item, "chord").is_some() || child(item, "grace").is_some() {
                        continue;
                    }
                    let d: f64 = text_of(child(item, "duration")).parse().unwrap_or(0.0);
                    let length = d / divisions * TICKS_PER_BEAT as f64;
                    let start = tick;
                    tick += length;
                    let Some(pitch) = child(item, "pitch") else {
                        tied = false;
                        continue;
                    };
                    let step = text_of(child(pitch, "step"));
                    let alter: f64 = text_of(child(pitch, "alter")).parse().unwrap_or(0.0);
                    let octave: i32 = text_of(child(pitch, "octave")).parse().unwrap_or(4);
                    let base = match step {
                        "C" => 0,
                        "D" => 2,
                        "E" => 4,
                        "F" => 5,
                        "G" => 7,
                        "A" => 9,
                        "B" => 11,
                        _ => 0,
                    };
                    let key = ((octave + 1) * 12 + base + alter.round() as i32).clamp(0, 127) as u8;
                    let ties: Vec<&str> = item
                        .children()
                        .filter(|n| n.has_tag_name("tie"))
                        .filter_map(|n| n.attribute("type"))
                        .collect();
                    if tied
                        && ties.contains(&"stop")
                        && let Some(last) = notes.last_mut()
                        && last.key == key
                    {
                        last.length = (tick.round() as u32).saturating_sub(last.start);
                        tied = ties.contains(&"start");
                        continue;
                    }
                    tied = ties.contains(&"start");
                    let lyric = item.children().find(|n| n.has_tag_name("lyric"));
                    let mut text = lyric
                        .map(|l| text_of(child(l, "text")).to_string())
                        .unwrap_or_default();
                    let syllabic = lyric.map(|l| text_of(child(l, "syllabic"))).unwrap_or("");
                    if matches!(syllabic, "begin" | "middle") && !text.is_empty() {
                        text.push('-');
                    }
                    if lyric.is_none() && !notes.is_empty() {
                        // A note without a lyric continues the previous vowel.
                        text = "-".into();
                    }
                    notes.push(note(
                        notes.len(),
                        start.round() as u32,
                        length.round().max(1.0) as u32,
                        key,
                        text,
                    ));
                }
                _ => {}
            }
        }
    }
    if tempo.is_empty() || tempo[0].tick != 0 {
        let bpm = tempo.first().map_or(120.0, |t| t.bpm);
        tempo.insert(0, Tempo { tick: 0, bpm });
    }
    Ok(Score {
        notes,
        tempo,
        ..Default::default()
    })
}

// --- VOCALOID5 -------------------------------------------------------------

/// A `.vpr` project: `Project/sequence.json` inside a zip. The first vocal
/// track's notes (part positions added), its tempo, and the controller curves
/// with VOCALOID5 names this engine shares.
pub fn vpr(bytes: &[u8]) -> Result<Score, String> {
    let files = crate::zip::unzip(bytes)?;
    let json = files
        .0
        .iter()
        .find(|(k, _)| k.ends_with("sequence.json"))
        .map(|(_, v)| v)
        .ok_or("The .vpr has no sequence.json")?;
    let doc: serde_json::Value = serde_json::from_slice(json).map_err(|e| format!(".vpr: {e}"))?;
    let mut tempo: Vec<Tempo> = doc["masterTrack"]["tempo"]["events"]
        .as_array()
        .map(|events| {
            events
                .iter()
                .filter_map(|e| {
                    Some(Tempo {
                        tick: e["pos"].as_u64()? as u32,
                        bpm: e["value"].as_f64()? as f32 / 100.0,
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    if tempo.is_empty() {
        tempo.push(Tempo {
            tick: 0,
            bpm: 120.0,
        });
    }
    let track = doc["tracks"]
        .as_array()
        .and_then(|tracks| {
            tracks.iter().find(|t| {
                t["parts"]
                    .as_array()
                    .is_some_and(|p| p.iter().any(|p| p["notes"].is_array()))
            })
        })
        .ok_or("The .vpr has no vocal track")?;
    let mut notes = Vec::new();
    let mut curves: std::collections::BTreeMap<Param, Vec<Point>> = Default::default();
    let mut language = Language::Ja;
    for part in track["parts"].as_array().into_iter().flatten() {
        let offset = part["pos"].as_u64().unwrap_or(0) as u32;
        if part["voice"]["langID"].as_u64() == Some(1) {
            language = Language::En;
        }
        for n in part["notes"].as_array().into_iter().flatten() {
            let mut item = note(
                notes.len(),
                offset + n["pos"].as_u64().unwrap_or(0) as u32,
                n["duration"].as_u64().unwrap_or(480) as u32,
                n["number"].as_u64().unwrap_or(60).min(127) as u8,
                n["lyric"].as_str().unwrap_or("").to_string(),
            );
            item.velocity = n["velocity"].as_u64().unwrap_or(64).min(127) as u8;
            if n["isProtected"].as_bool() == Some(true)
                && let Some(phoneme) = n["phoneme"].as_str()
            {
                item.phonemes = Some(phoneme.to_string());
            }
            notes.push(item);
        }
        for controller in part["controllers"].as_array().into_iter().flatten() {
            let param = match controller["name"].as_str().unwrap_or("") {
                "dynamics" => Param::Dynamics,
                "pitchBend" => Param::PitchBend,
                "pitchBendSens" => Param::PitchBendSensitivity,
                "breathiness" => Param::Breathiness,
                "brightness" => Param::Brightness,
                "clearness" => Param::Clearness,
                "character" | "genderFactor" => Param::Character,
                "portamento" => Param::PortamentoTiming,
                "growl" => Param::Growl,
                "exciter" => Param::Exciter,
                "air" => Param::Air,
                "mouth" => Param::Mouth,
                _ => continue,
            };
            let curve = curves.entry(param).or_default();
            for e in controller["events"].as_array().into_iter().flatten() {
                if let (Some(pos), Some(value)) = (e["pos"].as_u64(), e["value"].as_f64()) {
                    curve.push(Point {
                        tick: offset + pos as u32,
                        value: value as f32,
                    });
                }
            }
        }
    }
    // Phonemes in VOCALOID5's symbols are kept only when they parse here.
    for n in &mut notes {
        if let Some(p) = &n.phonemes
            && crate::phoneme::Phoneme::parse_list(language, p).is_err()
        {
            n.phonemes = None;
        }
    }
    Ok(Score {
        notes,
        tempo,
        language,
        curves,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn midi_with_lyrics_and_tempo() {
        // Format 0, 96 ticks per quarter, 100 BPM, two notes with lyrics.
        let mut track = Vec::new();
        track.extend_from_slice(&[0x00, 0xFF, 0x51, 0x03, 0x09, 0x27, 0xC0]); // 600000 µs
        track.extend_from_slice(&[0x00, 0xFF, 0x05, 0x03]);
        track.extend_from_slice("さ".as_bytes());
        track.extend_from_slice(&[0x00, 0x90, 60, 100]);
        track.extend_from_slice(&[0x60, 0x80, 60, 0]);
        track.extend_from_slice(&[0x00, 0xFF, 0x05, 0x03]);
        track.extend_from_slice("く".as_bytes());
        track.extend_from_slice(&[0x00, 0x90, 62, 100]);
        track.extend_from_slice(&[0x60, 0x90, 62, 0]);
        track.extend_from_slice(&[0x00, 0xFF, 0x2F, 0x00]);
        let mut file = b"MThd\0\0\0\x06\0\0\0\x01\0\x60MTrk".to_vec();
        file.extend_from_slice(&(track.len() as u32).to_be_bytes());
        file.extend_from_slice(&track);
        let score = score("song.mid", &file, Language::En).unwrap();
        assert_eq!(score.language, Language::Ja);
        assert_eq!(score.tempo[0].bpm, 100.0);
        assert_eq!(score.notes.len(), 2);
        assert_eq!(
            (
                score.notes[1].start,
                score.notes[1].length,
                score.notes[1].key
            ),
            (480, 480, 62)
        );
        assert_eq!(score.notes[1].lyric, "く");
    }

    #[test]
    fn ust_sections() {
        let text = "[#VERSION]\nUST Version1.2\n[#SETTING]\nTempo=140.00\n[#0000]\nLength=480\nLyric=R\nNoteNum=60\n[#0001]\nLength=960\nLyric=a か\nNoteNum=67\n[#TRACKEND]\n";
        let (bytes, _, _) = encoding_rs::SHIFT_JIS.encode(text);
        let score = score("song.ust", &bytes, Language::Ja).unwrap();
        assert_eq!(score.tempo[0].bpm, 140.0);
        assert_eq!(score.notes.len(), 1);
        assert_eq!((score.notes[0].start, score.notes[0].length), (480, 960));
        assert_eq!(score.notes[0].lyric, "か");
    }

    #[test]
    fn musicxml_ties_and_syllables() {
        let xml = r#"<?xml version="1.0"?><score-partwise><part id="P1"><measure number="1">
<attributes><divisions>2</divisions></attributes>
<direction><sound tempo="90"/></direction>
<note><pitch><step>C</step><octave>4</octave></pitch><duration>2</duration><voice>1</voice>
<lyric><syllabic>begin</syllabic><text>hap</text></lyric></note>
<note><pitch><step>D</step><alter>1</alter><octave>4</octave></pitch><duration>2</duration><voice>1</voice><tie type="start"/>
<lyric><syllabic>end</syllabic><text>py</text></lyric></note>
<note><pitch><step>D</step><alter>1</alter><octave>4</octave></pitch><duration>4</duration><voice>1</voice><tie type="stop"/></note>
<note><rest/><duration>2</duration><voice>1</voice></note>
<note><pitch><step>E</step><octave>4</octave></pitch><duration>2</duration><voice>1</voice>
<lyric><text>day</text></lyric></note>
</measure></part></score-partwise>"#;
        let score = score("song.musicxml", xml.as_bytes(), Language::En).unwrap();
        assert_eq!(score.tempo[0].bpm, 90.0);
        let summary: Vec<_> = score
            .notes
            .iter()
            .map(|n| (n.start, n.length, n.key, n.lyric.as_str()))
            .collect();
        assert_eq!(
            summary,
            [
                (0, 480, 60, "hap-"),
                (480, 1440, 63, "py"),
                (2400, 480, 64, "day")
            ]
        );
    }

    #[test]
    fn vpr_projects() {
        use std::io::Write;
        let sequence = r#"{"masterTrack":{"tempo":{"events":[{"pos":0,"value":13000}]}},
"tracks":[{"type":0,"parts":[{"pos":1920,"voice":{"langID":0},
"notes":[{"lyric":"ら","phoneme":"4 a","pos":0,"duration":480,"number":64,"velocity":80}],
"controllers":[{"name":"dynamics","events":[{"pos":0,"value":100}]}]}]}]}"#
            .as_bytes();
        // A stored (uncompressed) zip with one entry.
        let name = b"Project/sequence.json";
        let mut zip = Vec::new();
        zip.write_all(&0x0403_4b50u32.to_le_bytes()).unwrap();
        zip.write_all(&[20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
            .unwrap();
        zip.write_all(&(sequence.len() as u32).to_le_bytes())
            .unwrap();
        zip.write_all(&(sequence.len() as u32).to_le_bytes())
            .unwrap();
        zip.write_all(&(name.len() as u16).to_le_bytes()).unwrap();
        zip.write_all(&0u16.to_le_bytes()).unwrap();
        zip.write_all(name).unwrap();
        zip.write_all(sequence).unwrap();
        let central = zip.len() as u32;
        zip.write_all(&0x0201_4b50u32.to_le_bytes()).unwrap();
        zip.write_all(&[20, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
            .unwrap();
        zip.write_all(&(sequence.len() as u32).to_le_bytes())
            .unwrap();
        zip.write_all(&(sequence.len() as u32).to_le_bytes())
            .unwrap();
        zip.write_all(&(name.len() as u16).to_le_bytes()).unwrap();
        zip.write_all(&[0; 12]).unwrap();
        zip.write_all(&0u32.to_le_bytes()).unwrap();
        zip.write_all(name).unwrap();
        let size = zip.len() as u32 - central;
        zip.write_all(&0x0605_4b50u32.to_le_bytes()).unwrap();
        zip.write_all(&[0, 0, 0, 0, 1, 0, 1, 0]).unwrap();
        zip.write_all(&size.to_le_bytes()).unwrap();
        zip.write_all(&central.to_le_bytes()).unwrap();
        zip.write_all(&[0, 0]).unwrap();
        let score = score("song.vpr", &zip, Language::Ja).unwrap();
        assert_eq!(score.tempo[0].bpm, 130.0);
        assert_eq!(score.notes[0].start, 1920);
        assert_eq!(score.notes[0].velocity, 80);
        assert_eq!(score.curves[&Param::Dynamics][0].tick, 1920);
    }
}
