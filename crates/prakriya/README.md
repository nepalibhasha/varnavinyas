# varnavinyas-prakriya

Core orthographic correction engine with rule tracing.

For a high-level description of the crate’s design and module ownership, see
[`ARCHITECTURE.md`](./ARCHITECTURE.md).

## What This Crate Owns

This crate answers the question: “Given a word form, what is the correct standard form, and why?”

It is the rule engine behind word-level correction and is responsible for:

- looking up authoritative corrections from the static correction table
- applying pattern-based orthographic rules when no direct table entry exists
- returning a trace of which rule fired and how the output was derived

This is the closest thing the workspace has to a central normative orthography engine.

## Main APIs

- `derive(&str)` -> derive the standard form and a step trace
- `analyze(&str)` -> word analysis with rule notes
- `collect_rule_hits(&str)` -> collect all applicable rule hits before winner selection
- `is_in_correction_table(&str)` -> quick check for known incorrect forms

## Examples

### Derive the standard form

```rust
use varnavinyas_prakriya::derive;

let result = derive("राजनैतिक");
assert_eq!(result.output, "राजनीतिक");
assert!(!result.steps.is_empty());
```

Typical outcomes:

```text
राजनैतिक -> राजनीतिक   (correction-table path)
सूमार्ग   -> सुमार्ग     (pattern-rule path, 3(क)(अ)-1)
```

This distinction matters because table-backed forms are authoritative overrides,
while pattern-backed forms are derived from numbered Academy rules.

### Check whether a known incorrect form is table-backed

```rust
use varnavinyas_prakriya::is_in_correction_table;

assert!(is_in_correction_table("राजनैतिक"));
```

## Internal Structure

The rule engine is layered:

```text
input token
  ↓
correction table
  ↓
pattern rules
  ↓
winner selection
  ↓
Prakriya + Explanation
```

- correction table lookup first
- pattern rules second
- “already correct” fallback last

Pattern rules are registered via a domain-oriented registry:

- `niyama_registry::varna_vinyasa_rules()` for Academy orthography families
- `niyama_registry::usage_fix_rules()` for later cleanup-style rules

Runtime dispatch is assembled in `runtime.rs`, which merges those registry
groups, sorts them by priority, and caches the result for repeated correction.

Within `src/`, implementation is organized by domain:

- `varna_vinyasa/` -> Academy orthography families
  - `hrasva_dirgha` -> `(क) ह्रस्वदीर्घ वर्ण र मात्रा ...`
  - `chandrabindu_shirbindu` + `panchham` -> `(ख) चन्द्रविन्दु/शिरविन्दु/पञ्चम`
  - `ustai_ucharan_varnaharu` -> `(ग) श/ष/स, ऋ/रि, ब/व, य/ए, क्ष/छ्य, ज्ञ/ग्या`
  - `halanta_ra_ajanta` -> `(ङ) हलन्त र अजन्त`
  - `aadhi_vriddhi` -> `(क)` sub-rule path for आदिवृद्धि cases
- `usage_fixes/` -> later cleanup-style rules such as Section 4 shuddha/ashuddha fixes
- `model/` -> core derivation types such as `Prakriya`, `Rule`, `RuleSpec`, and `Step`
- `explanation.rs` -> shared outward-facing `Explanation` model used by analysis and diagnostics
- `runtime.rs` -> pattern-rule dispatch assembly and caching

## Crate Boundary (Important)

`prakriya` is intentionally word-centric: `derive(&str)` takes one token and returns one token-level correction path.

Rules that require multi-word context (especially most of Section 3 `(घ) पदयोग र पदवियोगसम्बन्धी नियम`) are implemented in `varnavinyas-parikshak` text-level passes, not inside `prakriya`.

Practical implication:

- keep token-level deterministic transforms in `prakriya`
- keep phrase/sentence boundary decisions in `parikshak`
- do not force context-sensitive spacing logic into `derive(&str)` unless we redesign the API

Example:

```text
prakriya handles:   सूमार्ग -> सुमार्ग
parikshak handles:  text-level join/split and punctuation diagnostics
```

## Rule Coverage Snapshot

`prakriya` owns token-level Academy Section 3 rule families. Current coverage is intentionally conservative where lexical, semantic, or verb-context signals are still weak.

### Section 3 `(क)` Hrasva/Dirgha

Owned by `src/varna_vinyasa/hrasva_dirgha.rs` and `src/varna_vinyasa/hrasva_dirgha/{a,aa,i,u,uu}.rs`.

Implemented highlights:

- `3(क)(अ)-1..7,10..13`: initial hrasva families for prefixes, `द्वि/त्रि`, names, aagantuk forms, pronouns, adjectives, numbers, avyaya, onomatopoeic words, tadbhav/deshaj/aagantuk fallback, and tatsam + Nepali suffixes.
- `3(क)(आ)-1..6,9,10`: medial hrasva families.
- `3(क)(ई)-1..2`: initial dirgha preservation for Sanskrit/tatsam and `सु`-upasarga families.
- `3(क)(उ)-1..2`: medial dirgha preservation for suffix and suffix-family patterns.
- `3(क)(इ)-1..7,9`: final hrasva families.
- `3(क)(ऊ)-1,2,3,5,7,8,9,11,12,13,14,15,16`: final dirgha families implemented directly or through shared final-class helpers.

