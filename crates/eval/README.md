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
  - reviewed winning compound pair, type and vigraha checks
  - negative evidence cases
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
  अत्याचार now round-trips as अति + आचार. The pipeline uses `best_analysis`
  for a supported root, matching the inspector, so विद्यार्थी retains its ई
  and splits as विद्या + अर्थी. All ten pairs are now required; there are no
  named exemptions or generic three-of-ten floor. The legacy `decompose` API
  remains available; its speculative roots are not the eval's supported stems.
- Samasa: five reviewed winning pairs, types and vigraha are required, alongside
  fourteen negative evidence cases. महोत्सव uses महा + उत्सव after dictionary
  review. The public analyzer requires reviewed formation evidence; raw sandhi
  remains exploratory. These fixtures do not measure general compound recall.
- Morphology: 22 curated examples pass; this is not comprehensive paradigm coverage.
- Grammar: fourteen sentences include negative compound cases, supported outer
  inflections and the existing contextual converb suggestion. `रामले गयो।` and `रामको किताबहरु हराए।` still have disabled
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
