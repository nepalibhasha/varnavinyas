use crate::consonant::Varga;

/// Classification of a Devanagari character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharType {
    /// स्वर (vowel): अ आ इ ई उ ऊ ऋ ए ऐ ओ औ
    Svar,
    /// व्यञ्जन (consonant): क ख ग ... ह
    Vyanjan,
    /// मात्रा (vowel sign): ा ि ी ु ू ृ े ै ो ौ
    Matra,
    /// हलन्त (virama): ्
    Halanta,
    /// चन्द्रबिन्दु: ँ
    Chandrabindu,
    /// शिरबिन्दु (anusvara): ं
    Shirbindu,
    /// विसर्ग: ः
    Visarga,
    /// नुक्ता: ़
    Nukta,
    /// अवग्रह: ऽ
    Avagraha,
    /// अंक: ० १ २ ... ९
    Numeral,
    /// दण्ड: । ॥
    Danda,
    /// Other marks/signs not in above categories (abbreviation sign, high spacing dot, etc.)
    OtherMark,
}

/// Detailed classification of a Devanagari character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DevanagariChar {
    pub char_type: CharType,
    pub varga: Option<Varga>,
    pub is_panchham: bool,
}

impl DevanagariChar {
    fn new(char_type: CharType) -> Self {
        Self {
            char_type,
            varga: None,
            is_panchham: false,
        }
    }

    fn consonant(varga: Varga, is_panchham: bool) -> Self {
        Self {
            char_type: CharType::Vyanjan,
            varga: Some(varga),
            is_panchham,
        }
    }
}

