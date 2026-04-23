/// `(घ) पदयोग र पदवियोगसम्बन्धी नियम` model.
///
/// Source:
/// - `docs/Notices-pages-77-99.md` lines 364–400
///
/// Design:
/// - Each Academy subrule is represented explicitly (`PadayogRule`).
/// - Each subrule has one or more concrete rewrite patterns.
/// - Checker applies all patterns with boundary/span guards.
///
/// Concrete rewrite pair with a terse explanation.
#[derive(Debug, Clone, Copy)]
pub struct PhraseRewrite {
    pub incorrect: &'static str,
    pub correct: &'static str,
    pub explanation: &'static str,
}

/// One Academy `(घ)` subrule bucket.
#[derive(Debug, Clone, Copy)]
pub struct PadayogRule {
    /// Stable citation (e.g., `3(घ)-पदयोग-४`).
    pub code: &'static str,
    /// Human readable rule label.
    pub label: &'static str,
    /// Concrete rewrite patterns for this subrule.
    pub rewrites: &'static [PhraseRewrite],
}

const P_1_UPASARGA_JOIN: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "बद नाम",
        correct: "बदनाम",
        explanation: "उपसर्ग जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "बे काम",
        correct: "बेकाम",
        explanation: "उपसर्ग जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "अधि पति",
        correct: "अधिपति",
        explanation: "उपसर्ग जोडेर लेख्नुपर्छ",
    },
];

const P_3_VIBHAKTI_JOIN: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "राम ले",
        correct: "रामले",
        explanation: "विभक्ति जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "भाइ लाई",
        correct: "भाइलाई",
        explanation: "विभक्ति जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "देश को",
        correct: "देशको",
        explanation: "विभक्ति जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "घर बाट",
        correct: "घरबाट",
        explanation: "विभक्ति जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "म देखि",
        correct: "मदेखि",
        explanation: "विभक्ति जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "देश मा",
        correct: "देशमा",
        explanation: "विभक्ति जोडेर लेख्नुपर्छ",
    },
];

const P_4_NAMAYOGI_JOIN: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "घर तिर",
        correct: "घरतिर",
        explanation: "नामयोगी जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "तिमी भन्दा",
        correct: "तिमीभन्दा",
        explanation: "नामयोगी जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "कोठा भित्र",
        correct: "कोठाभित्र",
        explanation: "नामयोगी जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "बिना काम",
        correct: "बिनाकाम",
        explanation: "नामयोगी जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "म सँग",
        correct: "मसँग",
        explanation: "नामयोगी जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "आज्ञा अनुसार",
        correct: "आज्ञाअनुसार",
        explanation: "नामयोगी जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "हामी बाहेक",
        correct: "हामीबाहेक",
        explanation: "नामयोगी जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "त्यस अन्तर्गत",
        correct: "त्यसअन्तर्गत",
        explanation: "नामयोगी जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "भने बमोजिम",
        correct: "भनेबमोजिम",
        explanation: "नामयोगी जोडेर लेख्नुपर्छ",
    },
];

const P_5_SAMASTA_JOIN: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "गाई गोठ",
        correct: "गाईगोठ",
        explanation: "समस्त शब्द जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "आमा छोरी",
        correct: "आमाछोरी",
        explanation: "समस्त शब्द जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "दाजु भाइ",
        correct: "दाजुभाइ",
        explanation: "समस्त शब्द जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "देश निकाला",
        correct: "देशनिकाला",
        explanation: "समस्त शब्द जोडेर लेख्नुपर्छ",
    },
];

const P_6_NIRARTHAK_DWITVA_JOIN: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "फटा फट",
        correct: "फटाफट",
        explanation: "निरर्थक द्वित्व जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "सरा सर",
        correct: "सरासर",
        explanation: "निरर्थक द्वित्व जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "हस्याङ फस्याङ",
        correct: "हस्याङफस्याङ",
        explanation: "निरर्थक द्वित्व जोडेर लेख्नुपर्छ",
    },
];

