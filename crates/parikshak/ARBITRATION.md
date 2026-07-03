# parikshak Diagnostic Arbitration

This document defines the target conflict-resolution contract for
`varnavinyas-parikshak`.

Scope:

- Arbitration is a `parikshak` text-pipeline concern.
- `prakriya::derive()` remains a token-level single-winner API.
- A future resolver should choose among text-span candidates emitted by
  `parikshak` passes; it should not change rule derivation inside `prakriya`.

## Candidate Contract

Each pass should be able to emit a candidate with:

- `span`: byte span in the original text.
- `incorrect` / `correction`: replacement surface.
- `rule`, `category`, `kind`, `confidence`: outward diagnostic metadata.
- `pass`: source pass family, such as word, tiryak, padayog, context, style,
  grammar, or punctuation.
- `specificity`: explicit table/exact rule, inventory-backed rule,
  generalized structural rule, or heuristic suggestion.

The current `Diagnostic` type already carries the outward fields. The resolver
work is mainly adding a private candidate wrapper and making the precedence
explicit.

## Precedence

Current implicit precedence is pipeline order plus `blocked_spans`. The resolver
should encode that order directly:

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
   the broader non-ambiguous padayog rewrite already contains the nested
   replacement. In that composite-rewrite case, keep the broader padayog
   diagnostic so replacement does not regress to a partial fix.

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

## Current Behaviors To Pin Before Switching

Before replacing `blocked_spans` with a resolver, tests should pin at least:

- `जगत` yields `जगत्`, not `जग त`.
- same-span word-level errors suppress generalized padayog splits.
- explicit padayog rewrites suppress weaker same-span generalized rewrites.
- strictly nested padayog overlaps use the same precedence tuple as same-span
  overlaps, except when a broader padayog rewrite subsumes the nested
  replacement.
- ambiguous/heuristic diagnostics do not block stronger errors.
- optional style/grammar variants do not displace hard errors.
- punctuation diagnostics remain visible next to word diagnostics when spans do
  not overlap.
- duplicate same-correction alternates collapse into one outward diagnostic.

## Migration Plan

1. Add focused tests for the behaviors above.
2. Introduce a private `Candidate` type and a resolver that accepts candidates
   from the existing passes.
3. First route only padayog/style/grammar candidates through the resolver while
   preserving word-level blocking.
4. Move word-level/context/tiryak candidates into the resolver after snapshot
   diffs show no behavior change.
5. Replace the padayog specificity shim with structured metadata. Today
   `infer_padayog_specificity` classifies some padayog rules from explanation
   text; distinct padayog subrule codes should carry that signal before the
   resolver becomes the only conflict gate.
6. Delete `blocked_spans` only after all candidate-producing passes are routed
   through the resolver.
