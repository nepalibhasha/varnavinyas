/// Vowel length classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SvarType {
    /// ह्रस्व (short): अ इ उ ऋ
    Hrasva,
    /// दीर्घ (long): आ ई ऊ ए ऐ ओ औ
    Dirgha,
}

/// Determine the vowel length of a svar or matra.
/// Returns `None` for non-vowel characters.
pub fn svar_type(c: char) -> Option<SvarType> {
    match c {
        // Hrasva svars
        'अ' | 'इ' | 'उ' | 'ऋ' | 'ऌ' => Some(SvarType::Hrasva),
        // Dirgha svars
        'आ' | 'ई' | 'ऊ' | 'ॠ' | 'ॡ' | 'ए' | 'ऐ' | 'ओ' | 'औ' => {
            Some(SvarType::Dirgha)
        }
        // Hrasva matras
        'ि' | 'ु' | 'ृ' | 'ॢ' => Some(SvarType::Hrasva),
        // Dirgha matras (ा maps to आ which is dirgha)
        'ा' | 'ी' | 'ू' | 'ॄ' | 'ॣ' | 'े' | 'ै' | 'ो' | 'ौ' => {
            Some(SvarType::Dirgha)
        }
        _ => None,
    }
}

/// Convert a hrasva vowel/matra to its dirgha counterpart.
/// इ→ई, उ→ऊ, ि→ी, ु→ू
/// Returns `None` if the input is not a convertible hrasva.
pub fn hrasva_to_dirgha(c: char) -> Option<char> {
    match c {
        'इ' => Some('ई'),
        'उ' => Some('ऊ'),
        'ऋ' => Some('ॠ'),
        'ऌ' => Some('ॡ'),
        'ि' => Some('ी'),
        'ु' => Some('ू'),
        'ृ' => Some('ॄ'),
        'ॢ' => Some('ॣ'),
        _ => None,
    }
}

/// Convert a dirgha vowel/matra to its hrasva counterpart.
/// ई→इ, ऊ→उ, ी→ि, ू→ु
/// Returns `None` if the input is not a convertible dirgha.
pub fn dirgha_to_hrasva(c: char) -> Option<char> {
    match c {
        'ई' => Some('इ'),
        'ऊ' => Some('उ'),
        'ॠ' => Some('ऋ'),
        'ॡ' => Some('ऌ'),
        'ी' => Some('ि'),
        'ू' => Some('ु'),
        'ॄ' => Some('ृ'),
        'ॣ' => Some('ॢ'),
        _ => None,
    }
}

/// Get the matra form of a svar (vowel).
/// अ→None (inherent vowel, no matra), आ→ा, इ→ि, etc.
pub fn svar_to_matra(c: char) -> Option<char> {
    match c {
        'अ' => None, // inherent vowel — no matra form
        'आ' => Some('ा'),
        'इ' => Some('ि'),
        'ई' => Some('ी'),
        'उ' => Some('ु'),
        'ऊ' => Some('ू'),
        'ऋ' => Some('ृ'),
        'ॠ' => Some('ॄ'),
        'ऌ' => Some('ॢ'),
        'ॡ' => Some('ॣ'),
        'ए' => Some('े'),
        'ऐ' => Some('ै'),
        'ओ' => Some('ो'),
        'औ' => Some('ौ'),
        _ => None,
    }
}

