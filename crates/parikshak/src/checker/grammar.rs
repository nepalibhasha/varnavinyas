use std::collections::HashSet;

use varnavinyas_prakriya::{DiagnosticKind, Rule};

use crate::diagnostic::{Diagnostic, DiagnosticCategory};
use crate::tokenizer::AnalyzedToken;

const QUANTIFIER_WORDS: &[&str] = &["धेरै", "सबै", "केही", "अनेक", "धेरैजसो"];
const INTRANSITIVE_VERB_FORMS: &[&str] = &[
    "छ",
    "थियो",
    "गयो",
    "जान्छ",
    "आयो",
    "आउँछ",
    "बस्यो",
    "हिँड्यो",
    "सुत्यो",
    "पुग्यो",
];
const MIN_SUFFIX_HEURISTIC_CONFIDENCE: f32 = 0.80;

pub(crate) fn add_grammar_diagnostics(
    tokens: &[AnalyzedToken],
    blocked_spans: &HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    use varnavinyas_vyakaran::MorphAnalyzer;

    let analyzer = varnavinyas_vyakaran::RuleBasedAnalyzer;

    for (idx, token) in tokens.iter().enumerate() {
        let span = (token.start, token.end);
        if blocked_spans.contains(&span) {
            continue;
        }

        let full = token.surface().into_owned();

        if let Ok(analyses) = analyzer.analyze(&full) {
            if analyses.len() > 1 {
                diagnostics.push(Diagnostic {
                    span,
                    incorrect: full.clone(),
                    correction: full.clone(),
                    rule: Rule::Vyakaran("morph-ambiguity"),
                    explanation: "व्याकरण विश्लेषण अस्पष्ट: एकभन्दा बढी सम्भावित संरचना".to_string(),
                    category: DiagnosticCategory::ShuddhaTable,
                    kind: DiagnosticKind::Ambiguous,
                    confidence: 0.55,
                    alternate_reasons: Vec::new(),
                });
            }
        }

        if has_plural_suffix(&full) && idx > 0 && is_quantifier(tokens[idx - 1].surface().as_ref())
        {
            let confidence = 0.62;
            if confidence >= MIN_SUFFIX_HEURISTIC_CONFIDENCE {
                let singular = strip_plural_suffix(&full).unwrap_or(&full).to_string();
                push_best_grammar_variant(
                    diagnostics,
                    Diagnostic {
                        span,
                        incorrect: full.clone(),
                        correction: singular,
                        rule: Rule::Vyakaran("quantifier-plural-redundancy"),
                        explanation: "परिमाणबोधक शब्दपछि बहुवचन -हरु/-हरू प्रायः अनावश्यक हुन्छ।"
                            .to_string(),
                        category: DiagnosticCategory::ShuddhaTable,
                        kind: DiagnosticKind::Variant,
                        confidence,
                        alternate_reasons: Vec::new(),
                    },
                );
            }
        }

        if has_ergative_suffix(token) && sentence_has_intransitive_predicate(tokens, idx) {
            let confidence = 0.68;
            if confidence >= MIN_SUFFIX_HEURISTIC_CONFIDENCE {
                push_best_grammar_variant(
                    diagnostics,
                    Diagnostic {
                        span,
                        incorrect: full.clone(),
                        correction: token.stem.clone(),
                        rule: Rule::Vyakaran("ergative-le-intransitive"),
                        explanation: "सामान्य अकर्मक क्रियासँग कर्तामा ले प्रायः प्रयोग हुँदैन।".to_string(),
                        category: DiagnosticCategory::ShuddhaTable,
                        kind: DiagnosticKind::Variant,
                        confidence,
                        alternate_reasons: Vec::new(),
                    },
                );
            }
        }

        if let Some(suggested_suffix) = suggested_genitive_suffix(token, tokens.get(idx + 1)) {
            let confidence = 0.64;
            if confidence >= MIN_SUFFIX_HEURISTIC_CONFIDENCE {
                push_best_grammar_variant(
                    diagnostics,
                    Diagnostic {
                        span,
                        incorrect: full.clone(),
                        correction: format!("{}{}", token.stem, suggested_suffix),
                        rule: Rule::Vyakaran("genitive-mismatch-plural"),
                        explanation: "बहुवचन संज्ञा अघि सामान्यतया सम्बन्ध सूचक का प्रयोग उपयुक्त हुन्छ।"
                            .to_string(),
                        category: DiagnosticCategory::ShuddhaTable,
                        kind: DiagnosticKind::Variant,
                        confidence,
                        alternate_reasons: Vec::new(),
                    },
                );
            }
        }

        let candidates = varnavinyas_samasa::analyze_compound(&full);
        if let Some(top) = candidates.first() {
            if top.score >= 0.75 {
                push_best_grammar_variant(
                    diagnostics,
                    Diagnostic {
                        span,
                        incorrect: full.clone(),
                        correction: format!("{} + {}", top.left, top.right),
                        rule: Rule::Vyakaran("samasa-heuristic"),
                        explanation: format!(
                            "समास सम्भावना ({:?}): {}",
                            top.samasa_type, top.vigraha
                        ),
                        category: DiagnosticCategory::Sandhi,
                        kind: DiagnosticKind::Variant,
                        confidence: top.score.min(0.9),
                        alternate_reasons: Vec::new(),
                    },
                );
            }
        }
    }
}

