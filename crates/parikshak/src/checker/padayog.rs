use std::collections::HashSet;

use varnavinyas_kosha::kosha;
use varnavinyas_prakriya::{DiagnosticKind, Rule};
use varnavinyas_shabda::has_supported_analysis;

use crate::diagnostic::{Diagnostic, DiagnosticCategory};

use super::common::{
    is_devanagari_word, is_numeric_segment, is_word_boundary, overlaps_existing_span,
    whitespace_segments,
};
use super::padayog_rules::PADAYOG_PADABIYOG_RULES;

const VIBHAKTI_TOKENS: &[&str] = &["ले", "लाई", "को", "बाट", "देखि", "मा", "का", "की"];
const PRATYAYA_TOKENS: &[&str] = &["ज्यू"];
const NAMAYOGI_TOKENS: &[&str] = &[
    "सँग",
    "तिर",
    "भन्दा",
    "भित्र",
    "प्रति",
    "अनुसार",
    "बाहेक",
    "अन्तर्गत",
    "बमोजिम",
];
const COMPARISON_SPLIT_TOKENS: &[&str] = &["जस्तो", "जस्तै", "जत्रो", "जसरी"];

pub(crate) fn add_padayog_padabiyog_diagnostics(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for rule in PADAYOG_PADABIYOG_RULES {
        for rw in rule.rewrites {
            for (start, _) in text.match_indices(rw.incorrect) {
                let end = start + rw.incorrect.len();
                let span = (start, end);

                if blocked_spans.contains(&span) || overlaps_existing_span(diagnostics, span) {
                    continue;
                }
                if !is_word_boundary(text, start, end) {
                    continue;
                }

                diagnostics.push(Diagnostic {
                    span,
                    incorrect: rw.incorrect.to_string(),
                    correction: rw.correct.to_string(),
                    rule: Rule::VarnaVinyasNiyam("3(घ)"),
                    explanation: format!(
                        "पदयोग/पदवियोग [{} | {}]: {}",
                        rule.code, rule.label, rw.explanation
                    ),
                    category: DiagnosticCategory::ShuddhaTable,
                    kind: DiagnosticKind::Error,
                    confidence: 0.95,
                    alternate_reasons: Vec::new(),
                });
                blocked_spans.insert(span);
            }
        }
    }
}

pub(crate) fn add_generalized_padayog_padabiyog_diagnostics(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    add_generalized_padayog_subrule_1_upasarga_join(text, blocked_spans, diagnostics);
    add_generalized_padayog_subrule_2_pratyaya_join(text, blocked_spans, diagnostics);
    add_generalized_padayog_subrule_3_vibhakti_join(text, blocked_spans, diagnostics);
    add_generalized_padayog_subrule_4_namayogi_join(text, blocked_spans, diagnostics);
    add_generalized_padayog_subrule_5_samasta_join(text, blocked_spans, diagnostics);
    add_generalized_padayog_subrule_6_nirarthak_dwitva_join(text, blocked_spans, diagnostics);
    add_generalized_padayog_subrule_7_akaran_n_join(text, blocked_spans, diagnostics);
    add_generalized_padayog_subrule_8_milit_kriyapad_join(text, blocked_spans, diagnostics);
    add_generalized_padayog_subrule_9_samyogak_join(text, blocked_spans, diagnostics);
    add_generalized_padayog_subrule_10_ota_varga_sambandhi_join(text, blocked_spans, diagnostics);
    add_generalized_padayog_subrule_11_sarah_join(text, blocked_spans, diagnostics);

    add_generalized_padabiyog_subrule_1_every_word_split(text, blocked_spans, diagnostics);
    add_generalized_padabiyog_subrule_2_vibhakti_pachhi_namayogi_split(
        text,
        blocked_spans,
        diagnostics,
    );
    add_generalized_padabiyog_subrule_3_lagi_nimti_split(text, blocked_spans, diagnostics);
    add_generalized_padabiyog_subrule_4_nipat_split(text, blocked_spans, diagnostics);
    add_generalized_padabiyog_subrule_5_n_samyogak_split(text, blocked_spans, diagnostics);
    add_generalized_padabiyog_subrule_6_aspect_split(text, blocked_spans, diagnostics);
    add_generalized_padabiyog_subrule_7_ne_cha_split(text, blocked_spans, diagnostics);
    add_generalized_padabiyog_subrule_8_samyukta_kriya_nipat_split(
        text,
        blocked_spans,
        diagnostics,
    );
    add_generalized_padabiyog_subrule_9_nu_n_pachhi_kriya_split(text, blocked_spans, diagnostics);
    add_generalized_padabiyog_subrule_10_sarthak_dwitva_split(text, blocked_spans, diagnostics);
    add_generalized_padabiyog_subrule_11_jana_thari_split(text, blocked_spans, diagnostics);
    add_generalized_padabiyog_subrule_12_shirsha_nam_split(text, blocked_spans, diagnostics);
    add_generalized_padabiyog_subrule_13_visheshan_nam_split(text, blocked_spans, diagnostics);
    add_generalized_saishanik_comparison_split(text, blocked_spans, diagnostics);
}

