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
- `classify_with_provenance`

It also exports the `Scheme`, `Origin`, `PunctuationMode`, and
`OrthographyMode` enums used by those functions. `OriginDecision` is a record
with nullable `origin`, `OriginSource` (`Override`, `Kosha`, `Heuristic`,
`Unknown`), and `confidence`. Unknown has no origin and zero confidence.

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
- `classify` returns a best-effort four-way origin category without provenance.
  Use `classify_with_provenance` for explanations, labelling heuristic results
  as inferred and absent origin as unknown. A fallback `Deshaj` is not verified etymology; see
  [Integration Notes](../../docs/INTEGRATION_NOTES.md).

## Status

Implemented MVP integration layer.

## Offline Evaluation Artifacts

The iOS and Android v0.1.4 ZIPs include generated Swift/Kotlin bindings, native
libraries, manifests with the source commit and file checksums, and identical
diagnostic fixtures for both orthography modes plus origin evidence fixtures.
Upgrade generated bindings and native libraries together. The iOS XCFramework includes
the generated C header and module map for each platform slice.

See [Mobile Evaluation](../../docs/MOBILE_EVALUATION.md) for integration,
UTF-8 span handling, dependencies, and the included evaluation harnesses.

## Compound information

Compound hints require reviewed formation evidence and preserve outer suffixes.
`samasa-heuristic` is informational; never apply its structural `correction`
string to text or present its ranking weight as an accuracy percentage. Public
signatures and diagnostic fields are unchanged. See the
[compound-analysis contract](../../docs/COMPOUND_ANALYSIS.md) for behavior and
artifact integration requirements.

`analyze_progressive` generates Swift/Kotlin `analyzeProgressive`, returning an
optional `ProgressiveAnalysis` with the main linking form and both verb lemmas.
For `खोजिरहेको`, these are `खोजि`, `खोज्नु`, `रहेको`, and `रहनु`. Existing APIs
are unchanged; regenerate bindings together with the native library. See
[the analysis contract](../../docs/INTEGRATION_NOTES.md#progressive-verb-constructions).
