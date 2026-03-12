#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

# Preflight: check required tools
if ! command -v wasm-pack &>/dev/null; then
  echo "Error: wasm-pack is not installed." >&2
  echo "" >&2
  echo "Install it with one of:" >&2
  echo "  cargo install wasm-pack" >&2
  echo "  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh" >&2
  exit 1
fi

if ! command -v cargo &>/dev/null; then
  echo "Error: cargo is not installed. Install Rust via https://rustup.rs" >&2
  exit 1
fi

EXPECTED_WASM_BINDGEN_VERSION="$(
  awk '
    $0 == "name = \"wasm-bindgen\"" {
      getline
      if ($1 == "version" && $2 == "=") {
        gsub(/"/, "", $3)
        print $3
        exit
      }
    }
  ' Cargo.lock
)"

if [ -z "${EXPECTED_WASM_BINDGEN_VERSION}" ]; then
  echo "Error: could not determine wasm-bindgen version from Cargo.lock." >&2
  exit 1
fi

if ! command -v wasm-bindgen &>/dev/null; then
  echo "Error: wasm-bindgen-cli is not installed." >&2
  echo "" >&2
  echo "Install the exact version required by this workspace lockfile:" >&2
  echo "  cargo install wasm-bindgen-cli --version ${EXPECTED_WASM_BINDGEN_VERSION}" >&2
  exit 1
fi

INSTALLED_WASM_BINDGEN_VERSION="$(wasm-bindgen --version | awk '{print $2}')"
if [ "${INSTALLED_WASM_BINDGEN_VERSION}" != "${EXPECTED_WASM_BINDGEN_VERSION}" ]; then
  echo "Error: wasm-bindgen-cli version mismatch." >&2
  echo "  Required: ${EXPECTED_WASM_BINDGEN_VERSION}" >&2
  echo "  Found:    ${INSTALLED_WASM_BINDGEN_VERSION}" >&2
  echo "" >&2
  echo "Install matching CLI version:" >&2
  echo "  cargo install -f wasm-bindgen-cli --version ${EXPECTED_WASM_BINDGEN_VERSION}" >&2
  exit 1
fi

echo "Building WASM bindings..."
# Use no-install mode to avoid implicit cargo install/network side effects in CI/sandboxes.
wasm-pack build crates/bindings-wasm \
  --target web \
  --out-dir ../../web/pkg \
  --release \
  --mode no-install

WASM_FILE="web/pkg/varnavinyas_bindings_wasm_bg.wasm"

# ── wasm-opt (optional) ──────────────────────────────────────────────────────
# Optimize for size when binaryen's wasm-opt is available. This mainly affects
# code size; the dominant lexicon data section is largely unaffected.
WASM_OPT_APPLIED="false"
WASM_OPT_VERSION="null"
WASM_SIZE_BEFORE=$(wc -c < "${WASM_FILE}")
WASM_SIZE_AFTER="${WASM_SIZE_BEFORE}"

if command -v wasm-opt &>/dev/null; then
  WASM_OPT_VERSION="\"$(wasm-opt --version 2>&1 | awk '{print $NF}')\""
  echo "Running wasm-opt -Oz ..."
  wasm-opt -Oz \
    --enable-bulk-memory \
    --enable-mutable-globals \
    --enable-nontrapping-float-to-int \
    "${WASM_FILE}" -o "${WASM_FILE}"
  WASM_SIZE_AFTER=$(wc -c < "${WASM_FILE}")
  WASM_OPT_APPLIED="true"
  SAVED=$((WASM_SIZE_BEFORE - WASM_SIZE_AFTER))
  echo "  wasm-opt: ${WASM_SIZE_BEFORE} → ${WASM_SIZE_AFTER} bytes (${SAVED} bytes saved)"
else
  echo "wasm-opt not found; skipping size optimisation."
  echo "  Install binaryen to enable: brew install binaryen  OR  apt install binaryen"
fi

BUILD_TIME_UTC="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
GIT_SHA="$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")"
ROOT_PKG_VERSION="$(
  awk '
    $0 == "[workspace.package]" { in_workspace = 1; next }
    /^\[/ && $0 != "[workspace.package]" { in_workspace = 0 }
    in_workspace && $1 == "version" && $2 == "=" {
      gsub(/"/, "", $3)
      print $3
      exit
    }
  ' Cargo.toml
)"
if [ -z "${ROOT_PKG_VERSION}" ]; then
  ROOT_PKG_VERSION="git-${GIT_SHA}"
fi
cat > web/build-info.json <<EOF
{
  "built_at_utc": "${BUILD_TIME_UTC}",
  "git_sha": "${GIT_SHA}",
  "version": "${ROOT_PKG_VERSION}",
  "wasm_bindgen_cli": "${INSTALLED_WASM_BINDGEN_VERSION}",
  "wasm_opt": {
    "applied": ${WASM_OPT_APPLIED},
    "version": ${WASM_OPT_VERSION},
    "flags": "-Oz",
    "size_before": ${WASM_SIZE_BEFORE},
    "size_after": ${WASM_SIZE_AFTER}
  }
}
EOF

echo "Done. Serve with: python3 -m http.server 8080 --directory web/"