fn add_generalized_padayog_subrule_1_upasarga_join(
    _text: &str,
    _blocked_spans: &mut HashSet<(usize, usize)>,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    // TODO(3(घ)-पदयोग-१): generalized upasarga joining needs morphology-aware validation.
}

fn add_generalized_padayog_subrule_2_pratyaya_join(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    add_generalized_padayog_pratyaya_join(text, blocked_spans, diagnostics);
}

fn add_generalized_padayog_subrule_3_vibhakti_join(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    add_generalized_padayog_vibhakti_join(text, blocked_spans, diagnostics);
}

fn add_generalized_padayog_subrule_4_namayogi_join(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    add_generalized_padayog_layered_join(text, blocked_spans, diagnostics);
    add_generalized_padayog_namayogi_join(text, blocked_spans, diagnostics);
}

fn add_generalized_padayog_subrule_5_samasta_join(
    _text: &str,
    _blocked_spans: &mut HashSet<(usize, usize)>,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    // TODO(3(घ)-पदयोग-५): generalized samasta joining needs compound-aware ranking.
}

fn add_generalized_padayog_subrule_6_nirarthak_dwitva_join(
    _text: &str,
    _blocked_spans: &mut HashSet<(usize, usize)>,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    // TODO(3(घ)-पदयोग-६): generalized nirarthak dvitva joining needs reduplication-aware guards.
}

fn add_generalized_padayog_subrule_7_akaran_n_join(
    _text: &str,
    _blocked_spans: &mut HashSet<(usize, usize)>,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    // TODO(3(घ)-पदयोग-७): generalized akaran-'न' joining needs stronger sentence context.
}

fn add_generalized_padayog_subrule_8_milit_kriyapad_join(
    _text: &str,
    _blocked_spans: &mut HashSet<(usize, usize)>,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    // TODO(3(घ)-पदयोग-८): generalized milit kriyapad joining needs a dedicated verb-complex analyzer.
}

fn add_generalized_padayog_subrule_9_samyogak_join(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    add_generalized_padayog_conjunction_join(text, blocked_spans, diagnostics);
}

fn add_generalized_padayog_subrule_10_ota_varga_sambandhi_join(
    _text: &str,
    _blocked_spans: &mut HashSet<(usize, usize)>,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    // TODO(3(घ)-पदयोग-१०): generalized ota/varga/sambandhi joining needs curated semantic inventories.
}

fn add_generalized_padayog_subrule_11_sarah_join(
    _text: &str,
    _blocked_spans: &mut HashSet<(usize, usize)>,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    // TODO(3(घ)-पदयोग-११): generalized 'सरह' joining needs a safe comparison-particle inventory.
}

fn add_generalized_padabiyog_subrule_1_every_word_split(
    _text: &str,
    _blocked_spans: &mut HashSet<(usize, usize)>,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    // TODO(3(घ)-पदवियोग-१): broad baseline principle; not a standalone rewrite rule.
}

fn add_generalized_padabiyog_subrule_2_vibhakti_pachhi_namayogi_split(
    _text: &str,
    _blocked_spans: &mut HashSet<(usize, usize)>,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    // TODO(3(घ)-पदवियोग-२): generalized split after vibhakti needs phrase-structure awareness.
}

fn add_generalized_padabiyog_subrule_3_lagi_nimti_split(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    add_generalized_padabiyog_vibhakti_split(text, blocked_spans, diagnostics);
}

