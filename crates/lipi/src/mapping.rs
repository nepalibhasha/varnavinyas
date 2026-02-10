#[cfg(feature = "legacy")]
use crate::legacy;
use crate::scheme::{LipiError, Scheme};

// =============================================================================
// Devanagari ↔ IAST mapping tables
// =============================================================================

/// Mapping pairs: (Devanagari, IAST).
/// Ordered longest-first within groups so greedy matching works.
const DEV_IAST_VOWELS: &[(&str, &str)] = &[
    ("औ", "au"),
    ("ऐ", "ai"),
    ("आ", "ā"),
    ("इ", "i"),
    ("ई", "ī"),
    ("उ", "u"),
    ("ऊ", "ū"),
    ("ऋ", "ṛ"),
    ("ॠ", "ṝ"),
    ("ऌ", "ḷ"),
    ("ॡ", "ḹ"),
    ("ए", "e"),
    ("ओ", "o"),
    ("अ", "a"),
];

const DEV_IAST_MATRA: &[(&str, &str)] = &[
    ("ौ", "au"),
    ("ै", "ai"),
    ("ा", "ā"),
    ("ि", "i"),
    ("ी", "ī"),
    ("ु", "u"),
    ("ू", "ū"),
    ("ृ", "ṛ"),
    ("ॄ", "ṝ"),
    ("े", "e"),
    ("ो", "o"),
];

const DEV_IAST_CONSONANTS: &[(&str, &str)] = &[
    ("क", "k"),
    ("ख", "kh"),
    ("ग", "g"),
    ("घ", "gh"),
    ("ङ", "ṅ"),
    ("च", "c"),
    ("छ", "ch"),
    ("ज", "j"),
    ("झ", "jh"),
    ("ञ", "ñ"),
    ("ट", "ṭ"),
    ("ठ", "ṭh"),
    ("ड", "ḍ"),
    ("ढ", "ḍh"),
    ("ण", "ṇ"),
    ("त", "t"),
    ("थ", "th"),
    ("द", "d"),
    ("ध", "dh"),
    ("न", "n"),
    ("प", "p"),
    ("फ", "ph"),
    ("ब", "b"),
    ("भ", "bh"),
    ("म", "m"),
    ("य", "y"),
    ("र", "r"),
    ("ल", "l"),
    ("व", "v"),
    ("श", "ś"),
    ("ष", "ṣ"),
    ("स", "s"),
    ("ह", "h"),
];

const DEV_IAST_SPECIAL: &[(&str, &str)] = &[
    ("ं", "ṃ"),
    ("ः", "ḥ"),
    ("ँ", "m̐"),
    ("ऽ", "'"),
    ("।", "|"),
    ("॥", "||"),
    ("्", ""), // virama — suppresses inherent vowel
];

const DEV_IAST_NUMERALS: &[(&str, &str)] = &[
    ("०", "0"),
    ("१", "1"),
    ("२", "2"),
    ("३", "3"),
    ("४", "4"),
    ("५", "5"),
    ("६", "6"),
    ("७", "7"),
    ("८", "8"),
    ("९", "9"),
];

// IAST → Devanagari mapping: sorted by IAST string length (longest first)
// for greedy matching from IAST side.
const IAST_DEV_CONSONANTS: &[(&str, &str)] = &[
    ("kh", "ख"),
    ("gh", "घ"),
    ("ch", "छ"),
    ("jh", "झ"),
    ("ṭh", "ठ"),
    ("ḍh", "ढ"),
    ("th", "थ"),
    ("dh", "ध"),
    ("ph", "फ"),
    ("bh", "भ"),
    ("ṅ", "ङ"),
    ("ñ", "ञ"),
    ("ṭ", "ट"),
    ("ḍ", "ड"),
    ("ṇ", "ण"),
    ("ś", "श"),
    ("ṣ", "ष"),
    ("k", "क"),
    ("g", "ग"),
    ("c", "च"),
    ("j", "ज"),
    ("t", "त"),
    ("d", "द"),
    ("n", "न"),
    ("p", "प"),
    ("b", "ब"),
    ("m", "म"),
    ("y", "य"),
    ("r", "र"),
    ("l", "ल"),
    ("v", "व"),
    ("s", "स"),
    ("h", "ह"),
];