const P_7_AKARAN_N_JOIN: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "न जाऊ",
        correct: "नजाऊ",
        explanation: "अकरण 'न' जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "खाँ दैन",
        correct: "खाँदैन",
        explanation: "अकरण 'न' जोडेर लेख्नुपर्छ",
    },
];

const P_8_MILIT_KRIYAPAD_JOIN: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "गइ सक्यो",
        correct: "गइसक्यो",
        explanation: "मिलित क्रियापद जोडेर",
    },
    PhraseRewrite {
        incorrect: "गरि हाल",
        correct: "गरिहाल",
        explanation: "मिलित क्रियापद जोडेर",
    },
    PhraseRewrite {
        incorrect: "पढ्नु पर्छ",
        correct: "पढ्नुपर्छ",
        explanation: "मिलित क्रियापद जोडेर",
    },
    PhraseRewrite {
        incorrect: "सुन्नु पर्छ",
        correct: "सुन्नुपर्छ",
        explanation: "मिलित क्रियापद जोडेर",
    },
];

const P_9_SAMYOJAK_JOIN: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "किन भने",
        correct: "किनभने",
        explanation: "संयोजक जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "ताप नि",
        correct: "तापनि",
        explanation: "संयोजक जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "यद्य पि",
        correct: "यद्यपि",
        explanation: "संयोजक जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "तथा पि",
        correct: "तथापि",
        explanation: "संयोजक जोडेर लेख्नुपर्छ",
    },
];

const P_10_OTA_VARGA_SAMBANDHI_JOIN: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "तीन ओटा",
        correct: "तीनओटा",
        explanation: "ओटा पद जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "शिक्षक वर्ग",
        correct: "शिक्षकवर्ग",
        explanation: "वर्ग पद जोडेर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "ज्ञान सम्बन्धी",
        correct: "ज्ञानसम्बन्धी",
        explanation: "सम्बन्धी पद जोडेर लेख्नुपर्छ",
    },
];

const P_11_SARAH_JOIN: &[PhraseRewrite] = &[PhraseRewrite {
    incorrect: "बुद्धि सरह",
    correct: "बुद्धिसरह",
    explanation: "तुलना पद जोडेर लेख्नुपर्छ",
}];

const V_2_NAMAYOGI_SPLIT: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "उसकाअगाडि",
        correct: "उसका अगाडि",
        explanation: "नामयोगी छुट्याएर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "मुखकापछाडि",
        correct: "मुखका पछाडि",
        explanation: "नामयोगी छुट्याएर लेख्नुपर्छ",
    },
];

const V_3_LAGI_NIMTI_SPLIT: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "देशकालागि",
        correct: "देशका लागि",
        explanation: "लागि अघि विभक्ति छुट्याउनुपर्छ",
    },
    PhraseRewrite {
        incorrect: "मेरानिम्ति",
        correct: "मेरा निम्ति",
        explanation: "निम्ति अघि विभक्ति छुट्याउनुपर्छ",
    },
    PhraseRewrite {
        incorrect: "रामकामा",
        correct: "रामका मा",
        explanation: "दुई विभक्ति छुट्याउनुपर्छ",
    },
];

const V_4_NIPAT_SPLIT: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "भनत",
        correct: "भन त",
        explanation: "निपात छुट्याएर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "आऊनि",
        correct: "आऊ नि",
        explanation: "निपात छुट्याएर लेख्नुपर्छ",
    },
];

const V_5_N_SAMYOJAK_SPLIT: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "नयता",
        correct: "न यता",
        explanation: "'न' संयोजक छुट्याएर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "नउता",
        correct: "न उता",
        explanation: "'न' संयोजक छुट्याएर लेख्नुपर्छ",
    },
];

const V_6_APURNA_PURNA_KRIYAPAD_SPLIT: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "जाँदैछ",
        correct: "जाँदै छ",
        explanation: "पक्षसूचक क्रियापद छुट्याएर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "आएकोछ",
        correct: "आएको छ",
        explanation: "पक्षसूचक क्रियापद छुट्याएर लेख्नुपर्छ",
    },
];

