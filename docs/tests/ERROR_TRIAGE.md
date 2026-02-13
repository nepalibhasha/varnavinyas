# Grammar Eval Error Triage

Use this workflow when `varnavinyas-eval` fails or when manual QA finds grammar-pass errors.

## Labels

### False positives
- `rule_too_broad`: heuristic is over-triggering for valid text.
- `lexicon_gap`: missing lexicon entries cause wrong heuristic context.
- `origin_misclass`: tatsam/tadbhav/origin signal is wrong and misroutes rules.

### False negatives
- `missing_rule`: no heuristic exists yet for this pattern.
- `coverage_gap`: heuristic exists but misses common morphology/surface variants.
- `ranking_error`: good candidate exists but is dropped by confidence/ranking.

## Triage Steps

1. Reproduce with:
   - `cargo test -p varnavinyas-eval --test grammar_eval -- --nocapture`
2. Isolate sentence and observed diagnostics.
3. Assign one primary label from above.
4. Add/adjust fixture in `docs/tests/grammar_sentences.toml`.
5. Add regression test near changed behavior:
   - `crates/parikshak/tests/grammar.rs`
6. Re-run:
   - `cargo test -p varnavinyas-parikshak --features grammar-pass -q`
   - `cargo test -p varnavinyas-eval --test grammar_eval -q`

## Issue Template

- Sentence:
- Expected behavior:
- Actual behavior:
- Label:
- Proposed fix:
- Added tests/fixtures:
