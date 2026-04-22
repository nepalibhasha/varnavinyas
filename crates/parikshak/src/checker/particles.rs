use std::collections::HashSet;

use varnavinyas_kosha::kosha;
use varnavinyas_prakriya::{DiagnosticKind, Rule};
use varnavinyas_shabda::has_supported_analysis;

use crate::diagnostic::{Diagnostic, DiagnosticCategory};
use crate::tokenizer::should_prefer_whole_word_over_short_nipat_split;

use super::common::{
    is_devanagari_word, is_numeric_segment, is_word_boundary, whitespace_segments,
};

const WORD_BOUND_NIPAT_SPLIT_TOKENS: &[&str] = &["चाहिँ", "झैँ", "नै", "पो", "नि", "त", "ल"];
const SENTENCE_BOUND_NIPAT_REFERENCE_TOKENS: &[&str] = &[
    "अँ",
    "अरे",
    "ए",
    "कि",
    "के",
    "क्या",
    "क्यार",
    "खै",
    "नाइँ",
    "यारे",
    "लौ",
    "र",
    "रे",
    "सके",
    "हँ",
    "हगि",
    "है",
];
// Keep auto-splitting conservative for वाक्याश्रित निपात. Very short or highly
// polyfunctional particles like 'र', 'के', 'कि', 'रे', 'है' need stronger
// sentence analysis than this local suffix-based pass currently has.
const SENTENCE_BOUND_NIPAT_SPLIT_TOKENS: &[&str] =
    &["अरे", "क्या", "क्यार", "खै", "नाइँ", "यारे", "सके", "हगि"];

fn suffix_left_candidates<'a>(token: &'a str, suffix: &str) -> Vec<&'a str> {
    let mut candidates = Vec::new();
    let token_len = token.chars().count();
    let suffix_len = suffix.chars().count();

    if let Some(left) = token.strip_suffix(suffix) {
        if !left.is_empty() {
            candidates.push(left);
        }
    }

    let mut suffix_chars = suffix.chars();
    let Some(onset) = suffix_chars.next() else {
        return candidates;
    };
    let tail = suffix_chars.as_str();
    if tail.is_empty() {
        return candidates;
    }

    if token_len > suffix_len {
        if let Some(left) = token.strip_suffix(tail) {
            if !left.is_empty() && left.ends_with(onset) && !candidates.contains(&left) {
                candidates.push(left);
            }
        }
    }

    candidates
}

pub(super) fn add_nipat_split_diagnostics(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    debug_assert!(
        SENTENCE_BOUND_NIPAT_SPLIT_TOKENS
            .iter()
            .all(|suffix| SENTENCE_BOUND_NIPAT_REFERENCE_TOKENS.contains(suffix))
    );

    let segments = whitespace_segments(text);

    for (token, start, end) in segments {
        if !is_devanagari_word(token) || is_numeric_segment(token) {
            continue;
        }

        if add_word_bound_nipat_split(token, start, end, text, blocked_spans, diagnostics) {
            continue;
        }

        let _ = add_sentence_bound_nipat_split(token, start, end, text, blocked_spans, diagnostics);
    }
}

fn add_word_bound_nipat_split(
    token: &str,
    start: usize,
    end: usize,
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) -> bool {
    for &suffix in WORD_BOUND_NIPAT_SPLIT_TOKENS {
        for left in suffix_left_candidates(token, suffix) {
            let normalized_left = normalize_joined_word(left).unwrap_or_else(|| left.to_string());
            if !candidate_is_supported(left) && !candidate_is_supported(&normalized_left) {
                continue;
            }

            if matches!(suffix, "त" | "ल") && normalized_left.chars().count() < 2 {
                continue;
            }
            if matches!(suffix, "नै") && candidate_is_lexically_attested(token) {
                continue;
            }
            if matches!(suffix, "त" | "ल" | "नि") && candidate_is_supported(token) {
                continue;
            }
            if should_prefer_whole_word_over_short_nipat_split(left, suffix, kosha()) {
                continue;
            }

            let correction = format!("{left} {suffix}");
            if !push_nipat_diagnostic(
                token,
                correction,
                start,
                end,
                text,
                blocked_spans,
                diagnostics,
                "शैक्षणिक व्याकरण पदवियोग (च): शब्दाश्रित निपातहरू पदवियोग गरी लेखिन्छन् ।",
                if matches!(suffix, "त" | "ल" | "नि") {
                    0.86
                } else {
                    0.9
                },
            ) {
                continue;
            }
            return true;
        }
    }

    false
}

fn add_sentence_bound_nipat_split(
    token: &str,
    start: usize,
    end: usize,
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) -> bool {
    for &suffix in SENTENCE_BOUND_NIPAT_SPLIT_TOKENS {
        let Some(left) = token.strip_suffix(suffix) else {
            continue;
        };
        if left.is_empty() {
            continue;
        }

        let normalized_left = normalize_joined_word(left).unwrap_or_else(|| left.to_string());
        if !candidate_is_supported(&normalized_left) {
            continue;
        }

        let correction = format!("{left} {suffix}");
        if !push_nipat_diagnostic(
            token,
            correction,
            start,
            end,
            text,
            blocked_spans,
            diagnostics,
            "शैक्षणिक व्याकरण पदवियोग (च): वाक्याश्रित निपातहरू पदवियोग गरी लेखिन्छन् ।",
            0.88,
        ) {
            continue;
        }
        return true;
    }

    false
}

fn push_nipat_diagnostic(
    token: &str,
    correction: String,
    start: usize,
    end: usize,
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
    explanation: &str,
    confidence: f32,
) -> bool {
    let span = (start, end);
    let overlaps_other_span = diagnostics
        .iter()
        .filter(|d| !matches!(d.kind, DiagnosticKind::Ambiguous))
        .any(|d| d.span != span && d.span.0 < span.1 && span.0 < d.span.1);
    if overlaps_other_span {
        return false;
    }
    if !is_word_boundary(text, span.0, span.1) {
        return false;
    }
    if correction == token || has_same_rewrite(diagnostics, span, &correction) {
        return false;
    }

    diagnostics.push(Diagnostic {
        span,
        incorrect: token.to_string(),
        correction,
        rule: Rule::VarnaVinyasNiyam("3(घ)"),
        explanation: explanation.to_string(),
        category: DiagnosticCategory::ShuddhaTable,
        kind: DiagnosticKind::Error,
        confidence,
        alternate_reasons: Vec::new(),
    });
    blocked_spans.insert(span);
    true
}

fn has_same_rewrite(diagnostics: &[Diagnostic], span: (usize, usize), correction: &str) -> bool {
    diagnostics
        .iter()
        .any(|d| d.span == span && d.correction == correction)
}

fn candidate_is_supported(candidate: &str) -> bool {
    let lex = kosha();
    lex.contains(candidate) || lex.lookup(candidate).is_some() || has_supported_analysis(candidate)
}

fn candidate_is_lexically_attested(candidate: &str) -> bool {
    let lex = kosha();
    lex.contains(candidate) || lex.lookup(candidate).is_some()
}

fn normalize_joined_word(candidate: &str) -> Option<String> {
    if let Some(base) = candidate.strip_suffix("संग") {
        return Some(format!("{base}सँग"));
    }
    if let Some(base) = candidate.strip_suffix("सङ") {
        return Some(format!("{base}सँग"));
    }
    None
}
