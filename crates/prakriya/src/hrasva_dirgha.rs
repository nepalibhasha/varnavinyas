use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::step::Step;
use varnavinyas_shabda::{Origin, classify};

pub const SPEC_SUFFIX_NU: RuleSpec = RuleSpec {
    id: "hd-suffix-nu",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 200,
    citation: Rule::VarnaVinyasNiyam("3(क)-suffix-नु"),
    examples: &[("स्वीकार्नु", "स्विकार्नु")],
};

pub const SPEC_SUFFIX_ELI: RuleSpec = RuleSpec {
    id: "hd-suffix-eli",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 210,
    citation: Rule::VarnaVinyasNiyam("3(क)-suffix-एली"),
    examples: &[("पूर्वेली", "पुर्वेली")],
};

pub const SPEC_SUFFIX_PRESERVES: RuleSpec = RuleSpec {
    id: "hd-suffix-preserves",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 220,
    citation: Rule::VarnaVinyasNiyam("3(क)(उ)"),
    examples: &[("पुर्वी", "पूर्वी"), ("पुर्वीय", "पूर्वीय")],
};

pub const SPEC_TADBHAV: RuleSpec = RuleSpec {
    id: "hd-tadbhav",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 230,
    citation: Rule::VarnaVinyasNiyam("3(क)-12"),
    examples: &[("मीठो", "मिठो")],
};

pub const SPEC_DIRGHA_ENDINGS: RuleSpec = RuleSpec {
    id: "hd-dirgha-endings",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 240,
    citation: Rule::VarnaVinyasNiyam("3(ई)"),
    examples: &[("भनि", "भनी"), ("गरि", "गरी")],
};

pub const SPEC_KINSHIP: RuleSpec = RuleSpec {
    id: "hd-kinship",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 250,
    citation: Rule::VarnaVinyasNiyam("3(क)(इ)-1"),
    examples: &[("दाजू", "दाजु"), ("भाउजु", "भाउजू")],
};

pub const SPEC_KOSHA_BACKED: RuleSpec = RuleSpec {
    id: "hd-kosha-backed",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 260,
    citation: Rule::VarnaVinyasNiyam("3(क)(ई)"),
    examples: &[("नेपालि", "नेपाली")],
};

pub fn rule_suffix_nu_hrasva(input: &str) -> Option<Prakriya> {
    // Guard: only applicable to words ending in -नु प्रत्यय (suffix)
    if !(input.ends_with("नु") || input.ends_with("र्नु")) {
        return None;
    }

    // स्वीकार्नु → स्विकार्नु
    // Only replace the LAST दीर्घ ई before the प्रत्यय, not all occurrences.
    if !input.contains('ी') {
        return None;
    }

    // Find the प्रत्यय position to scope our search
    let suffix_start = input.rfind("कार्नु").or_else(|| input.rfind("नु"))?;
    let prefix_part = &input[..suffix_start];

    // Find the last ई in the part before the प्रत्यय
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
                Rule::VarnaVinyasNiyam("3(क)-suffix-नु"),
                "प्रत्यय -नु लाग्दा मूल स्वर ह्रस्व हुन्छ",
                input,
                &output,
            )],
        ));
    }
    None
}

pub fn rule_suffix_eli_hrasva(input: &str) -> Option<Prakriya> {
    // Guard: only applicable to words ending in -एली प्रत्यय (suffix)
    if !(input.ends_with("एली") || input.ends_with("ेली")) {
        return None;
    }

    // पूर्वेली → पुर्वेली
    // Only replace the LAST दीर्घ ू before the प्रत्यय, not all occurrences.
    if !input.contains('ू') {
        return None;
    }

    let suffix_start = input.rfind("ेली")?;
    let prefix_part = &input[..suffix_start];

    // Find the last ू in the part before the suffix
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
                Rule::VarnaVinyasNiyam("3(क)-suffix-एली"),
                "प्रत्यय -एली लाग्दा मूल स्वर ह्रस्व हुन्छ",
                input,
                &output,
            )],
        ));
    }
    None
}

