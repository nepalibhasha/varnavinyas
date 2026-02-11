use crate::origin::Origin;

/// Known word → origin mappings.
/// These are verified against the Academy standard.
pub fn lookup_origin(word: &str) -> Option<Origin> {
    // Binary search on sorted table for efficiency
    ORIGIN_TABLE
        .binary_search_by_key(&word, |&(w, _)| w)
        .ok()
        .map(|i| ORIGIN_TABLE[i].1)
}

/// Sorted table of known word origins.
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
    ("भाइ", Origin::Tadbhav),
    ("भाउजू", Origin::Tadbhav),
    ("भाका", Origin::Deshaj),
    ("भएकामा", Origin::Tadbhav),
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
    ("शृङ्गार", Origin::Tatsam),
    ("शृङ्खला", Origin::Tatsam),
    ("शेष", Origin::Tatsam),
    ("संवाद", Origin::Tatsam),
    ("संसद्", Origin::Tatsam),
    ("संसारमा", Origin::Tadbhav),
    ("सहिद", Origin::Aagantuk),
    ("सामग्री", Origin::Tatsam),
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
pub static PREFIX_FORMS: &[(&str, &str, &str)] = &[
    // Standard prefixes (no transformation of root initial)
    ("प्र", "प्र", ""),
    // Consonant assimilation: उत् + ल → उल्ल (ल is doubled; strip उल् and root starts with ल)
    ("उत्", "उल्", ""),
    // उत् + च → उच्च
    ("उत्", "उच्", ""),
    ("उत्", "उत्", ""),
    ("सम्", "सम्", ""),
    ("सम्", "सं", ""),
    ("अभि", "अभि", ""),
    ("अनु", "अनु", ""),
    ("परि", "परि", ""),
    ("वि", "वि", ""),
    ("निर्", "निर्", ""),
    ("निर्", "निः", ""),
    ("निस्", "निस्", ""),
    ("निस्", "निः", ""),
    ("अ", "अ", ""),
    ("पुनः", "पुनः", ""),
    ("पुनः", "पुनर", ""), // पुनः before vowel → पुनर
];

/// Known suffixes.
pub static SUFFIXES: &[&str] = &[
    "ईकरण",
    "ता",
    "ई",
    "ईय",
    "नु",
    "एली",
    "ने",
    "को",
    "मा",
    "ले",
    "ित",
    "इक",
];
