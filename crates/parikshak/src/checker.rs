use std::collections::HashSet;

use varnavinyas_kosha::kosha;
use varnavinyas_lekhya::check_punctuation;
use varnavinyas_prakriya::DiagnosticKind;
use varnavinyas_prakriya::{Rule, derive};

use crate::diagnostic::{Diagnostic, DiagnosticCategory};
#[cfg(feature = "grammar-pass")]
use crate::tokenizer::AnalyzedToken;
use crate::tokenizer::tokenize_analyzed;

#[cfg(feature = "grammar-pass")]
const QUANTIFIER_WORDS: &[&str] = &["धेरै", "सबै", "केही", "अनेक", "धेरैजसो"];

#[cfg(feature = "grammar-pass")]
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

/// Baseline padayog/padabiyog phrase corrections from Section 3(घ).
/// This set is intentionally conservative and deterministic.
const PADAYOG_PHRASE_CORRECTIONS: &[(&str, &str, &str)] = &[
    ("घर तिर", "घरतिर", "नामयोगी जोडेर लेख्नुपर्छ"),
    ("तिमी भन्दा", "तिमीभन्दा", "नामयोगी जोडेर लेख्नुपर्छ"),
    ("कोठा भित्र", "कोठाभित्र", "नामयोगी जोडेर लेख्नुपर्छ"),
    ("बिना काम", "बिनाकाम", "नामयोगी जोडेर लेख्नुपर्छ"),
    ("म सँग", "मसँग", "नामयोगी जोडेर लेख्नुपर्छ"),
    ("आज्ञा अनुसार", "आज्ञाअनुसार", "नामयोगी जोडेर लेख्नुपर्छ"),
    ("हामी बाहेक", "हामीबाहेक", "नामयोगी जोडेर लेख्नुपर्छ"),
    ("त्यस अन्तर्गत", "त्यसअन्तर्गत", "नामयोगी जोडेर लेख्नुपर्छ"),
    ("भने बमोजिम", "भनेबमोजिम", "नामयोगी जोडेर लेख्नुपर्छ"),
];