/// Get the svar form of a matra (vowel sign).
/// ा→आ, ि→इ, ी→ई, etc.
pub fn matra_to_svar(c: char) -> Option<char> {
    match c {
        'ा' => Some('आ'),
        'ि' => Some('इ'),
        'ी' => Some('ई'),
        'ु' => Some('उ'),
        'ू' => Some('ऊ'),
        'ृ' => Some('ऋ'),
        'ॄ' => Some('ॠ'),
        'ॢ' => Some('ऌ'),
        'ॣ' => Some('ॡ'),
        'े' => Some('ए'),
        'ै' => Some('ऐ'),
        'ो' => Some('ओ'),
        'ौ' => Some('औ'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svar_type_hrasva() {
        assert_eq!(svar_type('अ'), Some(SvarType::Hrasva));
        assert_eq!(svar_type('इ'), Some(SvarType::Hrasva));
        assert_eq!(svar_type('उ'), Some(SvarType::Hrasva));
        assert_eq!(svar_type('ऋ'), Some(SvarType::Hrasva));
    }

    #[test]
    fn test_svar_type_dirgha() {
        assert_eq!(svar_type('आ'), Some(SvarType::Dirgha));
        assert_eq!(svar_type('ई'), Some(SvarType::Dirgha));
        assert_eq!(svar_type('ऊ'), Some(SvarType::Dirgha));
        assert_eq!(svar_type('ए'), Some(SvarType::Dirgha));
        assert_eq!(svar_type('ऐ'), Some(SvarType::Dirgha));
        assert_eq!(svar_type('ओ'), Some(SvarType::Dirgha));
        assert_eq!(svar_type('औ'), Some(SvarType::Dirgha));
    }

    #[test]
    fn test_svar_type_matras() {
        assert_eq!(svar_type('ि'), Some(SvarType::Hrasva));
        assert_eq!(svar_type('ु'), Some(SvarType::Hrasva));
        assert_eq!(svar_type('ृ'), Some(SvarType::Hrasva));
        assert_eq!(svar_type('ा'), Some(SvarType::Dirgha));
        assert_eq!(svar_type('ी'), Some(SvarType::Dirgha));
        assert_eq!(svar_type('ू'), Some(SvarType::Dirgha));
        assert_eq!(svar_type('े'), Some(SvarType::Dirgha));
        assert_eq!(svar_type('ै'), Some(SvarType::Dirgha));
        assert_eq!(svar_type('ो'), Some(SvarType::Dirgha));
        assert_eq!(svar_type('ौ'), Some(SvarType::Dirgha));
    }

    #[test]
    fn test_svar_type_non_vowel() {
        assert_eq!(svar_type('क'), None);
        assert_eq!(svar_type('A'), None);
        assert_eq!(svar_type('्'), None);
    }

    #[test]
    fn test_hrasva_to_dirgha() {
        assert_eq!(hrasva_to_dirgha('इ'), Some('ई'));
        assert_eq!(hrasva_to_dirgha('उ'), Some('ऊ'));
        assert_eq!(hrasva_to_dirgha('ि'), Some('ी'));
        assert_eq!(hrasva_to_dirgha('ु'), Some('ू'));
        assert_eq!(hrasva_to_dirgha('ऋ'), Some('ॠ'));
        assert_eq!(hrasva_to_dirgha('ृ'), Some('ॄ'));
        // Not convertible
        assert_eq!(hrasva_to_dirgha('अ'), None);
        assert_eq!(hrasva_to_dirgha('क'), None);
    }

    #[test]
    fn test_dirgha_to_hrasva() {
        assert_eq!(dirgha_to_hrasva('ई'), Some('इ'));
        assert_eq!(dirgha_to_hrasva('ऊ'), Some('उ'));
        assert_eq!(dirgha_to_hrasva('ी'), Some('ि'));
        assert_eq!(dirgha_to_hrasva('ू'), Some('ु'));
        assert_eq!(dirgha_to_hrasva('ॠ'), Some('ऋ'));
        assert_eq!(dirgha_to_hrasva('ॄ'), Some('ृ'));
        // Not convertible
        assert_eq!(dirgha_to_hrasva('आ'), None);
        assert_eq!(dirgha_to_hrasva('ए'), None);
    }

    #[test]
    fn test_hrasva_dirgha_roundtrip() {
        let hrasva_chars = ['इ', 'उ', 'ऋ', 'ि', 'ु', 'ृ'];
        for h in hrasva_chars {
            let d = hrasva_to_dirgha(h).unwrap();
            let back = dirgha_to_hrasva(d).unwrap();
            assert_eq!(back, h, "roundtrip failed for {h}");
        }
    }

    #[test]
    fn test_svar_to_matra() {
        assert_eq!(svar_to_matra('अ'), None); // inherent vowel
        assert_eq!(svar_to_matra('आ'), Some('ा'));
        assert_eq!(svar_to_matra('इ'), Some('ि'));
        assert_eq!(svar_to_matra('ई'), Some('ी'));
        assert_eq!(svar_to_matra('उ'), Some('ु'));
        assert_eq!(svar_to_matra('ऊ'), Some('ू'));
        assert_eq!(svar_to_matra('ऋ'), Some('ृ'));
        assert_eq!(svar_to_matra('ए'), Some('े'));
        assert_eq!(svar_to_matra('ऐ'), Some('ै'));
        assert_eq!(svar_to_matra('ओ'), Some('ो'));
        assert_eq!(svar_to_matra('औ'), Some('ौ'));
    }

    #[test]
    fn test_matra_to_svar() {
        assert_eq!(matra_to_svar('ा'), Some('आ'));
        assert_eq!(matra_to_svar('ि'), Some('इ'));
        assert_eq!(matra_to_svar('ी'), Some('ई'));
        assert_eq!(matra_to_svar('ु'), Some('उ'));
        assert_eq!(matra_to_svar('ू'), Some('ऊ'));
        assert_eq!(matra_to_svar('ृ'), Some('ऋ'));
        assert_eq!(matra_to_svar('े'), Some('ए'));
        assert_eq!(matra_to_svar('ै'), Some('ऐ'));
        assert_eq!(matra_to_svar('ो'), Some('ओ'));
        assert_eq!(matra_to_svar('ौ'), Some('औ'));
    }

    #[test]
    fn test_matra_svar_roundtrip() {
        let matras = ['ा', 'ि', 'ी', 'ु', 'ू', 'ृ', 'े', 'ै', 'ो', 'ौ'];
        for m in matras {
            let s = matra_to_svar(m).unwrap();
            let back = svar_to_matra(s).unwrap();
            assert_eq!(back, m, "matra roundtrip failed for {m}");
        }
    }
}