const IAST_DEV_VOWELS: &[(&str, &str)] = &[
    ("au", "औ"),
    ("ai", "ऐ"),
    ("ā", "आ"),
    ("ī", "ई"),
    ("ū", "ऊ"),
    ("ṛ", "ऋ"),
    ("ṝ", "ॠ"),
    ("ḷ", "ऌ"),
    ("ḹ", "ॡ"),
    ("a", "अ"),
    ("i", "इ"),
    ("u", "उ"),
    ("e", "ए"),
    ("o", "ओ"),
];

const IAST_DEV_MATRA: &[(&str, &str)] = &[
    ("au", "ौ"),
    ("ai", "ै"),
    ("ā", "ा"),
    ("ī", "ी"),
    ("ū", "ू"),
    ("ṛ", "ृ"),
    ("ṝ", "ॄ"),
    ("a", ""), // inherent vowel — no matra
    ("i", "ि"),
    ("u", "ु"),
    ("e", "े"),
    ("o", "ो"),
];

const IAST_DEV_SPECIAL: &[(&str, &str)] = &[
    ("ṃ", "ं"),
    ("ḥ", "ः"),
    ("m̐", "ँ"),
    ("'", "ऽ"),
    ("||", "॥"),
    ("|", "।"),
];

const IAST_DEV_NUMERALS: &[(&str, &str)] = &[
    ("0", "०"),
    ("1", "१"),
    ("2", "२"),
    ("3", "३"),
    ("4", "४"),
    ("5", "५"),
    ("6", "६"),
    ("7", "७"),
    ("8", "८"),
    ("9", "९"),
];

// =============================================================================
// Transliteration engine
// =============================================================================

pub(crate) fn transliterate_impl(
    input: &str,
    from: Scheme,
    to: Scheme,
) -> Result<String, LipiError> {
    match (from, to) {
        (Scheme::Devanagari, Scheme::Iast) => Ok(dev_to_iast(input)),
        (Scheme::Iast, Scheme::Devanagari) => Ok(iast_to_dev(input)),
        #[cfg(feature = "legacy")]
        (Scheme::Preeti, Scheme::Devanagari) => Ok(legacy::preeti_to_unicode(input)),
        #[cfg(feature = "legacy")]
        (Scheme::Kantipur, Scheme::Devanagari) => Ok(legacy::kantipur_to_unicode(input)),
        _ => Err(LipiError::UnsupportedPair { from, to }),
    }
}

/// Devanagari → IAST transliteration.
fn dev_to_iast(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let remaining: String = chars[i..].iter().collect();

        // Try consonant match first
        if let Some((dev, iast, consumed)) = find_match_dev(&remaining, DEV_IAST_CONSONANTS) {
            result.push_str(iast);
            i += dev.chars().count();

            // After a consonant, check for matra or virama
            if i < len {
                let after: String = chars[i..].iter().collect();
                if let Some((_, m_iast, m_consumed)) = find_match_dev(&after, DEV_IAST_MATRA) {
                    result.push_str(m_iast);
                    i += m_consumed;
                } else if after.starts_with('्') {
                    // virama — suppress inherent vowel
                    i += 1; // consume the virama
                // Don't add inherent 'a'
                } else {
                    // No matra and no virama → inherent vowel 'a'
                    result.push('a');
                }
            } else {
                // End of string → inherent vowel
                result.push('a');
            }
            let _ = consumed;
            continue;
        }

        // Try vowel match
        if let Some((_, iast, consumed)) = find_match_dev(&remaining, DEV_IAST_VOWELS) {
            result.push_str(iast);
            i += consumed;
            continue;
        }

        // Try special (anusvara, visarga, etc.)
        if let Some((_, iast, consumed)) = find_match_dev(&remaining, DEV_IAST_SPECIAL) {
            result.push_str(iast);
            i += consumed;
            continue;
        }

        // Try numerals
        if let Some((_, iast, consumed)) = find_match_dev(&remaining, DEV_IAST_NUMERALS) {
            result.push_str(iast);
            i += consumed;
            continue;
        }

        // Pass through unmapped characters
        result.push(chars[i]);
        i += 1;
    }

    result
}

