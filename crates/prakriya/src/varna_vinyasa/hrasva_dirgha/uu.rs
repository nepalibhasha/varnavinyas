use super::helpers::final_classes;
use crate::model::prakriya::Prakriya;
use crate::model::rule::Rule;
use crate::model::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::model::step::Step;
use varnavinyas_shabda::{Origin, classify, decompose};

// -----------------------------------------------------------------------------
// 3(क)(ऊ) शब्दका अन्त्यमा दीर्घ ईकार/ऊकारको प्रयोग
// Rule-book map:
// - 1  implemented in `rule_final_ii_suffix_dirgha`
// - 2  TODO: वती/वी प्रत्यय
// - 3  implemented in `rule_dirgha_endings` and `rule_kinship_tadbhav`
// - 4  TODO: स्त्रीलिङ्गी विशेषण
// - 5  implemented/shared in `rule_dirgha_endings`, `final_dirgha_class_for`
// - 6  TODO: ईकारान्त निर्जीव नाम
//      Blocker: current morphology/lexicon layers do not expose a reliable
//      animate-vs-inanimate distinction, so a systematic implementation would
//      overreach without extra semantic metadata.
// - 7  implemented/shared in `rule_pronoun_vowel_length`
// - 8  TODO: ईकारान्त विशेषण
// - 9  implemented/shared in `rule_dirgha_endings`, `final_dirgha_class_for`
// - 10 TODO: विध्यर्थक र स्त्रीलिङ्गी क्रियापद
// - 11 implemented/shared in `rule_dirgha_endings`, `final_dirgha_class_for`
// - 12 implemented/shared in `rule_dirgha_endings`, `final_dirgha_class_for`
// - 13 implemented/shared in `rule_dirgha_endings`, `final_dirgha_class_for`
// - 14 implemented in `rule_dirgha_endings`
// - 15 implemented/shared in `final_dirgha_class_for`
// - 16 implemented/shared in `final_dirgha_class_for`
// -----------------------------------------------------------------------------
pub const SPEC_FINAL_II_SUFFIX_DIRGHA: RuleSpec = RuleSpec {
    id: "hd-final-ii-suffix-dirgha",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 239,
    citation: Rule::VarnaVinyasNiyam("3(क)(ऊ)-1"),
    examples: &[("योगि", "योगी"), ("त्यागि", "त्यागी")],
};

pub const SPEC_DIRGHA_ENDINGS: RuleSpec = RuleSpec {
    id: "hd-dirgha-endings",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 240,
    citation: Rule::VarnaVinyasNiyam("3(क)(ऊ)"),
    examples: &[("भनि", "भनी"), ("गरि", "गरी")],
};

pub const SPEC_KOSHA_BACKED: RuleSpec = RuleSpec {
    id: "hd-kosha-backed",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 260,
    citation: Rule::VarnaVinyasNiyam("3(क)(ऊ)"),
    examples: &[("नेपालि", "नेपाली")],
};

pub fn rule_final_ii_suffix_dirgha(input: &str) -> Option<Prakriya> {
    let origin = classify(input);
    if matches!(origin, Origin::Tatsam) || !input.ends_with('ि') {
        return None;
    }

    let chars: Vec<char> = input.chars().collect();
    let mut output_chars = chars.clone();
    *output_chars.last_mut().unwrap() = 'ी';
    let output: String = output_chars.into_iter().collect();

    let kosha = varnavinyas_kosha::kosha();
    if !kosha.contains(&output) {
        return None;
    }

    let morphology = decompose(&output);
    if !morphology.suffixes.iter().any(|suffix| suffix == "ई") {
        return None;
    }
    if final_classes::final_dirgha_class_for(&output, "ई").0 != "3(क)(ऊ)" {
        return None;
    }

    Some(Prakriya::corrected(
        input,
        &output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam("3(क)(ऊ)-1"),
            "'ई' प्रत्यय अन्त्यमा आउने शब्दहरू दीर्घ हुन्छन्",
            input,
            &output,
        )],
    ))
}

