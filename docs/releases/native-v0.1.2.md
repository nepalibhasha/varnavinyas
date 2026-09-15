# Python, CLI, iOS, and Android v0.1.2

This coordinated release adds origin provenance without changing existing
classification calls or the four-value `Origin` enum. All packages use one
source commit and include matching diagnostic and origin fixtures, manifests,
integration documentation, and checksums.

## Origin Evidence

Python `shabda.classify_with_provenance(word)` and Swift/Kotlin
`classifyWithProvenance` return an `OriginDecision` with three fields:

- `origin`: an existing `Origin` value, or `None`/`nil`/`null` when unknown.
- `source`: `OriginSource.Override`, `Kosha`, `Heuristic`, or `Unknown`
  (language-specific enum casing applies).
- `confidence`: the core's evidence score. Unknown is always zero. These
  scores are not calibrated probabilities.

Dictionary and reviewed override evidence can be displayed as documented.
Heuristic evidence must be labelled inferred. No evidence must be displayed as
unknown, even for a recognized dictionary headword. For example, `नेपाले` has
no supported origin, while `टोपी` retains its documented Deshaj classification.

The older `classify()` calls still return their original four-way category,
including the Deshaj fallback. Use the new API for user-facing explanations.
Existing decomposition origin fields also retain their compatibility behavior.

## CLI

The new `varnavinyas classify "नेपाले" --format json` command emits:

```json
{"origin":null,"source":"unknown","confidence":0.0}
```

Known origins use `tatsam`, `tadbhav`, `deshaj`, or `aagantuk`. Sources use
`override`, `kosha`, `heuristic`, or `unknown`. Default text output explicitly
labels documented, inferred, or missing evidence. Unknown classification is
not an execution error; the command exits zero. Existing `check` output and
exit codes are unchanged.

## Compatibility and Evaluation

Python preserves all previous imports, classes, enum values, existing function
signatures, and positional arguments. Orthography mode remains keyword-only
on `check_text_with_options`. Academy-strict remains the default across all
surfaces; common-editorial support is unchanged. No linguistic rules or
dictionary data change in this release.

iOS and Android bundles include regenerated Swift/Kotlin bindings matching
their native libraries. Upgrade both together. Eight shared origin cases test
unknown input, headwords without origin metadata, all four origin categories,
dictionary/override/heuristic evidence, and legacy classification compatibility.
The existing 16 diagnostic fixtures remain unchanged. Manifests advertise
origin provenance and checksum both fixture files.

Release workflows exercise installed Python and generated Swift/Kotlin APIs
and all CLI targets. Mobile evaluation runs against host libraries, with an
additional iOS simulator SDK type-check; application/device loading still
requires integration testing. Python artifacts are published to GitHub Releases,
not PyPI. The C classification API retains its existing documented limitation.
