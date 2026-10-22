//! Russian spelling is close to pronunciation: the rules below cover
//! palatalization, iotated vowels, voicing assimilation, final devoicing and
//! the common silent or simplified clusters. Stress is not in the spelling, so
//! vowels are not reduced unless the lyric marks the stress (`+` after the
//! vowel, an acute accent, or ё); sung Russian reduces far less than speech.
use crate::phoneme::{Language, Phoneme, Syllable, syllabify};

fn ph(symbol: &str) -> Phoneme {
    Phoneme::parse(Language::Ru, symbol).unwrap_or_else(|| panic!("ru phoneme {symbol}"))
}

const VOWELS: &str = "аеёиоуыэюя";

/// Whole-word pronunciations that the rules below would get wrong.
const EXCEPTIONS: &[(&str, &str)] = &[
    ("что", "што"),
    ("чтобы", "штобы"),
    ("ничто", "ништо"),
    ("конечно", "конешно"),
    ("скучно", "скушно"),
    ("нарочно", "нарошно"),
    ("скворечник", "скворешник"),
    ("яичница", "яишница"),
    ("сегодня", "сиводня"),
    ("сегодняшний", "сиводняшний"),
    ("солнце", "сонце"),
    ("здравствуй", "здраствуй"),
    ("здравствуйте", "здраствуйте"),
    ("чувство", "чуство"),
    ("чувства", "чуства"),
    ("лестница", "лесница"),
    ("сердце", "серце"),
    ("счастье", "щастье"),
    ("счастья", "щастья"),
    ("счастлив", "щаслив"),
    ("счастливый", "щасливый"),
    ("счастливо", "щасливо"),
    ("бог", "бох"),
    ("лёгкий", "лёхкий"),
    ("легко", "лехко"),
    ("мягкий", "мяхкий"),
    ("мягко", "мяхко"),
    ("ещё", "ищё"),
];

/// Adjective and pronoun endings in -ого/-его are pronounced with в, except
/// in these words, where г is part of the stem.
const OGO_KEEPS_G: &[&str] = &[
    "много",
    "немного",
    "строго",
    "дорого",
    "недорого",
    "убого",
    "полого",
    "отлого",
    "строго",
    "пирога",
    "ого",
    "его-то",
];

fn respell(word: &str) -> String {
    if let Some((_, spoken)) = EXCEPTIONS.iter().find(|(w, _)| *w == word) {
        return spoken.to_string();
    }
    let mut w = word.to_string();
    let ends_ogo = (w.ends_with("ого") || w.ends_with("его")) && w.chars().count() >= 3;
    if ends_ogo && !OGO_KEEPS_G.contains(&word) {
        let cut = w.len() - "го".len();
        w.replace_range(cut.., "во");
    }
    for (from, to) in [
        ("тся", "ца"),
        ("ться", "ца"),
        ("сч", "щ"),
        ("зч", "щ"),
        ("жч", "щ"),
        ("стн", "сн"),
        ("здн", "зн"),
        ("лнц", "нц"),
        ("вств", "ств"),
        ("рдц", "рц"),
        ("стл", "сл"),
        ("ндск", "нск"),
        ("нтск", "нск"),
    ] {
        w = w.replace(from, to);
    }
    w
}

fn hard(c: char) -> Option<&'static str> {
    Some(match c {
        'б' => "b",
        'в' => "v",
        'г' => "g",
        'д' => "d",
        'ж' => "zh",
        'з' => "z",
        'к' => "k",
        'л' => "l",
        'м' => "m",
        'н' => "n",
        'п' => "p",
        'р' => "r",
        'с' => "s",
        'т' => "t",
        'ф' => "f",
        'х' => "x",
        'ц' => "ts",
        'ч' => "ch",
        'ш' => "sh",
        'щ' => "sch",
        'й' => "j",
        _ => return None,
    })
}

fn soften(symbol: &str) -> String {
    match symbol {
        "zh" | "sh" | "ts" | "ch" | "sch" | "j" => symbol.to_string(),
        s => format!("{s}'"),
    }
}

