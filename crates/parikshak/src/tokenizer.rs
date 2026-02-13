use varnavinyas_kosha::kosha;
use varnavinyas_prakriya::is_in_correction_table;

/// A token extracted from text.
#[derive(Debug, Clone)]
pub struct Token {
    /// The word text (without surrounding punctuation).
    pub text: String,
    /// Byte offset of the start of this token in the original text.
    pub start: usize,
    /// Byte offset of the end of this token in the original text.
    pub end: usize,
}

/// A token with suffix analysis — the stem and optional detached suffix.
#[derive(Debug, Clone)]
pub struct AnalyzedToken {
    /// The stem (after suffix detachment, or the full word if no suffix matched).
    pub stem: String,
    /// The detached suffix, if any.
    pub suffix: Option<String>,
    /// Byte offset of the start of the full token (stem+suffix) in the original text.
    pub start: usize,
    /// Byte offset of the end of the full token (stem+suffix) in the original text.
    pub end: usize,
}

/// Known Nepali postpositions and plural markers, ordered longest-first for greedy matching.
const SUFFIXES: &[&str] = &[
    "भित्र", "प्रति", "देखि", "हरू", "हरु", "लाई", "बाट", "सँग", "तिर", "का", "की", "ले", "को",
    "मा",
];

/// Vocative case markers. Only active behind `vocative-tokenization` feature.
#[cfg(feature = "vocative-tokenization")]
const VOCATIVE_SUFFIXES: &[&str] = &["ए", "ओ"];

/// Discourse particles (nipats). Only active behind `nipat-tokenization` feature.
/// Sorted longest-first. Single-char nipats (त, ल, नि) are risky — extra guard applied.
#[cfg(feature = "nipat-tokenization")]
const NIPATS: &[&str] = &["क्यारे", "नै", "पो", "रे", "खै", "नि", "ल", "त"];

/// Tokenize text into word tokens with byte offsets.
///
/// Splits on whitespace and strips surrounding punctuation from each token.
/// Only returns tokens that contain at least one Devanagari character.
pub fn tokenize(text: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut pos = 0;

    for segment in text.split_whitespace() {
        // Find the byte offset of this segment in the original text
        let seg_start = text[pos..].find(segment).map(|i| pos + i).unwrap_or(pos);
        let seg_end = seg_start + segment.len();
        pos = seg_end;

        // Strip leading/trailing punctuation to get the word core
        let (word, word_start, word_end) = strip_punctuation(segment, seg_start);

        if !word.is_empty() && has_devanagari(&word) {
            tokens.push(Token {
                text: word,
                start: word_start,
                end: word_end,
            });
        }
    }

    tokens
}

/// Tokenize text into analyzed tokens with suffix detachment.
///
/// For each whitespace-delimited token, tries to detach a known suffix (longest-first).
/// A suffix is only detached if the remaining stem exists in the kosha lexicon.
/// If no valid split is found, the full word becomes the stem with `suffix: None`.
pub fn tokenize_analyzed(text: &str) -> Vec<AnalyzedToken> {
    let tokens = tokenize(text);
    let lex = kosha();

    tokens
        .into_iter()
        .map(|tok| {
            for sfx in SUFFIXES {
                if let Some(stem) = tok.text.strip_suffix(sfx) {
                    if !stem.is_empty() && (lex.contains(stem) || is_in_correction_table(stem)) {
                        return AnalyzedToken {
                            stem: stem.to_string(),
                            suffix: Some(sfx.to_string()),
                            start: tok.start,
                            end: tok.end,
                        };
                    }
                    // Oblique form: stem ends in ा (oblique) but dictionary has ो form
                    // e.g., "केटालाई" → stem "केटा", but kosha has "केटो"
                    #[cfg(feature = "oblique-forms")]
                    if !stem.is_empty() {
                        if let Some(base) = stem.strip_suffix('ा') {
                            let candidate = format!("{base}ो");
                            if lex.contains(&candidate) {
                                return AnalyzedToken {
                                    stem: stem.to_string(),
                                    suffix: Some(sfx.to_string()),
                                    start: tok.start,
                                    end: tok.end,
                                };
                            }
                        }
                    }
                }
            }
            // Vocative markers: single-char ए/ओ with triple guard
            #[cfg(feature = "vocative-tokenization")]
            for voc in VOCATIVE_SUFFIXES {
                if let Some(stem) = tok.text.strip_suffix(voc) {
                    // Guard 1: stem exists in kosha
                    // Guard 2: full word is NOT in kosha (avoid splitting real words)
                    // Guard 3: stem must end in vowel/matra (vocative attaches to vowel stems)
                    if !stem.is_empty()
                        && lex.contains(stem)
                        && !lex.contains(&tok.text)
                        && stem.chars().last().is_some_and(|c| {
                            varnavinyas_akshar::is_svar(c) || varnavinyas_akshar::is_matra(c)
                        })
                    {
                        return AnalyzedToken {
                            stem: stem.to_string(),
                            suffix: Some(voc.to_string()),
                            start: tok.start,
                            end: tok.end,
                        };
                    }
                }
            }
            // Nipat (discourse particle) detachment with triple guard
            #[cfg(feature = "nipat-tokenization")]
            for nip in NIPATS {
                if let Some(stem) = tok.text.strip_suffix(nip) {
                    // Guard 1: stem exists in kosha
                    // Guard 2: full word is NOT in kosha
                    // Guard 3: risky single-char nipats (≤3 bytes) require stem to end in vowel/matra
                    let is_risky = nip.len() <= 3;
                    let vowel_ending = stem.chars().last().is_some_and(|c| {
                        varnavinyas_akshar::is_svar(c) || varnavinyas_akshar::is_matra(c)
                    });
                    if !stem.is_empty()
                        && lex.contains(stem)
                        && !lex.contains(&tok.text)
                        && (!is_risky || vowel_ending)
                    {
                        return AnalyzedToken {
                            stem: stem.to_string(),
                            suffix: Some(nip.to_string()),
                            start: tok.start,
                            end: tok.end,
                        };
                    }
                }
            }
            AnalyzedToken {
                stem: tok.text,
                suffix: None,
                start: tok.start,
                end: tok.end,
            }
        })
        .collect()
}

