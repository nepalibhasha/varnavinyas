/// Transliteration schemes supported by Varnavinyas.
///
/// Only schemes with implemented transliteration paths are included.
/// ISO 15919 and informal Nepali romanization will be added in Phase 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scheme {
    /// Devanagari Unicode script.
    Devanagari,
    /// International Alphabet of Sanskrit Transliteration.
    Iast,
    /// Preeti legacy font encoding.
    ///
    /// **Partial, one-way only** (Preeti → Devanagari). Requires `legacy` feature.
    #[cfg(feature = "legacy")]
    Preeti,
    /// Kantipur legacy font encoding.
    ///
    /// **Partial, one-way only** (Kantipur → Devanagari). Requires `legacy` feature.
    #[cfg(feature = "legacy")]
    Kantipur,
}

/// Error type for transliteration operations.
#[derive(Debug, thiserror::Error)]
pub enum LipiError {
    #[error("unsupported transliteration: {from:?} -> {to:?}")]
    UnsupportedPair { from: Scheme, to: Scheme },

    #[error("invalid input for scheme {scheme:?}: {detail}")]
    InvalidInput { scheme: Scheme, detail: String },

    #[error("unmappable character '{c}' in scheme {scheme:?}")]
    UnmappableChar { c: char, scheme: Scheme },
}

/// Attempt to detect the scheme of the input text.
pub(crate) fn detect_scheme_impl(input: &str) -> Option<Scheme> {
    if input.is_empty() {
        return None;
    }

    let mut devanagari_count = 0u32;
    let mut ascii_count = 0u32;
    let mut iast_diacritics = 0u32;
    let total = input.chars().count() as u32;

    for c in input.chars() {
        match c {
            '\u{0900}'..='\u{097F}' => devanagari_count += 1,
            'a'..='z' | 'A'..='Z' => ascii_count += 1,
            // IAST diacritics: ā ī ū ṛ ṝ ṃ ḥ ṣ ś ṅ ñ ṭ ḍ ṇ
            'ā' | 'ī' | 'ū' | 'ṛ' | 'ṝ' | 'ṃ' | 'ḥ' | 'ṣ' | 'ś' | 'ṅ' | 'ñ' | 'ṭ' | 'ḍ' | 'ṇ'
            | 'Ā' | 'Ī' | 'Ū' | 'Ṛ' | 'Ṝ' | 'Ṃ' | 'Ḥ' | 'Ṣ' | 'Ś' | 'Ṅ' | 'Ñ' | 'Ṭ' | 'Ḍ' | 'Ṇ' =>
            {
                iast_diacritics += 1;
                ascii_count += 1; // also counts as Latin
            }
            _ => {}
        }
    }

    if total == 0 {
        return None;
    }

    // If majority is Devanagari
    if devanagari_count * 2 > total {
        return Some(Scheme::Devanagari);
    }

    // If has IAST diacritics
    if iast_diacritics > 0 {
        return Some(Scheme::Iast);
    }

    // If mostly ASCII
    if ascii_count * 2 > total {
        return Some(Scheme::Iast); // default Latin to IAST
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_devanagari() {
        assert_eq!(detect_scheme_impl("नमस्ते"), Some(Scheme::Devanagari));
    }

    #[test]
    fn test_detect_iast_with_diacritics() {
        assert_eq!(detect_scheme_impl("namaskāra"), Some(Scheme::Iast));
    }

    #[test]
    fn test_detect_latin_defaults_to_iast() {
        assert_eq!(detect_scheme_impl("namaste"), Some(Scheme::Iast));
    }

    #[test]
    fn test_detect_empty() {
        assert_eq!(detect_scheme_impl(""), None);
    }
}