/// Section 4 phrase/sentence-level style variants.
/// These are guidance suggestions, not hard errors.
const STYLE_VARIANT_CORRECTIONS: &[(&str, &str, &str)] = &[
    (
        "मर्माहित भएको",
        "मर्माहत भएको",
        "शब्द-रूपगत प्रयोगमा मर्माहत रूप उपयुक्त हुन्छ",
    ),
    (
        "निर्देशित गरेको",
        "निर्देशन गरेको",
        "पदावली प्रयोगमा निर्देशन रूप उपयुक्त हुन्छ",
    ),
    (
        "इमानदारिता देखाउनु",
        "इमानदारी देखाउनु",
        "पदावली प्रयोगमा इमानदारी रूप प्रचलित छ",
    ),
    (
        "भन्नुभएको कुरा",
        "भनेको कुरा",
        "पदावली प्रयोगमा भनेको रूप सिफारिस गरिन्छ",
    ),
    (
        "पढ्नुभएको किताब",
        "पढेको किताब",
        "पदावली प्रयोगमा पढेको रूप सिफारिस गरिन्छ",
    ),
    (
        "कार्यक्रमको सम्बन्धमा",
        "कार्यक्रमका सम्बन्धमा",
        "सम्बन्धमा अघि बहुवचन कारकमा का उपयुक्त हुन्छ",
    ),
    (
        "सूचनाको आधारमा",
        "सूचनाका आधारमा",
        "आधारमा अघि बहुवचन कारकमा का उपयुक्त हुन्छ",
    ),
    (
        "उपस्थितिको बारेमा",
        "उपस्थितिका बारेमा",
        "बारेमा अघि बहुवचन कारकमा का उपयुक्त हुन्छ",
    ),
    (
        "अपहरित भएको",
        "अपहरण भएको",
        "प्रयोगगत रूपमा अपहरण भएको सिफारिस गरिन्छ",
    ),
    (
        "संरक्षित गरिएको",
        "संरक्षण गरिएको",
        "प्रयोगगत रूपमा संरक्षण गरिएको सिफारिस गरिन्छ",
    ),
    (
        "प्रसारित गरिएको",
        "प्रसारण गरिएको",
        "प्रयोगगत रूपमा प्रसारण गरिएको सिफारिस गरिन्छ",
    ),
    (
        "कामको लागि",
        "कामका लागि",
        "प्रयोगगत रूपमा कामका लागि सिफारिस गरिन्छ",
    ),
    (
        "देशको निम्ति",
        "देशका निम्ति",
        "प्रयोगगत रूपमा देशका निम्ति सिफारिस गरिन्छ",
    ),
    (
        "म सबैलाई हार्दिक स्वागत गर्न चाहन्छु",
        "म सबैलाई हार्दिक स्वागत गर्छु",
        "वक्तव्य शैलीमा प्रत्यक्ष स्वागत गर्छु रूप स्पष्ट हुन्छ",
    ),
    (
        "म अब कार्यक्रम सञ्चालन गर्न गइरहेको छु वा जाँदै छु",
        "म अब कार्यक्रम सञ्चालन गर्दै छु",
        "वाक्यगत सटीकता: सञ्चालन गर्दै छु रूप स्पष्ट र संक्षिप्त हुन्छ",
    ),
    (
        "अब यो प्रसारणका प्रमुख समाचारहरू सुन्नुहोस्",
        "अब यस प्रसारणका प्रमुख समाचारहरू सुन्नुहोस्",
        "तिर्यक् कारक प्रसङ्गमा यो -> यस रूप उपयुक्त हुन्छ",
    ),
    (
        "म यस कार्यक्रम यहाँ अन्त्य गर्दछु",
        "म यो कार्यक्रम यहीँ अन्त्य गर्दछु",
        "सरल कारक प्रयोगमा यो/यहीँ रूप उपयुक्त हुन्छ",
    ),
    (
        "लाखौँ नेपालका जनता गरिबीको रेखामुनि छन्",
        "नेपालका लाखौँ जनता गरिबीको रेखामुनि छन्",
        "पदक्रम मिलाउन नेपालका लाखौँ जनता रूप उपयुक्त हुन्छ",
    ),
    (
        "नेपाल मानव अधिकार आयोगद्वारा आयोजित टीकापुर हत्याकाण्डसम्बन्धी छलफल कार्यक्रममा मन्त्रीज्यूले पनि बोल्नुभयो",
        "टीकापुर हत्याकाण्डसम्बन्धी नेपाल मानव अधिकार आयोगद्वारा आयोजित छलफल कार्यक्रममा मन्त्रीज्यूले पनि बोल्नुभयो",
        "वाक्यगत अर्थ-स्पष्टताका लागि घटकहरूको पदक्रम मिलाउनु उपयुक्त हुन्छ",
    ),
    (
        "स्थानीय जनशक्तिको श्रमदानबाट दश किलोमिटर लामो गाडी गुड्न सक्ने सडक निर्माण गरियो",
        "स्थानीय जनशक्तिको श्रमदानबाट गाडी गुड्न सक्ने दश किलोमिटर लामो सडक निर्माण गरियो",
        "वाक्यमा विशेषण/विशेष्यको सम्बन्ध स्पष्ट राख्न पदक्रम मिलाउनु उपयुक्त हुन्छ",
    ),
    (
        "यहाँको सहयोगप्रति म कृतघ्न छु",
        "यहाँको सहयोगप्रति म कृतज्ञ छु",
        "कृतघ्न र कृतज्ञ अर्थ भिन्न छन्",
    ),
    (
        "ऊ राजनीतिमा निर्लिप्त छ",
        "ऊ राजनीतिमा लिप्त छ",
        "निर्लिप्त र लिप्त अर्थ भिन्न छन्",
    ),
];

