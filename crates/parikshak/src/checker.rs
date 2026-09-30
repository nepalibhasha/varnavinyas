use std::collections::HashSet;

use varnavinyas_kosha::kosha;
use varnavinyas_prakriya::DiagnosticKind;
use varnavinyas_prakriya::Rule;
use varnavinyas_shabda::{best_analysis, has_supported_analysis};

use crate::diagnostic::Diagnostic;
use crate::tokenizer::{
    best_detachment_candidate, best_supported_detachment,
    should_prefer_whole_word_over_short_nipat_split, tokenize_analyzed,
};

mod arbitration;
mod common;
mod context;
#[cfg(feature = "grammar-pass")]
mod grammar;
mod orthography_variants;
mod padayog;
mod padayog_rules;
mod particles;
mod punctuation;
mod style_variants;
mod tiryak;
mod word_level;

use arbitration::resolve_diagnostic_overlaps;
use context::add_context_diagnostics;
#[cfg(feature = "grammar-pass")]
use grammar::add_grammar_diagnostics;
use orthography_variants::ACCEPTED_ORTHOGRAPHIC_VARIANTS;
use padayog::{add_generalized_padayog_padabiyog_diagnostics, add_padayog_padabiyog_diagnostics};
use punctuation::punctuation_diagnostics;
use style_variants::add_style_variant_diagnostics;
use tiryak::{add_tiryak_diagnostics, check_word_tiryak};
use word_level::{check_word_impl, check_word_rules};

/// Runtime options for `check_text_with_options`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PunctuationMode {
    #[default]
    Strict,
    NormalizedEditorial,
}

/// How prescriptive orthographic variants should be classified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OrthographyMode {
    /// Academy-prescriptive forms are hard errors.
    #[default]
    AcademyStrict,
    /// Reviewed common forms are accepted as editorial variants.
    CommonEditorial,
}

