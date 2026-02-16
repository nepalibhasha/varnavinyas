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
    /// : / - / :- (निर्देशक — colon variants)
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
    check_tiryak_viram_spacing(text, &mut diagnostics);
    check_aijan_pair_spacing(text, &mut diagnostics);
    check_parentheses_balance(text, &mut diagnostics);
    check_spacing(text, &mut diagnostics);

    // Sort by span start
    diagnostics.sort_by_key(|d| d.span.0);
    diagnostics
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
                let next_char = if !is_eof {
                    Some(bytes[period_end])
                } else {
                    None
                };

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
    let prefix = &text[..pos];
    let word_start = prefix
        .rfind(|c: char| c.is_whitespace())
        .map_or(0, |i| i + 1);
    let word = &prefix[word_start..];

    // Use an allowlist for common Devanagari abbreviations.
    // Blanket "1-3 chars means abbreviation" causes false negatives like:
    // "म यहाँ हुँ. तिमी?" where "." should be flagged as "।".
    //
    // Keep this list conservative: false positive punctuation errors are cheaper
    // than missing genuine sentence-ending period misuse in Nepali text.
    let known_devanagari_abbreviations = ["डा", "श्री", "प्रा", "सं", "वि"];
    if known_devanagari_abbreviations.contains(&word) {
        return true;
    }

    // Chained abbreviations such as:
    // - "अ. दु. अ. आ."
    // - "त्रि.वि."
    //
    // If current token is short Devanagari and either:
    // 1) followed by another short token ending in '.', or
    // 2) preceded by another abbreviation token,
    // treat this period as abbreviation dot.
    if is_short_devanagari_token(word)
        && (follows_abbreviation_chain(text, pos)
            || preceded_by_abbreviation_chain(text, word_start))
    {
        return true;
    }

    // ASCII abbreviations (e.g., Dr., U.N.) are handled by upstream context:
    // this function is called only after confirming Devanagari context before '.',
    // so default to "not abbreviation".
    false
}

fn is_short_devanagari_token(token: &str) -> bool {
    let count = token.chars().count();
    if count == 0 || count > 4 {
        return false;
    }
    token.chars().all(is_devanagari_char)
}

fn follows_abbreviation_chain(text: &str, period_pos: usize) -> bool {
    let bytes = text.as_bytes();
    let mut i = period_pos + 1;

    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    if i >= bytes.len() {
        return false;
    }

    let mut j = i;
    while j < bytes.len() {
        let Some(ch) = text[j..].chars().next() else {
            break;
        };
        if !is_devanagari_char(ch) {
            break;
        }
        j += ch.len_utf8();
    }

    if j == i {
        return false;
    }

    let next_token = &text[i..j];
    if !is_short_devanagari_token(next_token) {
        return false;
    }

    j < bytes.len() && bytes[j] == b'.'
}

fn preceded_by_abbreviation_chain(text: &str, word_start: usize) -> bool {
    let bytes = text.as_bytes();
    if word_start == 0 {
        return false;
    }

    let mut i = word_start;
    while i > 0 && bytes[i - 1].is_ascii_whitespace() {
        i -= 1;
    }
    if i == 0 || bytes[i - 1] != b'.' {
        return false;
    }

    let prev_period_pos = i - 1;
    let prev_prefix = &text[..prev_period_pos];
    let prev_start = prev_prefix
        .rfind(|c: char| c.is_whitespace())
        .map_or(0, |idx| idx + 1);
    let prev_word = &prev_prefix[prev_start..];

    is_short_devanagari_token(prev_word)
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

/// Y12: In विकल्प form, slash should directly join alternatives (e.g., तिमी/उहाँ).
/// Flag spaces around `/` in Devanagari context.
fn check_tiryak_viram_spacing(text: &str, diagnostics: &mut Vec<LekhyaDiagnostic>) {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    for idx in 0..chars.len() {
        let (slash_pos, c) = chars[idx];
        if c != '/' {
            continue;
        }

        let has_space_before = idx > 0 && chars[idx - 1].1.is_whitespace();
        let has_space_after = idx + 1 < chars.len() && chars[idx + 1].1.is_whitespace();
        if !(has_space_before || has_space_after) {
            continue;
        }

        if !(has_devanagari_before_pos(text, slash_pos)
            || has_devanagari_after_pos(text, slash_pos + c.len_utf8()))
        {
            continue;
        }

        let span_start = if has_space_before {
            chars[idx - 1].0
        } else {
            slash_pos
        };
        let span_end = if has_space_after {
            chars[idx + 1].0 + chars[idx + 1].1.len_utf8()
        } else {
            slash_pos + c.len_utf8()
        };

        diagnostics.push(LekhyaDiagnostic {
            span: (span_start, span_end),
            found: text[span_start..span_end].to_string(),
            expected: "/".to_string(),
            rule: "Section 5: तिर्यक् विराम (/) विकल्पमा शब्दसँगै लेखिन्छ",
        });
    }
}

/// Y11: ऐजन should be written as `,,` (no space between commas).
fn check_aijan_pair_spacing(text: &str, diagnostics: &mut Vec<LekhyaDiagnostic>) {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b',' {
            i += 1;
            continue;
        }

        let mut j = i + 1;
        let mut had_space = false;
        while j < bytes.len() && bytes[j].is_ascii_whitespace() {
            had_space = true;
            j += 1;
        }

        if had_space
            && j < bytes.len()
            && bytes[j] == b','
            && (has_devanagari_before_pos(text, i) || has_devanagari_after_pos(text, j + 1))
        {
            diagnostics.push(LekhyaDiagnostic {
                span: (i, j + 1),
                found: text[i..j + 1].to_string(),
                expected: ",,".to_string(),
                rule: "Section 5: ऐजन चिह्नमा दुई अल्पविराम सँगै लेखिन्छ (,,)",
            });
            i = j + 1;
            continue;
        }

        i += 1;
    }
}

