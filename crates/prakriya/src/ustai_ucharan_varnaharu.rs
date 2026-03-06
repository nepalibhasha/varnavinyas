use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::step::Step;
use varnavinyas_kosha::kosha;
use varnavinyas_shabda::{Origin, classify};

pub const SPEC_SIBILANT: RuleSpec = RuleSpec {
    id: "ortho-sibilant",
    category: RuleCategory::ShaShaS,
    kind: DiagnosticKind::Error,
    priority: 310,
    citation: Rule::VarnaVinyasNiyam("3(ग)(अ)"),
    examples: &[("रजिष्टर", "रजिस्टर")],
};

pub const SPEC_BA_VA: RuleSpec = RuleSpec {
    id: "ortho-ba-va",
    category: RuleCategory::ShaShaS,
    kind: DiagnosticKind::Error,
    priority: 315,
    citation: Rule::VarnaVinyasNiyam("3(ग)(आ)"),
    examples: &[("बिदेश", "विदेश"), ("बिज्ञान", "विज्ञान")],
};

pub const SPEC_RI_KRI: RuleSpec = RuleSpec {
    id: "ortho-ri-kri",
    category: RuleCategory::RiKri,
    kind: DiagnosticKind::Error,
    priority: 320,
    citation: Rule::VarnaVinyasNiyam("3(ग)-ऋ"),
    examples: &[("रिषि", "ऋषि"), ("क्रिति", "कृति")],
};

pub const SPEC_YA_E: RuleSpec = RuleSpec {
    id: "ortho-ya-e",
    category: RuleCategory::YaE,
    kind: DiagnosticKind::Error,
    priority: 350,
    citation: Rule::VarnaVinyasNiyam("3(इ)"),
    examples: &[("एथार्थ", "यथार्थ"), ("यकता", "एकता")],
};

pub const SPEC_KSHA_CHHYA: RuleSpec = RuleSpec {
    id: "ortho-ksha-chhya",
    category: RuleCategory::KshaChhya,
    kind: DiagnosticKind::Error,
    priority: 360,
    citation: Rule::VarnaVinyasNiyam("3(उ)"),
    examples: &[("लछ्य", "लक्ष्य"), ("छेत्र", "क्षेत्र")],
};

pub const SPEC_GYA_GYAN: RuleSpec = RuleSpec {
    id: "ortho-gya-gyan",
    category: RuleCategory::GyaGyan,
    kind: DiagnosticKind::Error,
    priority: 365,
    citation: Rule::VarnaVinyasNiyam("3(ग)(ऊ)"),
    examples: &[("अग्यान", "अज्ञान"), ("प्रग्या", "प्रज्ञा")],
};

pub fn rule_sibilant(input: &str) -> Option<Prakriya> {
    let origin = classify(input);
    match origin {
        Origin::Aagantuk => {
            let mut output = input.to_string();
            let mut changed = false;
            if output.contains('ष') {
                output = output.replace('ष', "स");
                changed = true;
            }
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
            if input.contains('ष') {
                let output = input.replace('ष', "स");
                let lex = kosha();
                if lex.contains(input) && !lex.contains(&output) {
                    return None;
                }
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
        Origin::Tatsam => {}
    }
    None
}

pub fn rule_ri_kri(input: &str) -> Option<Prakriya> {
    let origin = classify(input);
    if !matches!(origin, Origin::Tatsam) {
        return None;
    }
    if let Some(rest) = input.strip_prefix("रि") {
        if rest.starts_with('ष') || rest.starts_with('त') {
            let output = format!("ऋ{rest}");
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(ग)-ऋ"),
                    "तत्सम शब्दमा ऋ हुन्छ (रि होइन)",
                    input,
                    &output,
                )],
            ));
        }
    }
    if input.contains("क्रि") {
        let output = input.replace("क्रि", "कृ");
        if output != input {
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(ग)-कृ"),
                    "तत्सम शब्दमा कृ हुन्छ (क्रि होइन)",
                    input,
                    &output,
                )],
            ));
        }
    }
    None
}

pub fn rule_ba_va(input: &str) -> Option<Prakriya> {
    if input.is_empty() {
        return None;
    }
    let kosha = kosha();
    if kosha.contains(input) {
        return None;
    }
    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len() {
        let swapped = match chars[i] {
            'ब' => 'व',
            'व' => 'ब',
            _ => continue,
        };
        let mut candidate = chars.clone();
        candidate[i] = swapped;
        let output: String = candidate.into_iter().collect();
        if kosha.contains(&output) {
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(ग)(आ)"),
                    "ब/व भेद: तत्सम रूपमा व प्रयोग हुन्छ",
                    input,
                    &output,
                )],
            ));
        }
    }
    None
}

pub fn rule_ya_e(input: &str) -> Option<Prakriya> {
    let chars: Vec<char> = input.chars().collect();
    if chars.is_empty() {
        return None;
    }
    let swap_char = match chars[0] {
        'ए' => 'य',
        'य' => 'ए',
        _ => return None,
    };
    let kosha = kosha();
    if kosha.contains(input) {
        return None;
    }
    let mut swapped = chars;
    swapped[0] = swap_char;
    let candidate: String = swapped.into_iter().collect();
    if kosha.contains(&candidate) {
        return Some(Prakriya::corrected(
            input,
            &candidate,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(इ)"),
                "ए/य भेद: शब्दादिमा ए र य फरक हुन्छ",
                input,
                &candidate,
            )],
        ));
    }
    None
}

pub fn rule_ksha_chhya(input: &str) -> Option<Prakriya> {
    let kosha = kosha();
    if kosha.contains(input) {
        return None;
    }
    if !input.contains("क्ष") && !input.contains('छ') && !input.contains("च्छ") {
        return None;
    }
    const SUBS: &[(&str, &str)] = &[
        ("छ्य", "क्ष्य"),
        ("क्ष्य", "छ्य"),
        ("छे", "क्षे"),
        ("क्षे", "छे"),
        ("क्ष", "च्छ"),
        ("च्छ", "क्ष"),
        ("छ", "क्ष"),
        ("क्ष", "छ"),
    ];
    for &(from, to) in SUBS {
        if input.contains(from) {
            let candidate = input.replace(from, to);
            if kosha.contains(&candidate) {
                return Some(Prakriya::corrected(
                    input,
                    &candidate,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(उ)"),
                        format!("क्ष/छ भेद: {} → {}", from, to),
                        input,
                        &candidate,
                    )],
                ));
            }
        }
    }
    None
}

pub fn rule_gya_gyan(input: &str) -> Option<Prakriya> {
    let kosha = kosha();
    if kosha.contains(input) {
        return None;
    }
    if !input.contains("ज्ञ") && !input.contains("ग्या") && !input.contains("ग्याँ")
    {
        return None;
    }
    const SUBS: &[(&str, &str)] = &[("ग्याँ", "ज्ञा"), ("ग्या", "ज्ञा")];
    for &(from, to) in SUBS {
        if input.contains(from) {
            let candidate = input.replace(from, to);
            if candidate != input && kosha.contains(&candidate) {
                return Some(Prakriya::corrected(
                    input,
                    &candidate,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(ग)(ऊ)"),
                        format!("ज्ञ/ग्याँ/ग्या भेद: {} → {}", from, to),
                        input,
                        &candidate,
                    )],
                ));
            }
        }
    }
    None
}
