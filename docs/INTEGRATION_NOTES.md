# Integration Notes

## Word Analysis and Explanations

Use `analyze_word_value` (or its JSON-string equivalent `analyze_word`) when
presenting origin and explanatory notes in a browser client:

| `origin_source` | Meaning | Presentation |
| --- | --- | --- |
| `kosha`, `override` | Dictionary metadata or reviewed evidence | Documented origin |
| `heuristic` | Inference from spelling patterns | Label as inferred |
| `unknown` | No origin evidence; `origin_confidence` is zero | Show unknown origin |

For the last case, word analysis also returns `origin: "unknown"`. These are
additional string values in existing fields; clients must handle them without
falling back to a देशज label. Lexicon membership alone does not establish origin.
Browser artifact manifests advertise this through
`capabilities.word_analysis_origin_provenance` and
`capabilities.word_analysis_origin_sources`. Treat an unrecognized origin or
source as unknown rather than claiming documented evidence.

The lower-level Rust `classify()` and the Python, C, and UniFFI `classify`
wrappers still return a best-effort four-way category for compatibility.
`decompose_word_value().origin` and existing Python decomposition origin fields
have the same limitation. Their fallback `Deshaj` is not documented etymology.

Starting with native v0.1.2, Python `shabda.classify_with_provenance()` and
Swift/Kotlin `classifyWithProvenance` expose an `OriginDecision`: nullable
`origin`, `source` (`Override`, `Kosha`, `Heuristic`, `Unknown`), and
`confidence`. Unknown is `None`/`nil`/`null`, with source Unknown and confidence
zero. This is an additive API; the existing `Origin` enum has no new values.
Rust callers use `classify_with_provenance()` and its `supported_origin()`
method to omit the legacy fallback. Evidence scores are not calibrated
probabilities. The C wrapper still lacks a provenance function.

CLI `classify WORD --format json` returns the same three fields using lowercase
category/source codes and JSON `null` for unknown origin. It does not change the
existing `check` command. New native APIs use a nullable origin; browser word
analysis retains its existing string `"unknown"` convention.

Morphology is conservative: an omitted split means there is insufficient
supported analysis, not that a word cannot have a derivation. For example,
`संघीय` no longer supplies `सम् + घीय` through circular sibling evidence, and
`हामी` does not borrow the unrelated noun `हाम` to justify an ई suffix.
The explicitly documented `तिनी = तिन + ई` analysis remains available.

`derive()` preserves ambiguous root/imperative pairs such as `हेर` / `हेर्`.
Word analysis explains both source rules without proposing a context-free
replacement. An unchanged derivation is not a claim that every grammatical
use of that spelling is correct.

Use `rule_code` for reference links and `alternate_reasons` / alternate rule
notes for independently supported additional explanations. Their count can
decrease when unsupported or redundant reasons are removed. In the web editor,
the current text diagnostic determines the correction and severity; isolated
word analysis supplements it with compatible linguistic information.

## Orthography Mode

The checker supports two orthography policies:

- `academy-strict`: compatibility default. Academy-prescriptive forms are
  emitted as `kind: "Error"`.
- `common-editorial`: reviewed common-vs-strict forms are emitted as
  `kind: "Variant"` with the strict form still present in `correction`.

The JSON diagnostic shape is unchanged. Consumers should key behavior from the
existing `kind` field:

- Treat `Error` as blocking.
- Treat `Variant` as non-blocking unless the user explicitly wants suggestions
  to fail a check.

The common-editorial boundary is curated, not frequency-based. Adding a future
variant requires:

1. Evidence of strict-source pressure from the Academy references or lexicon.
2. Evidence that the common form is stable enough for editorial use.
3. A source note in `crates/parikshak/src/checker/orthography_variants.rs`.
4. Tests proving strict mode remains blocking and common-editorial mode emits a
   variant.

Current reviewed common-editorial variants:

| Common form | Strict form | Category |
| --- | --- | --- |
| `संघीय` | `सङ्घीय` | `Chandrabindu` |
| `संघ` | `सङ्घ` | `Chandrabindu` |
| `संचार` | `सञ्चार` | `Chandrabindu` |
| `संकेत` | `सङ्केत` | `Chandrabindu` |
| `संसद` | `संसद्` | `Halanta` |
| `कांग्रेस` | `काङ्ग्रेस` | `Chandrabindu` |

Surface-specific option names:

| Surface | Option |
| --- | --- |
| CLI | `--orthography-mode academy-strict\|common-editorial` |
| Rust | `CheckOptions { orthography_mode, ... }` |
| LSP | `orthographyMode` or legacy `orthography_mode` |
| WASM | `check_text_value_with_options(text, grammar, orthography_mode)` |
| Python | `orthography_mode="academy_strict"` or `"common_editorial"` |
| C | `varnavinyas_check_text_with_all_options(..., orthography_mode, ...)` |
| UniFFI | `check_text_with_all_options(..., OrthographyMode, ...)` |

For browser artifacts, `check_text_value(text, grammar)` remains the
backward-compatible default API and uses `academy-strict`. Downstream clients
that need explicit policy selection should use
`check_text_value_with_options(text, grammar, orthography_mode)` when
`manifest.json` advertises the capability.