/// Academy 3(क)(उ) नियम १-२: प्रत्यय -ई/-ईय लाग्दा मूल शब्दको दीर्घ कायम रहन्छ।
/// अनावश्यक false-positive नहोस् भनेर सीमित ज्ञात रूपमै लागू गरिन्छ।
pub fn rule_suffix_preserves_dirgha(input: &str) -> Option<Prakriya> {
    // प्रत्यय लागेपछि दीर्घ हराएका ज्ञात गलत→सही रूपहरू।
    // अनावश्यक false-positive रोक्न यिनै रूपहरू मात्र सच्याइन्छ।
    static KNOWN_CORRECTIONS: &[(&str, &str, &str)] = &[
        // (गलत, सही, नियम विवरण)
        ("पुर्वी", "पूर्वी", "प्रत्यय -ई ले दीर्घ: पूर्व + ई = पूर्वी"),
        ("पुर्वीय", "पूर्वीय", "प्रत्यय -ईय ले दीर्घ: पूर्व + ईय = पूर्वीय"),
    ];

    for &(wrong, correct, desc) in KNOWN_CORRECTIONS {
        if input == wrong {
            return Some(Prakriya::corrected(
                input,
                correct,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(क)(उ)"),
                    desc,
                    input,
                    correct,
                )],
            ));
        }
    }

    None
}

/// Academy 3(क) नियम ३-१२: तद्भव/देशज/आगन्तुक शब्दमा ह्रस्व प्रयोग हुन्छ।
/// गैर-तत्सम शब्दमा अपेक्षित ठाउँमा दीर्घ ई/ऊ आएमा सुधार गरिन्छ।
pub fn rule_tadbhav_hrasva(input: &str) -> Option<Prakriya> {
    let origin = classify(input);

    // गैर-तत्सम शब्दमा मात्र लागू गर्ने
    if matches!(origin, Origin::Tatsam) {
        return None;
    }

    // ज्ञात दीर्घ रूप (स्त्रीलिङ्ग, नातागोता, तत्सम प्रत्यय आदि) मा हस्तक्षेप नगर्ने।
    // ती रूपहरूलाई rule_dirgha_endings र rule_kinship_tadbhav ले सम्हाल्छ।
    if is_feminine_dirgha_pattern(input)
        || is_kinship_dirgha_pattern(input)
        || has_tatsam_suffix(input)
    {
        return None;
    }

    // तद्भव/देशज: शब्दादि र शब्दमध्यमा दीर्घ ई→इ, ऊ→उ
    // शब्दान्तका लागि छुट्टै नियम भएकाले यहाँ शब्दमध्य/शब्दादिमा मात्र।
    let chars: Vec<char> = input.chars().collect();
    if chars.len() < 2 {
        return None;
    }

    let mut changed = false;
    let mut output_chars = chars.clone();

    // शब्दमध्य (शब्दान्त बाहेक) मा अनपेक्षित दीर्घ जाँच्ने।
    // शब्दान्तका लागि छुट्टै नियम लागू हुन्छ।
    for i in 0..chars.len().saturating_sub(1) {
        match chars[i] {
            'ी' => {
                // गैर-तत्सम शब्दको शब्दमध्यमा दीर्घ मात्रा ई→इ
                output_chars[i] = 'ि';
                changed = true;
            }
            'ू' => {
                // गैर-तत्सम शब्दको शब्दमध्यमा दीर्घ मात्रा ऊ→उ
                output_chars[i] = 'ु';
                changed = true;
            }
            'ई' => {
                // स्वतन्त्र स्वर ई→इ (जस्तै: रमाईलो → रमाइलो)
                output_chars[i] = 'इ';
                changed = true;
            }
            'ऊ' => {
                // Independent vowel ऊ→उ
                output_chars[i] = 'उ';
                changed = true;
            }
            _ => {}
        }
    }

    if changed {
        let output: String = output_chars.into_iter().collect();

        // ह्रस्व रूप शब्दकोशले प्रमाणित गर्दा मात्र सुधार लागू गर्ने।
        // यसले वैध दीर्घ पद भएका समासमा false-positive घटाउँछ
        // (जस्तै: गाईप्रतिकोमा "गाई" वैध पद हो)।
        let kosha = varnavinyas_kosha::kosha();
        if !kosha.contains(&output) {
            return None;
        }

        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(क)-12"),
                "तद्भव/देशज शब्दमा ह्रस्व स्वर प्रयोग हुन्छ",
                input,
                &output,
            )],
        ));
    }

    None
}