/// IAST → Devanagari transliteration.
fn iast_to_dev(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut i = 0;
    let len = input.len();

    while i < len {
        let remaining = &input[i..];

        // Try special first (longest match like "||" before "|")
        if let Some((_, dev, consumed)) = find_match_iast(remaining, IAST_DEV_SPECIAL) {
            result.push_str(dev);
            i += consumed;
            continue;
        }

        // Try consonant match (longest first: "kh" before "k")
        if let Some((_, dev, consumed)) = find_match_iast(remaining, IAST_DEV_CONSONANTS) {
            result.push_str(dev);
            i += consumed;

            // Check if next is another consonant (needs virama between them)
            // or a vowel (becomes matra)
            // Peek ahead to see if there's a vowel next
            let next_remaining = &input[i..];

            if let Some((_, matra, v_consumed)) = find_match_iast(next_remaining, IAST_DEV_MATRA) {
                if !matra.is_empty() {
                    // Non-empty matra (not inherent 'a')
                    result.push_str(matra);
                }
                // else: inherent 'a' → no matra needed
                i += v_consumed;
            } else {
                // No vowel follows → add virama (halanta)
                result.push('्');
            }
            continue;
        }

        // Try standalone vowel
        if let Some((_, dev, consumed)) = find_match_iast(remaining, IAST_DEV_VOWELS) {
            result.push_str(dev);
            i += consumed;
            continue;
        }

        // Try numerals
        if let Some((_, dev, consumed)) = find_match_iast(remaining, IAST_DEV_NUMERALS) {
            result.push_str(dev);
            i += consumed;
            continue;
        }

        // Pass through unmapped characters
        let c = remaining.chars().next().unwrap();
        result.push(c);
        i += c.len_utf8();
    }

    result
}

/// Find the longest matching entry from the table, matching from the start of `text`.
/// Returns (matched_key, mapped_value, chars_consumed).
fn find_match_dev<'a>(text: &str, table: &'a [(&str, &str)]) -> Option<(&'a str, &'a str, usize)> {
    let mut best: Option<(&str, &str, usize)> = None;

    for &(dev, iast) in table {
        if text.starts_with(dev) {
            let consumed = dev.chars().count();
            if best.is_none() || consumed > best.unwrap().2 {
                best = Some((dev, iast, consumed));
            }
        }
    }

    best
}