/// Classify a character within the Devanagari Unicode block.
/// Returns `None` for non-Devanagari characters.
pub fn classify(c: char) -> Option<DevanagariChar> {
    match c {
        // U+0900: ऀ (Inverted Chandrabindu) — treat as Chandrabindu
        '\u{0900}' => Some(DevanagariChar::new(CharType::Chandrabindu)),
        // U+0901: ँ Chandrabindu
        '\u{0901}' => Some(DevanagariChar::new(CharType::Chandrabindu)),
        // U+0902: ं Anusvara (Shirbindu)
        '\u{0902}' => Some(DevanagariChar::new(CharType::Shirbindu)),
        // U+0903: ः Visarga
        '\u{0903}' => Some(DevanagariChar::new(CharType::Visarga)),

        // U+0904-U+0914: Vowels (Svar)
        // U+0904: ऄ (short A — rare)
        // U+0905: अ, U+0906: आ, U+0907: इ, U+0908: ई
        // U+0909: उ, U+090A: ऊ, U+090B: ऋ, U+090C: ऌ
        // U+090D: ऍ (candra E), U+090E: ऎ (short E)
        // U+090F: ए, U+0910: ऐ
        // U+0911: ऑ (candra O), U+0912: ऒ (short O)
        // U+0913: ओ, U+0914: औ
        '\u{0904}'..='\u{0914}' => Some(DevanagariChar::new(CharType::Svar)),

        // U+0915-U+0939: Consonants (Vyanjan)
        // Ka-varga: क ख ग घ ङ (U+0915-U+0919)
        '\u{0915}' => Some(DevanagariChar::consonant(Varga::KaVarga, false)), // क
        '\u{0916}' => Some(DevanagariChar::consonant(Varga::KaVarga, false)), // ख
        '\u{0917}' => Some(DevanagariChar::consonant(Varga::KaVarga, false)), // ग
        '\u{0918}' => Some(DevanagariChar::consonant(Varga::KaVarga, false)), // घ
        '\u{0919}' => Some(DevanagariChar::consonant(Varga::KaVarga, true)),  // ङ (panchham)

        // Cha-varga: च छ ज झ ञ (U+091A-U+091E)
        '\u{091A}' => Some(DevanagariChar::consonant(Varga::ChaVarga, false)), // च
        '\u{091B}' => Some(DevanagariChar::consonant(Varga::ChaVarga, false)), // छ
        '\u{091C}' => Some(DevanagariChar::consonant(Varga::ChaVarga, false)), // ज
        '\u{091D}' => Some(DevanagariChar::consonant(Varga::ChaVarga, false)), // झ
        '\u{091E}' => Some(DevanagariChar::consonant(Varga::ChaVarga, true)),  // ञ (panchham)

        // Ta-varga (retroflex): ट ठ ड ढ ण (U+091F-U+0923)
        '\u{091F}' => Some(DevanagariChar::consonant(Varga::TaVarga, false)), // ट
        '\u{0920}' => Some(DevanagariChar::consonant(Varga::TaVarga, false)), // ठ
        '\u{0921}' => Some(DevanagariChar::consonant(Varga::TaVarga, false)), // ड
        '\u{0922}' => Some(DevanagariChar::consonant(Varga::TaVarga, false)), // ढ
        '\u{0923}' => Some(DevanagariChar::consonant(Varga::TaVarga, true)),  // ण (panchham)

        // Ta-varga2 (dental): त थ द ध न (U+0924-U+0928)
        '\u{0924}' => Some(DevanagariChar::consonant(Varga::TaVarga2, false)), // त
        '\u{0925}' => Some(DevanagariChar::consonant(Varga::TaVarga2, false)), // थ
        '\u{0926}' => Some(DevanagariChar::consonant(Varga::TaVarga2, false)), // द
        '\u{0927}' => Some(DevanagariChar::consonant(Varga::TaVarga2, false)), // ध
        '\u{0928}' => Some(DevanagariChar::consonant(Varga::TaVarga2, true)),  // न (panchham)

        // Pa-varga: प फ ब भ म (U+092A-U+092E)
        // U+0929: ऩ (NNNA) — nukta form, treat as consonant
        '\u{0929}' => Some(DevanagariChar::consonant(Varga::TaVarga2, false)), // ऩ (NNNA)
        '\u{092A}' => Some(DevanagariChar::consonant(Varga::PaVarga, false)),  // प
        '\u{092B}' => Some(DevanagariChar::consonant(Varga::PaVarga, false)),  // फ
        '\u{092C}' => Some(DevanagariChar::consonant(Varga::PaVarga, false)),  // ब
        '\u{092D}' => Some(DevanagariChar::consonant(Varga::PaVarga, false)),  // भ
        '\u{092E}' => Some(DevanagariChar::consonant(Varga::PaVarga, true)),   // म (panchham)

        // Antastha (semivowels): य र ल व (U+092F-U+0932, U+0935)
        '\u{092F}' => Some(DevanagariChar::consonant(Varga::Antastha, false)), // य
        '\u{0930}' => Some(DevanagariChar::consonant(Varga::Antastha, false)), // र
        // U+0931: ऱ (RRA) — nukta form
        '\u{0931}' => Some(DevanagariChar::consonant(Varga::Antastha, false)), // ऱ
        '\u{0932}' => Some(DevanagariChar::consonant(Varga::Antastha, false)), // ल
        // U+0933: ळ (LLA)
        '\u{0933}' => Some(DevanagariChar::consonant(Varga::Antastha, false)), // ळ
        // U+0934: ऴ (LLLA) — rare
        '\u{0934}' => Some(DevanagariChar::consonant(Varga::Antastha, false)), // ऴ
        '\u{0935}' => Some(DevanagariChar::consonant(Varga::Antastha, false)), // व

        // Ushma (sibilants): श ष स (U+0936-U+0938)
        '\u{0936}' => Some(DevanagariChar::consonant(Varga::Ushma, false)), // श
        '\u{0937}' => Some(DevanagariChar::consonant(Varga::Ushma, false)), // ष
        '\u{0938}' => Some(DevanagariChar::consonant(Varga::Ushma, false)), // स

        // U+0939: ह — Other
        '\u{0939}' => Some(DevanagariChar::consonant(Varga::Other, false)), // ह

        // U+093A-U+093B: reserved/rare combining marks — treat as Matra
        '\u{093A}'..='\u{093B}' => Some(DevanagariChar::new(CharType::Matra)),

        // U+093C: ़ Nukta
        '\u{093C}' => Some(DevanagariChar::new(CharType::Nukta)),

        // U+093D: ऽ Avagraha
        '\u{093D}' => Some(DevanagariChar::new(CharType::Avagraha)),

        // U+093E-U+094C: Matras (vowel signs)
        // ा ि ी ु ू ृ ॄ ॅ ॆ े ै ॉ ॊ ो ौ
        '\u{093E}'..='\u{094C}' => Some(DevanagariChar::new(CharType::Matra)),

        // U+094D: ् Virama (Halanta)
        '\u{094D}' => Some(DevanagariChar::new(CharType::Halanta)),

        // U+094E-U+094F: Prishthamatra E, AW — Matra
        '\u{094E}'..='\u{094F}' => Some(DevanagariChar::new(CharType::Matra)),

        // U+0950: ॐ OM — treat as Svar (sacred syllable)
        '\u{0950}' => Some(DevanagariChar::new(CharType::Svar)),

        // U+0951-U+0954: Vedic accent marks — treat as Matra (combining marks)
        '\u{0951}'..='\u{0954}' => Some(DevanagariChar::new(CharType::Matra)),

        // U+0955-U+0957: various combining marks
        '\u{0955}'..='\u{0957}' => Some(DevanagariChar::new(CharType::Matra)),

        // U+0958-U+095F: Nukta consonant forms (क़ ख़ ग़ ज़ ड़ ढ़ फ़ य़)
        '\u{0958}' => Some(DevanagariChar::consonant(Varga::KaVarga, false)), // क़
        '\u{0959}' => Some(DevanagariChar::consonant(Varga::KaVarga, false)), // ख़
        '\u{095A}' => Some(DevanagariChar::consonant(Varga::KaVarga, false)), // ग़
        '\u{095B}' => Some(DevanagariChar::consonant(Varga::ChaVarga, false)), // ज़
        '\u{095C}' => Some(DevanagariChar::consonant(Varga::TaVarga, false)), // ड़
        '\u{095D}' => Some(DevanagariChar::consonant(Varga::TaVarga, false)), // ढ़
        '\u{095E}' => Some(DevanagariChar::consonant(Varga::PaVarga, false)), // फ़
        '\u{095F}' => Some(DevanagariChar::consonant(Varga::Antastha, false)), // य़

        // U+0960-U+0961: Vocalic vowels ॠ ॡ
        '\u{0960}'..='\u{0961}' => Some(DevanagariChar::new(CharType::Svar)),

        // U+0962-U+0963: Vocalic matras ॢ ॣ
        '\u{0962}'..='\u{0963}' => Some(DevanagariChar::new(CharType::Matra)),

        // U+0964: । Danda
        '\u{0964}' => Some(DevanagariChar::new(CharType::Danda)),
        // U+0965: ॥ Double Danda
        '\u{0965}' => Some(DevanagariChar::new(CharType::Danda)),

        // U+0966-U+096F: Numerals ०-९
        '\u{0966}'..='\u{096F}' => Some(DevanagariChar::new(CharType::Numeral)),

        // U+0970: ॰ Abbreviation sign
        '\u{0970}' => Some(DevanagariChar::new(CharType::OtherMark)),

        // U+0971: ॱ High spacing dot
        '\u{0971}' => Some(DevanagariChar::new(CharType::OtherMark)),

        // U+0972-U+097F: Extended Devanagari (various regional vowels/consonants)
        // U+0972-U+0977: Regional vowels
        '\u{0972}'..='\u{0977}' => Some(DevanagariChar::new(CharType::Svar)),
        // U+0978-U+097F: Regional consonants
        '\u{0978}'..='\u{097F}' => Some(DevanagariChar::consonant(Varga::Other, false)),

        _ => None,
    }
}

