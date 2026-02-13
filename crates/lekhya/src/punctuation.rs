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
    check_quotes(text, &mut diagnostics);
    check_nirdeshak(text, &mut diagnostics);
    check_spacing(text, &mut diagnostics);

    // Sort by span start
    diagnostics.sort_by_key(|d| d.span.0);
    diagnostics
}

/// Y5: Detect bare `:` that should be `:-` (Nirdeshak) in Nepali.
fn check_nirdeshak(text: &str, diagnostics: &mut Vec<LekhyaDiagnostic>) {
    let bytes = text.as_bytes();
    for (i, c) in text.char_indices() {
        if c == ':' {
            // Check if it's already :-
            if i + 1 < bytes.len() && bytes[i + 1] == b'-' {
                continue;
            }

            // Check context
            if has_devanagari_before_pos(text, i) {
                diagnostics.push(LekhyaDiagnostic {
                    span: (i, i + 1),
                    found: ":".to_string(),
                    expected: ":-".to_string(),
                    rule: "Section 5: निर्देशक — use colon-dash (:-), not just colon (:)",
                });
            }
        }
    }
}

/// Y6/Y7: Convert straight quotes to smart quotes in Devanagari context.
/// "..." -> \u{201C}...\u{201D} and '...' -> \u{2018}...\u{2019}
fn check_quotes(text: &str, diagnostics: &mut Vec<LekhyaDiagnostic>) {
    // Basic state machine for quote balancing would be complex to implement stateless.
    // For now, we flag ANY straight quote in Devanagari context as "should be smart quote".
    // We can suggest opening/closing based on whitespace context.

    for (i, c) in text.char_indices() {
        if (c == '"' || c == '\'')
            && (has_devanagari_before_pos(text, i) || has_devanagari_after_pos(text, i + 1))
        {
            let is_double = c == '"';
            let found = c.to_string();

            // Heuristic: if preceded by space/start OR specific punctuation like '(', '[', '{', '-', it's opening.
            // Otherwise closing.
            let is_opening = i == 0 || {
                let prev_char = text[..i].chars().last().unwrap_or(' ');
                prev_char.is_whitespace() || "([{".contains(prev_char) || prev_char == '-'
            };

            let expected = if is_double {
                if is_opening { "\u{201C}" } else { "\u{201D}" }
            } else if is_opening {
                "\u{2018}"
            } else {
                "\u{2019}"
            };

            diagnostics.push(LekhyaDiagnostic {
                span: (i, i + c.len_utf8()),
                found,
                expected: expected.to_string(),
                rule: if is_double {
                    "Section 5: दोहोरो उद्धरण \u{2014} use smart quotes \u{201C}...\u{201D} instead of straight \""
                } else {
                    "Section 5: एकल उद्धरण \u{2014} use smart quotes \u{2018}...\u{2019} instead of straight '"
                },
            });
        }
    }
}

