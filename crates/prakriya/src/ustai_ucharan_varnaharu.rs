use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::step::Step;
use varnavinyas_akshar::is_matra;
use varnavinyas_kosha::kosha;
use varnavinyas_shabda::{Origin, classify};

// Section 3(ग) source context:
// docs/Notices-pages-77-99.md (pp. 76-80),
// "(ग) उस्तै उच्चारण हुने वर्णहरू (श/ष/स, ब/व, य/ए, ऋ/रि, क्ष/छ्य, क्षे/छे)..."
//
// Implementation policy for this module:
// - follow Academy subrule intent first;
// - keep transformations conservative with kosha plausibility checks;
// - prefer "no suggestion" over speculative replacement.
//
// Mapping note:
// - This module implements the word-level parts of (ग).
// - Broad lexical inventories in the document are represented as guard logic
//   rather than exhaustive in-code wordlists.

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

// (अ) 'श, ष, स' को प्रयोग
// Document subrules:
// - 'श' (तत्सम): patterns around sibilant clusters / चवर्ग / ऋ,र / विसर्ग
// - 'ष' (तत्सम): retroflex and इ/उ + क/प environments
// - 'स' (all origins): especially तद्भव/आगन्तुक normalization
//
// Current implementation focus:
// - subrule-8 and subrule-9 style normalization for तद्भव/आगन्तुक.
// - leaves tatsam "श/ष/स" distinctions unchanged at this layer.
pub const SPEC_SIBILANT: RuleSpec = RuleSpec {
    id: "ortho-sibilant",
    category: RuleCategory::ShaShaS,
    kind: DiagnosticKind::Error,
    priority: 310,
    citation: Rule::VarnaVinyasNiyam("3(ग)(अ)"),
    examples: &[("रजिष्टर", "रजिस्टर")],
};

// (आ) 'ब', 'व' र 'ओ' को प्रयोग
// Document subrules:
// - 'ब' environments and lexical classes
// - 'व' environments and lexical classes
// - dedicated 'ओ' usage classes
//
// Current implementation focus:
// - conservative b<->v single-position swap with kosha validation.
// - 'ओ' logic is currently not a dedicated generalized rule in this module.
pub const SPEC_BA_VA: RuleSpec = RuleSpec {
    id: "ortho-ba-va",
    category: RuleCategory::ShaShaS,
    kind: DiagnosticKind::Error,
    priority: 315,
    citation: Rule::VarnaVinyasNiyam("3(ग)(आ)"),
    examples: &[("बिदेश", "विदेश"), ("बिज्ञान", "विज्ञान")],
};

// (ई) 'ऋ' र 'रि' को प्रयोग
// Document subrules:
// - ऋ/kṛ-family forms are tatsam-only;
// - तद्भव/आगन्तुक generally keep "रि".
//
// Current implementation focus:
// - tatsam-only रि->ऋ and क्रि->कृ corrections, guarded by kosha.
pub const SPEC_RI_KRI: RuleSpec = RuleSpec {
    id: "ortho-ri-kri",
    category: RuleCategory::RiKri,
    kind: DiagnosticKind::Error,
    priority: 320,
    citation: Rule::VarnaVinyasNiyam("3(ग)-ऋ"),
    examples: &[("रिषि", "ऋषि"), ("क्रिति", "कृति")],
};

// (इ) 'ए' र 'य' को प्रयोग
// Document subrules include multiple morphological buckets (verb forms,
// participles, pronouns, tatsam/loan classes).
//
// Current implementation focus:
// - conservative word-initial ए<->य alternation with kosha validation.
pub const SPEC_YA_E: RuleSpec = RuleSpec {
    id: "ortho-ya-e",
    category: RuleCategory::YaE,
    kind: DiagnosticKind::Error,
    priority: 350,
    citation: Rule::VarnaVinyasNiyam("3(ग)(इ)"),
    examples: &[("एथार्थ", "यथार्थ"), ("यकता", "एकता")],
};

// (उ) क्ष/क्षे/क्ष्य र छ/छे/छ्य को प्रयोग
// Document subrules:
// - tatsam: क्ष/क्षे/क्ष्य;
// - broader usage: छ/छे/छ्य.
//
// Current implementation focus:
// - bidirectional candidate generation among listed grapheme families
//   with kosha attestation guard.
pub const SPEC_KSHA_CHHYA: RuleSpec = RuleSpec {
    id: "ortho-ksha-chhya",
    category: RuleCategory::KshaChhya,
    kind: DiagnosticKind::Error,
    priority: 360,
    citation: Rule::VarnaVinyasNiyam("3(ग)(उ)"),
    examples: &[("लछ्य", "लक्ष्य"), ("छेत्र", "क्षेत्र")],
};

// (ऊ) 'ज्ञ', 'ग्याँ' र 'ग्या' को प्रयोग
// Document subrules:
// - ज्ञ: tatsam-only class;
// - ग्याँ/ग्या: mostly Nepali/loan usage classes.
//
// Current implementation focus:
// - normalize ग्याँ/ग्या -> ज्ञा only when target is kosha-attested.
pub const SPEC_GYA_GYAN: RuleSpec = RuleSpec {
    id: "ortho-gya-gyan",
    category: RuleCategory::GyaGyan,
    kind: DiagnosticKind::Error,
    priority: 365,
    citation: Rule::VarnaVinyasNiyam("3(ग)(ऊ)"),
    examples: &[("अग्यान", "अज्ञान"), ("प्रग्या", "प्रज्ञा")],
};

