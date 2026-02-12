use varnavinyas_kosha::kosha;
use varnavinyas_lekhya::check_punctuation;
use varnavinyas_prakriya::{Rule, derive};

use varnavinyas_prakriya::DiagnosticKind;

use crate::diagnostic::{Diagnostic, DiagnosticCategory};
use crate::tokenizer::tokenize_analyzed;

/// Check a single word and return a diagnostic if it's incorrect.
///
/// Pipeline:
/// 1. Run prakriya::derive — authoritative Academy rules always win
/// 2. If derive has no opinion, consult kosha lexicon:
///    - Known word → confirmed correct (None)
///    - Unknown word → not flagged (None) — we have no correction to offer
///
/// Derive runs first because the sabdasakha lexicon contains observed word
/// forms (including common misspellings like राजनैतिक). Academy correction
/// rules are authoritative and must override lexicon presence.
pub fn check_word(word: &str) -> Option<Diagnostic> {
    if word.is_empty() {
        return None;
    }

    // Step 1: Authoritative Academy correction rules always take priority.
    let prakriya = derive(word);
    if !prakriya.is_correct {
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
        let category = DiagnosticCategory::from_rule(&rule);

        return Some(Diagnostic {
            span: (0, word.len()),
            incorrect: word.to_string(),
            correction: prakriya.output,
            rule,
            explanation,
            category,
            kind: DiagnosticKind::Error,
            confidence: 1.0,
        });
    }

    // Step 2: Derive found no correction. Consult lexicon for validation.
    // A word in the lexicon is confirmed correct. A word absent from both
    // the correction rules and the lexicon is unknown — we don't flag it
    // because we have no correction to offer.
    let _in_lexicon = kosha().contains(word);

    None
}

/// Check a full text and return all diagnostics.
///
/// Pipeline:
/// 1. Tokenize into Devanagari word tokens
/// 2. For each token: derive (rules) → kosha (lexicon validation)
/// 3. Run lekhya punctuation checks
/// 4. Return all diagnostics sorted by span
pub fn check_text(text: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Word-level checks (suffix-aware: checks stem, spans full token)
    let tokens = tokenize_analyzed(text);
    for token in &tokens {
        if let Some(mut diag) = check_word(&token.stem) {
            diag.span = (token.start, token.end);
            
            // Fix: If a suffix was detached, reattach it to the diagnostic strings.
            // The span covers the full token (stem+suffix), so the correction
            // must also be the full form to avoid data loss on replacement.
            if let Some(ref sfx) = token.suffix {
                diag.incorrect.push_str(sfx);
                diag.correction.push_str(sfx);
            }

            diagnostics.push(diag);
        }
    }

    // Punctuation checks
    for lekhya_diag in check_punctuation(text) {
        diagnostics.push(Diagnostic {
            span: lekhya_diag.span,
            incorrect: lekhya_diag.found,
            correction: lekhya_diag.expected,
            rule: Rule::ChihnaNiyam("Section 5"),
            explanation: lekhya_diag.rule.to_string(),
            category: DiagnosticCategory::Punctuation,
            kind: DiagnosticKind::Error,
            confidence: 1.0,
        });
    }

    diagnostics.sort_by_key(|d| d.span.0);
    diagnostics
}
