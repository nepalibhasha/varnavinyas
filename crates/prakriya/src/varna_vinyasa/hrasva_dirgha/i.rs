use super::helpers::{exact_headword_supported, final_classes, hrasva_helpers};
use crate::model::prakriya::Prakriya;
use crate::model::rule::Rule;
use crate::model::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::model::step::Step;
use varnavinyas_shabda::{Origin, classify};

// -----------------------------------------------------------------------------
// 3(क)(इ) शब्दका अन्त्यमा ह्रस्व इकार/उकारको प्रयोग
// Rule-book map:
// - 1  implemented in `rule_kinship_tadbhav` (masculine kinship hrasva)
// - 2  implemented/shared in `rule_final_hrasva_endings`
// - 3  implemented/shared in `rule_final_hrasva_endings`
// - 4  implemented/shared in `rule_final_hrasva_endings`
// - 5  implemented/shared in `rule_final_hrasva_endings`
// - 6  partial/shared in `rule_final_hrasva_endings`
// - 7  implemented/shared in `rule_final_hrasva_endings`
// - 8  TODO: 'नु'/'छु' प्रत्यय भएका क्रियापद (single-word context too ambiguous)
// - 9  implemented/shared in `rule_final_hrasva_endings`
// -----------------------------------------------------------------------------
pub const SPEC_KINSHIP: RuleSpec = RuleSpec {
    id: "hd-kinship",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 250,
    citation: Rule::VarnaVinyasNiyam("3(क)(इ)-1"),
    examples: &[("दाजू", "दाजु"), ("भाउजु", "भाउजू")],
};

pub const SPEC_FINAL_HRASVA_ENDINGS: RuleSpec = RuleSpec {
    id: "hd-final-hrasva-endings",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 245,
    citation: Rule::VarnaVinyasNiyam("3(क)(इ)"),
    examples: &[("आलू", "आलु"), ("गराई", "गराइ"), ("अगाडी", "अगाडि")],
};

pub fn rule_kinship_tadbhav(input: &str) -> Option<Prakriya> {
    let origin = classify(input);
    if !matches!(origin, Origin::Tadbhav | Origin::Deshaj) {
        return None;
    }

    static KINSHIP_HRASVA_BASES: &[(&str, &str, &str)] = &[
        ("दीदी", "दिदी", "तद्भव नातागोता शब्दमा सुरुको इ ह्रस्व हुन्छ"),
        ("बहीनी", "बहिनी", "तद्भव नातागोता शब्दमा शब्दमध्यको इ ह्रस्व हुन्छ"),
        ("मीतिनी", "मितिनी", "तद्भव नातागोता शब्दमा मूल इ ह्रस्व हुन्छ"),
        ("मीतिनि", "मितिनी", "तद्भव नातागोता शब्दमा मूल इ ह्रस्व हुन्छ"),
    ];

    for &(wrong_base, correct_base, desc) in KINSHIP_HRASVA_BASES {
        if let Some(rest) = input.strip_prefix(wrong_base) {
            let output = format!("{correct_base}{rest}");
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(क)(अ)-12"),
                    desc,
                    input,
                    &output,
                )],
            ));
        }
    }

    static MASC_KINSHIP_DIRGHA_TO_HRASVA: &[(&str, &str)] = &[
        ("दाजू", "दाजु"),
        ("बाबू", "बाबु"),
        ("भिनाजू", "भिनाजु"),
        ("साहू", "साहु"),
    ];

    for &(wrong, correct) in MASC_KINSHIP_DIRGHA_TO_HRASVA {
        if input == wrong {
            return Some(Prakriya::corrected(
                input,
                correct,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(क)(इ)-1"),
                    "पुलिङ्ग नातागोता शब्दमा ह्रस्व",
                    input,
                    correct,
                )],
            ));
        }
    }

    static FEM_KINSHIP_HRASVA_TO_DIRGHA: &[(&str, &str)] = &[
        ("भाउजु", "भाउजू"),
        ("फुपु", "फुपू"),
        ("सासु", "सासू"),
        ("बुहारि", "बुहारी"),
        ("जेठानि", "जेठानी"),
        ("कान्छि", "कान्छी"),
    ];

    for &(wrong, correct) in FEM_KINSHIP_HRASVA_TO_DIRGHA {
        if input == wrong {
            return Some(Prakriya::corrected(
                input,
                correct,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(क)(ऊ)-3"),
                    "स्त्रीलिङ्ग नातागोता शब्दमा दीर्घ",
                    input,
                    correct,
                )],
            ));
        }
    }

    None
}

pub fn rule_final_hrasva_endings(input: &str) -> Option<Prakriya> {
    let lex = varnavinyas_kosha::kosha();
    if input.ends_with("नू") {
        return None;
    }
    if final_classes::is_known_correct_final_dirgha(input) {
        return None;
    }
    let output = hrasva_helpers::replace_final_dirgha_with_hrasva(input);
    if output == input || !lex.contains(&output) {
        return None;
    }

    let (rule_ref, description) = final_classes::final_hrasva_class_for(&output)?;
    if rule_ref == "3(क)(इ)-9" && exact_headword_supported(input) {
        return None;
    }
    Some(Prakriya::corrected(
        input,
        &output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam(rule_ref),
            &description,
            input,
            &output,
        )],
    ))
}
