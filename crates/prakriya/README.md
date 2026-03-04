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

Pattern rules are grouped into domains such as:

- structural rules
- hrasva/dirgha rules
- orthographic rules

## Used By

- `varnavinyas-parikshak`
- `varnavinyas-bindings-wasm`
- other consumer crates that need word-level correction or explanation

## Current Limits

- Pattern rule arbitration is first-match by priority, not a full multi-candidate resolution engine.
- The correction table is authoritative but still finite; coverage grows as curated source material grows.

## Status

Core rule engine under active expansion.
