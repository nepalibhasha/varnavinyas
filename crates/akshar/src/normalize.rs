use unicode_normalization::UnicodeNormalization;

/// Normalize Devanagari text to a canonical form (NFC).
///
/// - Applies Unicode NFC normalization
/// - Standardizes visually identical sequences
///
/// Invariant: `normalize(normalize(s)) == normalize(s)` (idempotent)
pub fn normalize(text: &str) -> String {
    text.nfc().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_already_nfc() {
        let text = "नमस्ते";
        assert_eq!(normalize(text), text);
    }

    #[test]
    fn test_idempotence() {
        let text = "काठमाडौं नेपाल";
        let once = normalize(text);
        let twice = normalize(&once);
        assert_eq!(once, twice);
    }

    #[test]
    fn test_empty() {
        assert_eq!(normalize(""), "");
    }

    #[test]
    fn test_ascii_passthrough() {
        assert_eq!(normalize("hello"), "hello");
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn normalize_idempotent(s in "[\\u{0900}-\\u{097F}]{0,50}") {
            let once = normalize(&s);
            let twice = normalize(&once);
            prop_assert_eq!(&once, &twice);
        }
    }
}
