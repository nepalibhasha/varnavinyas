use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::step::Step;
use varnavinyas_kosha::kosha;
use varnavinyas_shabda::{Origin, classify};

pub const SPEC_HALANTA: RuleSpec = RuleSpec {
    id: "ortho-halanta",
    category: RuleCategory::Halanta,
    kind: DiagnosticKind::Error,
    priority: 330,
    citation: Rule::VarnaVinyasNiyam("3(ङ)"),
    examples: &[("बुद्धिमान", "बुद्धिमान्"), ("श्रीमान", "श्रीमान्")],
};

pub fn rule_halanta(input: &str) -> Option<Prakriya> {
    let lex = kosha();
    if let Some(stem) = input.strip_suffix("छ्") {
        let output = format!("{stem}छ");
        if lex.contains(&output) && !lex.contains(input) {
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(ङ)-अजन्त-5"),
                    "समापक क्रियापदको अन्त्यमा हलन्त लेखिँदैन (…छ)",
                    input,
                    &output,
                )],
            ));
        }
    }

    const VERB_SUFFIXES: &[(&str, &str, &str)] = &[
        ("छस", "छस्", "3(ङ)-2"),
        ("छन", "छन्", "3(ङ)-3"),
        ("इस", "इस्", "3(ङ)-2"),
    ];
    for (wrong_suffix, correct_suffix, rule_citation) in VERB_SUFFIXES {
        if let Some(stem) = input.strip_suffix(wrong_suffix) {
            let output = format!("{}{}", stem, correct_suffix);
            if lex.contains(&output) {
                return Some(Prakriya::corrected(
                    input,
                    &output,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam(rule_citation),
                        format!("क्रियापदमा हलन्त: {} -> {}", wrong_suffix, correct_suffix),
                        input,
                        &output,
                    )],
                ));
            }
        }
    }

    let origin = classify(input);
    if !matches!(origin, Origin::Tatsam) {
        return None;
    }
    let suffixes = [
        ("मान", "मान्", "3(ङ)-मान्"),
        ("वान", "वान्", "3(ङ)-वान्"),
        ("वत", "वत्", "3(ङ)-वत्"),
    ];
    for (wrong_suffix, correct_suffix, rule_citation) in suffixes {
        if let Some(stem) = input.strip_suffix(wrong_suffix) {
            let output = format!("{}{}", stem, correct_suffix);
            if lex.contains(input) && !lex.contains(&output) {
                return None;
            }
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam(rule_citation),
                    format!(
                        "तत्सम प्रत्ययमा हलन्त हुन्छ: {} -> {}",
                        wrong_suffix, correct_suffix
                    ),
                    input,
                    &output,
                )],
            ));
        }
    }
    None
}
