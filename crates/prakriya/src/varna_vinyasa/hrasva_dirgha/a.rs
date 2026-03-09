use super::helpers::hrasva_helpers;
use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::step::Step;
use varnavinyas_shabda::{Origin, classify, decompose};

// -----------------------------------------------------------------------------
// 3(क)(अ) शब्दका सुरुमा ह्रस्व इकार र उकारको प्रयोग
// Rule-book map:
// - 1  implemented in `rule_prefix_hrasva`
// - 2  implemented in `rule_dvi_tri_hrasva`
// - 3  implemented in `rule_initial_name_hrasva`
// - 4  implemented in `rule_initial_aagantuk_hrasva`
// - 5  implemented in `rule_pronoun_vowel_length`
// - 6  implemented in `rule_initial_adjective_hrasva`
// - 7  implemented in `rule_initial_number_hrasva`
// - 8  TODO: धातुहरू
// - 9  TODO: क्रियापदहरू
// - 10 implemented in `rule_initial_avyaya_hrasva`
// - 11 implemented in `rule_initial_onomatopoeic_hrasva`
// - 12 implemented/shared in `rule_tadbhav_hrasva` and `rule_kinship_tadbhav`
// - 13 implemented in `rule_suffix_nu_hrasva` / `rule_suffix_eli_hrasva`
// -----------------------------------------------------------------------------
// 3(क)(अ)-1: उपसर्गबाट बनेका शब्दमा सुरुको इ/उ ह्रस्व.
pub const SPEC_PREFIX_HRASVA: RuleSpec = RuleSpec {
    id: "hd-prefix-hrasva",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 195,
    citation: Rule::VarnaVinyasNiyam("3(क)(अ)-1"),
    examples: &[("नीबन्ध", "निबन्ध"), ("दूर्गति", "दुर्गति")],
};

// 3(क)(अ)-2: द्वि/त्रि अगाडि आउने सङ्ख्यावाचक शब्दमा ह्रस्व.
pub const SPEC_DVI_TRI_HRASVA: RuleSpec = RuleSpec {
    id: "hd-dvi-tri-hrasva",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 197,
    citation: Rule::VarnaVinyasNiyam("3(क)(अ)-2"),
    examples: &[("द्वीतीय", "द्वितीय"), ("त्रीयोग", "त्रियोग")],
};

// 3(क)(अ)-3: सबै अव्युत्पन्न नामहरू सुरुमा ह्रस्व हुन्छन्.
pub const SPEC_INITIAL_NAME_HRASVA: RuleSpec = RuleSpec {
    id: "hd-initial-name-hrasva",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 198,
    citation: Rule::VarnaVinyasNiyam("3(क)(अ)-3"),
    examples: &[("कीसान", "किसान"), ("ऊलो", "उलो")],
};

// 3(क)(अ)-4: सबै आगन्तुक नामहरू सुरुमा ह्रस्व हुन्छन्.
pub const SPEC_INITIAL_AAGANTUK_HRASVA: RuleSpec = RuleSpec {
    id: "hd-initial-aagantuk-hrasva",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 199,
    citation: Rule::VarnaVinyasNiyam("3(क)(अ)-4"),
    examples: &[("ईन्साफ", "इन्साफ"), ("ऊमेर", "उमेर")],
};

// 3(क)(अ)-13: तत्सम शब्दमा नेपाली प्रत्यय लागेपछि सुरुका इ/उ ह्रस्व हुन्छन्.
pub const SPEC_SUFFIX_NU: RuleSpec = RuleSpec {
    id: "hd-suffix-nu",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 200,
    citation: Rule::VarnaVinyasNiyam("3(क)(अ)-13"),
    examples: &[("स्वीकार्नु", "स्विकार्नु")],
};

pub const SPEC_SUFFIX_ELI: RuleSpec = RuleSpec {
    id: "hd-suffix-eli",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 210,
    citation: Rule::VarnaVinyasNiyam("3(क)(अ)-13"),
    examples: &[("पूर्वेली", "पुर्वेली")],
};