/// Academy 3(क)(ऊ) नियम १-१६: स्त्रीलिङ्गी नामपद, -ई/-वती प्रत्यय, पेसा/स्थान/भाषा आदिमा दीर्घ।
pub fn rule_dirgha_endings(input: &str) -> Option<Prakriya> {
    let origin = classify(input);

    // यो नियम मुख्यतः तद्भव/देशज स्त्रीलिङ्गी शब्दान्तमा लागू हुन्छ।
    if matches!(origin, Origin::Tatsam) {
        return None;
    }

    let chars: Vec<char> = input.chars().collect();
    if chars.is_empty() {
        return None;
    }

    let last = *chars.last().unwrap();

    // अन्त्यमा दीर्घ ई चाहिने प्रचलित स्त्रीलिङ्गी/विशेषण/नामयोगी प्रत्यय
    static DIRGHA_II_ENDINGS: &[&str] = &[
        "नी",   // स्त्रीलिङ्गी प्रत्यय: बहिनी, सम्धिनी
        "डी",   // स्थानबोधक/विशेषण: पहाडी
        "सानी", // स्त्रीलिङ्गी: खुर्सानी
    ];

    // अन्त्यमा दीर्घ ई चाहिने नामयोगी र असमापक क्रिया रूपहरू
    static DIRGHA_II_WORDS: &[&str] = &["अगाडी", "पछाडी", "माथी", "तली"];

    // अन्त्यमा ह्रस्व इ आएर त्यहाँ दीर्घ ई अनिवार्य हुने अवस्था जाँच्ने
    if last == 'ि' {
        // असमापक क्रिया: -ि अन्त्य भएका रूप -ी हुनुपर्छ
        // जस्तै: भनि→भनी, गरि→गरी
        // २-४ वर्णका छोटा क्रियारूपमा मात्र
        let char_count = chars.len();
        if (2..=4).contains(&char_count) {
            // असमापक क्रिया-जस्तो रूप हो कि होइन जाँच्ने
            let penult = chars[char_count - 2];
            if varnavinyas_akshar::is_vyanjan(penult) {
                let mut output_chars = chars.clone();
                output_chars[char_count - 1] = 'ी';
                let output: String = output_chars.into_iter().collect();
                return Some(Prakriya::corrected(
                    input,
                    &output,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(ई)"),
                        "असमापक क्रियामा अन्त्यमा दीर्घ ई हुन्छ",
                        input,
                        &output,
                    )],
                ));
            }
        }

        // स्त्रीलिङ्गी/स्थानबोधक/नामयोगी शब्दान्त
        for ending in DIRGHA_II_ENDINGS {
            let hrasva_ending = ending.replace('ी', "ि");
            if input.ends_with(&hrasva_ending) {
                let output = format!("{}{}", &input[..input.len() - hrasva_ending.len()], ending);
                return Some(Prakriya::corrected(
                    input,
                    &output,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(ई)"),
                        "स्त्रीलिङ्गी/विशेषण शब्दमा अन्तिम दीर्घ ई",
                        input,
                        &output,
                    )],
                ));
            }
        }

        // निश्चित नामयोगी/अव्यय रूप जाँच्ने
        for &correct_word in DIRGHA_II_WORDS {
            let hrasva_form = correct_word.replace('ी', "ि");
            if input == hrasva_form {
                return Some(Prakriya::corrected(
                    input,
                    correct_word,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(ई)"),
                        "सर्वनाम/अव्यय/सम्बन्धवाचक शब्दमा दीर्घ ई",
                        input,
                        correct_word,
                    )],
                ));
            }
        }

        // शब्दकोश-आधारित fallback, छुट्टै kosha_backed_dirgha_correction नियमले सम्हाल्छ।
    }

    None
}

