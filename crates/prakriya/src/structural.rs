use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::step::Step;
use varnavinyas_shabda::{Origin, classify};

pub const SPEC_SHRI: RuleSpec = RuleSpec {
    id: "struct-shri",
    category: RuleCategory::Structural,
    kind: DiagnosticKind::Error,
    priority: 100,
    citation: Rule::ShuddhaAshuddha("Section 4"),
    examples: &[("श्रृङ्गार", "शृङ्गार")],
};

pub const SPEC_REDUNDANT_SUFFIX: RuleSpec = RuleSpec {
    id: "struct-redundant-suffix",
    category: RuleCategory::Structural,
    kind: DiagnosticKind::Error,
    priority: 110,
    citation: Rule::ShuddhaAshuddha("Section 4"),
    examples: &[("सौन्दर्यता", "सौन्दर्य"), ("औचित्यता", "औचित्य")],
};

pub const SPEC_PANCHHAM: RuleSpec = RuleSpec {
    id: "struct-panchham",
    category: RuleCategory::Structural,
    kind: DiagnosticKind::Error,
    priority: 120,
    citation: Rule::VarnaVinyasNiyam("3(ख)-पञ्चम"),
    examples: &[("संकेत", "सङ्केत"), ("संघीय", "सङ्घीय")],
};

pub fn rule_shri_correction(input: &str) -> Option<Prakriya> {
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

pub fn rule_redundant_suffix(input: &str) -> Option<Prakriya> {
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

/// Academy 3(ख)(अ): panchham varna rules for tatsam words.
/// In tatsam words, anusvara (ं) before stop consonants → panchham varna:
/// - Before क/ख/ग/घ/क्ष → ङ् (e.g., संकेत→सङ्केत)
/// - Before च/छ/ज/झ → ञ् (e.g., संचार→सञ्चार)
/// - Before ट/ठ/ड/ढ/ण → ण् (e.g., कंटक→कण्टक)
/// - Before त/थ/द/ध/न/त्र → न् (e.g., संतोष→सन्तोष)
/// - Before प/फ/ब/भ/म → म् (e.g., संपन्न→सम्पन्न)
pub fn rule_panchham_varna(input: &str) -> Option<Prakriya> {
    let origin = classify(input);

    // Only for tatsam words (tadbhav/aagantuk write as-pronounced)
    if !matches!(origin, Origin::Tatsam) {
        return None;
    }

    if !input.contains('ं') {
        return None;
    }

    let chars: Vec<char> = input.chars().collect();
    let mut result = String::with_capacity(input.len() + 8);
    let mut changed = false;
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == 'ं' {
            if let Some(&next) = chars.get(i + 1) {
                if let Some(panchham) = get_panchham_for(next) {
                    // Replace ं with panchham varna + halanta
                    result.push(panchham);
                    result.push('्');
                    changed = true;
                    i += 1;
                    continue;
                }
            }
        }
        result.push(chars[i]);
        i += 1;
    }

    if changed {
        return Some(Prakriya::corrected(
            input,
            &result,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(ख)-पञ्चम"),
                "तत्सम शब्दमा स्पर्श व्यञ्जन अघि पञ्चम वर्ण प्रयोग",
                input,
                &result,
            )],
        ));
    }

    None
}

/// Get the panchham varna (fifth consonant) for a given stop consonant.
fn get_panchham_for(c: char) -> Option<char> {
    match c {
        'क' | 'ख' | 'ग' | 'घ' => Some('ङ'),
        'च' | 'छ' | 'ज' | 'झ' => Some('ञ'),
        'ट' | 'ठ' | 'ड' | 'ढ' => Some('ण'),
        'त' | 'थ' | 'द' | 'ध' | 'न' => Some('न'),
        'प' | 'फ' | 'ब' | 'भ' | 'म' => Some('म'),
        _ => None,
    }
}
