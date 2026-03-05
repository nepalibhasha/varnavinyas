# varnavinyas-sandhi

Sandhi application and reverse-splitting engine.

## What This Crate Owns

This crate handles sound-change behavior at morpheme boundaries.

It provides two related but different capabilities:

- forward sandhi: combine two forms according to sandhi rules
- reverse sandhi: attempt to reconstruct possible original members from a combined form

This crate is the foundation for authentic compound reconstruction and for any future “prove this split” workflow.

## Main APIs

- `apply(&str, &str)` -> apply sandhi forward
- `split(&str)` -> generate ranked reverse split candidates
- `split_best(&str)` -> return the strict safest single candidate (Authoritative only)
- `split_best_for_compound(&str)` -> return a best candidate for compound analysis (Likely+ with classical lexical evidence)
- lower-level rule helpers:
  - `apply_vowel_sandhi`
  - `apply_visarga_sandhi`
  - `apply_consonant_sandhi`

## Examples

### Forward sandhi

Use `apply` when you already know the two members and want to see the combined output.

```rust
use varnavinyas_sandhi::apply;

let result = apply("अति", "अधिक").unwrap();
assert_eq!(result.output, "अत्यधिक");
```

### Reverse split generation

Use `split` when you have a surface form and want candidate underlying members.

```rust
use varnavinyas_sandhi::split;

let results = split("अत्यधिक");

assert!(
    results.iter().any(|c| c.left == "अति" && c.right == "अधिक"),
    "expected to find अति + अधिक in reverse split candidates"
);
```

### Why reverse split exists

The main purpose of reverse splitting is to reconstruct plausible pre-sandhi forms for:

- compound inspection
- linguistic explanation
- samasa analysis built on top of sandhi candidates

For example, the crate should be able to recover candidates such as:

- `अत्यधिक` -> `अति + अधिक`
- `पुनरवलोकन` -> `पुनः + अवलोकन`

The current API returns `SandhiCandidate` values with confidence and authority metadata, not just raw string pairs.

Use `split_best` for public-facing safe suggestions where false positives must be minimized.

Use `split_best_for_compound` for samasa-style analysis where genuine compounds may only reach `Likely`, but non-classical lexicalized noise should still be filtered.

## Design Notes

- Forward application is deterministic.
- Reverse splitting is necessarily candidate-based: it explores possible internal boundaries and verifies candidates by re-applying forward rules.

## Used By

- `varnavinyas-samasa`
- WASM/browser analysis surfaces
- future linguistic inspection tools

## Current Limits

- Reverse splitting is still an over-generating search with lexical guards, not yet a fully authoritative ranking engine.
- Multiple candidates can be mechanically plausible even when only one is linguistically preferred.

## Status

Useful and tested core subsystem, with reverse-split ranking still needing a stronger evidence model.
