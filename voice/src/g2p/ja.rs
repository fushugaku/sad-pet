//! Japanese lyrics: hiragana, katakana or romaji, one syllable per mora.
//! っ becomes the closure `cl` at the end of the previous mora, ー repeats the
//! previous vowel and ん is the moraic nasal `N`.
use crate::phoneme::{Language, Phoneme, Syllable};

fn ph(symbol: &str) -> Phoneme {
    Phoneme::parse(Language::Ja, symbol).unwrap_or_else(|| panic!("ja phoneme {symbol}"))
}

/// Folds katakana into hiragana; other characters pass through.
fn hiragana(c: char) -> char {
    match c {
        '\u{30A1}'..='\u{30F6}' => char::from_u32(c as u32 - 0x60).unwrap_or(c),
        _ => c,
    }
}

/// The consonant and vowel of one full-size kana.
fn kana(c: char) -> Option<(&'static str, &'static str)> {
    const ROWS: [(&str, [char; 5], [&str; 5]); 14] = [
        ("", ['あ', 'い', 'う', 'え', 'お'], ["", "", "", "", ""]),
        (
            "k",
            ['か', 'き', 'く', 'け', 'こ'],
            ["k", "k", "k", "k", "k"],
        ),
        (
            "g",
            ['が', 'ぎ', 'ぐ', 'げ', 'ご'],
            ["g", "g", "g", "g", "g"],
        ),
        (
            "s",
            ['さ', 'し', 'す', 'せ', 'そ'],
            ["s", "sh", "s", "s", "s"],
        ),
        (
            "z",
            ['ざ', 'じ', 'ず', 'ぜ', 'ぞ'],
            ["z", "j", "z", "z", "z"],
        ),
        (
            "t",
            ['た', 'ち', 'つ', 'て', 'と'],
            ["t", "ch", "ts", "t", "t"],
        ),
        (
            "d",
            ['だ', 'ぢ', 'づ', 'で', 'ど'],
            ["d", "j", "z", "d", "d"],
        ),
        (
            "n",
            ['な', 'に', 'ぬ', 'ね', 'の'],
            ["n", "n", "n", "n", "n"],
        ),
        (
            "h",
            ['は', 'ひ', 'ふ', 'へ', 'ほ'],
            ["h", "h", "f", "h", "h"],
        ),
        (
            "b",
            ['ば', 'び', 'ぶ', 'べ', 'ぼ'],
            ["b", "b", "b", "b", "b"],
        ),
        (
            "p",
            ['ぱ', 'ぴ', 'ぷ', 'ぺ', 'ぽ'],
            ["p", "p", "p", "p", "p"],
        ),
        (
            "m",
            ['ま', 'み', 'む', 'め', 'も'],
            ["m", "m", "m", "m", "m"],
        ),
        (
            "r",
            ['ら', 'り', 'る', 'れ', 'ろ'],
            ["r", "r", "r", "r", "r"],
        ),
        ("y", ['や', '\0', 'ゆ', '\0', 'よ'], ["y", "", "y", "", "y"]),
    ];
    const VOWELS: [&str; 5] = ["a", "i", "u", "e", "o"];
    for (_, chars, consonants) in ROWS {
        if let Some(i) = chars.iter().position(|&k| k == c) {
            return Some((consonants[i], VOWELS[i]));
        }
    }
    match c {
        'わ' => Some(("w", "a")),
        'を' => Some(("", "o")),
        'ゔ' => Some(("v", "u")),
        _ => None,
    }
}

fn small_vowel(c: char) -> Option<&'static str> {
    match c {
        'ぁ' => Some("a"),
        'ぃ' => Some("i"),
        'ぅ' => Some("u"),
        'ぇ' => Some("e"),
        'ぉ' => Some("o"),
        'ゎ' => Some("a"),
        _ => None,
    }
}

fn small_y(c: char) -> Option<&'static str> {
    match c {
        'ゃ' => Some("a"),
        'ゅ' => Some("u"),
        'ょ' => Some("o"),
        _ => None,
    }
}

/// The palatalized form a consonant takes before ゃゅょ.
fn palatal(consonant: &str) -> &'static str {
    match consonant {
        "k" => "ky",
        "g" => "gy",
        "n" => "ny",
        "h" | "f" => "hy",
        "b" => "by",
        "p" => "py",
        "m" => "my",
        "r" => "ry",
        "s" | "sh" => "sh",
        "z" | "j" => "j",
        "t" | "ch" => "ch",
        "d" => "dy",
        _ => "y",
    }
}

#[derive(Default)]
struct Builder {
    syllables: Vec<Vec<Phoneme>>,
}
impl Builder {
    fn mora(&mut self, consonant: &str, vowel: &str) {
        let mut p = Vec::with_capacity(2);
        if !consonant.is_empty() {
            p.push(ph(consonant));
        }
        p.push(ph(vowel));
        self.syllables.push(p);
    }
    fn nasal(&mut self) {
        self.syllables.push(vec![ph("N")]);
    }
    fn closure(&mut self) {
        match self.syllables.last_mut() {
            Some(last) => last.push(Phoneme::CLOSURE),
            None => self.syllables.push(vec![Phoneme::CLOSURE]),
        }
    }
    fn long(&mut self) {
        let vowel = self
            .syllables
            .last()
            .and_then(|s| s.iter().rev().find(|p| p.is_nucleus()).copied());
        if let Some(v) = vowel {
            self.syllables.push(vec![v]);
        }
    }
    fn finish(self) -> Vec<Syllable> {
        self.syllables.into_iter().map(Syllable::new).collect()
    }
}

