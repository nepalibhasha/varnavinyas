# varnavinyas-eval

Evaluation harness for measuring quality, not just building features.

## What This Crate Owns

This crate runs curated evaluation suites against fixture datasets so the workspace can track regressions and quality drift across:

- orthographic correction
- sandhi
- samasa
- morphology
- grammar-pass behavior

Unlike regular unit tests, this crate is about behavior quality and dataset-backed expectations.

## Test Suites

- `sandhi_eval.rs`
  - known split recall
  - false-positive guard on headword census
- `samasa_eval.rs`
  - expected compound pair and type checks
- `morph_eval.rs`
  - morphology expectations against curated fixtures
- `grammar_eval.rs`
  - grammar-pass expectation checks

## Fixture Sources

- `docs/tests/gold.toml`
- `docs/tests/samasa_gold.toml`
- `docs/tests/morph_gold.toml`
- `docs/tests/grammar_sentences.toml`

## Run

```bash
cargo test -p varnavinyas-eval --tests -- --nocapture
```

## Example

To inspect sandhi quality specifically:

```bash
cargo test -p varnavinyas-eval --test sandhi_eval -- --nocapture
```

That run is meant to answer a focused question such as: “Are recent sandhi changes still recovering known splits without causing too many false positives?”

## Design Notes

- Keep fixtures curated and high-confidence.
- Prefer small, precise test sets over broad noisy datasets.
- This crate should eventually measure ranking quality and confidence calibration, not only binary pass/fail behavior.

## Used By

- maintainers validating regressions
- CI quality gates

## Status

Active evaluation harness for curated regression measurement.
