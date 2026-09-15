# Browser Artifact v0.1.5 — Proposed Release Notes

Status: local candidate; not published.

This release updates the compiled lexicon and makes correction explanations
more closely reflect the available source and grammatical evidence.

## Changes

- Refreshed lexicon metadata and reviewed rule inventories improve the evidence
  available for corrections and origin notes.
- Context-dependent root/imperative forms such as `हेर` / `हेर्` are preserved
  instead of entering a correction cycle. Word analysis explains the distinction.
- Word analysis reports unknown origins explicitly and distinguishes inferred
  origins from dictionary metadata or reviewed evidence.
- Morphology requires independent evidence for roots: `संघीय` no longer gets
  the unsupported `सम् + घीय` split.
- `हामि → हामी` keeps the pronoun explanation and drops unsupported suffix and
  redundant generic explanations. Accepted `हामी` no longer splits as `हाम + ई`;
  documented derivations such as `तिनी = तिन + ई` remain available.
- Rule selection and text-level arbitration use more specific evidence, and
  derivation steps expose `rule_code` for source-reference links.

## Consumer Compatibility

The browser artifact API and diagnostic schema versions remain `1`. Existing
exports remain available. Clients must accept `origin: "unknown"` and
`origin_source: "unknown"` in word analysis, with zero origin confidence.
Show heuristic origins as inferred; do not label missing evidence as देशज.
Use word analysis rather than the coarse decomposition origin for badges.

Corrections, morphology candidates, and alternate explanation counts can change.
Use `rule_code` and `category_code` as stable identifiers; do not depend on
display labels or a fixed number of reasons. See [Integration Notes](../INTEGRATION_NOTES.md).

The ZIP contains WASM and JavaScript glue, not the website UI. The website's
context-aware inspector and origin badges deploy separately through GitHub
Pages; downstream clients need equivalent handling in their own UI.

## Publication Follow-up

Before tagging a final release, resolve or explicitly assess the dependency
advisory failures reported by the current CI run, rerun the release checks,
and verify downstream handling of the new origin values. Build the final
artifact from the accepted committed revision; do not promote an older local
candidate after changing its source.
