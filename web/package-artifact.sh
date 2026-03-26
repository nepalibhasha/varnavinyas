#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PKG_DIR="$SCRIPT_DIR/pkg"
DIST_DIR="$SCRIPT_DIR/dist"
ARTIFACT_DIR="$DIST_DIR/varnavinyas-browser-artifact"
ARTIFACT_ZIP="$DIST_DIR/varnavinyas-browser-artifact.zip"
REFRESH_WEB_WASM="${REFRESH_WEB_WASM:-1}"
ARTIFACT_VERSION_OVERRIDE="${ARTIFACT_VERSION:-}"

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

REQUIRED_EXPORTS="
check_word_value
analyze_word_value
best_affix_analysis_value
decompose_word_value
sandhi_split_value
analyze_compound_value
"

for fn in $REQUIRED_EXPORTS; do
  if ! grep -q "export function ${fn}" "$PKG_DIR/varnavinyas_bindings_wasm.js"; then
    echo "Error: required export ${fn} missing from $PKG_DIR/varnavinyas_bindings_wasm.js" >&2
    exit 1
  fi
done

extract_json_string() {
  local key="$1"
  awk -F'"' -v target="$key" '$2 == target { print $4; exit }' "$SCRIPT_DIR/build-info.json"
}

normalize_artifact_version() {
  local raw="$1"
  raw="${raw#refs/tags/}"
  raw="${raw#browser-artifact-}"
  printf '%s' "$raw"
}

ARTIFACT_VERSION="$ARTIFACT_VERSION_OVERRIDE"
if [ -z "$ARTIFACT_VERSION" ]; then
  ARTIFACT_VERSION="$(extract_json_string version)"
fi
if [ -z "$ARTIFACT_VERSION" ]; then
  ARTIFACT_VERSION="unknown"
fi
ARTIFACT_VERSION="$(normalize_artifact_version "$ARTIFACT_VERSION")"

GIT_SHA="$(extract_json_string git_sha)"
if [ -z "$GIT_SHA" ]; then
  GIT_SHA="unknown"
fi

ARTIFACT_VERSION_SAFE="$(printf '%s' "$ARTIFACT_VERSION" | tr '/ ' '--')"
VERSIONED_ARTIFACT_ZIP="$DIST_DIR/varnavinyas-browser-artifact-${ARTIFACT_VERSION_SAFE}.zip"

mkdir -p "$DIST_DIR"
rm -rf "$ARTIFACT_DIR"
mkdir -p "$ARTIFACT_DIR/pkg"

cp "$PKG_DIR/varnavinyas_bindings_wasm_bg.wasm" "$ARTIFACT_DIR/pkg/"
cp "$PKG_DIR/varnavinyas_bindings_wasm.js" "$ARTIFACT_DIR/pkg/"
cp "$SCRIPT_DIR/build-info.json" "$ARTIFACT_DIR/"

cat > "$ARTIFACT_DIR/manifest.json" <<EOF
{
  "artifact": "varnavinyas-browser-artifact",
  "artifact_version": "${ARTIFACT_VERSION}",
  "git_sha": "${GIT_SHA}",
  "description": "Browser-consumable WASM package for downstream extension clients",
  "pkg_dir": "pkg",
  "entry_js": "pkg/varnavinyas_bindings_wasm.js",
  "entry_wasm": "pkg/varnavinyas_bindings_wasm_bg.wasm",
  "build_info": "build-info.json",
  "required_exports": [
    "check_word_value",
    "analyze_word_value",
    "best_affix_analysis_value",
    "decompose_word_value",
    "sandhi_split_value",
    "analyze_compound_value"
  ]
}
EOF

rm -f "$ARTIFACT_ZIP"
rm -f "$VERSIONED_ARTIFACT_ZIP"
(cd "$ARTIFACT_DIR" && zip -qr "$ARTIFACT_ZIP" manifest.json build-info.json pkg/)
cp "$ARTIFACT_ZIP" "$VERSIONED_ARTIFACT_ZIP"

echo "Artifact directory: $ARTIFACT_DIR"
echo "Artifact zip:       $ARTIFACT_ZIP"
echo "Versioned zip:      $VERSIONED_ARTIFACT_ZIP"
