use crate::model::prakriya::Prakriya;
use crate::model::rule::Rule;
use crate::model::step::Step;
use varnavinyas_akshar::is_matra;
use varnavinyas_kosha::{Kosha, kosha};

const PS_SANSKRIT_VA_TO_BA_RULE_CODE: &str = "3(ग)(आ)-PS-Saisanik-4(घ)-ब";
const PS_SANSKRIT_VA_TO_BA_EXACT_OVERRIDES: &[(&str, &str)] = &[
    ("विन्दु", "बिन्दु"),
    ("विम्ब", "बिम्ब"),
    ("वेला", "बेला"),
    ("कुवेला", "कुबेला"),
    ("सुवेला", "सुबेला"),
    ("वार", "बार"),
    ("आइतवार", "आइतबार"),
    ("बुधवार", "बुधबार"),
    ("विना", "बिना"),
];
const PS_SANSKRIT_VA_TO_BA_SUPPORTED_REPLACEMENTS: &[(&str, &str)] =
    &[("विन्दु", "बिन्दु"), ("विम्ब", "बिम्ब"), ("विना", "बिना")];

// Academy 3(ग)(आ): single-position ब/व swap with kosha validation.
// -----------------------------------------------------------------------------
// 3(ग)(आ) 'ब', 'व' र 'ओ' को प्रयोग
// -----------------------------------------------------------------------------
pub fn rule_ba_va(input: &str) -> Option<Prakriya> {
    if input.is_empty() {
        return None;
    }
    let kosha = kosha();
    if let Some(output) = normalize_ps_sanskrit_va_to_ba(input, kosha) {
        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam(PS_SANSKRIT_VA_TO_BA_RULE_CODE),
                "शैक्षणिक व्याकरण ४(घ): संस्कृत व् भएका केही शब्द नेपालीमा ब लेखिन्छ",
                input,
                &output,
            )],
        ));
    }
    if kosha.contains(input) {
        return None;
    }
    let chars: Vec<char> = input.chars().collect();

    // (आ)-'ओ' उपशीर्षक: minimally scoped, attested normalization.
    // Subrule 1/3 style: initial ओ class and tatsam ओ words.
    if input.starts_with('औ') {
        let output = input.replacen('औ', "ओ", 1);
        if kosha.contains(&output) {
            let citation = if output.starts_with("ओज") || output.starts_with("ओम्") {
                "3(ग)(आ)-ओ-3"
            } else {
                "3(ग)(आ)-ओ-1"
            };
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam(citation),
                    "'ओ' उच्चारण हुने शब्दमा ओ-रूप प्रयोग हुन्छ",
                    input,
                    &output,
                )],
            ));
        }
    }
    if input.starts_with('उ') {
        let output = input.replacen('उ', "ओ", 1);
        if kosha.contains(&output) && !is_supported_u_initial_e_verb_form(input, kosha) {
            let citation = if output.starts_with("ओज") || output.starts_with("ओम्") {
                "3(ग)(आ)-ओ-3"
            } else {
                "3(ग)(आ)-ओ-1"
            };
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam(citation),
                    "'ओ' उच्चारण हुने शब्दमा ओ-रूप प्रयोग हुन्छ",
                    input,
                    &output,
                )],
            ));
        }
    }
    // Subrule 2 style (क्रियापद): जाऊ/खाऊ-type -> जाओ/खाओ orthography.
    // Keep a deterministic notice list first, then fallback to attested transform.
    const O_VERB_FIXED: &[(&str, &str)] = &[
        ("जाऊ", "जाओ"),
        ("खाऊ", "खाओ"),
        ("गाऊ", "गाओ"),
        ("बनाऊस्", "बनाओस्"),
        ("देऊस्", "देओस्"),
    ];
    for &(wrong, correct) in O_VERB_FIXED {
        if input == wrong {
            return Some(Prakriya::corrected(
                input,
                correct,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(ग)(आ)-ओ-2"),
                    "ओ-श्रेणीका क्रियापदमा ओ-लेखन हुन्छ",
                    input,
                    correct,
                )],
            ));
        }
    }
    if input.contains("ाउ") {
        let output = input.replacen("ाउ", "ाओ", 1);
        if kosha.contains(&output) {
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(ग)(आ)-ओ-2"),
                    "ओ-श्रेणीका क्रियापदमा ओ-लेखन हुन्छ",
                    input,
                    &output,
                )],
            ));
        }
    }

    for i in 0..chars.len() {
        let (swapped, from) = match chars[i] {
            'ब' => ('व', 'ब'),
            'व' => ('ब', 'व'),
            _ => continue,
        };
        let mut candidate = chars.clone();
        candidate[i] = swapped;
        let output: String = candidate.into_iter().collect();
        if kosha.contains(&output) {
            // (आ)-ब/व numbered subrules are largely lexical buckets.
            // We keep this as an attested candidate swap and classify by local context.
            let next_base = chars.iter().skip(i + 1).find(|&&c| !is_matra(c)).copied();
            let citation = if from == 'व' && matches!(next_base, Some('द' | 'ध' | 'ल' | 'ह' | 'म'))
            {
                // 'ब' को प्रयोग, subrule 1
                "3(ग)(आ)-ब-1"
            } else if from == 'व' && i > 0 && chars[i - 1] == 'म' {
                // 'ब' को प्रयोग, subrule 2
                "3(ग)(आ)-ब-2"
            } else if from == 'व'
                && i == 0
                && (output.starts_with("बे") || output.starts_with("बद") || output.starts_with("बि"))
            {
                // 'ब' को प्रयोग, subrule 7
                "3(ग)(आ)-ब-7"
            } else if from == 'व'
                && (output.ends_with("ुवा")
                    || output.ends_with("ेर्नु")
                    || output.ends_with("ाङ्गो")
                    || output.ends_with("ुढो")
                    || output.ends_with("िटुलो")
                    || output.ends_with("ुच्चो")
                    || output.ends_with("ौलाहा")
                    || output.ends_with("लियो"))
            {
                // 'ब' को प्रयोग, subrule 4
                "3(ग)(आ)-ब-4"
            } else if from == 'व'
                && matches!(
                    output.as_str(),
                    "अब" | "तब" | "जब" | "बजे" | "बरु" | "बर्र" | "बेर" | "ब्यारे"
                )
            {
                // 'ब' को प्रयोग, subrule 5
                "3(ग)(आ)-ब-5"
            } else if from == 'व'
                && (output.ends_with("ग्नु")
                    || output.ends_with("र्चनु")
                    || output.ends_with("स्नु")
                    || output.ends_with("ेर्नु")
                    || output.ends_with("ँच्नु")
                    || output.ends_with("िर्सनु"))
            {
                // 'ब' को प्रयोग, subrule 6
                "3(ग)(आ)-ब-6"
            } else if from == 'ब' && i == 0 && input.starts_with("बि") {
                // 'व' को प्रयोग, subrule 1 (वि-उपसर्ग)
                "3(ग)(आ)-व-1"
            } else if from == 'ब'
                && ((output.contains("वै") || output.contains('ृ'))
                    || output.contains("र्ष")
                    || output.contains("र्ग")
                    || output.contains("र्ण")
                    || output.contains("वृक्ष")
                    || output.contains("वृष्टि")
                    || output.contains("वृद्धि"))
            {
                // 'व' को प्रयोग, subrule 2
                "3(ग)(आ)-व-2"
            } else if from == 'ब'
                && (output.ends_with("वर")
                    || output.contains("तव्य")
                    || output.contains("त्व")
                    || output.contains("वत")
                    || output.contains("वान")
                    || output.contains("वती"))
            {
                // 'व' को प्रयोग, subrule 3 (वत्-प्रत्यय समूह)
                "3(ग)(आ)-व-3"
            } else if from == 'ब' && output.starts_with("संव") {
                // 'व' को प्रयोग, subrule 4
                "3(ग)(आ)-व-4"
            } else if from == 'ब' && output.ends_with('व') {
                // 'व' को प्रयोग, subrule 5
                "3(ग)(आ)-व-5"
            } else if from == 'ब'
                && (output.ends_with("ावट")
                    || output.ends_with("ुवा")
                    || output.ends_with("वाला")
                    || output.ends_with("वार")
                    || output.ends_with("वारी"))
            {
                // 'व' को प्रयोग, subrule 10
                "3(ग)(आ)-व-10"
            } else if from == 'ब'
                && matches!(
                    output.as_str(),
                    "वर" | "वरिपरि"
                        | "वारपार"
                        | "वाल्ल"
                        | "प्वाक्क"
                        | "ट्वाक्क"
                        | "ह्वात्त"
                        | "छ्वाल्ल"
                )
            {
                // 'व' को प्रयोग, subrule 9
                "3(ग)(आ)-व-9"
            } else if from == 'ब'
                && (output.ends_with("वाउनु")
                    || output.ends_with("वायो")
                    || output.ends_with("वाउँछ"))
            {
                // 'व' को प्रयोग, subrule 8
                "3(ग)(आ)-व-8"
            } else if from == 'ब'
                && (output.ends_with("ुवा")
                    || output.ends_with("ाडे")
                    || output.ends_with("ादार")
                    || output.ends_with("ोलवाला"))
            {
                // 'व' को प्रयोग, subrule 7
                "3(ग)(आ)-व-7"
            } else if from == 'ब' {
                // lexical buckets (subrule 6..9)
                "3(ग)(आ)-व-6"
            } else {
                // lexical buckets (subrule 3..6)
                "3(ग)(आ)-ब-3"
            };
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam(citation),
                    "ब/व भेद: सन्दर्भअनुसार ब वा व प्रयोग हुन्छ",
                    input,
                    &output,
                )],
            ));
        }
    }
    None
}

