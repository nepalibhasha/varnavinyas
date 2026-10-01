//! Bounded converb constructions, not general noun/verb disambiguation.
use std::sync::LazyLock;

use varnavinyas_kosha::{kosha, part_of_speech};

use super::{
    ContextCandidate, ContextRoleHint, DiagnosticCategory, DiagnosticKind, Rule, SentenceSpan,
};
use crate::{DiagnosticEvidence, tokenizer::AnalyzedToken};

const DATA: &str = include_str!("../../../../../data/rule_inventories/context_converbs.tsv");
const SOURCE: &str = "PS-Saisanik-ह्रस्वदीर्घ-(भ)";
static ENTRIES: LazyLock<Vec<Entry<'static>>> = LazyLock::new(|| parse_inventory(DATA));

struct Entry<'a> {
    short: &'a str,
    long: &'a str,
    infinitive: &'a str,
    left: Vec<Vec<&'a str>>,
    right: Vec<Vec<&'a str>>,
    kind: DiagnosticKind,
}

fn parse_inventory(data: &str) -> Vec<Entry<'_>> {
    let mut lines = data.lines();
    assert_eq!(
        lines.next(),
        Some("short\tlong\tinfinitive\tleft_contexts\tright_contexts\tkind\tsource\treview_status")
    );
    let mut entries: Vec<Entry<'_>> = Vec::new();
    for line in lines.filter(|line| !line.trim().is_empty()) {
        let f: Vec<_> = line.split('\t').collect();
        assert_eq!(f.len(), 8, "invalid converb row: {line}");
        assert!(
            f.iter().all(|field| !field.is_empty()),
            "missing field: {line}"
        );
        assert_eq!(f[7], "reviewed", "unreviewed frame: {line}");
        assert!(
            f[6].split(';').any(|source| source == SOURCE),
            "missing rule source: {line}"
        );
        let stem = f[0]
            .strip_suffix('ि')
            .or_else(|| f[0].strip_suffix('इ'))
            .expect("short final i");
        let expected = format!("{stem}{}", if f[0].ends_with('ि') { "ी" } else { "ई" });
        assert_eq!(f[1], expected, "not a final-i pair: {line}");
        assert!(f[2].ends_with("नु"), "not an infinitive: {line}");
        assert!(
            !entries.iter().any(|entry| entry.short == f[0]),
            "duplicate converb: {line}"
        );
        entries.push(Entry {
            short: f[0],
            long: f[1],
            infinitive: f[2],
            left: parse_phrases(f[3]),
            right: parse_phrases(f[4]),
            kind: match f[5] {
                "error" => DiagnosticKind::Error,
                "ambiguous" => DiagnosticKind::Ambiguous,
                _ => panic!("unknown converb kind: {line}"),
            },
        });
    }
    entries
}

fn parse_phrases(field: &str) -> Vec<Vec<&str>> {
    field
        .split('|')
        .map(|phrase| {
            let words: Vec<_> = phrase.split(' ').collect();
            assert!(
                words.len() <= 3 && words.iter().all(|word| !word.is_empty()),
                "invalid frame: {field}"
            );
            words
        })
        .collect()
}

/// Match contiguous tokens only. Every intervening source character must be
/// horizontal whitespace: clauses, quotations, sense numbers and line breaks
/// must never supply evidence for a neighboring construction.
fn phrase_matches(text: &str, tokens: &[AnalyzedToken], start: usize, words: &[&str]) -> bool {
    let Some(slice) = tokens.get(start..start + words.len()) else {
        return false;
    };
    slice
        .iter()
        .zip(words)
        .all(|(token, word)| token.surface() == *word)
        && slice
            .windows(2)
            .all(|pair| horizontal_gap(text, &pair[0], &pair[1]))
}

fn horizontal_gap(text: &str, left: &AnalyzedToken, right: &AnalyzedToken) -> bool {
    let gap = &text[left.end..right.start];
    !gap.is_empty()
        && gap
            .chars()
            .all(|ch| ch.is_whitespace() && !matches!(ch, '\n' | '\r' | '\u{2028}' | '\u{2029}'))
}

pub(super) fn candidates(
    text: &str,
    tokens: &[AnalyzedToken],
    sentence: SentenceSpan,
) -> Vec<ContextCandidate> {
    let lex = kosha();
    let mut out = Vec::new();
    for idx in sentence.start_token..sentence.end_token {
        let surface = tokens[idx].surface();
        let Some(entry) = ENTRIES.iter().find(|entry| entry.short == surface) else {
            continue;
        };
        // An attested spelling alone cannot establish either a verb reading or
        // a safe replacement. Require independent lexical evidence for both.
        if !lex.is_correction_target(entry.long)
            || !lex.is_correction_target(entry.infinitive)
            || !lex
                .lookup(entry.infinitive)
                .is_some_and(|word| part_of_speech::is_verb(word.pos))
        {
            continue;
        }
        let left_match = entry.left.iter().any(|words| {
            idx.checked_sub(words.len()).is_some_and(|start| {
                start >= sentence.start_token
                    && phrase_matches(text, tokens, start, words)
                    && horizontal_gap(text, &tokens[idx - 1], &tokens[idx])
            })
        });
        let right_match = entry.right.iter().any(|words| {
            idx + 1 + words.len() <= sentence.end_token
                && phrase_matches(text, tokens, idx + 1, words)
                && horizontal_gap(text, &tokens[idx], &tokens[idx + 1])
        });
        if !left_match || !right_match {
            continue;
        }
        let span = tokens[idx].span();
        out.push(ContextCandidate {
            span, incorrect: text[span.0..span.1].to_owned(), correction: entry.long.to_owned(),
            rule: Rule::VarnaVinyasNiyam("PS-Saisanik-ह्रस्वदीर्घ-(भ)-context-कृदन्त"),
            explanation: format!("शैक्षणिक व्याकरण (भ): यस क्रिया-निर्माणमा '{}' को एर-अर्थक रूप '{}' हुन्छ; नाम वा क्रियाविशेषणका रूपमा '{}' छुट्टै मान्य छ", entry.infinitive, entry.long, entry.short),
            category: DiagnosticCategory::HrasvaDirgha, kind: entry.kind, confidence: 0.90,
            role_hint: ContextRoleHint::Converb,
            evidence: DiagnosticEvidence::CuratedInventory,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_has_independent_verb_and_replacement_evidence() {
        for entry in parse_inventory(DATA) {
            assert!(kosha().contains(entry.short));
            assert!(kosha().is_correction_target(entry.long));
            assert!(part_of_speech::is_verb(
                kosha().lookup(entry.infinitive).unwrap().pos
            ));
        }
    }

    #[test]
    fn parser_rejects_unreviewed_or_malformed_frames() {
        for data in [
            DATA.replace("reviewed", "draft"),
            DATA.replace(SOURCE, "unknown"),
            DATA.replace("पारी", "पारु"),
            DATA.replace("पूरा|", "|"),
            DATA.replace("\terror\t", "\tautomatic\t"),
            format!("{DATA}{}\n", DATA.lines().nth(1).unwrap()),
        ] {
            assert!(std::panic::catch_unwind(|| parse_inventory(&data)).is_err());
        }
    }
}
