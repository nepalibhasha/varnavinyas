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

echo "Building WASM bindings..."
wasm-pack build crates/bindings-wasm \
  --target web \
  --out-dir ../../web/pkg \
  --release

echo "Done. Serve with: python3 -m http.server 8080 --directory web/"
