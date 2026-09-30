use super::helpers::{final_classes, has_exact_headword_left_compound_tail};
use super::u::rule_initial_tatsam_dirgha;
use crate::model::prakriya::Prakriya;
use crate::model::rule::Rule;
use crate::model::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::model::step::Step;
use varnavinyas_shabda::{Origin, classify};

// -----------------------------------------------------------------------------
// 3(क)(ऊ) शब्दका अन्त्यमा दीर्घ ईकार/ऊकारको प्रयोग
// Rule-book map:
// - 1  implemented in `rule_final_ii_suffix_dirgha`
// - 2  implemented conservatively in `rule_final_vati_vi_dirgha`
// - 3  implemented in `rule_dirgha_endings` and `rule_kinship_tadbhav`
// - 4  reviewed source examples in `rule_reviewed_final_dirgha`
//      TODO: generalize beyond reviewed examples with semantic evidence.
//      Blocker: current lexicon/POS data do not reliably separate feminine
//      adjectives from noun/adjective homographs. Academy examples like गोरी,
//      काली, सानी, मोटी are inconsistently tagged (some adjective, some noun,
//      some derived forms), so a systematic rule here would currently overlap
//      too much with the broader final-dirgha adjective/feminine classes.
// - 5  implemented/shared in `rule_dirgha_endings`, `final_dirgha_class_for`
// - 6  reviewed source examples in `rule_reviewed_final_dirgha`
//      TODO: generalize beyond reviewed examples with semantic evidence.
//      Blocker: current morphology/lexicon layers do not expose a reliable
//      animate-vs-inanimate distinction, so a systematic implementation would
//      overreach without extra semantic metadata.
// - 7  implemented/shared in `rule_pronoun_vowel_length`
// - 8  implemented in `rule_final_adjective_dirgha` and `final_dirgha_class_for`
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

pub const SPEC_REVIEWED_FINAL_DIRGHA: RuleSpec = RuleSpec {
    id: "hd-reviewed-final-dirgha",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 238,
    citation: Rule::VarnaVinyasNiyam("3(क)(ऊ)-4,6"),
    examples: &[("चुच्चि", "चुच्ची"), ("फर्सि", "फर्सी")],
};

pub fn rule_reviewed_final_dirgha(input: &str) -> Option<Prakriya> {
    let base = input.strip_suffix('ि')?;
    let candidate = format!("{base}ी");
    let entry = super::final_ii_classes::lookup(&candidate)?;
    Some(Prakriya::corrected(
        input,
        entry.word,
        vec![Step::new(
            Rule::VarnaVinyasNiyam(entry.rule_code),
            entry.explanation,
            input,
            entry.word,
        )],
    ))
}

pub const SPEC_FINAL_VATI_VI_DIRGHA: RuleSpec = RuleSpec {
    id: "hd-final-vati-vi-dirgha",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 239,
    citation: Rule::VarnaVinyasNiyam("3(क)(ऊ)-2"),
    examples: &[("रूपवति", "रूपवती"), ("मेधावि", "मेधावी")],
};

pub const SPEC_FINAL_ADJECTIVE_DIRGHA: RuleSpec = RuleSpec {
    id: "hd-final-adjective-dirgha",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 239,
    citation: Rule::VarnaVinyasNiyam("3(क)(ऊ)-8"),
    examples: &[("धनि", "धनी"), ("ज्ञानि", "ज्ञानी")],
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
    let stem = input.strip_suffix('ि')?;
    let output = format!("{stem}ी");
    if !super::ii_suffix::is_reviewed_derivative(&output) {
        return None;
    }

    Some(Prakriya::corrected(
        input,
        &output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam("3(क)(ऊ)-1"),
            super::ii_suffix::EXPLANATION,
            input,
            &output,
        )],
    ))
}

pub fn rule_final_vati_vi_dirgha(input: &str) -> Option<Prakriya> {
    let origin = classify(input);
    if matches!(origin, Origin::Tatsam) || !input.ends_with('ि') {
        return None;
    }
    if final_classes::ps_final_dirgha_exception_for_hrasva(input).is_some() {
        return None;
    }

    let chars: Vec<char> = input.chars().collect();
    let mut output_chars = chars.clone();
    *output_chars.last_mut().unwrap() = 'ी';
    let output: String = output_chars.into_iter().collect();

    if !final_classes::is_vati_vi_suffix_dirgha(&output) {
        return None;
    }

    let kosha = varnavinyas_kosha::kosha();
    if !kosha.contains(&output) || kosha.contains(input) {
        return None;
    }

    Some(Prakriya::corrected(
        input,
        &output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam("3(क)(ऊ)-2"),
            "वती, वी प्रत्यय लागेर बनेका शब्दहरू दीर्घ हुन्छन्",
            input,
            &output,
        )],
    ))
}