/// Runtime options for `check_text_with_options`.
#[derive(Debug, Clone, Copy, Default)]
pub struct CheckOptions {
    /// Enable optional grammar-aware heuristics.
    ///
    /// This only has effect when compiled with the `grammar-pass` feature.
    pub grammar: bool,
}

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
        let category = prakriya
            .category
            .map(DiagnosticCategory::from_rule_category)
            .unwrap_or_else(|| DiagnosticCategory::from_rule(&rule));

        return Some(Diagnostic {
            span: (0, word.len()),
            incorrect: word.to_string(),
            correction: prakriya.output,
            rule,
            explanation,
            category,
            kind: prakriya.kind,
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

/// Check full text with runtime options.
pub fn check_text_with_options(text: &str, options: CheckOptions) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut blocked_spans: HashSet<(usize, usize)> = HashSet::new();

    // Word-level checks (suffix-aware: checks stem, spans full token)
    let tokens = tokenize_analyzed(text);
    let lex = kosha();
    for token in &tokens {
        // If the full token (stem+suffix) is a known word, skip correction.
        // e.g. "संसदमा" = संसद + मा — the stem "संसद" triggers a halanta rule,
        // but the agglutinative form "संसदमा" is a valid word in the lexicon.
        if let Some(ref sfx) = token.suffix {
            let full = format!("{}{}", token.stem, sfx);
            if lex.contains(&full) {
                continue;
            }
        }

        if let Some(mut diag) = check_word(&token.stem) {
            diag.span = (token.start, token.end);

            // If a suffix was detached, reattach it to the diagnostic strings.
            // The span covers the full token (stem+suffix), so the correction
            // must also be the full form to avoid data loss on replacement.
            if let Some(ref sfx) = token.suffix {
                diag.incorrect.push_str(sfx);
                diag.correction.push_str(sfx);
            }

            blocked_spans.insert(diag.span);
            diagnostics.push(diag);
        }
    }

    add_padayog_phrase_diagnostics(text, &mut blocked_spans, &mut diagnostics);

    if options.grammar {
        add_style_variant_diagnostics(text, &mut blocked_spans, &mut diagnostics);
    }

    #[cfg(feature = "grammar-pass")]
    if options.grammar {
        add_grammar_diagnostics(&tokens, &blocked_spans, &mut diagnostics);
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

fn add_padayog_phrase_diagnostics(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for &(incorrect, correct, explanation) in PADAYOG_PHRASE_CORRECTIONS {
        for (start, _) in text.match_indices(incorrect) {
            let end = start + incorrect.len();
            let span = (start, end);

            if blocked_spans.contains(&span) || overlaps_existing_span(diagnostics, span) {
                continue;
            }
            if !is_word_boundary(text, start, end) {
                continue;
            }

            diagnostics.push(Diagnostic {
                span,
                incorrect: incorrect.to_string(),
                correction: correct.to_string(),
                rule: Rule::VarnaVinyasNiyam("3(घ)"),
                explanation: format!("पदयोग/पदवियोग: {explanation}"),
                category: DiagnosticCategory::ShuddhaTable,
                kind: DiagnosticKind::Error,
                confidence: 0.95,
            });
            blocked_spans.insert(span);
        }
    }
}

fn add_style_variant_diagnostics(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for &(incorrect, correct, explanation) in STYLE_VARIANT_CORRECTIONS {
        for (start, _) in text.match_indices(incorrect) {
            let end = start + incorrect.len();
            let span = (start, end);

            if blocked_spans.contains(&span) || overlaps_existing_span(diagnostics, span) {
                continue;
            }
            if !is_word_boundary(text, start, end) {
                continue;
            }

            diagnostics.push(Diagnostic {
                span,
                incorrect: incorrect.to_string(),
                correction: correct.to_string(),
                rule: Rule::Vyakaran("section4-phrase-style"),
                explanation: format!("Section 4 शैली सुझाव: {explanation}"),
                category: DiagnosticCategory::ShuddhaTable,
                kind: DiagnosticKind::Variant,
                confidence: 0.78,
            });
            blocked_spans.insert(span);
        }
    }
}

fn overlaps_existing_span(diagnostics: &[Diagnostic], candidate: (usize, usize)) -> bool {
    diagnostics
        .iter()
        .any(|d| d.span.0 < candidate.1 && candidate.0 < d.span.1)
}

fn is_word_boundary(text: &str, start: usize, end: usize) -> bool {
    let prev_ok = if start == 0 {
        true
    } else {
        text[..start]
            .chars()
            .next_back()
            .is_none_or(is_boundary_char)
    };

    let next_ok = if end >= text.len() {
        true
    } else {
        text[end..].chars().next().is_none_or(is_boundary_char)
    };

    prev_ok && next_ok
}

fn is_boundary_char(c: char) -> bool {
    c.is_whitespace()
        || matches!(
            c,
            '.' | ','
                | '!'
                | '?'
                | ';'
                | ':'
                | '-'
                | '('
                | ')'
                | '['
                | ']'
                | '{'
                | '}'
                | '"'
                | '\''
                | '/'
                | '।'
                | '…'
        )
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
fn add_grammar_diagnostics(
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

        let full = token_full_form(token);

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
                });
            }
        }

        if has_plural_suffix(&full) && idx > 0 && is_quantifier(&token_full_form(&tokens[idx - 1]))
        {
            let singular = strip_plural_suffix(&full).unwrap_or(&full).to_string();
            push_best_grammar_variant(
                diagnostics,
                Diagnostic {
                    span,
                    incorrect: full.clone(),
                    correction: singular,
                    rule: Rule::Vyakaran("quantifier-plural-redundancy"),
                    explanation: "परिमाणबोधक शब्दपछि बहुवचन -हरु/-हरू प्रायः अनावश्यक हुन्छ।".to_string(),
                    category: DiagnosticCategory::ShuddhaTable,
                    kind: DiagnosticKind::Variant,
                    confidence: 0.62,
                },
            );
        }

        if has_ergative_suffix(token) && sentence_has_intransitive_predicate(tokens, idx) {
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
                    confidence: 0.68,
                },
            );
        }

        if let Some(suggested_suffix) = suggested_genitive_suffix(token, tokens.get(idx + 1)) {
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
                    confidence: 0.64,
                },
            );
        }

        // Optional samasa hint: expose high-confidence split as variant guidance.
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
                    },
                );
            }
        }
    }
}

