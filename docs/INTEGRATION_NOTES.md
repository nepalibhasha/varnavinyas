# Integration Notes

## Current release set

Browser v0.1.7 and Python/CLI/iOS/Android v0.1.4 share one source commit.
The [release notes](releases/native-v0.1.4.md) describe corrected compound,
spacing, and progressive-verb behavior. Existing checker calls and diagnostic
fields remain available; progressive analysis is additive, and Python/WASM
affix-role consumers must handle the new postposition and comparison roles.
Upgrade generated mobile bindings with their matching native libraries.

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
- Treat `Ambiguous` as a conditional reading requiring individual review. Keep
  it out of bulk correction and retain its explanation; a nonempty `correction`
  does not establish an error.

Starting with browser v0.1.6 and native v0.1.3, bounded converb contexts can emit
`Ambiguous` with grammar heuristics disabled. For example,
`पत्रहरूलाई लेखि पठाइन्` offers `लेखी` if the intended meaning is “having
written”; the independent short noun/adverb reading remains valid. These
suggestions behave the same in both orthography modes. Known short forms are
preserved by isolated word checks, and possession, punctuation, quotes or line
boundaries do not establish the required context. This is bounded grammatical
evidence, not general sentence disambiguation.

When upgrading, use one source commit across browser, Python, CLI and mobile
artifacts. Compare native manifests' `source_commit` with the browser manifest's
`git_sha`; compare the diagnostic and origin fixture hashes across native
packages. Rebuild consumer assets and regenerate any precomputed sample
diagnostics. Install generated Swift/Kotlin bindings with their matching native
libraries, preserve orthography-mode selection, and test UTF-8 span conversion
with emoji and repeated words. API signatures, diagnostic schema and default
orthography policy are unchanged in this release set.

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

## Progressive verb constructions

The checker accepts regular progressive participles supported by dictionary verb
lemmas before guessing case suffixes. `खोजिरहेको`, `खोजिरहेकी`, and `खोजिरहेका`
must not become `खोजिरहनेको/की/का`. The same behavior applies with grammar on or
off, in both orthography modes, and through every checker binding. Outer
inflections such as `खोजिरहेकाले` retain the progressive base and a separate `ले`.
Rule-backed spelling corrections still take precedence.

The additive `shabda::analyze_progressive()` API returns a supported reading or
`None`, with `surface`, `main_form`, `main_lemma`, `auxiliary_form`,
`auxiliary_lemma`, and `negative`. For `खोजिरहेको`, the surface parts are
`खोजि + रहेको`; their lemmas are `खोज्नु` and `रहनु`. `खोजि` is a linking form,
not a dictionary lemma; `को` is internal to the participle here, not a genitive
case marker. The inspector presents this as verb construction information.

Python exposes `shabda.analyze_progressive()` returning a read-only
`ProgressiveAnalysis` or `None`. Generated Swift/Kotlin bindings expose
`analyzeProgressive()` with an optional record. WASM offers JSON
`analyze_progressive()` and typed `analyze_progressive_value()`;
the browser manifest advertises `progressive_verb_analysis`. C offers
`varnavinyas_analyze_progressive()` returning owned JSON (free it with
`varnavinyas_free_string`), JSON `null` for unsupported forms, and a null pointer
for invalid input. Existing signatures and diagnostic fields are unchanged.

This is a bounded analysis: it checks the short linking vowel
`ि/इ` and independently attested verb infinitives, supports an optional negative
`न`. Reviewed irregular linking forms `गइ -> जानु` and `भइ -> हुनु` are kept in
`data/rule_inventories/verb_linking_forms.tsv` with source evidence and parser
checks. It does not infer sentence agreement, tense, other irregular stems, or
arbitrary auxiliary chains. Analyze the stem from the outer affix API before requesting
progressive analysis of a suffixed word. No analysis is not proof of a spelling
error. These changes require rebuilt browser/native artifacts to reach consumers;
generated Swift/Kotlin code must match its new native library.

## Relational suffix roles and joining

The affix analysis distinguishes reviewed postpositions (`सम्म`, `सँग`, `सँगै`,
`तिर`, `भित्र`) from case markers such as `को` and `ले`. Comparison use of `सरह`
has its own role: `मानिससरह` has host `मानिस` and comparison marker `सरह`;
`घरसम्म` has host `घर` and postposition `सम्म`. These are surface analyses,
not claims that every joined word is a compound. The reviewed inventory is
`data/rule_inventories/relational_suffixes.tsv`.

Rust and Python append `AffixKind::Postposition` and `AffixKind::ComparisonMarker`
without changing existing members. WASM affix segments can now carry
`kind: "postposition"` or `kind: "comparison_marker"`; previously reviewed
postpositions were reported as `case_marker`. Consumers that exhaustively match
affix roles need to handle both new values. UniFFI and C do not expose this affix
enum; their checker diagnostic schema remains unchanged.
The browser manifest lists the supported roles in `capabilities.affix_segment_kinds`.

The checker generalizes `मानिस सरह -> मानिससरह` using noun headword evidence
and `घर सम्म -> घरसम्म` using authoritative host evidence. It avoids joining
case-bearing hosts such as `मानिसको सरह` and `घरको सम्म`, or unsupported hosts.
The Notice's 3(घ)-पदयोग-११ supports `सरह`; school grammar 5(अ)(ख) supports
`सम्म`. The school grammar's separate `जस्तो/जस्तै/जत्रो/जसरी` family is
preserved. Both orthography modes and grammar settings use these joining rules.

## Spacing prescriptions versus word structure

The school institutional/topic-spacing rule requires a reviewed complete pair
from 5(आ)(ख). Merely decomposing a word into two known nouns is not a correction.
`वायुसेवा`, `वायुसेवाको`, `जनसेवा`, and `राज्यव्यवस्था` no longer produce
blanket splits. Explicit prescriptions such as `नेपालसरकार -> नेपाल सरकार`
and `समाजसेवा -> समाज सेवा` remain spelling/spacing errors in the main checker,
independent of the optional grammar toggle. The source pairs live in
`data/rule_inventories/institutional_spacing.tsv`; their outer case suffixes stay
on the final member. Diagnostic fields, kinds, and stable category codes are
unchanged. Rebuild checker artifacts to deliver this fix to downstream clients.
