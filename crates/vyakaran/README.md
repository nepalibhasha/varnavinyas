# varnavinyas-vyakaran

Grammar and morphology analysis framework for Nepali.

## What This Crate Owns

This crate is intended to handle grammatical analysis beyond orthographic correction. Its domain includes:

- morphological analysis
- grammatical feature assignment
- case/number/gender/person/tense-related interpretation

It is the natural home for deeper grammar-aware reasoning that should not live in `parikshak` heuristics forever.

## Main Concepts

The crate defines:

- grammatical categories such as `Gender`, `Number`, `Case`, `Person`, `Tense`
- `Features` -> a container for grammatical properties
- `MorphAnalysis` -> structured analysis output
- `MorphAnalyzer` -> trait for analyzers

## Current Implementations

- `StubAnalyzer` -> explicit placeholder that returns `NotImplemented`
- `RuleBasedAnalyzer` -> feature-gated MVP implementation

## Example

```rust
use varnavinyas_vyakaran::{MorphAnalyzer, StubAnalyzer};

let analyzer = StubAnalyzer;
let result = analyzer.analyze("नेपाल");
assert!(result.is_err());
```

That example shows the current state honestly: the abstraction is in place, but the grammar engine is still maturing.

## Used By

- optional grammar-pass logic in `varnavinyas-parikshak`
- future deeper linguistic analysis paths

## Current Limits

- This crate is still early-stage compared with the orthography stack.
- It should be treated as an evolving framework, not yet a complete grammar authority.

## Status

Experimental but architecturally important.