/// Strip leading and trailing punctuation from a token.
/// Returns (stripped_word, adjusted_start, adjusted_end).
fn strip_punctuation(token: &str, offset: usize) -> (String, usize, usize) {
    let chars: Vec<char> = token.chars().collect();

    // Find first non-punctuation char
    let start = chars
        .iter()
        .position(|c| !is_punctuation(*c))
        .unwrap_or(chars.len());

    // Find last non-punctuation char
    let end = chars
        .iter()
        .rposition(|c| !is_punctuation(*c))
        .map(|i| i + 1)
        .unwrap_or(0);

    if start >= end {
        return (String::new(), offset, offset);
    }

    let word: String = chars[start..end].iter().collect();

    // Calculate byte offsets
    let leading_bytes: usize = chars[..start].iter().map(|c| c.len_utf8()).sum();
    let word_bytes: usize = chars[start..end].iter().map(|c| c.len_utf8()).sum();

    (
        word,
        offset + leading_bytes,
        offset + leading_bytes + word_bytes,
    )
}

/// Check if a character is punctuation (for tokenization purposes).
fn is_punctuation(c: char) -> bool {
    matches!(
        c,
        '.' | ','
            | '!'
            | '?'
            | ';'
            | ':'
            | '-'
            | '('
            | ')'
            | '['
            | ']'
            | '{'
            | '}'
            | '"'
            | '\''
            | '/'
            | '।'
            | '…'
    )
}

/// Check if a string contains any Devanagari character.
fn has_devanagari(s: &str) -> bool {
    s.chars().any(|c| ('\u{0900}'..='\u{097F}').contains(&c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_tokenization() {
        let tokens = tokenize("नेपाल राम्रो देश हो।");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].text, "नेपाल");
        assert_eq!(tokens[3].text, "हो");
    }

    #[test]
    fn strips_trailing_danda() {
        let tokens = tokenize("देश हो।");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[1].text, "हो");
    }

    #[test]
    fn empty_input() {
        assert!(tokenize("").is_empty());
        assert!(tokenize("   ").is_empty());
    }

    #[test]
    fn skips_english_tokens() {
        let tokens = tokenize("hello नेपाल world");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].text, "नेपाल");
    }

    #[test]
    fn preserves_byte_offsets() {
        let text = "नेपाल राम्रो";
        let tokens = tokenize(text);
        assert_eq!(tokens.len(), 2);
        assert_eq!(&text[tokens[0].start..tokens[0].end], "नेपाल");
        assert_eq!(&text[tokens[1].start..tokens[1].end], "राम्रो");
    }

    // --- O8 acceptance tests: suffix-aware tokenizer ---

    /// O8.1: "रामलाई" → stem "राम", suffix "लाई"
    #[test]
    fn o8_1_detach_laai() {
        let tokens = tokenize_analyzed("रामलाई");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].stem, "राम");
        assert_eq!(tokens[0].suffix.as_deref(), Some("लाई"));
    }

    /// O8.2: "घरहरु" → stem "घर", suffix "हरु"
    #[test]
    fn o8_2_detach_haru() {
        let tokens = tokenize_analyzed("घरहरु");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].stem, "घर");
        assert_eq!(tokens[0].suffix.as_deref(), Some("हरु"));
    }

    /// O8.3: "नेपालमा" → stem "नेपाल", suffix "मा"
    #[test]
    fn o8_3_detach_maa() {
        let tokens = tokenize_analyzed("नेपालमा");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].stem, "नेपाल");
        assert_eq!(tokens[0].suffix.as_deref(), Some("मा"));
    }

    /// O8.4: Unknown stem keeps original token unsplit.
    #[test]
    fn o8_4_unknown_stem_unsplit() {
        let tokens = tokenize_analyzed("ज्ञपतमा");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].stem, "ज्ञपतमा");
        assert_eq!(tokens[0].suffix, None);
    }

    /// O8.5: Longest suffix wins — "घरभित्र" matches "भित्र" (15 bytes), not shorter.
    #[test]
    fn o8_5_longest_suffix_wins() {
        let tokens = tokenize_analyzed("घरभित्र");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].stem, "घर");
        assert_eq!(tokens[0].suffix.as_deref(), Some("भित्र"));
    }

    /// O8.6: tokenize() still returns Vec<Token> unchanged.
    #[test]
    fn o8_6_tokenize_unchanged() {
        let tokens = tokenize("रामलाई नेपालमा");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].text, "रामलाई");
        assert_eq!(tokens[1].text, "नेपालमा");
    }

    /// O8.7: tokenize_analyzed() compiles and returns Vec<AnalyzedToken>.
    #[test]
    fn o8_7_returns_analyzed_tokens() {
        let tokens: Vec<AnalyzedToken> = tokenize_analyzed("राम नेपाल");
        assert_eq!(tokens.len(), 2);
    }

    /// O8.8: Byte offsets cover the full original unsplit form.
    #[test]
    fn o8_8_byte_offsets_cover_full_token() {
        let text = "रामलाई नेपालमा";
        let tokens = tokenize_analyzed(text);
        assert_eq!(tokens.len(), 2);
        assert_eq!(&text[tokens[0].start..tokens[0].end], "रामलाई");
        assert_eq!(&text[tokens[1].start..tokens[1].end], "नेपालमा");
    }
}
