# varnavinyas-lipi

Transliteration and script-detection utilities for Varnavinyas.

## What This Crate Owns

This crate converts text between supported writing schemes and detects likely input schemes.

It is the script-conversion layer for:

- Devanagari <-> IAST transliteration
- optional legacy font decoding paths behind feature flags

## Main APIs

- `transliterate(&str, Scheme, Scheme)` -> convert between supported schemes
- `detect_scheme(&str)` -> best-effort input scheme detection

## Supported Schemes

Current built-in schemes:

- `Devanagari`
- `Iast`

Optional legacy schemes behind the `legacy` feature:

- `Preeti`
- `Kantipur`

## Example

```rust
use varnavinyas_lipi::{Scheme, transliterate};

let rom = transliterate("नेपाल", Scheme::Devanagari, Scheme::Iast).unwrap();
assert_eq!(rom, "nepāla");
```

```rust
use varnavinyas_lipi::detect_scheme;

assert!(detect_scheme("नेपाल").is_some());
```

## Used By

- CLI transliteration commands
- bindings crates
- web-facing surfaces that need script conversion

## Current Limits

- Scheme detection is intentionally simple and can be ambiguous for plain ASCII input.
- This crate is not yet a full multi-scheme romanization framework.

## Status

Stable script-conversion utility crate.
