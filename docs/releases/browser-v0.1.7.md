# Browser artifact v0.1.7

This release matches Python, CLI, iOS, and Android v0.1.4 from one source commit.
It fixes unsupported compound splits, false spacing corrections such as
`वायुसेवा -> वायु सेवा`, and progressive forms such as `खोजिरहेको`.
Reviewed prescriptions such as `नेपालसरकार -> नेपाल सरकार` remain active.
See the matching [native release notes](https://github.com/nepalibhasha/varnavinyas/blob/browser-artifact-v0.1.7/docs/releases/native-v0.1.4.md)
for examples, compatibility, and evaluation coverage.

Existing WASM exports, diagnostic fields, stable category/rule codes, UTF-8
spans, origin provenance, and orthography-mode defaults are preserved.
`academy-strict` remains the default; `common-editorial` remains available.
Additive `analyze_progressive()` and `analyze_progressive_value()` return
supported main/auxiliary forms and their verb lemmas. The manifest advertises
`progressive_verb_analysis` and lists affix roles in `affix_segment_kinds`,
including new `postposition` and `comparison_marker` values.

With grammar enabled, `samasa-heuristic` provides reviewed compound information.
Display it as **बनोट**, not an error or a replacement arrow. Never apply its
structural `correction` string to the document, individually or in bulk, and
do not present its ranking weight as an accuracy percentage. Updating WASM
removes unsupported engine hints; consumer-owned renderers need to adopt this
presentation separately. The Varnavinyas web UI includes the rendering fix.

Spelling and पदयोग/पदवियोग checks still run with grammar disabled. A clicked
word's structure is available independently of that toggle. Grammar enables
selected compound information and optional grammar/style suggestions; it does
not provide complete sentence parsing or arbitrary compound decomposition.

Upgrade the artifact pin from 0.1.6 to 0.1.7, rebuild WASM/JavaScript bundles,
and refresh precomputed diagnostics. Match `git_sha` against native manifests'
`source_commit`, update matching offline mobile bindings and libraries, and
test original UTF-8 spans with emoji and repeated words. The browser runtime
evaluates all 42 shared diagnostic fixtures.
