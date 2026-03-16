# Varnavinyas Feature Status

> **Last Updated**: 2026-03-10

This page is a coarse status snapshot, not a rule-by-rule checklist.

For Academy rule coverage details, use:
- `docs/RULES.md`
- crate-level architecture docs
- tests in `docs/tests/*.toml` and `crates/*/tests/*`

```text
Core utilities     -> stable
Lexicon / checker  -> stable to active
Rule coverage      -> active expansion
Some Academy areas -> still partial / TODO
```

## Core Stack

| Component | Scope | Status | Notes |
|---|---|---|---|
| `akshar` | Devanagari classification and normalization | Stable | Core utility layer |
| `lipi` | Transliteration and legacy font conversion | Stable | Used by CLI and bindings |
| `kosha` | Lexicon and headword metadata | Stable | Compile-time lexical assets |
| `shabda` | Origin classification and lightweight morphology | Stable | Used for rule selection and explanations |
| `prakriya` | Token-level orthography correction and rule tracing | Active | Academy-aligned rule coverage is still expanding |
| `lekhya` | Punctuation diagnostics | Stable | Section 5-oriented checks |
| `parikshak` | End-to-end text checking pipeline | Stable | Main production-facing checker |

## User-Facing Surfaces

| Surface | Status | Notes |
|---|---|---|
| Web app | Active | Checker, inspector, and rules reference are usable |
| Browser extension product | Migrated downstream | Shipped extension ownership is downstream; this repo keeps only the browser artifact contract and packaging path |
| CLI | Active | Suitable for local workflows and CI |
| WASM bindings | Active | Used by the web app and packaged for downstream browser clients |
| Python bindings | Active | Core module surface exists; packaging metadata and release workflow can improve |
| LSP | Active | Server crate and editor-facing integration exist; performance and UX can still improve |
| C / UniFFI bindings | Active | Available for integration scenarios |

## Rule Coverage Snapshot

| Area | Status | Notes |
|---|---|---|
| Section 3 `(क)` ह्रस्व/दीर्घ | Active | Significant coverage exists; some numbered subrules remain TODO where context is still too weak |
| Section 3 `(ख)` चन्द्रविन्दु/शिरविन्दु/पञ्चम | Active | Core coverage exists; some edge cases still rely on conservative guards |
| Section 3 `(ग)` उस्तै उच्चारण हुने वर्णहरू | Active | Major families implemented in dedicated modules |
| Section 3 `(घ)` पदयोग/पदवियोग | Partial | Mostly text-level in `parikshak`, not token-level in `prakriya` |
| Section 3 `(ङ)` हलन्त/अजन्त | Active | Core patterns implemented with conservative boundaries |
| Section 5 punctuation | Stable | Implemented in `lekhya` and integrated through `parikshak` |

## Current Priorities

- continue Academy rule coverage where it can be implemented systematically
- keep web, CLI, LSP, and bindings aligned with stable diagnostic contracts
- avoid replacing systematic rules with growing patch tables unless necessary

## Validation Sources

The main regression sources are:

- `docs/tests/gold.toml`
- `docs/tests/grammar_sentences.toml`
- `docs/tests/morph_gold.toml`
- `docs/tests/samasa_gold.toml`
- crate integration/unit tests
