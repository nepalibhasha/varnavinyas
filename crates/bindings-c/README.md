# varnavinyas-bindings-c

C ABI wrapper for embedding Varnavinyas in non-Rust environments.

## What This Crate Owns

This crate exposes a narrow, C-compatible API over selected workspace functionality. It is intended for:

- native integrations that cannot call Rust directly
- thin wrappers in other languages
- environments where a simple C ABI is the safest interoperability target

## What It Exposes

The current exported surface is intentionally small:

- `varnavinyas_check_text` -> returns JSON diagnostics for full-text checking
- `varnavinyas_check_text_with_options` -> returns JSON diagnostics with grammar, punctuation-mode, and debug heuristic options; keeps Academy-strict orthography
- `varnavinyas_check_text_with_all_options` -> additionally accepts orthography mode
- `varnavinyas_check_word` -> returns one JSON diagnostic object or `null`
- `varnavinyas_transliterate` -> transliterates between supported schemes
- `varnavinyas_classify` -> origin classification
- `varnavinyas_free_string` -> frees returned strings
- `varnavinyas_version` -> returns the library version string

Constants exported for callers:

- `SCHEME_DEVANAGARI`, `SCHEME_IAST`
- `PUNCTUATION_STRICT`, `PUNCTUATION_NORMALIZED_EDITORIAL`
- `ORTHOGRAPHY_ACADEMY_STRICT`, `ORTHOGRAPHY_COMMON_EDITORIAL`

## Example

```c
char *json = varnavinyas_check_text("यो बाक्यमा गल्ति छ");
if (json != NULL) {
    puts(json);
    varnavinyas_free_string(json);
}

char *strict_json = varnavinyas_check_text_with_options(
    "यो बाक्यमा गल्ति छ",
    false,
    PUNCTUATION_STRICT,
    false
);
if (strict_json != NULL) {
    puts(strict_json);
    varnavinyas_free_string(strict_json);
}

char *common_json = varnavinyas_check_text_with_all_options(
    "नेपाली कांग्रेस",
    false,
    PUNCTUATION_STRICT,
    ORTHOGRAPHY_COMMON_EDITORIAL,
    false
);
if (common_json != NULL) {
    puts(common_json);
    varnavinyas_free_string(common_json);
}
```

## Design Notes

- The ABI is conservative by design.
- Complex outputs are serialized as JSON strings rather than exported as large C structs.
- This crate should remain a transport layer, not a place where language logic is implemented.

## Used By

- downstream native applications that need a stable C boundary
- foreign-language wrappers that prefer C over direct Rust FFI

## Current Limits

- The API is intentionally narrower than the Rust API.
- Structured outputs are flattened into JSON for portability.
- ABI versioning is still minimal and should be strengthened before broad external adoption.

## Status

Implemented MVP wrapper.
