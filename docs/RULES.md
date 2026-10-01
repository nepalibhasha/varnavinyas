# Linguistic Rules

Varnavinyas encodes Nepali orthography rules as auditable Rust code. This document maps the local Academy references to the implementation layers.

## Sources

The rule layer currently uses two local Academy references:

1. Nepal Academy orthography notice excerpt:
   - source PDF: <https://mofaga.gov.np/notice-file/Notices-20211029142422901.pdf>
   - local excerpt: `docs/Notices-pages-77-99.md`
2. School-grammar reference for additional and sometimes conflicting spacing/morphosyntactic guidance:
   - local excerpt: `docs/PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md`

When these two sources conflict, follow `docs/RULE_SOURCE_POLICY.md`. The current policy prefers `PS-Saisanik...` for the known conflict set.

## Implementation Model

```text
Single-token orthography       -> prakriya
Multi-token spacing/context    -> parikshak
Punctuation                    -> lekhya
Lexical plausibility/metadata  -> kosha + shabda
Surface-specific presentation  -> CLI / LSP / Web / Bindings
```

Rules are plain Rust functions plus small schema-checked TSV inventories that
are compiled into the owning rule modules. There is no external JSON/YAML rule
engine. Shared metadata and browser rule labels live outside the rule engine,
but production decisions remain in code and reviewed inventories.

## Rule Categories

| Source Area | Description | Current Implementation | Status |
|---|---|---|---|
| Section 3 `(क)` | ह्रस्व/दीर्घ vowel length | `crates/prakriya/src/varna_vinyasa/hrasva_dirgha.rs` and `hrasva_dirgha/{a,aa,i,u,uu}.rs` | Partial / expanded |
| Section 3 `(ख)` | चन्द्रविन्दु, शिरविन्दु, पञ्चम वर्ण | `crates/prakriya/src/varna_vinyasa/chandrabindu_shirbindu.rs`, `chandrabindu_shirbindu/*`, `panchham.rs` | Partial / expanded |
| Section 3 `(ग)` | श/ष/स, ऋ/रि, ब/व, य/ए, क्ष/छ्य, ज्ञ/gya families | `crates/prakriya/src/varna_vinyasa/ustai_ucharan_varnaharu.rs` and submodules | Partial |
| Section 3 `(घ)` | पदयोग/पदवियोग | `crates/parikshak/src/checker/padayog.rs`, `padayog_rules.rs`, and phrase-specific checker passes | Partial / active |
| Section 3 `(ङ)` | हलन्त/अजन्त | `crates/prakriya/src/varna_vinyasa/halanta_ra_ajanta.rs` and submodules | Partial / expanded |
| Section 3 `(च)` | Glyph forms, conjunct writing, डिको and three-tier script layout | Textual conjunct inspection in `akshar`; handwriting/font layout belongs to rendering and teaching | Outside automatic spelling correction |
| Section 4 | शुद्ध-अशुद्ध table | `crates/prakriya/src/correction_table.rs` plus rule-backed exceptions | Active, not a pure table mirror |
| Section 5 | punctuation and formatting | `crates/lekhya/src/punctuation.rs`, integrated through `parikshak` | Stable |
| `PS-Saisanik` section 7 | तिर्यक् रूपको प्रयोग | `crates/parikshak/src/checker/tiryak.rs` | Partial / active |

## Current Coverage Notes

