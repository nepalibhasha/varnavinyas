# Browser artifact v0.1.6

This release uses the same source commit as Python, CLI, iOS and Android v0.1.3.
It preserves the existing WASM exports, diagnostic schema and orthography-mode
options while updating the engine's conservative lexical, suffix and spelling
rules. See the matching [native release notes](https://github.com/nepalibhasha/varnavinyas/blob/browser-artifact-v0.1.6/docs/releases/native-v0.1.3.md) for behavior
changes and examples.

For example, `पत्रहरूलाई लेखि पठाइन्` now offers `लेखी` conditionally, including
reviewed predicate inflections and noun suffixes. The diagnostic is `Ambiguous`,
available even with grammar heuristics disabled, and requires individual review.
Keep it out of bulk correction; display its explanation rather than presenting
the possible reading as a definite error. Known short readings remain valid.
Both orthography modes retain their existing defaults and semantics.

Consumers should update the artifact pin from 0.1.5 to 0.1.6, rebuild their WASM
and JavaScript bundles, and regenerate precomputed diagnostic samples. Check the
manifest's `git_sha` against native packages' `source_commit`. Diagnostic offsets
still refer to UTF-8 bytes in the original text; test client conversion with
emoji, shaping variants and repeated words. The browser runtime also passes the
24 shared native diagnostic fixtures and preserves unknown-origin presentation.
