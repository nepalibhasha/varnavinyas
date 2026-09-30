# parikshak Diagnostic Arbitration

This document defines the conflict-resolution contract for
`varnavinyas-parikshak`.

Scope:

- Arbitration is a `parikshak` text-pipeline concern.
- `prakriya::derive()` remains a token-level single-winner API.
- The resolver chooses among text-span candidates emitted by `parikshak`
  passes; it does not change rule derivation inside `prakriya`.

## Candidate Contract

Each pass emits diagnostics with:

- `span`: byte span in the original text.
- `incorrect` / `correction`: replacement surface.
- `rule`, `category`, `kind`, `confidence`: outward diagnostic metadata.
- `evidence`: explicit table/exact rule, inventory-backed rule, generalized
  structural rule, or heuristic suggestion.

The private candidate wrapper derives the source pass from stable rule metadata
and uses `DiagnosticEvidence` directly as specificity.

Evidence is assigned by the producing pass, never parsed from human-readable
explanations. Word-level emission retains correction-table provenance from the
`RuleHit`, even when its outward citation names a broad orthography rule.
Generalized particle suffix splits rank below reviewed inventories and exact
whole-word corrections:
`सम्धिनि` becomes `सम्धिनी`, rather than `सम्धि नि`.

Rust callers constructing `Diagnostic` must supply `evidence`. Binding and JSON
DTOs intentionally omit this arbitration field; their payloads are unchanged.

## Precedence

The resolver encodes this precedence for padayog overlaps and duplicate
replacements; pass-local `blocked_spans` guards still apply:

1. Non-overlapping candidates all survive.
2. Higher diagnostic kind wins for overlapping spans:
   `Error` > `Variant` > `Ambiguous`.
3. For the same kind, higher specificity wins:
   exact table / cited explicit rule > curated inventory > generalized rule >
   heuristic.
4. Punctuation is independent unless it overlaps a non-punctuation diagnostic;
   in an overlap, non-punctuation wins.
5. For the same kind and specificity, source pass precedence is:
   `word` > `tiryak` > `padayog` > `context` > `style` > `grammar`.
6. For otherwise equivalent candidates, higher confidence wins.
7. If candidates have the same span and correction, keep one diagnostic and
   merge distinct alternate reasons instead of surfacing duplicates.
8. For strictly nested padayog overlaps, use the same precedence tuple unless
   applying the nested replacement at its relative source span produces the
   broader non-ambiguous padayog correction after whitespace is ignored. In
   that composite-rewrite case, keep the broader padayog diagnostic so
   replacement does not regress to a partial fix.

The important generalization from the `जगत` class of bugs is:

- word-level Academy corrections on the same token span beat generalized
  padayog/padabiyog splits.
- exact or cited padayog/padabiyog rewrites can beat generalized word-family
  rewrites on the same span.
- A generalized splitter should not need bespoke knowledge of every word-level
  rule family; the resolver should enforce that ordering.

The current implementation encodes this as `kind > specificity > pass >
confidence`. This order is deliberate: pass rank is a tie-breaker after the
rule's evidentiary strength, not a blanket "word always wins" rule.

## Regression Coverage

Tests pin:

- `जगत` yields `जगत्`, not `जग त`.
- same-span word-level errors suppress generalized padayog splits.
- explicit padayog rewrites suppress weaker same-span generalized rewrites.
- strictly nested padayog overlaps use the same precedence tuple as same-span
  overlaps, except when a broader padayog rewrite positionally subsumes the
  nested replacement.
- ambiguous/heuristic diagnostics do not block stronger errors.
- optional style/grammar variants do not displace hard errors.
- punctuation diagnostics remain visible next to word diagnostics when spans do
  not overlap.
- duplicate same-correction alternates collapse into one outward diagnostic.

## Remaining Pipeline Boundaries

The pipeline resolves word, tiryak, and padayog candidates before running
context, style, and optional grammar passes, then resolves again. Some later
passes still use `blocked_spans` to avoid emitting competing diagnostics. The
resolver does not yet arbitrate every possible cross-pass overlap. Extending
that scope requires dedicated tests and reviewed corpus snapshots.