// 3(क)(अ)-12: तद्भव/देशज/आगन्तुक शब्दमा अपेक्षित ठाउँमा ह्रस्व.
pub const SPEC_TADBHAV: RuleSpec = RuleSpec {
    id: "hd-tadbhav",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 230,
    citation: Rule::VarnaVinyasNiyam("3(क)(अ)-12"),
    examples: &[("मीठो", "मिठो")],
};

// Cross-cutting pronoun rule:
// - 3(क)(अ)-5: non-monosyllabic pronouns start with hrasva
// - 3(क)(ऊ)-7: those same pronouns end in dirgha
pub const SPEC_PRONOUN: RuleSpec = RuleSpec {
    id: "hd-pronoun",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 225,
    citation: Rule::VarnaVinyasNiyam("3(क)(अ)-5/3(क)(ऊ)-7"),
    examples: &[("हामि", "हामी"), ("तीमी", "तिमी")],
};

// 3(क)(अ)-6: विशेषणको सुरुका इकार/उकार ह्रस्व हुन्छन्.
pub const SPEC_INITIAL_ADJECTIVE_HRASVA: RuleSpec = RuleSpec {
    id: "hd-initial-adjective-hrasva",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 226,
    citation: Rule::VarnaVinyasNiyam("3(क)(अ)-6"),
    examples: &[("ईमानदार", "इमानदार"), ("चीसो", "चिसो")],
};

// 3(क)(अ)-7: 'तीन'बाहेक सङ्ख्यावाचक शब्दहरू सुरुमा ह्रस्व हुन्छन्.
pub const SPEC_INITIAL_NUMBER_HRASVA: RuleSpec = RuleSpec {
    id: "hd-initial-number-hrasva",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 227,
    citation: Rule::VarnaVinyasNiyam("3(क)(अ)-7"),
    examples: &[("ऊन्नाइस", "उन्नाइस"), ("त्रीचालिस", "त्रिचालिस")],
};

// 3(क)(अ)-10: अव्ययहरू सबै सुरुमा ह्रस्व हुन्छन्.
pub const SPEC_INITIAL_AVYAYA_HRASVA: RuleSpec = RuleSpec {
    id: "hd-initial-avyaya-hrasva",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 233,
    citation: Rule::VarnaVinyasNiyam("3(क)(अ)-10"),
    examples: &[("हीजो", "हिजो"), ("भीत्र", "भित्र")],
};

// 3(क)(अ)-11: अनुकरणात्मक शब्दहरू सबै सुरुमा ह्रस्व हुन्छन्.
pub const SPEC_INITIAL_ONOMATOPOEIC_HRASVA: RuleSpec = RuleSpec {
    id: "hd-initial-onomatopoeic-hrasva",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 234,
    citation: Rule::VarnaVinyasNiyam("3(क)(अ)-11"),
    examples: &[("कीटिक्क", "किटिक्क"), ("टूलुटुलु", "टुलुटुलु")],
};

pub fn rule_prefix_hrasva(input: &str) -> Option<Prakriya> {
    const PREFIX_PATTERNS: &[(&str, &str)] = &[
        ("नी", "नि"),
        ("दू", "दु"),
        ("वी", "वि"),
        ("ऊत", "उत"),
        ("ऊप", "उप"),
        ("कू", "कु"),
        ("सू", "सु"),
    ];

    const CANONICAL_PREFIXES: &[&str] = &["नि", "दु", "वि", "उत", "उप", "कु", "सु"];

    let lex = varnavinyas_kosha::kosha();
    if lex.contains(input) {
        return None;
    }
    for &(wrong, correct) in PREFIX_PATTERNS {
        if !input.starts_with(wrong) {
            continue;
        }
        let output = input.replacen(wrong, correct, 1);
        if output == input || !lex.contains(&output) {
            continue;
        }
        let has_valid_prefix_stem = CANONICAL_PREFIXES.iter().any(|prefix| {
            output
                .strip_prefix(prefix)
                .is_some_and(|rest| rest.chars().count() >= 2 && lex.contains(rest))
        });
        if !has_valid_prefix_stem {
            continue;
        }
        if hrasva_helpers::is_initial_hrasva_onomatopoeic(&output) {
            continue;
        }
        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(क)(अ)-1"),
                "उपसर्गबाट बनेका शब्दहरू ह्रस्व हुन्छन्",
                input,
                &output,
            )],
        ));
    }
    None
}

