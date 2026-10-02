# Browser artifact v0.1.8

This release matches Python, CLI, iOS, and Android v0.1.5 from one source commit.
It improves nasal spelling explanations: dictionary-backed loan/tadbhav forms
use the lexical citation `3(ख)(अ)-3-lex` instead of claiming tatsama origin.
The existing `कांग्रेस → काङ्ग्रेस` correction and distinct `काँग्रेस`
alternate remain available. See the matching
[native release notes](https://github.com/nepalibhasha/varnavinyas/blob/browser-artifact-v0.1.8/docs/releases/native-v0.1.5.md)
for source distinctions and consumer guidance.

`संघीय`, `कांग्रेस`, and `संकेत` remain errors in the default
`academy-strict` mode and optional `Variant` diagnostics in `common-editorial`.
Existing WASM exports, payload fields, stable category codes, UTF-8 spans,
origin provenance, and API/schema versions are preserved. Consumers should
handle unfamiliar rule citations through their stable category mapping.

The separately deployed web UI exposes saved strict/editorial choices, counts
variants separately, removes their error strike-through and confidence
percentage, and aligns the rules reference with Academy source examples.
The browser artifact ZIP contains WASM and JavaScript glue; consumer-owned
renderers must implement the optional-variant presentation themselves and keep
variants out of bulk correction.

Upgrade the browser artifact pin to v0.1.8, rebuild bundles, and refresh cached
diagnostics. Compare `git_sha` with native v0.1.5 manifests' `source_commit`.
The runtime checks evaluate all 44 shared diagnostic cases and preserve
the existing compound-information and ambiguous-suggestion handling.
