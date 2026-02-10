/// Convert Preeti-encoded text to Unicode Devanagari.
///
/// **Partial support — not production-grade.** This mapping covers common
/// ASCII→Devanagari pairs but is incomplete. Multi-character sequences,
/// ligature reordering, and positional variants are not handled.
///
/// Handles the specific case of the 'i' matra (Preeti 'f'), which appears
/// before the consonant in legacy fonts but must be after in Unicode.
///
/// Unmapped ASCII characters pass through unchanged.
pub fn preeti_to_unicode(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 3);
    let mut pending_i_matra = false;

    for c in input.chars() {
        if c == 'f' || c == 'F' {
            if pending_i_matra {
                // Two 'f's/F's in a row? Dump the first one.
                result.push('ि');
            }
            pending_i_matra = true;
            continue;
        }

        let mapped = preeti_char(c);

        if pending_i_matra {
            if let Some(dev) = mapped {
                // We have a pending 'i' matra and just found a Devanagari char.
                // Output char first, then the matra.
                result.push_str(dev);
                result.push('ि');
            } else {
                // Found a non-mapped char (like space or punctuation).
                // Dump the matra first (best effort), then the char.
                result.push('ि');
                result.push(c);
            }
            pending_i_matra = false;
        } else {
            // Normal processing
            if let Some(dev) = mapped {
                result.push_str(dev);
            } else {
                result.push(c);
            }
        }
    }

    // Trailing pending matra?
    if pending_i_matra {
        result.push('ि');
    }

    result
}

/// Convert Kantipur-encoded text to Unicode Devanagari.
///
/// **Partial support — not production-grade.** Only a basic consonant and
/// numeral subset is mapped. There is no reverse (Unicode→Kantipur) path;
/// this conversion is **one-way only**.
///
/// Unmapped ASCII characters pass through unchanged.
pub fn kantipur_to_unicode(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 3);
    for c in input.chars() {
        if let Some(dev) = kantipur_char(c) {
            result.push_str(dev);
        } else {
            result.push(c);
        }
    }
    result
}

/// Preeti ASCII → Unicode Devanagari mapping.
/// Source: Preeti font documentation and community mapping tables.
fn preeti_char(c: char) -> Option<&'static str> {
    match c {
        // Consonants
        's' => Some("स"),
        'j' => Some("ज"),
        'b' => Some("ब"),
        'v' => Some("व"),
        'k' => Some("क"),
        'l' => Some("ल"),
        'd' => Some("द"),
        'h' => Some("ह"),
        'g' => Some("ग"),
        'r' => Some("र"),
        't' => Some("त"),
        'n' => Some("न"),
        'p' => Some("प"),
        'y' => Some("य"),
        'q' => Some("ट"),
        'w' => Some("ध"),
        'e' => Some("भ"),
        'u' => Some("म"),
        'i' => Some("ष"),
        'o' => Some("ड"),
        'c' => Some("छ"),
        'x' => Some("ख"),
        'z' => Some("श"),
        'a' => Some("ा"), // aa matra
        ';' => Some("ं"),  // anusvara
        // Aspirated consonants and special
        'Q' => Some("ठ"),
        'W' => Some("ढ"),
        'E' => Some("घ"),
        'R' => Some("झ"),
        'T' => Some("ञ"),
        'Y' => Some("ङ"),
        'U' => Some("थ"),
        'I' => Some("ण"),
        'O' => Some("फ"),
        'P' => Some("ँ"),  // chandrabindu
        'S' => Some("ृ"),  // ri matra
        'D' => Some("्"),  // halanta/virama
        'G' => Some("।"), // danda
        'H' => Some("अ"),
        'J' => Some("आ"),
        'K' => Some("इ"),
        'L' => Some("ई"),
        ':' => Some("उ"),
        '"' => Some("ऊ"),
        'Z' => Some("ए"),
        'C' => Some("ऐ"),
        'V' => Some("ओ"),
        'B' => Some("औ"),
        'N' => Some("ऋ"),
        'X' => Some("ॐ"),
        // Matras
        'F' => Some("ि"), // i matra
        '[' => Some("ी"), // ii matra
        ']' => Some("ू"),  // uu matra
        '\\' => Some("ु"), // u matra
        '/' => Some("्र"), // ra-halanta
        // Numerals
        '0' => Some("०"),
        '1' => Some("१"),
        '2' => Some("२"),
        '3' => Some("३"),
        '4' => Some("४"),
        '5' => Some("५"),
        '6' => Some("६"),
        '7' => Some("७"),
        '8' => Some("८"),
        '9' => Some("९"),
        // Punctuation
        '.' => Some("।"),
        ',' => Some(","),
        '!' => Some("!"),
        '?' => Some("?"),
        '-' => Some("-"),
        // Matras continued
        'm' => Some("े"),  // e matra
        'M' => Some("ै"),  // ai matra
        'A' => Some("ो"), // o matra
        '>' => Some("ौ"), // au matra
        _ => None,
    }
}

