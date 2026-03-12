#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PKG_DIR="$SCRIPT_DIR/pkg"
DIST_DIR="$SCRIPT_DIR/dist"
ARTIFACT_DIR="$DIST_DIR/varnavinyas-browser-artifact"
ARTIFACT_ZIP="$DIST_DIR/varnavinyas-browser-artifact.zip"
REFRESH_WEB_WASM="${REFRESH_WEB_WASM:-1}"

if [ "$REFRESH_WEB_WASM" = "1" ]; then
  bash "$SCRIPT_DIR/build.sh"
fi

# build-info.json is written by build.sh (includes wasm_opt metadata).
# If the web build was skipped, the existing file is reused as-is.
if [ ! -f "$SCRIPT_DIR/build-info.json" ]; then
  echo "Error: build-info.json not found; run web/build.sh first." >&2
  exit 1
fi

if [ ! -f "$PKG_DIR/varnavinyas_bindings_wasm_bg.wasm" ]; then
  echo "Error: expected WASM binary in $PKG_DIR" >&2
  exit 1
fi

if [ ! -f "$PKG_DIR/varnavinyas_bindings_wasm.js" ]; then
  echo "Error: expected WASM JS glue in $PKG_DIR" >&2
  exit 1
fi

mkdir -p "$DIST_DIR"
rm -rf "$ARTIFACT_DIR"
mkdir -p "$ARTIFACT_DIR/pkg"

cp "$PKG_DIR/varnavinyas_bindings_wasm_bg.wasm" "$ARTIFACT_DIR/pkg/"
cp "$PKG_DIR/varnavinyas_bindings_wasm.js" "$ARTIFACT_DIR/pkg/"
cp "$SCRIPT_DIR/build-info.json" "$ARTIFACT_DIR/"

cat > "$ARTIFACT_DIR/manifest.json" <<EOF
{
  "artifact": "varnavinyas-browser-artifact",
  "description": "Browser-consumable WASM package for downstream extension clients",
  "pkg_dir": "pkg",
  "entry_js": "pkg/varnavinyas_bindings_wasm.js",
  "entry_wasm": "pkg/varnavinyas_bindings_wasm_bg.wasm",
  "build_info": "build-info.json",
  "required_exports": [
    "check_word_value",
    "analyze_word_value",
    "decompose_word_value",
    "sandhi_split_value",
    "analyze_compound_value"
  ]
}
EOF

rm -f "$ARTIFACT_ZIP"
(cd "$ARTIFACT_DIR" && zip -qr "$ARTIFACT_ZIP" manifest.json build-info.json pkg/)

echo "Artifact directory: $ARTIFACT_DIR"
echo "Artifact zip:       $ARTIFACT_ZIP"