fn push_best_grammar_variant(diagnostics: &mut Vec<Diagnostic>, candidate: Diagnostic) {
    let existing = diagnostics.iter_mut().find(|d| {
        d.span == candidate.span
            && matches!(d.kind, DiagnosticKind::Variant)
            && matches!(d.rule, Rule::Vyakaran(_))
    });

    if let Some(diag) = existing {
        if candidate.confidence > diag.confidence {
            *diag = candidate;
        }
    } else {
        diagnostics.push(candidate);
    }
}

fn has_plural_suffix(word: &str) -> bool {
    word.ends_with("हरू") || word.ends_with("हरु")
}

fn strip_plural_suffix(word: &str) -> Option<&str> {
    word.strip_suffix("हरू").or_else(|| word.strip_suffix("हरु"))
}

fn is_quantifier(word: &str) -> bool {
    QUANTIFIER_WORDS.contains(&word)
}

fn has_ergative_suffix(token: &AnalyzedToken) -> bool {
    token.suffix.as_deref() == Some("ले")
}

fn sentence_has_intransitive_predicate(tokens: &[AnalyzedToken], subject_idx: usize) -> bool {
    tokens
        .iter()
        .skip(subject_idx + 1)
        .any(|tok| is_intransitive_verb_form(tok.surface().as_ref()))
}

fn is_intransitive_verb_form(word: &str) -> bool {
    INTRANSITIVE_VERB_FORMS.contains(&word)
}

fn suggested_genitive_suffix(
    token: &AnalyzedToken,
    next_token: Option<&AnalyzedToken>,
) -> Option<String> {
    let suffix = token.suffix.as_deref()?;
    if suffix == "का" || !matches!(suffix, "को" | "की") {
        return None;
    }

    let next = next_token?;
    if has_plural_suffix(next.surface().as_ref()) {
        Some("का".to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_variant(span: (usize, usize), rule_code: &'static str, confidence: f32) -> Diagnostic {
        Diagnostic {
            span,
            incorrect: "x".to_string(),
            correction: "y".to_string(),
            rule: Rule::Vyakaran(rule_code),
            explanation: "heuristic".to_string(),
            category: DiagnosticCategory::ShuddhaTable,
            kind: DiagnosticKind::Variant,
            confidence,
            alternate_reasons: Vec::new(),
        }
    }

    #[test]
    fn keeps_highest_confidence_variant_per_span() {
        let mut diagnostics = Vec::new();

        push_best_grammar_variant(
            &mut diagnostics,
            mk_variant((3, 12), "quantifier-plural-redundancy", 0.62),
        );
        push_best_grammar_variant(
            &mut diagnostics,
            mk_variant((3, 12), "samasa-heuristic", 0.86),
        );

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule, Rule::Vyakaran("samasa-heuristic"));
        assert_eq!(diagnostics[0].confidence, 0.86);
    }

    #[test]
    fn keeps_variants_for_different_spans() {
        let mut diagnostics = Vec::new();

        push_best_grammar_variant(
            &mut diagnostics,
            mk_variant((0, 6), "quantifier-plural-redundancy", 0.62),
        );
        push_best_grammar_variant(
            &mut diagnostics,
            mk_variant((7, 14), "ergative-le-intransitive", 0.68),
        );

        assert_eq!(diagnostics.len(), 2);
    }
}
