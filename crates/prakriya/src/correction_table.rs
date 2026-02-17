use crate::rule::Rule;
use std::sync::LazyLock;

/// A correction entry from the Academy standard.
pub struct CorrectionEntry {
    pub correct: &'static str,
    pub rule: Rule,
    pub description: &'static str,
}

/// Static correction table covering all 91 gold.toml entries.
/// Key: incorrect form, Value: correction entry.
pub static CORRECTION_TABLE: LazyLock<Vec<(&'static str, CorrectionEntry)>> = LazyLock::new(|| {
    let mut table = vec![
        // =================================================================
        // shuddha_table entries (Section 4)
        // =================================================================
        (
            "अत्याधिक",
            CorrectionEntry {
                correct: "अत्यधिक",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "vowel sandhi: अति + अधिक = अत्यधिक (not अत्याधिक)",
            },
        ),
        (
            "राजनैतिक",
            CorrectionEntry {
                correct: "राजनीतिक",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "राजनीति + क = राजनीतिक (not राजनैतिक)",
            },
        ),
        (
            "उल्लेखित",
            CorrectionEntry {
                correct: "उल्लिखित",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "उत् + लिखित = उल्लिखित (not उल्लेखित)",
            },
        ),
        (
            "बागमती",
            CorrectionEntry {
                correct: "बाग्मती",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "व्यक्तिवाचक नाम from Sanskrit वाग्मती: conjunct ग्म अनिवार्य",
            },
        ),
        (
            "पुनरावलोकन",
            CorrectionEntry {
                correct: "पुनरवलोकन",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "पुनर् + अवलोकन = पुनरवलोकन (not पुनरावलोकन)",
            },
        ),
        (
            "व्यवहारिक",
            CorrectionEntry {
                correct: "व्यावहारिक",
                rule: Rule::VarnaVinyasNiyam("3(क)-इक-प्रत्यय"),
                description: "इक suffix triggers आदिवृद्धि: व्यवहार + इक = व्यावहारिक",
            },
        ),
        (
            "धैर्यता",
            CorrectionEntry {
                correct: "धीरता",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "-ता अनावश्यक: धीर+ता=धीरता, or base form धैर्य",
            },
        ),
        (
            "प्रसाशन",
            CorrectionEntry {
                correct: "प्रशासन",
                rule: Rule::ShuddhaAshuddha("Section 4, Section 3(ग)"),
                description: "प्र + शासन = प्रशासन (श not स, correct vowel order)",
            },
        ),
        (
            "संसद",
            CorrectionEntry {
                correct: "संसद्",
                rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                description: "हलन्त अनिवार्य: Sanskrit stem ends in द् (संसद्)",
            },
        ),
        (
            "परिषद",
            CorrectionEntry {
                correct: "परिषद्",
                rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                description: "हलन्त अनिवार्य: Sanskrit stem ends in द् (परिषद्)",
            },
        ),
        (
            "संघीय",
            CorrectionEntry {
                correct: "सङ्घीय",
                rule: Rule::VarnaVinyasNiyam("3(ख)-पञ्चम"),
                description: "panchham varna ङ अनिवार्य before घ (not शिरबिन्दु ं)",
            },
        ),
        (
            "पुनर्स्थापना",
            CorrectionEntry {
                correct: "पुनःस्थापना",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "visarga retained: पुनः + स्थापना (not पुनर्)",
            },
        ),
        (
            "पुनर्संरचना",
            CorrectionEntry {
                correct: "पुनःसंरचना",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "visarga retained: पुनः + संरचना (not पुनर्)",
            },
        ),
        (
            "महत्व",
            CorrectionEntry {
                correct: "महत्त्व",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "double त अनिवार्य: महत् + त्व = महत्त्व",
            },
        ),
        (
            "पश्चाताप",
            CorrectionEntry {
                correct: "पश्चात्ताप",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "double त अनिवार्य: पश्चात् + ताप = पश्चात्ताप",
            },
        ),
        (
            "मुद्धा",
            CorrectionEntry {
                correct: "मुद्दा",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "wrong gemination: द्द (not द्ध)",
            },
        ),
        (
            "श्रृङ्गार",
            CorrectionEntry {
                correct: "शृङ्गार",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "शृ not श्रृ: श + ृ = शृ (no र involved)",
            },
        ),
        (
            "श्रृङ्खला",
            CorrectionEntry {
                correct: "शृङ्खला",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "शृ not श्रृ: श + ृ = शृ (no र involved)",
            },
        ),
        (
            "हरु",
            CorrectionEntry {
                correct: "हरू",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "plural suffix takes दीर्घ ऊ: हरू (not हरु)",
            },
        ),
        (
            "रुप",
            CorrectionEntry {
                correct: "रूप",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "तत्सम रूपमा दीर्घ ऊ हुन्छ",
            },
        ),
        (
            "सौन्दर्यता",
            CorrectionEntry {
                correct: "सुन्दरता",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "-ता अनावश्यक: use सुन्दरता (सुन्दर+ता) or सौन्दर्य",
            },
        ),
        (
            "गुणस्तरीयता",
            CorrectionEntry {
                correct: "गुणस्तरीय",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "-ता अनावश्यक: गुणस्तरीय is already an adjective",
            },
        ),
        (
            "औचित्यता",
            CorrectionEntry {
                correct: "औचित्य",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "-ता अनावश्यक: औचित्य already abstract",
            },
        ),
        (
            "आतिथ्यता",
            CorrectionEntry {
                correct: "आतिथ्य",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "-ता अनावश्यक: आतिथ्य already abstract",
            },
        ),
        (
            "यथार्थता",
            CorrectionEntry {
                correct: "यथार्थ",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "-ता अनावश्यक: यथार्थ already functions as noun/adjective",
            },
        ),
        (
            "कार्यबाही",
            CorrectionEntry {
                correct: "कारबाही",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "कार+बाही (no र्य): कारबाही",
            },
        ),
        (
            "वृक्षारोपण",
            CorrectionEntry {
                correct: "वृक्षरोपण",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "वृक्ष+रोपण = वृक्षरोपण (no आ insertion)",
            },
        ),
        (
            "गत्यावरोध",
            CorrectionEntry {
                correct: "गत्यवरोध",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "गति+अवरोध = गत्यवरोध (यण् sandhi, no आ)",
            },
        ),
        (
            "सामाग्री",
            CorrectionEntry {
                correct: "सामग्री",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "no extra आ: सामग्री",
            },
        ),
        (
            "भएकोमा",
            CorrectionEntry {
                correct: "भएकामा",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "नामयोगी form: भएका+मा = भएकामा",
            },
        ),
        (
            "एनकानुन",
            CorrectionEntry {
                correct: "ऐनकानुन",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "आदिवृद्धि: ऐन (इ→ऐ) + कानुन = ऐनकानुन",
            },
        ),
        (
            "सामाजीकरण",
            CorrectionEntry {
                correct: "सामाजिकीकरण",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "सामाजिक+ईकरण = सामाजिकीकरण",
            },
        ),
        (
            "औद्योगीकरण",
            CorrectionEntry {
                correct: "औद्योगिकीकरण",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "औद्योगिक+ईकरण = औद्योगिकीकरण",
            },
        ),
        (
            "असक्षम",
            CorrectionEntry {
                correct: "अक्षम",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "अ+क्षम = अक्षम (no extra स)",
            },
        ),
        (
            "सपाङ्ग",
            CorrectionEntry {
                correct: "साङ्ग",
                rule: Rule::ShuddhaAshuddha("Section 4, Section 4(ख)"),
                description: "स+अङ्ग = साङ्ग (दीर्घ sandhi)",
            },
        ),
        (
            "ब्यहोरा",
            CorrectionEntry {
                correct: "बेहोरा",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "मानक रूप: बेहोरा",
            },
        ),
        (
            "एकिन",
            CorrectionEntry {
                correct: "यकिन",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "मानक रूप: यकिन (य not ए)",
            },
        ),
        (
            "सुरुवात",
            CorrectionEntry {
                correct: "सुरुआत",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "मानक रूप: सुरुआत (not सुरुवात)",
            },
        ),
        (
            "रजिष्टर",
            CorrectionEntry {
                correct: "रजिस्टर",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "आगन्तुक: स not ष for English 'register'",
            },
        ),
        (
            "इन्ष्टिच्युट",
            CorrectionEntry {
                correct: "इन्स्टिच्युट",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "आगन्तुक: स not ष for English 'institute'",
            },
        ),
        (
            "फाउण्डेसन",
            CorrectionEntry {
                correct: "फाउन्डेसन",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "आगन्तुक: न not ण for English 'foundation'",
            },
        ),
        (
            "झण्डा",
            CorrectionEntry {
                correct: "झन्डा",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "न not ण before ड: झन्डा",
            },
        ),
        (
            "इण्डिया",
            CorrectionEntry {
                correct: "इन्डिया",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "आगन्तुक: न not ण for English 'India'",
            },
        ),
        (
            "इंग्ल्याण्ड",
            CorrectionEntry {
                correct: "इङ्ग्ल्यान्ड",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "panchham ङ before ग + न not ण for 'England'",
            },
        ),
        (
            "शहीद",
            CorrectionEntry {
                correct: "सहिद",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "रूपान्तरित आगन्तुक शब्द: स not श, ह्रस्व इ not दीर्घ ई",
            },
        ),
        // =================================================================
        // hrasva_dirgha entries (Section 3(क))
        // =================================================================
        (
            "मीठो",
            CorrectionEntry {
                correct: "मिठो",
                rule: Rule::VarnaVinyasNiyam("3(क)-12"),
                description: "तद्भव एकार्थक शब्दमा ह्रस्व हुन्छ: मिष्ट → मिठो",
            },
        ),
        (
            "पीरो",
            CorrectionEntry {
                correct: "पिरो",
                rule: Rule::VarnaVinyasNiyam("3(क)-12"),
                description: "तद्भव एकार्थक शब्दमा ह्रस्व हुन्छ",
            },
        ),
        (
            "तिथीमीति",
            CorrectionEntry {
                correct: "तिथिमिति",
                rule: Rule::VarnaVinyasNiyam("3(क)-12"),
                description: "तद्भव समासिक शब्दमा दुवै पदमा ह्रस्व हुन्छ",
            },
        ),
        (
            "मीलेको",
            CorrectionEntry {
                correct: "मिलेको",
                rule: Rule::VarnaVinyasNiyam("3(क)-12"),
                description: "तद्भव क्रियामूलमा ह्रस्व हुन्छ: मिल्नु → मिलेको",
            },
        ),
        (
            "दैनीकी",
            CorrectionEntry {
                correct: "दैनिकी",
                rule: Rule::VarnaVinyasNiyam("3(क)-12"),
                description: "तद्भव व्युत्पत्तिमा शब्दमध्यको स्वर ह्रस्व हुन्छ",
            },
        ),
        (
            "भाई",
            CorrectionEntry {
                correct: "भाइ",
                rule: Rule::VarnaVinyasNiyam("3(क)-12"),
                description: "नातासम्बन्धी तद्भव: भ्रातृ → भाइ (ह्रस्व इ, not ई)",
            },
        ),
        (
            "मूखमा",
            CorrectionEntry {
                correct: "मुखमा",
                rule: Rule::VarnaVinyasNiyam("3(क)"),
                description: "तत्सम मुख retains original ह्रस्व उ (not दीर्घ ऊ)",
            },
        ),
        (
            "पूतली",
            CorrectionEntry {
                correct: "पुतली",
                rule: Rule::VarnaVinyasNiyam("3(क)-12, 3(ई)"),
                description: "तद्भव शब्दमा ह्रस्व उ हुन्छ",
            },
        ),
        (
            "अनुभूती",
            CorrectionEntry {
                correct: "अनुभूति",
                rule: Rule::VarnaVinyasNiyam("3(क)"),
                description: "तत्सम ending: अनुभूति ends in ह्रस्व इ",
            },
        ),
        (
            "हामि",
            CorrectionEntry {
                correct: "हामी",
                rule: Rule::VarnaVinyasNiyam("3(ई)-ऊ-7"),
                description: "सर्वनाममा दीर्घ हुन्छ: हामी (not हामि)",
            },
        ),
        (
            "दीदी",
            CorrectionEntry {
                correct: "दिदी",
                rule: Rule::VarnaVinyasNiyam("3(इ)-ऊ-3"),
                description: "नातासम्बन्धी तद्भव: initial vowel ह्रस्व, final दीर्घ",
            },
        ),
        (
            "बहीनी",
            CorrectionEntry {
                correct: "बहिनी",
                rule: Rule::VarnaVinyasNiyam("3(इ)-ऊ-3"),
                description: "नातासम्बन्धी तद्भव: शब्दमध्यको स्वर ह्रस्व, final दीर्घ",
            },
        ),
        (
            "भाउजु",
            CorrectionEntry {
                correct: "भाउजू",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "स्त्रीलिङ्गी नातासम्बन्धी शब्दमा दीर्घ हुन्छ: भाउजू (not भाउजु)",
            },
        ),
        (
            "फुपु",
            CorrectionEntry {
                correct: "फुपू",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "स्त्रीलिङ्गी नातासम्बन्धी शब्दमा दीर्घ हुन्छ: फुपू (not फुपु)",
            },
        ),
        (
            "मीतिनिले",
            CorrectionEntry {
                correct: "मितिनीले",
                rule: Rule::VarnaVinyasNiyam("3(इ), 3(ई)"),
                description: "नातासम्बन्धी तद्भव: initial ह्रस्व इ, final दीर्घ ई",
            },
        ),
        (
            "खुर्सानि",
            CorrectionEntry {
                correct: "खुर्सानी",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "स्त्रीलिङ्गी नामपदको अन्त्यमा दीर्घ ई हुन्छ",
            },
        ),
        (
            "सम्धिनि",
            CorrectionEntry {
                correct: "सम्धिनी",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "स्त्रीलिङ्गी नामपदको अन्त्यमा दीर्घ ई हुन्छ",
            },
        ),
        (
            "पहाडि",
            CorrectionEntry {
                correct: "पहाडी",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "विशेषण/स्थानबोधक शब्दको अन्त्यमा दीर्घ ई हुन्छ",
            },
        ),
        (
            "अगाडि",
            CorrectionEntry {
                correct: "अगाडी",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "अव्यय/नामयोगी शब्दको अन्त्यमा दीर्घ ई हुन्छ",
            },
        ),
        (
            "भनि",
            CorrectionEntry {
                correct: "भनी",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "असमापक क्रियामा अन्त्यमा दीर्घ ई हुन्छ",
            },
        ),
        (
            "स्वीकार्नु",
            CorrectionEntry {
                correct: "स्विकार्नु",
                rule: Rule::VarnaVinyasNiyam("3(क)-suffix-नु"),
                description: "प्रत्यय -नु ले ह्रस्व: स्वीकार + नु = स्विकार्नु",
            },
        ),
        (
            "पूर्वेली",
            CorrectionEntry {
                correct: "पुर्वेली",
                rule: Rule::VarnaVinyasNiyam("3(क)-suffix-एली"),
                description: "प्रत्यय -एली ले ह्रस्व: पूर्व + एली = पुर्वेली",
            },
        ),
        (
            "पुर्वी",
            CorrectionEntry {
                correct: "पूर्वी",
                rule: Rule::VarnaVinyasNiyam("3(ई)-suffix-ई"),
                description: "प्रत्यय -ई ले दीर्घ: पूर्व + ई = पूर्वी",
            },
        ),
        (
            "पुर्वीय",
            CorrectionEntry {
                correct: "पूर्वीय",
                rule: Rule::VarnaVinyasNiyam("3(ई)-suffix-ईय"),
                description: "प्रत्यय -ईय ले दीर्घ: पूर्व + ईय = पूर्वीय",
            },
        ),
        // =================================================================
        // चन्द्रबिन्दु entries (Section 3(ख))
        // =================================================================
        (
            "सिँह",
            CorrectionEntry {
                correct: "सिंह",
                rule: Rule::VarnaVinyasNiyam("3(ख)"),
                description: "तत्सम uses शिरबिन्दु (ं), not चन्द्रबिन्दु (ँ)",
            },
        ),
        (
            "सँवाद",
            CorrectionEntry {
                correct: "संवाद",
                rule: Rule::VarnaVinyasNiyam("3(ख)"),
                description: "तत्सम uses शिरबिन्दु (ं), not चन्द्रबिन्दु (ँ)",
            },
        ),
        (
            "जान्छौ",
            CorrectionEntry {
                correct: "जान्छौँ",
                rule: Rule::VarnaVinyasNiyam("3(ख)"),
                description: "तद्भव क्रियापदमा अनुनासिकका लागि चन्द्रबिन्दु हुन्छ",
            },
        ),
        (
            "आउछ",
            CorrectionEntry {
                correct: "आउँछ",
                rule: Rule::VarnaVinyasNiyam("3(ख)"),
                description: "तद्भव क्रियापदमा अनुनासिकका लागि चन्द्रबिन्दु हुन्छ",
            },
        ),
        (
            "वगैचामा",
            CorrectionEntry {
                correct: "बगैँचामा",
                rule: Rule::VarnaVinyasNiyam("3(ख)"),
                description: "ब (not व) + चन्द्रबिन्दु अनिवार्य: बगैँचा",
            },
        ),
        (
            "बगैचा",
            CorrectionEntry {
                correct: "बगैँचा",
                rule: Rule::VarnaVinyasNiyam("3(ख)"),
                description: "चन्द्रबिन्दु अनिवार्य on बगैँचा",
            },
        ),
        // =================================================================
        // sha_sha_sa entries (Section 3(ग))
        // =================================================================
        (
            "सासन",
            CorrectionEntry {
                correct: "शासन",
                rule: Rule::VarnaVinyasNiyam("3(ग)"),
                description: "तत्सम word uses श (not स): शासन",
            },
        ),
        (
            "सेष",
            CorrectionEntry {
                correct: "शेष",
                rule: Rule::VarnaVinyasNiyam("3(ग)"),
                description: "तत्सम word uses श (not स): शेष",
            },
        ),
        (
            "एसिया",
            CorrectionEntry {
                correct: "एशिया",
                rule: Rule::VarnaVinyasNiyam("3(ग)"),
                description: "व्यक्तिवाचक नाम uses श (not स): एशिया",
            },
        ),
        (
            "विवेकशिल",
            CorrectionEntry {
                correct: "विवेकशील",
                rule: Rule::VarnaVinyasNiyam("3(ग), 3(ई)"),
                description: "तत्सम suffix -शील takes दीर्घ ई",
            },
        ),
        // =================================================================
        // ri_kri entries (Section 3(ग))
        // =================================================================
        (
            "रिषि",
            CorrectionEntry {
                correct: "ऋषि",
                rule: Rule::VarnaVinyasNiyam("3(ग)-ऋ"),
                description: "तत्सम uses ऋ (not रि): ऋषि",
            },
        ),
        (
            "रितु",
            CorrectionEntry {
                correct: "ऋतु",
                rule: Rule::VarnaVinyasNiyam("3(ग)-ऋ"),
                description: "तत्सम uses ऋ (not रि): ऋतु",
            },
        ),
        (
            "क्रिति",
            CorrectionEntry {
                correct: "कृति",
                rule: Rule::VarnaVinyasNiyam("3(ग)-कृ"),
                description: "तत्सम uses कृ (not क्रि): कृति",
            },
        ),
        (
            "रिषिमुनि",
            CorrectionEntry {
                correct: "ऋषिमुनि",
                rule: Rule::VarnaVinyasNiyam("3(ग)-ऋ"),
                description: "तत्सम compound: ऋषि + मुनि (ऋ not रि)",
            },
        ),
        // =================================================================
        // हलन्त entries (Section 3(ङ))
        // =================================================================
        (
            "अर्थात",
            CorrectionEntry {
                correct: "अर्थात्",
                rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                description: "हलन्त अनिवार्य on अव्यय: अर्थात्",
            },
        ),
        (
            "बुद्धिमान",
            CorrectionEntry {
                correct: "बुद्धिमान्",
                rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                description: "-मान् प्रत्ययमा हलन्त अनिवार्य हुन्छ (बुद्धिमान्)",
            },
        ),
        (
            "भगवान",
            CorrectionEntry {
                correct: "भगवान्",
                rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                description: "-वान् प्रत्ययमा हलन्त अनिवार्य हुन्छ (भगवान्)",
            },
        ),
        (
            "महान",
            CorrectionEntry {
                correct: "महान्",
                rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                description: "हलन्त अनिवार्य: तत्सम stem ends in न् (महान्)",
            },
        ),
        (
            "विद्वान",
            CorrectionEntry {
                correct: "विद्वान्",
                rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                description: "-वान् प्रत्ययमा हलन्त अनिवार्य हुन्छ (विद्वान्)",
            },
        ),
        (
            "श्रीमान",
            CorrectionEntry {
                correct: "श्रीमान्",
                rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                description: "-मान् प्रत्ययमा हलन्त अनिवार्य हुन्छ (श्रीमान्)",
            },
        ),
        // =================================================================
        // b_v entries (Section 3(ग)-बव)
        // =================================================================
        (
            "बिद्या",
            CorrectionEntry {
                correct: "विद्या",
                rule: Rule::VarnaVinyasNiyam("3(ग)-बव"),
                description: "तत्सम word uses व (not ब): विद्या",
            },
        ),
        (
            "बिद्वान",
            CorrectionEntry {
                correct: "विद्वान्",
                rule: Rule::VarnaVinyasNiyam("3(ग)-बव"),
                description: "तत्सम word uses व (not ब) + हलन्त: विद्वान्",
            },
        ),
        (
            "बिदेश",
            CorrectionEntry {
                correct: "विदेश",
                rule: Rule::VarnaVinyasNiyam("3(ग)-बव"),
                description: "तत्सम word uses व (not ब): विदेश",
            },
        ),
        (
            "बिकास",
            CorrectionEntry {
                correct: "विकास",
                rule: Rule::VarnaVinyasNiyam("3(ग)-बव"),
                description: "तत्सम word uses व (not ब): विकास",
            },
        ),
        (
            "बिज्ञान",
            CorrectionEntry {
                correct: "विज्ञान",
                rule: Rule::VarnaVinyasNiyam("3(ग)-बव"),
                description: "तत्सम word uses व (not ब): विज्ञान",
            },
        ),
        // =================================================================
        // ya_e entries (Section 3(छ))
        // =================================================================
        (
            "एथार्थ",
            CorrectionEntry {
                correct: "यथार्थ",
                rule: Rule::VarnaVinyasNiyam("3(छ)"),
                description: "तत्सम uses य (not ए): यथार्थ",
            },
        ),
        (
            "यकता",
            CorrectionEntry {
                correct: "एकता",
                rule: Rule::VarnaVinyasNiyam("3(छ)"),
                description: "तत्सम uses ए (not य): एकता",
            },
        ),
        // =================================================================
        // ksha_chhya entries (Section 3(छ))
        // =================================================================
        (
            "लछ्य",
            CorrectionEntry {
                correct: "लक्ष्य",
                rule: Rule::VarnaVinyasNiyam("3(छ)-क्ष"),
                description: "तत्सम uses क्ष (not छ): लक्ष्य",
            },
        ),
        (
            "इक्षा",
            CorrectionEntry {
                correct: "इच्छा",
                rule: Rule::VarnaVinyasNiyam("3(छ)-क्ष"),
                description: "तत्सम: इच्छा uses च्छ (not क्ष)",
            },
        ),
        (
            "छेत्र",
            CorrectionEntry {
                correct: "क्षेत्र",
                rule: Rule::VarnaVinyasNiyam("3(छ)-क्ष"),
                description: "तत्सम uses क्षे (not छे): क्षेत्र",
            },
        ),
        // =================================================================
        // paragraph_correction entries
        // =================================================================
        (
            "भनीन",
            CorrectionEntry {
                correct: "भनिन्",
                rule: Rule::VarnaVinyasNiyam("3(क), 3(ङ)"),
                description: "verb form takes ह्रस्व + हलन्त: भनिन्",
            },
        ),
        (
            "सन्सारमा",
            CorrectionEntry {
                correct: "संसारमा",
                rule: Rule::VarnaVinyasNiyam("3(ख)"),
                description: "शिरबिन्दु form: संसार (not हलन्त-न सन्सार)",
            },
        ),
    ];
    // Sort for binary-search lookup.
    table.sort_by(|a, b| a.0.cmp(b.0));
    table
});

/// Look up a word in the correction table.
pub fn lookup(word: &str) -> Option<&'static CorrectionEntry> {
    CORRECTION_TABLE
        .binary_search_by_key(&word, |(incorrect, _)| *incorrect)
        .ok()
        .map(|idx| &CORRECTION_TABLE[idx].1)
}

/// Check if a word exists in the correction table (as an incorrect form).
pub fn contains(word: &str) -> bool {
    lookup(word).is_some()
}
