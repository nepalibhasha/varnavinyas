//! Bounded lexical support for progressive participles, not a conjugation parser.

use std::sync::LazyLock;
use varnavinyas_kosha::{kosha, part_of_speech};

const LINKING_FORMS: &str = include_str!("../../../data/rule_inventories/verb_linking_forms.tsv");
static IRREGULAR_LINKS: LazyLock<Vec<(&str, &str)>> =
    LazyLock::new(|| parse_linking_forms(LINKING_FORMS));

fn parse_linking_forms(data: &str) -> Vec<(&str, &str)> {
    let mut lines = data.lines();
    assert_eq!(
        lines.next(),
        Some("linking_form\tinfinitive\tsource\treview_status")
    );
    let mut rows = Vec::new();
    for line in lines.filter(|line| !line.trim().is_empty()) {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 4, "invalid linking form: {line}");
        assert!(
            fields[..3].iter().all(|field| !field.trim().is_empty()),
            "missing linking evidence: {line}"
        );
        assert_eq!(fields[3], "reviewed", "unreviewed linking form: {line}");
        assert!(
            fields[0].ends_with('ि') || fields[0].ends_with('इ'),
            "not a short linking form: {line}"
        );
        assert!(fields[1].ends_with("नु"), "not an infinitive: {line}");
        assert!(
            !rows.iter().any(|(form, _)| *form == fields[0]),
            "duplicate linking form: {line}"
        );
        rows.push((fields[0], fields[1]));
    }
    rows
}

/// Dictionary-backed reading of a regular progressive participle.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ProgressiveAnalysis {
    pub surface: String,
    /// Main verb's surface linking form, e.g. खोजि (not a lemma).
    pub main_form: String,
    /// Independently attested infinitive, e.g. खोज्नु.
    pub main_lemma: String,
    /// Inflected auxiliary, e.g. रहेको; -को is internal to this form.
    pub auxiliary_form: String,
    pub auxiliary_lemma: String,
    /// Whether an optional negative न was detached from the main form.
    pub negative: bool,
}

/// Recognize regular `-िरहेको/-इरहेको` participles and their `-की/-का` forms.
///
/// The linking vowel must be short and the reconstructed infinitive must have
/// dictionary verb metadata. This does not infer tense, agreement, or outer
/// case/particle suffixes. Those suffixes belong to the existing affix analysis.
pub fn has_supported_progressive_form(word: &str) -> bool {
    analyze_progressive(word).is_some()
}

/// Identify the main linking form and रहनु auxiliary using verb headwords.
/// Returns None outside the supported regular family; it does not split outer
/// suffixes or claim that an unrecognized word is incorrect.
pub fn analyze_progressive(word: &str) -> Option<ProgressiveAnalysis> {
    let (linked_stem, auxiliary_form) = ["रहेको", "रहेकी", "रहेका"]
        .iter()
        .find_map(|ending| word.strip_suffix(ending).map(|stem| (stem, *ending)))?;
    let stem = linked_stem
        .strip_suffix('ि')
        .or_else(|| linked_stem.strip_suffix('इ'))?;
    let lex = kosha();
    if !lex
        .lookup("रहनु")
        .is_some_and(|entry| part_of_speech::is_verb(entry.pos))
    {
        return None;
    }

    let (main_lemma, negative) = if let Some(lemma) = supported_infinitive(linked_stem, stem) {
        (lemma, false)
    } else {
        (
            supported_infinitive(linked_stem.strip_prefix('न')?, stem.strip_prefix('न')?)?,
            true,
        )
    };
    Some(ProgressiveAnalysis {
        surface: word.to_string(),
        main_form: linked_stem.to_string(),
        main_lemma,
        auxiliary_form: auxiliary_form.to_string(),
        auxiliary_lemma: "रहनु".to_string(),
        negative,
    })
}

