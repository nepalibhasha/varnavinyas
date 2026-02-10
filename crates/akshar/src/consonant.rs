/// Consonant group (varga).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Varga {
    /// क ख ग घ ङ
    KaVarga,
    /// च छ ज झ ञ
    ChaVarga,
    /// ट ठ ड ढ ण (retroflex)
    TaVarga,
    /// त थ द ध न (dental)
    TaVarga2,
    /// प फ ब भ म
    PaVarga,
    /// य र ल व (semivowels)
    Antastha,
    /// श ष स (sibilants)
    Ushma,
    /// ह, and other consonants
    Other,
}

/// Get the consonant's varga classification.
/// Returns `None` for non-consonant characters.
pub fn varga(c: char) -> Option<Varga> {
    match c {
        'क' | 'ख' | 'ग' | 'घ' | 'ङ' => Some(Varga::KaVarga),
        'च' | 'छ' | 'ज' | 'झ' | 'ञ' => Some(Varga::ChaVarga),
        'ट' | 'ठ' | 'ड' | 'ढ' | 'ण' => Some(Varga::TaVarga),
        'त' | 'थ' | 'द' | 'ध' | 'न' => Some(Varga::TaVarga2),
        'प' | 'फ' | 'ब' | 'भ' | 'म' => Some(Varga::PaVarga),
        'य' | 'र' | 'ल' | 'व' | 'ळ' => Some(Varga::Antastha),
        'श' | 'ष' | 'स' => Some(Varga::Ushma),
        'ह' => Some(Varga::Other),
        // Nukta consonant forms
        '\u{0958}' | '\u{0959}' | '\u{095A}' => Some(Varga::KaVarga), // क़ ख़ ग़
        '\u{095B}' => Some(Varga::ChaVarga),                          // ज़
        '\u{095C}' | '\u{095D}' => Some(Varga::TaVarga),              // ड़ ढ़
        '\u{095E}' => Some(Varga::PaVarga),                           // फ़
        '\u{095F}' => Some(Varga::Antastha),                          // य़
        _ => None,
    }
}

/// Check if the character is a panchham varna (fifth consonant of each varga).
/// ङ ञ ण न म
pub fn is_panchham(c: char) -> bool {
    matches!(c, 'ङ' | 'ञ' | 'ण' | 'न' | 'म')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varga_ka() {
        for c in ['क', 'ख', 'ग', 'घ', 'ङ'] {
            assert_eq!(varga(c), Some(Varga::KaVarga), "expected KaVarga for {c}");
        }
    }

    #[test]
    fn test_varga_cha() {
        for c in ['च', 'छ', 'ज', 'झ', 'ञ'] {
            assert_eq!(varga(c), Some(Varga::ChaVarga), "expected ChaVarga for {c}");
        }
    }

    #[test]
    fn test_varga_ta_retroflex() {
        for c in ['ट', 'ठ', 'ड', 'ढ', 'ण'] {
            assert_eq!(varga(c), Some(Varga::TaVarga), "expected TaVarga for {c}");
        }
    }

    #[test]
    fn test_varga_ta_dental() {
        for c in ['त', 'थ', 'द', 'ध', 'न'] {
            assert_eq!(varga(c), Some(Varga::TaVarga2), "expected TaVarga2 for {c}");
        }
    }

    #[test]
    fn test_varga_pa() {
        for c in ['प', 'फ', 'ब', 'भ', 'म'] {
            assert_eq!(varga(c), Some(Varga::PaVarga), "expected PaVarga for {c}");
        }
    }

    #[test]
    fn test_varga_antastha() {
        for c in ['य', 'र', 'ल', 'व'] {
            assert_eq!(varga(c), Some(Varga::Antastha), "expected Antastha for {c}");
        }
    }

    #[test]
    fn test_varga_ushma() {
        for c in ['श', 'ष', 'स'] {
            assert_eq!(varga(c), Some(Varga::Ushma), "expected Ushma for {c}");
        }
    }

    #[test]
    fn test_varga_other() {
        assert_eq!(varga('ह'), Some(Varga::Other));
    }

    #[test]
    fn test_varga_non_consonant() {
        assert_eq!(varga('अ'), None);
        assert_eq!(varga('ा'), None);
        assert_eq!(varga('A'), None);
    }

    #[test]
    fn test_is_panchham() {
        assert!(is_panchham('ङ'));
        assert!(is_panchham('ञ'));
        assert!(is_panchham('ण'));
        assert!(is_panchham('न'));
        assert!(is_panchham('म'));
    }

    #[test]
    fn test_is_not_panchham() {
        assert!(!is_panchham('क'));
        assert!(!is_panchham('ग'));
        assert!(!is_panchham('च'));
        assert!(!is_panchham('ट'));
        assert!(!is_panchham('प'));
        assert!(!is_panchham('य'));
        assert!(!is_panchham('श'));
        assert!(!is_panchham('ह'));
        assert!(!is_panchham('A'));
    }
}
