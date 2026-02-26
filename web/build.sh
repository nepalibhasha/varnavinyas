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

BUILD_TIME_UTC="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
GIT_SHA="$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")"
cat > web/build-info.json <<EOF
{
  "built_at_utc": "${BUILD_TIME_UTC}",
  "git_sha": "${GIT_SHA}"
}
EOF

echo "Done. Serve with: python3 -m http.server 8080 --directory web/"
