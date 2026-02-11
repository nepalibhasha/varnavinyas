use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::step::Step;
use varnavinyas_shabda::{Origin, classify};

/// Apply orthographic pattern rules (chandrabindu, sibilant, ri/kri, halanta, ya/e, ksha/chhya).
pub fn apply_orthographic_rules(input: &str) -> Option<Prakriya> {
    // chandrabindu/shirbindu rules
    if let Some(p) = rule_chandrabindu_shirbindu(input) {
        return Some(p);
    }

    // sibilant rules (श/ष/स)
    if let Some(p) = rule_sibilant(input) {
        return Some(p);
    }

    // ऋ/कृ rules
    if let Some(p) = rule_ri_kri(input) {
        return Some(p);
    }

    // halanta rules
    if let Some(p) = rule_halanta(input) {
        return Some(p);
    }

    None
}

/// Academy 3(ख): chandrabindu vs shirbindu rules based on word origin.
/// - Tatsam: NEVER chandrabindu (ँ) → use shirbindu (ं)
/// - Tadbhav/Aagantuk: NEVER shirbindu (ं) → use chandrabindu (ँ) for nasalization
fn rule_chandrabindu_shirbindu(input: &str) -> Option<Prakriya> {
    let origin = classify(input);

    match origin {
        Origin::Tatsam => {
            // Tatsam words should NOT have chandrabindu (ँ) — use shirbindu (ं)
            if input.contains('ँ') {
                let output = input.replace('ँ', "ं");
                return Some(Prakriya::corrected(
                    input,
                    &output,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(ख)"),
                        "तत्सम शब्दमा शिरबिन्दु (ं) प्रयोग हुन्छ, चन्द्रबिन्दु (ँ) होइन",
                        input,
                        &output,
                    )],
                ));
            }
        }
        Origin::Tadbhav | Origin::Deshaj => {
            // Tadbhav/Deshaj words should NOT have shirbindu (ं) for nasalization —
            // use chandrabindu (ँ).
            // BUT: shirbindu is valid in tadbhav when it represents a panchham varna
            // simplification (e.g., संसार is tadbhav but ं is valid before स).
            // So only flag anusvara (ं) when it's NOT before a stop consonant.
            if input.contains('ं') {
                let chars: Vec<char> = input.chars().collect();
                let mut output_chars = chars.clone();
                let mut changed = false;

                for i in 0..chars.len() {
                    if chars[i] == 'ं' {
                        // Check what follows the anusvara
                        let next = chars.get(i + 1).copied();
                        let before_stop = next.is_some_and(is_stop_consonant);

                        // Only replace with chandrabindu if NOT before a stop consonant
                        // (before stops, anusvara is a valid shorthand for panchham)
                        if !before_stop {
                            output_chars[i] = 'ँ';
                            changed = true;
                        }
                    }
                }

                if changed {
                    let output: String = output_chars.into_iter().collect();
                    return Some(Prakriya::corrected(
                        input,
                        &output,
                        vec![Step::new(
                            Rule::VarnaVinyasNiyam("3(ख)"),
                            "तद्भव/देशज शब्दमा चन्द्रबिन्दु (ँ) प्रयोग हुन्छ, शिरबिन्दु (ं) होइन",
                            input,
                            &output,
                        )],
                    ));
                }
            }
        }
        Origin::Aagantuk => {
            // Aagantuk words also use chandrabindu for nasalization
            if input.contains('ं') {
                let chars: Vec<char> = input.chars().collect();
                let mut output_chars = chars.clone();
                let mut changed = false;

                for i in 0..chars.len() {
                    if chars[i] == 'ं' {
                        let next = chars.get(i + 1).copied();
                        let before_stop = next.is_some_and(is_stop_consonant);
                        if !before_stop {
                            output_chars[i] = 'ँ';
                            changed = true;
                        }
                    }
                }

                if changed {
                    let output: String = output_chars.into_iter().collect();
                    return Some(Prakriya::corrected(
                        input,
                        &output,
                        vec![Step::new(
                            Rule::VarnaVinyasNiyam("3(ख)"),
                            "आगन्तुक शब्दमा अनुनासिकमा चन्द्रबिन्दु (ँ) प्रयोग हुन्छ",
                            input,
                            &output,
                        )],
                    ));
                }
            }
        }
    }

    None
}

