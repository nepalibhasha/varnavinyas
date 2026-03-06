# varnavinyas-prakriya

Core orthographic correction engine with rule tracing.

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
- `is_in_correction_table(&str)` -> quick check for known incorrect forms

## Examples

### Derive the standard form

```rust
use varnavinyas_prakriya::derive;

let result = derive("राजनैतिक");
assert_eq!(result.output, "राजनीतिक");
assert!(!result.steps.is_empty());
```

### Check whether a known incorrect form is table-backed

```rust
use varnavinyas_prakriya::is_in_correction_table;

assert!(is_in_correction_table("राजनैतिक"));
```

## Internal Structure

The rule engine is layered:

- correction table lookup first
- pattern rules second
- “already correct” fallback last

Pattern rules are registered via a niyama-oriented registry:

- `niyama_registry::section3_rules()` for Section 3 (`३. नेपाली वर्णविन्यास`)
- `niyama_registry::non_section3_rules()` for non-Section-3 rules (e.g., Section 4 structural)

Within Section 3, implementation is organized in descriptive modules:

- `hrasva_dirgha` -> `(क) ह्रस्वदीर्घ वर्ण र मात्रा ...`
- `chandrabindu_shirbindu` (+ `structural::rule_panchham_varna`) -> `(ख) चन्द्रविन्दु/शिरविन्दु/पञ्चम`
- `ustai_ucharan_varnaharu` -> `(ग) श/ष/स, ऋ/रि, ब/व, य/ए, क्ष/छ्य, ज्ञ/ग्या`
- `halanta_ra_ajanta` -> `(ङ) हलन्त र अजन्त`
- `aadhi_vriddhi` -> `(क)` sub-rule path for आदिवृद्धि cases

`orthographic` is currently kept as a compatibility facade that re-exports rule specs/functions from the newer descriptive modules.

## Crate Boundary (Important)

`prakriya` is intentionally word-centric: `derive(&str)` takes one token and returns one token-level correction path.

Rules that require multi-word context (especially most of Section 3 `(घ) पदयोग र पदवियोगसम्बन्धी नियम`) are implemented in `varnavinyas-parikshak` text-level passes, not inside `prakriya`.

Practical implication:

- keep token-level deterministic transforms in `prakriya`
- keep phrase/sentence boundary decisions in `parikshak`
- do not force context-sensitive spacing logic into `derive(&str)` unless we redesign the API

## Used By

- `varnavinyas-parikshak`
- `varnavinyas-bindings-wasm`
- other consumer crates that need word-level correction or explanation

## Current Limits

- Pattern rule arbitration is first-match by priority, not a full multi-candidate resolution engine.
- The correction table is authoritative but still finite; coverage grows as curated source material grows.

## Status

Core rule engine under active expansion.
