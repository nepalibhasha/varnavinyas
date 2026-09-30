# Offline mobile integration experiment — 2026-09-28

This is a native checker and correction-adapter experiment, not a shipped
Sabdasakha feature. The mobile applications still call the backend for
spellcheck. No app project, version, deployment, or published artifact was
changed by this experiment.

## Packages and method

Downloaded the published iOS and Android v0.1.2 evaluation packages. Both
manifests identify `bfdc8cd0605b4a2c6dd0a1071bfa28a9d4b55875`. Verified every
manifest payload's size and SHA-256 using `release/verify-package.mjs`.

The iOS probe links the published static simulator slice and compiles the
published Swift bindings. It runs under iOS 18.5 on an x86_64 iPhone 16 Pro
simulator. The Android probe compiles the published Kotlin bindings with Kotlin
1.9.24 and JNA 5.13.0, dexes them, and executes via `app_process` on the API 36
x86_64 emulator. It loads the published `.so` and the Android JNA dispatch library.
Android airplane mode is enabled, Wi-Fi disabled, and `ip route` is empty.
The iOS harness makes no network requests; the simulator's network was not
blocked. Neither harness uses the Sabdasakha backend or downloads runtime data.

These are native runtime checks, not full SwiftUI/Compose UI tests. Physical
arm64 devices, R8 builds, app lifecycle, and store-package size remain untested.

## Adapter behavior

`OfflineSession.swift` and `OfflineSession.kt` are matching prototypes that:

- Retain the original diagnostic JSON, including explanations and extra fields.
- Apply only `Error` diagnostics; leave `Variant`, `Ambiguous`, warnings, and
  samasa analyses unchanged during Fix all.
- Apply UTF-8 byte spans rather than confusing them with Swift grapheme or
  Kotlin UTF-16 indices. Validate the source text and ranges first.
- Reject the entire correction operation if text or revision changed, a source
  span mismatches, ranges overlap, or the output is invalid UTF-8.

Swift compares the snapshot's exact UTF-8 bytes, since Swift String equality
can consider differently encoded canonical equivalents equal. The additional
`SnapshotRegression.swift` check passes against the actual adapter on the host
Swift runtime; it can also be run on the simulator with the script's optional
third argument `SnapshotRegression`.

The eight adapter checks cover emoji/multiline/repeated words, stale text,
changed revision, strict orthography, informational grammar, comma spacing,
mismatched ranges, and unknown origin without a lexical verdict. The shared
16 diagnostic and eight origin fixtures run through the generated bindings too.

Increment the revision on **every input or checker-option change**, including
undo, paste, clear, orthography-mode selection, and grammar toggles. Run the
synchronous native call off the UI thread, debounce edits, and accept only the
current revision's result. Use one background worker and coalesce pending
requests to the latest edit, rather than starting overlapping native calls.
Cancellation may discard a result; it does not
interrupt an already-running native call. Clear stale review actions immediately.

The prototype chooses common-editorial to match the website. Existing mobile
server calls currently inherit Academy-strict. A production integration should
make mode selection explicit and preserve the existing default unless the
product deliberately changes it. Both modes pass the shared fixtures.

## Portability fix found by the experiment

The v0.1.2 Kotlin evaluation harness calls `JSONArray.similar()`. That method
is available in the host JVM `org.json` dependency, but absent from Android's
platform `org.json`, so the original harness does not compile directly against
the Android SDK.

`release/Evaluate.kt` now compares objects and arrays recursively with confidence
tolerance, object-key order independence, and array-order significance. Three
comparison regressions check numeric equivalence, array order, and extra keys.
The Android probe uses this updated harness with the unchanged published
bindings and native library. Existing release ZIPs have not been replaced;
`docs/MOBILE_EVALUATION.md` documents the workaround.

## Offline product boundary

Use the native engine as the local provider for orthography, punctuation, and
optional grammar hints. Compute local capability flags from the bundled engine;
server `/health` should describe optional server features, not control whether
local explanations, categories, or grammar are usable when disconnected.

An empty diagnostic list is **not dictionary membership**. The UniFFI API does
not expose a word-existence check, and unknown origin is not equivalent to an
unknown dictionary word. Keep dictionary status explicitly “not checked” until
a dictionary provider has checked it. Suggested wording for the local-only
state: “वर्णविन्यासको त्रुटि भेटिएन। शब्दकोश जाँच भएको छैन।”

Keep definition lookup and Roman search behind a separate local dictionary
provider. Build on Sabdasakha's existing `docs/design/offline-mobile-assessment.md`
and `docs/design/offline-roman-experiment.md`: those evaluated Brihat/Pragya packs
and a compact Roman index. They still need native app integration, response
normalization, source/variant handling, and atomic update tests. Optional network
lookups can enrich local results; lack of connectivity should not fail the local
checker. Retain source, nullable origin, and confidence when explaining provenance.

The existing mobile response models contain a deduplicated word list without
spans. Do not squeeze punctuation or occurrence-specific native diagnostics
into that contract. Keep a separate occurrence-based local result model; adapt
the review list while preserving the raw diagnostics. Both existing apps already
distinguish optional/ambiguous results from mandatory fixes.

## Reproduction

Download and extract the v0.1.2 evaluation ZIPs, verify their manifests, and boot
a disposable simulator/emulator. The scripts assume the current published
package layouts. Keep generated bindings and native libraries from the same ZIP.

```bash
bash docs/experiments/offline-mobile/run-ios.sh /path/to/ios-package SIMULATOR_UDID
bash docs/experiments/offline-mobile/run-android.sh /path/to/android-package /path/to/tools /path/to/android-sdk EMULATOR_SERIAL
```

The Android tools directory needs `kotlinc/` (1.9.24) and an extracted
`jna/` (5.13.0 AAR with `classes.jar` and `jni/`). The Android script uses SDK
platform/build-tools 36. It does not turn off connectivity; enable airplane
mode and disable Wi-Fi before running. The iOS script builds an optimized
probe for the host simulator architecture. Temporary build output goes under
`/tmp`; Android probe files go under `/data/local/tmp/varnavinyas-offline-probe`.

Runtime results and preliminary timing measurements are in
[results.json](results.json). Timings include binding conversion and JSON
parsing, use 20 warm samples, report the upper median and the nearest-rank p95,
and run on a busy shared development Mac. They are feasibility measurements,
not physical-device performance targets. `cold_check_ms` includes first native
load/initialization and the first adapter check, but excludes process launch.

Android ELF sizes (decimal MB) are 10.23 arm64, 9.69 armv7, and 10.21 x86_64.
Their gzip-6 sizes are 2.47, 2.39, and 2.48 MB respectively. These describe library
files, not delivered APK/AAB sizes. Per-user delivery normally includes one ABI;
the multi-ABI evaluation ZIP is not the app download size.

Next, wire this adapter behind a development flag into each app's provider,
keeping dictionary verification separate, then exercise first launch offline,
rapid edits and undo, mode changes, background/resume, and real low-memory phones
with the dictionary packs installed. Measure performance in release builds
before choosing debounce and document-length limits.