- Section 3 `(क)` has broad coverage for many initial, medial, and final hrasva-dirgha classes. Remaining gaps are mostly verb-sensitive classes and semantic classes that need stronger morphology/lexicon signals.
- Section 3 `(घ)` and `PS-Saisanik` spacing rules live in `parikshak` because they need neighboring tokens, spacing, punctuation, or phrase context.
- `parikshak` arbitrates overlapping text diagnostics explicitly; see `crates/parikshak/ARBITRATION.md` for the current `kind > specificity > pass > confidence` contract.
- Section 3 `(ङ)` includes inventory-backed ajanta coverage for the Notice example lists and the `PS-Saisanik` loanword-ajanta examples such as `कोट् -> कोट`.
- Ambiguous bare root/imperative pairs such as `हेर` / `हेर्` are preserved and explained through word-analysis notes until grammatical context can resolve the choice.
- The Section 3 `(क)(ऊ)-1` ई-suffix explanation requires reviewed derivational evidence in `data/rule_inventories/ii_suffix_derivatives.tsv`. Mechanical suffix stripping alone does not establish the rule's applicability; pronouns retain their specific vowel-length rule.
- Section 4 is not simply a lexicon lookup. `correction_table.rs` currently contains 81 entries: 38 Section 4-style entries, 42 rule-backed holdouts, and 1 documented stopgap. See `crates/prakriya/README.md`.
- `तिर्यक्`, comparison spacing, institutional/title splits, and similar school-grammar phrase behavior should be first-class checker rules, not correction-table growth.

## Numbered Coverage And Dependencies

Reviewed 2026-09-30 against both local source extracts and current code. A
registered function or passing example does not establish complete coverage
of a grammatical class. "Guarded" below means implemented for supported
lexical/morphological families, with generalization still bounded by evidence.

| Source subrules | Current scope | Remaining dependency |
|---|---|---|
| Notice `3(क)(अ)-1..7,10,11` | Guarded initial vowel families | Broader reviewed examples and counterexamples |
| Notice `3(क)(अ)-8,9` | General verb/dhatu classification deferred | Reliable verb roots and inflection evidence |
| Notice `3(क)(अ)-12` | Explicit corrections plus guarded tadbhav fallback; semantic pairs preserved | Context for meaning-dependent pairs such as फूल/फुल |
| Notice `3(क)(अ)-13` | Supported नु/एली derivative families | Additional validated derivations |
| Notice `3(क)(आ)-1,3..6,9,10` | Guarded medial vowel families | Broader reviewed class membership |
| Notice `3(क)(आ)-2` | Partial suffix-family coverage | Derivational evidence and suffix exceptions |
| Notice `3(क)(आ)-7,8` | General verb/passive-voice rules deferred | Verb and voice analysis |
| Notice `3(क)(इ)-1..5,7,9` | Guarded final-hrasva families | Reviewed exceptions and lexical classes |
| Notice `3(क)(इ)-6` | Partial case-marker coverage | Stronger attachment/context validation |
| Notice `3(क)(इ)-8` | General नु/छु verb class deferred | Verbal context, including imperative contrasts |
| Notice `3(क)(ई)-1,2`, `(उ)-1,2` | Guarded tatsam/prefix/suffix preservation | Origin and derivation evidence |
| Notice `3(क)(ऊ)-1..3,5,7..9,11..16` | Guarded final-dirgha families; ई derivations use a reviewed inventory | Wider lexical/derivational coverage |
| Notice `3(क)(ऊ)-4,6`, PS hrasva/dirgha `(थ)` | 29 unique reviewed feminine-adjective and inanimate-noun examples; specific corrections and accepted-form explanations | Expansion requires reviewed semantic evidence; endings alone do not prove gender or animacy |
| Notice `3(क)(ऊ)-10` | Some attested examples; class generalization deferred | Verb and grammatical-context evidence |
| Notice `3(घ)` पदयोग `1,5..8,10,11` | Explicit examples exist; generalized forms remain TODOs | Prefixes, compound ranking, reduplication, coordination, verb complexes and semantic inventories |
| Notice `3(घ)` पदवियोग `1,5,8,11..13` | Explicit examples exist; broad splitting remains deferred | Syntax, verb complexes, classifier/name inventories; baseline “each word separate” is not an independent rewrite |
| PS abbreviation hrasva/dirgha `(छ)` | Compact dotted initials protected in spelling and punctuation, in both modes | Normalizing joined/dotted abbreviation vowels and broader spaced-chain recognition remain open |
| PS tatsam final-उ `(छ)` | All 15 printed examples are reviewed exact targets, with correction and accepted-form explanations | Expansion requires reviewed source evidence; no blanket ऊ-shortening |
| Notice `3(ङ)`, PS halanta/ajanta | Reviewed examples and loan families; ambiguous bare verbs retained | Context to distinguish bare roots/imperatives and productive verb forms |
| Notice Section 4 phrase/sentence examples | Selected phrase corrections and optional suggestions | Syntax and meaning; word-table coverage does not imply sentence grammar coverage |
| Notice Section 5 | Mechanical scanner checks and abbreviation guards | Syntactic punctuation placement is not modeled |

