# varnavinyas-parikshak Architecture

This document describes the high-level architecture of `varnavinyas-parikshak`.

The goal of this crate is explicit: take text input, run the relevant
token-level and text-level checks, and return unified diagnostics with spans and
categories.

## Goals

- Keep `prakriya` as the source of token-level orthographic truth.
- Keep text-level context decisions in `parikshak`, not in token derivation.
- Combine hard errors, variants, and heuristic suggestions in one pipeline.
- Preserve stable category and rule metadata for UI, CLI, and bindings.

## Core Types

The main public types are:

- `Diagnostic`: one flagged span in text
- `DiagnosticCategory`: stable outward-facing category code set
- `CheckOptions`: runtime knobs for grammar and punctuation behavior
- `Token` / `AnalyzedToken`: tokenization outputs for text passes

`DiagnosticReason` is a type alias to `varnavinyas_prakriya::Explanation`, so
alternate applicable reasons share the same outward-facing explanation model as
`WordAnalysis`.

## Pipeline

`check_text_with_options(text, options)` follows this order:

```mermaid
flowchart TD
    A[Input text]
    B[Tokenize]
    C[check_word / word-level checks]
    D[Tiryak pass]
    E[Padayog / padabiyog passes]
    F[Arbitrate overlaps]
    G[Context diagnostics]
    H[Optional style / grammar]
    I[Arbitrate overlaps]
    J[Punctuation]
    K[Sort by span]
    L[Diagnostics]

    A --> B --> C --> D --> E --> F --> G --> H --> I --> J --> K --> L
```

1. tokenize text into `AnalyzedToken`s
2. run token-level orthography checks through `check_word`
3. adjust context-sensitive token cases that need neighboring token context
4. run `PS-Saisanik` tiryak diagnostics
5. run text-level padayog/padabiyog passes
6. arbitrate word/tiryak/padayog overlaps
7. run context-sensitive diagnostics
8. optionally run style/grammar heuristic passes
9. arbitrate overlaps again after late passes
10. run punctuation diagnostics
11. filter noop heuristics if disabled
12. sort diagnostics by span

This keeps high-confidence token corrections early, lets later passes respect
blocked spans, and makes cross-pass overlap behavior explicit in
`checker/arbitration.rs`.

## Token Contract

`check_text_with_options` tokenizes once into `AnalyzedToken`s. Text-level
passes should consume that shared token slice instead of re-splitting the source
string. The old `whitespace_segments` helper has been removed, so new passes
cannot silently reintroduce the pre-migration path.

The token contract is:

- `start`/`end` are byte offsets into the original input and cover the token
  surface, excluding boundary punctuation.
- `source_text(text)` returns the exact original substring for the token span.
- `stem` and `suffix` expose conservative suffix/particle detachment; if a
  suffix is detached, `surface()` reconstructs the full token form.
- `span()` is the stable unit for blocking and conflict checks.
- `is_devanagari_word()` and `is_numeric()` centralize the filters that
  padayog, particle, and style passes previously computed locally.
- Neighbor-sensitive passes should use `tokens.windows(2)` or
  `tokens.windows(3)` so phrase spans are formed from the first and last token
  offsets.
- Attached punctuation remains outside token spans. Exact literal phrase
  rewrites may still use source-string matching and boundary checks when they
  intentionally need punctuation-sensitive behavior.

### Example

```text
input text:  यो बाक्यमा गल्ति छ.

possible passes:
- word-level:  बाक्यमा -> वाक्यमा
- punctuation: final '.' under Nepali punctuation policy
```

One input string can therefore produce multiple diagnostics owned by different
passes in the same pipeline.

## Module Ownership

- `checker.rs`
  Owns pipeline orchestration and runtime options.

- `checker/arbitration.rs`
  Owns private diagnostic overlap arbitration. Current precedence is
  `kind > specificity > pass > confidence`, with a composite padayog exception
  for broader rewrites that already include the nested correction.

- `checker/word_level.rs`
  Owns token-level integration with `prakriya` and `kosha`.

- `checker/padayog.rs`
  Owns text-level padayog/padabiyog passes.

- `checker/padayog_rules.rs`
  Owns explicit `(घ)` rewrite tables backing the padayog pass.

- `checker/punctuation.rs`
  Owns punctuation diagnostics.

- `checker/style_variants.rs`
  Owns style-only phrase suggestions.

- `checker/grammar.rs`
  Owns optional grammar-aware heuristics behind the feature gate.

- `tokenizer.rs`
  Owns practical tokenization and suffix-aware token analysis.

- `diagnostic.rs`
  Owns the outward-facing diagnostic model and stable category codes.

- `presentation.rs`
  Owns serializable DTOs for bindings and CLI/JSON output.

## Boundary With `prakriya`

`prakriya` is token-centric and deterministic.

`parikshak` adds:

```text
prakriya   -> "this token should become that token"
parikshak  -> "this span in this text should be flagged"
```

- tokenization
- lexicon-aware ambiguity handling
- phrase-level join/split passes
- punctuation
- optional grammar/style heuristics
- span management

Practical rule:

- if a rule transforms one token into another token, prefer `prakriya`
- if a rule needs neighboring tokens, spacing, or sentence context, prefer `parikshak`

Section `3(घ)` is the clearest example of this boundary.

### Example

```text
prakriya:   राजनैतिक -> राजनीतिक
parikshak:  तलमाथि -> तल माथि
```

The first is a one-token normalization problem. The second is a text-level
join/split problem.

## Category Contract

`DiagnosticCategory` is a stable outward-facing contract.

UI code, CLI JSON, bindings, and the LSP all rely on its `category_code`
values. Category evolution therefore requires synchronized updates across:

- Rust category mapping
- web category metadata
- CSS category selectors
- smoke tests and any UI assumptions

This is why category additions are small but cross-cutting changes.

### Example

If a new category code is introduced in Rust, the same change usually has to be
reflected in:

- `web/js/rules-data.js`
- `web/js/utils.js`
- `web/css/style.css`
- any smoke tests that assume the category set

## Arbitration Contract

`parikshak` currently uses both legacy `blocked_spans` and explicit arbitration.
`blocked_spans` prevents later passes from proposing obvious overlaps, while
`checker/arbitration.rs` resolves candidates that still collide after pass
execution.

The public contract is documented in [`ARBITRATION.md`](./ARBITRATION.md). The
important current rules are:

- non-overlapping diagnostics survive together
- for overlapping diagnostics, precedence is `kind > specificity > pass > confidence`
- same-span duplicate corrections merge alternate reasons instead of surfacing
  duplicate primary diagnostics
- broader padayog rewrites can suppress a nested diagnostic only when the
  broader replacement already contains the nested correction
- punctuation is appended after the final resolver today, so punctuation overlap
  arbitration remains future work

Any change to these rules should add focused arbitration tests and run the
corpus snapshot gate.

## Design Principles

- Prefer explicit passes over generic rule frameworks.
- Keep pass ownership local to the module doing the work.
- Keep heuristic suggestions conservative and easy to disable.
- Treat token-level and text-level correctness as separate concerns.
- Preserve stable outward-facing diagnostic codes.
- Use corpus snapshots to verify that large text-pipeline refactors are
  behaviorally neutral unless a reviewed diff says otherwise.
