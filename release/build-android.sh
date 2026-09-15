#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
VERSION="${1:?release version required}"
[[ "$VERSION" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]
WORK=dist/android-build
STAGING="dist/varnavinyas-android-artifact-${VERSION}"
mkdir -p "$WORK/bindings" "$WORK/tools"
cargo build --release --locked -p varnavinyas-bindings-uniffi
cargo run --release --locked -p varnavinyas-bindings-uniffi --bin uniffi-bindgen -- generate \
  --library target/release/libvarnavinyas_bindings_uniffi.so --language kotlin --out-dir "$WORK/bindings"
# Host JVM evaluation exercises the generated bindings against the shared fixtures.
curl -fL --retry 3 https://github.com/JetBrains/kotlin/releases/download/v1.9.24/kotlin-compiler-1.9.24.zip -o "$WORK/tools/kotlin.zip"
unzip -qo "$WORK/tools/kotlin.zip" -d "$WORK/tools"
curl -fL --retry 3 https://repo.maven.apache.org/maven2/net/java/dev/jna/jna/5.13.0/jna-5.13.0.jar -o "$WORK/tools/jna.jar"
curl -fL --retry 3 https://repo.maven.apache.org/maven2/org/json/json/20240303/json-20240303.jar -o "$WORK/tools/json.jar"
"$WORK/tools/kotlinc/bin/kotlinc" "$WORK/bindings/uniffi/varnavinyas_bindings_uniffi/varnavinyas_bindings_uniffi.kt" \
  release/Evaluate.kt -cp "$WORK/tools/jna.jar:$WORK/tools/json.jar" -include-runtime -d "$WORK/evaluate.jar"
java -Djna.library.path="$PWD/target/release" -cp "$WORK/evaluate.jar:$WORK/tools/jna.jar:$WORK/tools/json.jar" \
  EvaluateKt docs/tests/mobile_diagnostics.json
export ANDROID_NDK_HOME="${ANDROID_NDK_LATEST_HOME:-${ANDROID_NDK_HOME:-}}"
test -d "$ANDROID_NDK_HOME"
cargo ndk --target aarch64-linux-android --target armv7-linux-androideabi --target x86_64-linux-android \
  --platform 21 -- build --release --locked -p varnavinyas-bindings-uniffi --lib
rm -rf "$STAGING"
mkdir -p "$STAGING/evaluation"
for pair in 'aarch64-linux-android arm64-v8a' 'armv7-linux-androideabi armeabi-v7a' 'x86_64-linux-android x86_64'; do
  read -r target abi <<< "$pair"
  mkdir -p "$STAGING/jniLibs/$abi"
  cp "target/$target/release/libvarnavinyas_bindings_uniffi.so" "$STAGING/jniLibs/$abi/"
done
cp -R "$WORK/bindings" "$STAGING/bindings"
cp release/Evaluate.kt "$STAGING/evaluation/"
node release/package-metadata.mjs android "$VERSION" "$STAGING"
node release/verify-package.mjs "$STAGING"
(cd dist && zip -qr "varnavinyas-android-artifact-${VERSION}.zip" "varnavinyas-android-artifact-${VERSION}")
(cd dist && shasum -a 256 "varnavinyas-android-artifact-${VERSION}.zip" > "varnavinyas-android-artifact-${VERSION}.zip.sha256")