pub fn rule_dvi_tri_hrasva(input: &str) -> Option<Prakriya> {
    let lex = varnavinyas_kosha::kosha();
    for (wrong, correct) in [("द्वी", "द्वि"), ("त्री", "त्रि")] {
        if !input.starts_with(wrong) {
            continue;
        }
        let output = input.replacen(wrong, correct, 1);
        if output == input || !lex.contains(&output) {
            continue;
        }
        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(क)(अ)-2"),
                "द्वि/त्रि अगाडि आउने सङ्ख्यावाचक शब्दमा ह्रस्व हुन्छ",
                input,
                &output,
            )],
        ));
    }
    None
}

pub fn rule_initial_name_hrasva(input: &str) -> Option<Prakriya> {
    let lex = varnavinyas_kosha::kosha();
    if lex.contains(input) {
        return None;
    }

    let output = hrasva_helpers::initial_dirgha_to_hrasva(input)?;
    if !lex.contains(&output) {
        return None;
    }
    if input == "तीन" || output == "तिन" {
        return None;
    }
    if hrasva_helpers::is_initial_hrasva_adjective(&output)
        || hrasva_helpers::is_initial_hrasva_number(&output)
        || hrasva_helpers::is_initial_hrasva_avyaya(&output)
        || hrasva_helpers::is_initial_hrasva_onomatopoeic(&output)
    {
        return None;
    }

    let morphology = decompose(&output);
    if !morphology.prefixes.is_empty() || !morphology.suffixes.is_empty() {
        return None;
    }
    if matches!(morphology.origin, Origin::Aagantuk | Origin::Tatsam) {
        return None;
    }

    Some(Prakriya::corrected(
        input,
        &output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam("3(क)(अ)-3"),
            "सबै अव्युत्पन्न नामहरू सुरुमा ह्रस्व हुन्छन्",
            input,
            &output,
        )],
    ))
}

pub fn rule_initial_aagantuk_hrasva(input: &str) -> Option<Prakriya> {
    let lex = varnavinyas_kosha::kosha();
    if lex.contains(input) {
        return None;
    }

    let output = hrasva_helpers::initial_dirgha_to_hrasva(input)?;
    if !lex.contains(&output) {
        return None;
    }
    if !matches!(classify(&output), Origin::Aagantuk) {
        return None;
    }
    if hrasva_helpers::is_initial_hrasva_onomatopoeic(&output) {
        return None;
    }

    Some(Prakriya::corrected(
        input,
        &output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam("3(क)(अ)-4"),
            "सबै आगन्तुक नामहरू सुरुमा ह्रस्व हुन्छन्",
            input,
            &output,
        )],
    ))
}

pub fn rule_suffix_nu_hrasva(input: &str) -> Option<Prakriya> {
    if !(input.ends_with("नु") || input.ends_with("र्नु")) {
        return None;
    }
    if !input.contains('ी') {
        return None;
    }

    let suffix_start = input.rfind("कार्नु").or_else(|| input.rfind("नु"))?;
    let prefix_part = &input[..suffix_start];
    let last_ii_pos = prefix_part.rfind('ी')?;
    let mut output = String::with_capacity(input.len());
    let mut pos = 0;
    for ch in input.chars() {
        let byte_pos = pos;
        pos += ch.len_utf8();
        if byte_pos == last_ii_pos {
            output.push('ि');
        } else {
            output.push(ch);
        }
    }

    if output != input {
        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(क)(अ)-13"),
                "प्रत्यय -नु लाग्दा मूल स्वर ह्रस्व हुन्छ",
                input,
                &output,
            )],
        ));
    }
    None
}

