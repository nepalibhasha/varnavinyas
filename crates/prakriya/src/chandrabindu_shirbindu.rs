use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::step::Step;
use varnavinyas_akshar::is_vyanjan;
use varnavinyas_kosha::kosha;
use varnavinyas_shabda::{Origin, OriginSource, classify_with_provenance};

pub const SPEC_CHANDRABINDU: RuleSpec = RuleSpec {
    id: "ortho-chandrabindu",
    category: RuleCategory::Chandrabindu,
    kind: DiagnosticKind::Error,
    priority: 300,
    citation: Rule::VarnaVinyasNiyam("3(ख)"),
    examples: &[("सिँह", "सिंह")],
};

/// Academy 3(ख): शब्दउत्पत्तिअनुसार चन्द्रबिन्दु/शिरबिन्दु प्रयोग।
/// - तत्सम: चन्द्रबिन्दु (ँ) होइन, शिरबिन्दु (ं)।
/// - तद्भव/आगन्तुक: अनुनासिकमा शिरबिन्दु (ं) होइन, चन्द्रबिन्दु (ँ)।
pub fn rule_chandrabindu(input: &str) -> Option<Prakriya> {
    let origin_decision = classify_with_provenance(input);
    let origin = origin_decision.origin;
    let source = origin_decision.source;

    match origin {
        Origin::Tatsam => {
            if input.contains('ँ') {
                let output = input.replace('ँ', "ं");
                return Some(Prakriya::corrected(
                    input,
                    &output,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(ख)"),
                        "तत्सम शब्दमा शिरबिन्दु (ं) प्रयोग हुन्छ, चन्द्रबिन्दु (ँ) होइन",
                        input,
                        &output,
                    )],
                ));
            }
        }
        Origin::Tadbhav | Origin::Deshaj => {
            if input.contains('ं') {
                let chars: Vec<char> = input.chars().collect();
                let mut output_chars = chars.clone();
                let mut changed = false;

                for i in 0..chars.len() {
                    if chars[i] == 'ं' {
                        let next = chars.get(i + 1).copied();
                        let before_stop = next.is_some_and(is_stop_consonant);
                        if !before_stop && should_replace_shirbindu(input, &chars, i, source) {
                            output_chars[i] = 'ँ';
                            changed = true;
                        }
                    }
                }

                if changed {
                    let output: String = output_chars.into_iter().collect();
                    return Some(Prakriya::corrected(
                        input,
                        &output,
                        vec![Step::new(
                            Rule::VarnaVinyasNiyam("3(ख)"),
                            "तद्भव/देशज शब्दमा चन्द्रबिन्दु (ँ) प्रयोग हुन्छ, शिरबिन्दु (ं) होइन",
                            input,
                            &output,
                        )],
                    ));
                }
            }
        }
        Origin::Aagantuk => {
            if input.contains('ं') {
                let chars: Vec<char> = input.chars().collect();
                let mut output_chars = chars.clone();
                let mut changed = false;

                for i in 0..chars.len() {
                    if chars[i] == 'ं' {
                        let next = chars.get(i + 1).copied();
                        let before_stop = next.is_some_and(is_stop_consonant);
                        if !before_stop && should_replace_shirbindu(input, &chars, i, source) {
                            output_chars[i] = 'ँ';
                            changed = true;
                        }
                    }
                }

                if changed {
                    let output: String = output_chars.into_iter().collect();
                    return Some(Prakriya::corrected(
                        input,
                        &output,
                        vec![Step::new(
                            Rule::VarnaVinyasNiyam("3(ख)"),
                            "आगन्तुक शब्दमा अनुनासिकमा चन्द्रबिन्दु (ँ) प्रयोग हुन्छ",
                            input,
                            &output,
                        )],
                    ));
                }
            }
        }
    }

    None
}

fn is_stop_consonant(c: char) -> bool {
    is_vyanjan(c)
        && matches!(
            c,
            'क' | 'ख'
                | 'ग'
                | 'घ'
                | 'ङ'
                | 'च'
                | 'छ'
                | 'ज'
                | 'झ'
                | 'ञ'
                | 'ट'
                | 'ठ'
                | 'ड'
                | 'ढ'
                | 'ण'
                | 'त'
                | 'थ'
                | 'द'
                | 'ध'
                | 'न'
                | 'प'
                | 'फ'
                | 'ब'
                | 'भ'
                | 'म'
        )
}

fn should_replace_shirbindu(
    input: &str,
    chars: &[char],
    idx: usize,
    _origin_source: OriginSource,
) -> bool {
    if idx + 1 == chars.len() && idx > 0 && matches!(chars[idx - 1], 'े' | 'ौ') {
        return true;
    }

    if kosha().contains(input) {
        return false;
    }

    let mut candidate_chars = chars.to_vec();
    candidate_chars[idx] = 'ँ';
    let candidate: String = candidate_chars.into_iter().collect();
    kosha().contains(&candidate)
}
