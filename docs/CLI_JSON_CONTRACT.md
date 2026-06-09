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

By default, only diagnostics with `kind: "error"` are blocking. With
`--fail-on-suggestions`, any diagnostic is blocking.

## Diagnostic Object

Each array item has these required fields:

| Field | Type | Meaning |
| --- | --- | --- |
| `line` | integer | 1-based line number of the diagnostic start. |
| `column` | integer | 1-based character column of the diagnostic start. |
| `incorrect` | string | Source text covered by the diagnostic. |
| `correction` | string | Suggested replacement text. |
| `rule` | string | Stable-ish rule identifier or rule display string. |
| `category` | string | Stable diagnostic category code, such as `HrasvaDirgha`. |
| `explanation` | string | Human-readable explanation. |
| `kind` | string | Lowercase diagnostic kind, such as `error`, `variant`, or `ambiguous`. |
| `confidence` | number | Confidence score in the range `0.0` to `1.0`. |

When a word has additional independent reasons, the diagnostic also includes:

| Field | Type | Meaning |
| --- | --- | --- |
| `alternate_reasons` | array | Additional applicable rule reasons. Omitted when empty. |

Each `alternate_reasons` item has these required fields:

| Field | Type | Meaning |
| --- | --- | --- |
| `rule` | string | Alternate rule identifier or rule display string. |
| `category` | string | Alternate diagnostic category code. |
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
    "category": "HrasvaDirgha",
    "explanation": "...",
    "kind": "error",
    "confidence": 1.0
  }
]
```