The vowel rows account for the source's numbered classes; the other rows record
the principal remaining boundaries, not an exhaustive certification of every
printed example. Keep source wording, implementation guards, positive examples,
and nearest valid counterexamples together when closing a row.

Immediate engineering work is not blocked by the missing grammatical model:
source inventories, regression fixtures and consumer tests can grow safely.
Three low-confidence source readings still require a better scan: Notice
पदयोग-6 झैझगडा/झैँझगडा, the PS तद्भव मिठो parenthetical मिष्ट/मिष्ठ, and PS
`४(ख)` ह्वार्त. Only those readings are blocked. Joined-future common-editorial
treatment also needs a product policy decision; the current checker continues
to require separate forms in both modes.

## Evaluation Evidence

The curated checker gold set contains 97 word entries and 13 paragraph entries.
Every word now must receive an expected replacement and a cited explanation;
both correct-form false-positive tests and corpus snapshots remain separate
gates. This is fixture coverage, not a percentage of all Academy rules.

Sandhi's conservative-stem eval pipeline now recovers all 10 expected pairs,
and each pair is required individually. It uses `shabda::best_analysis`, matching
the inspector, instead of letting legacy `decompose` strip the final ई in
विद्यार्थी. This is pipeline alignment, not a new linguistic splitting rule.
यण् forward/reverse encoding now preserves the
right member's vowel sign, recovering अत्याचार as अति + आचार with lexical
and exact forward-verification guards intact.
The inspector suppresses legacy morphology that disagrees with the supported
stem, while retaining supported outer affixes and compound explanations.
Samasa requires five reviewed winning pairs/types/vigraha and rejects fourteen
unsupported interpretations. महोत्सव uses महा + उत्सव after dictionary review.
The public analyzer uses the sourced `samasa.tsv` inventory; POS and spelling
round trips alone are insufficient. Morphology has 22 fixtures. The fourteen
grammar sentences include negative compound cases and an inflected-compound
case; case/agreement examples still do not require their intended detections.
The unlabeled headword census measures split activity, not a false-positive rate.
See `crates/eval/README.md` for limits and the remaining evaluation work.

## Example Rule Shape

```rust
// crates/prakriya/src/varna_vinyasa/hrasva_dirgha/a.rs

pub fn rule_suffix_nu_hrasva(input: &str) -> Option<Prakriya> {
    // Verbal suffix families such as -नु are handled with lexical and
    // derivational guards before returning a correction path.
}
```

## Diagnostics

When a rule is violated, user-facing surfaces receive a diagnostic with:

1. incorrect text and byte span
2. suggested correction
3. rule citation and stable `rule_code`
4. human-readable explanation
5. stable `category_code`
6. confidence and optional alternate reasons

Stable diagnostic categories are defined in `crates/parikshak/src/diagnostic.rs` and are consumed by the web UI, CLI JSON, LSP, and bindings.

## Alignment Strategy

- Use the two source markdowns for authority and citations.
- Use `docs/RULE_SOURCE_POLICY.md` when sources conflict.
- Keep `docs/tests/gold.toml` as the regression ground truth, not as independent linguistic authority.
- Prefer first-class rules over one-off table entries.
- Keep broad fallbacks below specific numbered rules and suppress duplicate alternate hits when a specific rule already explains the same correction.
- Prefer schema-checked inventories with provenance fields for growing cited example lists, especially when the alternative is scattered Rust constants.
- Treat raw lexicon attestation as plausibility evidence, not proof that a form is a safe correction target.
- Keep explanation evidence distinct from spelling plausibility: origin notes require documented provenance, and alternate rule notes require independent support. See `docs/INTEGRATION_NOTES.md` for the consumer-facing contract.
