# varnavinyas-types

Shared semantic types for the Varnavinyas workspace.

## What This Crate Owns

This crate is the common vocabulary layer for concepts that multiple crates need to agree on.

Today it mainly provides:

- `Origin` -> the shared origin taxonomy used across lexical analysis

The long-term role of this crate is larger: it should hold shared enums and structs for concepts like provenance, authority tiers, and confidence bands so that all surfaces speak the same semantic language.

## Why It Exists

Without a shared types crate, core concepts get duplicated across crates and drift over time. This crate prevents that by centralizing domain-wide semantic definitions.

## Example

```rust
use varnavinyas_types::Origin;

assert_eq!(Origin::Tatsam.nepali_label(), "तत्सम");
assert_eq!(Origin::Deshaj.transliterated_label(), "deshaj");
```

## Used By

- `varnavinyas-shabda`
- bindings and other consumers that need a stable shared domain enum

## Current Limits

- The crate is intentionally small right now.
- More shared semantic types should move here as the workspace matures.

## Status

Small but important shared-domain crate.
