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
- `classify()` retains a best-effort four-way category for existing rule consumers.
  A fallback `Deshaj` value is not proof of native origin. Present origin using
  `classify_with_provenance()`: `Override` / `Kosha` indicate documented evidence,
  `Heuristic` indicates a positive spelling-pattern inference, and `Unknown`
  indicates no evidence (confidence zero). Headword membership alone is not origin
  evidence. Existing tagged headwords and reviewed overrides retain precedence.
- Morphological decomposition is intentionally conservative and lexicality-gated.

## Current Limits

- Heuristic origin classification is useful but not a full etymological model.
- Decomposition is still lightweight and should not be mistaken for full grammatical parsing.

## Status

Important lexical interpretation layer, still evolving beyond MVP heuristics.

Prefix analyses require root evidence independent of prefixed whole words.
A word such as सङ्घीय cannot prove the proposed root घीय. Attested case or
particle siblings remain usable for productive inflections whose bare stem is
missing from the lexicon (for example मच्छिन्द्रनाथको).
