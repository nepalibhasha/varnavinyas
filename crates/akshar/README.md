# varnavinyas-akshar

**Character & Script Utilities for Nepali Devanagari.**

This is the foundational crate for the Varnavinyas workspace. It handles Unicode normalization, character classification, and syllable analysis.

## Features
- **Classification**: Identify Vowels (स्वर), Consonants (व्यञ्जन), Matras, and Marks.
- **Normalization**: Canonicalize Devanagari text (NFC with Nepali specifics).
- **Segmentation**: Split text into Aksharas (syllables) and grapheme clusters.

## Usage

```rust
use varnavinyas_akshar::Akshara;

let text = "नमस्ते";
// Analyze characters...
```

## Status
✅ Stable
