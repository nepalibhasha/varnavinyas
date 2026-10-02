# Python, CLI, iOS, and Android v0.1.5

This release matches browser artifact v0.1.8. All five releases use one source
commit. Python, CLI, and UniFFI package versions are 0.1.5. Mobile archives
include matching generated bindings, native libraries, manifests, checksums,
evaluation harnesses, and shared diagnostic/origin fixtures.

## Behavior and source explanations

`संघीय`, `कांग्रेस`, and `संकेत` retain their existing corrections:
`सङ्घीय`, `काङ्ग्रेस`, and `सङ्केत`. They remain `Error` in the default
`academy-strict` mode and reviewed optional `Variant` diagnostics in
`common-editorial`. This release does not expand the reviewed variant registry.

The Academy Notice explicitly gives `सङ्केत` under `3(ख)(अ)-2` and
`संघीय → सङ्घीय` in Section 4. Neither normative rulebook names `कांग्रेस`.
The dictionary marks `काङ्ग्रेस` as an English loanword and also contains
`काँग्रेस`. Dictionary-backed loan/tadbhav nasal spellings now cite
`3(ख)(अ)-3-lex` and explain pronunciation-based lexical spelling rather than
incorrectly claiming tatsama origin. The distinct `काँग्रेस` alternate remains
available to callers of the additive rule-hit API. Lexical chandrabindu
explanations also describe dictionary evidence directly.

The web UI now exposes saved strict/editorial choices, counts optional variants
separately from errors, and shows them without an error strike-through or a
confidence percentage. Its rules reference describes the numbered rules and
their source examples. These UI changes are deployed separately from native
packages and the browser WASM ZIP.

## Consumer handling and compatibility

Existing calls, enum members, diagnostic payload fields, category codes,
orthography defaults, punctuation defaults, and span conventions are preserved.
API and diagnostic schema versions remain unchanged. The lexical rule citation
above is more specific; consumers should map unfamiliar rule codes through the
stable category code rather than relying on a complete hardcoded rule list.

Present reviewed `Variant` diagnostics as optional forms. Count them separately
from errors and exclude them from bulk correction. The `correction` field
remains the preferred strict spelling. Preserve the existing handling of
`Ambiguous` contextual suggestions and `samasa-heuristic` information.

Pin browser v0.1.8 and native v0.1.5 together, compare browser `git_sha` with
native `source_commit`, and verify fixture hashes. Upgrade generated Swift/Kotlin
bindings together with their native libraries. Rebuild consumer bundles and
refresh precomputed diagnostics. Python wheels are distributed through GitHub
Releases, not PyPI.

## Evaluation

The packages share 44 diagnostic fixtures and eight origin fixtures. The two
new diagnostic cases cover all three reviewed nasal spellings in strict and
editorial modes, including the congress source explanation. Browser, installed
Python, CLI, and generated Swift/Kotlin host bindings evaluate these fixtures.
iOS additionally type-checks generated Swift against the simulator SDK.

iOS includes arm64 device and arm64/x86_64 simulator XCFramework libraries.
Android includes arm64-v8a, armeabi-v7a, and x86_64 native libraries. These are
offline integration/evaluation packages; device loading and client UI behavior
still require consumer integration tests.