const V_7_NE_CHA_SPLIT: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "खानेछ",
        correct: "खाने छ",
        explanation: "'ने छ' छुट्याएर लेख्नुपर्छ",
    },
    PhraseRewrite {
        incorrect: "आउनेछ",
        correct: "आउने छ",
        explanation: "'ने छ' छुट्याएर लेख्नुपर्छ",
    },
];

const V_8_SAMYUKTA_KRIYA_NIPAT_SPLIT: &[PhraseRewrite] = &[PhraseRewrite {
    incorrect: "पढ्नुनैपर्छ",
    correct: "पढ्नु नै पर्छ",
    explanation: "संयुक्त क्रियामा निपात छुट्याएर लेख्नुपर्छ",
}];

const V_9_NU_N_PACHHI_KRIYA_SPLIT: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "जानुछ",
        correct: "जानु छ",
        explanation: "'नु' पछि क्रियापद छुट्याएर",
    },
    PhraseRewrite {
        incorrect: "भन्नसक्छ",
        correct: "भन्न सक्छ",
        explanation: "'न' पछि क्रियापद छुट्याएर",
    },
];

const V_10_SARTHAK_DWITVA_SPLIT: &[PhraseRewrite] = &[];

const V_11_JANA_THARI_SPLIT: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "चारजना",
        correct: "चार जना",
        explanation: "कोटिकर पद छुट्याएर",
    },
    PhraseRewrite {
        incorrect: "सयथरी",
        correct: "सय थरी",
        explanation: "कोटिकर पद छुट्याएर",
    },
];

const V_12_SHIRSHA_NAM_SPLIT: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "कास्कीजिल्ला",
        correct: "कास्की जिल्ला",
        explanation: "शीर्ष नाम छुट्याएर",
    },
    PhraseRewrite {
        incorrect: "कोसीनदी",
        correct: "कोसी नदी",
        explanation: "शीर्ष नाम छुट्याएर",
    },
];

const V_13_VISHESHAN_NAM_SPLIT: &[PhraseRewrite] = &[
    PhraseRewrite {
        incorrect: "भित्तेपात्रो",
        correct: "भित्ते पात्रो",
        explanation: "विशेषण-नाम छुट्याएर",
    },
    PhraseRewrite {
        incorrect: "शुभसमाचार",
        correct: "शुभ समाचार",
        explanation: "विशेषण-नाम छुट्याएर",
    },
    PhraseRewrite {
        incorrect: "खानेपानी",
        correct: "खाने पानी",
        explanation: "विशेषण-नाम छुट्याएर",
    },
];

