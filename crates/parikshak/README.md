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
- `varnavinyas-vyakaran` for lightweight verb evidence; its full MVP analyzer
  and `varnavinyas-samasa` remain optional in grammar heuristic paths

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

Word and text checking consult whole-word rules before speculative outer-suffix
detachment. For an unknown surface, the early correction must have a safe
lexical target; an unsupported suffix rewrite must not override a supported
case-marker/particle stack. This preserves `मीलेको -> मिलेको` without changing
valid `रामकोपनि` or `संसदमा`. Text diagnostics cover the original full token.

### Contextual final-i converbs

The always-on context pass recognizes selected constructions under
`PS-Saisanik-ह्रस्वदीर्घ-(भ)`; it does not perform general grammatical parsing.
For example, `काम पूरा पारि घर फर्कियो` suggests `पारी`, while
`राम खोला पारि बस्छ` and isolated `check_word("पारि")` remain unchanged.

Frames live in `data/rule_inventories/context_converbs.tsv`, with mandatory
source and review status. Both neighboring phrases must match within the same
sentence using horizontal whitespace only. Punctuation, quotations, line
breaks, and dictionary sense numbers cannot bridge a frame. A supported verb
infinitive and safe correction target provide independent lexical checks.
This behavior is shared by both orthography modes and does not require the
optional `grammar-pass` feature or runtime grammar flag.

Selected `मिलाइ`, `पकाइ`, `लगाइ`, `बनाइ`, `भइ`, and `लेखि` constructions
instead emit `Ambiguous` suggestions with heuristic evidence. For example,
`चिठी लेखि पठायो` offers `लेखी` if the author means “having written”. The
explanation explicitly preserves the competing noun/adverb meaning. Clients
must not treat these as definite errors or include them in automatic bulk
correction. `यसको मिलाइ राम्रो छ` and `तिम्रा लेखि ...` receive no such hint.
Unlisted contexts are left alone; this inventory is intentionally bounded,
not a complete parser or a general gender/agreement checker.

The following predicate may also be identified by a reviewed infinitive
family through `vyakaran::verb_evidence`: `चिठी लेखि पठाइन्`,
`चिठी लेखि पठाइयो` and `चिठी लेखि पठाउँछ` retain the same conditional
`लेखी` suggestion as `चिठी लेखि पठायो`. A suffix by itself is insufficient:
the infinitive must independently be a safe, dictionary-supported verb.
Irregular and unreviewed families are left to the explicit frames.

Reviewed writing-object nouns `चिठी` and `पत्र` may carry plural `हरू`,
object marking `लाई`, or both: `पत्रहरूलाई लेखि पठाइन्` offers the same
review suggestion. This uses supported suffix analysis plus explicit noun
provenance, including dictionary entries whose POS metadata is missing.
It does not reinterpret genitives (`पत्रको`, `तिम्रा`), locatives or arbitrary
suffix stacks as objects. Diagnostic spans still cover only `लेखि`.

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
  Joining now generalizes beyond the literal `बुद्धि सरह` example to dictionary-backed
  noun hosts such as `मानिस सरह`. School-backed `सम्म` joins authoritative bare
  hosts such as `घर सम्म`. Case-bearing hosts and unsupported hosts are guarded;
  this does not join arbitrary noun phrases or verb constructions.
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

## Compound information

Compound hints require reviewed formation evidence and preserve outer suffixes.
`samasa-heuristic` is informational; never apply its structural `correction`
string to text or present its ranking weight as an accuracy percentage. Public
signatures and diagnostic fields are unchanged. See the
[compound-analysis contract](../../docs/COMPOUND_ANALYSIS.md) for behavior and
artifact integration requirements.

Regular progressive verb forms with dictionary-backed infinitives are recognized
before speculative case detachment: `खोजिरहेको` remains unchanged instead of
becoming `खोजिरहनेको`. Rule-backed corrections retain precedence. See
[progressive constructions](../../docs/INTEGRATION_NOTES.md#progressive-verb-constructions).
