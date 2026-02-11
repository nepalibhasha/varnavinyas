use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::step::Step;
use varnavinyas_shabda::{Origin, classify};

/// Apply hrasva/dirgha pattern rules.
/// Returns Some(Prakriya) if a rule fired, None otherwise.
pub fn apply_hrasva_dirgha_rules(input: &str) -> Option<Prakriya> {
    // Rule: suffix -नु triggers hrasva on root vowel
    if input.ends_with("नु") || input.ends_with("र्नु") {
        if let Some(p) = rule_suffix_nu_hrasva(input) {
            return Some(p);
        }
    }

    // Rule: suffix -एली triggers hrasva on root vowel
    if input.ends_with("एली") || input.ends_with("ेली") {
        if let Some(p) = rule_suffix_eli_hrasva(input) {
            return Some(p);
        }
    }

    // Rule: suffix -ई preserves dirgha
    // Rule: suffix -ईय preserves dirgha
    // (These fix hrasva→dirgha, the reverse direction)
    if let Some(p) = rule_suffix_preserves_dirgha(input) {
        return Some(p);
    }

    // Rule: tadbhav single-meaning word takes hrasva
    if let Some(p) = rule_tadbhav_hrasva(input) {
        return Some(p);
    }

    // Rule: feminine/pronoun/postposition/absolutive takes dirgha
    if let Some(p) = rule_dirgha_endings(input) {
        return Some(p);
    }

    // Rule: kinship tadbhav patterns
    if let Some(p) = rule_kinship_tadbhav(input) {
        return Some(p);
    }

    None
}

fn rule_suffix_nu_hrasva(input: &str) -> Option<Prakriya> {
    // स्वीकार्नु → स्विकार्नु
    // Only replace the LAST dirgha ई before the suffix, not all occurrences.
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
                "suffix -नु triggers hrasva on root vowel",
                input,
                &output,
            )],
        ));
    }
    None
}

fn rule_suffix_eli_hrasva(input: &str) -> Option<Prakriya> {
    // पूर्वेली → पुर्वेली
    // Only replace the LAST dirgha ू before the suffix, not all occurrences.
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
                "suffix -एली triggers hrasva on root vowel",
                input,
                &output,
            )],
        ));
    }
    None
}

