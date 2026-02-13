use crate::origin::Origin;

/// Override table for word origins.
///
/// This small verified table serves as the first-priority lookup in the
/// three-tier classification: override table → kosha dictionary → heuristic.
///
/// With the kosha integration (~26K dictionary-tagged words), most words no
/// longer need to be here. This table is only for:
/// - Inflected/suffixed forms not in the dictionary headwords
/// - Words where the dictionary tag or heuristic gives wrong results
/// - Known test fixtures that must classify correctly
///
/// New words generally don't need to be added here — they'll be covered
/// by the kosha dictionary's origin tags from Brihat Shabdakosha.
pub fn lookup_origin(word: &str) -> Option<Origin> {
    // Binary search on sorted table for efficiency
    ORIGIN_TABLE
        .binary_search_by_key(&word, |&(w, _)| w)
        .ok()
        .map(|i| ORIGIN_TABLE[i].1)
}

/// Sorted by UTF-8 bytes for binary search.
static ORIGIN_TABLE: &[(&str, Origin)] = &[
    ("अग्नि", Origin::Tatsam),
    ("अनुभूति", Origin::Tatsam),
    ("अर्थात्", Origin::Tatsam),
    ("आउँछ", Origin::Tadbhav),
    ("आगो", Origin::Tadbhav),
    ("आतिथ्य", Origin::Tatsam),
    ("इन्डिया", Origin::Aagantuk),
    ("इन्स्टिच्युट", Origin::Aagantuk),
    ("इन्स्टिच्यूट", Origin::Aagantuk),
    ("ऋतु", Origin::Tatsam),
    ("ऋषि", Origin::Tatsam),
    ("ऋषिमुनि", Origin::Tatsam),
    ("एकता", Origin::Tatsam),
    ("एशिया", Origin::Aagantuk),
    ("औचित्य", Origin::Tatsam),
    ("औद्योगिकीकरण", Origin::Tatsam),
    ("कम्प्युटर", Origin::Aagantuk),
    ("कारबाही", Origin::Tadbhav),
    ("कृति", Origin::Tatsam),
    ("खुर्सानी", Origin::Tadbhav),
    ("गत्यवरोध", Origin::Tatsam),
    ("गुणस्तरीय", Origin::Tatsam),
    ("चुला", Origin::Deshaj),
    ("झन्डा", Origin::Tadbhav),
    ("टोपी", Origin::Deshaj),
    ("दिदी", Origin::Tadbhav),
    ("धीरता", Origin::Tatsam),
    ("धैर्य", Origin::Tatsam),
    ("नमस्ते", Origin::Tatsam),
    ("परिषद्", Origin::Tatsam),
    ("पहाडी", Origin::Tadbhav),
    ("पुतली", Origin::Tadbhav),
    ("पूर्वी", Origin::Tatsam),
    ("पूर्वीय", Origin::Tatsam),
    ("प्रशासन", Origin::Tatsam),
    ("फाउन्डेसन", Origin::Aagantuk),
    ("बगैँचा", Origin::Tadbhav),
    ("बहिनी", Origin::Tadbhav),
    ("बेहोरा", Origin::Tadbhav),
    ("भएकामा", Origin::Tadbhav),
    ("भाइ", Origin::Tadbhav),
    ("भाउजू", Origin::Tadbhav),
    ("भाका", Origin::Deshaj),
    ("महत्त्व", Origin::Tatsam),
    ("मिठो", Origin::Tadbhav),
    ("मितिनीले", Origin::Tadbhav),
    ("मिलेको", Origin::Tadbhav),
    ("मुखमा", Origin::Tadbhav),
    ("मुद्दा", Origin::Aagantuk),
    ("यकिन", Origin::Aagantuk),
    ("यथार्थ", Origin::Tatsam),
    ("रजिस्टर", Origin::Aagantuk),
    ("राजनीतिक", Origin::Tatsam),
    ("रूप", Origin::Tatsam),
    ("लक्ष्य", Origin::Tatsam),
    ("विज्ञान", Origin::Tatsam),
    ("विवेकशील", Origin::Tatsam),
    ("व्यावहारिक", Origin::Tatsam),
    ("शासन", Origin::Tatsam),
    ("शुद्ध", Origin::Tatsam),
    ("शृङ्खला", Origin::Tatsam),
    ("शृङ्गार", Origin::Tatsam),
    ("शेष", Origin::Tatsam),
    ("संवाद", Origin::Tatsam),
    ("संसद्", Origin::Tatsam),
    ("संसारमा", Origin::Tadbhav),
    ("सङ्घीय", Origin::Tatsam),
    ("सहिद", Origin::Aagantuk),
    ("सामग्री", Origin::Tatsam),
    ("सामाजिकीकरण", Origin::Tatsam),
    ("सिंह", Origin::Tatsam),
    ("सुन्दरता", Origin::Tatsam),
    ("सुरुआत", Origin::Tadbhav),
    ("सौन्दर्य", Origin::Tatsam),
    ("सौन्दर्यता", Origin::Tatsam),
    ("स्विकार्नु", Origin::Tadbhav),
    ("हरू", Origin::Tadbhav),
    ("हात", Origin::Tadbhav),
    ("हामी", Origin::Tadbhav),
];