pub fn rule_suffix_eli_hrasva(input: &str) -> Option<Prakriya> {
    if !(input.ends_with("एली") || input.ends_with("ेली")) {
        return None;
    }
    if !input.contains('ू') {
        return None;
    }

    let suffix_start = input.rfind("ेली")?;
    let prefix_part = &input[..suffix_start];
    let last_uu_pos = prefix_part.rfind('ू')?;
    let mut output = String::with_capacity(input.len());
    let mut pos = 0;
    for ch in input.chars() {
        let byte_pos = pos;
        pos += ch.len_utf8();
        if byte_pos == last_uu_pos {
            output.push('ु');
        } else {
            output.push(ch);
        }
    }

    if output != input {
        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(क)(अ)-13"),
                "प्रत्यय -एली लाग्दा मूल स्वर ह्रस्व हुन्छ",
                input,
                &output,
            )],
        ));
    }
    None
}

/// Academy 3(क)(अ) नियम ३-१२: तद्भव/देशज/आगन्तुक शब्दमा ह्रस्व प्रयोग हुन्छ।
/// गैर-तत्सम शब्दमा अपेक्षित ठाउँमा दीर्घ ई/ऊ आएमा सुधार गरिन्छ।
pub fn rule_tadbhav_hrasva(input: &str) -> Option<Prakriya> {
    let origin = classify(input);
    if matches!(origin, Origin::Tatsam) {
        return None;
    }
    if rule_prefix_hrasva(input).is_some() {
        return None;
    }
    if hrasva_helpers::is_feminine_dirgha_pattern(input)
        || hrasva_helpers::is_kinship_dirgha_pattern(input)
        || hrasva_helpers::is_pronoun_candidate(input)
        || hrasva_helpers::has_tatsam_suffix(input)
        || input.contains("हरू")
    {
        return None;
    }

    let chars: Vec<char> = input.chars().collect();
    if chars.len() < 2 {
        return None;
    }

    let mut changed = false;
    let mut output_chars = chars.clone();
    for i in 0..chars.len().saturating_sub(1) {
        match chars[i] {
            'ी' => {
                output_chars[i] = 'ि';
                changed = true;
            }
            'ू' => {
                output_chars[i] = 'ु';
                changed = true;
            }
            'ई' => {
                output_chars[i] = 'इ';
                changed = true;
            }
            'ऊ' => {
                output_chars[i] = 'उ';
                changed = true;
            }
            _ => {}
        }
    }

    if changed {
        let output: String = output_chars.into_iter().collect();
        let kosha = varnavinyas_kosha::kosha();
        if !kosha.contains(&output) {
            return None;
        }
        if hrasva_helpers::has_specific_hrasva_prefix_structure(&output)
            || hrasva_helpers::is_initial_underived_name_candidate(&output)
            || hrasva_helpers::is_initial_aagantuk_name_candidate(&output)
            || hrasva_helpers::is_initial_hrasva_adjective(&output)
            || hrasva_helpers::is_initial_hrasva_number(&output)
            || hrasva_helpers::is_initial_hrasva_avyaya(&output)
            || hrasva_helpers::is_initial_hrasva_onomatopoeic(&output)
            || hrasva_helpers::has_medial_hrasva_suffix_family(&output)
            || hrasva_helpers::is_medial_hrasva_aagantuk_name(&output)
            || hrasva_helpers::is_medial_hrasva_adjective(&output)
            || hrasva_helpers::is_medial_hrasva_avyaya(&output)
        {
            return None;
        }

        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(क)(अ)-12"),
                "तद्भव/देशज शब्दमा ह्रस्व स्वर प्रयोग हुन्छ",
                input,
                &output,
            )],
        ));
    }

    None
}