/// Find the longest matching IAST entry, matching from the start of `text`.
fn find_match_iast<'a>(text: &str, table: &'a [(&str, &str)]) -> Option<(&'a str, &'a str, usize)> {
    let mut best: Option<(&str, &str, usize)> = None;

    for &(iast, dev) in table {
        if text.starts_with(iast) {
            let consumed = iast.len(); // byte length for IAST strings
            if best.is_none() || consumed > best.unwrap().2 {
                best = Some((iast, dev, consumed));
            }
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Devanagari → IAST ---

    #[test]
    fn test_dev_to_iast_simple_vowels() {
        assert_eq!(dev_to_iast("अ"), "a");
        assert_eq!(dev_to_iast("आ"), "ā");
        assert_eq!(dev_to_iast("इ"), "i");
        assert_eq!(dev_to_iast("ई"), "ī");
        assert_eq!(dev_to_iast("उ"), "u");
        assert_eq!(dev_to_iast("ऊ"), "ū");
        assert_eq!(dev_to_iast("ऋ"), "ṛ");
        assert_eq!(dev_to_iast("ए"), "e");
        assert_eq!(dev_to_iast("ऐ"), "ai");
        assert_eq!(dev_to_iast("ओ"), "o");
        assert_eq!(dev_to_iast("औ"), "au");
    }

    #[test]
    fn test_dev_to_iast_consonants_with_inherent_vowel() {
        assert_eq!(dev_to_iast("क"), "ka");
        assert_eq!(dev_to_iast("ख"), "kha");
        assert_eq!(dev_to_iast("ग"), "ga");
        assert_eq!(dev_to_iast("न"), "na");
        assert_eq!(dev_to_iast("म"), "ma");
    }

    #[test]
    fn test_dev_to_iast_consonant_with_matra() {
        assert_eq!(dev_to_iast("का"), "kā");
        assert_eq!(dev_to_iast("कि"), "ki");
        assert_eq!(dev_to_iast("की"), "kī");
        assert_eq!(dev_to_iast("कु"), "ku");
        assert_eq!(dev_to_iast("कू"), "kū");
        assert_eq!(dev_to_iast("के"), "ke");
        assert_eq!(dev_to_iast("कै"), "kai");
        assert_eq!(dev_to_iast("को"), "ko");
        assert_eq!(dev_to_iast("कौ"), "kau");
    }

    #[test]
    fn test_dev_to_iast_virama() {
        assert_eq!(dev_to_iast("क्"), "k");
    }

    #[test]
    fn test_dev_to_iast_conjunct() {
        assert_eq!(dev_to_iast("क्ष"), "kṣa");
    }

    #[test]
    fn test_dev_to_iast_namaste() {
        assert_eq!(dev_to_iast("नमस्ते"), "namaste");
    }

    #[test]
    fn test_dev_to_iast_numerals() {
        assert_eq!(dev_to_iast("१२३"), "123");
    }

    #[test]
    fn test_dev_to_iast_anusvara_visarga() {
        assert_eq!(dev_to_iast("ं"), "ṃ");
        assert_eq!(dev_to_iast("ः"), "ḥ");
    }

    // --- IAST → Devanagari ---

    #[test]
    fn test_iast_to_dev_simple_vowels() {
        assert_eq!(iast_to_dev("a"), "अ");
        assert_eq!(iast_to_dev("ā"), "आ");
        assert_eq!(iast_to_dev("i"), "इ");
        assert_eq!(iast_to_dev("ī"), "ई");
        assert_eq!(iast_to_dev("u"), "उ");
        assert_eq!(iast_to_dev("ū"), "ऊ");
    }

    #[test]
    fn test_iast_to_dev_consonant_with_vowel() {
        assert_eq!(iast_to_dev("ka"), "क");
        assert_eq!(iast_to_dev("kā"), "का");
        assert_eq!(iast_to_dev("ki"), "कि");
        assert_eq!(iast_to_dev("kī"), "की");
    }

    #[test]
    fn test_iast_to_dev_consonant_cluster() {
        assert_eq!(iast_to_dev("kṣa"), "क्ष");
    }

    #[test]
    fn test_iast_to_dev_namaste() {
        assert_eq!(iast_to_dev("namaste"), "नमस्ते");
    }

    #[test]
    fn test_iast_to_dev_aspirates() {
        assert_eq!(iast_to_dev("kha"), "ख");
        assert_eq!(iast_to_dev("gha"), "घ");
        assert_eq!(iast_to_dev("cha"), "छ");
        assert_eq!(iast_to_dev("jha"), "झ");
        assert_eq!(iast_to_dev("ṭha"), "ठ");
        assert_eq!(iast_to_dev("ḍha"), "ढ");
        assert_eq!(iast_to_dev("tha"), "थ");
        assert_eq!(iast_to_dev("dha"), "ध");
        assert_eq!(iast_to_dev("pha"), "फ");
        assert_eq!(iast_to_dev("bha"), "भ");
    }

    #[test]
    fn test_dev_to_iast_mixed_passthrough() {
        assert_eq!(dev_to_iast("hello"), "hello");
        assert_eq!(dev_to_iast("क hello"), "ka hello");
    }

    // --- Roundtrip ---

    #[test]
    fn test_roundtrip_simple() {
        let texts = ["नमस्ते", "क", "अ", "काठमाडौं"];
        for text in texts {
            let iast = dev_to_iast(text);
            let back = iast_to_dev(&iast);
            assert_eq!(back, text, "roundtrip failed for {text}: IAST={iast}");
        }
    }
}