fn supported_infinitive(linked_stem: &str, stem: &str) -> Option<String> {
    if stem.is_empty() || stem.ends_with('्') {
        return None;
    }
    let lex = kosha();
    let is_verb = |lemma: &str| {
        lex.lookup(lemma)
            .is_some_and(|entry| part_of_speech::is_verb(entry.pos))
    };
    if let Some((_, lemma)) = IRREGULAR_LINKS
        .iter()
        .find(|(form, _)| *form == linked_stem)
    {
        // Reviewed suppletion wins over unrelated, coincidentally spelled verbs.
        return is_verb(lemma).then(|| lemma.to_string());
    }
    let endings: &[&str] = if stem.ends_with(['ा', 'आ']) {
        &["नु", "्नु", "उनु"]
    } else {
        &["नु", "्नु"]
    };
    endings
        .iter()
        .find_map(|ending| {
            let lemma = format!("{stem}{ending}");
            is_verb(&lemma).then_some(lemma)
        })
        // इ-final infinitive stems retain their vowel (चुहिनु -> चुहि + रहेको).
        .or_else(|| {
            let lemma = format!("{linked_stem}नु");
            is_verb(&lemma).then_some(lemma)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_surface_parts_and_both_lemmas() {
        let analysis = analyze_progressive("खोजिरहेको").unwrap();
        assert_eq!(analysis.main_form, "खोजि");
        assert_eq!(analysis.main_lemma, "खोज्नु");
        assert_eq!(analysis.auxiliary_form, "रहेको");
        assert_eq!(analysis.auxiliary_lemma, "रहनु");
        assert!(!analysis.negative);
        let negative = analyze_progressive("नखोजिरहेकी").unwrap();
        assert_eq!(negative.main_form, "नखोजि");
        assert_eq!(negative.main_lemma, "खोज्नु");
        assert!(negative.negative);
    }

    #[test]
    fn progressive_participles_require_a_verb_lemma() {
        for stem in ["खोजि", "लेखि", "पढि", "गरि", "खाइ", "बनाइ", "नखोजि"]
        {
            for ending in ["रहेको", "रहेकी", "रहेका"] {
                let word = format!("{stem}{ending}");
                assert!(has_supported_progressive_form(&word), "{word}");
            }
        }
        // मनु is a noun, not evidence for a verb म + इ + रहेको.
        for word in [
            "मइरहेको",
            "झझझिरहेकी",
            "खोजीरहेको",
            "खाईरहेको",
            "खोज्रहेको",
            "खोजिरहेकोले",
            "खोजिरहनेको",
            "रहेको",
        ] {
            assert!(!has_supported_progressive_form(word), "{word}");
        }
    }

    #[test]
    fn dictionary_definition_forms_generalize_across_stem_families() {
        // Definitions from both dictionaries, not a whitelist of whole words.
        for (main, lemma) in [
            ("उदाइ", "उदाउनु"),
            ("बलि", "बल्नु"),
            ("हिँडि", "हिँड्नु"),
            ("ढाकि", "ढाक्नु"),
            ("छोपि", "छोप्नु"),
            ("कामि", "काम्नु"),
            ("रहि", "रहनु"),
            ("बोकि", "बोक्नु"),
            ("खटि", "खट्नु"),
            ("चम्कि", "चम्कनु"),
            ("घुमि", "घुम्नु"),
            ("चुहि", "चुहिनु"),
            ("देखिइ", "देखिनु"),
            ("रोकिइ", "रोकिनु"),
            ("घोत्लिइ", "घोत्लिनु"),
            ("खेलाइ", "खेलाउनु"),
            ("बढाइ", "बढाउनु"),
            ("दिइ", "दिनु"),
            ("लिइ", "लिनु"),
            ("आइ", "आउनु"),
            ("गइ", "जानु"),
            ("भइ", "हुनु"),
        ] {
            for ending in ["रहेको", "रहेकी", "रहेका"] {
                let word = format!("{main}{ending}");
                let reading = analyze_progressive(&word).unwrap_or_else(|| panic!("{word}"));
                assert_eq!(reading.main_lemma, lemma, "{word}");
                assert_eq!(reading.main_form, main, "{word}");
                assert_eq!(
                    format!("{}{}", reading.main_form, reading.auxiliary_form),
                    word
                );
            }
        }
    }

    #[test]
    fn linking_inventory_rejects_missing_evidence_and_duplicate_forms() {
        let header = "linking_form\tinfinitive\tsource\treview_status\n";
        for row in [
            "गइ\tजानु\t\treviewed",
            "गइ\tजानु\tsource\tdraft",
            "गई\tजानु\tsource\treviewed",
            "गइ\tजानु\tsource\treviewed\nगइ\tजानु\tsource\treviewed",
        ] {
            let data = format!("{header}{row}");
            assert!(std::panic::catch_unwind(|| parse_linking_forms(&data)).is_err());
        }
    }
}
