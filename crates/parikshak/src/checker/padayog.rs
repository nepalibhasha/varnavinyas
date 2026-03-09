use std::collections::HashSet;

use varnavinyas_kosha::kosha;
use varnavinyas_prakriya::{DiagnosticKind, Rule};

use crate::diagnostic::{Diagnostic, DiagnosticCategory};
use crate::padayog_padabiyog::PADAYOG_PADABIYOG_RULES;

use super::common::{
    is_devanagari_word, is_word_boundary, overlaps_existing_span, whitespace_segments,
};

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
                        "पदयोग/पदवियोग [{} | {} | {:?}]: {}",
                        rule.code, rule.label, rule.action, rw.explanation
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
    add_generalized_padayog_vibhakti_join(text, blocked_spans, diagnostics);
    add_generalized_padayog_conjunction_join(text, blocked_spans, diagnostics);
    add_generalized_padabiyog_vibhakti_split(text, blocked_spans, diagnostics);
    add_generalized_padabiyog_verb_complex_split(text, blocked_spans, diagnostics);
}

fn add_generalized_padayog_vibhakti_join(
    text: &str,
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // 3(घ)-पदयोग-३: सबै विभक्तिहरू जोडेर लेख्नुपर्छ
    const VIBHAKTI_TOKENS: &[&str] = &["ले", "लाई", "को", "बाट", "देखि", "मा", "का", "की"];
    let lex = kosha();
    let segments = whitespace_segments(text);

    for pair in segments.windows(2) {
        let (left, lstart, _) = pair[0];
        let (right, _, rend) = pair[1];
        if !is_devanagari_word(left) || !is_devanagari_word(right) {
            continue;
        }
        if !VIBHAKTI_TOKENS.contains(&right) {
            continue;
        }

        let candidate = format!("{left}{right}");
        if !lex.contains(&candidate) {
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
            correction: candidate,
            rule: Rule::VarnaVinyasNiyam("3(घ)"),
            explanation: "पदयोग/पदवियोग [3(घ)-पदयोग-३ | विभक्ति जोडेर लेख्नुपर्छ | Join]: सामान्य विभक्ति पद जोडेर लेख्नुपर्छ".to_string(),
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
            explanation: "पदयोग/पदवियोग [3(घ)-पदवियोग-३ | लागि/निम्ति/दोहोरो विभक्ति छुट्याएर | Split]: जोडिएर आएको पद छुट्याएर लेख्नुपर्छ".to_string(),
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
                    "पदयोग/पदवियोग [3(घ)-पदयोग-९ | संयोजक जोडेर लेख्नुपर्छ | Join]: संयोजक पद जोडेर लेख्नुपर्छ"
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
                if is_aspect {
                    candidate = Some((
                        format!("{prefix} {aux}"),
                        "3(घ)-पदवियोग-६ | पक्षसूचक क्रियापद छुट्याएर",
                    ));
                    break;
                }
                if is_ne_future {
                    candidate = Some((format!("{prefix} {aux}"), "3(घ)-पदवियोग-७ | 'ने छ' छुट्याएर"));
                    break;
                }
            }
        }

        if candidate.is_none() {
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
            explanation: format!("पदयोग/पदवियोग [{subrule} | Split]: क्रियापद पद छुट्याएर लेख्नुपर्छ"),
            category: DiagnosticCategory::ShuddhaTable,
            kind: DiagnosticKind::Error,
            confidence: 0.90,
            alternate_reasons: Vec::new(),
        });
        blocked_spans.insert(span);
    }
}
