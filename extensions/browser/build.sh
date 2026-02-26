#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
PKG_SRC="$ROOT_DIR/web/pkg"
PKG_DST="$SCRIPT_DIR/pkg"
DIST_DIR="$SCRIPT_DIR/dist"

# ── 1. Verify source artifacts ──
if [ ! -f "$PKG_SRC/varnavinyas_bindings_wasm_bg.wasm" ]; then
  echo "Error: WASM artifacts not found at $PKG_SRC" >&2
  echo "Run 'bash web/build.sh' from the repo root first." >&2
  exit 1
fi

# ── 2. Copy WASM artifacts ──
echo "Copying WASM artifacts…"
mkdir -p "$PKG_DST"
cp "$PKG_SRC/varnavinyas_bindings_wasm_bg.wasm" "$PKG_DST/"
cp "$PKG_SRC/varnavinyas_bindings_wasm.js" "$PKG_DST/"

# ── 3. Size report ──
echo ""
echo "── Size report ──"

WASM_RAW=$(wc -c < "$PKG_DST/varnavinyas_bindings_wasm_bg.wasm")
JS_RAW=$(wc -c < "$PKG_DST/varnavinyas_bindings_wasm.js")
echo "  .wasm:  $(echo "$WASM_RAW" | awk '{printf "%.1f MB", $1/1048576}')"
echo "  .js:    $(echo "$JS_RAW" | awk '{printf "%.1f KB", $1/1024}')"

# ── 4. Package zip for store submission ──
mkdir -p "$DIST_DIR"
ZIP_PATH="$DIST_DIR/varnavinyas-extension.zip"
rm -f "$ZIP_PATH"

# Include only extension files (not build.sh, dist/, .gitignore)
(cd "$SCRIPT_DIR" && zip -qr "$ZIP_PATH" \
  manifest.json \
  icons/ \
  src/ \
  pkg/ \
  -x '*.DS_Store')

ZIP_SIZE=$(wc -c < "$ZIP_PATH")
echo "  zip:    $(echo "$ZIP_SIZE" | awk '{printf "%.1f MB", $1/1048576}') ($ZIP_SIZE bytes)"

if [ "$ZIP_SIZE" -lt 8388608 ]; then
  echo "  OK: under 8 MB (Chrome Web Store limit: 10 MB)"
else
  echo "  WARNING: exceeds 8 MB target"
fi

echo ""
echo "── Output ──"
echo "  Unpacked: $SCRIPT_DIR"
echo "  Zip:      $ZIP_PATH"
