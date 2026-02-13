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

/// Position within a varga (1-indexed).
/// 1st=voiceless unaspirated, 2nd=voiceless aspirated,
/// 3rd=voiced unaspirated, 4th=voiced aspirated, 5th=nasal (panchham).
pub fn varga_position(c: char) -> Option<u8> {
    match c {
        'क' | 'च' | 'ट' | 'त' | 'प' => Some(1),
        'ख' | 'छ' | 'ठ' | 'थ' | 'फ' => Some(2),
        'ग' | 'ज' | 'ड' | 'द' | 'ब' => Some(3),
        'घ' | 'झ' | 'ढ' | 'ध' | 'भ' => Some(4),
        'ङ' | 'ञ' | 'ण' | 'न' | 'म' => Some(5),
        _ => None,
    }
}

/// Check if a consonant is voiceless (1st or 2nd of its varga, or sibilants).
pub fn is_voiceless(c: char) -> bool {
    matches!(varga_position(c), Some(1 | 2)) || matches!(c, 'श' | 'ष' | 'स')
}

/// Check if a consonant is voiced (3rd, 4th, 5th of its varga, semivowels, or ह).
pub fn is_voiced(c: char) -> bool {
    matches!(varga_position(c), Some(3..=5)) || matches!(c, 'य' | 'र' | 'ल' | 'व' | 'ह')
}

/// Get the panchham (nasal, 5th consonant) of a given varga.
pub fn panchham_of(v: Varga) -> Option<char> {
    match v {
        Varga::KaVarga => Some('ङ'),
        Varga::ChaVarga => Some('ञ'),
        Varga::TaVarga => Some('ण'),
        Varga::TaVarga2 => Some('न'),
        Varga::PaVarga => Some('म'),
        _ => None,
    }
}

/// Get the voiced counterpart of a voiceless stop (position 1→3, 2→4).
pub fn voiced_counterpart(c: char) -> Option<char> {
    match c {
        'क' => Some('ग'),
        'ख' => Some('घ'),
        'च' => Some('ज'),
        'छ' => Some('झ'),
        'ट' => Some('ड'),
        'ठ' => Some('ढ'),
        'त' => Some('द'),
        'थ' => Some('ध'),
        'प' => Some('ब'),
        'फ' => Some('भ'),
        _ => None,
    }
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

    #[test]
    fn test_voiceless() {
        for c in [
            'क', 'ख', 'च', 'छ', 'ट', 'ठ', 'त', 'थ', 'प', 'फ', 'श', 'ष', 'स',
        ] {
            assert!(is_voiceless(c), "expected voiceless for {c}");
        }
        for c in [
            'ग', 'घ', 'ज', 'झ', 'ड', 'ढ', 'द', 'ध', 'ब', 'भ', 'म', 'य', 'र', 'ह',
        ] {
            assert!(!is_voiceless(c), "unexpected voiceless for {c}");
        }
    }

    #[test]
    fn test_voiced() {
        for c in [
            'ग', 'घ', 'ङ', 'ज', 'झ', 'ञ', 'ड', 'ढ', 'ण', 'द', 'ध', 'न', 'ब', 'भ', 'म', 'य', 'र',
            'ल', 'व', 'ह',
        ] {
            assert!(is_voiced(c), "expected voiced for {c}");
        }
        for c in ['क', 'ख', 'च', 'छ', 'ट', 'ठ', 'त', 'थ', 'प', 'फ'] {
            assert!(!is_voiced(c), "unexpected voiced for {c}");
        }
    }

    #[test]
    fn test_panchham_of() {
        assert_eq!(panchham_of(Varga::KaVarga), Some('ङ'));
        assert_eq!(panchham_of(Varga::ChaVarga), Some('ञ'));
        assert_eq!(panchham_of(Varga::TaVarga), Some('ण'));
        assert_eq!(panchham_of(Varga::TaVarga2), Some('न'));
        assert_eq!(panchham_of(Varga::PaVarga), Some('म'));
        assert_eq!(panchham_of(Varga::Antastha), None);
        assert_eq!(panchham_of(Varga::Ushma), None);
        assert_eq!(panchham_of(Varga::Other), None);
    }

    #[test]
    fn test_voiced_counterpart() {
        assert_eq!(voiced_counterpart('क'), Some('ग'));
        assert_eq!(voiced_counterpart('ख'), Some('घ'));
        assert_eq!(voiced_counterpart('त'), Some('द'));
        assert_eq!(voiced_counterpart('प'), Some('ब'));
        assert_eq!(voiced_counterpart('ग'), None); // already voiced
        assert_eq!(voiced_counterpart('म'), None); // nasal, not a stop
        assert_eq!(voiced_counterpart('य'), None); // semivowel
    }

    #[test]
    fn test_varga_position() {
        assert_eq!(varga_position('क'), Some(1));
        assert_eq!(varga_position('ख'), Some(2));
        assert_eq!(varga_position('ग'), Some(3));
        assert_eq!(varga_position('घ'), Some(4));
        assert_eq!(varga_position('ङ'), Some(5));
        assert_eq!(varga_position('य'), None); // semivowel, no position
        assert_eq!(varga_position('अ'), None); // vowel
    }
}
