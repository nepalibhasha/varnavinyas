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
