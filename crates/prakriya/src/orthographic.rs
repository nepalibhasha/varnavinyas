use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::step::Step;
use varnavinyas_akshar::{is_matra, is_svar, is_vyanjan};
use varnavinyas_kosha::kosha;
use varnavinyas_shabda::{Origin, OriginSource, classify, classify_with_provenance};

pub const SPEC_CHANDRABINDU: RuleSpec = RuleSpec {
    id: "ortho-chandrabindu",
    category: RuleCategory::Chandrabindu,
    kind: DiagnosticKind::Error,
    priority: 300,
    citation: Rule::VarnaVinyasNiyam("3(ख)"),
    examples: &[("सिँह", "सिंह")],
};

pub const SPEC_SIBILANT: RuleSpec = RuleSpec {
    id: "ortho-sibilant",
    category: RuleCategory::ShaShaS,
    kind: DiagnosticKind::Error,
    priority: 310,
    citation: Rule::VarnaVinyasNiyam("3(ग)(अ)"),
    examples: &[("रजिष्टर", "रजिस्टर")],
};

pub const SPEC_RI_KRI: RuleSpec = RuleSpec {
    id: "ortho-ri-kri",
    category: RuleCategory::RiKri,
    kind: DiagnosticKind::Error,
    priority: 320,
    citation: Rule::VarnaVinyasNiyam("3(ग)-ऋ"),
    examples: &[("रिषि", "ऋषि"), ("क्रिति", "कृति")],
};

pub const SPEC_HALANTA: RuleSpec = RuleSpec {
    id: "ortho-halanta",
    category: RuleCategory::Halanta,
    kind: DiagnosticKind::Error,
    priority: 330,
    citation: Rule::VarnaVinyasNiyam("3(ङ)"),
    examples: &[("बुद्धिमान", "बुद्धिमान्"), ("श्रीमान", "श्रीमान्")],
};