fn add_generalized_padabiyog_subrule_4_nipat_split(
    _text: &str,
    _blocked_spans: &mut HashSet<(usize, usize)>,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    // TODO(3(घ)-पदवियोग-४): generalized nipat splitting needs a dedicated particle inventory.
}

fn add_generalized_padabiyog_subrule_5_n_samyogak_split(
    _text: &str,
    _blocked_spans: &mut HashSet<(usize, usize)>,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    // TODO(3(घ)-पदवियोग-५): generalized 'न' conjunction splitting needs sentence-level coordination cues.
}

fn add_generalized_padabiyog_subrule_6_aspect_split(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    add_generalized_padabiyog_verb_complex_split(
        text,
        blocked_spans,
        diagnostics,
        true,
        false,
        false,
    );
}

fn add_generalized_padabiyog_subrule_7_ne_cha_split(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    add_generalized_padabiyog_verb_complex_split(
        text,
        blocked_spans,
        diagnostics,
        false,
        true,
        false,
    );
}

fn add_generalized_padabiyog_subrule_8_samyukta_kriya_nipat_split(
    _text: &str,
    _blocked_spans: &mut HashSet<(usize, usize)>,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    // TODO(3(घ)-पदवियोग-८): generalized split with intervening nipat needs multi-token verb analysis.
}

fn add_generalized_padabiyog_subrule_9_nu_n_pachhi_kriya_split(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    add_generalized_padabiyog_verb_complex_split(
        text,
        blocked_spans,
        diagnostics,
        false,
        false,
        true,
    );
}

fn add_generalized_padabiyog_subrule_10_sarthak_dwitva_split(
    _text: &str,
    _blocked_spans: &mut HashSet<(usize, usize)>,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    // TODO(3(घ)-पदवियोग-१०): generalized meaningful reduplication splitting needs lexical repetition guards.
}

fn add_generalized_padabiyog_subrule_11_jana_thari_split(
    _text: &str,
    _blocked_spans: &mut HashSet<(usize, usize)>,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    // TODO(3(घ)-पदवियोग-११): generalized classifier splitting needs a curated classifier inventory.
}

fn add_generalized_padabiyog_subrule_12_shirsha_nam_split(
    _text: &str,
    _blocked_spans: &mut HashSet<(usize, usize)>,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    // TODO(3(घ)-पदवियोग-१२): generalized title-name splitting needs named-entity inventories.
}

fn add_generalized_padabiyog_subrule_13_visheshan_nam_split(
    _text: &str,
    _blocked_spans: &mut HashSet<(usize, usize)>,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    // TODO(3(घ)-पदवियोग-१३): generalized adjective-noun splitting needs stronger syntax than spacing heuristics.
}

fn add_generalized_saishanik_comparison_split(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let segments = whitespace_segments(text);

    for (token, start, end) in segments {
        if !is_devanagari_word(token) || is_numeric_segment(token) {
            continue;
        }

        for &suffix in COMPARISON_SPLIT_TOKENS {
            let Some(left) = token.strip_suffix(suffix) else {
                continue;
            };
            if left.is_empty() {
                continue;
            }

            let Some(normalized_left) = normalize_joined_word(left) else {
                continue;
            };

            let correction = format!("{normalized_left} {suffix}");
            let span = (start, end);
            if blocked_spans.contains(&span) || overlaps_existing_span(diagnostics, span) {
                continue;
            }
            if !is_word_boundary(text, span.0, span.1) {
                continue;
            }
            if correction == token {
                continue;
            }
            if has_same_rewrite(diagnostics, span, &correction) {
                continue;
            }

            diagnostics.push(Diagnostic {
                span,
                incorrect: token.to_string(),
                correction,
                rule: Rule::VarnaVinyasNiyam("3(घ)"),
                explanation: "शैक्षणिक व्याकरण पदवियोग (झ): जस्तो/जस्तै/जत्रो/जसरी जस्ता तुलनाबोधक पदका अगाडि आउने नाम, सर्वनाम आदि अलग डिकामा लेखिन्छन्".to_string(),
                category: DiagnosticCategory::ShuddhaTable,
                kind: DiagnosticKind::Error,
                confidence: 0.92,
                alternate_reasons: Vec::new(),
            });
            blocked_spans.insert(span);
            break;
        }
    }
}