/// Academy 3(क)(अ)-5 and 3(क)(ऊ)-7:
/// one-syllable pronouns are excluded, but the common multi-syllable pronouns
/// start with hrasva and end with dirgha.
pub fn rule_pronoun_vowel_length(input: &str) -> Option<Prakriya> {
    static PRONOUNS: &[&str] = &["तिमी", "तिनी", "यिनी", "उनी", "हामी"];

    for &correct in PRONOUNS {
        let hrasva_final = hrasva_helpers::replace_final_dirgha_with_hrasva(correct);
        if input == hrasva_final {
            return Some(Prakriya::corrected(
                input,
                correct,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(क)(ऊ)-7"),
                    "सर्वनामको अन्त्यमा दीर्घ हुन्छ",
                    input,
                    correct,
                )],
            ));
        }

        if let Some(wrong_start) = hrasva_helpers::pronoun_wrong_dirgha_start(correct) {
            if input == wrong_start {
                return Some(Prakriya::corrected(
                    input,
                    correct,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(क)(अ)-5"),
                        "एकाक्षरीबाहेक सर्वनामको सुरुमा ह्रस्व इ/उ हुन्छ",
                        input,
                        correct,
                    )],
                ));
            }

            let wrong_both = hrasva_helpers::replace_final_dirgha_with_hrasva(&wrong_start);
            if input == wrong_both {
                let interim = hrasva_helpers::replace_final_dirgha_with_hrasva(correct);
                return Some(Prakriya::corrected(
                    input,
                    correct,
                    vec![
                        Step::new(
                            Rule::VarnaVinyasNiyam("3(क)(अ)-5"),
                            "एकाक्षरीबाहेक सर्वनामको सुरुमा ह्रस्व इ/उ हुन्छ",
                            input,
                            &interim,
                        ),
                        Step::new(
                            Rule::VarnaVinyasNiyam("3(क)(ऊ)-7"),
                            "सर्वनामको अन्त्यमा दीर्घ हुन्छ",
                            &interim,
                            correct,
                        ),
                    ],
                ));
            }
        }
    }

    None
}

pub fn rule_initial_adjective_hrasva(input: &str) -> Option<Prakriya> {
    let lex = varnavinyas_kosha::kosha();
    if lex.contains(input) {
        return None;
    }

    let output = hrasva_helpers::initial_dirgha_to_hrasva(input)?;
    if !lex.contains(&output) || !hrasva_helpers::is_initial_hrasva_adjective(&output) {
        return None;
    }

    Some(Prakriya::corrected(
        input,
        &output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam("3(क)(अ)-6"),
            "विशेषणको सुरुका इकार उकार ह्रस्व हुन्छन्",
            input,
            &output,
        )],
    ))
}

pub fn rule_initial_number_hrasva(input: &str) -> Option<Prakriya> {
    let lex = varnavinyas_kosha::kosha();
    if lex.contains(input) {
        return None;
    }

    let output = hrasva_helpers::initial_dirgha_to_hrasva(input)?;
    if !lex.contains(&output) || !hrasva_helpers::is_initial_hrasva_number(&output) {
        return None;
    }

    Some(Prakriya::corrected(
        input,
        &output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam("3(क)(अ)-7"),
            "सङ्ख्यावाचक शब्दहरू 'तीन'बाहेक सबै सुरुमा ह्रस्व हुन्छन्",
            input,
            &output,
        )],
    ))
}

pub fn rule_initial_avyaya_hrasva(input: &str) -> Option<Prakriya> {
    let lex = varnavinyas_kosha::kosha();
    if lex.contains(input) {
        return None;
    }

    let output = hrasva_helpers::initial_dirgha_to_hrasva(input)?;
    if !lex.contains(&output) || !hrasva_helpers::is_initial_hrasva_avyaya(&output) {
        return None;
    }
    if hrasva_helpers::is_initial_hrasva_onomatopoeic(&output) {
        return None;
    }

    Some(Prakriya::corrected(
        input,
        &output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam("3(क)(अ)-10"),
            "अव्ययहरू सबै सुरुमा ह्रस्व हुन्छन्",
            input,
            &output,
        )],
    ))
}

pub fn rule_initial_onomatopoeic_hrasva(input: &str) -> Option<Prakriya> {
    let lex = varnavinyas_kosha::kosha();
    if lex.contains(input) {
        return None;
    }

    let output = hrasva_helpers::initial_dirgha_to_hrasva(input)?;
    if !lex.contains(&output) || !hrasva_helpers::is_initial_hrasva_onomatopoeic(&output) {
        return None;
    }

    Some(Prakriya::corrected(
        input,
        &output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam("3(क)(अ)-11"),
            "अनुकरणात्मक शब्दहरू सबै सुरुमा ह्रस्व हुन्छन्",
            input,
            &output,
        )],
    ))
}
