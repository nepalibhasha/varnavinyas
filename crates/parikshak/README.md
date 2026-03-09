# varnavinyas-parikshak

Top-level checking pipeline for text diagnostics.

For a high-level description of the checker pipeline and crate boundaries, see
[`ARCHITECTURE.md`](./ARCHITECTURE.md).

## What This Crate Owns

This is the main orchestrator crate. It combines the lower-level engines into a practical text checker.

It is responsible for:

- tokenizing text
- running word-level orthography checks
- attaching punctuation diagnostics
- optionally adding grammar/samasa-style heuristics
- returning unified diagnostics with spans, categories, rule citations, and confidence

If you want “check this text and tell me what to flag”, this is the crate to call.

## Main APIs

- `check_word(&str)` -> check a single word
- `check_text(&str)` -> default full-text diagnostics
- `check_text_with_options(&str, CheckOptions)` -> full-text diagnostics with runtime options
- `tokenize(&str)` / `tokenize_analyzed(&str)` -> tokenization helpers

## Examples

### Check a single word

```rust
use varnavinyas_parikshak::check_word;

let diag = check_word("राजनैतिक").unwrap();
assert_eq!(diag.correction, "राजनीतिक");
```

### Check full text

```rust
use varnavinyas_parikshak::check_text;

let diagnostics = check_text("यो बाक्यमा गल्ति छ.");
assert!(!diagnostics.is_empty());
```

## Depends On

- `varnavinyas-prakriya`
- `varnavinyas-lekhya`
- `varnavinyas-kosha`
- optionally `varnavinyas-vyakaran` and `varnavinyas-samasa` in heuristic paths

## Design Notes

- `parikshak` is the integration layer, not the source of core orthographic truth.
- It should preserve distinctions between hard errors, variants, and heuristic suggestions.

## Current Limits

- Some higher-level diagnostics are still heuristic and intentionally conservative.
- Tokenization is practical but not yet a full linguistic parser.

## Status

Primary production-facing checker pipeline.
