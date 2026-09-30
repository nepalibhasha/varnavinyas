# varnavinyas-lekhya

Punctuation and writing-convention checks for Nepali text.

## What This Crate Owns

This crate is the punctuation-focused checker. It implements the Section 5 style and punctuation rules that are separate from word-level spelling correction.

It is responsible for:

- sentence-final punctuation checks
- quote normalization checks
- spacing around punctuation
- selected bracket, slash, and ellipsis conventions

## Main API

- `check_punctuation(&str)` -> returns `LekhyaDiagnostic` entries with spans, found text, expected text, and rule text
- `dotted_abbreviation_spans(&str)` -> UTF-8 byte spans for compact Devanagari initial chains, shared with the text checker

## Example

```rust
use varnavinyas_lekhya::check_punctuation;

let diagnostics = check_punctuation("यो वाक्य गलत छ.");
assert!(!diagnostics.is_empty());
assert_eq!(diagnostics[0].expected, "।");
```

## Design Notes

- `lekhya` is intentionally separate from `prakriya` because punctuation rules operate on running text, not isolated word forms.
- It should remain deterministic and explicit about what is a hard violation versus editorial normalization.
- Compact dotted abbreviations such as `बी.बी.सी.` and `सी.डी.ओ.` retain their dots, including at EOF or a line break. PS-Saisanik hrasva/dirgha `(छ)` preserves their long vowels; `parikshak` uses the same spans to avoid checking each initial as an ordinary word. This guard does not normalize abbreviation vowels or recognize all spaced initial chains.

## Used By

- `varnavinyas-parikshak`
- downstream surfaces such as CLI, LSP, and WASM consumers

## Current Limits

- Some checks are context heuristics, especially quote direction and abbreviation handling.
- It is a text scanner, not a full syntactic punctuation parser.

## Status

Stable punctuation subsystem with room for deeper context modeling.
