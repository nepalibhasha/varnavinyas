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
3. For the same kind, source pass precedence is:
   `word` > `tiryak` > `padayog` > `context` > `style` > `grammar`.
4. Punctuation is independent unless it overlaps a non-punctuation diagnostic;
   in an overlap, non-punctuation wins.
5. For the same pass, higher specificity wins:
   exact table / cited explicit rule > curated inventory > generalized rule >
   heuristic.
6. For otherwise equivalent candidates, higher confidence wins.
7. If candidates have the same span and correction, keep one diagnostic and
   merge distinct alternate reasons instead of surfacing duplicates.

The important generalization from the `जगत` class of bugs is:

- word-level Academy corrections on the same token span beat generalized
  padayog/padabiyog splits.
- A generalized splitter should not need bespoke knowledge of every word-level
  rule family; the resolver should enforce that ordering.

## Current Behaviors To Pin Before Switching

Before replacing `blocked_spans` with a resolver, tests should pin at least:

- `जगत` yields `जगत्`, not `जग त`.
- same-span word-level errors suppress generalized padayog splits.
- explicit padayog rewrites suppress weaker same-span generalized rewrites.
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
5. Delete `blocked_spans` only after all candidate-producing passes are routed
   through the resolver.