fn voicing_pair(symbol: &str) -> Option<(&'static str, bool)> {
    // (partner, is_voiced)
    const PAIRS: [(&str, &str); 14] = [
        ("p", "b"),
        ("p'", "b'"),
        ("t", "d"),
        ("t'", "d'"),
        ("k", "g"),
        ("k'", "g'"),
        ("f", "v"),
        ("f'", "v'"),
        ("s", "z"),
        ("s'", "z'"),
        ("sh", "zh"),
        ("x", "g"),
        ("ts", "ts"),
        ("ch", "ch"),
    ];
    PAIRS.iter().find_map(|&(unvoiced, voiced)| {
        if symbol == unvoiced {
            Some((voiced, false))
        } else if symbol == voiced {
            Some((unvoiced, true))
        } else {
            None
        }
    })
}

fn obstruent(symbol: &str) -> bool {
    voicing_pair(symbol).is_some() || symbol == "sch"
}

/// Phonemes of one word (Cyrillic letters, optional stress marks).
pub fn word(text: &str) -> Vec<Phoneme> {
    let lower = text.to_lowercase();
    // A stress mark (+ or combining acute) follows the stressed vowel.
    let mut letters = String::new();
    let mut stressed = None;
    for c in lower.chars() {
        if c == '+' || c == '\u{301}' || c == '\'' {
            stressed = Some(letters.chars().count().saturating_sub(1));
        } else if c.is_alphabetic() || c == '-' {
            letters.push(c);
        }
    }
    if stressed.is_none() {
        stressed = letters.chars().position(|c| c == 'ё');
    }
    let marked = stressed.is_some();
    let word = if marked {
        letters.clone()
    } else {
        respell(&letters)
    };
    let chars: Vec<char> = word.chars().filter(|&c| c != '-').collect();

    let mut out: Vec<(String, bool)> = Vec::new(); // (symbol, stressed vowel)
    for (i, &c) in chars.iter().enumerate() {
        let prev = i.checked_sub(1).map(|j| chars[j]);
        let next = chars.get(i + 1).copied();
        let after_consonant = prev.is_some_and(|p| hard(p).is_some() && p != 'й');
        let prev_unsoftenable = prev.is_some_and(|p| matches!(p, 'ж' | 'ш' | 'ц'));
        let is_stressed = stressed == Some(i);
        if let Some(symbol) = hard(c) {
            let soft = next.is_some_and(|n| "еёиюяь".contains(n));
            let symbol = if soft {
                soften(symbol)
            } else {
                symbol.to_string()
            };
            // Doubled letters are sung as one consonant.
            if out.last().is_some_and(|(s, _)| *s == symbol) {
                continue;
            }
            out.push((symbol, false));
            continue;
        }
        let iotated = !after_consonant;
        let vowel = match c {
            'а' => "a",
            'о' => "o",
            'у' => "u",
            'ы' => "y",
            'э' => "e",
            'и' if prev_unsoftenable => "y",
            'и' => {
                if prev == Some('ь') {
                    out.push(("j".into(), false));
                }
                "i"
            }
            'е' | 'ё' | 'ю' | 'я' => {
                if iotated {
                    out.push(("j".into(), false));
                }
                match c {
                    'е' => "e",
                    'ё' => "o",
                    'ю' => "u",
                    _ => "a",
                }
            }
            _ => continue, // ь, ъ and anything else
        };
        let vowel = if marked && !is_stressed {
            match vowel {
                "o" => "a",
                v => v,
            }
        } else {
            vowel
        };
        out.push((vowel.into(), is_stressed));
    }
    // Voicing assimilation, right to left, and final devoicing.
    let mut next_voicing: Option<bool> = Some(false); // word end acts as voiceless
    for (symbol, _) in out.iter_mut().rev() {
        if obstruent(symbol) {
            if let Some(voiced_next) = next_voicing
                && let Some((partner, voiced)) = voicing_pair(symbol)
                && voiced != voiced_next
            {
                *symbol = partner.to_string();
            }
            // в and в' do not voice what comes before them.
            next_voicing = if symbol.starts_with('v') {
                None
            } else {
                voicing_pair(symbol).map(|(_, v)| v).or(Some(false))
            };
        } else {
            next_voicing = None;
        }
    }
    out.iter().map(|(s, _)| ph(s)).collect()
}