/// Check if the character is a Devanagari vowel (स्वर).
pub fn is_svar(c: char) -> bool {
    matches!(classify(c), Some(dc) if dc.char_type == CharType::Svar)
}

/// Check if the character is a Devanagari consonant (व्यञ्जन).
pub fn is_vyanjan(c: char) -> bool {
    matches!(classify(c), Some(dc) if dc.char_type == CharType::Vyanjan)
}

/// Check if the character is a vowel sign (मात्रा).
pub fn is_matra(c: char) -> bool {
    matches!(classify(c), Some(dc) if dc.char_type == CharType::Matra)
}

/// Check if the character is a virama (हलन्त).
pub fn is_halanta(c: char) -> bool {
    matches!(classify(c), Some(dc) if dc.char_type == CharType::Halanta)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vowels() {
        let vowels = ['अ', 'आ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ', 'ए', 'ऐ', 'ओ', 'औ'];
        for v in vowels {
            let dc = classify(v).unwrap_or_else(|| panic!("classify({v}) returned None"));
            assert_eq!(dc.char_type, CharType::Svar, "expected Svar for {v}");
        }
    }

    #[test]
    fn test_consonants() {
        let consonants = [
            'क', 'ख', 'ग', 'घ', 'ङ', 'च', 'छ', 'ज', 'झ', 'ञ', 'ट', 'ठ', 'ड', 'ढ', 'ण', 'त', 'थ',
            'द', 'ध', 'न', 'प', 'फ', 'ब', 'भ', 'म', 'य', 'र', 'ल', 'व', 'श', 'ष', 'स', 'ह',
        ];
        for c in consonants {
            let dc = classify(c).unwrap_or_else(|| panic!("classify({c}) returned None"));
            assert_eq!(dc.char_type, CharType::Vyanjan, "expected Vyanjan for {c}");
        }
    }

    #[test]
    fn test_matras() {
        let matras = ['ा', 'ि', 'ी', 'ु', 'ू', 'ृ', 'े', 'ै', 'ो', 'ौ'];
        for m in matras {
            let dc = classify(m).unwrap_or_else(|| panic!("classify({m}) returned None"));
            assert_eq!(dc.char_type, CharType::Matra, "expected Matra for {m}");
        }
    }

    #[test]
    fn test_halanta() {
        let dc = classify('्').unwrap();
        assert_eq!(dc.char_type, CharType::Halanta);
    }

    #[test]
    fn test_chandrabindu_shirbindu_visarga() {
        assert_eq!(classify('ँ').unwrap().char_type, CharType::Chandrabindu);
        assert_eq!(classify('ं').unwrap().char_type, CharType::Shirbindu);
        assert_eq!(classify('ः').unwrap().char_type, CharType::Visarga);
    }

    #[test]
    fn test_nukta() {
        assert_eq!(classify('़').unwrap().char_type, CharType::Nukta);
    }

    #[test]
    fn test_avagraha() {
        assert_eq!(classify('ऽ').unwrap().char_type, CharType::Avagraha);
    }

    #[test]
    fn test_numerals() {
        for c in '०'..='९' {
            let dc = classify(c).unwrap_or_else(|| panic!("classify({c}) returned None"));
            assert_eq!(dc.char_type, CharType::Numeral, "expected Numeral for {c}");
        }
    }

    #[test]
    fn test_dandas() {
        assert_eq!(classify('।').unwrap().char_type, CharType::Danda);
        assert_eq!(classify('॥').unwrap().char_type, CharType::Danda);
    }

    #[test]
    fn test_non_devanagari_returns_none() {
        assert!(classify('A').is_none());
        assert!(classify('z').is_none());
        assert!(classify('中').is_none());
        assert!(classify('0').is_none());
        assert!(classify(' ').is_none());
    }

    #[test]
    fn test_boolean_helpers() {
        assert!(is_svar('अ'));
        assert!(is_svar('ऐ'));
        assert!(!is_svar('क'));

        assert!(is_vyanjan('क'));
        assert!(is_vyanjan('ह'));
        assert!(!is_vyanjan('अ'));

        assert!(is_matra('ा'));
        assert!(is_matra('ै'));
        assert!(!is_matra('अ'));

        assert!(is_halanta('्'));
        assert!(!is_halanta('क'));
    }

    #[test]
    fn test_all_codepoints_in_range_classified() {
        // Every codepoint in U+0900-U+097F should be classified
        for cp in 0x0900u32..=0x097Fu32 {
            if let Some(c) = char::from_u32(cp) {
                assert!(classify(c).is_some(), "U+{cp:04X} ({c}) not classified");
            }
        }
    }

    #[test]
    fn test_panchham_in_classify() {
        let panchhams = ['ङ', 'ञ', 'ण', 'न', 'म'];
        for c in panchhams {
            let dc = classify(c).unwrap();
            assert!(dc.is_panchham, "expected is_panchham for {c}");
        }

        let non_panchhams = ['क', 'ग', 'च', 'ट', 'प', 'य', 'श', 'ह'];
        for c in non_panchhams {
            let dc = classify(c).unwrap();
            assert!(!dc.is_panchham, "unexpected is_panchham for {c}");
        }
    }

    #[test]
    fn test_varga_assignment() {
        assert_eq!(classify('क').unwrap().varga, Some(Varga::KaVarga));
        assert_eq!(classify('च').unwrap().varga, Some(Varga::ChaVarga));
        assert_eq!(classify('ट').unwrap().varga, Some(Varga::TaVarga));
        assert_eq!(classify('त').unwrap().varga, Some(Varga::TaVarga2));
        assert_eq!(classify('प').unwrap().varga, Some(Varga::PaVarga));
        assert_eq!(classify('य').unwrap().varga, Some(Varga::Antastha));
        assert_eq!(classify('श').unwrap().varga, Some(Varga::Ushma));
        assert_eq!(classify('ह').unwrap().varga, Some(Varga::Other));
    }

    #[test]
    fn test_nukta_consonants() {
        // Use precomposed codepoints U+0958-U+095F
        let nukta_consonants = [
            '\u{0958}', '\u{0959}', '\u{095A}', '\u{095B}', '\u{095C}', '\u{095D}', '\u{095E}',
            '\u{095F}',
        ];
        for c in nukta_consonants {
            let dc = classify(c);
            assert!(
                dc.is_some(),
                "nukta consonant U+{:04X} not classified",
                c as u32
            );
            assert_eq!(
                dc.unwrap().char_type,
                CharType::Vyanjan,
                "nukta consonant U+{:04X} should be Vyanjan",
                c as u32
            );
        }
    }
}