// Academy 3(ग)(अ): normalize to "स" for aagantuk/tadbhav paths where plausible.
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
                if !lex.contains(input) || lex.contains(&output) {
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
                if !lex.contains(input) || lex.contains(&output) {
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

// Academy 3(ग)(ई): enforce तत्सम ऋ/कृ forms only when lexically plausible.
pub fn rule_ri_kri(input: &str) -> Option<Prakriya> {
    let origin = classify(input);
    if !matches!(origin, Origin::Tatsam) {
        return None;
    }
    let lex = kosha();
    if lex.contains(input) {
        return None;
    }

    if let Some(rest) = input.strip_prefix("रि") {
        if rest.starts_with('ष') || rest.starts_with('त') {
            let output = format!("ऋ{rest}");
            if lex.contains(&output) {
                return Some(Prakriya::corrected(
                    input,
                    &output,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(ग)(ई)-ऋ-1"),
                        "तत्सम शब्दमा ऋ हुन्छ (रि होइन)",
                        input,
                        &output,
                    )],
                ));
            }
        }
    }
    if input.contains("क्रि") {
        let output = input.replace("क्रि", "कृ");
        if output != input && lex.contains(&output) {
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(ग)(ई)-ऋ-1"),
                    "तत्सम शब्दमा कृ हुन्छ (क्रि होइन)",
                    input,
                    &output,
                )],
            ));
        }
    }
    None
}

// Academy 3(ग)(आ): single-position ब/व swap with kosha validation.
pub fn rule_ba_va(input: &str) -> Option<Prakriya> {
    if input.is_empty() {
        return None;
    }
    let kosha = kosha();
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
            } else if from == 'ब' && (output.contains("वै") || output.contains('ृ')) {
                // 'व' को प्रयोग, subrule 2
                "3(ग)(आ)-व-2"
            } else if from == 'ब'
                && (output.contains("र्ष")
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

// Academy 3(ग)(इ): first-letter ए<->य alternation with lexicon guard.
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
        let citation = if input.starts_with('य') {
            // य -> ए correction path.
            if candidate.ends_with("एँ")
                || candidate.ends_with("ए")
                || candidate.ends_with("एछ")
                || candidate.ends_with("एछौ")
                || candidate.ends_with("एछु")
            {
                "3(ग)(इ)-ए-1"
            } else if candidate.contains("एर")
                || candidate.contains("एको")
                || candidate.contains("एका")
                || candidate.contains("एकी")
            {
                "3(ग)(इ)-ए-2"
            } else if candidate.starts_with("एक") || candidate.starts_with("एघार") {
                "3(ग)(इ)-ए-5"
            } else {
                "3(ग)(इ)-ए-6"
            }
        } else {
            // ए -> य correction path.
            if candidate.starts_with("यो")
                || candidate.starts_with("यो ")
                || candidate.starts_with("यत")
                || candidate.starts_with("यह")
                || candidate.starts_with("त्यो")
            {
                "3(ग)(इ)-य-1"
            } else if candidate.ends_with("यौ")
                || candidate.ends_with("यो")
                || candidate.ends_with("यौँ")
            {
                "3(ग)(इ)-य-2"
            } else if candidate.ends_with("िया")
                || candidate.ends_with("ैया")
                || candidate.ends_with("्यौली")
                || candidate.ends_with("्याइँ")
            {
                "3(ग)(इ)-य-3"
            } else if candidate.starts_with("यज्ञ")
                || candidate.starts_with("यक्ष")
                || candidate.starts_with("यथ")
                || candidate.starts_with("यति")
            {
                "3(ग)(इ)-य-4"
            } else {
                "3(ग)(इ)-य-5"
            }
        };
        return Some(Prakriya::corrected(
            input,
            &candidate,
            vec![Step::new(
                Rule::VarnaVinyasNiyam(citation),
                "ए/य भेद: शब्दादिमा ए र य फरक हुन्छ",
                input,
                &candidate,
            )],
        ));
    }
    None
}

// Academy 3(ग)(उ): canonicalize क्ष/छ/च्छ variants using known lexical targets.
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
                let citation = if to.contains("क्ष") {
                    "3(ग)(उ)-क्ष-1"
                } else if to == "छे" || from == "क्षे" {
                    "3(ग)(उ)-छे-2"
                } else {
                    "3(ग)(उ)-छ्य-3"
                };
                return Some(Prakriya::corrected(
                    input,
                    &candidate,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam(citation),
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

// Academy 3(ग)(ऊ): prefer ज्ञ-series only when candidate lemma is attested.
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
                let citation = if from == "ग्याँ" {
                    "3(ग)(ऊ)-2"
                } else {
                    "3(ग)(ऊ)-3"
                };
                return Some(Prakriya::corrected(
                    input,
                    &candidate,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam(citation),
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
