# varnavinyas-akshar

Low-level Devanagari text utilities for the Varnavinyas workspace.

## What This Crate Owns

This crate is the script-mechanics layer. It is responsible for:

- classifying Devanagari codepoints
- normalizing text into a predictable Unicode form
- splitting text into aksharas (pronounceable orthographic units)
- exposing reusable vowel/consonant helpers used by higher-level crates

If another crate needs to reason about Devanagari structure, it should generally depend on `varnavinyas-akshar` instead of re-implementing character logic.

## Main APIs

- `classify(char)` -> classify a codepoint into Devanagari character categories
- `normalize(&str)` -> normalize text before analysis
- `split_aksharas(&str)` -> segment text into akshara units with byte spans
- vowel helpers such as `hrasva_to_dirgha`, `dirgha_to_hrasva`, `svar_to_matra`
- consonant helpers such as `varga`, `panchham_of`, `is_voiced`

## Examples

### Split into aksharas

```rust
use varnavinyas_akshar::split_aksharas;

let parts = split_aksharas("नमस्ते");
let texts: Vec<&str> = parts.iter().map(|a| a.text.as_str()).collect();
assert_eq!(texts, vec!["न", "मस्", "ते"]);
```

### Convert vowel length

```rust
use varnavinyas_akshar::hrasva_to_dirgha;

assert_eq!(hrasva_to_dirgha('इ'), Some('ई'));
```

## Used By

- `varnavinyas-sandhi` for split guards and rule application
- `varnavinyas-lipi` for script-aware processing
- `varnavinyas-parikshak` and token-level analysis paths

## Current Limits

- Segmentation is hand-rolled and optimized for modern Nepali Devanagari use, not full Indic edge-case coverage.
- It is a structural layer only. It does not know lexical validity or grammar.

## Status

Stable foundation crate.