/// Kantipur ASCII → Unicode Devanagari mapping (basic subset).
fn kantipur_char(c: char) -> Option<&'static str> {
    match c {
        // Basic consonants (Kantipur mapping)
        's' => Some("स"),
        'j' => Some("ज"),
        'b' => Some("ब"),
        'v' => Some("व"),
        'k' => Some("क"),
        'l' => Some("ल"),
        'd' => Some("द"),
        'h' => Some("ह"),
        'g' => Some("ग"),
        'r' => Some("र"),
        't' => Some("त"),
        'n' => Some("न"),
        'p' => Some("प"),
        'y' => Some("य"),
        // Numerals
        '0' => Some("०"),
        '1' => Some("१"),
        '2' => Some("२"),
        '3' => Some("३"),
        '4' => Some("४"),
        '5' => Some("५"),
        '6' => Some("६"),
        '7' => Some("७"),
        '8' => Some("८"),
        '9' => Some("९"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preeti_consonants() {
        assert_eq!(preeti_to_unicode("s"), "स");
        assert_eq!(preeti_to_unicode("k"), "क");
        assert_eq!(preeti_to_unicode("n"), "न");
    }

    #[test]
    fn test_preeti_numerals() {
        assert_eq!(preeti_to_unicode("123"), "१२३");
    }

    #[test]
    fn test_preeti_known_string() {
        // "gDofn" in Preeti = ग + ् + फ + ल + न
        // This is a smoke test for known conversions
        let result = preeti_to_unicode("gDofn");
        assert!(result.contains('ग'));
        assert!(result.contains('्'));
    }

    #[test]
    fn test_preeti_passthrough() {
        // Characters not in the Preeti map pass through
        assert_eq!(preeti_to_unicode("@#$"), "@#$");
    }

    #[test]
    fn test_preeti_imatra_reordering() {
        // "fk" in Preeti: f='ि' (visual left), k='क' -> Unicode 'क' + 'ि' = "कि"
        let result = preeti_to_unicode("fk");
        assert_eq!(result, "कि");

        // "l" = 'ल'
        // "fl" = 'ि' + 'ल' -> Unicode 'ल' + 'ि' = "लि"
        assert_eq!(preeti_to_unicode("fl"), "लि");

        // "Fk" in Preeti: F='ि' (alternate style), k='क' -> Unicode 'क' + 'ि' = "कि"
        assert_eq!(preeti_to_unicode("Fk"), "कि");
    }

    #[test]
    fn test_preeti_imatra_edge_cases() {
        // "f " -> 'ि' + space (no consonant to attach to) -> dump matra then space
        // This is invalid Unicode rendering (dotted circle) but preserves data
        assert_eq!(preeti_to_unicode("f "), "ि ");

        // "ffk" -> double matra? First one dumps, second attaches
        // 'ि' + 'क' + 'ि' -> "ि कि"
        assert_eq!(preeti_to_unicode("ffk"), "िकि");
    }
}
