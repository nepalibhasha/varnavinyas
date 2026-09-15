# Python, CLI, iOS, and Android v0.1.1

This coordinated release set uses one source commit. Each artifact carries a
manifest with that commit and checksums. iOS and Android include matching
generated bindings, shared diagnostic fixtures, and evaluation harnesses.

## Behavior Changes

- Refreshed dictionary metadata and reviewed inventories improve correction
  evidence and explanations.
- Ambiguous root/imperative pairs such as हेर / हेर् are preserved rather than
  entering a correction cycle.
- हामि → हामी keeps the pronoun rule and drops unsupported alternate reasons.
  Morphology no longer treats हामी as हाम + ई or संघीय as सम् + घीय. Explicitly
  documented derivations such as तिनी = तिन + ई remain available.
- Correction fallbacks and overlapping diagnostics use more specific evidence.
  Corrections, confidence, ordering, and alternate-reason counts can therefore
  differ from v0.1.0; diagnostic identifiers and public API names are preserved.
- PyO3 0.29.2 and crossbeam-epoch 0.9.20 resolve the previously reported
  dependency advisories. Python class conversions retain compatibility.

## Python API Compatibility

The import name remains `varnavinyas`, with the same eight submodules, existing
classes, enums, attributes, return types, and checking functions. Existing
`Rule` objects remain structured objects; they are not replaced with strings.
Python 3.10+ is supported through the abi3 wheel. The version is now 0.1.1.
There are no removals or renames, and classification retains its existing
four-value `Origin` enum. The browser's new `origin: "unknown"` value is not
introduced into this Python enum.

Use `check_text_with_options(text, orthography_mode="common_editorial")` for
reviewed editorial variants. Both underscore and hyphen spellings of the
orthography modes are accepted. `academy_strict` remains the default. Existing
positional/keyword calls retain their parameter order and defaults:
`(text, grammar=False, punctuation_mode="strict", include_noop_heuristics=False)`.
The new `orthography_mode` argument is keyword-only, so the existing fourth
positional Boolean remains `include_noop_heuristics`.

## Orthography Modes Across Surfaces

| Surface | Explicit mode selection |
| --- | --- |
| Python | `varnavinyas.check_text_with_options(text, orthography_mode="common_editorial")` |
| CLI | `varnavinyas check --orthography-mode common-editorial --format json` |
| Swift | `checkTextWithAllOptions(..., orthographyMode: .commonEditorial, ...)` |
| Kotlin | `checkTextWithAllOptions(..., OrthographyMode.COMMON_EDITORIAL, ...)` |

Academy-strict is the compatibility default. Common-editorial changes only
reviewed forms such as संघीय from `Error` to `Variant`, retaining सङ्घीय as the
suggested standard spelling. It does not suppress ordinary mistakes.
CLI exit code 1 means a blocking error; variants alone exit 0 unless
`--fail-on-suggestions` is used. The CLI JSON shape remains unchanged.

Mobile checking returns JSON strings with UTF-8 byte spans. CLI JSON instead
reports one-based lines and character columns. See the included integration
and mobile evaluation guides for exact conventions and fixture usage.