pub fn rule_dirgha_endings(input: &str) -> Option<Prakriya> {
    let origin = classify(input);
    if matches!(origin, Origin::Tatsam) {
        return None;
    }

    let chars: Vec<char> = input.chars().collect();
    if chars.is_empty() {
        return None;
    }

    let last = *chars.last().unwrap();

    static DIRGHA_II_ENDINGS: &[&str] = &["नी", "डी", "सानी"];
    static HRASVA_FINAL_I_EXCEPTIONS: &[&str] = &[
        "अगाडि",
        "पछाडि",
        "माथि",
        "अनि",
        "पनि",
        "मुनि",
        "भोलि",
        "फेरि",
        "चोटि",
        "बर्सेनि",
        "देखि",
        "लागि",
        "जति",
        "कति",
        "उति",
        "नाति",
        "तापनि",
    ];
    static DIRGHA_II_WORDS: &[&str] = &["तली"];

    if last == 'ि' {
        if HRASVA_FINAL_I_EXCEPTIONS.contains(&input) {
            return None;
        }

        let mut explicit_dirgha_chars = chars.clone();
        *explicit_dirgha_chars.last_mut().unwrap() = 'ी';
        let explicit_dirgha: String = explicit_dirgha_chars.into_iter().collect();
        if final_classes::is_place_river_language_dirgha(&explicit_dirgha) {
            return Some(Prakriya::corrected(
                input,
                &explicit_dirgha,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(क)(ऊ)-11"),
                    "स्थान, नदी र भाषा बुझाउने शब्दमा दीर्घ हुन्छ",
                    input,
                    &explicit_dirgha,
                )],
            ));
        }

        if final_classes::is_number_final_dirgha(&explicit_dirgha) {
            return Some(Prakriya::corrected(
                input,
                &explicit_dirgha,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(क)(ऊ)-9"),
                    "सङ्ख्यावाचक शब्दहरू अन्त्यमा दीर्घ हुन्छन्",
                    input,
                    &explicit_dirgha,
                )],
            ));
        }

        if final_classes::is_profession_jati_thar_dirgha(&explicit_dirgha) {
            return Some(Prakriya::corrected(
                input,
                &explicit_dirgha,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(क)(ऊ)-5"),
                    "पेसा, जाति र थर बुझाउने शब्दमा दीर्घ हुन्छ",
                    input,
                    &explicit_dirgha,
                )],
            ));
        }

        if final_classes::is_ari_tari_adverb_dirgha(&explicit_dirgha) {
            return Some(Prakriya::corrected(
                input,
                &explicit_dirgha,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(क)(ऊ)-13"),
                    "अरी, तरी अन्त्यमा आउने अव्यय शब्दमा दीर्घ हुन्छ",
                    input,
                    &explicit_dirgha,
                )],
            ));
        }

        if final_classes::is_hi_final_dirgha(&explicit_dirgha) {
            return Some(Prakriya::corrected(
                input,
                &explicit_dirgha,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(क)(ऊ)-12"),
                    "'चाहिँ'बाहेक 'ही' अन्त्यमा आउने शब्दमा दीर्घ हुन्छ",
                    input,
                    &explicit_dirgha,
                )],
            ));
        }

        let char_count = chars.len();
        if (2..=4).contains(&char_count) {
            let penult = chars[char_count - 2];
            if varnavinyas_akshar::is_vyanjan(penult) {
                let mut output_chars = chars.clone();
                output_chars[char_count - 1] = 'ी';
                let output: String = output_chars.into_iter().collect();
                return Some(Prakriya::corrected(
                    input,
                    &output,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(क)(ऊ)-14"),
                        "असमापक क्रियामा अन्त्यमा दीर्घ ई हुन्छ",
                        input,
                        &output,
                    )],
                ));
            }
        }

        for ending in DIRGHA_II_ENDINGS {
            let hrasva_ending = ending.replace('ी', "ि");
            if input.ends_with(&hrasva_ending) {
                let output = format!("{}{}", &input[..input.len() - hrasva_ending.len()], ending);
                return Some(Prakriya::corrected(
                    input,
                    &output,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(क)(ऊ)-3"),
                        "स्त्रीलिङ्गी/विशेषण शब्दमा अन्तिम दीर्घ ई",
                        input,
                        &output,
                    )],
                ));
            }
        }

        for &correct_word in DIRGHA_II_WORDS {
            let hrasva_form = correct_word.replace('ी', "ि");
            if input == hrasva_form {
                return Some(Prakriya::corrected(
                    input,
                    correct_word,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(क)(ऊ)-12"),
                        "सर्वनाम/अव्यय/सम्बन्धवाचक शब्दमा दीर्घ ई",
                        input,
                        correct_word,
                    )],
                ));
            }
        }
    }

    if last == 'इ' {
        let mut explicit_dirgha_chars = chars.clone();
        *explicit_dirgha_chars.last_mut().unwrap() = 'ई';
        let explicit_dirgha: String = explicit_dirgha_chars.into_iter().collect();

        if final_classes::is_number_final_dirgha(&explicit_dirgha) {
            return Some(Prakriya::corrected(
                input,
                &explicit_dirgha,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(क)(ऊ)-9"),
                    "सङ्ख्यावाचक शब्दहरू अन्त्यमा दीर्घ हुन्छन्",
                    input,
                    &explicit_dirgha,
                )],
            ));
        }
    }

    if last == 'ु' {
        let mut explicit_dirgha_chars = chars.clone();
        *explicit_dirgha_chars.last_mut().unwrap() = 'ू';
        let explicit_dirgha: String = explicit_dirgha_chars.into_iter().collect();

        if final_classes::is_profession_jati_thar_dirgha(&explicit_dirgha) {
            return Some(Prakriya::corrected(
                input,
                &explicit_dirgha,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(क)(ऊ)-5"),
                    "पेसा, जाति र थर बुझाउने शब्दमा दीर्घ हुन्छ",
                    input,
                    &explicit_dirgha,
                )],
            ));
        }
    }

    None
}

