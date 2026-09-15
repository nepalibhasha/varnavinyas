# Web App Guide

This directory contains the static web UI for Varnavinyas.

## What Is Here

```text
web/
  index.html          page shell
  css/style.css       styles
  js/*.js             app modules
  pkg/                generated WASM glue
  build.sh            WASM build helper
  package-artifact.sh downstream browser artifact packager
  smoke-test.sh       static consistency checks
```

- `index.html`: page shell
- `css/style.css`: styles
- `js/*.js`: app modules (checker, inspector, rules reference, wasm bridge)
- `pkg/`: generated WASM bindings consumed by the browser
- `build.sh`: builds `pkg/` from Rust WASM bindings
- `package-artifact.sh`: packages `pkg/` + metadata for downstream browser clients
- `smoke-test.sh`: quick end-to-end static checks

## Local Run

From repo root:

```bash
bash web/build.sh
python3 -m http.server 8080 --directory web/
```

Open `http://localhost:8080`.

```mermaid
flowchart LR
    A[Rust/WASM bindings]
    B[web/pkg]
    C[web/js modules]
    D[Browser UI]

    A -->|bash web/build.sh| B --> C --> D
```

## Smoke Test

From repo root:

```bash
bash web/smoke-test.sh
```

The smoke test validates:

- WASM artifacts and exported functions
- category mapping consistency (Rust -> JS -> CSS)
- key static assets served by a local HTTP server
- inspector and rule-reference regressions (`node --test web/tests/*.test.mjs`, Node 22+ recommended)

## Downstream Clients

This repo can also produce a browser-consumable artifact for downstream extension clients.

From repo root:

```bash
bash web/package-artifact.sh
```

This emits:

- `web/dist/varnavinyas-browser-artifact/`
- `web/dist/varnavinyas-browser-artifact.zip`
- `web/dist/varnavinyas-browser-artifact-<version>.zip`

Packaging rebuilds WASM by default. Set `REFRESH_WEB_WASM=0` only when reusing
a verified build of the same source revision. Set `ARTIFACT_VERSION` explicitly
for a release; without it, this workspace currently uses a `git-<sha>` version.
Build from committed source so `build-info.json.git_sha` identifies the contents.
Its `built_at_utc` value is the source commit time, not the packaging time.

The ZIP contains the WASM binary, JavaScript glue, manifest, and build metadata.
It does **not** include the website's editor, inspector, CSS, or rule-reference
modules. Downstream clients must update their own rendering when adopting
analysis changes such as `origin: "unknown"`; see
[Integration Notes](../docs/INTEGRATION_NOTES.md).

GitHub Pages rebuilds the full website on pushes to `main`. A separate tag
matching `browser-artifact-v*` builds and publishes the downstream ZIP through
`.github/workflows/release-browser-artifact.yml`. See the
[release procedure](../docs/DEVELOPMENT.md#publishing-browser-artifacts).

The artifact `manifest.json` is the downstream contract surface. It includes:

- `artifact_api_version`
- `capabilities`
- `required_exports`

`check_text_value(text, grammar)` is the backward-compatible default text API
and uses `academy-strict` orthography mode. Clients that need explicit policy
control should prefer `check_text_value_with_options(text, grammar,
orthography_mode)` when the manifest advertises that capability.

## Editing Notes

- Keep diagnostics keyed by `category_code` (machine-stable), not display labels.
- `checkText(..., { grammar: true })` enables heuristic/style variants in UI.
- `checkText(..., { orthographyMode: "common-editorial" })` downgrades only
  reviewed common-vs-strict orthographic forms to non-blocking variants. See
  `../docs/INTEGRATION_NOTES.md`.
- The editor passes its contextual result to the inspector. Corrections and severity
  follow the current checking policy; isolated word analysis supplies linguistic
  structure and compatible explanatory notes. Text edits and policy changes dismiss
  stale inspector content.
- Rule links resolve `rule_code`, retaining `rule` as their readable label.
- Rule citations are rendered through `wrapRuleTooltip(...)` in `js/rules-data.js`.

## Common Issues

- `wasm-bindgen-cli version mismatch`:
  run the version printed by `web/build.sh`.
- Browser still shows old behavior:
  hard refresh after rebuilding `web/pkg/`.

Origin badges distinguish documented (`kosha` / `override`), inferred
(`heuristic`), and unknown (`unknown`) origins. They never treat a missing
origin tag as evidence that a word is देशज.
