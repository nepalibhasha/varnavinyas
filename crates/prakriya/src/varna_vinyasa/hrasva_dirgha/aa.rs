use super::helpers::hrasva_helpers;
use crate::model::prakriya::Prakriya;
use crate::model::rule::Rule;
use crate::model::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::model::step::Step;
use varnavinyas_shabda::{Origin, classify, decompose};

// -----------------------------------------------------------------------------
// 3(क)(आ) शब्दका बिचमा ह्रस्व इकार/उकारको प्रयोग
// Rule-book map:
// - 1  implemented in `rule_medial_prefix_hrasva`
// - 2  partial in `rule_medial_suffix_hrasva`
// - 3  implemented conservatively in `rule_medial_derived_name_hrasva`
// - 4  implemented conservatively in `rule_medial_underived_name_hrasva`
// - 5  implemented in `rule_medial_aagantuk_name_hrasva`
// - 6  implemented in `rule_medial_adjective_hrasva`
// - 7  TODO: क्रियापद
// - 8  TODO: कर्म वा भाव वाच्यका क्रियापद
// - 9  implemented in `rule_medial_avyaya_hrasva`
// - 10 implemented in `rule_medial_onomatopoeic_hrasva`
// -----------------------------------------------------------------------------
pub const SPEC_MEDIAL_PREFIX_HRASVA: RuleSpec = RuleSpec {
    id: "hd-medial-prefix-hrasva",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 228,
    citation: Rule::VarnaVinyasNiyam("3(क)(आ)-1"),
    examples: &[("अनूभव", "अनुभव"), ("अभीमान", "अभिमान")],
};

pub const SPEC_MEDIAL_SUFFIX_HRASVA: RuleSpec = RuleSpec {
    id: "hd-medial-suffix-hrasva",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 229,
    citation: Rule::VarnaVinyasNiyam("3(क)(आ)-2"),
    examples: &[("भौतीक", "भौतिक"), ("गरीमा", "गरिमा")],
};

pub const SPEC_MEDIAL_DERIVED_NAME_HRASVA: RuleSpec = RuleSpec {
    id: "hd-medial-derived-name-hrasva",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 231,
    citation: Rule::VarnaVinyasNiyam("3(क)(आ)-3"),
    examples: &[("बिसाऊनी", "बिसाउनी"), ("भरीया", "भरिया")],
};

pub const SPEC_MEDIAL_UNDERIVED_NAME_HRASVA: RuleSpec = RuleSpec {
    id: "hd-medial-underived-name-hrasva",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 232,
    citation: Rule::VarnaVinyasNiyam("3(क)(आ)-4"),
    examples: &[("कुकूर", "कुकुर"), ("पटूका", "पटुका")],
};

pub const SPEC_MEDIAL_AAGANTUK_NAME_HRASVA: RuleSpec = RuleSpec {
    id: "hd-medial-aagantuk-name-hrasva",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 235,
    citation: Rule::VarnaVinyasNiyam("3(क)(आ)-5"),
    examples: &[("कमीटी", "कमिटी"), ("कानून", "कानुन")],
};

pub const SPEC_MEDIAL_ADJECTIVE_HRASVA: RuleSpec = RuleSpec {
    id: "hd-medial-adjective-hrasva",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 236,
    citation: Rule::VarnaVinyasNiyam("3(क)(आ)-6"),
    examples: &[("पोसीलो", "पोसिलो"), ("कलीलो", "कलिलो")],
};

pub const SPEC_MEDIAL_AVYAYA_HRASVA: RuleSpec = RuleSpec {
    id: "hd-medial-avyaya-hrasva",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 237,
    citation: Rule::VarnaVinyasNiyam("3(क)(आ)-9"),
    examples: &[("अहीले", "अहिले"), ("बाहीर", "बाहिर")],
};

pub const SPEC_MEDIAL_ONOMATOPOEIC_HRASVA: RuleSpec = RuleSpec {
    id: "hd-medial-onomatopoeic-hrasva",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 238,
    citation: Rule::VarnaVinyasNiyam("3(क)(आ)-10"),
    examples: &[("सूटुक्क", "सुटुक्क"), ("टीलिक्क", "टिलिक्क")],
};

pub fn rule_medial_prefix_hrasva(input: &str) -> Option<Prakriya> {
    const PREFIX_PATTERNS: &[(&str, &str, &str)] = &[
        ("अनू", "अनु", "अनु"),
        ("अभी", "अभि", "अभि"),
        ("अती", "अति", "अति"),
        ("अधी", "अधि", "अधि"),
        ("प्रती", "प्रति", "प्रति"),
        ("परी", "परि", "परि"),
    ];

    let lex = varnavinyas_kosha::kosha();
    if lex.contains(input) {
        return None;
    }

    for &(wrong, correct, canonical_prefix) in PREFIX_PATTERNS {
        if !input.starts_with(wrong) {
            continue;
        }
        let output = input.replacen(wrong, correct, 1);
        if !lex.contains(&output) {
            continue;
        }

        let morphology = decompose(&output);
        if !morphology.prefixes.iter().any(|p| p == canonical_prefix) {
            continue;
        }

        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(क)(आ)-1"),
                "अनु, अभि, अति, अधि, प्रति, परि उपसर्ग लागेका शब्दमा बिचमा ह्रस्व हुन्छ",
                input,
                &output,
            )],
        ));
    }

    None
}