Known gaps:

- `(क)(अ)-8/-9`, `(क)(आ)-7/-8`, and `(क)(इ)-8` need stronger verb-context support.
- `(क)(ऊ)-4/-6/-10` need stronger POS or semantic metadata.
- Some numbered classes still rely on conservative attested-family logic rather than full derivational analysis.

### Section 3 `(ख)` Chandrabindu / Shirbindu / Panchham

Owned by `src/varna_vinyasa/chandrabindu_shirbindu.rs`, `src/varna_vinyasa/chandrabindu_shirbindu/*`, and `src/varna_vinyasa/panchham.rs`.

Implemented highlights:

- `3(ख)(आ)-1`: tatsam `ँ` -> `ं` normalization.
- `3(ख)(आ)-2..4`: chandrabindu handling for first-person/nasal verb forms, `...दा/दै`, and `...छ/थ` patterns.
- `3(ख)(अ)-2`: panchham substitution before class consonants.
- `3(ख)(अ)-3`: guarded non-tatsam over-Sanskritized `ञ्/ण्` conjunct normalization.

Known gaps:

- Broader over-Sanskritized non-tatsam variants still need safer inference.
- More notice-example parity should be added through fixtures.

### Section 3 `(ग)` Similar-Sounding Letters

Owned by `src/varna_vinyasa/ustai_ucharan_varnaharu.rs` and its submodules.

Implemented rule families:

- `rule_sibilant` for श/ष/स.
- `rule_ri_kri` for ऋ/रि and कृ/क्रि.
- `rule_ba_va` for ब/व.
- `rule_ya_e` for य/ए.
- `rule_ksha_chhya` for क्ष/छ्य.
- `rule_gya_gyan` for ज्ञ/gya variants.

Known gaps:

- Full subsection granularity and exception handling.
- Better acceptance of valid dhatu + प्रत्यय/रूप paradigms before fallback suggestions.
- Broader derivational family handling with stronger false-positive guards.

### Section 3 `(ङ)` Halanta / Ajanta

Owned by `src/varna_vinyasa/halanta_ra_ajanta.rs` and its submodules.

Implemented highlights:

- Halanta numbered subrules: `3(ङ)-1,2,3,4`.
- Ajanta numbered subrules: `3(ङ)-अजन्त-1..8`.
- Dedicated halanta and ajanta functions orchestrated by `rule_halanta`.

Known gaps:

- Sentence-level intent/context disambiguation is only partially modeled.
- Productive verb paradigms need more lexical validation.

### Section 3 `(च)` Lipi-Specific Notes

No complete dedicated subsection module exists yet. Related behavior is scattered across table/rule handling and should not be expanded without a clearer source-backed scope.

## Correction Table Audit

`src/correction_table.rs` is not meant to become a second rule engine.

Current inventory:

- total correction-table entries: 81
- `Rule::ShuddhaAshuddha(...)` entries: 38
- `Rule::VarnaVinyasNiyam(...)` entries: 42
- `Rule::Vyakaran(...)` stopgaps: 1

Current policy:

- keep explicit Section 4 shuddha/ashuddha entries in the table
- migrate rule-backed Section 3 entries out when rule-path parity and winner selection are strong enough
- document temporary stopgaps with a removal or replacement path

Known rule-backed Section 4 fixtures that now flow through rule paths instead of direct table entries:

- `संसद`
- `परिषद`
- `फाउण्डेसन`
- `झण्डा`
- `इण्डिया`

Tracked stopgaps and holdouts:

- `अध्यन -> अध्ययन`: currently a `Rule::Vyakaran("kosha")` stopgap. It needs source confirmation or a genuine niyama-backed path.
- `बिद्वान -> विद्वान्`: still needs the correction table because current rule evaluation does not compose `3(ग)(आ)` ब/व with `3(ङ)` halanta into one winning output.
- `भएकोमा -> भएकामा`: currently table-backed because lower-level `prakriya` gold coverage expects a direct derivation path, while generalized `तिर्यक्` handling lives in `parikshak`.

## Multi-Step Derivation Policy

`derive()` deliberately does not run a fixpoint loop over rule outputs yet.
Composition cases stay table-backed until chained behavior can be arbitrated
without surprising callers.

Do not remove a correction-table entry just because each component rule exists.
Remove it only when:

- the composed output is produced by an explicit, tested derivation path
- winner stability tests show `derive()` still returns the intended top result
- duplicate-hit suppression keeps alternate reasons readable
- corpus snapshot diffs show no unexpected false-positive expansion

The current known composition holdout is `बिद्वान -> विद्वान्` (`ब/व` plus
halanta). It should remain finite and table-backed until multi-step composition
has a resolver underneath it.

## Used By

- `varnavinyas-parikshak`
- `varnavinyas-bindings-wasm`
- other consumer crates that need word-level correction or explanation

## Current Limits

- Pattern rule arbitration is first-match by priority, not a full multi-candidate resolution engine.
- `derive()` stays single-winner even though `collect_rule_hits()` can expose alternate applicable reasons.
- The correction table is authoritative but still finite; coverage grows as curated source material grows.

## Status

Core rule engine under active expansion.
