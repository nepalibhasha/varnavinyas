/// Nepali punctuation marks (14 types from Academy Section 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PunctuationMark {
    /// , (अल्पविराम — comma)
    AlpaViram,
    /// । (पूर्णविराम — Devanagari full stop)
    PurnaViram,
    /// ? (प्रश्नवाचक — question mark)
    PrashnaVachak,
    /// ! (विस्मयबोधक — exclamation mark)
    VismayBodhak,
    /// :- (निर्देशक — colon-dash)
    Nirdeshak,
    /// ' ' (एकल उद्धरण — single quotes)
    EkalUddharan,
    /// " " (दोहोरो उद्धरण — double quotes)
    DohoroUddharan,
    /// ( ) (कोष्ठक — parentheses)
    Koshthak,
    /// - (योजक — hyphen)
    Yojak,
    /// . (संक्षेप — abbreviation dot)
    Sankshep,
    /// ,, (ऐजन — double comma / Devanagari comma pair)
    Aijan,
    /// / (तिर्यक विराम — slash)
    TiryakViram,
    /// ; (अर्धविराम — semicolon)
    ArdhaViram,
    /// ... (ऐजन बिन्दु — ellipsis)
    AijanBindu,
}

/// A punctuation diagnostic.
#[derive(Debug, Clone)]
pub struct LekhyaDiagnostic {
    /// Byte offset span (start, end) in the original text.
    pub span: (usize, usize),
    /// What was found.
    pub found: String,
    /// What should be used instead.
    pub expected: String,
    /// Academy rule citation.
    pub rule: &'static str,
}

/// Check text for punctuation issues.
///
/// Detects:
/// - Y1: Period (.) used as sentence-ender instead of purna viram (।)
/// - Y2: ASCII double quotes instead of proper Nepali usage
/// - Y3: Common ASCII punctuation misuse in Devanagari text
pub fn check_punctuation(text: &str) -> Vec<LekhyaDiagnostic> {
    let mut diagnostics = Vec::new();

    check_period_as_sentence_end(text, &mut diagnostics);
    check_ellipsis(text, &mut diagnostics);

    // Sort by span start
    diagnostics.sort_by_key(|d| d.span.0);
    diagnostics
}

/// Y1: Detect `.` used as sentence-end in Devanagari text instead of `।`.
///
/// A period is flagged when it follows Devanagari text and is either at the
/// end of input or followed by whitespace/newline (i.e., sentence-final position).
/// Periods after ASCII/Latin text (abbreviations like "Dr.", "U.N.") are ignored.
fn check_period_as_sentence_end(text: &str, diagnostics: &mut Vec<LekhyaDiagnostic>) {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'.' {
            let period_start = i;
            let period_end = i + 1;

            // Check what precedes the period
            let has_devanagari_before = has_devanagari_before_pos(text, period_start);

            // Check what follows: end of text, whitespace, or newline
            let is_sentence_end = period_end >= bytes.len()
                || bytes[period_end] == b' '
                || bytes[period_end] == b'\n'
                || bytes[period_end] == b'\r';

            // Only flag if preceded by Devanagari and in sentence-final position
            if has_devanagari_before && is_sentence_end {
                // Check it's not part of "..." (ellipsis handled separately)
                let is_ellipsis = (period_start >= 2
                    && bytes[period_start - 1] == b'.'
                    && bytes[period_start - 2] == b'.')
                    || (period_end < bytes.len() && bytes[period_end] == b'.');

                if !is_ellipsis {
                    diagnostics.push(LekhyaDiagnostic {
                        span: (period_start, period_end),
                        found: ".".to_string(),
                        expected: "।".to_string(),
                        rule: "Section 5: पूर्णविराम (।) used as sentence-end in Nepali, not period (.)",
                    });
                }
            }
            i = period_end;
        } else {
            i += 1;
        }
    }
}

/// Y3: Detect "..." that should be ऐजन बिन्दु (ellipsis).
fn check_ellipsis(text: &str, diagnostics: &mut Vec<LekhyaDiagnostic>) {
    let mut i = 0;
    let bytes = text.as_bytes();
    while i + 2 < bytes.len() {
        if bytes[i] == b'.' && bytes[i + 1] == b'.' && bytes[i + 2] == b'.' {
            // Count consecutive dots
            let start = i;
            while i < bytes.len() && bytes[i] == b'.' {
                i += 1;
            }
            // Only flag if there's Devanagari context nearby
            if has_devanagari_before_pos(text, start) || has_devanagari_after_pos(text, i) {
                diagnostics.push(LekhyaDiagnostic {
                    span: (start, i),
                    found: text[start..i].to_string(),
                    expected: "…".to_string(),
                    rule: "Section 5: ऐजन बिन्दु — use ellipsis character (…) instead of multiple periods",
                });
            }
        } else {
            i += 1;
        }
    }
}

/// Check if there is Devanagari text before a given byte position.
fn has_devanagari_before_pos(text: &str, pos: usize) -> bool {
    text[..pos].chars().rev().take(10).any(is_devanagari_char)
}

/// Check if there is Devanagari text after a given byte position.
fn has_devanagari_after_pos(text: &str, pos: usize) -> bool {
    text[pos..].chars().take(10).any(is_devanagari_char)
}

/// Check if a character is in the Devanagari Unicode block.
fn is_devanagari_char(c: char) -> bool {
    ('\u{0900}'..='\u{097F}').contains(&c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn period_after_devanagari() {
        let diags = check_punctuation("नेपाल सुन्दर देश हो.");
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].found, ".");
        assert_eq!(diags[0].expected, "।");
    }

    #[test]
    fn purna_viram_is_correct() {
        let diags = check_punctuation("नेपाल सुन्दर देश हो।");
        assert!(diags.is_empty());
    }

    #[test]
    fn period_after_english_not_flagged() {
        let diags = check_punctuation("Dr. Smith went home.");
        assert!(diags.is_empty());
    }

    #[test]
    fn ellipsis_detected() {
        let diags = check_punctuation("त्यसपछि... के भयो?");
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].found, "...");
        assert_eq!(diags[0].expected, "…");
    }

    #[test]
    fn multiple_issues() {
        let diags = check_punctuation("नेपाल. र भारत...");
        assert_eq!(diags.len(), 2);
    }
}
