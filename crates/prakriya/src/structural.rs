use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::step::Step;

/// Apply structural pattern rules (panchham, gemination, redundant suffix, loanword, श्रृ→शृ).
pub fn apply_structural_rules(input: &str) -> Option<Prakriya> {
    // श्रृ → शृ correction
    if let Some(p) = rule_shri_to_shri(input) {
        return Some(p);
    }

    // Redundant -ता suffix
    if let Some(p) = rule_redundant_ta(input) {
        return Some(p);
    }

    // Panchham varna rules (ं → ङ/ञ/ण/न/म before specific consonants)
    if let Some(p) = rule_panchham_varna(input) {
        return Some(p);
    }

    None
}

fn rule_shri_to_shri(input: &str) -> Option<Prakriya> {
    // श्रृ → शृ (common error pattern)
    if input.contains("श्रृ") {
        let output = input.replace("श्रृ", "शृ");
        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::ShuddhaAshuddha("Section 4"),
                "शृ not श्रृ: श + ृ = शृ (no र involved)",
                input,
                &output,
            )],
        ));
    }
    None
}

fn rule_redundant_ta(input: &str) -> Option<Prakriya> {
    // Words ending in -र्यता or -त्यता → remove -ता
    // e.g., सौन्दर्यता → सौन्दर्य, औचित्यता → औचित्य
    if input.ends_with("र्यता") || input.ends_with("त्यता") || input.ends_with("थ्यता")
    {
        let output = input.strip_suffix("ता").unwrap();
        return Some(Prakriya::corrected(
            input,
            output,
            vec![Step::new(
                Rule::ShuddhaAshuddha("Section 4"),
                "redundant -ता: abstract noun already complete",
                input,
                output,
            )],
        ));
    }
    None
}

fn rule_panchham_varna(_input: &str) -> Option<Prakriya> {
    // ं before क/ख/ग/घ → ङ्
    // ं before च/छ/ज/झ → ञ्
    // etc.
    // This is complex and handled by correction table for specific words.
    None
}
