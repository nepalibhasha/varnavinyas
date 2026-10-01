# Compound-analysis contract

Compound interpretations require reviewed formation evidence. Finding two
attested words inside another word, or reproducing its spelling with sandhi,
does not establish its grammatical structure. The engine suppresses unsupported
samasa hints for forms such as सवारीमा, दशकमा, आयात, आर्थिक, विकास, यातायात,
व्यवस्थापन and रिसाइकल. Their spelling remains accepted.

The reviewed inventory preserves सूर्योदय, महोत्सव, एकचक्र (the numeral
reading), पूर्वाधार and मापदण्ड. The महोत्सव member is महा, not मह; पूर्वाधार
uses the adjectival पहिलेको sense and explains पहिलेको आवश्यक आधार. Additional
words require sourced review, not a blanket noun/numeral/POS fallback.

Outer inflections are analyzed separately. With grammar enabled, सूर्योदयमा
reports सूर्य + उदय + मा on the original word's span. The explanation identifies
मा as an outer suffix; it is not part of the compound member उदय.

## Consumer behavior

- `samasa-heuristic` remains informational, with kind `Variant` and category code
  `Sandhi`. Never apply its `correction` string to the document, individually or
  in bulk; this field carries a structural analysis for compatibility.
- Label the analysis as बनोट, rather than rendering a correction arrow. Ranking
  weights in `confidence`/compound `score` are not calibrated probabilities and
  should not be displayed as accuracy percentages.
- An empty `analyze_compound` result means no supported analysis, not an error.
  Raw sandhi APIs remain available for exploratory analysis; an `Authoritative`
  spelling score does not establish a samasa type or semantic vigraha.
- Prefix formation (such as वि in विकास) and suffix formation (अर्थ + इक with
  आदिवृद्धि in आर्थिक) must not be relabeled as a generic तत्पुरुष interpretation.

Function signatures, diagnostic fields, category/rule codes, UTF-8 byte spans,
Python enum cases and orthography-mode defaults remain unchanged. Both modes
use the same compound evidence policy. Grammar-disabled checks remain unchanged.

## Artifact integration

These engine changes require rebuilt browser, Python, CLI, iOS and Android
packages from the same source commit. A web-only rebuild using an older WASM
artifact will retain the old engine behavior. Publish new immutable versions;
do not replace existing release tags/assets. Upgrade offline mobile libraries
and their matching generated bindings together, and refresh manifests/checksums
and the shared fixtures. The diagnostic fixtures now include 28 cases, including
unsupported compounds and an inflected compound with an emoji before its span.

The Varnavinyas web presentation also changes; downstream UIs should follow the
informational rendering policy above. Updating WASM removes the spurious engine
hints but does not automatically change consumer-owned card renderers.
