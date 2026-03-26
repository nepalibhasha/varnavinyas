use crate::tokenizer::is_supported_stem;
use varnavinyas_kosha::kosha;
use varnavinyas_prakriya::DiagnosticKind;
use varnavinyas_prakriya::{Rule, RuleHit, collect_rule_hits};

use crate::diagnostic::{
    Diagnostic, DiagnosticCategory, DiagnosticReason, choose_diagnostic_category,
};
use crate::tokenizer::AnalyzedToken;

const AMBIGUOUS_HALANTA_DHATU_FORMS: &[&str] = &["भन", "गर", "पढ", "हेर", "लेख", "बुझ", "लुक"];

fn is_numeric_token(word: &str) -> bool {
    let mut saw_digit = false;

    for ch in word.chars() {
        if ch.is_numeric() {
            saw_digit = true;
            continue;
        }

        if matches!(ch, ',' | '.' | '/' | '-' | ':' | '%' | '–' | '—') {
            continue;
        }

        return false;
    }

    saw_digit
}

fn lexically_supported(word: &str, lex: &varnavinyas_kosha::Kosha) -> bool {
    lex.contains(word) || lex.lookup(word).is_some()
}

fn has_known_compound_split(word: &str, lex: &varnavinyas_kosha::Kosha) -> bool {
    let total_chars = word.chars().count();
    if total_chars < 4 {
        return false;
    }

    for (idx, _) in word.char_indices().skip(1) {
        let left = &word[..idx];
        let right = &word[idx..];
        if left.chars().count() < 2 || right.chars().count() < 2 {
            continue;
        }
        if lexically_supported(left, lex) && lexically_supported(right, lex) {
            return true;
        }
    }

    false
}

fn has_known_infinitive_candidate(stem: &str, lex: &varnavinyas_kosha::Kosha) -> bool {
    let mut candidates = Vec::new();

    if stem.ends_with('्') {
        candidates.push(format!("{stem}नु"));
    } else {
        candidates.push(format!("{stem}नु"));
        candidates.push(format!("{stem}्नु"));
    }

    if stem.ends_with('ा') {
        candidates.push(format!("{stem}उनु"));
        candidates.push(format!("{stem}इनु"));
    }

    candidates
        .into_iter()
        .any(|candidate| lexically_supported(&candidate, lex))
}

fn has_supported_productive_verb_form(word: &str, lex: &varnavinyas_kosha::Kosha) -> bool {
    for suffix in ["दै", "ँदै"] {
        if let Some(stem) = word.strip_suffix(suffix) {
            if stem.chars().count() >= 2 && has_known_infinitive_candidate(stem, lex) {
                return true;
            }
        }
    }

    if let Some(stem) = word.strip_suffix("ेन") {
        if stem.chars().count() >= 2 && has_known_infinitive_candidate(stem, lex) {
            return true;
        }
    }

    if let Some(stem) = word.strip_suffix("इयो") {
        if stem.chars().count() >= 2 && has_known_infinitive_candidate(stem, lex) {
            return true;
        }
    }

    false
}

fn should_offer_nearby_suggestion(word: &str, suggestion: &str) -> bool {
    let word_chars: Vec<char> = word.chars().collect();
    let suggestion_chars: Vec<char> = suggestion.chars().collect();
    if word_chars.is_empty() || suggestion_chars.is_empty() {
        return false;
    }

    word_chars.first() == suggestion_chars.first() && word_chars.last() == suggestion_chars.last()
}