/// Y8: Basic parentheses sanity check — unmatched `(` or `)` in Devanagari context.
fn check_parentheses_balance(text: &str, diagnostics: &mut Vec<LekhyaDiagnostic>) {
    let mut stack: Vec<usize> = Vec::new();

    for (i, c) in text.char_indices() {
        match c {
            '(' => stack.push(i),
            ')' => {
                if stack.pop().is_none()
                    && (has_devanagari_before_pos(text, i) || has_devanagari_after_pos(text, i + 1))
                {
                    diagnostics.push(LekhyaDiagnostic {
                        span: (i, i + 1),
                        found: ")".to_string(),
                        expected: "()".to_string(),
                        rule: "Section 5: कोष्ठक चिह्न सन्तुलित रूपमा प्रयोग हुनुपर्छ",
                    });
                }
            }
            _ => {}
        }
    }

    for start in stack {
        if has_devanagari_before_pos(text, start) || has_devanagari_after_pos(text, start + 1) {
            diagnostics.push(LekhyaDiagnostic {
                span: (start, start + 1),
                found: "(".to_string(),
                expected: "()".to_string(),
                rule: "Section 5: कोष्ठक चिह्न सन्तुलित रूपमा प्रयोग हुनुपर्छ",
            });
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
    fn short_devanagari_word_dot_is_not_abbreviation() {
        let diags = check_punctuation("म यहाँ हुँ. तिमी कहाँ छौ?");
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
        assert_eq!(diags[0].expected, "\u{2026}");
    }

    #[test]
    fn nirdeshak_colon_is_allowed() {
        let diags = check_punctuation("उदाहरण:");
        assert!(diags.is_empty());
    }

    #[test]
    fn abbreviation_chain_is_allowed() {
        let diags = check_punctuation("अ. दु. अ. आ.ले सबैलाई सचेत गरायो।");
        assert!(diags.is_empty(), "Abbreviation chain should be allowed");
    }

    #[test]
    fn compact_abbreviation_chain_is_allowed() {
        let diags = check_punctuation("त्रि.वि.ले नतिजा प्रकाशित गर्‍यो।");
        assert!(
            diags.is_empty(),
            "Compact abbreviation chain should be allowed"
        );
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
    fn tiryak_viram_spacing_detected() {
        let diags = check_punctuation("तिमी / उहाँ आउनुहुन्छ।");
        assert!(
            diags
                .iter()
                .any(|d| d.expected == "/" && d.rule.contains("तिर्यक् विराम")),
            "Expected slash spacing diagnostic, got: {diags:?}"
        );
    }

    #[test]
    fn tiryak_viram_compact_ok() {
        let diags = check_punctuation("तिमी/उहाँ आउनुहुन्छ।");
        assert!(
            !diags
                .iter()
                .any(|d| d.expected == "/" && d.rule.contains("तिर्यक् विराम"))
        );
    }

    #[test]
    fn aijan_pair_spacing_detected() {
        let diags = check_punctuation("राम , , श्याम");
        assert!(
            diags
                .iter()
                .any(|d| d.expected == ",," && d.rule.contains("ऐजन")),
            "Expected ऐजन spacing diagnostic, got: {diags:?}"
        );
    }

    #[test]
    fn unmatched_open_paren_detected() {
        let diags = check_punctuation("नेपाल (सुन्दर देश हो।");
        assert!(
            diags
                .iter()
                .any(|d| d.found == "(" && d.rule.contains("कोष्ठक")),
            "Expected unmatched '(' diagnostic, got: {diags:?}"
        );
    }

    #[test]
    fn unmatched_close_paren_detected() {
        let diags = check_punctuation("नेपाल) सुन्दर देश हो।");
        assert!(
            diags
                .iter()
                .any(|d| d.found == ")" && d.rule.contains("कोष्ठक")),
            "Expected unmatched ')' diagnostic, got: {diags:?}"
        );
    }

    #[test]
    fn multiple_issues() {
        let diags = check_punctuation("नेपाल. र भारत...");
        assert_eq!(diags.len(), 2);
    }
}