/// Runtime options for `check_text_with_options`.
#[derive(Debug, Clone, Copy, Default)]
pub struct CheckOptions {
    /// Enable optional grammar-aware heuristics.
    ///
    /// This only has effect when compiled with the `grammar-pass` feature.
    pub grammar: bool,
    /// How reviewed common-vs-strict orthographic variants should be classified.
    pub orthography_mode: OrthographyMode,
    /// How Section 5 punctuation diagnostics should be classified.
    pub punctuation_mode: PunctuationMode,
    /// Debug-only: include heuristic diagnostics that do not change text.
    pub include_noop_heuristics: bool,
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
pub fn check_word(word: &str) -> Option<Diagnostic> {
    check_word_with_options(word, CheckOptions::default())
}

/// Check a single word with runtime options.
pub fn check_word_with_options(word: &str, options: CheckOptions) -> Option<Diagnostic> {
    if let std::borrow::Cow::Owned(lookup) = varnavinyas_akshar::orthographic_lookup_form(word) {
        return check_word_with_options(&lookup, options).map(|mut diagnostic| {
            diagnostic.span = (0, word.len());
            diagnostic.incorrect = word.to_string();
            diagnostic
        });
    }

    if matches!(word, "नं") {
        return None;
    }

    if matches!(word, "भाको" | "नभाको") {
        return None;
    }

    if let Some(diag) = check_word_tiryak(word) {
        return Some(apply_orthography_mode(diag, options.orthography_mode));
    }

    let lex = kosha();
    if lex.contains(word) || lex.lookup(word).is_some() {
        return check_word_impl(word)
            .map(|diag| apply_orthography_mode(diag, options.orthography_mode));
    }

    // A safe whole-word correction beats guessed suffix detachment. Do not
    // promote an unsupported rewrite (e.g. रामकोपनि -> रामकोपनी) over an
    // independently supported case-marker/particle stack.
    if let Some(diag) =
        check_word_rules(word).filter(|diag| lex.is_correction_target(&diag.correction))
    {
        return Some(apply_orthography_mode(diag, options.orthography_mode));
    }

    if let Some(analysis) = best_analysis(word) {
        if !analysis.suffixes.is_empty() {
            let detached = word.strip_prefix(&analysis.stem).unwrap_or_default();
            if should_prefer_whole_word_over_short_nipat_split(&analysis.stem, detached, lex) {
                return check_word_impl(word)
                    .map(|diag| apply_orthography_mode(diag, options.orthography_mode));
            }
            if lex.contains(word) {
                return None;
            }

            if let Some(mut diag) = check_word_impl(&analysis.stem) {
                diag.span = (0, word.len());
                diag.incorrect = word.to_string();
                if let Some(detached) = word.strip_prefix(&analysis.stem) {
                    diag.correction.push_str(detached);
                }
                return Some(apply_orthography_mode(diag, options.orthography_mode));
            }
            return None;
        }
    }

    if let Some((stem, detached)) = best_supported_detachment(word, lex) {
        if should_prefer_whole_word_over_short_nipat_split(&stem, &detached, lex) {
            return check_word_impl(word)
                .map(|diag| apply_orthography_mode(diag, options.orthography_mode));
        }
        if let Some(mut diag) = check_word_impl(&stem) {
            diag.span = (0, word.len());
            diag.incorrect = word.to_string();
            diag.correction.push_str(&detached);
            return Some(apply_orthography_mode(diag, options.orthography_mode));
        }
    }

    if let Some((stem, detached)) = best_detachment_candidate(word, lex) {
        if should_prefer_whole_word_over_short_nipat_split(&stem, &detached, lex) {
            return check_word_impl(word)
                .map(|diag| apply_orthography_mode(diag, options.orthography_mode));
        }
        if let Some(mut diag) = check_word_impl(&stem) {
            let candidate = format!("{}{}", diag.correction, detached);
            if lex.contains(&candidate)
                || lex.lookup(&candidate).is_some()
                || has_supported_analysis(&candidate)
            {
                diag.span = (0, word.len());
                diag.incorrect = word.to_string();
                diag.correction = candidate;
                return Some(apply_orthography_mode(diag, options.orthography_mode));
            }
        }
    }

    check_word_impl(word).map(|diag| apply_orthography_mode(diag, options.orthography_mode))
}

fn apply_orthography_mode(mut diag: Diagnostic, mode: OrthographyMode) -> Diagnostic {
    if mode != OrthographyMode::CommonEditorial {
        return diag;
    }
    if let Some(variant) = ACCEPTED_ORTHOGRAPHIC_VARIANTS.iter().find(|variant| {
        diag.incorrect == variant.common
            && diag.correction == variant.strict
            && diag.category == variant.category
    }) {
        debug_assert!(!variant.source_note.is_empty());
        diag.kind = DiagnosticKind::Variant;
        diag.confidence = diag.confidence.min(0.72);
        diag.explanation = format!("{}। {}", variant.reason, diag.explanation);
    }
    diag
}

/// Check full text with runtime options.
pub fn check_text_with_options(text: &str, options: CheckOptions) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut blocked_spans: HashSet<(usize, usize)> = HashSet::new();

    // Word-level checks (suffix-aware: checks stem, spans full token)
    let tokens = tokenize_analyzed(text);
    let abbreviations = varnavinyas_lekhya::dotted_abbreviation_spans(text);
    for token in &tokens {
        if abbreviations
            .iter()
            .any(|&(start, end)| start <= token.start && token.end <= end)
        {
            continue;
        }
        // Match the public word-checking order: whole-word rules first, then
        // supported suffix detachment. A guessed suffix stack must not hide
        // an existing correction such as मीलेको -> मिलेको.
        if let Some(mut diag) = check_word_with_options(token.source_text(text), options) {
            diag.span = (token.start, token.end);

            if !matches!(diag.kind, DiagnosticKind::Ambiguous) {
                blocked_spans.insert(diag.span);
            }
            diagnostics.push(diag);
        }
    }

