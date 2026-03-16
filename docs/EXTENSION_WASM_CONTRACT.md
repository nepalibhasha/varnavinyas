# Extension WASM Contract

## Purpose

This document defines the browser-facing `varnavinyas` WASM contract intended for downstream extension clients.

It exists so downstream clients do not depend on repo layout or informal assumptions.

## Artifact

The browser artifact is produced from this repo and contains:

- `pkg/varnavinyas_bindings_wasm.js`
- `pkg/varnavinyas_bindings_wasm_bg.wasm`
- `build-info.json`
- `manifest.json`

Packaging command:

```bash
bash web/package-artifact.sh
```

Output:

- `web/dist/varnavinyas-browser-artifact/`
- `web/dist/varnavinyas-browser-artifact.zip`
- `web/dist/varnavinyas-browser-artifact-<version>.zip`

Published release path:

- GitHub releases tagged as `browser-artifact-v*`
- release asset: `varnavinyas-browser-artifact-v<semver>.zip`

## Required Exports

Downstream extension clients may rely on these typed exports:

- `check_word_value(word)`
- `analyze_word_value(word)`
- `decompose_word_value(word)`
- `sandhi_split_value(word)`
- `analyze_compound_value(word)`

These are currently defined in:

- [`crates/bindings-wasm/src/lib.rs`](../crates/bindings-wasm/src/lib.rs)

## Behavioral Contract

### Offline-first requirement

These exports must work entirely locally in browser/WASM context.

They must not require:

- downstream API access
- network access
- extension background worker mediation

### Stability expectation

The following should be treated as downstream contract:

- export names
- object-vs-null return shape
- top-level field names returned by typed APIs
- artifact manifest fields
- artifact zip contents

Breaking changes should be deliberate and documented.

## Expected Result Shapes

The exact shape is defined by the Rust serializers, but downstream clients should expect:

### `check_word_value(word)`

Returns:

- `null` if no diagnostic is produced
- otherwise a diagnostic object derived from `ApiDiagnostic`

Important top-level fields include:

- `incorrect`
- `correction`
- `rule`
- `rule_code`
- `explanation`
- `category`
- `category_code`
- `kind`
- `confidence`
- optional `alternate_reasons`

### `analyze_word_value(word)`

Returns a word-analysis object derived from `ApiWordAnalysis`.

Important top-level fields may include:

- `word`
- `origin`
- `origin_source`
- `origin_confidence`
- `is_correct`
- `correction`
- `rule_notes`

### `decompose_word_value(word)`

Returns a morpheme decomposition object with:

- `root`
- `prefixes`
- `suffixes`
- `origin`

### `sandhi_split_value(word)`

Returns an array of sandhi candidates.

Important fields include:

- `left`
- `right`
- `output`
- `sandhi_type`
- `family`
- `rule_id`
- `rule_citation`
- `authority`
- `confidence`

### `analyze_compound_value(word)`

Returns an array of samasa candidates.

Important fields include:

- `left`
- `right`
- `samasa_type`
- `score`
- `vigraha`

## Non-goals

This contract does not include:

- downstream API lookup
- extension host permissions
- popup/background/content logic
- downstream branding or navigation

Those belong to the downstream client repo.
