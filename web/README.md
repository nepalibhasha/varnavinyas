# Web App Guide

This directory contains the static web UI for Varnavinyas.

## What Is Here

- `index.html`: page shell
- `css/style.css`: styles
- `js/*.js`: app modules (checker, inspector, rules reference, wasm bridge)
- `pkg/`: generated WASM bindings consumed by the browser
- `build.sh`: builds `pkg/` from Rust WASM bindings
- `smoke-test.sh`: quick end-to-end static checks

## Local Run

From repo root:

```bash
bash web/build.sh
python3 -m http.server 8080 --directory web/
```

Open `http://localhost:8080`.

## Smoke Test

From repo root:

```bash
bash web/smoke-test.sh
```

The smoke test validates:

- WASM artifacts and exported functions
- category mapping consistency (Rust -> JS -> CSS)
- key static assets served by a local HTTP server

## Editing Notes

- Keep diagnostics keyed by `category_code` (machine-stable), not display labels.
- `checkText(..., { grammar: true })` enables heuristic/style variants in UI.
- Rule citations are rendered through `wrapRuleTooltip(...)` in `js/rules-data.js`.

## Common Issues

- `wasm-bindgen-cli version mismatch`:
  run the version printed by `web/build.sh`.
- Browser still shows old behavior:
  hard refresh after rebuilding `web/pkg/`.
