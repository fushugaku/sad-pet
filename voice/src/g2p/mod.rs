//! Lyrics to phonemes for Japanese, English and Russian.
pub mod en;
pub mod ja;
pub mod ru;

use crate::phoneme::{Language, Phoneme, Syllable, syllabify};

/// Syllables of a lyric in one language; unreadable Japanese falls back to
/// nothing, so the caller can sing the default vowel instead.
pub fn syllables(
    language: Language,
    text: &str,
    dictionary: Option<&en::Dictionary>,
) -> Vec<Syllable> {
    match language {
        Language::Ja => ja::syllables(text).unwrap_or_default(),
        Language::En => en::syllables(text, dictionary),
        Language::Ru => ru::syllables(text),
    }
}

/// Picks the language of a lyric from its script: kana means Japanese and
/// Cyrillic Russian; Latin text keeps the track language.
pub fn detect(text: &str, fallback: Language) -> Language {
    for c in text.chars() {
        match c {
            '\u{3040}'..='\u{30FF}' => return Language::Ja,
            '\u{0400}'..='\u{04FF}' => return Language::Ru,
            _ => {}
        }
    }
    fallback
}

/// Phonemes of a voicebank alias such as `- ka`, `a か`, `а к` or `k aa`.
/// A `-` token is silence; tokens are tried as the language's own phoneme
/// symbols as well as spelled text.
pub fn alias(language: Language, text: &str) -> Option<Vec<Phoneme>> {
    let mut out = Vec::new();
    for token in text.split_whitespace() {
        let parsed = match language {
            Language::Ja => ja::alias(token)
                .filter(|p| !p.is_empty())
                .or_else(|| Phoneme::parse(language, token).map(|p| vec![p])),
            Language::Ru => ru::alias(token),
            // ARPAsing aliases are phoneme symbols; spelling them as words
            // would read a layer suffix as letters.
            Language::En => Phoneme::parse(language, token).map(|p| vec![p]),
        };
        let parsed = match Phoneme::parse(language, token) {
            Some(p) if p.language().is_none() => Some(vec![p]),
            _ => parsed,
        };
        out.extend(parsed?);
    }
    Some(out)
}

/// What one note sings.
#[derive(Clone, Debug, PartialEq)]
pub enum NoteText {
    /// One or more syllables start on this note.
    Syllables(Vec<Syllable>),
    /// A melisma: the previous vowel continues on this note's pitch.
    Hold,
}

/// One note's lyric as the editor stores it.
#[derive(Clone, Copy, Debug)]
pub struct Lyric<'a> {
    pub text: &'a str,
    /// Explicit phonemes, space-separated, replace the lyric.
    pub phonemes: Option<&'a str>,
    pub language: Language,
}

/// Turns per-note lyrics into what each note sings. A lyric of `-` holds the
/// previous vowel; `+` sings the next syllable of the word started on an
/// earlier note; a lyric ending in `-` (as in `hap-`, `py`) continues into the
/// next note's text. An empty lyric sings `default`.
pub fn assign(
    lyrics: &[Lyric],
    default: &str,
    dictionary: Option<&en::Dictionary>,
) -> Vec<NoteText> {
    let mut out: Vec<NoteText> = Vec::with_capacity(lyrics.len());
    let mut i = 0;
    while i < lyrics.len() {
        let lyric = lyrics[i];
        if let Some(text) = lyric.phonemes.filter(|p| !p.trim().is_empty()) {
            let phonemes = Phoneme::parse_list(lyric.language, text).unwrap_or_default();
            out.push(if phonemes.is_empty() {
                NoteText::Hold
            } else {
                NoteText::Syllables(syllabify(&phonemes))
            });
            i += 1;
            continue;
        }
        let text = lyric.text.trim();
        if text == "-" || text == "+" || text == "ー" {
            out.push(NoteText::Hold);
            i += 1;
            continue;
        }
        // Gather the notes that share this word.
        let mut word = String::new();
        let mut group = 1;
        let mut part = if text.is_empty() { default } else { text };
        loop {
            match part.strip_suffix('-').filter(|p| !p.is_empty()) {
                Some(stem) => word.push_str(stem),
                None => {
                    word.push_str(part);
                    break;
                }
            }
            let Some(next) = lyrics.get(i + group) else {
                break;
            };
            if next.phonemes.is_some_and(|p| !p.trim().is_empty()) {
                break;
            }
            group += 1;
            part = next.text.trim();
        }
        while lyrics
            .get(i + group)
            .is_some_and(|l| l.text.trim() == "+" && l.phonemes.is_none_or(|p| p.trim().is_empty()))
        {
            group += 1;
        }
        let language = detect(&word, lyric.language);
        let mut syllables = syllables(language, &word, dictionary);
        if syllables.is_empty() {
            syllables = self::syllables(language, default, dictionary);
        }
        // One syllable per note; extra notes hold, extra syllables share the last note.
        let mut syllables = syllables.into_iter();
        for k in 0..group {
            let take: Vec<Syllable> = if k + 1 == group {
                syllables.by_ref().collect()
            } else {
                syllables.next().into_iter().collect()
            };
            out.push(if take.is_empty() {
                NoteText::Hold
            } else {
                NoteText::Syllables(take)
            });
        }
        i += group;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    fn lyric(text: &str, language: Language) -> Lyric<'_> {
        Lyric {
            text,
            phonemes: None,
            language,
        }
    }
    fn count(t: &NoteText) -> usize {
        match t {
            NoteText::Syllables(s) => s.len(),
            NoteText::Hold => 0,
        }
    }
    #[test]
    fn words_spread_over_notes() {
        let en = Language::En;
        let notes = [
            lyric("happy", en),
            lyric("+", en),
            lyric("-", en),
            lyric("day", en),
        ];
        let out = assign(&notes, "la", None);
        assert_eq!(out.iter().map(count).collect::<Vec<_>>(), [1, 1, 0, 1]);
        let notes = [lyric("hap-", en), lyric("py", en)];
        assert_eq!(
            assign(&notes, "la", None)
                .iter()
                .map(count)
                .collect::<Vec<_>>(),
            [1, 1]
        );
        let ru = Language::Ru;
        let notes = [lyric("при-", ru), lyric("вет", ru), lyric("", ru)];
        let out = assign(&notes, "а", None);
        assert_eq!(out.iter().map(count).collect::<Vec<_>>(), [1, 1, 1]);
        // Kana is Japanese even on an English track.
        let out = assign(&[lyric("さくら", en)], "la", None);
        assert_eq!(count(&out[0]), 3);
    }
    #[test]
    fn explicit_phonemes_win() {
        let notes = [Lyric {
            text: "ignored",
            phonemes: Some("s k ay"),
            language: Language::En,
        }];
        let out = assign(&notes, "la", None);
        let NoteText::Syllables(s) = &out[0] else {
            panic!()
        };
        assert_eq!(s[0].phonemes.len(), 3);
    }
    #[test]
    fn aliases_in_each_language() {
        let show = |l, t| {
            alias(l, t)
                .unwrap()
                .iter()
                .map(|p| p.symbol())
                .collect::<Vec<_>>()
                .join(" ")
        };
        assert_eq!(show(Language::Ja, "- か"), "sil k a");
        assert_eq!(show(Language::Ja, "a ka"), "a k a");
        assert_eq!(show(Language::Ja, "a k"), "a k");
        assert_eq!(show(Language::Ja, "a n"), "a N");
        assert_eq!(show(Language::Ja, "ky"), "ky");
        assert_eq!(show(Language::En, "- k aa"), "sil k aa");
        assert_eq!(show(Language::Ru, "а к"), "a k");
    }
}
