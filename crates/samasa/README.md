# varnavinyas-samasa

Compound analysis layer for identifying likely samasa interpretations.

## What This Crate Owns

This crate analyzes a word as a possible compound and proposes ranked interpretations of its members.

It is responsible for:

- taking possible split candidates
- validating them against lexical presence
- assigning a provisional samasa type
- returning ranked candidate analyses

Conceptually, this crate sits on top of `sandhi` and `kosha`.

## Main API

- `analyze_compound(&str)` -> returns ranked `SamasaCandidate` values

Each candidate currently includes:

- `left`
- `right`
- `samasa_type`
- `score`
- `vigraha`

## Example

```rust
use varnavinyas_samasa::analyze_compound;

let candidates = analyze_compound("अत्यधिक");

if let Some(top) = candidates.first() {
    let _pair = (&top.left, &top.right);
    let _score = top.score;
}
```

The goal is to produce ranked compound interpretations that higher-level tools can inspect, not to force a single answer in every case.

## Depends On

- `varnavinyas-sandhi`
- `varnavinyas-kosha`

## Design Notes

- This crate is currently a heuristic analyzer, not a formal grammatical proof engine.
- Ranking is useful for hinting, but scores should be interpreted as plausibility, not certainty.

## Current Limits

- It inherits over-generation from reverse sandhi splitting.
- Many candidate classifications are still shallow lexical heuristics.
- It is not yet strong enough to declare one split “the correct one” in all cases.

## Status

MVP compound analyzer with useful ranking, but not yet authoritative disambiguation.
