# varnavinyas-parikshak

**Spell Checker & Integration Pipeline.**

The top-level crate that composes all other crates into a usable spell-checking tool.

## Features
- **Pipeline**: Tokenization -> Normalization -> Lexicon -> Rule Analysis -> Diagnostics.
- **Diagnostics**: Rich error reporting with rule citations and specific corrections.
- **Batch Processing**: Capable of processing large documents.

## Usage

```rust
use varnavinyas_parikshak::check_text;

let diagnostics = check_text("यो बाक्यमा गल्ति छ");
```

## Status
✅ Stable
