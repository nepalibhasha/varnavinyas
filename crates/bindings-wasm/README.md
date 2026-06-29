# varnavinyas-bindings-wasm

WebAssembly bridge for browser and JavaScript consumers.

## What This Crate Owns

This crate exposes core Varnavinyas functionality to JavaScript through `wasm-bindgen`. It is the runtime bridge used by:

- the web app
- downstream browser clients, including extensions
- other JS/TS consumers that need in-browser analysis

## Main APIs

The crate exposes browser-friendly entry points. Prefer typed or non-string exports when possible:

- `check_text_value(text, grammar)`
- `check_text_value_with_options(text, grammar, orthography_mode)`
- `check_word_value(word)`
- `derive_value(word)`
- `analyze_word_value(word)`
- `decompose_word_value(word)`
- `analyze_affixes_value(word)`
- `best_affix_analysis_value(word)`
- `has_supported_affix_analysis(word)`
- `analyze_compound_value(word)`
- `sandhi_apply_value(first, second)`
- `sandhi_split_value(word)`
- `sandhi_split_best_for_compound_value(word)`
- `transliterate(input, from, to)`

Legacy JSON-string helpers are also exposed:

- `check_text(text)`
- `check_text_with_options(text, grammar)`
- `check_text_with_all_options(text, grammar, orthography_mode)`
- `check_word(word)`
- `derive(word)`
- `analyze_word(word)`
- `decompose_word(word)`
- `analyze_affixes(word)`
- `best_affix_analysis(word)`
- `analyze_compound(word)`
- `sandhi_apply(first, second)`
- `sandhi_split(word)`
- `sandhi_split_best_for_compound(word)`

## Example

```js
import init, { check_word_value, sandhi_split_value } from "./pkg/varnavinyas_bindings_wasm.js";

await init();

const diag = check_word_value("राजनैतिक");
const diags = check_text_value_with_options("नेपाली कांग्रेस", false, "common-editorial");
const splits = sandhi_split_value("अत्यधिक");
```

## Result Shapes

The exact shapes are defined by Rust serializers in `src/lib.rs`, `varnavinyas-prakriya`, and `varnavinyas-parikshak`.

Important stable fields:

- `check_text_value` and `check_text_value_with_options` return an array of diagnostics with `span_start`, `span_end`, `incorrect`, `correction`, `rule`, `rule_code`, `explanation`, `category`, `category_code`, `kind`, `confidence`, and optional `alternate_reasons`.
- `check_word_value` returns one diagnostic object or `null`.
- `derive_value` returns `input`, `output`, `is_correct`, and `steps`.
- `analyze_word_value` returns word origin, correctness, correction, rule notes, and alternate rule notes.
- `decompose_word_value` returns `root`, `prefixes`, `suffixes`, and `origin`.
- `analyze_affixes_value` returns ranked affix analyses with `surface`, `stem`, `root`, prefix/suffix segments, and `score`.
- `analyze_compound_value` returns samasa candidates with `left`, `right`, `samasa_type`, `score`, and `vigraha`.
- `sandhi_apply_value` returns `output`, `sandhi_type`, `family`, `rule_id`, and `rule_citation`.
- `sandhi_split_value` returns candidates with `left`, `right`, `output`, `sandhi_type`, `family`, `rule_id`, `rule_citation`, `authority`, and `confidence`.

The following are downstream contract:

- export names
- object-vs-null return shape
- top-level field names returned by typed APIs
- local/offline execution

`orthography_mode` accepts `"academy-strict"` or `"common-editorial"`.
Academy-strict is the compatibility default. Common-editorial downgrades only
reviewed common-vs-strict orthographic forms to `kind: "Variant"`.

## Design Notes

- This crate is an adapter. Core behavior should be implemented in the domain crates, not here.
- It should preserve structured information for frontends whenever possible.
- Generated JS/WASM artifacts must stay in sync with the Rust source; stale generated files can break consumers.
- Browser clients should validate required exports during their build.

## Used By

- `web/`
- downstream browser clients consuming the packaged artifact in `web/dist/`

## Current Limits

- Public schema stability should be tightened further for long-lived frontend clients.
- Optional exports and generated artifacts require careful rebuild discipline.

## Build

```bash
bash web/build.sh
```

Browser artifacts for downstream clients are packaged by:

```bash
bash web/package-artifact.sh
```

## Status

Production-facing adapter used by current browser surfaces.