pub const SPEC_AADHI_VRIDDHI: RuleSpec = RuleSpec {
    id: "ortho-aadhi-vriddhi",
    category: RuleCategory::AadhiVriddhi,
    kind: DiagnosticKind::Error,
    priority: 340,
    citation: Rule::VarnaVinyasNiyam("3(क)"),
    examples: &[("अर्थिक", "आर्थिक"), ("इतिहासिक", "ऐतिहासिक")],
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

/// Academy 3(ख): शब्दउत्पत्तिअनुसार चन्द्रबिन्दु/शिरबिन्दु प्रयोग।
/// - तत्सम: चन्द्रबिन्दु (ँ) होइन, शिरबिन्दु (ं)।
/// - तद्भव/आगन्तुक: अनुनासिकमा शिरबिन्दु (ं) होइन, चन्द्रबिन्दु (ँ)।
pub fn rule_chandrabindu(input: &str) -> Option<Prakriya> {
    let origin_decision = classify_with_provenance(input);
    let origin = origin_decision.origin;
    let source = origin_decision.source;

    match origin {
        Origin::Tatsam => {
            // तत्सम शब्दमा चन्द्रबिन्दु (ँ) भए शिरबिन्दु (ं) मा रूपान्तरण गर्ने।
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
            // तद्भव/देशजमा अनुनासिकका लागि चन्द्रबिन्दु (ँ) चाहिन्छ।
            // तर पञ्चम-वर्णको सटिक संकेतका रूपमा केही अवस्थामा शिरबिन्दु (ं) वैध हुन्छ
            // (जस्तै: संसारमा स अघि ं वैध मानिन्छ)।
            // त्यसैले स्पर्श व्यञ्जनअघि आएको ं लाई नबदल्ने।
            if input.contains('ं') {
                let chars: Vec<char> = input.chars().collect();
                let mut output_chars = chars.clone();
                let mut changed = false;

                for i in 0..chars.len() {
                    if chars[i] == 'ं' {
                        // ं पछि कुन वर्ण छ भन्ने जाँच
                        let next = chars.get(i + 1).copied();
                        let before_stop = next.is_some_and(is_stop_consonant);

                        // स्पर्श व्यञ्जनअघि नभए मात्र ं → ँ रूपान्तरण
                        if !before_stop && should_replace_shirbindu(input, &chars, i, source) {
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
            // आगन्तुक शब्दमा पनि अनुनासिकका लागि चन्द्रबिन्दु प्रयोग हुन्छ।
            if input.contains('ं') {
                let chars: Vec<char> = input.chars().collect();
                let mut output_chars = chars.clone();
                let mut changed = false;

                for i in 0..chars.len() {
                    if chars[i] == 'ं' {
                        let next = chars.get(i + 1).copied();
                        let before_stop = next.is_some_and(is_stop_consonant);
                        if !before_stop && should_replace_shirbindu(input, &chars, i, source) {
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

/// Academy 3(ग)(अ): शब्दउत्पत्तिअनुसार श/ष/स प्रयोग।
/// - आगन्तुक: सामान्यतः स (ष→स अनिवार्य, श→स सन्दर्भअनुसार)।
/// - तद्भव/देशज: ष→स।
/// - तत्सम: मूल श/ष/स कायम।
pub fn rule_sibilant(input: &str) -> Option<Prakriya> {
    let origin = classify(input);

    match origin {
        Origin::Aagantuk => {
            // आगन्तुक शब्दमा स को प्रयोग प्राथमिक हुन्छ।
            let mut output = input.to_string();
            let mut changed = false;

            if output.contains('ष') {
                output = output.replace('ष', "स");
                changed = true;
            }
            // ध्यान: आगन्तुकमा श→स सधैं अनिवार्य हुँदैन।
            // केही व्यक्तिवाचक नाममा श कायम रहन्छ (जस्तै: एशिया)।
            // त्यसैले यहाँ सार्वत्रिक ष→स मात्र लागू।

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

            // आगन्तुकमा ण→न पनि जाँच्ने।
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
            // तद्भव/देशजमा ष→स लागू हुन्छ।
            // तद्भवमा श चाहिँ कतिपय रूपमा वैध हुन सक्छ।
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
            // तत्सममा मूल श/ष/स जोगाइने; परिवर्तन हुँदैन।
        }
    }

    None
}

pub fn rule_ri_kri(input: &str) -> Option<Prakriya> {
    // ऋ/कृ नियम तत्सम वर्गीकृत शब्दमा मात्र लागू।
    // क्रिकेटजस्ता आगन्तुक शब्दमा रूपान्तरण नगर्ने।
    let origin = classify(input);
    if !matches!(origin, Origin::Tatsam) {
        return None;
    }

    // ढाँचा: तत्सम शब्दको सुरुको रि → ऋ
    if let Some(rest) = input.strip_prefix("रि") {
        // ज्ञात रूप: रिषि→ऋषि, रितु→ऋतु
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

    // ढाँचा: तत्सममा क्रि → कृ
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

/// Academy 3(ङ): तत्सम प्रत्ययमा हलन्त प्रयोगका नियम।
///
/// -मान, -वान, -वत अन्त्य भएका तत्सम रूपहरूमा हलन्त अनिवार्य हुन्छ
/// (-मान्, -वान्, -वत्)।
/// अन्य केही हलन्त सुधार (जस्तै महान→महान्) correction table मा राखिएका छन्।
pub fn rule_halanta(input: &str) -> Option<Prakriya> {
    let lex = kosha();

    // अजन्त-पक्ष: ...छ मा अन्त्य हुने समापक क्रियापदमा हलन्त आउँदैन।
    // जस्तै: जान्छ् -> जान्छ, गर्छ् -> गर्छ।
    // अनावश्यक सुधार नहोस् भनेर शब्दकोश-प्रमाणित अवस्थामा मात्र लागू।
    if let Some(stem) = input.strip_suffix("छ्") {
        let output = format!("{stem}छ");
        if lex.contains(&output) && !lex.contains(input) {
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(ङ)-अजन्त-5"),
                    "समापक क्रियापदको अन्त्यमा हलन्त लेखिँदैन (…छ)",
                    input,
                    &output,
                )],
            ));
        }
    }

    // 3(ङ) अनुसार क्रियापद हलन्त ढाँचा:
    // - दोस्रो पुरुष हेपार्थ (जस्तै: गर्छस्)
    // - तेस्रो पुरुष बहुवचन/आदरसूचक (जस्तै: जान्छन्)
    // शब्दकोशमा हलन्त-रूप भेटिँदा मात्र लागू।
    const VERB_SUFFIXES: &[(&str, &str, &str)] = &[
        ("छस", "छस्", "3(ङ)-2"),
        ("छन", "छन्", "3(ङ)-3"),
        ("इस", "इस्", "3(ङ)-2"),
    ];

    for (wrong_suffix, correct_suffix, rule_citation) in VERB_SUFFIXES {
        if let Some(stem) = input.strip_suffix(wrong_suffix) {
            let output = format!("{}{}", stem, correct_suffix);
            if lex.contains(&output) {
                return Some(Prakriya::corrected(
                    input,
                    &output,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam(rule_citation),
                        format!("क्रियापदमा हलन्त: {} -> {}", wrong_suffix, correct_suffix),
                        input,
                        &output,
                    )],
                ));
            }
        }
    }

    let origin = classify(input);
    if !matches!(origin, Origin::Tatsam) {
        return None;
    }

    // हलन्त चाहिने तत्सम प्रत्यय ढाँचा: -मान/-वान/-वत -> -मान्/-वान्/-वत्
    // जस्तै: बुद्धिमान -> बुद्धिमान्, भगवान -> भगवान्, विधिवत -> विधिवत्
    let suffixes = [
        ("मान", "मान्", "3(ङ)-मान्"),
        ("वान", "वान्", "3(ङ)-वान्"),
        ("वत", "वत्", "3(ङ)-वत्"),
    ];

    for (wrong_suffix, correct_suffix, rule_citation) in suffixes {
        if let Some(stem) = input.strip_suffix(wrong_suffix) {
            let output = format!("{}{}", stem, correct_suffix);

            // Guard: इनपुट शब्द शब्दकोशमा छ तर हलन्त-रूप छैन भने
            // यो प्रत्ययरूप होइन, मूल नामपद हुन सक्छ (जस्तै: सम्मान)।
            // त्यसैले हलन्त-रूप पनि वैध हुँदा मात्र सुधार गर्ने।
            if lex.contains(input) && !lex.contains(&output) {
                return None;
            }

            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam(rule_citation),
                    format!(
                        "तत्सम प्रत्ययमा हलन्त हुन्छ: {} -> {}",
                        wrong_suffix, correct_suffix
                    ),
                    input,
                    &output,
                )],
            ));
        }
    }

    None
}

/// Apply vriddhi to the first vowel position in a character sequence.
/// Returns `None` if already in vriddhi form or no applicable vowel found.
///
/// Vriddhi mappings: अ→आ, इ/ई→ऐ, उ/ऊ→औ (both standalone svars and matras).
fn apply_vriddhi(chars: &[char]) -> Option<Vec<char>> {
    for (i, &c) in chars.iter().enumerate() {
        if is_svar(c) {
            let vriddhi = match c {
                'अ' => 'आ',
                'इ' | 'ई' => 'ऐ',
                'उ' | 'ऊ' => 'औ',
                'आ' | 'ऐ' | 'औ' => return None,
                _ => return None,
            };
            let mut result = chars.to_vec();
            result[i] = vriddhi;
            return Some(result);
        }
        if is_matra(c) {
            let vriddhi = match c {
                'ि' | 'ी' => 'ै',
                'ु' | 'ू' => 'ौ',
                'ा' | 'ै' | 'ौ' => return None,
                _ => return None,
            };
            let mut result = chars.to_vec();
            result[i] = vriddhi;
            return Some(result);
        }
        if is_vyanjan(c) {
            let next = chars.get(i + 1).copied();
            // Matra follows — vowel will be handled in the next iteration
            if next.is_some_and(is_matra) {
                continue;
            }
            // Halanta (virama) — consonant cluster, skip
            if next == Some('्') {
                continue;
            }
            // Inherent अ — vriddhi is आ, represented by ा matra
            let mut result = chars.to_vec();
            result.insert(i + 1, 'ा');
            return Some(result);
        }
    }
    None
}

/// Academy 3(क): ādhivr̥ddhi with -इक suffix.
///
/// When -इक is added to a root, the first vowel undergoes vr̥ddhi:
/// अ→आ, इ/ई→ऐ, उ/ऊ→औ. The root must exist in kosha.
pub fn rule_aadhi_vriddhi(input: &str) -> Option<Prakriya> {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    if len < 3 {
        return None;
    }

    // Check for -इक suffix: ि + क (matra form) or इ + क (standalone)
    let has_ik = (chars[len - 2] == 'ि' || chars[len - 2] == 'इ') && chars[len - 1] == 'क';
    if !has_ik {
        return None;
    }

    // Strip suffix to get candidate root
    let root: String = chars[..len - 2].iter().collect();
    if root.is_empty() {
        return None;
    }

    let kosha = varnavinyas_kosha::kosha();
    if !kosha.contains(&root) {
        return None;
    }

    let corrected_chars = apply_vriddhi(&chars)?;
    let output: String = corrected_chars.into_iter().collect();

    if output == input {
        return None;
    }

    Some(Prakriya::corrected(
        input,
        &output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam("3(क)"),
            "इक प्रत्ययमा आदिवृद्धि: प्रथम स्वरमा वृद्धि हुन्छ",
            input,
            &output,
        )],
    ))
}

/// Academy 3(इ): ए/य distinction.
///
/// तत्सम words use य (यज्ञ, यथार्थ). एक-derived words use ए (एक, एकता).
/// Swaps initial ए↔य and validates against kosha.
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

    let kosha = varnavinyas_kosha::kosha();
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

/// Academy 3(उ): क्ष/छ distinction.
///
/// क्ष/क्षे/क्ष्य is तत्सम-only. छ/छे/छ्य is used in all origins.
/// Tries character substitutions and validates against kosha.
pub fn rule_ksha_chhya(input: &str) -> Option<Prakriya> {
    let kosha = varnavinyas_kosha::kosha();
    if kosha.contains(input) {
        return None;
    }

    if !input.contains("क्ष") && !input.contains('छ') && !input.contains("च्छ") {
        return None;
    }

    // Longer patterns first to avoid partial matches
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

/// Academy 3(ग)(ऊ): ज्ञ / ग्याँ / ग्या distinction.
///
/// - तत्सम words use ज्ञ.
/// - Nepali/loan words may use ग्याँ or ग्या.
///
/// This rule is intentionally kosha-backed to avoid aggressive rewrites.
pub fn rule_gya_gyan(input: &str) -> Option<Prakriya> {
    let kosha = varnavinyas_kosha::kosha();
    if kosha.contains(input) {
        return None;
    }

    if !input.contains("ज्ञ") && !input.contains("ग्या") && !input.contains("ग्याँ")
    {
        return None;
    }

    // Conservative direction only: common misspelling ग्याँ/ग्या in तत्सम words.
    // Map to ज्ञा and accept only if kosha confirms the candidate.
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

/// वर्ण स्पर्श-व्यञ्जन (क-वर्गदेखि म-वर्ग) हो कि होइन जाँच्ने।
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

/// गैर-तत्सम ं → ँ रूपान्तरण सुरक्षित छ कि छैन निर्धारण गर्ने।
///
/// override/kosha बाट उच्च-विश्वसनीयता वर्गीकरण आएमा सिधै लागू।
/// heuristic मात्र हुँदा:
/// - चन्द्रबिन्दु-रूप शब्दकोशमा भेटिनु, वा
/// - गरें/जान्छौं जस्ता स्पष्ट क्रियारूप संकेत हुनु आवश्यक।
fn should_replace_shirbindu(
    _input: &str,
    chars: &[char],
    idx: usize,
    origin_source: OriginSource,
) -> bool {
    if !matches!(origin_source, OriginSource::Heuristic) {
        return true;
    }

    if idx + 1 == chars.len() && idx > 0 && matches!(chars[idx - 1], 'े' | 'ौ') {
        return true;
    }

    let mut candidate_chars = chars.to_vec();
    candidate_chars[idx] = 'ँ';
    let candidate: String = candidate_chars.into_iter().collect();
    kosha().contains(&candidate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halanta_maan() {
        let p = rule_halanta("बुद्धिमान").expect("should correct बुद्धिमान");
        assert_eq!(p.output, "बुद्धिमान्");

        let p = rule_halanta("शक्तिमान").expect("should correct शक्तिमान");
        assert_eq!(p.output, "शक्तिमान्");
    }

    #[test]
    fn test_halanta_vaan() {
        // विद्वान contains द्व conjunct → तत्सम heuristic
        let p = rule_halanta("विद्वान").expect("should correct विद्वान");
        assert_eq!(p.output, "विद्वान्");
    }

    #[test]
    fn test_halanta_vat() {
        let p = rule_halanta("आत्मवत").expect("should correct आत्मवत");
        assert_eq!(p.output, "आत्मवत्");
    }

    #[test]
    fn test_halanta_skips_non_tatsam() {
        assert!(rule_halanta("नेपाल").is_none());
    }

    #[test]
    fn test_halanta_verb_second_person_disrespect() {
        let p = rule_halanta("गर्छस").expect("should correct गर्छस");
        assert_eq!(p.output, "गर्छस्");
    }

    #[test]
    fn test_halanta_verb_third_person_plural() {
        let p = rule_halanta("जान्छन").expect("should correct जान्छन");
        assert_eq!(p.output, "जान्छन्");
    }

    #[test]
    fn test_halanta_verb_irregular_is() {
        let p = rule_halanta("आइस").expect("should correct आइस");
        assert_eq!(p.output, "आइस्");
    }

    #[test]
    fn test_ajanta_terminal_chha_without_halanta() {
        let p = rule_halanta("जान्छ्").expect("should correct जान्छ्");
        assert_eq!(p.output, "जान्छ");

        let p = rule_halanta("गर्छ्").expect("should correct गर्छ्");
        assert_eq!(p.output, "गर्छ");
    }

    // --- Aadhi-vriddhi tests ---

    #[test]
    fn test_aadhi_vriddhi_standalone_a() {
        // अर्थिक → आर्थिक (standalone अ → आ)
        let p = rule_aadhi_vriddhi("अर्थिक").expect("should correct अर्थिक");
        assert_eq!(p.output, "आर्थिक");
    }

    #[test]
    fn test_aadhi_vriddhi_standalone_i() {
        // इतिहासिक → ऐतिहासिक (standalone इ → ऐ)
        let p = rule_aadhi_vriddhi("इतिहासिक").expect("should correct इतिहासिक");
        assert_eq!(p.output, "ऐतिहासिक");
    }

    #[test]
    fn test_aadhi_vriddhi_matra_i() {
        // दिनिक → दैनिक (matra ि → ै)
        let p = rule_aadhi_vriddhi("दिनिक").expect("should correct दिनिक");
        assert_eq!(p.output, "दैनिक");
    }

    #[test]
    fn test_aadhi_vriddhi_standalone_u() {
        // उद्योगिक → औद्योगिक (standalone उ → औ)
        let p = rule_aadhi_vriddhi("उद्योगिक").expect("should correct उद्योगिक");
        assert_eq!(p.output, "औद्योगिक");
    }

    #[test]
    fn test_aadhi_vriddhi_already_correct() {
        // आर्थिक → None (root "आर्थ" not in kosha)
        assert!(rule_aadhi_vriddhi("आर्थिक").is_none());
    }

    #[test]
    fn test_aadhi_vriddhi_no_suffix() {
        // संगीत → None (no -इक suffix)
        assert!(rule_aadhi_vriddhi("संगीत").is_none());
    }

    // --- Ya/E distinction tests ---

    #[test]
    fn test_ya_e_e_to_ya() {
        // एथार्थ → यथार्थ
        let p = rule_ya_e("एथार्थ").expect("should correct एथार्थ");
        assert_eq!(p.output, "यथार्थ");
    }

    #[test]
    fn test_ya_e_ya_to_e() {
        // यकता → एकता
        let p = rule_ya_e("यकता").expect("should correct यकता");
        assert_eq!(p.output, "एकता");
    }

    #[test]
    fn test_ya_e_already_valid() {
        // एक → None (in kosha)
        assert!(rule_ya_e("एक").is_none());
    }

    #[test]
    fn test_ya_e_no_match() {
        // नेपाल → None (doesn't start with ए/य)
        assert!(rule_ya_e("नेपाल").is_none());
    }

    // --- Ksha/Chhya distinction tests ---

    #[test]
    fn test_ksha_chhya_chhy_to_kshy() {
        // लछ्य → लक्ष्य
        let p = rule_ksha_chhya("लछ्य").expect("should correct लछ्य");
        assert_eq!(p.output, "लक्ष्य");
    }

    #[test]
    fn test_ksha_chhya_chh_to_ksh() {
        // छमा → क्षमा (छ→क्ष)
        let p = rule_ksha_chhya("छमा").expect("should correct छमा");
        assert_eq!(p.output, "क्षमा");
    }

    #[test]
    fn test_ksha_chhya_ksh_to_chchh() {
        // इक्षा → इच्छा
        let p = rule_ksha_chhya("इक्षा").expect("should correct इक्षा");
        assert_eq!(p.output, "इच्छा");
    }

    #[test]
    fn test_ksha_chhya_already_valid() {
        // क्षेत्र → None (in kosha)
        assert!(rule_ksha_chhya("क्षेत्र").is_none());
    }

    // --- Gya/Gyan distinction tests ---

    #[test]
    fn test_gya_gyan_gya_to_gya_nya() {
        // अग्यान -> अज्ञान
        let p = rule_gya_gyan("अग्यान").expect("should correct अग्यान");
        assert_eq!(p.output, "अज्ञान");
    }

    #[test]
    fn test_gya_gyan_another_gya_to_gya_nya() {
        // प्रग्या -> प्रज्ञा
        let p = rule_gya_gyan("प्रग्या").expect("should correct प्रग्या");
        assert_eq!(p.output, "प्रज्ञा");
    }

    #[test]
    fn test_gya_gyan_keeps_valid_loanword() {
        // ग्यारेज is a valid loanword form
        assert!(rule_gya_gyan("ग्यारेज").is_none());
    }

    #[test]
    fn test_gya_gyan_keeps_valid_tatsam() {
        // अज्ञान is valid तत्सम form
        assert!(rule_gya_gyan("अज्ञान").is_none());
    }

    #[test]
    fn test_chandrabindu_does_not_overflag_tatsam_shirbindu() {
        assert!(rule_chandrabindu("अंश").is_none());
        assert!(rule_chandrabindu("अंशु").is_none());
        assert!(rule_chandrabindu("संसार").is_none());
        // Tatsam words where Anusvara is before a stop consonant (classic pancham varna logic)
        // These should also be ignored by the rule, as Shirbindu is valid here.
        assert!(rule_chandrabindu("संघर्ष").is_none());
        assert!(rule_chandrabindu("संघीय").is_none());
    }

    #[test]
    fn test_chandrabindu_keeps_common_corrections() {
        let p = rule_chandrabindu("बांस").expect("should correct बांस");
        assert_eq!(p.output, "बाँस");

        let p = rule_chandrabindu("हांस").expect("should correct हांस");
        assert_eq!(p.output, "हाँस");

        let p = rule_chandrabindu("गरें").expect("should correct गरें");
        assert_eq!(p.output, "गरेँ");

        let p = rule_chandrabindu("जान्छौं").expect("should correct जान्छौं");
        assert_eq!(p.output, "जान्छौँ");
    }
}
