# varnavinyas-bindings-python

**Python Bindings for Varnavinyas.**

Exposes core functionality to Python via PyO3.

## Installation

```bash
pip install varnavinyas
```

## Usage

```python
import varnavinyas

diagnostics = varnavinyas.parikshak.check_text_with_options(
    "नेपाल एक सुन्दर देश हो।",
    grammar=True,
    punctuation_mode="strict",  # or "normalized_editorial"
    include_noop_heuristics=False,
)

result = varnavinyas.sandhi.apply("अति", "अधिक")
print(result.sandhi_type.display_label)  # "स्वर सन्धि"
```

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

- Publish/release automation for Python wheels in CI
- Python-level runtime integration tests (import + API smoke tests)