fn normalize_ps_sanskrit_va_to_ba(input: &str, kosha: &Kosha) -> Option<String> {
    for &(wrong, right) in PS_SANSKRIT_VA_TO_BA_EXACT_OVERRIDES {
        if input == wrong {
            return Some(right.to_string());
        }
    }

    for &(wrong, right) in PS_SANSKRIT_VA_TO_BA_SUPPORTED_REPLACEMENTS {
        if !input.contains(wrong) {
            continue;
        }
        let output = input.replacen(wrong, right, 1);
        if supported_form(&output, kosha) {
            return Some(output);
        }
    }

    if let Some(stem) = input.strip_suffix("वार") {
        let output = format!("{stem}बार");
        if supported_form(&output, kosha) {
            return Some(output);
        }
    }

    None
}

fn supported_form(output: &str, kosha: &Kosha) -> bool {
    kosha.contains(output) || kosha.lookup(output).is_some()
}

fn is_supported_u_initial_e_verb_form(input: &str, kosha: &Kosha) -> bool {
    let Some(stem) = input.strip_suffix("े") else {
        return false;
    };
    if !stem.starts_with('उ') || stem.chars().count() < 2 {
        return false;
    }

    [format!("{stem}्नु"), format!("{stem}नु")]
        .iter()
        .any(|candidate| supported_form(candidate, kosha))
}