/// Y2, Y4, Y13, Y14: Check spacing for ?, !, ;, ,
/// Standard rule: attached to previous word, followed by space.
fn check_spacing(text: &str, diagnostics: &mut Vec<LekhyaDiagnostic>) {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    for idx in 0..chars.len() {
        let (pos, c) = chars[idx];
        if matches!(c, '?' | '!' | ';' | ',') && has_devanagari_before_pos(text, pos) {
            // Check if preceded by space (error)
            if idx > 0 && chars[idx - 1].1.is_whitespace() {
                let prev_pos = chars[idx - 1].0;
                diagnostics.push(LekhyaDiagnostic {
                    span: (prev_pos, pos + c.len_utf8()),
                    found: format!(" {}", c),
                    expected: c.to_string(),
                    rule: "Section 5: punctuation should attach to the previous word",
                });
            }
        }
    }
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

            if has_devanagari_before {
                // Check what follows
                let is_eof = period_end >= bytes.len();
                let next_char = if !is_eof { Some(bytes[period_end]) } else { None };

                let is_newline = matches!(next_char, Some(b'\n' | b'\r'));
                let is_space = matches!(next_char, Some(b' '));

                // Case 1: End of sentence/text (EOF or Newline). ALWAYS an error (should be ।).
                // Even if it's an abbreviation, a sentence must end with ।.
                if is_eof || is_newline {
                    // Check exclusion for ellipsis handled separately
                    let is_part_of_ellipsis = (period_start >= 2
                        && bytes[period_start - 1] == b'.'
                        && bytes[period_start - 2] == b'.')
                        || (period_end < bytes.len() && bytes[period_end] == b'.'); // Lookahead safety check

                    if !is_part_of_ellipsis {
                        diagnostics.push(LekhyaDiagnostic {
                            span: (period_start, period_end),
                            found: ".".to_string(),
                            expected: "।".to_string(),
                            rule: "Section 5: पूर्णविराम (।) used as sentence-end in Nepali, not period (.)",
                        });
                    }
                }
                // Case 2: Medial period (followed by space). Check for abbreviation.
                else if is_space {
                    let is_abbreviation = is_likely_abbreviation(text, period_start);
                    if !is_abbreviation {
                        // Check exclusion for ellipsis
                        let is_part_of_ellipsis = (period_start >= 2
                            && bytes[period_start - 1] == b'.'
                            && bytes[period_start - 2] == b'.')
                            || (period_end < bytes.len() && bytes[period_end] == b'.');

                        if !is_part_of_ellipsis {
                            diagnostics.push(LekhyaDiagnostic {
                                span: (period_start, period_end),
                                found: ".".to_string(),
                                expected: "।".to_string(),
                                rule: "Section 5: पूर्णविराम (।) used as sentence-end in Nepali, not period (.)",
                            });
                        }
                    }
                }
            }
            i = period_end;
        } else {
            i += 1;
        }
    }
}

/// Helper for Y10: Check if the text before `pos` looks like an abbreviation.
fn is_likely_abbreviation(text: &str, pos: usize) -> bool {
    // Look back to find the start of the word
    let prefix = &text[..pos];
    let word_start = prefix
        .rfind(|c: char| c.is_whitespace())
        .map(|i| i + 1)
        .unwrap_or(0);
    let word = &prefix[word_start..];

    // Common sentence-ending verbs that are short but definitely NOT abbreviations.
    // If the word is one of these, it's a full stop error, not an abbreviation.
    let common_enders = [
        "हो", "छ", "हुन्", "छन्", "थियो", "थिन्", "भयो", "गर्यो",
    ];
    if common_enders.contains(&word) {
        return false;
    }

    // If word is very short (1-3 chars) and fully Devanagari, treat as likely abbreviation.
    // BUT exclude numbers — digits are never abbreviations.
    // e.g. "५." is "5.", not an abbreviation.
    let char_count = word.chars().count();
    char_count > 0 
        && char_count <= 3 
        && word.chars().all(|c| is_devanagari_char(c) && !c.is_numeric())
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
                    expected: "\u{2026}".to_string(),
                    rule: "Section 5: ऐजन बिन्दु \u{2014} use ellipsis character (\u{2026}) instead of multiple periods",
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
    fn abbreviation_dot_allowed() {
        let diags = check_punctuation("डा. राम");
        assert!(diags.is_empty(), "Abbreviation dot should be allowed");
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
        assert_eq!(diags[0].expected, "\u{2026}");
    }

    #[test]
    fn nirdeshak_detected() {
        let diags = check_punctuation("उदाहरण:");
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].found, ":");
        assert_eq!(diags[0].expected, ":-");
    }

    #[test]
    fn smart_quotes_detected() {
        let diags = check_punctuation("\"नेपाल\"");
        assert_eq!(diags.len(), 2);
        assert_eq!(diags[0].expected, "\u{201C}");
        assert_eq!(diags[1].expected, "\u{201D}");
    }

    #[test]
    fn spacing_detected() {
        let diags = check_punctuation("के छ ?");
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].found, " ?");
        assert_eq!(diags[0].expected, "?");
    }

    #[test]
    fn multiple_issues() {
        let diags = check_punctuation("नेपाल. र भारत...");
        assert_eq!(diags.len(), 2);
    }
}