    add_tiryak_diagnostics(text, &tokens, &mut blocked_spans, &mut diagnostics);
    add_padayog_padabiyog_diagnostics(text, &mut blocked_spans, &mut diagnostics);
    add_generalized_padayog_padabiyog_diagnostics(
        text,
        &tokens,
        &mut blocked_spans,
        &mut diagnostics,
    );
    resolve_diagnostic_overlaps(&mut diagnostics);
    add_context_diagnostics(text, &tokens, &mut blocked_spans, &mut diagnostics);

    if options.grammar {
        add_style_variant_diagnostics(text, &tokens, &mut blocked_spans, &mut diagnostics);
    }

    #[cfg(feature = "grammar-pass")]
    if options.grammar {
        add_grammar_diagnostics(&tokens, &blocked_spans, &mut diagnostics);
    }

    resolve_diagnostic_overlaps(&mut diagnostics);

    // Punctuation checks
    let punctuation_kind = match options.punctuation_mode {
        PunctuationMode::Strict => DiagnosticKind::Error,
        PunctuationMode::NormalizedEditorial => DiagnosticKind::Variant,
    };
    let punctuation_confidence = match options.punctuation_mode {
        PunctuationMode::Strict => 1.0,
        PunctuationMode::NormalizedEditorial => 0.72,
    };

    diagnostics.extend(punctuation_diagnostics(
        text,
        punctuation_kind,
        punctuation_confidence,
    ));

    if !options.include_noop_heuristics {
        diagnostics.retain(|d| !is_noop_heuristic_diagnostic(d));
    }

    diagnostics.sort_by_key(|d| d.span.0);
    diagnostics
}

fn is_noop_heuristic_diagnostic(d: &Diagnostic) -> bool {
    if d.incorrect != d.correction {
        return false;
    }
    if !matches!(d.kind, DiagnosticKind::Variant | DiagnosticKind::Ambiguous) {
        return false;
    }
    matches!(d.rule, Rule::Vyakaran(_))
}

/// Check a full text and return all diagnostics.
///
/// Pipeline:
/// 1. Tokenize into Devanagari word tokens
/// 2. For each token: derive (rules) → kosha (lexicon validation)
/// 3. Run lekhya punctuation checks
/// 4. Return all diagnostics sorted by span
pub fn check_text(text: &str) -> Vec<Diagnostic> {
    check_text_with_options(text, CheckOptions::default())
}

#[cfg(feature = "grammar-pass")]
#[cfg(test)]
mod noop_heuristic_tests {
    use super::*;
    use crate::DiagnosticCategory;

    #[test]
    fn filters_noop_grammar_variant() {
        let d = Diagnostic {
            evidence: crate::DiagnosticEvidence::Generalized,
            span: (0, 6),
            incorrect: "सुनारलाई".to_string(),
            correction: "सुनारलाई".to_string(),
            rule: Rule::Vyakaran("morph-ambiguity"),
            explanation: "x".to_string(),
            category: DiagnosticCategory::ShuddhaTable,
            kind: DiagnosticKind::Variant,
            confidence: 0.55,
            alternate_reasons: Vec::new(),
        };
        assert!(is_noop_heuristic_diagnostic(&d));
    }

    #[test]
    fn keeps_non_noop_diagnostic() {
        let d = Diagnostic {
            evidence: crate::DiagnosticEvidence::Generalized,
            span: (0, 3),
            incorrect: "हरु".to_string(),
            correction: "हरू".to_string(),
            rule: Rule::VarnaVinyasNiyam("3(ई)"),
            explanation: "x".to_string(),
            category: DiagnosticCategory::HrasvaDirgha,
            kind: DiagnosticKind::Error,
            confidence: 1.0,
            alternate_reasons: Vec::new(),
        };
        assert!(!is_noop_heuristic_diagnostic(&d));
    }
}