pub fn rule_medial_suffix_hrasva(input: &str) -> Option<Prakriya> {
    const SUFFIX_PATTERNS: &[(&str, &str)] = &[
        ("ीका", "िका"),
        ("ीमा", "िमा"),
        ("ीष्ठ", "िष्ठ"),
        ("ीक", "िक"),
        ("ीत", "ित"),
        ("ीम", "िम"),
        ("ूक", "ुक"),
        ("ईक", "इक"),
        ("ईका", "इका"),
        ("ईत", "इत"),
        ("ईम", "इम"),
        ("ईमा", "इमा"),
        ("ईष्ठ", "इष्ठ"),
        ("ऊक", "उक"),
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
        if !lex.contains(&output) || !hrasva_helpers::has_medial_hrasva_suffix_family(&output) {
            continue;
        }

        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(क)(आ)-2"),
                "इक, इका, इत, इम, इमा, इष्ठ, उक प्रत्यय लागेका शब्दमा बिचमा ह्रस्व हुन्छ",
                input,
                &output,
            )],
        ));
    }

    None
}

pub fn rule_medial_derived_name_hrasva(input: &str) -> Option<Prakriya> {
    let lex = varnavinyas_kosha::kosha();
    if lex.contains(input) {
        return None;
    }

    for output in hrasva_helpers::medial_dirgha_to_hrasva_candidates(input) {
        if !lex.contains(&output) || !hrasva_helpers::is_medial_derived_name(&output) {
            continue;
        }
        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(क)(आ)-3"),
                "व्युत्पन्न नामहरू सबै बिचमा ह्रस्व हुन्छन्",
                input,
                &output,
            )],
        ));
    }

    None
}

pub fn rule_medial_underived_name_hrasva(input: &str) -> Option<Prakriya> {
    let lex = varnavinyas_kosha::kosha();
    if lex.contains(input) {
        return None;
    }

    for output in hrasva_helpers::medial_dirgha_to_hrasva_candidates(input) {
        if !lex.contains(&output) || !hrasva_helpers::is_medial_underived_name(&output) {
            continue;
        }
        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(क)(आ)-4"),
                "अव्युत्पन्न नामहरू सबै बिचमा ह्रस्व हुन्छन्",
                input,
                &output,
            )],
        ));
    }

    None
}

pub fn rule_medial_aagantuk_name_hrasva(input: &str) -> Option<Prakriya> {
    let lex = varnavinyas_kosha::kosha();
    for output in hrasva_helpers::medial_dirgha_to_hrasva_candidates(input) {
        if !lex.contains(&output) || !matches!(classify(&output), Origin::Aagantuk) {
            continue;
        }
        if !hrasva_helpers::is_name_pos(&output) {
            continue;
        }
        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(क)(आ)-5"),
                "आगन्तुक नामहरू सबै बिचमा ह्रस्व हुन्छन्",
                input,
                &output,
            )],
        ));
    }

    None
}

pub fn rule_medial_adjective_hrasva(input: &str) -> Option<Prakriya> {
    let lex = varnavinyas_kosha::kosha();
    if lex.contains(input) {
        return None;
    }
    if input.contains("हरू") {
        return None;
    }

    for output in hrasva_helpers::medial_dirgha_to_hrasva_candidates(input) {
        if !lex.contains(&output) || !hrasva_helpers::is_medial_hrasva_adjective(&output) {
            continue;
        }
        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(क)(आ)-6"),
                "सबै विशेषणहरू बिचमा ह्रस्व हुन्छन्",
                input,
                &output,
            )],
        ));
    }

    None
}

pub fn rule_medial_avyaya_hrasva(input: &str) -> Option<Prakriya> {
    let lex = varnavinyas_kosha::kosha();
    if lex.contains(input) {
        return None;
    }

    for output in hrasva_helpers::medial_dirgha_to_hrasva_candidates(input) {
        if !lex.contains(&output) || !hrasva_helpers::is_medial_hrasva_avyaya(&output) {
            continue;
        }
        if hrasva_helpers::is_initial_hrasva_onomatopoeic(&output) {
            continue;
        }
        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(क)(आ)-9"),
                "सबै अव्ययहरू बिचमा ह्रस्व हुन्छन्",
                input,
                &output,
            )],
        ));
    }

    None
}

pub fn rule_medial_onomatopoeic_hrasva(input: &str) -> Option<Prakriya> {
    let lex = varnavinyas_kosha::kosha();
    if lex.contains(input) {
        return None;
    }

    for output in hrasva_helpers::medial_dirgha_to_hrasva_candidates(input) {
        if !lex.contains(&output) || !hrasva_helpers::is_initial_hrasva_onomatopoeic(&output) {
            continue;
        }
        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(क)(आ)-10"),
                "सबै अनुकरणात्मक शब्दहरू बिचमा ह्रस्व हुन्छन्",
                input,
                &output,
            )],
        ));
    }

    None
}
