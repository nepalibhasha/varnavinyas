# varnavinyas-python

PyO3-based Python bindings for the Varnavinyas workspace.

## What This Crate Owns

This crate exposes Rust functionality to Python as an extension module. It is the Python-facing bridge for:

- spell checking
- transliteration
- sandhi analysis
- word classification
- morphology/correction helpers

## Module Layout

The Python extension is organized into submodules that mirror the Rust workspace:

- `akshar`
- `lipi`
- `shabda`
- `sandhi`
- `prakriya`
- `kosha`
- `lekhya`
- `parikshak`

## Example

```python
import varnavinyas

diagnostics = varnavinyas.parikshak.check_text_with_options(
    "नेपाल एक सुन्दर देश हो।",
    grammar=True,
    punctuation_mode="strict",  # or "normalized_editorial"
    orthography_mode="academy_strict",  # or "common_editorial"
    include_noop_heuristics=False,
)

result = varnavinyas.sandhi.apply("अति", "अधिक")
print(result.sandhi_type.display_label)  # "स्वर सन्धि"
```

## Design Notes

- This crate should stay thin: Rust owns the actual language logic.
- Python consumers should get the same semantics as the Rust APIs, not a separate behavior fork.
- Use `shabda.classify_with_provenance` for origin explanations; it exposes core evidence without the legacy fallback.

## Used By

- Python scripts
- notebooks
- data evaluation and offline analysis workflows

## Current Limits

- The Python-facing API is still less documented and less typed than the Rust layer.
- It inherits the strengths and weaknesses of the core crates; it is not an independent implementation.

## Status

Implemented modules:

- `akshar`
- `lipi`
- `shabda`
- `sandhi`
- `prakriya`
- `kosha`
- `lekhya`
- `parikshak`

Current gaps:

- Broader wheel platform coverage and package-index publishing (the current
  workflow publishes a Linux x86_64 wheel to GitHub Releases)

CI and the wheel release workflow install the built wheel and run
`bash crates/bindings-python/smoke-test.sh`. This covers submodule imports,
enum arguments, nested results, exceptions, and representative corrections.
Set `PYTHON` to select an isolated environment containing the installed wheel.

## Releases and Classification Limits

Tags matching `python-artifact-v*` trigger
`.github/workflows/release-python-artifact.yml`, which builds and uploads a
wheel to GitHub Releases. This does not publish to PyPI.

Version 0.1.3 preserves the existing Python API and updates spelling and
contextual reading behavior. Selected `Ambiguous` suggestions can appear with
grammar heuristics disabled; display their explanations and require individual
review instead of bulk correction. See the [coordinated release
notes](../../docs/releases/native-v0.1.3.md) and [Integration
Notes](../../docs/INTEGRATION_NOTES.md).

`shabda.classify()` retains the four-way best-effort `Origin` enum. For
explanations, use the additive v0.1.2 API:

```python
from varnavinyas import shabda

decision = shabda.classify_with_provenance("नेपाले")
assert decision.origin is None
assert decision.source == shabda.OriginSource.Unknown
assert decision.confidence == 0.0

documented = shabda.classify_with_provenance("टोपी")
assert documented.origin == shabda.Origin.Deshaj
assert documented.source == shabda.OriginSource.Override
```

`OriginDecision` is a read-only result with `origin`, `source`, and `confidence`.
Sources are `Override`, `Kosha`, `Heuristic`, and `Unknown`. Label heuristic
results as inferred; only dictionary/override results are documented. The
legacy Deshaj fallback is unchanged and is not verified etymology. See
[Integration Notes](../../docs/INTEGRATION_NOTES.md).

In v0.1.1, `check_text_with_options` preserves the v0.1.0 positional order
`(text, grammar=False, punctuation_mode="strict", include_noop_heuristics=False)`.
Use the keyword-only `orthography_mode="common_editorial"` to request reviewed
editorial variants. Omitting it keeps Academy-strict behavior. See
[coordinated release notes](../../docs/releases/native-v0.1.1.md).

## Compound information

Compound hints require reviewed formation evidence and preserve outer suffixes.
`samasa-heuristic` is informational; never apply its structural `correction`
string to text or present its ranking weight as an accuracy percentage. Public
signatures and diagnostic fields are unchanged. See the
[compound-analysis contract](../../docs/COMPOUND_ANALYSIS.md) for behavior and
artifact integration requirements.

`shabda.analyze_progressive("खोजिरहेको")` returns a read-only
`ProgressiveAnalysis` with `main_form="खोजि"`, `main_lemma="खोज्नु"`,
`auxiliary_form="रहेको"`, and `auxiliary_lemma="रहनु"`, or `None` outside the
supported family. Existing Python APIs are unchanged. See
[the analysis contract](../../docs/INTEGRATION_NOTES.md#progressive-verb-constructions).

Affix segments additionally distinguish `AffixKind.Postposition` (`घरसम्म`)
and `AffixKind.ComparisonMarker` (`मानिससरह`). Existing enum members are
preserved; reviewed postpositions previously returned `CaseMarker`. Update
exhaustive role handlers using the [relational suffix contract](../../docs/INTEGRATION_NOTES.md#relational-suffix-roles-and-joining).