fn normalize_joined_word(word: &str) -> Option<String> {
    if let Some(diag) = super::check_word(word) {
        if !matches!(diag.kind, DiagnosticKind::Ambiguous) {
            return Some(diag.correction);
        }
    }

    if let Some(candidate) = normalize_namayogi_variant(word) {
        return Some(candidate);
    }

    let lex = kosha();
    if lex.contains(word) || lex.lookup(word).is_some() || has_supported_analysis(word) {
        return Some(word.to_string());
    }

    None
}

fn normalize_namayogi_variant(word: &str) -> Option<String> {
    const VARIANTS: &[(&str, &str)] = &[("संग", "सँग"), ("सङ", "सँग")];

    let lex = kosha();
    for &(prefix, canonical) in VARIANTS {
        let Some(rest) = word.strip_prefix(prefix) else {
            continue;
        };

        let candidate = format!("{canonical}{rest}");
        if candidate == word {
            continue;
        }

        if lex.contains(&candidate)
            || lex.lookup(&candidate).is_some()
            || has_supported_analysis(&candidate)
        {
            return Some(candidate);
        }
    }

    None
}

fn left_supports_pratyaya_join(left: &str) -> bool {
    let lex = kosha();
    lex.contains(left) || lex.lookup(left).is_some()
}

fn starts_with_namayogi(form: &str) -> bool {
    NAMAYOGI_TOKENS.iter().any(|token| form.starts_with(token))
}

fn has_same_rewrite(diagnostics: &[Diagnostic], span: (usize, usize), correction: &str) -> bool {
    diagnostics
        .iter()
        .any(|d| d.span == span && d.correction == correction)
}

fn add_generalized_padayog_layered_join(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let segments = whitespace_segments(text);

    for pair in segments.windows(2) {
        let (left, lstart, _) = pair[0];
        let (right, _, rend) = pair[1];
        if !is_devanagari_word(left) || !is_devanagari_word(right) {
            continue;
        }
        if is_numeric_segment(left) {
            continue;
        }
        let correction = if VIBHAKTI_TOKENS.contains(&right) {
            let Some(correction) = normalize_joined_word(&format!("{left}{right}")) else {
                continue;
            };
            correction
        } else if starts_with_namayogi(right) {
            let Some(correction) = normalize_joined_word(&format!("{left}{right}")) else {
                continue;
            };
            correction
        } else {
            continue;
        };

        let span = (lstart, rend);
        if !is_word_boundary(text, span.0, span.1) {
            continue;
        }
        if correction == text[span.0..span.1] {
            continue;
        }
        if has_same_rewrite(diagnostics, span, &correction) {
            continue;
        }

        let (subrule, explanation) = if starts_with_namayogi(right) {
            (
                "3(घ)-पदयोग-४",
                "पदयोग/पदवियोग [3(घ)-पदयोग-४ | नामयोगी जोडेर लेख्नुपर्छ]: भित्री जोडाइ मिलाएर नामयोगी पद क्रमशः जोडेर लेख्नुपर्छ",
            )
        } else {
            (
                "3(घ)-पदयोग-३",
                "पदयोग/पदवियोग [3(घ)-पदयोग-३ | विभक्ति जोडेर लेख्नुपर्छ]: भित्री जोडाइ मिलाएर विभक्ति पद जोडेर लेख्नुपर्छ",
            )
        };

        diagnostics.push(Diagnostic {
            span,
            incorrect: text[span.0..span.1].to_string(),
            correction,
            rule: Rule::VarnaVinyasNiyam("3(घ)"),
            explanation: explanation.to_string(),
            category: DiagnosticCategory::ShuddhaTable,
            kind: DiagnosticKind::Error,
            confidence: if subrule == "3(घ)-पदयोग-४" {
                0.94
            } else {
                0.93
            },
            alternate_reasons: Vec::new(),
        });
        blocked_spans.insert(span);
    }

    for triple in segments.windows(3) {
        let (left, lstart, _) = triple[0];
        let (mid, _, _) = triple[1];
        let (right, _, rend) = triple[2];
        if !is_devanagari_word(left) || !is_devanagari_word(mid) || !is_devanagari_word(right) {
            continue;
        }
        if is_numeric_segment(left) || is_numeric_segment(mid) {
            continue;
        }
        if !VIBHAKTI_TOKENS.contains(&right) {
            continue;
        }

        let inner = match normalize_joined_word(&format!("{mid}{right}")) {
            Some(inner) if starts_with_namayogi(&inner) => inner,
            _ => continue,
        };

        let Some(correction) = normalize_joined_word(&format!("{left}{inner}")) else {
            continue;
        };

        let span = (lstart, rend);
        if !is_word_boundary(text, span.0, span.1) {
            continue;
        }
        if correction == text[span.0..span.1] {
            continue;
        }
        if has_same_rewrite(diagnostics, span, &correction) {
            continue;
        }

        diagnostics.push(Diagnostic {
            span,
            incorrect: text[span.0..span.1].to_string(),
            correction,
            rule: Rule::VarnaVinyasNiyam("3(घ)"),
            explanation: "पदयोग/पदवियोग [3(घ)-पदयोग-४ | नामयोगी जोडेर लेख्नुपर्छ]: भित्री जोडाइ मिलाएर नामयोगी पद क्रमशः जोडेर लेख्नुपर्छ".to_string(),
            category: DiagnosticCategory::ShuddhaTable,
            kind: DiagnosticKind::Error,
            confidence: 0.94,
            alternate_reasons: Vec::new(),
        });
        blocked_spans.insert(span);
    }
}

