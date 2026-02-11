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
}