#[cfg(feature = "grammar-pass")]
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

#[cfg(feature = "grammar-pass")]
fn has_plural_suffix(word: &str) -> bool {
    word.ends_with("हरू") || word.ends_with("हरु")
}

#[cfg(feature = "grammar-pass")]
fn strip_plural_suffix(word: &str) -> Option<&str> {
    word.strip_suffix("हरू").or_else(|| word.strip_suffix("हरु"))
}

#[cfg(feature = "grammar-pass")]
fn is_quantifier(word: &str) -> bool {
    QUANTIFIER_WORDS.contains(&word)
}

#[cfg(feature = "grammar-pass")]
fn has_ergative_suffix(token: &AnalyzedToken) -> bool {
    token.suffix.as_deref() == Some("ले")
}

#[cfg(feature = "grammar-pass")]
fn sentence_has_intransitive_predicate(tokens: &[AnalyzedToken], subject_idx: usize) -> bool {
    tokens
        .iter()
        .skip(subject_idx + 1)
        .any(|tok| is_intransitive_verb_form(&token_full_form(tok)))
}

#[cfg(feature = "grammar-pass")]
fn is_intransitive_verb_form(word: &str) -> bool {
    INTRANSITIVE_VERB_FORMS.contains(&word)
}

#[cfg(feature = "grammar-pass")]
fn suggested_genitive_suffix(
    token: &AnalyzedToken,
    next_token: Option<&AnalyzedToken>,
) -> Option<String> {
    let suffix = token.suffix.as_deref()?;
    if suffix == "का" || !matches!(suffix, "को" | "की") {
        return None;
    }

    let next = next_token?;
    if has_plural_suffix(&token_full_form(next)) {
        Some("का".to_string())
    } else {
        None
    }
}

#[cfg(feature = "grammar-pass")]
fn token_full_form(token: &AnalyzedToken) -> String {
    match &token.suffix {
        Some(sfx) => format!("{}{}", token.stem, sfx),
        None => token.stem.clone(),
    }
}

#[cfg(all(test, feature = "grammar-pass"))]
mod grammar_variant_refine_tests {
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