/// शब्दकोश-आधारित शब्दान्त ह्रस्व→दीर्घ सुधार।
///
/// छुट्टै नियम: गैर-तत्सम शब्द अन्त्यमा ह्रस्व (ि/ु) भए
/// त्यसको दीर्घ रूप शब्दकोशमा छ कि छैन जाँचिन्छ।
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

/// शब्दकोश-आधारित ह्रस्व→दीर्घ सुधारको आन्तरिक कार्यान्वयन।
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
        // ह्रस्व रूप आफैं शब्दकोशमा वैध छ भने त्रुटि नदेखाउने।
        return None;
    }

    let mut dirgha_chars: Vec<char> = chars.to_vec();
    *dirgha_chars.last_mut().unwrap() = dirgha;
    let dirgha_form: String = dirgha_chars.into_iter().collect();

    if kosha.contains(&dirgha_form) {
        let rule_ref = if dirgha == 'ी' {
            "3(क)(ई)"
        } else {
            "3(क)(ऊ)"
        };
        return Some(Prakriya::corrected(
            input,
            &dirgha_form,
            vec![Step::new(
                Rule::VarnaVinyasNiyam(rule_ref),
                format!("शब्दको अन्त्यमा दीर्घ {} आवश्यक (शब्दकोश प्रमाणित)", vowel_label),
                input,
                &dirgha_form,
            )],
        ));
    }

    None
}

/// Academy 3(क)(इ) नियम १: पुलिङ्ग नातागोता शब्दको अन्त्यमा ह्रस्व हुन्छ।
/// अपवाद: खसी, सम्धी, हात्ती, स्वामीमा दीर्घ हुन्छ।
pub fn rule_kinship_tadbhav(input: &str) -> Option<Prakriya> {
    let origin = classify(input);
    if !matches!(origin, Origin::Tadbhav | Origin::Deshaj) {
        return None;
    }

    // पुलिङ्ग नातागोता शब्द: अन्त्यमा ह्रस्व अनिवार्य
    // Academy 3(क)(इ)-1: दाजु, बाबु, भिनाजु, काका, मामा आदि
    // सामान्यतः अन्त्यमा व्यञ्जन + ु हुन्छ (दीर्घ ू होइन)
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

    // स्त्रीलिङ्गी नातागोता शब्द: अन्त्यमा दीर्घ अनिवार्य
    // Academy 3(ई): भाउजू, फुपू, सासू आदि
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
                    Rule::VarnaVinyasNiyam("3(ई)"),
                    "स्त्रीलिङ्ग नातागोता शब्दमा दीर्घ",
                    input,
                    correct,
                )],
            ));
        }
    }

    None
}

/// शब्दमा दीर्घ ई अपेक्षित तत्सम-व्युत्पन्न प्रत्यय छ कि छैन जाँच्ने।
fn has_tatsam_suffix(input: &str) -> bool {
    input.ends_with("ीकरण")
        || input.ends_with("ीकृत")
        || input.ends_with("ीकार")
        || input.ends_with("ीय")
        || input.ends_with("ीन")
}

/// शब्द ज्ञात स्त्रीलिङ्गी दीर्घ-अन्त्य ढाँचासँग मिल्छ कि छैन जाँच्ने।
fn is_feminine_dirgha_pattern(input: &str) -> bool {
    // ी मा अन्त्य हुने सही स्त्रीलिङ्गी रूपहरू
    input.ends_with("नी") || input.ends_with("डी") || input.ends_with("ती") || input.ends_with("ली")
}

/// शब्द नातागोता वर्गमा पर्छ कि (वा त्यसमाथि प्रत्यय लागेको रूप हो कि) जाँच्ने।
fn is_kinship_dirgha_pattern(input: &str) -> bool {
    static KINSHIP_BASES: &[&str] = &[
        "दिदी",
        "बहिनी",
        "भाउजू",
        "फुपू",
        "सासू",
        "जेठानी",
        "कान्छी",
        "बुहारी",
        "मितिनी",
    ];
    // ठीक उही रूप वा प्रत्यय-लागेका रूप (ले, मा, को, लाई आदि) जाँच्ने
    for base in KINSHIP_BASES {
        if input == *base || input.starts_with(base) {
            return true;
        }
    }
    false
}
