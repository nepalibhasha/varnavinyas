# varnavinyas-kosha

Lexicon engine and lexical metadata lookup for the Varnavinyas workspace.

## What This Crate Owns

This crate is the dictionary-backed lexical layer. It provides:

- fast word existence checks
- headword metadata lookup
- reviewed lexicon-tier lookup
- correction-target safety checks
- origin tag lookup
- source-language lookup

Higher-level crates use `kosha` as the main gate for deciding whether a form is known and lexically plausible.

## Data Model

The crate currently builds a singleton lexicon from embedded compile-time assets:

- `data/words.txt` -> word-form inventory
- `data/headwords.tsv` -> headword metadata
- `data/lexicon_overrides.tsv` -> reviewed tier overrides for attested forms

It uses:

- an `fst::Set` for fast membership checks
- a sorted headword table for metadata lookup
- a reviewed override table for forms that are attested but unsafe as generic
  correction outputs

## Main APIs

- `kosha()` -> global singleton lexicon
- `contains(&str)` -> check whether a form is known
- `lookup(&str)` -> retrieve headword metadata
- `lexicon_tier(&str)` -> retrieve reviewed quality tier for a form
- `is_correction_target(&str)` -> test whether a form is safe as a suggested correction target
- `origin_of(&str)` -> infer origin from dictionary tags
- `source_language_of(&str)` -> source language from dictionary tags

## Examples

### Check lexical presence

```rust
use varnavinyas_kosha::kosha;

let lex = kosha();
assert!(lex.contains("नेपाल"));
```

### Look up lexical origin tags

```rust
use varnavinyas_kosha::kosha;

let lex = kosha();
let _origin = lex.origin_of("नेपाल");
```

## Used By

- `varnavinyas-shabda`
- `varnavinyas-sandhi`
- `varnavinyas-samasa`
- `varnavinyas-parikshak`

## Current Limits

- Metadata is still relatively shallow and string-based.
- `contains()` only tells you that a form exists. Correction rules should use
  `is_correction_target()` when raw attestation could cause false positives.
- Lexicon tiers are intentionally lazy and override-driven; they are not yet a
  full canonical/variant/generated classification for the whole lexicon.
- This is a lexical index, not yet a full lexical knowledge graph.

## Status

Core foundational data crate.
