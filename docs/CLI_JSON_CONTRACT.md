# CLI JSON Output Contract

This contract covers `varnavinyas check --format json`, the integration surface
intended for editor packages such as `nepali.el`.

## Stability

- Release tags for CLI binaries use the stable form `cli-vMAJOR.MINOR.PATCH`.
- The JSON top level is a list of diagnostics. Clean input returns `[]`.
- Existing fields will keep their names, JSON types, and basic meanings within a
  compatible release line.
- New fields may be added to diagnostic objects or nested objects. Consumers must
  ignore fields they do not understand.
- Field ordering and pretty-printing are not part of the contract.

## Exit Codes

- `0`: no blocking diagnostics were found.
- `1`: at least one blocking diagnostic was found.
- `2`: input, argument, or runtime usage error.

By default, only diagnostics with `kind: "Error"` are blocking. With
`--fail-on-suggestions`, any diagnostic is blocking.

`--orthography-mode academy-strict` is the default and reports Academy-
prescriptive spellings as `kind: "Error"`. `--orthography-mode
common-editorial` keeps the same diagnostic object shape but reports reviewed
common-vs-strict orthographic forms as `kind: "Variant"` with the strict form in
`correction`. This mode is intentionally curated; unreviewed spelling mistakes
remain errors.

## Diagnostic Object

Each array item has these required fields:

| Field | Type | Meaning |
| --- | --- | --- |
| `line` | integer | 1-based line number of the diagnostic start. |
| `column` | integer | 1-based character column of the diagnostic start. |
| `incorrect` | string | Source text covered by the diagnostic. |
| `correction` | string | Suggested replacement text. |
| `rule` | string | Stable-ish rule identifier or rule display string. |
| `rule_code` | string | Stable machine-readable rule code. |
| `category` | string | Stable diagnostic category code, such as `HrasvaDirgha`. |
| `category_code` | string | Stable diagnostic category code, same value as `category`. |
| `category_label` | string | Human-readable category label. |
| `explanation` | string | Human-readable explanation. |
| `kind` | string | Diagnostic kind, such as `Error`, `Variant`, or `Ambiguous`. |
| `confidence` | number | Confidence score in the range `0.0` to `1.0`. |

When a word has additional independent reasons, the diagnostic also includes:

| Field | Type | Meaning |
| --- | --- | --- |
| `alternate_reasons` | array | Additional applicable rule reasons. Omitted when empty. |

Each `alternate_reasons` item has these required fields:

| Field | Type | Meaning |
| --- | --- | --- |
| `rule` | string | Alternate rule identifier or rule display string. |
| `rule_code` | string | Stable machine-readable alternate rule code. |
| `category` | string | Alternate diagnostic category code. |
| `category_code` | string | Alternate diagnostic category code, same value as `category`. |
| `category_label` | string | Human-readable alternate category label. |
| `explanation` | string | Human-readable alternate explanation. |
| `correction` | string | Suggested replacement for the alternate reason. |

## Example

```json
[
  {
    "line": 1,
    "column": 1,
    "incorrect": "अत्याधिक",
    "correction": "अत्यधिक",
    "rule": "...",
    "rule_code": "...",
    "category": "ShuddhaTable",
    "category_code": "ShuddhaTable",
    "category_label": "शुद्ध-अशुद्ध",
    "explanation": "...",
    "kind": "Error",
    "confidence": 1.0
  }
]
```
