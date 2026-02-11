use tower_lsp::lsp_types::{Position, Range};

/// Precomputed line-start byte offsets for efficient byte↔Position conversion.
pub struct LineIndex {
    /// line_starts[i] = byte offset where line i begins.
    line_starts: Vec<usize>,
    /// The full text, retained for character-level iteration.
    text: String,
}

impl LineIndex {
    pub fn new(text: &str) -> Self {
        let mut line_starts = vec![0];
        for (i, b) in text.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push(i + 1);
            }
        }
        Self {
            line_starts,
            text: text.to_string(),
        }
    }

    /// Convert a byte offset to an LSP Position (0-based line, UTF-16 character offset).
    pub fn byte_offset_to_position(&self, byte_offset: usize) -> Position {
        let line = match self.line_starts.binary_search(&byte_offset) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        let line_start = self.line_starts[line];
        let utf16_col = self.text[line_start..byte_offset]
            .chars()
            .map(|c| c.len_utf16() as u32)
            .sum();
        Position {
            line: line as u32,
            character: utf16_col,
        }
    }

    /// Convert a byte span (start, end) to an LSP Range.
    pub fn byte_span_to_range(&self, span: (usize, usize)) -> Range {
        Range {
            start: self.byte_offset_to_position(span.0),
            end: self.byte_offset_to_position(span.1),
        }
    }

    /// Convert an LSP Position back to a byte offset.
    pub fn position_to_byte_offset(&self, pos: Position) -> usize {
        let line = pos.line as usize;
        if line >= self.line_starts.len() {
            return self.text.len();
        }
        let line_start = self.line_starts[line];
        let line_text = if line + 1 < self.line_starts.len() {
            &self.text[line_start..self.line_starts[line + 1]]
        } else {
            &self.text[line_start..]
        };

        let mut utf16_count: u32 = 0;
        let mut byte_offset = line_start;
        for ch in line_text.chars() {
            if utf16_count >= pos.character {
                break;
            }
            utf16_count += ch.len_utf16() as u32;
            byte_offset += ch.len_utf8();
        }
        byte_offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_single_line() {
        let idx = LineIndex::new("hello");
        assert_eq!(
            idx.byte_offset_to_position(0),
            Position {
                line: 0,
                character: 0
            }
        );
        assert_eq!(
            idx.byte_offset_to_position(5),
            Position {
                line: 0,
                character: 5
            }
        );
    }

    #[test]
    fn multiline_devanagari() {
        // "नमस्ते\nसंसार" — each Devanagari char is 3 bytes UTF-8, 1 unit UTF-16
        let text = "नमस्ते\nसंसार";
        let idx = LineIndex::new(text);

        // 'न' starts at byte 0 → line 0, char 0
        assert_eq!(
            idx.byte_offset_to_position(0),
            Position {
                line: 0,
                character: 0
            }
        );

        // 'सं' on line 1, byte offset = "नमस्ते\n".len()
        let line1_start = "नमस्ते\n".len();
        assert_eq!(
            idx.byte_offset_to_position(line1_start),
            Position {
                line: 1,
                character: 0
            }
        );

        // 'सा' = second char on line 1
        let second_char = line1_start + "सं".len();
        assert_eq!(
            idx.byte_offset_to_position(second_char),
            Position {
                line: 1,
                character: 2 // सं is two chars: स + ं
            }
        );
    }

    #[test]
    fn roundtrip_position_byte() {
        let text = "नमस्ते\nसंसार\nनेपाल";
        let idx = LineIndex::new(text);

        // Test several positions round-trip
        for byte_off in [0, 3, 6, "नमस्ते\n".len(), text.len()] {
            let pos = idx.byte_offset_to_position(byte_off);
            let recovered = idx.position_to_byte_offset(pos);
            assert_eq!(
                byte_off, recovered,
                "roundtrip failed for byte offset {byte_off}"
            );
        }
    }

    #[test]
    fn span_to_range() {
        let text = "नमस्ते संसार";
        let idx = LineIndex::new(text);

        // Span covering "संसार"
        let start = "नमस्ते ".len();
        let end = text.len();
        let range = idx.byte_span_to_range((start, end));

        assert_eq!(range.start.line, 0);
        assert_eq!(range.end.line, 0);
        assert!(range.start.character < range.end.character);
    }
}
