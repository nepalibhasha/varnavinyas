# varnavinyas-bindings-wasm

WebAssembly bridge for browser and JavaScript consumers.

## What This Crate Owns

This crate exposes core Varnavinyas functionality to JavaScript through `wasm-bindgen`. It is the runtime bridge used by:

- the web app
- the browser extension
- other JS/TS consumers that need in-browser analysis

## Main APIs

The crate exposes browser-friendly entry points such as:

- `check_text` / `check_text_value`
- `check_word` / `check_word_value`
- `derive`
- `transliterate`
- `analyze_word`
- `decompose_word`
- `sandhi_apply`, `sandhi_split`
- `analyze_compound`

## Example

```js
import init, { check_word_value, sandhi_split_value } from "./pkg/varnavinyas_bindings_wasm.js";

await init();

const diag = check_word_value("राजनैतिक");
const splits = sandhi_split_value("अत्यधिक");
```

## Design Notes

- This crate is an adapter. Core behavior should be implemented in the domain crates, not here.
- It should preserve structured information for frontends whenever possible.
- Generated JS/WASM artifacts must stay in sync with the Rust source; stale generated files can break consumers.

## Used By

- `web/`
- `extensions/browser/`

## Current Limits

- Public schema stability should be tightened further for long-lived frontend clients.
- Optional exports and generated artifacts require careful rebuild discipline.

## Build

```bash
bash web/build.sh
```

## Status

Production-facing adapter used by current browser surfaces.