pub fn kosha_backed_dirgha_correction(input: &str) -> Option<Prakriya> {
    let origin = classify(input);
    if matches!(origin, Origin::Tatsam) {
        return None;
    }

    let chars: Vec<char> = input.chars().collect();
    if chars.is_empty() {
        return None;
    }

    let last = *chars.last().unwrap();

    if last == 'ि' {
        return kosha_backed_dirgha_impl(input, &chars, 'ि', 'ी', "ई");
    }

    if last == 'ु' {
        return kosha_backed_dirgha_impl(input, &chars, 'ु', 'ू', "ऊ");
    }

    None
}

fn kosha_backed_dirgha_impl(
    input: &str,
    chars: &[char],
    hrasva: char,
    dirgha: char,
    vowel_label: &str,
) -> Option<Prakriya> {
    debug_assert_eq!(*chars.last().unwrap(), hrasva);

    let kosha = varnavinyas_kosha::kosha();
    if kosha.contains(input) {
        return None;
    }

    let mut dirgha_chars: Vec<char> = chars.to_vec();
    *dirgha_chars.last_mut().unwrap() = dirgha;
    let dirgha_form: String = dirgha_chars.into_iter().collect();

    if has_specific_final_dirgha_rule(input, &dirgha_form) {
        return None;
    }

    if kosha.contains(&dirgha_form) {
        let (rule_ref, explanation) =
            final_classes::final_dirgha_class_for(&dirgha_form, vowel_label);
        return Some(Prakriya::corrected(
            input,
            &dirgha_form,
            vec![Step::new(
                Rule::VarnaVinyasNiyam(rule_ref),
                explanation,
                input,
                &dirgha_form,
            )],
        ));
    }

    None
}

fn has_specific_final_dirgha_rule(input: &str, expected_output: &str) -> bool {
    super::rule_dirgha_endings(input)
        .into_iter()
        .chain(super::rule_kinship_tadbhav(input))
        .any(|p| p.output == expected_output)
}
