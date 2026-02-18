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
                description: "स्वर सन्धि: अति + अधिक = अत्यधिक (अत्याधिक होइन)",
            },
        ),
        (
            "उपरोक्त",
            CorrectionEntry {
                correct: "उपर्युक्त",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "मानक रूप: उपरोक्त होइन, उपर्युक्त",
            },
        ),
        (
            "राजनैतिक",
            CorrectionEntry {
                correct: "राजनीतिक",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "राजनीति + क = राजनीतिक (राजनैतिक होइन)",
            },
        ),
        (
            "उल्लेखित",
            CorrectionEntry {
                correct: "उल्लिखित",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "उत् + लिखित = उल्लिखित (उल्लेखित होइन)",
            },
        ),
        (
            "बागमती",
            CorrectionEntry {
                correct: "बाग्मती",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "व्यक्तिवाचक नाम संस्कृत वाग्मतीबाट: संयुक्ताक्षर ग्म अनिवार्य",
            },
        ),
        (
            "पुनरावलोकन",
            CorrectionEntry {
                correct: "पुनरवलोकन",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "पुनर् + अवलोकन = पुनरवलोकन (पुनरावलोकन होइन)",
            },
        ),
        (
            "व्यवहारिक",
            CorrectionEntry {
                correct: "व्यावहारिक",
                rule: Rule::VarnaVinyasNiyam("3(क)-इक-प्रत्यय"),
                description: "इक प्रत्ययमा आदिवृद्धि: व्यवहार + इक = व्यावहारिक",
            },
        ),
        (
            "धैर्यता",
            CorrectionEntry {
                correct: "धीरता",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "-ता अनावश्यक: धीर+ता=धीरता, वा आधाररूप धैर्य",
            },
        ),
        (
            "प्रसाशन",
            CorrectionEntry {
                correct: "प्रशासन",
                rule: Rule::ShuddhaAshuddha("Section 4, Section 3(ग)"),
                description: "प्र + शासन = प्रशासन (श हुन्छ, स होइन; स्वरक्रम सही)",
            },
        ),
        (
            "संसद",
            CorrectionEntry {
                correct: "संसद्",
                rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                description: "हलन्त अनिवार्य: संस्कृत मूलको अन्त्य द् मा हुन्छ (संसद्)",
            },
        ),
        (
            "परिषद",
            CorrectionEntry {
                correct: "परिषद्",
                rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                description: "हलन्त अनिवार्य: संस्कृत मूलको अन्त्य द् मा हुन्छ (परिषद्)",
            },
        ),
        (
            "संघीय",
            CorrectionEntry {
                correct: "सङ्घीय",
                rule: Rule::VarnaVinyasNiyam("3(ख)-पञ्चम"),
                description: "घ अघि पञ्चम वर्ण ङ अनिवार्य (शिरबिन्दु ं होइन)",
            },
        ),
        (
            "पुनर्स्थापना",
            CorrectionEntry {
                correct: "पुनःस्थापना",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "विसर्ग कायम: पुनः + स्थापना (पुनर् होइन)",
            },
        ),
        (
            "पुनर्संरचना",
            CorrectionEntry {
                correct: "पुनःसंरचना",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "विसर्ग कायम: पुनः + संरचना (पुनर् होइन)",
            },
        ),
        (
            "महत्व",
            CorrectionEntry {
                correct: "महत्त्व",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "दोहोरो त अनिवार्य: महत् + त्व = महत्त्व",
            },
        ),
        (
            "पश्चाताप",
            CorrectionEntry {
                correct: "पश्चात्ताप",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "दोहोरो त अनिवार्य: पश्चात् + ताप = पश्चात्ताप",
            },
        ),
        (
            "मुद्धा",
            CorrectionEntry {
                correct: "मुद्दा",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "गलत द्वित्व: द्द (द्ध होइन)",
            },
        ),
        (
            "श्रृङ्गार",
            CorrectionEntry {
                correct: "शृङ्गार",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "शृ होइन श्रृ: श + ृ = शृ (रको संलग्नता हुँदैन)",
            },
        ),
        (
            "श्रृङ्खला",
            CorrectionEntry {
                correct: "शृङ्खला",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "शृ होइन श्रृ: श + ृ = शृ (रको संलग्नता हुँदैन)",
            },
        ),
        (
            "हरु",
            CorrectionEntry {
                correct: "हरू",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "बहुवचन प्रत्ययमा दीर्घ ऊ हुन्छ: हरू (हरु होइन)",
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
                description: "-ता अनावश्यक: सुन्दरता (सुन्दर+ता) वा सौन्दर्य प्रयोग गर्नुपर्छ",
            },
        ),
        (
            "गुणस्तरीयता",
            CorrectionEntry {
                correct: "गुणस्तरीय",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "-ता अनावश्यक: गुणस्तरीय आफैं विशेषण हो",
            },
        ),
        (
            "औचित्यता",
            CorrectionEntry {
                correct: "औचित्य",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "-ता अनावश्यक: औचित्य आफैं भाववाचक रूप हो",
            },
        ),
        (
            "आतिथ्यता",
            CorrectionEntry {
                correct: "आतिथ्य",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "-ता अनावश्यक: आतिथ्य आफैं भाववाचक रूप हो",
            },
        ),
        (
            "यथार्थता",
            CorrectionEntry {
                correct: "यथार्थ",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "-ता अनावश्यक: यथार्थ आफैं नामपद/विशेषणका रूपमा चल्छ",
            },
        ),
        (
            "कार्यबाही",
            CorrectionEntry {
                correct: "कारबाही",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "कार+बाही (र्य बिना): कारबाही",
            },
        ),
        (
            "वृक्षारोपण",
            CorrectionEntry {
                correct: "वृक्षरोपण",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "वृक्ष+रोपण = वृक्षरोपण (अतिरिक्त आ हुँदैन)",
            },
        ),
        (
            "गत्यावरोध",
            CorrectionEntry {
                correct: "गत्यवरोध",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "गति+अवरोध = गत्यवरोध (यण् सन्धि, अतिरिक्त आ हुँदैन)",
            },
        ),
        (
            "सामाग्री",
            CorrectionEntry {
                correct: "सामग्री",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "अतिरिक्त आ हुँदैन: सामग्री",
            },
        ),
        (
            "भएकोमा",
            CorrectionEntry {
                correct: "भएकामा",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "नामयोगी रूप: भएका+मा = भएकामा",
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
                description: "अ+क्षम = अक्षम (अतिरिक्त स हुँदैन)",
            },
        ),
        (
            "सपाङ्ग",
            CorrectionEntry {
                correct: "साङ्ग",
                rule: Rule::ShuddhaAshuddha("Section 4, Section 4(ख)"),
                description: "स+अङ्ग = साङ्ग (दीर्घ सन्धि)",
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
                description: "मानक रूप: यकिन (य होइन ए)",
            },
        ),
        (
            "सुरुवात",
            CorrectionEntry {
                correct: "सुरुआत",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "मानक रूप: सुरुआत (सुरुवात होइन)",
            },
        ),
        (
            "रजिष्टर",
            CorrectionEntry {
                correct: "रजिस्टर",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "आगन्तुक: स होइन ष 'register' का लागि",
            },
        ),
        (
            "इन्ष्टिच्युट",
            CorrectionEntry {
                correct: "इन्स्टिच्युट",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "आगन्तुक: स होइन ष 'institute' का लागि",
            },
        ),
        (
            "फाउण्डेसन",
            CorrectionEntry {
                correct: "फाउन्डेसन",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "आगन्तुक शब्दमा न प्रयोग हुन्छ, ण होइन: फाउन्डेसन",
            },
        ),
        (
            "झण्डा",
            CorrectionEntry {
                correct: "झन्डा",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "ड अघि न प्रयोग हुन्छ, ण होइन: झन्डा",
            },
        ),
        (
            "इण्डिया",
            CorrectionEntry {
                correct: "इन्डिया",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "आगन्तुक शब्दमा न प्रयोग हुन्छ, ण होइन: इन्डिया",
            },
        ),
        (
            "इंग्ल्याण्ड",
            CorrectionEntry {
                correct: "इङ्ग्ल्यान्ड",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "ग अघि पञ्चम वर्ण ङ र न प्रयोग हुन्छ, ण होइन: इङ्ग्ल्यान्ड",
            },
        ),
        (
            "शहीद",
            CorrectionEntry {
                correct: "सहिद",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "रूपान्तरित आगन्तुक शब्दमा स प्रयोग हुन्छ (श होइन), ह्रस्व इ हुन्छ (दीर्घ ई होइन)",
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
                description: "नातागोता तद्भव शब्दमा ह्रस्व इ हुन्छ: भ्रातृ → भाइ",
            },
        ),
        (
            "मूखमा",
            CorrectionEntry {
                correct: "मुखमा",
                rule: Rule::VarnaVinyasNiyam("3(क)"),
                description: "तत्सम शब्द मुख मा मूल ह्रस्व उ नै रहन्छ (दीर्घ ऊ होइन)",
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
                description: "तत्सम शब्द अनुभूति को अन्त्य ह्रस्व इमा हुन्छ",
            },
        ),
        (
            "हामि",
            CorrectionEntry {
                correct: "हामी",
                rule: Rule::VarnaVinyasNiyam("3(ई)-ऊ-7"),
                description: "सर्वनाममा दीर्घ हुन्छ: हामी (हामि होइन)",
            },
        ),
        (
            "दीदी",
            CorrectionEntry {
                correct: "दिदी",
                rule: Rule::VarnaVinyasNiyam("3(इ)-ऊ-3"),
                description: "नातागोता तद्भव शब्दमा सुरुको स्वर ह्रस्व र अन्त्य दीर्घ हुन्छ",
            },
        ),
        (
            "बहीनी",
            CorrectionEntry {
                correct: "बहिनी",
                rule: Rule::VarnaVinyasNiyam("3(इ)-ऊ-3"),
                description: "नातागोता तद्भव शब्दमा शब्दमध्यको स्वर ह्रस्व र अन्त्य दीर्घ हुन्छ",
            },
        ),
        (
            "भाउजु",
            CorrectionEntry {
                correct: "भाउजू",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "स्त्रीलिङ्गी नातागोता शब्दमा दीर्घ हुन्छ: भाउजू (भाउजु होइन)",
            },
        ),
        (
            "फुपु",
            CorrectionEntry {
                correct: "फुपू",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "स्त्रीलिङ्गी नातागोता शब्दमा दीर्घ हुन्छ: फुपू (फुपु होइन)",
            },
        ),
        (
            "मीतिनिले",
            CorrectionEntry {
                correct: "मितिनीले",
                rule: Rule::VarnaVinyasNiyam("3(इ), 3(ई)"),
                description: "नातागोता तद्भव शब्दमा सुरुमा ह्रस्व इ र अन्त्यमा दीर्घ ई हुन्छ",
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
                description: "विशेषण तथा स्थानबोधक शब्दको अन्त्यमा दीर्घ ई हुन्छ",
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
                rule: Rule::VarnaVinyasNiyam("3(क)-प्रत्यय-नु"),
                description: "प्रत्यय -नु ले ह्रस्व: स्वीकार + नु = स्विकार्नु",
            },
        ),
        (
            "पूर्वेली",
            CorrectionEntry {
                correct: "पुर्वेली",
                rule: Rule::VarnaVinyasNiyam("3(क)-प्रत्यय-एली"),
                description: "प्रत्यय -एली ले ह्रस्व: पूर्व + एली = पुर्वेली",
            },
        ),
        (
            "पुर्वी",
            CorrectionEntry {
                correct: "पूर्वी",
                rule: Rule::VarnaVinyasNiyam("3(ई)-प्रत्यय-ई"),
                description: "प्रत्यय -ई ले दीर्घ: पूर्व + ई = पूर्वी",
            },
        ),
        (
            "पुर्वीय",
            CorrectionEntry {
                correct: "पूर्वीय",
                rule: Rule::VarnaVinyasNiyam("3(ई)-प्रत्यय-ईय"),
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
                description: "तत्सम शब्दमा शिरबिन्दु (ं) प्रयोग हुन्छ, चन्द्रबिन्दु (ँ) होइन",
            },
        ),
        (
            "सँवाद",
            CorrectionEntry {
                correct: "संवाद",
                rule: Rule::VarnaVinyasNiyam("3(ख)"),
                description: "तत्सम शब्दमा शिरबिन्दु (ं) प्रयोग हुन्छ, चन्द्रबिन्दु (ँ) होइन",
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
                description: "ब (व होइन) + चन्द्रबिन्दु अनिवार्य: बगैँचा",
            },
        ),
        (
            "बगैचा",
            CorrectionEntry {
                correct: "बगैँचा",
                rule: Rule::VarnaVinyasNiyam("3(ख)"),
                description: "बगैँचा शब्दमा चन्द्रबिन्दु अनिवार्य हुन्छ",
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
                description: "तत्सम शब्दमा श (स होइन): शासन",
            },
        ),
        (
            "सेष",
            CorrectionEntry {
                correct: "शेष",
                rule: Rule::VarnaVinyasNiyam("3(ग)"),
                description: "तत्सम शब्दमा श (स होइन): शेष",
            },
        ),
        (
            "एशिया",
            CorrectionEntry {
                correct: "एसिया",
                rule: Rule::VarnaVinyasNiyam("3(ग)"),
                description: "आगन्तुक शब्दमा स (श होइन): एसिया",
            },
        ),
        (
            "विवेकशिल",
            CorrectionEntry {
                correct: "विवेकशील",
                rule: Rule::VarnaVinyasNiyam("3(ग), 3(ई)"),
                description: "तत्सम प्रत्यय -शील मा दीर्घ ई हुन्छ",
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
                description: "तत्सम शब्दमा ऋ (रि होइन): ऋषि",
            },
        ),
        (
            "रितु",
            CorrectionEntry {
                correct: "ऋतु",
                rule: Rule::VarnaVinyasNiyam("3(ग)-ऋ"),
                description: "तत्सम शब्दमा ऋ (रि होइन): ऋतु",
            },
        ),
        (
            "क्रिति",
            CorrectionEntry {
                correct: "कृति",
                rule: Rule::VarnaVinyasNiyam("3(ग)-कृ"),
                description: "तत्सम शब्दमा कृ (क्रि होइन): कृति",
            },
        ),
        (
            "रिषिमुनि",
            CorrectionEntry {
                correct: "ऋषिमुनि",
                rule: Rule::VarnaVinyasNiyam("3(ग)-ऋ"),
                description: "तत्सम समास: ऋषि + मुनि (ऋ, रि होइन)",
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
                description: "अव्यय अर्थात् मा हलन्त अनिवार्य हुन्छ",
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
                description: "हलन्त अनिवार्य: तत्सम मूलको अन्त्य न् मा हुन्छ (महान्)",
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
                description: "तत्सम शब्दमा व (ब होइन): विद्या",
            },
        ),
        (
            "बिद्वान",
            CorrectionEntry {
                correct: "विद्वान्",
                rule: Rule::VarnaVinyasNiyam("3(ग)-बव"),
                description: "तत्सम शब्दमा व (ब होइन) र हलन्त: विद्वान्",
            },
        ),
        (
            "बिदेश",
            CorrectionEntry {
                correct: "विदेश",
                rule: Rule::VarnaVinyasNiyam("3(ग)-बव"),
                description: "तत्सम शब्दमा व (ब होइन): विदेश",
            },
        ),
        (
            "बिकास",
            CorrectionEntry {
                correct: "विकास",
                rule: Rule::VarnaVinyasNiyam("3(ग)-बव"),
                description: "तत्सम शब्दमा व (ब होइन): विकास",
            },
        ),
        (
            "बिज्ञान",
            CorrectionEntry {
                correct: "विज्ञान",
                rule: Rule::VarnaVinyasNiyam("3(ग)-बव"),
                description: "तत्सम शब्दमा व (ब होइन): विज्ञान",
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
                description: "तत्सम शब्दमा य (ए होइन): यथार्थ",
            },
        ),
        (
            "यकता",
            CorrectionEntry {
                correct: "एकता",
                rule: Rule::VarnaVinyasNiyam("3(छ)"),
                description: "तत्सम शब्दमा ए (य होइन): एकता",
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
                description: "तत्सम शब्दमा क्ष (छ होइन): लक्ष्य",
            },
        ),
        (
            "इक्षा",
            CorrectionEntry {
                correct: "इच्छा",
                rule: Rule::VarnaVinyasNiyam("3(छ)-क्ष"),
                description: "तत्सम शब्द इच्छा मा च्छ हुन्छ (क्ष होइन)",
            },
        ),
        (
            "छेत्र",
            CorrectionEntry {
                correct: "क्षेत्र",
                rule: Rule::VarnaVinyasNiyam("3(छ)-क्ष"),
                description: "तत्सम शब्दमा क्षे (छे होइन): क्षेत्र",
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
                description: "क्रियापदको रूप भनिन् मा ह्रस्व + हलन्त हुन्छ",
            },
        ),
        (
            "सन्सारमा",
            CorrectionEntry {
                correct: "संसारमा",
                rule: Rule::VarnaVinyasNiyam("3(ख)"),
                description: "संसार मा शिरबिन्दु रूप हुन्छ (हलन्त-न सन्सार होइन)",
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
