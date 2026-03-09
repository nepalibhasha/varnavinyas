use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::step::Step;
use varnavinyas_kosha::kosha;
use varnavinyas_shabda::{Origin, classify};

fn corrected_if_attested(
    input: &str,
    output: String,
    citation: Rule,
    explanation: &'static str,
) -> Option<Prakriya> {
    if output == input {
        return None;
    }
    let lex = kosha();
    if !lex.contains(&output) {
        return None;
    }
    Some(Prakriya::corrected(
        input,
        &output,
        vec![Step::new(citation, explanation, input, &output)],
    ))
}

// -----------------------------------------------------------------------------
// 3(ग)(अ) 'श, ष, स' को प्रयोग
// -----------------------------------------------------------------------------
pub fn rule_sibilant(input: &str) -> Option<Prakriya> {
    let origin = classify(input);
    let lex = kosha();
    match origin {
        Origin::Aagantuk => {
            // (अ)-'स' प्रयोग, subrule 9: आगन्तुकमा स-प्राथमिकता.
            if input.contains('ष') {
                let output = input.replace('ष', "स");
                if let Some(p) = corrected_if_attested(
                    input,
                    output,
                    Rule::VarnaVinyasNiyam("3(ग)(अ)-9"),
                    "आगन्तुक शब्दमा 'स' प्राथमिक: ष→स",
                ) {
                    return Some(p);
                }
            }
            if input.contains('श') {
                let output = input.replace('श', "स");
                if let Some(p) = corrected_if_attested(
                    input,
                    output,
                    Rule::VarnaVinyasNiyam("3(ग)(अ)-9"),
                    "आगन्तुक शब्दमा 'स' प्राथमिक: श→स",
                ) {
                    return Some(p);
                }
            }
        }
        Origin::Tadbhav | Origin::Deshaj => {
            // (अ)-'स' प्रयोग, subrule 8: तत्सम→तद्भव मार्गमा श/ष -> स.
            if input.contains('ष') {
                let output = input.replace('ष', "स");
                if !lex.contains(input) && lex.contains(&output) {
                    return Some(Prakriya::corrected(
                        input,
                        &output,
                        vec![Step::new(
                            Rule::VarnaVinyasNiyam("3(ग)(अ)-8"),
                            "तद्भव/देशज रूपान्तरण: ष→स",
                            input,
                            &output,
                        )],
                    ));
                }
            }
            if input.contains('श') {
                let output = input.replace('श', "स");
                if !lex.contains(input) && lex.contains(&output) {
                    return Some(Prakriya::corrected(
                        input,
                        &output,
                        vec![Step::new(
                            Rule::VarnaVinyasNiyam("3(ग)(अ)-8"),
                            "तद्भव/देशज रूपान्तरण: श→स",
                            input,
                            &output,
                        )],
                    ));
                }
            }
        }
        Origin::Tatsam => {
            // (अ) 'श' उपशीर्षक subrule 1:
            // दुई सिबिलन्ट सँगै आएमा अगाडिको श.
            let chars: Vec<char> = input.chars().collect();
            if chars.len() >= 2 {
                for i in 0..(chars.len() - 1) {
                    let a = chars[i];
                    let b = chars[i + 1];
                    let is_sibilant = |c: char| matches!(c, 'श' | 'ष' | 'स');
                    if is_sibilant(a) && is_sibilant(b) && a != 'श' {
                        let mut candidate = chars.clone();
                        candidate[i] = 'श';
                        let output: String = candidate.into_iter().collect();
                        if let Some(p) = corrected_if_attested(
                            input,
                            output,
                            Rule::VarnaVinyasNiyam("3(ग)(अ)-श-1"),
                            "दुई सिबिलन्ट सँगै आएमा अगाडिको 'श' हुन्छ",
                        ) {
                            return Some(p);
                        }
                    }
                }
            }

            // (अ) 'श' उपशीर्षक subrule 2:
            // चवर्ग/ल अगाडि श.
            if chars.len() >= 2 {
                for i in 0..(chars.len() - 1) {
                    if matches!(chars[i], 'ष' | 'स')
                        && matches!(chars[i + 1], 'च' | 'छ' | 'ज' | 'झ' | 'ञ' | 'ल')
                    {
                        let mut candidate = chars.clone();
                        candidate[i] = 'श';
                        let output: String = candidate.into_iter().collect();
                        if let Some(p) = corrected_if_attested(
                            input,
                            output,
                            Rule::VarnaVinyasNiyam("3(ग)(अ)-श-2"),
                            "चवर्ग/ल अगाडि 'श' प्रयोग हुन्छ",
                        ) {
                            return Some(p);
                        }
                    }
                }
            }

            // (अ) 'श' उपशीर्षक subrule 3:
            // ऋ/र सन्दर्भमा श (जस्तै श्र, शृ...).
            if input.contains("स्र")
                || input.contains("ष्र")
                || input.contains("सृ")
                || input.contains("षृ")
            {
                let output = input
                    .replace("स्र", "श्र")
                    .replace("ष्र", "श्र")
                    .replace("सृ", "शृ")
                    .replace("षृ", "शृ");
                if let Some(p) = corrected_if_attested(
                    input,
                    output,
                    Rule::VarnaVinyasNiyam("3(ग)(अ)-श-3"),
                    "ऋ/र सन्दर्भमा 'श' प्रयोग हुन्छ",
                ) {
                    return Some(p);
                }
            }

            // (अ) 'श' उपशीर्षक subrule 4:
            // विसर्गका अगाडि श.
            if input.contains("सः") || input.contains("षः") {
                let output = input.replace("सः", "शः").replace("षः", "शः");
                if let Some(p) = corrected_if_attested(
                    input,
                    output,
                    Rule::VarnaVinyasNiyam("3(ग)(अ)-श-4"),
                    "विसर्ग अगाडि 'श' प्रयोग हुन्छ",
                ) {
                    return Some(p);
                }
            }

            // (अ) 'ष' उपशीर्षक subrule 1:
            // ट/ठ/ड/ढ/ण/पका अगाडि ष.
            if chars.len() >= 2 {
                for i in 0..(chars.len() - 1) {
                    if matches!(chars[i], 'श' | 'स')
                        && matches!(chars[i + 1], 'ट' | 'ठ' | 'ड' | 'ढ' | 'ण' | 'प')
                    {
                        let mut candidate = chars.clone();
                        candidate[i] = 'ष';
                        let output: String = candidate.into_iter().collect();
                        if let Some(p) = corrected_if_attested(
                            input,
                            output,
                            Rule::VarnaVinyasNiyam("3(ग)(अ)-ष-1"),
                            "ट/ठ/ड/ढ/ण/पका अगाडि प्रायः 'ष' हुन्छ",
                        ) {
                            return Some(p);
                        }
                    }
                }
            }

            // (अ) 'ष' उपशीर्षक subrule 2:
            // इ/उपछि क/प सन्दर्भमा ष्क/ष्प प्रकार.
            if input.contains("िसक")
                || input.contains("िशक")
                || input.contains("ुसक")
                || input.contains("ुशक")
                || input.contains("िसप")
                || input.contains("िशप")
                || input.contains("ुसप")
                || input.contains("ुशप")
            {
                let output = input
                    .replace("िसक", "िष्क")
                    .replace("िशक", "िष्क")
                    .replace("ुसक", "ुष्क")
                    .replace("ुशक", "ुष्क")
                    .replace("िसप", "िष्प")
                    .replace("िशप", "िष्प")
                    .replace("ुसप", "ुष्प")
                    .replace("ुशप", "ुष्प");
                if let Some(p) = corrected_if_attested(
                    input,
                    output,
                    Rule::VarnaVinyasNiyam("3(ग)(अ)-ष-2"),
                    "इ/उ सन्दर्भमा क/प अगाडि 'ष' प्रयोग हुन्छ",
                ) {
                    return Some(p);
                }
            }
        }
    }
    None
}
