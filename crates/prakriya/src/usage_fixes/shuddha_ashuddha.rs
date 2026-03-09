use crate::model::prakriya::Prakriya;
use crate::model::rule::Rule;
use crate::model::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::model::step::Step;

pub const SPEC_SHRI: RuleSpec = RuleSpec {
    id: "struct-shri",
    category: RuleCategory::Structural,
    kind: DiagnosticKind::Error,
    priority: 100,
    citation: Rule::ShuddhaAshuddha("Section 4"),
    examples: &[("श्रृङ्गार", "शृङ्गार")],
};

pub const SPEC_REDUNDANT_SUFFIX: RuleSpec = RuleSpec {
    id: "struct-redundant-suffix",
    category: RuleCategory::Structural,
    kind: DiagnosticKind::Error,
    priority: 110,
    citation: Rule::ShuddhaAshuddha("Section 4"),
    examples: &[("सौन्दर्यता", "सौन्दर्य"), ("औचित्यता", "औचित्य")],
};

pub fn rule_shri_correction(input: &str) -> Option<Prakriya> {
    if input.contains("श्रृ") {
        let output = input.replace("श्रृ", "शृ");
        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::ShuddhaAshuddha("Section 4"),
                "शृ not श्रृ: श + ृ = शृ (no र involved)",
                input,
                &output,
            )],
        ));
    }
    None
}

pub fn rule_redundant_suffix(input: &str) -> Option<Prakriya> {
    if input.chars().count() < 6 {
        return None;
    }
    if input.ends_with("र्यता") || input.ends_with("त्यता") || input.ends_with("थ्यता")
    {
        let output = input.strip_suffix("ता").unwrap();
        return Some(Prakriya::corrected(
            input,
            output,
            vec![Step::new(
                Rule::ShuddhaAshuddha("Section 4"),
                "-ता अनावश्यक: abstract noun already complete",
                input,
                output,
            )],
        ));
    }
    None
}
