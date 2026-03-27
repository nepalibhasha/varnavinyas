use super::helpers::{
    chandrabindu_subrule_for, is_stop_consonant, nasalize_diphthong_suffix,
    should_replace_shirbindu,
};
use crate::model::prakriya::Prakriya;
use crate::model::rule::Rule;
use crate::model::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::model::step::Step;
use varnavinyas_kosha::kosha;
use varnavinyas_shabda::{Origin, classify_with_provenance};

pub const SPEC_CHANDRABINDU: RuleSpec = RuleSpec {
    id: "ortho-chandrabindu",
    category: RuleCategory::Chandrabindu,
    kind: DiagnosticKind::Error,
    priority: 300,
    citation: Rule::VarnaVinyasNiyam("3(ख)"),
    examples: &[("सिँह", "सिंह")],
};

// -----------------------------------------------------------------------------
// 3(ख)(आ) चन्द्रविन्दुको प्रयोग
// Implemented here:
// - 3(ख)(आ)-1 tatsam: no chandrabindu
// - 3(ख)(आ)-2 first-person nasalized verb forms
// - 3(ख)(आ)-3 ...दा/...दै forms
// - 3(ख)(आ)-4 ...छ/...थ forms after dvisvara stems
// -----------------------------------------------------------------------------
pub fn rule_chandrabindu(input: &str) -> Option<Prakriya> {
    if let Some(output) = supported_non_tatsam_chandrabindu_form(input) {
        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(ख)(आ)-lex"),
                "तद्भव/अव्यय मानक रूपमा चन्द्रबिन्दु (ँ) प्रयोग हुन्छ",
                input,
                &output,
            )],
        ));
    }

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
                        Rule::VarnaVinyasNiyam("3(ख)(आ)-1"),
                        "तत्सम शब्दमा चन्द्रबिन्दु हुँदैन; शिरबिन्दु (ं) प्रयोग हुन्छ",
                        input,
                        &output,
                    )],
                ));
            }
        }
        Origin::Tadbhav | Origin::Deshaj => {
            if let Some((output, subrule)) = nasalize_diphthong_suffix(input) {
                return Some(Prakriya::corrected(
                    input,
                    &output,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam(subrule),
                        "द्विस्वरान्त धातुमा अनुनासिक चन्द्रबिन्दु प्रयोग हुन्छ",
                        input,
                        &output,
                    )],
                ));
            }
            if input.contains('ँ') {
                let output = input.replace('ँ', "ं");
                if kosha().contains(&output)
                    && matches!(classify_with_provenance(&output).origin, Origin::Tatsam)
                {
                    return Some(Prakriya::corrected(
                        input,
                        &output,
                        vec![Step::new(
                            Rule::VarnaVinyasNiyam("3(ख)(आ)-1"),
                            "तत्सम रूपमा शिरबिन्दु (ं) प्रयोग हुन्छ",
                            input,
                            &output,
                        )],
                    ));
                }
            }
            if input.contains('ं') {
                let chars: Vec<char> = input.chars().collect();
                let mut output_chars = chars.clone();
                let mut changed = false;

                for i in 0..chars.len() {
                    if chars[i] == 'ं' {
                        let next = chars.get(i + 1).copied();
                        let mut candidate_chars = chars.clone();
                        candidate_chars[i] = 'ँ';
                        let candidate: String = candidate_chars.into_iter().collect();
                        let subrule = chandrabindu_subrule_for(&candidate);
                        let force = subrule != "3(ख)(आ)-1";
                        let before_stop = next.is_some_and(is_stop_consonant);
                        if before_stop && !force {
                            continue;
                        }
                        if force || should_replace_shirbindu(input, &chars, i, source) {
                            output_chars[i] = 'ँ';
                            changed = true;
                        }
                    }
                }

                if changed {
                    let output: String = output_chars.into_iter().collect();
                    let subrule = chandrabindu_subrule_for(&output);
                    return Some(Prakriya::corrected(
                        input,
                        &output,
                        vec![Step::new(
                            Rule::VarnaVinyasNiyam(subrule),
                            "तद्भव/देशज शब्दमा चन्द्रबिन्दु (ँ) प्रयोग हुन्छ, शिरबिन्दु (ं) होइन",
                            input,
                            &output,
                        )],
                    ));
                }
            }
        }
        Origin::Aagantuk => {
            if let Some((output, subrule)) = nasalize_diphthong_suffix(input) {
                return Some(Prakriya::corrected(
                    input,
                    &output,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam(subrule),
                        "द्विस्वरान्त धातुमा अनुनासिक चन्द्रबिन्दु प्रयोग हुन्छ",
                        input,
                        &output,
                    )],
                ));
            }
            if input.contains('ँ') {
                let output = input.replace('ँ', "ं");
                if kosha().contains(&output)
                    && matches!(classify_with_provenance(&output).origin, Origin::Tatsam)
                {
                    return Some(Prakriya::corrected(
                        input,
                        &output,
                        vec![Step::new(
                            Rule::VarnaVinyasNiyam("3(ख)(आ)-1"),
                            "तत्सम रूपमा शिरबिन्दु (ं) प्रयोग हुन्छ",
                            input,
                            &output,
                        )],
                    ));
                }
            }
            if input.contains('ं') {
                let chars: Vec<char> = input.chars().collect();
                let mut output_chars = chars.clone();
                let mut changed = false;

                for i in 0..chars.len() {
                    if chars[i] == 'ं' {
                        let next = chars.get(i + 1).copied();
                        let mut candidate_chars = chars.clone();
                        candidate_chars[i] = 'ँ';
                        let candidate: String = candidate_chars.into_iter().collect();
                        let subrule = chandrabindu_subrule_for(&candidate);
                        let force = subrule != "3(ख)(आ)-1";
                        let before_stop = next.is_some_and(is_stop_consonant);
                        if before_stop && !force {
                            continue;
                        }
                        if force || should_replace_shirbindu(input, &chars, i, source) {
                            output_chars[i] = 'ँ';
                            changed = true;
                        }
                    }
                }

                if changed {
                    let output: String = output_chars.into_iter().collect();
                    let subrule = chandrabindu_subrule_for(&output);
                    return Some(Prakriya::corrected(
                        input,
                        &output,
                        vec![Step::new(
                            Rule::VarnaVinyasNiyam(subrule),
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

fn supported_non_tatsam_chandrabindu_form(input: &str) -> Option<String> {
    if !input.contains('ं') {
        return None;
    }

    let lex = kosha();
    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        if chars[i] != 'ं' {
            continue;
        }

        let mut candidate_chars = chars.clone();
        candidate_chars[i] = 'ँ';
        let candidate: String = candidate_chars.into_iter().collect();
        if chandrabindu_subrule_for(&candidate) != "3(ख)(आ)-1" {
            continue;
        }
        let decision = classify_with_provenance(&candidate);

        if matches!(decision.origin, Origin::Tatsam) {
            continue;
        }

        let Some(entry) = lex.lookup(&candidate) else {
            continue;
        };
        let pos = entry.pos;
        if pos.contains("अव्य")
            || pos.contains("क्रि.वि.")
            || pos.contains("क्रियाविशेषण")
            || pos.contains("नामयोगी")
            || pos.contains("ना.यो.")
        {
            return Some(candidate);
        }
    }

    None
}
