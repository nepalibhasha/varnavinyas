use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::step::Step;
use varnavinyas_shabda::{Origin, classify};

const AJANTA_1_SINGLETONS: &[&str] = &["म", "तँ", "र", "न", "त"];
const AJANTA_2_AVYAYA: &[&str] = &["बाहिर", "भित्र", "आज", "तिर", "जब", "तब", "निर"];
const AJANTA_3_AJNAARTHA: &[&str] = &["भन", "पढ", "गर", "हेर", "बुझ", "लुक", "लेख"];
const AJANTA_4_NEG_N: &[&str] = &["जान्नँ", "गर्दैन", "भन्दैन", "लेखिन", "भनेन"];
const AJANTA_6_ASAMAPAK: &[&str] = &["गर्न", "हेर्न", "लेख्न", "पढ्न", "जान"];
const AJANTA_7_ANUKARANATMAK: &[&str] = &["टिलिक्क", "टुप्लुक्क", "स्वाट्ट"];
const AJANTA_8_NAMES: &[&str] = &["भात", "झ्याल", "राम", "रात", "देश", "हिमाल"];
const AJANTA_8_PRONOUNS: &[&str] = &["यस", "त्यस", "जस", "कस", "उस", "जुन"];
const AJANTA_8_ADJECTIVES: &[&str] = &["कठोर", "गरिब", "बिस", "तिस", "जवान"];

fn corrected(
    input: &str,
    output: String,
    code: &'static str,
    explanation: &'static str,
) -> Prakriya {
    Prakriya::corrected(
        input,
        &output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam(code),
            explanation,
            input,
            &output,
        )],
    )
}

// -----------------------------------------------------------------------------
// 3(ङ) अजन्त लेख्नुपर्ने रूप
// Implemented subrules:
// - 3(ङ)-अजन्त-1 .. 8
// -----------------------------------------------------------------------------
pub(super) fn rule_ajanta_required(input: &str) -> Option<Prakriya> {
    let Some(stem) = input.strip_suffix('्') else {
        return None;
    };

    if input.ends_with("छस्") || input.ends_with("छन्") || input.ends_with("इस्") || input == "अर्थात्"
    {
        return None;
    }
    if (input.ends_with("मान्") || input.ends_with("वान्") || input.ends_with("वत्"))
        && matches!(classify(stem), Origin::Tatsam)
    {
        return None;
    }

    let output = stem.to_string();
    if AJANTA_1_SINGLETONS.contains(&output.as_str()) {
        return Some(corrected(
            input,
            output,
            "3(ङ)-अजन्त-1",
            "एकाक्षरी सर्वनाम/अव्ययमा हलन्त लेखिँदैन",
        ));
    }

    if AJANTA_2_AVYAYA.contains(&output.as_str()) {
        return Some(corrected(
            input,
            output,
            "3(ङ)-अजन्त-2",
            "स्वरान्त अव्ययमा हलन्त लेखिँदैन",
        ));
    }

    if AJANTA_3_AJNAARTHA.contains(&output.as_str()) {
        return Some(corrected(
            input,
            output,
            "3(ङ)-अजन्त-3",
            "सामान्य आदरार्थी आज्ञार्थ क्रियापदमा हलन्त लेखिँदैन",
        ));
    }

    if AJANTA_4_NEG_N.contains(&output.as_str()) {
        return Some(corrected(
            input,
            output,
            "3(ङ)-अजन्त-4",
            "अन्त्यमा 'न' आउने अकरण क्रियापदमा हलन्त लेखिँदैन",
        ));
    }

    if output.ends_with('छ') {
        return Some(corrected(
            input,
            output,
            "3(ङ)-अजन्त-5",
            "अन्त्यमा 'छ' आउने समापक क्रियापदमा हलन्त लेखिँदैन",
        ));
    }

    if AJANTA_6_ASAMAPAK.contains(&output.as_str()) {
        return Some(corrected(
            input,
            output,
            "3(ङ)-अजन्त-6",
            "असमापक क्रियापदमा हलन्त लेखिँदैन",
        ));
    }

    if AJANTA_7_ANUKARANATMAK.contains(&output.as_str()) {
        return Some(corrected(
            input,
            output,
            "3(ङ)-अजन्त-7",
            "अनुकरणात्मक शब्दको अन्त्यमा हलन्त लेखिँदैन",
        ));
    }

    if AJANTA_8_NAMES.contains(&output.as_str())
        || AJANTA_8_PRONOUNS.contains(&output.as_str())
        || AJANTA_8_ADJECTIVES.contains(&output.as_str())
    {
        return Some(corrected(
            input,
            output,
            "3(ङ)-अजन्त-8",
            "कतिपय नाम/सर्वनाम/विशेषणमा लेखन अजन्त हुन्छ",
        ));
    }

    None
}