fn add_generalized_padayog_vibhakti_join(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // 3(घ)-पदयोग-३: सबै विभक्तिहरू जोडेर लेख्नुपर्छ
    let segments = whitespace_segments(text);

    for pair in segments.windows(2) {
        let (left, lstart, _) = pair[0];
        let (right, _, rend) = pair[1];
        if !is_devanagari_word(left) || !is_devanagari_word(right) {
            continue;
        }
        if is_numeric_segment(left) {
            continue;
        }
        if !VIBHAKTI_TOKENS.contains(&right) {
            continue;
        }

        let Some(candidate) = normalize_joined_word(&format!("{left}{right}")) else {
            continue;
        };

        let span = (lstart, rend);
        if blocked_spans.contains(&span) || overlaps_existing_span(diagnostics, span) {
            continue;
        }
        if !is_word_boundary(text, span.0, span.1) {
            continue;
        }
        if candidate == text[span.0..span.1] {
            continue;
        }
        if has_same_rewrite(diagnostics, span, &candidate) {
            continue;
        }

        diagnostics.push(Diagnostic {
            span,
            incorrect: text[span.0..span.1].to_string(),
            correction: candidate,
            rule: Rule::VarnaVinyasNiyam("3(घ)"),
            explanation:
                "पदयोग/पदवियोग [3(घ)-पदयोग-३ | विभक्ति जोडेर लेख्नुपर्छ]: सामान्य विभक्ति पद जोडेर लेख्नुपर्छ"
                    .to_string(),
            category: DiagnosticCategory::ShuddhaTable,
            kind: DiagnosticKind::Error,
            confidence: 0.92,
            alternate_reasons: Vec::new(),
        });
        blocked_spans.insert(span);
    }
}

fn add_generalized_padayog_pratyaya_join(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let segments = whitespace_segments(text);

    for pair in segments.windows(2) {
        let (left, lstart, _) = pair[0];
        let (right, _, rend) = pair[1];
        if !is_devanagari_word(left) || !is_devanagari_word(right) {
            continue;
        }
        if is_numeric_segment(left) {
            continue;
        }
        if !PRATYAYA_TOKENS.contains(&right) {
            continue;
        }
        if !left_supports_pratyaya_join(left) {
            continue;
        }

        let correction = format!("{left}{right}");
        let span = (lstart, rend);
        if blocked_spans.contains(&span) || overlaps_existing_span(diagnostics, span) {
            continue;
        }
        if !is_word_boundary(text, span.0, span.1) {
            continue;
        }
        if correction == text[span.0..span.1] {
            continue;
        }
        if has_same_rewrite(diagnostics, span, &correction) {
            continue;
        }

        diagnostics.push(Diagnostic {
            span,
            incorrect: text[span.0..span.1].to_string(),
            correction,
            rule: Rule::VarnaVinyasNiyam("3(घ)"),
            explanation: "पदयोग/पदवियोग [3(घ)-पदयोग-२ | प्रत्यय जोडेर लेख्नुपर्छ]: मान्य आधार शब्दसँग मानार्थक प्रत्यय जोडेर लेख्नुपर्छ".to_string(),
            category: DiagnosticCategory::ShuddhaTable,
            kind: DiagnosticKind::Error,
            confidence: 0.91,
            alternate_reasons: Vec::new(),
        });
        blocked_spans.insert(span);
    }
}

