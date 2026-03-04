# varnavinyas-shabda

Word-level lexical analysis: origin classification and lightweight morphology.

## What This Crate Owns

This crate sits above the raw lexicon and tries to answer lexical questions about a single word:

- what is this word's likely origin?
- can it be decomposed into root + affix-like parts?

It is the bridge between raw lexical presence (`kosha`) and higher-level linguistic interpretation.

## Main APIs

- `classify(&str)` -> classify origin
- `classify_with_provenance(&str)` -> origin plus source and confidence
- `source_language(&str)` -> dictionary-derived source language when available
- `decompose(&str)` -> lightweight prefix/suffix decomposition

## Examples

### Classify with provenance

```rust
use varnavinyas_shabda::classify_with_provenance;

let decision = classify_with_provenance("नेपाल");
let _origin = decision.origin;
let _source = decision.source;
```

### Lightweight decomposition

```rust
use varnavinyas_shabda::decompose;

let analysis = decompose("उल्लिखित");
let _root = analysis.root;
```

## Depends On

- `varnavinyas-kosha`
- `varnavinyas-types`

## Design Notes

- The crate prefers dictionary-backed facts when available.
- When no lexical tag exists, it falls back to explicit heuristics.
- Morphological decomposition is intentionally conservative and lexicality-gated.

## Current Limits

- Heuristic origin classification is useful but not a full etymological model.
- Decomposition is still lightweight and should not be mistaken for full grammatical parsing.

## Status

Important lexical interpretation layer, still evolving beyond MVP heuristics.
