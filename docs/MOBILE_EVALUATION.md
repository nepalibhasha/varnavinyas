# Offline Mobile Evaluation

iOS and Android evaluation ZIPs in the v0.1.2 release set come from the same
Git commit as the Python and CLI releases. Compare `source_commit` and
`fixtures.sha256` and `origin_fixtures.sha256` in each `manifest.json` before evaluating. Every payload file
has a SHA-256 checksum in the manifest. The manifest is not a signature.

The dictionary and rules are compiled into each native library. Checking text
requires no server, network connection, API key, or downloaded lexicon. Build
dependencies such as Kotlin/JNA must be resolved when integrating the package;
they are not fetched by the checker at runtime.

## iOS

The ZIP contains `VarnavinyasBindingsUniFFI.xcframework` with arm64 device and
arm64/x86_64 simulator libraries, their C headers and module maps, plus
`bindings/varnavinyas_bindings_uniffi.swift`. Minimum deployment target: iOS 13.
Add the static XCFramework to the Xcode target and compile the Swift source in
the same application/module as the evaluation source. Do not embed the static
framework as a dynamic framework. `bindings/` also carries the original
generated C header and module map for integrations that configure search paths
directly. Use the generated bindings and native library from the same ZIP.

```swift
let json = checkTextWithAllOptions(text: "संघीय", grammar: false,
    punctuationMode: .strict, orthographyMode: .commonEditorial,
    includeNoopHeuristics: false)
```

## Android

Copy `jniLibs/` into the Android module's `src/main/jniLibs/` and generated
`bindings/uniffi/` sources into its Kotlin source set. The package contains
arm64-v8a, armeabi-v7a, and x86_64 libraries built for API 21 or later. Add:

```kotlin
implementation("net.java.dev.jna:jna:5.13.0@aar")
```

Use Kotlin 1.9 or later. The namespace is `uniffi.varnavinyas_bindings_uniffi`.
The library loads through JNA; keep generated binding classes and JNA classes
when enabling R8/ProGuard (for example `-keep class uniffi.** { *; }` and
`-keep class com.sun.jna.** { *; }`).

```kotlin
val json = checkTextWithAllOptions("संघीय", false, PunctuationMode.STRICT,
    OrthographyMode.COMMON_EDITORIAL, false)
```

## API and Diagnostic Contract

`checkText` and the older `checkTextWithOptions` keep Academy-strict orthography.
`checkTextWithAllOptions` adds explicit orthography selection. Both platforms
return JSON diagnostic strings with the same fields and semantics:

- `academy-strict`: reviewed standard spellings are `Error` diagnostics.
- `common-editorial`: reviewed alternatives such as संघीय become `Variant`,
  while ordinary mistakes such as हामि remain `Error`.
- `correction` remains the Academy spelling in either mode. A `Variant` is not
  automatically a mandatory replacement.
- `span_start`/`span_end` are zero-based, half-open **UTF-8 byte offsets** in the
  original text. Swift String and Kotlin UTF-16 indices require conversion.
- `rule_code` and `category_code` are stable identifiers. Display labels and
  the number of alternate reasons can change. Empty `alternate_reasons` is
  omitted. Ignore unfamiliar fields for forward compatibility.
- `grammar` enables heuristic diagnostics; it is off by default. In evaluation
  and production, make the option explicit rather than silently changing it.
- `classify` preserves its four-way compatibility enum. Its fallback Deshaj
  does not prove origin. Use `classifyWithProvenance` for explanations.

```swift
let decision = classifyWithProvenance(word: "नेपाले")
assert(decision.origin == nil && decision.source == .unknown)
```

```kotlin
val decision = classifyWithProvenance("नेपाले")
check(decision.origin == null && decision.source == OriginSource.UNKNOWN)
```

The `OriginDecision` record exposes nullable `origin`, `source`, and
`confidence`. Source Override/Kosha means documented evidence; Heuristic means
inferred; Unknown means no evidence, absent origin, and zero confidence.
Scores are not calibrated probabilities. A documented Deshaj result remains
distinct from unknown. Existing `Origin` enum cases and `classify` behavior
are unchanged. Compile the generated bindings with the matching native library.

## Shared Fixtures and Evaluation

Both ZIPs contain the identical `fixtures/diagnostics.json`, including options
and full expected diagnostics. Cases cover both modes, clean and incorrect
pronouns, ambiguous verbs, multiline UTF-8 offsets, alternate reasons, grammar,
and punctuation. Treat array order as significant and JSON object-key order as
irrelevant; compare numeric confidence with a small floating-point tolerance
if the consumer reserializes it.

`evaluation/evaluate.swift` and `evaluation/Evaluate.kt` run these fixtures
through the generated bindings, followed by the eight origin cases in
`fixtures/origins.json`. Both command-line harnesses take diagnostic and origin
fixture paths as their first and second arguments. Kotlin's JVM harness uses `org.json` (available
on Android, supplied separately for the host JVM). In an app test target, adapt
the harness to read the bundled fixture resource rather than command-line args.

Release workflows execute the fixtures through generated Swift/Kotlin bindings
against the host native library. iOS additionally type-checks the generated
Swift for the simulator SDK. These checks establish binding/JSON parity;
device-specific loading and your app's indexing/UI still need integration
testing. Both archives are evaluation packages, not App Store or Play Store apps.

The fixture source is `docs/tests/mobile_diagnostics.json`. To intentionally
refresh it, run `cargo run -p varnavinyas-bindings-uniffi --example
export_evaluation_fixtures`, review the JSON diff, and run the
`evaluation_fixtures` integration test. Never regenerate expected results
silently during the release build.
