use crate::model::prakriya::Prakriya;
use crate::model::rule::Rule;
use crate::model::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::model::step::Step;

// -----------------------------------------------------------------------------
// 3(क)(ई) शब्दका सुरुमा दीर्घ ईकार/ऊकारको प्रयोग
// Rule-book map:
// - 1  TODO: संस्कृतबाट जस्ताको तस्तै आएका दीर्घ-आदि शब्द
// - 2  TODO: 'सु' उपसर्ग लागेका उकारादि शब्द
// -----------------------------------------------------------------------------
// 3(क)(उ) शब्दका बिचमा दीर्घ ईकार/ऊकारको प्रयोग
// Rule-book map:
// - 1  implemented in `rule_suffix_preserves_dirgha`
// - 2  implemented in `rule_suffix_family_preserves_dirgha`
// -----------------------------------------------------------------------------
pub const SPEC_SUFFIX_PRESERVES: RuleSpec = RuleSpec {
    id: "hd-suffix-preserves",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 220,
    citation: Rule::VarnaVinyasNiyam("3(क)(उ)-1"),
    examples: &[("पुर्वी", "पूर्वी"), ("पुर्वीय", "पूर्वीय")],
};

pub const SPEC_SUFFIX_FAMILY_PRESERVES_DIRGHA: RuleSpec = RuleSpec {
    id: "hd-suffix-family-preserves-dirgha",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 221,
    citation: Rule::VarnaVinyasNiyam("3(क)(उ)-2"),
    examples: &[("एकिकरण", "एकीकरण"), ("एकिकृत", "एकीकृत")],
};

pub fn rule_suffix_preserves_dirgha(input: &str) -> Option<Prakriya> {
    static KNOWN_CORRECTIONS: &[(&str, &str, &str)] = &[
        ("पुर्वी", "पूर्वी", "प्रत्यय -ई ले दीर्घ: पूर्व + ई = पूर्वी"),
        ("पुर्वीय", "पूर्वीय", "प्रत्यय -ईय ले दीर्घ: पूर्व + ईय = पूर्वीय"),
    ];

    for &(wrong, correct, desc) in KNOWN_CORRECTIONS {
        if input == wrong {
            return Some(Prakriya::corrected(
                input,
                correct,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(क)(उ)-1"),
                    desc,
                    input,
                    correct,
                )],
            ));
        }
    }

    None
}

pub fn rule_suffix_family_preserves_dirgha(input: &str) -> Option<Prakriya> {
    const SUFFIX_PATTERNS: &[(&str, &str)] = &[
        ("िकरण", "ीकरण"),
        ("िकृत", "ीकृत"),
        ("िकार", "ीकार"),
        ("िभवन", "ीभवन"),
        ("िभूत", "ीभूत"),
        ("िभाव", "ीभाव"),
    ];

    let lex = varnavinyas_kosha::kosha();
    if lex.contains(input) {
        return None;
    }

    for &(wrong, correct) in SUFFIX_PATTERNS {
        if !input.ends_with(wrong) {
            continue;
        }
        let output = input.replacen(wrong, correct, 1);
        if !lex.contains(&output) {
            continue;
        }

        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(क)(उ)-2"),
                "करण, कृत, कार, भवन, भूत, भावसँग जोडिएका शब्दमा बिचमा दीर्घ हुन्छ",
                input,
                &output,
            )],
        ));
    }

    None
}