/// Syllables of a lyric that may hold several words.
pub fn syllables(text: &str) -> Vec<Syllable> {
    text.split_whitespace()
        .flat_map(|w| syllabify(&word(w)))
        .collect()
}

/// The phonemes of a UTAU alias written in Cyrillic (`ка`, `а к`) or in the
/// inventory's own symbols (`k a`, `t' i`).
pub fn alias(text: &str) -> Option<Vec<Phoneme>> {
    if text
        .chars()
        .any(|c| ('а'..='я').contains(&c.to_lowercase().next().unwrap_or(c)) || c == 'ё')
    {
        // Each space-separated part is spelled on its own; a lone consonant
        // letter stays hard unless it carries ь.
        return Some(
            text.split_whitespace()
                .flat_map(|w| match Phoneme::parse(Language::Ru, w) {
                    Some(p) => vec![p],
                    None => part(w),
                })
                .collect(),
        );
    }
    Phoneme::parse_list(Language::Ru, text).ok()
}

fn part(text: &str) -> Vec<Phoneme> {
    // Alias parts are fragments: no word-final devoicing or respelling.
    let chars: Vec<char> = text.to_lowercase().chars().collect();
    let mut out = Vec::new();
    for (i, &c) in chars.iter().enumerate() {
        let next = chars.get(i + 1).copied();
        let prev = i.checked_sub(1).map(|j| chars[j]);
        if let Some(symbol) = hard(c) {
            let soft = next.is_some_and(|n| "еёиюяь".contains(n));
            out.push(ph(&if soft {
                soften(symbol)
            } else {
                symbol.to_string()
            }));
        } else if VOWELS.contains(c) {
            let after_consonant = prev.is_some_and(|p| hard(p).is_some() && p != 'й');
            let unsoft = prev.is_some_and(|p| matches!(p, 'ж' | 'ш' | 'ц'));
            let v = match c {
                'а' => "a",
                'о' => "o",
                'у' => "u",
                'ы' => "y",
                'э' => "e",
                'и' if unsoft => "y",
                'и' => "i",
                _ => {
                    if !after_consonant {
                        out.push(ph("j"));
                    }
                    match c {
                        'е' => "e",
                        'ё' => "o",
                        'ю' => "u",
                        _ => "a",
                    }
                }
            };
            out.push(ph(v));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    fn show(text: &str) -> String {
        word(text)
            .iter()
            .map(|p| p.symbol())
            .collect::<Vec<_>>()
            .join(" ")
    }
    #[test]
    fn spelling_rules() {
        assert_eq!(show("привет"), "p r' i v' e t");
        assert_eq!(show("любовь"), "l' u b o f'");
        assert_eq!(show("яблоко"), "j a b l o k o");
        assert_eq!(show("я+блоко"), "j a b l a k a");
        assert_eq!(show("сказка"), "s k a s k a");
        assert_eq!(show("вокзал"), "v o g z a l");
        assert_eq!(show("жизнь"), "zh y z n'");
        assert_eq!(show("улыбается"), "u l y b a j e ts a");
        assert_eq!(show("его"), "j e v o");
        assert_eq!(show("много"), "m n o g o");
        assert_eq!(show("солнце"), "s o n ts e");
        assert_eq!(show("ёлка"), "j o l k a");
        assert_eq!(show("подъезд"), "p o d j e s t");
        assert_eq!(show("счастье"), "sch a s t' j e");
        assert_eq!(show("ночь"), "n o ch");
        assert_eq!(show("в"), "f");
    }
    #[test]
    fn aliases() {
        let show = |t| {
            alias(t)
                .unwrap()
                .iter()
                .map(|p| p.symbol())
                .collect::<Vec<_>>()
                .join(" ")
        };
        assert_eq!(show("ка"), "k a");
        assert_eq!(show("а к"), "a k");
        assert_eq!(show("- ля"), "sil l' a");
        assert_eq!(show("t' i"), "t' i");
    }
}
