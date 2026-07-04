# Integration Notes

## Orthography Mode

The checker supports two orthography policies:

- `academy-strict`: compatibility default. Academy-prescriptive forms are
  emitted as `kind: "Error"`.
- `common-editorial`: reviewed common-vs-strict forms are emitted as
  `kind: "Variant"` with the strict form still present in `correction`.

The JSON diagnostic shape is unchanged. Consumers should key behavior from the
existing `kind` field:

- Treat `Error` as blocking.
- Treat `Variant` as non-blocking unless the user explicitly wants suggestions
  to fail a check.

The common-editorial boundary is curated, not frequency-based. Adding a future
variant requires:

1. Evidence of strict-source pressure from the Academy references or lexicon.
2. Evidence that the common form is stable enough for editorial use.
3. A source note in `crates/parikshak/src/checker/orthography_variants.rs`.
4. Tests proving strict mode remains blocking and common-editorial mode emits a
   variant.

Current reviewed common-editorial variants:

| Common form | Strict form | Category |
| --- | --- | --- |
| `संघीय` | `सङ्घीय` | `Chandrabindu` |
| `संघ` | `सङ्घ` | `Chandrabindu` |
| `संचार` | `सञ्चार` | `Chandrabindu` |
| `संकेत` | `सङ्केत` | `Chandrabindu` |
| `संसद` | `संसद्` | `Halanta` |
| `कांग्रेस` | `काङ्ग्रेस` | `Chandrabindu` |

Surface-specific option names:

| Surface | Option |
| --- | --- |
| CLI | `--orthography-mode academy-strict\|common-editorial` |
| Rust | `CheckOptions { orthography_mode, ... }` |
| LSP | `orthographyMode` or legacy `orthography_mode` |
| WASM | `check_text_value_with_options(text, grammar, orthography_mode)` |
| Python | `orthography_mode="academy_strict"` or `"common_editorial"` |
| C | `varnavinyas_check_text_with_all_options(..., orthography_mode, ...)` |
| UniFFI | `check_text_with_all_options(..., OrthographyMode, ...)` |

For browser artifacts, `check_text_value(text, grammar)` remains the
backward-compatible default API and uses `academy-strict`. Downstream clients
that need explicit policy selection should use
`check_text_value_with_options(text, grammar, orthography_mode)` when
`manifest.json` advertises the capability.
