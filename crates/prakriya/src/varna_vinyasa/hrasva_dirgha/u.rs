use super::helpers::hrasva_helpers;
use crate::model::prakriya::Prakriya;
use crate::model::rule::Rule;
use crate::model::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::model::step::Step;
use varnavinyas_shabda::{Origin, classify, decompose};

// -----------------------------------------------------------------------------
// 3(क)(ई) शब्दका सुरुमा दीर्घ ईकार/ऊकारको प्रयोग
// Rule-book map:
// - 1  implemented conservatively in `rule_initial_tatsam_dirgha`
// - 2  implemented in `rule_su_prefix_preserves_dirgha`
// -----------------------------------------------------------------------------
// 3(क)(उ) शब्दका बिचमा दीर्घ ईकार/ऊकारको प्रयोग
// Rule-book map:
// - 1  implemented in `rule_suffix_preserves_dirgha`
// - 2  implemented in `rule_suffix_family_preserves_dirgha`
// -----------------------------------------------------------------------------
pub const SPEC_INITIAL_TATSAM_DIRGHA: RuleSpec = RuleSpec {
    id: "hd-initial-tatsam-dirgha",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 218,
    citation: Rule::VarnaVinyasNiyam("3(क)(ई)-1"),
    examples: &[("इश्वर", "ईश्वर"), ("भुमि", "भूमि")],
};

pub const SPEC_SU_PREFIX_PRESERVES_DIRGHA: RuleSpec = RuleSpec {
    id: "hd-su-prefix-preserves-dirgha",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 219,
    citation: Rule::VarnaVinyasNiyam("3(क)(ई)-2"),
    examples: &[("सुक्ति", "सूक्ति"), ("सुक्त", "सूक्त")],
};

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

pub fn rule_initial_tatsam_dirgha(input: &str) -> Option<Prakriya> {
    if rule_su_prefix_preserves_dirgha(input).is_some() {
        return None;
    }

    let output = hrasva_helpers::initial_hrasva_to_dirgha(input)?;
    let kosha = varnavinyas_kosha::kosha();

    if kosha.contains(input) || !kosha.contains(&output) {
        return None;
    }

    if !matches!(classify(&output), Origin::Tatsam) {
        return None;
    }

    let morphology = decompose(&output);
    if !morphology.prefixes.is_empty() || !morphology.suffixes.is_empty() {
        return None;
    }

    Some(Prakriya::corrected(
        input,
        &output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam("3(क)(ई)-1"),
            "संस्कृतबाट नेपालीमा जस्ताको तस्तै आएका शब्दका सुरुमा दीर्घ हुन्छ",
            input,
            &output,
        )],
    ))
}

pub fn rule_su_prefix_preserves_dirgha(input: &str) -> Option<Prakriya> {
    let lex = varnavinyas_kosha::kosha();
    if !input.starts_with("सु") {
        return None;
    }

    let output = input.replacen("सु", "सू", 1);
    let base_tail = output.strip_prefix("सू")?;
    let base = format!("उ{base_tail}");

    if !lex.contains(&output) || !lex.contains(&base) {
        return None;
    }

    Some(Prakriya::corrected(
        input,
        &output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam("3(क)(ई)-2"),
            "उकारादि शब्दमा 'सु' उपसर्ग लागेर बनेका शब्दमा सुरुमा दीर्घ हुन्छ",
            input,
            &output,
        )],
    ))
}

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
