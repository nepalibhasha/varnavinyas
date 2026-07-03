use std::collections::HashSet;

use varnavinyas_kosha::kosha;
use varnavinyas_prakriya::{DiagnosticKind, Rule};

use crate::diagnostic::{Diagnostic, DiagnosticCategory};
use crate::tokenizer::AnalyzedToken;

use super::arbitration::overlaps_existing_span;

#[derive(Debug, Clone, Copy)]
struct SentenceSpan {
    start_token: usize,
    end_token: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContextRoleHint {
    FinalPredicate,
}

impl ContextRoleHint {
    fn rank(self) -> u8 {
        match self {
            Self::FinalPredicate => 1,
        }
    }
}

#[derive(Debug, Clone)]
struct ContextCandidate {
    span: (usize, usize),
    incorrect: String,
    correction: String,
    rule: Rule,
    explanation: String,
    category: DiagnosticCategory,
    kind: DiagnosticKind,
    confidence: f32,
    role_hint: ContextRoleHint,
}

impl From<ContextCandidate> for Diagnostic {
    fn from(candidate: ContextCandidate) -> Self {
        Diagnostic {
            span: candidate.span,
            incorrect: candidate.incorrect,
            correction: candidate.correction,
            rule: candidate.rule,
            explanation: candidate.explanation,
            category: candidate.category,
            kind: candidate.kind,
            confidence: candidate.confidence,
            alternate_reasons: Vec::new(),
        }
    }
}

pub(crate) fn add_context_diagnostics(
    text: &str,
    tokens: &[AnalyzedToken],
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for sentence in sentence_spans(text, tokens) {
        let mut candidates = sentence_context_candidates(text, tokens, sentence);
        candidates.sort_by(|a, b| {
            b.confidence
                .total_cmp(&a.confidence)
                .then_with(|| b.role_hint.rank().cmp(&a.role_hint.rank()))
                .then_with(|| b.span.1.cmp(&a.span.1))
                .then_with(|| a.span.0.cmp(&b.span.0))
        });

        for candidate in candidates {
            let span = candidate.span;
            if blocked_spans.contains(&span) || overlaps_existing_span(diagnostics, span) {
                continue;
            }

            let kind = candidate.kind;
            diagnostics.push(candidate.into());
            if !matches!(kind, DiagnosticKind::Ambiguous) {
                blocked_spans.insert(span);
            }
        }
    }
}

fn sentence_spans(text: &str, tokens: &[AnalyzedToken]) -> Vec<SentenceSpan> {
    if tokens.is_empty() {
        return Vec::new();
    }

    let mut spans = Vec::new();
    let mut start_token = 0;

    for idx in 0..tokens.len() {
        let gap_end = tokens.get(idx + 1).map_or(text.len(), |next| next.start);
        if has_sentence_boundary(&text[tokens[idx].end..gap_end]) {
            spans.push(SentenceSpan {
                start_token,
                end_token: idx + 1,
            });
            start_token = idx + 1;
        }
    }

    if start_token < tokens.len() {
        spans.push(SentenceSpan {
            start_token,
            end_token: tokens.len(),
        });
    }

    spans
}

fn has_sentence_boundary(gap: &str) -> bool {
    gap.chars().any(is_sentence_terminal_char)
}

fn is_sentence_terminal_char(ch: char) -> bool {
    matches!(ch, '।' | '.' | '!' | '?' | '؟')
}

fn sentence_context_candidates(
    text: &str,
    tokens: &[AnalyzedToken],
    sentence: SentenceSpan,
) -> Vec<ContextCandidate> {
    let mut out = Vec::new();

    if let Some(candidate) = phrase_backed_final_hos_candidate(text, tokens, sentence) {
        out.push(candidate);
        return out;
    }

    if let Some(candidate) = structural_final_hos_candidate(text, tokens, sentence) {
        out.push(candidate);
    }

    out
}

fn phrase_backed_final_hos_candidate(
    text: &str,
    tokens: &[AnalyzedToken],
    sentence: SentenceSpan,
) -> Option<ContextCandidate> {
    let last_idx = sentence.end_token.checked_sub(1)?;
    let last_surface = tokens[last_idx].surface();
    if last_surface.as_ref() != "होस" || sentence.end_token - sentence.start_token < 2 {
        return None;
    }

    let prev_idx = last_idx.checked_sub(1)?;
    let prev_surface = tokens[prev_idx].surface();
    let phrase_candidate = format!("{prev_surface} होस्");
    let lex = kosha();
    if !lex.contains(&phrase_candidate) && lex.lookup(&phrase_candidate).is_none() {
        return None;
    }

    let span = (tokens[last_idx].start, tokens[last_idx].end);
    Some(ContextCandidate {
        span,
        incorrect: text[span.0..span.1].to_string(),
        correction: "होस्".to_string(),
        rule: Rule::VarnaVinyasNiyam("3(ङ)-context-होस्"),
        explanation: "वाक्यको अन्त्यमा कामना/आशीर्वादसूचक प्रयोगमा 'होस्' लेखिन्छ".to_string(),
        category: DiagnosticCategory::Halanta,
        kind: DiagnosticKind::Error,
        confidence: 0.97,
        role_hint: ContextRoleHint::FinalPredicate,
    })
}

fn structural_final_hos_candidate(
    text: &str,
    tokens: &[AnalyzedToken],
    sentence: SentenceSpan,
) -> Option<ContextCandidate> {
    let last_idx = sentence.end_token.checked_sub(1)?;
    let last_surface = tokens[last_idx].surface();
    if last_surface.as_ref() != "होस" || sentence.end_token - sentence.start_token < 2 {
        return None;
    }

    let prev_idx = last_idx.checked_sub(1)?;
    let prev_surface = tokens[prev_idx].surface();
    if !looks_like_final_predicate_context(prev_surface.as_ref()) {
        return None;
    }

    let span = (tokens[last_idx].start, tokens[last_idx].end);
    Some(ContextCandidate {
        span,
        incorrect: text[span.0..span.1].to_string(),
        correction: "होस्".to_string(),
        rule: Rule::VarnaVinyasNiyam("3(ङ)-context-होस्"),
        explanation: "वाक्यको अन्त्यमा कामना/काम्य अर्थ दिने क्रियापदमा 'होस्' लेखिन्छ".to_string(),
        category: DiagnosticCategory::Halanta,
        kind: DiagnosticKind::Error,
        confidence: 0.89,
        role_hint: ContextRoleHint::FinalPredicate,
    })
}

fn looks_like_final_predicate_context(prev_surface: &str) -> bool {
    if looks_like_case_marked_or_possessive(prev_surface) {
        return false;
    }
    if looks_like_finite_verb(prev_surface) {
        return false;
    }
    true
}

fn looks_like_case_marked_or_possessive(surface: &str) -> bool {
    surface.ends_with("को")
        || surface.ends_with("का")
        || surface.ends_with("की")
        || matches!(
            surface,
            "मेरो"
                | "तेरो"
                | "उसको"
                | "उनको"
                | "हाम्रो"
                | "तिम्रो"
                | "तपाईंको"
                | "आफ्नो"
                | "यिनको"
                | "तिनको"
        )
}

fn looks_like_finite_verb(surface: &str) -> bool {
    surface.ends_with("छ")
        || surface.ends_with("छन्")
        || surface.ends_with("छु")
        || surface.ends_with("छौ")
        || surface.ends_with("थियो")
        || surface.ends_with("गयो")
        || surface.ends_with("आयो")
        || surface.ends_with("भयो")
        || surface.ends_with("गर्यो")
        || surface.ends_with("गर्छ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::tokenize_analyzed;

    #[test]
    fn sentence_spans_split_on_terminal_punctuation() {
        let text = "नेपाल राम्रो हो। यहाँ शान्ति होस";
        let tokens = tokenize_analyzed(text);

        let spans = sentence_spans(text, &tokens);
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].start_token, 0);
        assert_eq!(spans[0].end_token, 3);
        assert_eq!(spans[1].start_token, 3);
        assert_eq!(spans[1].end_token, 6);
    }

    #[test]
    fn structural_final_hos_rejects_possessive_nominal_context() {
        let text = "उसको होस";
        let tokens = tokenize_analyzed(text);
        let sentence = SentenceSpan {
            start_token: 0,
            end_token: 2,
        };

        assert!(structural_final_hos_candidate(text, &tokens, sentence).is_none());
    }
}
