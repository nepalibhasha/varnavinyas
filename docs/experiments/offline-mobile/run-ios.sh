#!/usr/bin/env bash
set -euo pipefail
PACKAGE="${1:?path to extracted varnavinyas-ios-artifact-v0.1.2}"
DEVICE="${2:?booted iOS simulator UDID}"
PROBE="${3:-Probe}"
case "$PROBE" in Probe|SnapshotRegression) ;; *) exit 2 ;; esac
HERE="$(cd "$(dirname "$0")" && pwd)"
WORK="$(mktemp -d /tmp/varnavinyas-ios-probe.XXXXXX)"
SLICE="$PACKAGE/VarnavinyasBindingsUniFFI.xcframework/ios-arm64_x86_64-simulator"
sed 's/@main struct Evaluation/enum Evaluation/' "$PACKAGE/evaluation/evaluate.swift" > "$WORK/Fixtures.swift"
xcrun --sdk iphonesimulator swiftc -O "$PACKAGE/bindings/varnavinyas_bindings_uniffi.swift" \
    "$WORK/Fixtures.swift" "$HERE/OfflineSession.swift" "$HERE/$PROBE.swift" "$SLICE/simulator.a" \
    -target "$(uname -m)-apple-ios13.0-simulator" -sdk "$(xcrun --sdk iphonesimulator --show-sdk-path)" \
    -I "$SLICE/Headers" -module-cache-path "$WORK/module-cache" -o "$WORK/probe"
xcrun simctl spawn "$DEVICE" "$WORK/probe" "$PACKAGE/fixtures/diagnostics.json" "$PACKAGE/fixtures/origins.json"