/// Check a single word and return a diagnostic if it's incorrect.
///
/// Pipeline:
/// 1. Run prakriya::derive — authoritative Academy rules always win
/// 2. If derive has no opinion, consult kosha lexicon:
///    - Known word → confirmed correct (None)
///    - Unknown with close lexicon near-match → flagged as Ambiguous
///    - Other unknown word → not flagged (None)
///
/// Derive runs first because the sabdasakha lexicon contains observed word
/// forms (including common misspellings like राजनैतिक). Academy correction
/// rules are authoritative and must override lexicon presence.
pub(crate) fn check_word_impl(word: &str) -> Option<Diagnostic> {
    if word.is_empty() {
        return None;
    }
    if is_numeric_token(word) {
        return None;
    }

    let hits = collect_rule_hits(word);
    if let Some(primary_hit) = hits.first() {
        let prakriya = &primary_hit.prakriya;
        let rule = prakriya
            .steps
            .first()
            .map(|s| s.rule)
            .unwrap_or(Rule::ShuddhaAshuddha("unknown"));
        let explanation = prakriya
            .steps
            .first()
            .map(|s| s.description.clone())
            .unwrap_or_default();
        let category = choose_diagnostic_category(prakriya.category, &rule);

        return Some(Diagnostic {
            span: (0, word.len()),
            incorrect: word.to_string(),
            correction: prakriya.output.clone(),
            rule,
            explanation,
            category,
            kind: prakriya.kind,
            confidence: 1.0,
            alternate_reasons: alternate_reasons_from_hits(&hits),
        });
    }

    let lex = kosha();
    if is_supported_stem(word, lex) {
        return None;
    }
    if has_known_compound_split(word, lex) || has_supported_productive_verb_form(word, lex) {
        return None;
    }

    if let Some(suggestion) = lex.suggest_nearby(word, 1) {
        if suggestion == word {
            return None;
        }
        if !should_offer_nearby_suggestion(word, &suggestion) {
            return None;
        }
        return Some(Diagnostic {
            span: (0, word.len()),
            incorrect: word.to_string(),
            correction: suggestion,
            rule: Rule::ShuddhaAshuddha("unknown"),
            explanation: "शब्द शब्दकोशमा भेटिएन; सम्भावित वर्तनी त्रुटि".to_string(),
            category: DiagnosticCategory::ShuddhaTable,
            kind: DiagnosticKind::Ambiguous,
            confidence: 0.72,
            alternate_reasons: Vec::new(),
        });
    }

    None
}

pub(crate) fn adjust_context_sensitive_nga_halanta_rule(
    idx: usize,
    tokens: &[AnalyzedToken],
    token: &AnalyzedToken,
    diag: &mut Diagnostic,
) {
    if !matches!(diag.rule, Rule::VarnaVinyasNiyam("3(ङ)-1")) {
        return;
    }
    if !AMBIGUOUS_HALANTA_DHATU_FORMS.contains(&token.stem.as_str()) {
        return;
    }
    if tokens.len() > 1 && idx < tokens.len() {
        diag.kind = DiagnosticKind::Ambiguous;
        diag.confidence = 0.55;
        diag.explanation =
            "यो रूप धातुरूप वा आज्ञार्थ दुवै सन्दर्भमा आउन सक्छ; सन्दर्भअनुसार जाँच गर्नुहोस्".to_string();
    }
}

pub(crate) fn alternate_reasons_from_hits(hits: &[RuleHit]) -> Vec<DiagnosticReason> {
    let mut reasons = Vec::new();
    for hit in hits.iter().skip(1) {
        let correction = hit.prakriya.output.clone();
        for step in &hit.prakriya.steps {
            let reason = DiagnosticReason::new(step.rule, step.description.clone())
                .with_correction(correction.clone())
                .with_category(hit.category);
            if !reasons.iter().any(|existing: &DiagnosticReason| {
                existing.rule == reason.rule
                    && existing.explanation == reason.explanation
                    && existing.correction == reason.correction
            }) {
                reasons.push(reason);
            }
        }
    }
    reasons
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostic::diagnostic_reason_category;
    use varnavinyas_prakriya::{Prakriya, RuleCategory, Step};

    #[test]
    fn builds_alternate_reasons_from_secondary_hits() {
        let hits = vec![
            RuleHit {
                spec_id: Some("primary"),
                priority: 10,
                category: RuleCategory::HrasvaDirgha,
                kind: DiagnosticKind::Error,
                prakriya: Prakriya::corrected(
                    "सूमार्ग",
                    "सुमार्ग",
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(क)(अ)-1"),
                        "उपसर्गबाट बनेका शब्दहरू ह्रस्व हुन्छन्",
                        "सूमार्ग",
                        "सुमार्ग",
                    )],
                )
                .with_metadata(RuleCategory::HrasvaDirgha, DiagnosticKind::Error),
            },
            RuleHit {
                spec_id: Some("secondary"),
                priority: 20,
                category: RuleCategory::ShaShaS,
                kind: DiagnosticKind::Error,
                prakriya: Prakriya::corrected(
                    "सूमार्ग",
                    "शुमार्ग",
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(ग)(अ)-1"),
                        "तालव्य श को प्रयोग",
                        "सूमार्ग",
                        "शुमार्ग",
                    )],
                )
                .with_metadata(RuleCategory::ShaShaS, DiagnosticKind::Error),
            },
        ];

        let reasons = alternate_reasons_from_hits(&hits);

        assert_eq!(reasons.len(), 1);
        assert_eq!(reasons[0].rule, Rule::VarnaVinyasNiyam("3(ग)(अ)-1"));
        assert_eq!(reasons[0].correction.as_deref(), Some("शुमार्ग"));
        assert_eq!(
            diagnostic_reason_category(&reasons[0]),
            DiagnosticCategory::ShaShaS
        );
    }
}
