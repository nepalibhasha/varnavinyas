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
