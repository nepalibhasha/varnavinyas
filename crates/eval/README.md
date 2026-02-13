# varnavinyas-eval

Evaluation test suites for regression tracking across sandhi, samasa, morphology, and grammar-pass behavior.

## Test suites

- `sandhi_eval.rs`
  - split recall sanity for known examples
  - headword census with false-positive guard (`split rate < 15%`)

- `samasa_eval.rs`
  - validates expected compound pair + type from `docs/tests/samasa_gold.toml`

- `morph_eval.rs`
  - validates `vyakaran` MVP analyses against `docs/tests/morph_gold.toml`

- `grammar_eval.rs`
  - validates grammar-pass diagnostic expectations from `docs/tests/grammar_sentences.toml`

## Run commands

```bash
cargo test -p varnavinyas-eval --test sandhi_eval -- --nocapture
cargo test -p varnavinyas-eval --test samasa_eval -- --nocapture
cargo test -p varnavinyas-eval --test morph_eval -- --nocapture
cargo test -p varnavinyas-eval --test grammar_eval -- --nocapture
```

Run all eval tests:

```bash
cargo test -p varnavinyas-eval --tests -- --nocapture
```

## Dataset locations

- `docs/tests/gold.toml`
- `docs/tests/samasa_gold.toml`
- `docs/tests/morph_gold.toml`
- `docs/tests/grammar_sentences.toml`

Keep fixtures high-confidence and deterministic. Add small curated sets first, then expand with measured threshold updates.

## Triage

When eval failures occur, classify and track them using:

- `docs/tests/ERROR_TRIAGE.md`
