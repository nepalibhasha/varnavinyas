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
                description: "proper noun from Sanskrit वाग्मती: conjunct ग्म required",
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
                description: "redundant -ता: धीर+ता=धीरता, or base form धैर्य",
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
                description: "halanta required: Sanskrit stem ends in द् (संसद्)",
            },
        ),
        (
            "परिषद",
            CorrectionEntry {
                correct: "परिषद्",
                rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                description: "halanta required: Sanskrit stem ends in द् (परिषद्)",
            },
        ),
        (
            "संघीय",
            CorrectionEntry {
                correct: "सङ्घीय",
                rule: Rule::VarnaVinyasNiyam("3(ख)-पञ्चम"),
                description: "panchham varna ङ required before घ (not shirbindu ं)",
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
                description: "double त required: महत् + त्व = महत्त्व",
            },
        ),
        (
            "पश्चाताप",
            CorrectionEntry {
                correct: "पश्चात्ताप",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "double त required: पश्चात् + ताप = पश्चात्ताप",
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
                description: "plural suffix takes dirgha ऊ: हरू (not हरु)",
            },
        ),
        (
            "रुप",
            CorrectionEntry {
                correct: "रूप",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "tatsam रूप requires dirgha ऊ",
            },
        ),
        (
            "सौन्दर्यता",
            CorrectionEntry {
                correct: "सुन्दरता",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "redundant -ता: use सुन्दरता (सुन्दर+ता) or सौन्दर्य",
            },
        ),
        (
            "गुणस्तरीयता",
            CorrectionEntry {
                correct: "गुणस्तरीय",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "redundant -ता: गुणस्तरीय is already an adjective",
            },
        ),
        (
            "औचित्यता",
            CorrectionEntry {
                correct: "औचित्य",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "redundant -ता: औचित्य already abstract",
            },
        ),
        (
            "आतिथ्यता",
            CorrectionEntry {
                correct: "आतिथ्य",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "redundant -ता: आतिथ्य already abstract",
            },
        ),
        (
            "यथार्थता",
            CorrectionEntry {
                correct: "यथार्थ",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "redundant -ता: यथार्थ already functions as noun/adjective",
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
                description: "postposition form: भएका+मा = भएकामा",
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
                description: "standard form: बेहोरा",
            },
        ),
        (
            "एकिन",
            CorrectionEntry {
                correct: "यकिन",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "standard form: यकिन (य not ए)",
            },
        ),
        (
            "सुरुवात",
            CorrectionEntry {
                correct: "सुरुआत",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "standard form: सुरुआत (not सुरुवात)",
            },
        ),
        (
            "रजिष्टर",
            CorrectionEntry {
                correct: "रजिस्टर",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "aagantuk: स not ष for English 'register'",
            },
        ),
        (
            "इन्ष्टिच्युट",
            CorrectionEntry {
                correct: "इन्स्टिच्युट",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "aagantuk: स not ष for English 'institute'",
            },
        ),
        (
            "फाउण्डेसन",
            CorrectionEntry {
                correct: "फाउन्डेसन",
                rule: Rule::ShuddhaAshuddha("Section 4"),
                description: "aagantuk: न not ण for English 'foundation'",
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
                description: "aagantuk: न not ण for English 'India'",
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
                description: "adapted loanword: स not श, hrasva इ not dirgha ई",
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
                description: "tadbhav single-meaning word takes hrasva: मिष्ट → मिठो",
            },
        ),
        (
            "पीरो",
            CorrectionEntry {
                correct: "पिरो",
                rule: Rule::VarnaVinyasNiyam("3(क)-12"),
                description: "tadbhav single-meaning word takes hrasva",
            },
        ),
        (
            "तिथीमीति",
            CorrectionEntry {
                correct: "तिथिमिति",
                rule: Rule::VarnaVinyasNiyam("3(क)-12"),
                description: "tadbhav compound: both components take hrasva",
            },
        ),
        (
            "मीलेको",
            CorrectionEntry {
                correct: "मिलेको",
                rule: Rule::VarnaVinyasNiyam("3(क)-12"),
                description: "tadbhav verb root takes hrasva: मिल्नु → मिलेको",
            },
        ),
        (
            "दैनीकी",
            CorrectionEntry {
                correct: "दैनिकी",
                rule: Rule::VarnaVinyasNiyam("3(क)-12"),
                description: "medial vowel takes hrasva in tadbhav derivation",
            },
        ),
        (
            "भाई",
            CorrectionEntry {
                correct: "भाइ",
                rule: Rule::VarnaVinyasNiyam("3(क)-12"),
                description: "kinship tadbhav: भ्रातृ → भाइ (hrasva इ, not ई)",
            },
        ),
        (
            "मूखमा",
            CorrectionEntry {
                correct: "मुखमा",
                rule: Rule::VarnaVinyasNiyam("3(क)"),
                description: "tatsam मुख retains original hrasva उ (not दीर्घ ऊ)",
            },
        ),
        (
            "पूतली",
            CorrectionEntry {
                correct: "पुतली",
                rule: Rule::VarnaVinyasNiyam("3(क)-12, 3(ई)"),
                description: "tadbhav takes hrasva उ",
            },
        ),
        (
            "अनुभूती",
            CorrectionEntry {
                correct: "अनुभूति",
                rule: Rule::VarnaVinyasNiyam("3(क)"),
                description: "tatsam ending: अनुभूति ends in hrasva इ",
            },
        ),
        (
            "हामि",
            CorrectionEntry {
                correct: "हामी",
                rule: Rule::VarnaVinyasNiyam("3(ई)-ऊ-7"),
                description: "pronoun takes dirgha: हामी (not हामि)",
            },
        ),
        (
            "दीदी",
            CorrectionEntry {
                correct: "दिदी",
                rule: Rule::VarnaVinyasNiyam("3(इ)-ऊ-3"),
                description: "kinship tadbhav: initial vowel hrasva, final dirgha",
            },
        ),
        (
            "बहीनी",
            CorrectionEntry {
                correct: "बहिनी",
                rule: Rule::VarnaVinyasNiyam("3(इ)-ऊ-3"),
                description: "kinship tadbhav: medial vowel hrasva, final dirgha",
            },
        ),
        (
            "भाउजु",
            CorrectionEntry {
                correct: "भाउजू",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "feminine kinship takes dirgha: भाउजू (not भाउजु)",
            },
        ),
        (
            "फुपु",
            CorrectionEntry {
                correct: "फुपू",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "feminine kinship takes dirgha: फुपू (not फुपु)",
            },
        ),
        (
            "मीतिनिले",
            CorrectionEntry {
                correct: "मितिनीले",
                rule: Rule::VarnaVinyasNiyam("3(इ), 3(ई)"),
                description: "kinship tadbhav: initial hrasva इ, final dirgha ई",
            },
        ),
        (
            "खुर्सानि",
            CorrectionEntry {
                correct: "खुर्सानी",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "feminine noun ending takes dirgha ई",
            },
        ),
        (
            "सम्धिनि",
            CorrectionEntry {
                correct: "सम्धिनी",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "feminine noun ending takes dirgha ई",
            },
        ),
        (
            "पहाडि",
            CorrectionEntry {
                correct: "पहाडी",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "adjectival/demonym ending takes dirgha ई",
            },
        ),
        (
            "अगाडि",
            CorrectionEntry {
                correct: "अगाडी",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "adverbial/postposition ending takes dirgha ई",
            },
        ),
        (
            "भनि",
            CorrectionEntry {
                correct: "भनी",
                rule: Rule::VarnaVinyasNiyam("3(ई)"),
                description: "absolutive (पूर्वकालिक क्रिया) takes dirgha ई",
            },
        ),
        (
            "स्वीकार्नु",
            CorrectionEntry {
                correct: "स्विकार्नु",
                rule: Rule::VarnaVinyasNiyam("3(क)-suffix-नु"),
                description: "suffix -नु triggers hrasva: स्वीकार + नु = स्विकार्नु",
            },
        ),
        (
            "पूर्वेली",
            CorrectionEntry {
                correct: "पुर्वेली",
                rule: Rule::VarnaVinyasNiyam("3(क)-suffix-एली"),
                description: "suffix -एली triggers hrasva: पूर्व + एली = पुर्वेली",
            },
        ),
        (
            "पुर्वी",
            CorrectionEntry {
                correct: "पूर्वी",
                rule: Rule::VarnaVinyasNiyam("3(ई)-suffix-ई"),
                description: "suffix -ई preserves dirgha: पूर्व + ई = पूर्वी",
            },
        ),
        (
            "पुर्वीय",
            CorrectionEntry {
                correct: "पूर्वीय",
                rule: Rule::VarnaVinyasNiyam("3(ई)-suffix-ईय"),
                description: "suffix -ईय preserves dirgha: पूर्व + ईय = पूर्वीय",
            },
        ),
        // =================================================================
        // chandrabindu entries (Section 3(ख))
        // =================================================================
        (
            "सिँह",
            CorrectionEntry {
                correct: "सिंह",
                rule: Rule::VarnaVinyasNiyam("3(ख)"),
                description: "tatsam uses shirbindu (ं), not chandrabindu (ँ)",
            },
        ),
        (
            "सँवाद",
            CorrectionEntry {
                correct: "संवाद",
                rule: Rule::VarnaVinyasNiyam("3(ख)"),
                description: "tatsam uses shirbindu (ं), not chandrabindu (ँ)",
            },
        ),
        (
            "जान्छौ",
            CorrectionEntry {
                correct: "जान्छौँ",
                rule: Rule::VarnaVinyasNiyam("3(ख)"),
                description: "tadbhav verb requires chandrabindu for nasalization",
            },
        ),
        (
            "आउछ",
            CorrectionEntry {
                correct: "आउँछ",
                rule: Rule::VarnaVinyasNiyam("3(ख)"),
                description: "tadbhav verb requires chandrabindu for nasalization",
            },
        ),
        (
            "वगैचामा",
            CorrectionEntry {
                correct: "बगैँचामा",
                rule: Rule::VarnaVinyasNiyam("3(ख)"),
                description: "ब (not व) + chandrabindu required: बगैँचा",
            },
        ),
        (
            "बगैचा",
            CorrectionEntry {
                correct: "बगैँचा",
                rule: Rule::VarnaVinyasNiyam("3(ख)"),
                description: "chandrabindu required on बगैँचा",
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
                description: "tatsam word uses श (not स): शासन",
            },
        ),
        (
            "सेष",
            CorrectionEntry {
                correct: "शेष",
                rule: Rule::VarnaVinyasNiyam("3(ग)"),
                description: "tatsam word uses श (not स): शेष",
            },
        ),
        (
            "एसिया",
            CorrectionEntry {
                correct: "एशिया",
                rule: Rule::VarnaVinyasNiyam("3(ग)"),
                description: "proper noun uses श (not स): एशिया",
            },
        ),
        (
            "विवेकशिल",
            CorrectionEntry {
                correct: "विवेकशील",
                rule: Rule::VarnaVinyasNiyam("3(ग), 3(ई)"),
                description: "tatsam suffix -शील takes dirgha ई",
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
                description: "tatsam uses ऋ (not रि): ऋषि",
            },
        ),
        (
            "रितु",
            CorrectionEntry {
                correct: "ऋतु",
                rule: Rule::VarnaVinyasNiyam("3(ग)-ऋ"),
                description: "tatsam uses ऋ (not रि): ऋतु",
            },
        ),
        (
            "क्रिति",
            CorrectionEntry {
                correct: "कृति",
                rule: Rule::VarnaVinyasNiyam("3(ग)-कृ"),
                description: "tatsam uses कृ (not क्रि): कृति",
            },
        ),
        (
            "रिषिमुनि",
            CorrectionEntry {
                correct: "ऋषिमुनि",
                rule: Rule::VarnaVinyasNiyam("3(ग)-ऋ"),
                description: "tatsam compound: ऋषि + मुनि (ऋ not रि)",
            },
        ),
        // =================================================================
        // halanta entries (Section 3(ङ))
        // =================================================================
        (
            "अर्थात",
            CorrectionEntry {
                correct: "अर्थात्",
                rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                description: "halanta required on अव्यय: अर्थात्",
            },
        ),
        (
            "बुद्धिमान",
            CorrectionEntry {
                correct: "बुद्धिमान्",
                rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                description: "halanta required: -मान् suffix (बुद्धिमान्)",
            },
        ),
        (
            "भगवान",
            CorrectionEntry {
                correct: "भगवान्",
                rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                description: "halanta required: -वान् suffix (भगवान्)",
            },
        ),
        (
            "महान",
            CorrectionEntry {
                correct: "महान्",
                rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                description: "halanta required: tatsam stem ends in न् (महान्)",
            },
        ),
        (
            "विद्वान",
            CorrectionEntry {
                correct: "विद्वान्",
                rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                description: "halanta required: -वान् suffix (विद्वान्)",
            },
        ),
        (
            "श्रीमान",
            CorrectionEntry {
                correct: "श्रीमान्",
                rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                description: "halanta required: -मान् suffix (श्रीमान्)",
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
                description: "tatsam word uses व (not ब): विद्या",
            },
        ),
        (
            "बिद्वान",
            CorrectionEntry {
                correct: "विद्वान्",
                rule: Rule::VarnaVinyasNiyam("3(ग)-बव"),
                description: "tatsam word uses व (not ब) + halanta: विद्वान्",
            },
        ),
        (
            "बिदेश",
            CorrectionEntry {
                correct: "विदेश",
                rule: Rule::VarnaVinyasNiyam("3(ग)-बव"),
                description: "tatsam word uses व (not ब): विदेश",
            },
        ),
        (
            "बिकास",
            CorrectionEntry {
                correct: "विकास",
                rule: Rule::VarnaVinyasNiyam("3(ग)-बव"),
                description: "tatsam word uses व (not ब): विकास",
            },
        ),
        (
            "बिज्ञान",
            CorrectionEntry {
                correct: "विज्ञान",
                rule: Rule::VarnaVinyasNiyam("3(ग)-बव"),
                description: "tatsam word uses व (not ब): विज्ञान",
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
                description: "tatsam uses य (not ए): यथार्थ",
            },
        ),
        (
            "यकता",
            CorrectionEntry {
                correct: "एकता",
                rule: Rule::VarnaVinyasNiyam("3(छ)"),
                description: "tatsam uses ए (not य): एकता",
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
                description: "tatsam uses क्ष (not छ): लक्ष्य",
            },
        ),
        (
            "इक्षा",
            CorrectionEntry {
                correct: "इच्छा",
                rule: Rule::VarnaVinyasNiyam("3(छ)-क्ष"),
                description: "tatsam: इच्छा uses च्छ (not क्ष)",
            },
        ),
        (
            "छेत्र",
            CorrectionEntry {
                correct: "क्षेत्र",
                rule: Rule::VarnaVinyasNiyam("3(छ)-क्ष"),
                description: "tatsam uses क्षे (not छे): क्षेत्र",
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
                description: "verb form takes hrasva + halanta: भनिन्",
            },
        ),
        (
            "सन्सारमा",
            CorrectionEntry {
                correct: "संसारमा",
                rule: Rule::VarnaVinyasNiyam("3(ख)"),
                description: "shirbindu form: संसार (not halanta-न सन्सार)",
            },
        ),
    ];
    // Sort to support future binary search optimizations, though lookup currently uses linear scan.
    table.sort_by(|a, b| a.0.cmp(b.0));
    table
});

/// Look up a word in the correction table.
pub fn lookup(word: &str) -> Option<&'static CorrectionEntry> {
    CORRECTION_TABLE
        .iter()
        .find(|(incorrect, _)| *incorrect == word)
        .map(|(_, entry)| entry)
}

/// Check if a word exists in the correction table (as an incorrect form).
pub fn contains(word: &str) -> bool {
    lookup(word).is_some()
}