fn add_generalized_padayog_namayogi_join(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let segments = whitespace_segments(text);

    for pair in segments.windows(2) {
        let (left, lstart, _) = pair[0];
        let (right, _, rend) = pair[1];
        if !is_devanagari_word(left) || !is_devanagari_word(right) {
            continue;
        }
        if is_numeric_segment(left) {
            continue;
        }
        if !NAMAYOGI_TOKENS.contains(&right) {
            continue;
        }

        let Some(correction) = normalize_joined_word(&format!("{left}{right}")) else {
            continue;
        };

        let span = (lstart, rend);
        if blocked_spans.contains(&span) || overlaps_existing_span(diagnostics, span) {
            continue;
        }
        if !is_word_boundary(text, span.0, span.1) {
            continue;
        }
        if correction == text[span.0..span.1] {
            continue;
        }
        if has_same_rewrite(diagnostics, span, &correction) {
            continue;
        }

        diagnostics.push(Diagnostic {
            span,
            incorrect: text[span.0..span.1].to_string(),
            correction,
            rule: Rule::VarnaVinyasNiyam("3(घ)"),
            explanation:
                "पदयोग/पदवियोग [3(घ)-पदयोग-४ | नामयोगी जोडेर लेख्नुपर्छ]: सामान्य नामयोगी पद जोडेर लेख्नुपर्छ"
                    .to_string(),
            category: DiagnosticCategory::ShuddhaTable,
            kind: DiagnosticKind::Error,
            confidence: 0.92,
            alternate_reasons: Vec::new(),
        });
        blocked_spans.insert(span);
    }
}

fn add_generalized_padabiyog_vibhakti_split(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // 3(घ)-पदवियोग-३: लागि, निम्ति र दुईओटा विभक्ति एकै ठाउँमा आएमा छुट्याएर
    for (seg, start, end) in whitespace_segments(text) {
        if !is_devanagari_word(seg) {
            continue;
        }

        let suggestion = if let Some(prefix) = seg.strip_suffix("लागि") {
            if prefix.chars().count() >= 2 {
                Some(format!("{prefix} लागि"))
            } else {
                None
            }
        } else if let Some(prefix) = seg.strip_suffix("निम्ति") {
            if prefix.chars().count() >= 2 {
                Some(format!("{prefix} निम्ति"))
            } else {
                None
            }
        } else if let Some(prefix) = seg.strip_suffix("कामा") {
            if prefix.chars().count() >= 1 {
                Some(format!("{prefix}का मा"))
            } else {
                None
            }
        } else {
            None
        };

        let Some(correction) = suggestion else {
            continue;
        };
        let span = (start, end);
        if blocked_spans.contains(&span) || overlaps_existing_span(diagnostics, span) {
            continue;
        }
        if !is_word_boundary(text, span.0, span.1) {
            continue;
        }
        if correction == seg {
            continue;
        }

        diagnostics.push(Diagnostic {
            span,
            incorrect: seg.to_string(),
            correction,
            rule: Rule::VarnaVinyasNiyam("3(घ)"),
            explanation: "पदयोग/पदवियोग [3(घ)-पदवियोग-३ | लागि/निम्ति/दोहोरो विभक्ति छुट्याएर]: जोडिएर आएको पद छुट्याएर लेख्नुपर्छ".to_string(),
            category: DiagnosticCategory::ShuddhaTable,
            kind: DiagnosticKind::Error,
            confidence: 0.90,
            alternate_reasons: Vec::new(),
        });
        blocked_spans.insert(span);
    }
}

