# varnavinyas-parikshak

Top-level checking pipeline for text diagnostics.

For a high-level description of the checker pipeline and crate boundaries, see
[`ARCHITECTURE.md`](./ARCHITECTURE.md).

## What This Crate Owns

This is the main orchestrator crate. It combines the lower-level engines into a practical text checker.

```text
text
  ↓
tokenization
  ↓
word-level checks
  ↓
text-level passes
  ↓
Diagnostics[]
```

It is responsible for:

- tokenizing text
- running word-level orthography checks
- attaching punctuation diagnostics
- optionally adding grammar/samasa-style heuristics
- returning unified diagnostics with spans, stable category codes, rule citations, confidence, and alternate reasons

If you want “check this text and tell me what to flag”, this is the crate to call.

## Main APIs

- `check_word(&str)` -> check a single word
- `check_text(&str)` -> default full-text diagnostics
- `check_text_with_options(&str, CheckOptions)` -> full-text diagnostics with runtime options
- `tokenize(&str)` / `tokenize_analyzed(&str)` -> tokenization helpers

## Examples

### Check a single word

```rust
use varnavinyas_parikshak::check_word;

let diag = check_word("राजनैतिक").unwrap();
assert_eq!(diag.correction, "राजनीतिक");
```

Typical single-word outcomes:

```text
राजनैतिक -> राजनीतिक   (hard orthography correction)
अध्यन    -> अध्ययन     (documented correction-table stopgap)
```

### Check full text

```rust
use varnavinyas_parikshak::check_text;

let diagnostics = check_text("यो बाक्यमा गल्ति छ.");
assert!(!diagnostics.is_empty());
```

Typical full-text behavior:

```text
यो बाक्यमा गल्ति छ.
   |        |       |
   |        |       punctuation diagnostic
   |        word-level diagnostic
   token-level correction candidate
```

## Depends On

- `varnavinyas-prakriya`
- `varnavinyas-lekhya`
- `varnavinyas-kosha`
- optionally `varnavinyas-vyakaran` and `varnavinyas-samasa` in heuristic paths

## Design Notes

```text
Rule truth          -> prakriya
Text orchestration  -> parikshak
Presentation        -> CLI / web / LSP / bindings
```

- `parikshak` is the integration layer, not the source of core orthographic truth.
- It should preserve distinctions between hard errors, variants, and heuristic suggestions.
- `DiagnosticReason` shares the same outward-facing `Explanation` shape used by `prakriya::WordAnalysis`.
- `category_code` is a stable contract used across CLI, web, LSP, and bindings.
- Text-level overlap arbitration is documented in [`ARBITRATION.md`](./ARBITRATION.md). Current precedence is `kind > specificity > pass > confidence`, with composite padayog rewrites allowed to subsume nested diagnostics only when the broader replacement already includes the nested correction.
- Shared tokenization is the expected path for text-level passes; new passes should consume `AnalyzedToken`s instead of doing independent whitespace splitting.

## Text-Level Rule Coverage

`parikshak` owns rules that need neighboring tokens, spacing, punctuation, or sentence context.

### Section 3 `(घ)` Padayog / Padabiyog

Owned by:

- `src/checker/padayog.rs`
- `src/checker/padayog_rules.rs`
- related checker passes such as `particles.rs` and `style_variants.rs`

Implemented highlights:

- explicit rewrite-table coverage for many `3(घ)` subrules in `padayog_rules.rs`
- generalized vibhakti joins
- honorific `ज्यू` joins
- conjunction joins
- `लागि/निम्ति` and double-vibhakti splits
- selected verb-complex splits
- `PS-Saisanik` comparison splits for `जस्तो/जस्तै/जत्रो/जसरी`
- `... स्वरूप` joins
- middle-name joins
- conservative one-meaning compound joins
- institutional/topic/title phrase splits
- meaningful reduplication splits
- nominal-verb splits
- `... गरी` splits
- nipat splits
- `जना` splits
- divisive-`न` splits
- multi-word samasa splits

Known gaps:

- `3(घ)-पदयोग-१,५,६,७,८,१०,११` need stronger morphology, compound ranking, sentence context, or curated semantic inventories before broad generalization.
- `3(घ)-पदवियोग-१,५,८,११,१२,१३` remain broad/context-sensitive and are not safe as standalone rewrites.
- `सरह` remains notice-only evidence in the local sources and is not part of the current `PS-Saisanik` comparison override.
- `थरी` remains notice-backed only in the current local source set.
- Most token-window passes now work across attached punctuation because they share tokenizer spans; exact literal phrase rewrites may still need punctuation-specific coverage.
- Broader `PS-Saisanik` inventories need wider regression coverage before broad expansion.

## Regression Gates

Use the corpus snapshot when changing tokenization, pass ordering, arbitration,
or broad checker fallbacks:

```bash
cargo test -p varnavinyas-parikshak --test corpus_snapshot -q
```

If a behavior change is intentional, inspect the snapshot diff line-by-line and
then refresh with:

```bash
VARNAVINYAS_UPDATE_CORPUS_SNAPSHOT=1 cargo test -p varnavinyas-parikshak --test corpus_snapshot -q
```

### `PS-Saisanik` Tiryak

Owned by `src/checker/tiryak.rs`.

Implemented highlights:

- `७(क)`: `-एको/-नु + ले/मा` oblique forms.
- `७(ख)`: oblique pronoun + case forms.
- `७(ग)`: pronoun/determiner oblique forms before an inflected nounish head.

Known gaps:

- broader pronoun inventory
- stronger noun-feature detection for `७(ग)`
- fuller UI/source-reference integration across all surfaces

## Current Limits

- Some higher-level diagnostics are still heuristic and intentionally conservative.
- Tokenization is practical but not yet a full linguistic parser.

## Status

Primary production-facing checker pipeline.
