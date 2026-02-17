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
    // Guard: only applicable to words ending in -नु suffix
    if !(input.ends_with("नु") || input.ends_with("र्नु")) {
        return None;
    }

    // स्वीकार्नु → स्विकार्नु
    // Only replace the LAST दीर्घ ई before the suffix, not all occurrences.
    if !input.contains('ी') {
        return None;
    }

    // Find the suffix position to scope our search
    let suffix_start = input.rfind("कार्नु").or_else(|| input.rfind("नु"))?;
    let prefix_part = &input[..suffix_start];

    // Find the last ई in the part before the suffix
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
                "प्रत्यय -नु ले ह्रस्व on root vowel",
                input,
                &output,
            )],
        ));
    }
    None
}

pub fn rule_suffix_eli_hrasva(input: &str) -> Option<Prakriya> {
    // Guard: only applicable to words ending in -एली suffix
    if !(input.ends_with("एली") || input.ends_with("ेली")) {
        return None;
    }

    // पूर्वेली → पुर्वेली
    // Only replace the LAST दीर्घ ू before the suffix, not all occurrences.
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
                "प्रत्यय -एली ले ह्रस्व on root vowel",
                input,
                &output,
            )],
        ));
    }
    None
}

/// Academy 3(क)(उ) rules 1-2: suffixes -ई/-ईय preserve दीर्घ in the stem.
/// Only fires for specific known stem patterns to avoid false positives.
pub fn rule_suffix_preserves_dirgha(input: &str) -> Option<Prakriya> {
    // Known incorrect→correct pairs where a दीर्घ-suffix stem lost its दीर्घ.
    // We only correct specific known patterns to avoid false positives.
    static KNOWN_CORRECTIONS: &[(&str, &str, &str)] = &[
        // (incorrect, correct, rule description)
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

/// Academy 3(क) rules 3-12: तद्भव/deshaj/आगन्तुक words take ह्रस्व.
/// If a non-तत्सम word has दीर्घ ई/ऊ where ह्रस्व is expected, correct it.
pub fn rule_tadbhav_hrasva(input: &str) -> Option<Prakriya> {
    let origin = classify(input);

    // Only apply to non-तत्सम words
    if matches!(origin, Origin::Tatsam) {
        return None;
    }

    // Don't interfere with known दीर्घ patterns (feminine, नातासम्बन्धी, तत्सम suffixes, etc.)
    // Those are handled by rule_dirgha_endings and rule_नातासम्बन्धी_tadbhav
    if is_feminine_dirgha_pattern(input)
        || is_नातासम्बन्धी_dirgha_pattern(input)
        || has_tatsam_suffix(input)
    {
        return None;
    }

    // Tadbhav/Deshaj: word-initial and word-medial दीर्घ ई→इ, ऊ→उ
    // (not word-final, which has separate rules)
    let chars: Vec<char> = input.chars().collect();
    if chars.len() < 2 {
        return None;
    }

    let mut changed = false;
    let mut output_chars = chars.clone();

    // Check medial positions (not final) for unexpected दीर्घ
    // Final position has its own rules (दीर्घ for feminine, etc.)
    for i in 0..chars.len().saturating_sub(1) {
        match chars[i] {
            'ी' => {
                // Medial दीर्घ matra ई→इ in non-तत्सम words
                output_chars[i] = 'ि';
                changed = true;
            }
            'ू' => {
                // Medial दीर्घ matra ऊ→उ in non-तत्सम words
                output_chars[i] = 'ु';
                changed = true;
            }
            'ई' => {
                // Independent vowel ई→इ (e.g. रमाईलो → रमाइलो)
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

        // Only apply if the ह्रस्व form is validated by the dictionary.
        // This prevents false positives on compounds containing legitimate
        // दीर्घ stems (e.g. गाईप्रतिको — "गाई" is a valid word).
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

/// Academy 3(क)(ऊ) rules 1-16: feminine nouns, -ई/-वती suffixes, profession/place names → दीर्घ.
pub fn rule_dirgha_endings(input: &str) -> Option<Prakriya> {
    let origin = classify(input);

    // This rule primarily applies to तद्भव/deshaj feminine endings
    if matches!(origin, Origin::Tatsam) {
        return None;
    }

    let chars: Vec<char> = input.chars().collect();
    if chars.is_empty() {
        return None;
    }

    let last = *chars.last().unwrap();

    // Common feminine/adjectival/नामयोगी suffixes that require दीर्घ ई at end
    static DIRGHA_II_ENDINGS: &[&str] = &[
        "नी",   // feminine suffix: बहिनी, सम्धिनी
        "डी",   // demonym/adjective: पहाडी
        "सानी", // feminine: खुर्सानी
    ];

    // Postpositions and asamapaka verb forms that require final दीर्घ ई
    static DIRGHA_II_WORDS: &[&str] = &["अगाडी", "पछाडी", "माथी", "तली"];

    // Check if word ends in ह्रस्व इ where दीर्घ ई is अनिवार्य
    if last == 'ि' {
        // असमापक क्रिया: verb forms ending in -ि should be -ी
        // e.g., भनि→भनी, गरि→गरी
        // Only for short verb-like forms (2-4 chars)
        let char_count = chars.len();
        if (2..=4).contains(&char_count) {
            // Check if it looks like an asamapaka verb form
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

        // Feminine/demonym/नामयोगी endings
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

        // Check specific नामयोगी/adverb words
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

        // Dictionary-backed fallback moved to standalone kosha_backed_dirgha_correction rule
    }

    None
}

/// Dictionary-backed ह्रस्व→दीर्घ correction at word-final position.
///
/// Standalone rule: checks if a non-तत्सम word ends in ह्रस्व (ि/ु)
/// and the दीर्घ form exists in the dictionary.
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

/// Inner implementation for dictionary-backed ह्रस्व→दीर्घ correction.
fn kosha_backed_dirgha_impl(
    input: &str,
    chars: &[char],
    ह्रस्व: char,
    दीर्घ: char,
    vowel_label: &str,
) -> Option<Prakriya> {
    debug_assert_eq!(*chars.last().unwrap(), ह्रस्व);

    let kosha = varnavinyas_kosha::kosha();
    if kosha.contains(input) {
        // The ह्रस्व form itself is a valid dictionary word — don't flag it
        return None;
    }

    let mut dirgha_chars: Vec<char> = chars.to_vec();
    *dirgha_chars.last_mut().unwrap() = दीर्घ;
    let dirgha_form: String = dirgha_chars.into_iter().collect();

    if kosha.contains(&dirgha_form) {
        let rule_ref = if दीर्घ == 'ी' {
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

/// Academy 3(क)(इ) rule 1: masculine नातासम्बन्धी terms take ह्रस्व at end.
/// Exceptions: खसी, सम्धी, हात्ती, स्वामी are दीर्घ.
pub fn rule_kinship_tadbhav(input: &str) -> Option<Prakriya> {
    let origin = classify(input);
    if !matches!(origin, Origin::Tadbhav | Origin::Deshaj) {
        return None;
    }

    // Masculine नातासम्बन्धी terms that MUST end in ह्रस्व
    // Academy 3(क)(इ)-1: दाजु, बाबु, भिनाजु, काका, मामा, etc.
    // These are typically: consonant + ु at end (not ू)
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

    // Feminine नातासम्बन्धी terms that MUST end in दीर्घ
    // Academy 3(ई): भाउजू, फुपू, सासू, etc.
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

/// Check if word contains a तत्सम-derived suffix where दीर्घ ई is expected.
fn has_tatsam_suffix(input: &str) -> bool {
    input.ends_with("ीकरण")
        || input.ends_with("ीकृत")
        || input.ends_with("ीकार")
        || input.ends_with("ीय")
        || input.ends_with("ीन")
}

/// Check if the word matches a known feminine दीर्घ ending pattern.
fn is_feminine_dirgha_pattern(input: &str) -> bool {
    // Words ending in ी that are correct feminine forms
    input.ends_with("नी") || input.ends_with("डी") || input.ends_with("ती") || input.ends_with("ली")
}

/// Check if the word is a नातासम्बन्धी term (or suffixed form thereof) that has its own rules.
fn is_नातासम्बन्धी_dirgha_pattern(input: &str) -> bool {
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
    // Check exact match or suffixed forms (ले, मा, को, लाई, etc.)
    for base in KINSHIP_BASES {
        if input == *base || input.starts_with(base) {
            return true;
        }
    }
    false
}
