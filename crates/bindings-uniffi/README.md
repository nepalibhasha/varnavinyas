# varnavinyas-bindings-uniffi

UniFFI bridge for Swift/Kotlin-style consumers.

## What This Crate Owns

This crate provides a higher-level foreign-function interface for platforms that integrate well with UniFFI, especially:

- iOS / Swift
- Android / Kotlin
- other native clients that benefit from generated bindings

## What It Exposes

The exported API currently focuses on a compact core:

- `check_text`
- `check_text_with_options`
- `check_text_with_all_options`
- `check_word`
- `transliterate`
- `classify`

It also exports the `Scheme`, `Origin`, `PunctuationMode`, and
`OrthographyMode` enums used by those functions.

## Example

Conceptually, native consumers use generated bindings for the exported functions:

```text
check_text("यो बाक्यमा गल्ति छ")
check_text_with_options("यो बाक्यमा गल्ति छ", false, PunctuationMode.Strict, false)
check_text_with_all_options("नेपाली कांग्रेस", false, PunctuationMode.Strict, OrthographyMode.CommonEditorial, false)
check_word("अध्यन")
transliterate("नेपाल", Devanagari, Iast)
classify("नेपाल")
```

The exact call shape depends on the generated Swift/Kotlin package, but the exported Rust API is the function set above.

## Design Notes

- This crate is meant to be a stable language boundary, not the main place to add features first.
- It should mirror trusted core semantics from Rust.
- As the core gains richer structured outputs, this crate should move away from string-heavy contracts where possible.

## Used By

- mobile applications
- native clients that want generated bindings instead of manual C FFI

## Current Limits

- The surface area is intentionally narrower than the Rust API.
- Some outputs are still simplified for portability.

## Status

Implemented MVP integration layer.