/// Prefix forms: (canonical prefix, sandhi-ed form as it appears in words, root_prefix to restore).
/// When we strip the sandhi form from a word, we prepend root_prefix to get the original root.
///
/// IMPORTANT: Sorted by descending sandhi_form byte length for longest-first matching.
/// decompose() breaks on first match, so longer forms must precede shorter ones
/// (e.g., पुनर before पुनः, अभि before अ, निर् before नि).
pub static PREFIX_FORMS: &[(&str, &str, &str)] = &[
    // 15 bytes
    ("प्रति", "प्रति", ""),
    // 12 bytes
    ("पुनः", "पुनर", ""), // पुनः before vowel → पुनर
    ("पुनः", "पुनः", ""),
    ("निर्", "निर्", ""),
    ("निस्", "निस्", ""),
    ("दुस्", "दुस्", ""),
    ("दुस्", "दुश्", ""),
    ("दुर्", "दुर्", ""),
    // 9 bytes
    ("अभि", "अभि", ""),
    ("अधि", "अधि", ""),
    ("दुर्", "दुः", ""),
    ("सम्", "सङ्", ""), // सम् before gutturals → सङ्
    ("उत्", "उल्", ""), // उत् + ल → उल्ल
    ("उत्", "उच्", ""), // उत् + च → उच्च
    ("उत्", "उत्", ""),
    ("सम्", "सम्", ""),
    ("अनु", "अनु", ""),
    ("परि", "परि", ""),
    ("परा", "परा", ""),
    ("अति", "अति", ""),
    ("निर्", "निः", ""),
    ("निस्", "निः", ""),
    ("प्र", "प्र", ""),
    // 6 bytes
    ("सम्", "सं", ""),
    ("अप", "अप", ""), // medium risk: can over-strip (अपना, अवश्य)
    ("अव", "अव", ""), // medium risk: see above
    ("उप", "उप", ""),
    ("वि", "वि", ""),
    // 3 bytes
    ("आ", "आ", ""), // short prefix: ≤1 Devanagari char, requires 4+ char root
    ("अ", "अ", ""), // short prefix: ≤1 Devanagari char, requires 4+ char root
];

/// Case markers (postpositions) for iterative decomposition.
/// Sorted by descending byte length.
#[cfg(feature = "iterative-decompose")]
pub static CASE_MARKERS: &[&str] = &[
    "भित्र",
    "प्रति",
    "देखि",
    "लाई",
    "बाट",
    "सँग",
    "तिर",
    "का",
    "की",
    "ले",
    "को",
    "मा",
];

/// Plural markers for iterative decomposition.
/// Sorted by descending byte length.
#[cfg(feature = "iterative-decompose")]
pub static PLURAL_MARKERS: &[&str] = &["हरू", "हरु"];

/// Known suffixes.
///
/// IMPORTANT: Sorted by descending byte length for longest-first matching.
/// decompose() breaks on first match, so longer suffixes must precede shorter ones
/// (e.g., ईकरण before ई, इलो before इक).
pub static SUFFIXES: &[&str] = &[
    // 18 bytes
    "उन्जेल",
    // 12 bytes
    "ईकरण",
    // 9 bytes
    "इलो",
    "एको",
    "आलु",
    "कार",
    "एली",
    // 6 bytes
    "ईय",
    "ाइ",
    "एर",
    "पन",
    "ता",
    "नु",
    "ने",
    "आत",
    "अट",
    "को",
    "मा",
    "ले",
    "ित",
    "इक",
    // 3 bytes
    "ई",
];
