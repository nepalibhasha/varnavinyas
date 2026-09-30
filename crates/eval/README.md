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
  - split-activity budget on an unlabeled headword census
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

That run measures known-pair recall and how often the headword corpus produces
splits. The corpus has no per-word correctness labels, so its split rate is not
a measured false-positive rate. Negative fixtures provide a separate, limited
false-positive check.

## Current Gates And Known Gaps

- Checker gold: all 97 word entries require an expected correction and a cited
  explanation; correct-form and 13 paragraph fixtures are checked separately.
- Sandhi: all currently covered expected pairs are required individually.
  अत्याचार (long-आ reconstruction) and विद्यार्थी (morphology-first removal of
  final ई) remain named gaps in the 10-pair recall denominator. A generic
  three-of-ten floor no longer allows covered examples to regress.
- Samasa: both confirmed pairs and their types are required. The third fixture,
  महोत्सव, explicitly records its मह/महा disagreement in `pair_review`; review
  linguistic evidence before changing the fixture or promoting a different split.
- Morphology: 22 curated examples pass; this is not comprehensive paradigm coverage.
- Grammar: seven sentences include only two required positive results, both
  `samasa-heuristic`. `रामले गयो।` and `रामको किताबहरु हराए।` still have disabled
  expected grammar detections. Orthographic corrections on those sentences do
  not establish grammar accuracy.

Next, add source-backed positive/near-negative pairs for the deferred verb and
gender classes, phrase boundaries, and enabled case/agreement rules. A new gap
must have an explicit review reason; broad percentage thresholds must not hide
regressions in previously covered examples.

## Design Notes

- Keep fixtures curated and high-confidence.
- Prefer small, precise test sets over broad noisy datasets.
- This crate should eventually measure ranking quality and confidence calibration, not only binary pass/fail behavior.

## Used By

- maintainers validating regressions
- CI quality gates

## Status

Active evaluation harness for curated regression measurement.