/// Academy 3(ग)(अ): sibilant rules based on word origin.
/// - Aagantuk: ष→स, श→स (only स is used in foreign words)
/// - Tadbhav: ष→स (retroflex sibilant becomes dental)
/// - Tatsam: preserve original श/ष/स
fn rule_sibilant(input: &str) -> Option<Prakriya> {
    let origin = classify(input);

    match origin {
        Origin::Aagantuk => {
            // Aagantuk words should only use स (dental sibilant)
            // Replace ष and श with स
            let mut output = input.to_string();
            let mut changed = false;

            if output.contains('ष') {
                output = output.replace('ष', "स");
                changed = true;
            }
            // Note: श→स in aagantuk is contextual.
            // Some proper nouns retain श (e.g., एशिया).
            // Only apply ष→स which is more universal.

            if changed {
                return Some(Prakriya::corrected(
                    input,
                    &output,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(ग)(अ)-9"),
                        "आगन्तुक शब्दमा 'स' मात्र प्रयोग: ष→स",
                        input,
                        &output,
                    )],
                ));
            }

            // Also check: ण→न in aagantuk (foreign words don't use retroflex nasal)
            if input.contains('ण') {
                let output = input.replace('ण', "न");
                return Some(Prakriya::corrected(
                    input,
                    &output,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(ग)(अ)-9"),
                        "आगन्तुक शब्दमा 'न' प्रयोग: ण→न",
                        input,
                        &output,
                    )],
                ));
            }
        }
        Origin::Tadbhav | Origin::Deshaj => {
            // Tadbhav words: ष→स (retroflex becomes dental)
            // But tadbhav can legitimately have श (palatal sibilant)
            if input.contains('ष') {
                let output = input.replace('ष', "स");
                return Some(Prakriya::corrected(
                    input,
                    &output,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(ग)(अ)-8"),
                        "तद्भव शब्दमा ष→स: मूर्धन्य ष तद्भवमा हुँदैन",
                        input,
                        &output,
                    )],
                ));
            }
        }
        Origin::Tatsam => {
            // Tatsam preserves original sibilants — no change
        }
    }

    None
}

fn rule_ri_kri(input: &str) -> Option<Prakriya> {
    // Only apply ऋ/कृ rules to words that classify as tatsam.
    // Foreign words like क्रिकेट must not be mutated.
    let origin = classify(input);
    if !matches!(origin, Origin::Tatsam) {
        return None;
    }

    // Pattern: रि at start of tatsam word → ऋ
    if let Some(rest) = input.strip_prefix("रि") {
        // Known patterns: रिषि→ऋषि, रितु→ऋतु
        if rest.starts_with('ष') || rest.starts_with('त') {
            let output = format!("ऋ{rest}");
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(ग)-ऋ"),
                    "tatsam uses ऋ (not रि)",
                    input,
                    &output,
                )],
            ));
        }
    }

    // Pattern: क्रि in tatsam → कृ
    if input.contains("क्रि") {
        let output = input.replace("क्रि", "कृ");
        if output != input {
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(ग)-कृ"),
                    "tatsam uses कृ (not क्रि)",
                    input,
                    &output,
                )],
            ));
        }
    }

    None
}

/// Academy 3(ङ): halanta rules.
/// - Verb roots (धातु), 2nd person disrespect, 3rd person plural → halanta
/// - -मान्/-वान्/-वत् suffix words → halanta
/// - Monosyllabic pronouns/avyaya (म, तँ, र, न) → NO halanta
fn rule_halanta(input: &str) -> Option<Prakriya> {
    let origin = classify(input);
    if !matches!(origin, Origin::Tatsam) {
        return None;
    }

    // Tatsam words ending in specific consonant stems that require halanta
    // These are words where the final consonant needs explicit halanta marker
    static HALANTA_REQUIRED: &[(&str, &str)] = &[
        // -मान् / -वान् / -वत् suffix tatsam words
        ("बुद्धिमान", "बुद्धिमान्"),
        ("श्रीमान", "श्रीमान्"),
        ("भगवान", "भगवान्"),
        ("महान", "महान्"),
        ("विद्वान", "विद्वान्"),
        // Common tatsam avyaya requiring halanta
        ("प्रायः", "प्रायः"), // This one is already correct with visarga
        ("पुनः", "पुनः"),
    ];

    for &(wrong, correct) in HALANTA_REQUIRED {
        if input == wrong && wrong != correct {
            return Some(Prakriya::corrected(
                input,
                correct,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(ङ)"),
                    "तत्सम शब्दमा हलन्त चिह्न आवश्यक",
                    input,
                    correct,
                )],
            ));
        }
    }

    None
}

/// Check if a character is a stop consonant (sparsha vyanjana: ka-ma varga).
fn is_stop_consonant(c: char) -> bool {
    matches!(
        c,
        'क' | 'ख'
            | 'ग'
            | 'घ'
            | 'ङ'
            | 'च'
            | 'छ'
            | 'ज'
            | 'झ'
            | 'ञ'
            | 'ट'
            | 'ठ'
            | 'ड'
            | 'ढ'
            | 'ण'
            | 'त'
            | 'थ'
            | 'द'
            | 'ध'
            | 'न'
            | 'प'
            | 'फ'
            | 'ब'
            | 'भ'
            | 'म'
    )
}
