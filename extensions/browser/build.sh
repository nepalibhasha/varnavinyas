#!/usr/bin/env bash
set -euo pipefail

# DEPRECATED:
# Browser-extension product ownership has moved to a downstream client repo.
# This script remains only as a temporary compatibility path while the
# migration is in progress. New popup/background/content/manifest changes
# should be made in the downstream client repo, not here.
# varnavinyas should own only the offline browser artifact (`web/pkg`,
# `web/package-artifact.sh`) and the WASM contract consumed downstream.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
PKG_SRC="$ROOT_DIR/web/pkg"
PKG_DST="$SCRIPT_DIR/pkg"
DIST_DIR="$SCRIPT_DIR/dist"
REFRESH_WEB_WASM="${REFRESH_WEB_WASM:-1}"
FIREFOX_MANIFEST="$SCRIPT_DIR/manifest.firefox.json"

echo "WARNING: extensions/browser/ in varnavinyas is deprecated."
echo "WARNING: make extension product changes in the downstream client repo instead."

# ── 1. Refresh web/pkg (default on) ──
if [ "$REFRESH_WEB_WASM" = "1" ]; then
  echo "Refreshing web/pkg via web/build.sh…"
  bash "$ROOT_DIR/web/build.sh"
else
  echo "Skipping web/pkg refresh (REFRESH_WEB_WASM=$REFRESH_WEB_WASM)"
fi

# ── 2. Verify source artifacts ──
if [ ! -f "$PKG_SRC/varnavinyas_bindings_wasm_bg.wasm" ]; then
  echo "Error: WASM artifacts not found at $PKG_SRC" >&2
  echo "Run 'bash web/build.sh' from the repo root first." >&2
  exit 1
fi

# ── 3. Copy WASM artifacts ──
echo "Copying WASM artifacts…"
mkdir -p "$PKG_DST"
cp "$PKG_SRC/varnavinyas_bindings_wasm_bg.wasm" "$PKG_DST/"
cp "$PKG_SRC/varnavinyas_bindings_wasm.js" "$PKG_DST/"

# ── 4. Size report ──
echo ""
echo "── Size report ──"

WASM_RAW=$(wc -c < "$PKG_DST/varnavinyas_bindings_wasm_bg.wasm")
JS_RAW=$(wc -c < "$PKG_DST/varnavinyas_bindings_wasm.js")
echo "  .wasm:  $(echo "$WASM_RAW" | awk '{printf "%.1f MB", $1/1048576}')"
echo "  .js:    $(echo "$JS_RAW" | awk '{printf "%.1f KB", $1/1024}')"

# ── 5. Package zip for store submission ──
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

# ── 6. Firefox dev bundle (optional) ──
FIREFOX_UNPACKED_PATH=""
FIREFOX_ZIP_PATH=""
if [ -f "$FIREFOX_MANIFEST" ]; then
  FIREFOX_UNPACKED="$DIST_DIR/firefox-unpacked"
  FIREFOX_ZIP_PATH="$DIST_DIR/varnavinyas-extension-firefox.zip"
  rm -rf "$FIREFOX_UNPACKED"
  mkdir -p "$FIREFOX_UNPACKED"
  cp -R "$SCRIPT_DIR/icons" "$FIREFOX_UNPACKED/"
  cp -R "$SCRIPT_DIR/src" "$FIREFOX_UNPACKED/"
  cp -R "$SCRIPT_DIR/pkg" "$FIREFOX_UNPACKED/"
  cp "$FIREFOX_MANIFEST" "$FIREFOX_UNPACKED/manifest.json"

  rm -f "$FIREFOX_ZIP_PATH"
  (cd "$FIREFOX_UNPACKED" && zip -qr "$FIREFOX_ZIP_PATH" \
    manifest.json \
    icons/ \
    src/ \
    pkg/ \
    -x '*.DS_Store')

  FIREFOX_UNPACKED_PATH="$FIREFOX_UNPACKED"
fi

echo ""
echo "── Output ──"
echo "  Unpacked: $SCRIPT_DIR"
echo "  Zip:      $ZIP_PATH"
if [ -n "$FIREFOX_UNPACKED_PATH" ]; then
  echo "  Firefox unpacked: $FIREFOX_UNPACKED_PATH"
  echo "  Firefox zip:      $FIREFOX_ZIP_PATH"
fi
