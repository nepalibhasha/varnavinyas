# Python, CLI, iOS, and Android v0.1.4

This release matches browser artifact v0.1.7. All five releases use one source
commit. Python and CLI package versions are 0.1.4. The offline mobile archives
include matching generated Swift/Kotlin bindings, native libraries, manifests,
checksums, evaluation harnesses, and shared diagnostic/origin fixtures.

## Behavior changes

- Unsupported compound interpretations are suppressed. `विकास`, `आयात`,
  `आर्थिक`, `सवारीमा`, and `यातायात` no longer acquire speculative समास splits.
  Reviewed compounds such as `सूर्योदय` and `पूर्वाधार` retain their analyses;
  `पूर्वाधारमा` preserves the outer `मा` separately.
- Spacing corrections require a reviewed prescription for the complete pair.
  `वायुसेवा`, `वायुसेवाको`, `जनसेवा`, and `राज्यव्यवस्था` are accepted instead
  of being split merely because their components are known words. Explicit
  prescriptions such as `नेपालसरकार -> नेपाल सरकार` and
  `समाजसेवा -> समाज सेवा` remain corrections.
- Dictionary-backed progressive forms such as `खोजिरहेको`, `खोजिरहेकी`, and
  `खोजिरहेका` are accepted rather than changed to `खोजिरहनेको/की/का`.
  `खोजिरहेकाले` preserves the progressive base plus outer `ले`. The additive
  progressive API identifies `खोजि` (lemma `खोज्नु`) and `रहेको` (lemma
  `रहनु`). `उर्लिरहेको` and `ओर्लिरहेको` retain their distinct verb lemmas.
- Reviewed relational suffixes have more precise roles. `घरसम्म` has host
  `घर` and postposition `सम्म`; `मानिससरह` has host `मानिस` and comparison
  marker `सरह`. Joining rules recognize `घर सम्म -> घरसम्म` and
  `मानिस सरह -> मानिससरह`, while preserving case-bearing contexts such as
  `घरको सम्म` and `मानिसको सरह`.

These spelling/spacing fixes work with grammar on or off, in both orthography
modes. Compound information in text diagnostics requires grammar enabled.
The checker is not a complete sentence grammar parser; an absent structural
analysis does not establish a spelling error.

## Consumer handling and compatibility

`samasa-heuristic` is information, not a correction. Its existing `correction`
field contains a structural string such as `सूर्य + उदय`. Never apply this
string to text, individually or in bulk. Display it as **बनोट**, without
striking through the original word or showing an accuracy percentage. Its
`Variant` kind, `Sandhi` category code, rule code, and payload shape are
unchanged. See `COMPOUND_ANALYSIS.md` in the native packages.

Existing Python imports, classes, functions, signatures, positional arguments,
and enum members are preserved. `shabda.analyze_progressive()` adds a read-only
`ProgressiveAnalysis` or `None`. `AffixKind.Postposition` and
`AffixKind.ComparisonMarker` are appended; consumers matching every affix role
must handle these new members. UniFFI adds optional `analyzeProgressive()`
results; existing calls remain available. Upgrade generated Swift/Kotlin code
and native libraries together. CLI check JSON retains its existing shape and
line/character span convention.

`academy-strict` remains the default. Explicit `common-editorial` selection
continues to report reviewed orthography alternatives as optional variants.
Keep `Ambiguous` contextual suggestions out of bulk correction and display
their explanations. Diagnostic category codes, UTF-8 byte offsets for bindings,
origin provenance, and punctuation defaults are unchanged.

Pin browser v0.1.7 and native v0.1.4 together. Compare native manifests'
`source_commit` with the browser manifest's `git_sha`, and compare diagnostic
and origin fixture hashes across native packages. Rebuild consumer bundles and
refresh precomputed diagnostics. Updating the engine does not update a
consumer-owned card renderer; follow the information policy above. Python wheels
are distributed through GitHub Releases, not PyPI.

## Evaluation

The packages include 42 shared diagnostic cases and eight origin cases covering
both orthography modes, unsupported compounds, preserved inflections,
progressive verbs, relational joining, reviewed institutional spacing, and
UTF-8 spans. Browser, installed Python, CLI, and generated Swift/Kotlin host
bindings evaluate the same diagnostic fixtures. Mobile harnesses also verify
progressive analysis; iOS type-checks generated Swift against the simulator SDK.

The iOS archive contains device and simulator XCFramework slices. Android
contains arm64-v8a, armeabi-v7a, and x86_64 libraries. These are offline evaluation
packages, not App Store or Play Store apps. Device loading, client text-index
conversion, and app UI behavior still require consumer integration tests.
