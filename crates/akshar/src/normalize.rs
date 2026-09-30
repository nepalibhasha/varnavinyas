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

/// Remove Devanagari consonant shaping controls for lexical comparison only.
///
/// ZWJ/ZWNJ after consonant+virama select glyph forms; they are retained in
/// source text and must not become spelling corrections by themselves.
/// See https://www.unicode.org/versions/Unicode17.0.0/core-spec/chapter-12/
/// This is deliberately separate from NFC normalization and preserves controls
/// outside a Devanagari virama sequence (including emoji and other scripts).
pub fn orthographic_lookup_form(text: &str) -> std::borrow::Cow<'_, str> {
    use std::borrow::Cow;
    if !text.contains(['\u{200c}', '\u{200d}']) {
        return Cow::Borrowed(text);
    }
    let mut output: Option<String> = None;
    let mut previous = None;
    let mut before_previous = None;
    for (offset, ch) in text.char_indices() {
        let next = text[offset + ch.len_utf8()..].chars().next();
        let shaping_control = matches!(ch, '\u{200c}' | '\u{200d}')
            && previous == Some('्')
            && before_previous.is_some_and(crate::is_vyanjan)
            && next.is_none_or(crate::is_vyanjan);
        if shaping_control {
            output.get_or_insert_with(|| text[..offset].to_string());
        } else if let Some(output) = &mut output {
            output.push(ch);
        }
        before_previous = previous;
        previous = Some(ch);
    }
    output.map_or(Cow::Borrowed(text), Cow::Owned)
}

#[cfg(test)]
mod lookup_tests {
    use super::orthographic_lookup_form;
    use std::borrow::Cow;

    #[test]
    fn lookup_controls_are_limited_to_devanagari_virama_sequences() {
        for (input, expected) in [
            ("पुर्‍याउने", "पुर्याउने"),
            ("जगत्‌का", "जगत्का"),
            ("शक्‌ति", "शक्ति"),
            ("जगत्‍", "जगत्"),
        ] {
            assert_eq!(orthographic_lookup_form(input), expected);
            assert_eq!(
                orthographic_lookup_form(&orthographic_lookup_form(input)),
                expected
            );
        }
        for input in ["नमस्ते", "घर‍", "क‍्ष", "क्‍‍ष", "👩‍💻", "ر‌و", "abc‍"]
        {
            assert!(
                matches!(orthographic_lookup_form(input), Cow::Borrowed(value) if value == input),
                "{input}"
            );
        }
    }
}
