#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
VERSION="${1:?release version required}"
[[ "$VERSION" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]
export IPHONEOS_DEPLOYMENT_TARGET=13.0
WORK=dist/ios-build
STAGING="dist/varnavinyas-ios-artifact-${VERSION}"
mkdir -p "$WORK/bindings"
cargo build --release --locked -p varnavinyas-bindings-uniffi
cargo run --release --locked -p varnavinyas-bindings-uniffi --bin uniffi-bindgen -- generate \
  --library target/release/libvarnavinyas_bindings_uniffi.dylib --language swift --out-dir "$WORK/bindings"
swiftc "$WORK/bindings/varnavinyas_bindings_uniffi.swift" release/evaluate.swift \
  -module-cache-path "$WORK/module-cache" \
  -Xcc "-fmodule-map-file=$WORK/bindings/varnavinyas_bindings_uniffiFFI.modulemap" \
  -I "$WORK/bindings" -L target/release -lvarnavinyas_bindings_uniffi \
  -Xlinker -rpath -Xlinker "$PWD/target/release" -o "$WORK/evaluate"
"$WORK/evaluate" docs/tests/mobile_diagnostics.json docs/tests/origin_classification.json
for target in aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios; do
  cargo build --release --locked --target "$target" -p varnavinyas-bindings-uniffi --lib
done
swiftc -typecheck "$WORK/bindings/varnavinyas_bindings_uniffi.swift" \
  -module-cache-path "$WORK/module-cache" \
  -target arm64-apple-ios13.0-simulator -sdk "$(xcrun --sdk iphonesimulator --show-sdk-path)" \
  -Xcc "-fmodule-map-file=$WORK/bindings/varnavinyas_bindings_uniffiFFI.modulemap" -I "$WORK/bindings"
lipo -create target/aarch64-apple-ios-sim/release/libvarnavinyas_bindings_uniffi.a \
  target/x86_64-apple-ios/release/libvarnavinyas_bindings_uniffi.a -output "$WORK/simulator.a"
mkdir -p "$WORK/headers"
cp "$WORK/bindings/varnavinyas_bindings_uniffiFFI.h" "$WORK/headers/"
cp "$WORK/bindings/varnavinyas_bindings_uniffiFFI.modulemap" "$WORK/headers/module.modulemap"
rm -rf "$STAGING"
mkdir -p "$STAGING/evaluation"
xcodebuild -create-xcframework \
  -library "$PWD/target/aarch64-apple-ios/release/libvarnavinyas_bindings_uniffi.a" -headers "$PWD/$WORK/headers" \
  -library "$PWD/$WORK/simulator.a" -headers "$PWD/$WORK/headers" \
  -output "$PWD/$STAGING/VarnavinyasBindingsUniFFI.xcframework"
cp -R "$WORK/bindings" "$STAGING/bindings"
cp release/evaluate.swift "$STAGING/evaluation/"
node release/package-metadata.mjs ios "$VERSION" "$STAGING"
node release/verify-package.mjs "$STAGING"
(cd dist && zip -qr "varnavinyas-ios-artifact-${VERSION}.zip" "varnavinyas-ios-artifact-${VERSION}")
(cd dist && shasum -a 256 "varnavinyas-ios-artifact-${VERSION}.zip" > "varnavinyas-ios-artifact-${VERSION}.zip.sha256")
