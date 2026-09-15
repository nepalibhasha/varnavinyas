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
- When the core crates gain better provenance/authority metadata, this crate should expose that directly.

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

`shabda.classify()` retains the four-way best-effort `Origin` enum. It does not
expose the core's origin provenance, so a fallback `Deshaj` must not be shown as
verified etymology. See [Integration Notes](../../docs/INTEGRATION_NOTES.md).

In v0.1.1, `check_text_with_options` preserves the v0.1.0 positional order
`(text, grammar=False, punctuation_mode="strict", include_noop_heuristics=False)`.
Use the keyword-only `orthography_mode="common_editorial"` to request reviewed
editorial variants. Omitting it keeps Academy-strict behavior. See
[coordinated release notes](../../docs/releases/native-v0.1.1.md).