/// Academy 3(क)(उ) rules 1-2: suffixes -ई/-ईय preserve dirgha in the stem.
/// Only fires for specific known stem patterns to avoid false positives.
fn rule_suffix_preserves_dirgha(input: &str) -> Option<Prakriya> {
    // Known incorrect→correct pairs where a dirgha-suffix stem lost its dirgha.
    // We only correct specific known patterns to avoid false positives.
    static KNOWN_CORRECTIONS: &[(&str, &str, &str)] = &[
        // (incorrect, correct, rule description)
        ("पुर्वी", "पूर्वी", "suffix -ई preserves dirgha: पूर्व + ई = पूर्वी"),
        (
            "पुर्वीय",
            "पूर्वीय",
            "suffix -ईय preserves dirgha: पूर्व + ईय = पूर्वीय",
        ),
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

/// Academy 3(क) rules 3-12: tadbhav/deshaj/aagantuk words take hrasva.
/// If a non-tatsam word has dirgha ई/ऊ where hrasva is expected, correct it.
fn rule_tadbhav_hrasva(input: &str) -> Option<Prakriya> {
    let origin = classify(input);

    // Only apply to non-tatsam words
    if matches!(origin, Origin::Tatsam) {
        return None;
    }

    // Don't interfere with known dirgha patterns (feminine, kinship, tatsam suffixes, etc.)
    // Those are handled by rule_dirgha_endings and rule_kinship_tadbhav
    if is_feminine_dirgha_pattern(input)
        || is_kinship_dirgha_pattern(input)
        || has_tatsam_suffix(input)
    {
        return None;
    }

    // Tadbhav/Deshaj: word-initial and word-medial dirgha ई→इ, ऊ→उ
    // (not word-final, which has separate rules)
    let chars: Vec<char> = input.chars().collect();
    if chars.len() < 2 {
        return None;
    }

    let mut changed = false;
    let mut output_chars = chars.clone();

    // Check medial positions (not final) for unexpected dirgha
    // Final position has its own rules (dirgha for feminine, etc.)
    for i in 0..chars.len().saturating_sub(1) {
        match chars[i] {
            'ी' => {
                // Medial ई→इ in non-tatsam words
                output_chars[i] = 'ि';
                changed = true;
            }
            'ू' => {
                // Medial ऊ→उ in non-tatsam words
                output_chars[i] = 'ु';
                changed = true;
            }
            _ => {}
        }
    }

    if changed {
        let output: String = output_chars.into_iter().collect();
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

/// Academy 3(क)(ऊ) rules 1-16: feminine nouns, -ई/-वती suffixes, profession/place names → dirgha.
fn rule_dirgha_endings(input: &str) -> Option<Prakriya> {
    let origin = classify(input);

    // This rule primarily applies to tadbhav/deshaj feminine endings
    if matches!(origin, Origin::Tatsam) {
        return None;
    }

    let chars: Vec<char> = input.chars().collect();
    if chars.is_empty() {
        return None;
    }

    let last = *chars.last().unwrap();

    // Common feminine/adjectival/postposition suffixes that require dirgha ई at end
    static DIRGHA_II_ENDINGS: &[&str] = &[
        "नी",   // feminine suffix: बहिनी, सम्धिनी
        "डी",   // demonym/adjective: पहाडी
        "सानी", // feminine: खुर्सानी
    ];

    // Postpositions and absolutives that require dirgha ई
    static DIRGHA_II_WORDS: &[&str] = &["अगाडी", "पछाडी", "माथी", "तली"];

    // Check if word ends in hrasva इ where dirgha ई is required
    if last == 'ि' {
        // Absolutive (पूर्वकालिक क्रिया): verb forms ending in -ि should be -ी
        // e.g., भनि→भनी, गरि→गरी
        // Only for short verb-like forms (2-4 chars)
        let char_count = chars.len();
        if (2..=4).contains(&char_count) {
            // Check if it looks like an absolutive verb form
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
                        "पूर्वकालिक क्रिया (absolutive) takes dirgha ई",
                        input,
                        &output,
                    )],
                ));
            }
        }

        // Feminine/demonym/postposition endings
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

        // Check specific postposition/adverb words
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
    }

    // Check for hrasva उ at word-final where dirgha ऊ is required
    if last == 'ु' {
        // Feminine kinship terms ending in ू: handled by rule_kinship_tadbhav
        // Pronoun हामू → handled elsewhere
        // Plural suffix हरू: handled by correction table
    }

    None
}

/// Academy 3(क)(इ) rule 1: masculine kinship terms take hrasva at end.
/// Exceptions: खसी, सम्धी, हात्ती, स्वामी are dirgha.
fn rule_kinship_tadbhav(input: &str) -> Option<Prakriya> {
    let origin = classify(input);
    if !matches!(origin, Origin::Tadbhav | Origin::Deshaj) {
        return None;
    }

    // Masculine kinship terms that MUST end in hrasva
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

    // Feminine kinship terms that MUST end in dirgha
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

/// Check if word contains a tatsam-derived suffix where dirgha ई is expected.
fn has_tatsam_suffix(input: &str) -> bool {
    input.ends_with("ीकरण")
        || input.ends_with("ीकृत")
        || input.ends_with("ीकार")
        || input.ends_with("ीय")
        || input.ends_with("ीन")
}

/// Check if the word matches a known feminine dirgha ending pattern.
fn is_feminine_dirgha_pattern(input: &str) -> bool {
    // Words ending in ी that are correct feminine forms
    input.ends_with("नी") || input.ends_with("डी") || input.ends_with("ती") || input.ends_with("ली")
}

/// Check if the word is a kinship term (or suffixed form thereof) that has its own rules.
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
    // Check exact match or suffixed forms (ले, मा, को, लाई, etc.)
    for base in KINSHIP_BASES {
        if input == *base || input.starts_with(base) {
            return true;
        }
    }
    false
}