fn add_generalized_padayog_conjunction_join(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // 3(घ)-पदयोग-९: केही संयोजकहरू जोडेर लेख्नुपर्छ
    const JOINS: &[(&str, &str, &str)] = &[
        ("किन", "भने", "किनभने"),
        ("ताप", "नि", "तापनि"),
        ("यद्य", "पि", "यद्यपि"),
        ("तथा", "पि", "तथापि"),
    ];
    let segments = whitespace_segments(text);
    for pair in segments.windows(2) {
        let (left, lstart, _) = pair[0];
        let (right, _, rend) = pair[1];
        for &(a, b, joined) in JOINS {
            if left != a || right != b {
                continue;
            }
            let span = (lstart, rend);
            if blocked_spans.contains(&span) || overlaps_existing_span(diagnostics, span) {
                continue;
            }
            if !is_word_boundary(text, span.0, span.1) {
                continue;
            }
            diagnostics.push(Diagnostic {
                span,
                incorrect: text[span.0..span.1].to_string(),
                correction: joined.to_string(),
                rule: Rule::VarnaVinyasNiyam("3(घ)"),
                explanation:
                    "पदयोग/पदवियोग [3(घ)-पदयोग-९ | संयोजक जोडेर लेख्नुपर्छ]: संयोजक पद जोडेर लेख्नुपर्छ"
                        .to_string(),
                category: DiagnosticCategory::ShuddhaTable,
                kind: DiagnosticKind::Error,
                confidence: 0.94,
                alternate_reasons: Vec::new(),
            });
            blocked_spans.insert(span);
        }
    }
}

fn add_generalized_padabiyog_verb_complex_split(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
    include_aspect: bool,
    include_ne_future: bool,
    include_nu_modal: bool,
) {
    // 3(घ)-पदवियोग-६/७/९:
    // - अपूर्ण/पूर्ण पक्ष जनाउने क्रियापद छुट्याएर
    // - 'ने छ' भविष्यत् कालबोधक क्रियापद छुट्याएर
    // - 'नु'/'न' पछि आउने क्रियापद छुट्याएर
    const AUX_SUFFIXES: &[&str] = &[
        "छन्",
        "छौ",
        "छु",
        "छ",
        "थिए",
        "थियो",
        "थिई",
        "थिइन्",
        "थिइ",
        "थें",
    ];
    const MODAL_SUFFIXES: &[&str] = &["सक्छन्", "सक्छु", "सक्छौ", "सक्छ", "सक्दैन"];

    for (seg, start, end) in whitespace_segments(text) {
        if !is_devanagari_word(seg) {
            continue;
        }

        let mut candidate: Option<(String, &'static str)> = None;

        for &aux in AUX_SUFFIXES {
            if let Some(prefix) = seg.strip_suffix(aux) {
                if prefix.chars().count() < 2 {
                    continue;
                }
                let is_aspect = prefix.ends_with("दै")
                    || prefix.ends_with("दो")
                    || prefix.ends_with("को")
                    || prefix.ends_with("का")
                    || prefix.ends_with("की");
                let is_ne_future = prefix.ends_with("ने");
                if include_aspect && is_aspect {
                    candidate = Some((
                        format!("{prefix} {aux}"),
                        "3(घ)-पदवियोग-६ | पक्षसूचक क्रियापद छुट्याएर",
                    ));
                    break;
                }
                if include_ne_future && is_ne_future {
                    candidate = Some((format!("{prefix} {aux}"), "3(घ)-पदवियोग-७ | 'ने छ' छुट्याएर"));
                    break;
                }
            }
        }

        if include_nu_modal && candidate.is_none() {
            for &modal in MODAL_SUFFIXES {
                if let Some(prefix) = seg.strip_suffix(modal) {
                    if prefix.ends_with("नु") || prefix.ends_with('न') {
                        candidate = Some((
                            format!("{prefix} {modal}"),
                            "3(घ)-पदवियोग-९ | 'नु'/'न' पछि क्रियापद छुट्याएर",
                        ));
                        break;
                    }
                }
            }
        }

        let Some((correction, subrule)) = candidate else {
            continue;
        };
        if correction == seg {
            continue;
        }
        let span = (start, end);
        if blocked_spans.contains(&span) || overlaps_existing_span(diagnostics, span) {
            continue;
        }
        if !is_word_boundary(text, span.0, span.1) {
            continue;
        }

        diagnostics.push(Diagnostic {
            span,
            incorrect: seg.to_string(),
            correction,
            rule: Rule::VarnaVinyasNiyam("3(घ)"),
            explanation: format!("पदयोग/पदवियोग [{subrule}]: क्रियापद पद छुट्याएर लेख्नुपर्छ"),
            category: DiagnosticCategory::ShuddhaTable,
            kind: DiagnosticKind::Error,
            confidence: 0.90,
            alternate_reasons: Vec::new(),
        });
        blocked_spans.insert(span);
    }
}