pub const PADAYOG_PADABIYOG_RULES: &[PadayogRule] = &[
    PadayogRule {
        code: "3(घ)-पदयोग-१",
        label: "उपसर्ग जोडेर लेख्नुपर्छ",
        rewrites: P_1_UPASARGA_JOIN,
    },
    PadayogRule {
        code: "3(घ)-पदयोग-३",
        label: "विभक्ति जोडेर लेख्नुपर्छ",
        rewrites: P_3_VIBHAKTI_JOIN,
    },
    PadayogRule {
        code: "3(घ)-पदयोग-४",
        label: "नामयोगी जोडेर लेख्नुपर्छ",
        rewrites: P_4_NAMAYOGI_JOIN,
    },
    PadayogRule {
        code: "3(घ)-पदयोग-५",
        label: "समस्त शब्द जोडेर लेख्नुपर्छ",
        rewrites: P_5_SAMASTA_JOIN,
    },
    PadayogRule {
        code: "3(घ)-पदयोग-६",
        label: "निरर्थक द्वित्व जोडेर लेख्नुपर्छ",
        rewrites: P_6_NIRARTHAK_DWITVA_JOIN,
    },
    PadayogRule {
        code: "3(घ)-पदयोग-७",
        label: "अकरण 'न' जोडेर लेख्नुपर्छ",
        rewrites: P_7_AKARAN_N_JOIN,
    },
    PadayogRule {
        code: "3(घ)-पदयोग-८",
        label: "मिलित क्रियापद जोडेर लेख्नुपर्छ",
        rewrites: P_8_MILIT_KRIYAPAD_JOIN,
    },
    PadayogRule {
        code: "3(घ)-पदयोग-९",
        label: "संयोजक जोडेर लेख्नुपर्छ",
        rewrites: P_9_SAMYOJAK_JOIN,
    },
    PadayogRule {
        code: "3(घ)-पदयोग-१०",
        label: "ओटा/वर्ग/सम्बन्धी पद जोडेर लेख्नुपर्छ",
        rewrites: P_10_OTA_VARGA_SAMBANDHI_JOIN,
    },
    PadayogRule {
        code: "3(घ)-पदयोग-११",
        label: "सरह तुलना पद जोडेर लेख्नुपर्छ",
        rewrites: P_11_SARAH_JOIN,
    },
    PadayogRule {
        code: "3(घ)-पदवियोग-२",
        label: "विभक्तिपछि नामयोगी छुट्याएर लेख्नुपर्छ",
        rewrites: V_2_NAMAYOGI_SPLIT,
    },
    PadayogRule {
        code: "3(घ)-पदवियोग-३",
        label: "लागि/निम्ति/दोहोरो विभक्ति छुट्याएर",
        rewrites: V_3_LAGI_NIMTI_SPLIT,
    },
    PadayogRule {
        code: "3(घ)-पदवियोग-४",
        label: "निपात छुट्याएर लेख्नुपर्छ",
        rewrites: V_4_NIPAT_SPLIT,
    },
    PadayogRule {
        code: "3(घ)-पदवियोग-५",
        label: "'न' संयोजक छुट्याएर लेख्नुपर्छ",
        rewrites: V_5_N_SAMYOJAK_SPLIT,
    },
    PadayogRule {
        code: "3(घ)-पदवियोग-६",
        label: "अपूर्ण/पूर्ण पक्ष क्रियापद छुट्याएर",
        rewrites: V_6_APURNA_PURNA_KRIYAPAD_SPLIT,
    },
    PadayogRule {
        code: "3(घ)-पदवियोग-७",
        label: "'ने छ' क्रियापद छुट्याएर",
        rewrites: V_7_NE_CHA_SPLIT,
    },
    PadayogRule {
        code: "3(घ)-पदवियोग-८",
        label: "संयुक्त क्रियामा निपात छुट्याएर",
        rewrites: V_8_SAMYUKTA_KRIYA_NIPAT_SPLIT,
    },
    PadayogRule {
        code: "3(घ)-पदवियोग-९",
        label: "'नु'/'न' पछि क्रियापद छुट्याएर",
        rewrites: V_9_NU_N_PACHHI_KRIYA_SPLIT,
    },
    PadayogRule {
        code: "3(घ)-पदवियोग-१०",
        label: "सार्थक द्वित्व छुट्याएर",
        rewrites: V_10_SARTHAK_DWITVA_SPLIT,
    },
    PadayogRule {
        code: "3(घ)-पदवियोग-११",
        label: "जना/थरी जस्ता कोटिकर पद छुट्याएर",
        rewrites: V_11_JANA_THARI_SPLIT,
    },
    PadayogRule {
        code: "3(घ)-पदवियोग-१२",
        label: "शीर्ष नाम छुट्याएर लेख्नुपर्छ",
        rewrites: V_12_SHIRSHA_NAM_SPLIT,
    },
    PadayogRule {
        code: "3(घ)-पदवियोग-१३",
        label: "विशेषण र नाम छुट्याएर लेख्नुपर्छ",
        rewrites: V_13_VISHESHAN_NAM_SPLIT,
    },
];

// Coverage note:
// - पदयोग-२ (प्रत्यय) and पदवियोग-१ (प्रत्येक शब्द छुट्याएर) are broad
//   principles; they need token/morph-aware general rules beyond fixed pairs.
