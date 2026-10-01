# Python, CLI, iOS, and Android v0.1.3

This release updates the orthography engine and matches browser artifact v0.1.6.
All five artifacts use one source commit. Python and CLI package versions are
0.1.3; mobile ZIPs include matching generated bindings, manifests, integration
documentation, checksums, and shared diagnostic/origin fixtures.

## Behavior changes

- Conservative lexical and suffix evidence prevents corrections based on an
  unsupported root or a merely attested alternative. Supported short-vowel
  words and verb readings take precedence over broad fallback rules.
- Reviewed vowel-length, nasal, vocalic-r, gya/gyan and ya/e rules cover more
  supported forms with their precise source citations. For example, `द्रिष्टि`
  offers `दृष्टि`, while `क्रिया`, `ग्यास` and independent short forms remain
  accepted. `दिइ` and `नभइ` offer `दिई` and `नभई`.
- Devanagari shaping variants are compared without changing the original
  diagnostic spans. For example, `अग्‍यान` offers `अज्ञान`, while `पुर्‍याउने`
  remains accepted. Joining corrections preserve intervening punctuation.
- Bounded converb constructions distinguish definite errors from competing
  readings. `पूरा पारि राख्यो` offers `पारी` as an error. In
  `पत्रहरूलाई लेखि पठाइन्`, `लेखी` is an `Ambiguous` suggestion, conditional
  on the intended verb meaning. Reviewed predicate inflections and noun suffixes
  support that reading; `पत्रको` does not establish an object reading.

## Consumer handling

Display `Ambiguous` explanations and require individual review. Never bulk-apply
them as errors, even when `correction` is nonempty. The CLI does not exit with
an error for an ambiguous suggestion alone. Selected context suggestions are
available with `grammar=false` and behave identically in both orthography modes.
Isolated word checks preserve supported short forms such as `लेखि` and `मिलाइ`.
Coverage is bounded; this release does not provide general sentence-level
grammatical disambiguation.

Existing Python imports, classes, enum cases, function signatures and positional
arguments are preserved. Diagnostic schema, category codes, UTF-8 byte offsets,
origin provenance, punctuation defaults, and orthography defaults are unchanged.
`academy-strict` remains the default. Explicit `common-editorial` selection still
reports reviewed alternatives as optional variants. CLI JSON keeps its existing
line/character span convention documented in `CLI_JSON_CONTRACT.md`.

Upgrade both generated Swift/Kotlin bindings and native libraries together.
Compare `source_commit` and fixture hashes across manifests; the matching browser
manifest calls its commit field `git_sha`. Refresh consumer bundles and any
precomputed sample diagnostics. Server-based mobile clients receive engine
changes through the Python dependency; offline clients require native packages.

## Evaluation

Packages carry 24 shared diagnostic cases and eight origin cases. The new cases
cover inflected converb context, valid competing readings, genitive safeguards,
both orthography modes, and original UTF-8 spans. Browser, installed Python,
generated Swift/Kotlin host bindings and CLI harnesses exercise the same cases.
iOS additionally type-checks generated Swift against the simulator SDK.
Device loading, mobile text-index conversion and app UI behavior still require
consumer integration tests. These mobile archives are offline evaluation
packages, not App Store or Play Store apps. Python wheels are distributed through
GitHub Releases, not PyPI.