fn from_kana(text: &str) -> Option<Vec<Syllable>> {
    let chars: Vec<char> = text.chars().map(hiragana).collect();
    let mut out = Builder::default();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        match c {
            'っ' => out.closure(),
            'ん' => out.nasal(),
            'ー' | '〜' | '～' => out.long(),
            _ if c.is_whitespace() => {}
            _ => {
                if let Some(vowel) = small_vowel(c).or_else(|| small_y(c)) {
                    // A small kana on its own is sung as its vowel.
                    out.mora(if small_y(c).is_some() { "y" } else { "" }, vowel);
                } else {
                    let (consonant, vowel) = kana(c)?;
                    if let Some(v) = next.and_then(small_y) {
                        out.mora(palatal(consonant), v);
                        i += 1;
                    } else if let Some(v) = next.and_then(small_vowel) {
                        let consonant = match (c, consonant) {
                            ('う', _) => "w",
                            ('い', _) => "y",
                            (_, "ch") | (_, "sh") | (_, "j") | (_, "ts") | (_, "f") | (_, "v") => {
                                consonant
                            }
                            ('て', _) | ('と', _) => "t",
                            ('で', _) | ('ど', _) => "d",
                            (_, c) => c,
                        };
                        out.mora(consonant, v);
                        i += 1;
                    } else {
                        out.mora(consonant, vowel);
                    }
                }
            }
        }
        i += 1;
    }
    Some(out.finish())
}

const ROMAJI_CONSONANTS: [&str; 40] = [
    "kw", "ky", "gy", "sh", "sy", "ch", "cy", "ty", "dy", "ts", "ny", "hy", "my", "ry", "by", "py",
    "zy", "jy", "fy", "k", "g", "s", "z", "j", "t", "d", "n", "h", "f", "b", "p", "m", "y", "r",
    "l", "w", "v", "c", "q", "x",
];

fn from_romaji(text: &str) -> Option<Vec<Syllable>> {
    let lower = text.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    let mut out = Builder::default();
    let mut i = 0;
    while i < bytes.len() {
        let rest = &lower[i..];
        let b = bytes[i];
        if b == b' ' || b == b'\'' {
            i += 1;
            continue;
        }
        if b == b'-' {
            out.long();
            i += 1;
            continue;
        }
        if let Some(v) = b"aiueo".iter().position(|&v| v == b) {
            out.mora("", ["a", "i", "u", "e", "o"][v]);
            i += 1;
            continue;
        }
        // A doubled consonant is a geminate: っ, also in Hepburn's "tch".
        if (bytes.get(i + 1) == Some(&b) && b != b'n') || rest.starts_with("tch") {
            out.closure();
            i += 1;
            continue;
        }
        if b == b'n' {
            let next = bytes.get(i + 1).copied();
            if next.is_none_or(|n| !b"aiueoy".contains(&n)) {
                out.nasal();
                i += if next == Some(b'n')
                    && !bytes.get(i + 2).is_some_and(|n| b"aiueoy".contains(n))
                {
                    2
                } else {
                    1
                };
                continue;
            }
        }
        let consonant = ROMAJI_CONSONANTS.iter().find(|c| rest.starts_with(**c))?;
        let after = i + consonant.len();
        let vowel = bytes
            .get(after)
            .and_then(|&v| b"aiueo".iter().position(|&x| x == v))?;
        let vowel = ["a", "i", "u", "e", "o"][vowel];
        let consonant = match (*consonant, vowel) {
            ("s", "i") => "sh",
            ("z", "i") | ("j", _) | ("zy", _) | ("jy", _) => "j",
            ("t", "i") | ("cy", _) | ("ch", _) => "ch",
            ("t", "u") => "ts",
            ("h", "u") | ("fy", _) => "f",
            ("sy", _) | ("sh", _) => "sh",
            ("c", _) | ("q", _) => "k",
            ("l", _) => "r",
            ("w", "o") => "",
            ("x", _) => "",
            (c, _) => c,
        };
        out.mora(consonant, vowel);
        i = after + 1;
    }
    Some(out.finish())
}

/// Syllables of a Japanese lyric; `None` if it contains something unreadable.
pub fn syllables(text: &str) -> Option<Vec<Syllable>> {
    let text = text.trim();
    if text.is_empty() {
        return Some(Vec::new());
    }
    if text.is_ascii() {
        from_romaji(text)
    } else {
        from_kana(text)
    }
}

/// The phonemes of a UTAU alias written in kana or romaji, e.g. `a か`.
pub fn alias(text: &str) -> Option<Vec<Phoneme>> {
    Some(
        syllables(text)?
            .into_iter()
            .flat_map(|s| s.phonemes)
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn show(s: Option<Vec<Syllable>>) -> String {
        s.unwrap()
            .iter()
            .map(|s| {
                s.phonemes
                    .iter()
                    .map(|p| p.symbol())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join(" | ")
    }
    #[test]
    fn kana_and_romaji_agree() {
        assert_eq!(show(syllables("きゃっと")), "ky a cl | t o");
        assert_eq!(show(syllables("キャット")), "ky a cl | t o");
        assert_eq!(show(syllables("kyatto")), "ky a cl | t o");
        assert_eq!(show(syllables("しんじゅく")), "sh i | N | j u | k u");
        assert_eq!(show(syllables("shinjuku")), "sh i | N | j u | k u");
        assert_eq!(show(syllables("ティー")), "t i | i");
        assert_eq!(show(syllables("ふぁ")), "f a");
        assert_eq!(show(syllables("tsuki")), "ts u | k i");
        assert_eq!(show(syllables("を")), "o");
        assert_eq!(show(syllables("konnichiwa")), "k o | N | n i | ch i | w a");
    }
}
