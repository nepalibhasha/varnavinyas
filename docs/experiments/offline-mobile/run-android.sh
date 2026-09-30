#!/usr/bin/env bash
set -euo pipefail
PACKAGE="${1:?path to extracted varnavinyas-android-artifact-v0.1.2}"
TOOLS="${2:?directory containing kotlinc and extracted JNA 5.13.0 AAR in jna/}"
SDK="${3:?Android SDK root}"
SERIAL="${4:?booted Android emulator serial}"
HERE="$(cd "$(dirname "$0")" && pwd)"
WORK="$(mktemp -d /tmp/varnavinyas-android-probe.XXXXXX)"
REMOTE=/data/local/tmp/varnavinyas-offline-probe
ADB="$SDK/platform-tools/adb"
ABI="$($ADB -s "$SERIAL" shell getprop ro.product.cpu.abi | tr -d '\r')"
sed 's/fun main(args:/fun evaluateFixtures(args:/' "$HERE/../../../release/Evaluate.kt" > "$WORK/Fixtures.kt"
"$TOOLS/kotlinc/bin/kotlinc" "$PACKAGE/bindings/uniffi/varnavinyas_bindings_uniffi/varnavinyas_bindings_uniffi.kt" \
    "$WORK/Fixtures.kt" "$HERE/OfflineSession.kt" "$HERE/Probe.kt" \
    -cp "$TOOLS/jna/classes.jar:$SDK/platforms/android-36/android.jar" -include-runtime -d "$WORK/probe.jar"
mkdir "$WORK/dex"
"$SDK/build-tools/36.0.0/d8" --min-api 21 --lib "$SDK/platforms/android-36/android.jar" \
    --output "$WORK/dex" "$WORK/probe.jar" "$TOOLS/jna/classes.jar"
"$ADB" -s "$SERIAL" shell mkdir -p "$REMOTE"
"$ADB" -s "$SERIAL" push "$WORK/dex/classes.dex" "$PACKAGE/jniLibs/$ABI/libvarnavinyas_bindings_uniffi.so" \
    "$TOOLS/jna/jni/$ABI/libjnidispatch.so" "$PACKAGE/fixtures/diagnostics.json" "$PACKAGE/fixtures/origins.json" "$REMOTE/"
"$ADB" -s "$SERIAL" shell "CLASSPATH=$REMOTE/classes.dex app_process -Djna.boot.library.path=$REMOTE -Djna.library.path=$REMOTE /system/bin ProbeKt $REMOTE/diagnostics.json $REMOTE/origins.json"
