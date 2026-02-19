# varnavinyas-lipi

**Transliteration Engine for Nepali.**

Handles conversion between Devanagari and various romanization schemes, as well as legacy font conversion.

## Features
- **Schemes**: Devanagari ↔ IAST, ISO-15919.
- **Legacy Support**: Convert Preeti / Kantipur (TTF) encodings to Unicode.

## Usage

```rust
use varnavinyas_lipi::{transliterate, Scheme};

let rom = transliterate("नेपाल", Scheme::Devanagari, Scheme::Iast).unwrap();
assert_eq!(rom, "nepāla");
```

## Status
✅ Stable
