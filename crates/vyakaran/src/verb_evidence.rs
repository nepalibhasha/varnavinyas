//! Lexically supported readings for a bounded, regular verb family.
//!
//! A reading is morphological evidence, not a decision about a word's role
//! in its sentence. It must not by itself establish an orthographic error.
use std::sync::LazyLock;

use varnavinyas_kosha::{kosha, part_of_speech};

const DATA: &str = include_str!("../../../data/rule_inventories/regular_aaunu_verb_forms.tsv");
static ENDINGS: LazyLock<Vec<Ending<'static>>> = LazyLock::new(|| parse_endings(DATA));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerbForm {
    Finite,
    Participle,
    Infinitive,
    Converb,
}

struct Ending<'a> {
    suffix: &'a str,
    form: VerbForm,
}

fn parse_endings(data: &str) -> Vec<Ending<'_>> {
    let mut lines = data.lines();
    assert_eq!(lines.next(), Some("ending\tform\tsource\treview_status"));
    let mut out: Vec<Ending<'_>> = Vec::new();
    for line in lines {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 4, "invalid verb ending: {line}");
        assert!(
            fields.iter().all(|field| !field.is_empty()),
            "missing verb evidence: {line}"
        );
        assert_eq!(fields[3], "reviewed", "unreviewed verb ending: {line}");
        assert!(
            fields[2]
                .split(';')
                .any(|source| source == "reviewed-regular-aaunu"),
            "wrong verb family: {line}"
        );
        assert!(fields[0].starts_with('ा'), "not an आउनु ending: {line}");
        assert!(
            !out.iter().any(|ending| ending.suffix == fields[0]),
            "duplicate verb ending: {line}"
        );
        let form = match fields[1] {
            "finite" => VerbForm::Finite,
            "participle" => VerbForm::Participle,
            "infinitive" => VerbForm::Infinitive,
            "converb" => VerbForm::Converb,
            _ => panic!("unknown verb form: {line}"),
        };
        out.push(Ending {
            suffix: fields[0],
            form,
        });
    }
    out
}

/// Recognize reviewed inflections of a supplied, independently attested verb.
/// Currently supports regular आउनु verbs and one optional negative न prefix.
/// It does not infer irregular stems, gender agreement or a unique POS.
pub fn form_for_infinitive(surface: &str, infinitive: &str) -> Option<VerbForm> {
    let lex = kosha();
    if !lex.is_correction_target(infinitive)
        || !lex
            .lookup(infinitive)
            .is_some_and(|entry| part_of_speech::is_verb(entry.pos))
    {
        return None;
    }
    let stem = infinitive.strip_suffix("ाउनु")?;
    if stem.is_empty() {
        return None;
    }
    let positive = |word: &str| {
        let suffix = word.strip_prefix(stem)?;
        ENDINGS
            .iter()
            .find(|ending| ending.suffix == suffix)
            .map(|ending| ending.form)
    };
    positive(surface).or_else(|| surface.strip_prefix('न').and_then(positive))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_inflections_require_an_independent_verb_lemma() {
        for word in ["पठायो", "पठाइन्", "पठाइयो", "पठाएँ", "पठाउँछ", "पठाउनुभयो"]
        {
            assert_eq!(
                form_for_infinitive(word, "पठाउनु"),
                Some(VerbForm::Finite),
                "{word}"
            );
        }
        for word in ["पठाएको", "पठाउने", "पठाउँदै"] {
            assert_eq!(
                form_for_infinitive(word, "पठाउनु"),
                Some(VerbForm::Participle),
                "{word}"
            );
        }
        assert_eq!(
            form_for_infinitive("नपठाउने", "पठाउनु"),
            Some(VerbForm::Participle)
        );
        assert_eq!(
            form_for_infinitive("पठाएर", "पठाउनु"),
            Some(VerbForm::Converb)
        );
        assert_eq!(
            form_for_infinitive("पठाउनु", "पठाउनु"),
            Some(VerbForm::Infinitive)
        );
        assert_eq!(
            form_for_infinitive("लगाएको", "लगाउनु"),
            Some(VerbForm::Participle)
        );
        assert_eq!(
            form_for_infinitive("बनाउँछ", "बनाउनु"),
            Some(VerbForm::Finite)
        );
        for (word, lemma) in [
            ("कखगायो", "कखगाउनु"),
            ("पठतयो", "पठाउनु"),
            ("पठाइ", "पठाउनु"),
            ("पठाएकोको", "पठाउनु"),
            ("लेखायो", "लेख्नु"),
            ("पढाइ", "पढाइ"),
        ] {
            assert!(form_for_infinitive(word, lemma).is_none(), "{word}/{lemma}");
        }
    }

    #[test]
    fn malformed_or_unreviewed_endings_are_rejected() {
        for data in [
            DATA.replace("reviewed\n", "draft\n"),
            DATA.replace("reviewed-regular-aaunu", "unknown-family"),
            DATA.replace("finite\t", "unknown\t"),
            DATA.replace("ायो\t", "यो\t"),
            format!("{DATA}{}\n", DATA.lines().nth(1).unwrap()),
        ] {
            assert!(std::panic::catch_unwind(|| parse_endings(&data)).is_err());
        }
    }
}