pub fn rule_final_adjective_dirgha(input: &str) -> Option<Prakriya> {
    if !input.ends_with('ि') {
        return None;
    }

    if rule_final_ii_suffix_dirgha(input).is_some() || rule_reviewed_final_dirgha(input).is_some() {
        return None;
    }

    let chars: Vec<char> = input.chars().collect();
    let mut output_chars = chars.clone();
    *output_chars.last_mut().unwrap() = 'ी';
    let output: String = output_chars.into_iter().collect();

    let kosha = varnavinyas_kosha::kosha();
    if !kosha.contains(&output) || kosha.contains(input) {
        return None;
    }

    if !final_classes::is_adjective_final_dirgha(&output) {
        return None;
    }

    Some(Prakriya::corrected(
        input,
        &output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam("3(क)(ऊ)-8"),
            "सबै ईकारान्त विशेषणहरू दीर्घ हुन्छन्",
            input,
            &output,
        )],
    ))
}

pub fn rule_dirgha_endings(input: &str) -> Option<Prakriya> {
    if rule_reviewed_final_dirgha(input).is_some() {
        return None;
    }
    // The pronoun class supplies the applicable reason. Its final ी does not
    // establish a feminine noun or gerund. Independently evidenced suffix
    // and adjective senses are still collected by their own rules.
    if super::rule_pronoun_vowel_length(input).is_some() {
        return None;
    }
    if let Some(output) = final_classes::ps_final_dirgha_exception_for_hrasva(input) {
        return Some(Prakriya::corrected(
            input,
            output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("PS-Saisanik-ह्रस्वदीर्घ-(च)-अपवाद"),
                "शैक्षणिक व्याकरण ह्रस्वदीर्घ (च): शब्दान्तमा दीर्घ चलेका अपवाद शब्द दीर्घ नै लेखिन्छन्",
                input,
                output,
            )],
        ));
    }

    let origin = classify(input);
    if matches!(origin, Origin::Tatsam) {
        return None;
    }

    // 3(क)(ई)-1 owns words whose first syllable preserves tatsam दीर्घ.
    // Broad final-dirgha heuristics should back off in those cases so words
    // like भुमि -> भूमि do not also surface bogus alternates such as भुमी.
    if rule_initial_tatsam_dirgha(input).is_some() {
        return None;
    }

    let chars: Vec<char> = input.chars().collect();
    if chars.is_empty() {
        return None;
    }

    let last = *chars.last().unwrap();

    static DIRGHA_II_ENDINGS: &[&str] = &["नी", "डी", "सानी"];
    static HRASVA_NI_COMPOUND_TAILS: &[&str] = &["अनि", "पनि", "मुनि", "बर्सेनि"];
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

        if final_classes::is_adjective_final_dirgha(&explicit_dirgha) {
            return Some(Prakriya::corrected(
                input,
                &explicit_dirgha,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(क)(ऊ)-8"),
                    "सबै ईकारान्त विशेषणहरू दीर्घ हुन्छन्",
                    input,
                    &explicit_dirgha,
                )],
            ));
        }

        // The remaining endings are heuristic. Preserve attested headwords
        // such as the conjunction कि and postposition पछि. Explicit source
        // classes above and correction-table entries still take precedence.
        if varnavinyas_kosha::kosha().lookup(input).is_some() {
            return None;
        }

        let char_count = chars.len();
        if (2..=4).contains(&char_count) {
            let penult = chars[char_count - 2];
            let stem = input.strip_suffix('ि').unwrap_or(input);
            let verb_supported = [format!("{stem}नु"), format!("{stem}्नु")]
                .iter()
                .any(|verb| varnavinyas_kosha::kosha().lookup(verb).is_some());
            if varnavinyas_akshar::is_vyanjan(penult) && verb_supported {
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
                if hrasva_ending == "नि"
                    && has_exact_headword_left_compound_tail(input, HRASVA_NI_COMPOUND_TAILS)
                {
                    return None;
                }
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
        .chain(super::rule_pronoun_vowel_length(input))
        .chain(rule_reviewed_final_dirgha(input))
        .any(|p| p.output == expected_output)
}
