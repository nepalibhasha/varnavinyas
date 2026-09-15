#!/usr/bin/env bash
set -euo pipefail

# Run against an installed wheel; PYTHON can select an isolated test environment.
FIXTURE_PATH="$(cd "$(dirname "$0")/../.." && pwd)/docs/tests/mobile_diagnostics.json"
"${PYTHON:-python3}" - "$FIXTURE_PATH" <<'PY'
import json
import math
import sys
import varnavinyas as v

for name in ("akshar", "lipi", "shabda", "sandhi", "prakriya", "kosha", "lekhya", "parikshak"):
    assert getattr(v, name), name

# Enum arguments exercise PyO3's explicit FromPyObject compatibility opt-in.
scheme = v.lipi.Scheme
assert v.lipi.transliterate("नेपाल", scheme.Devanagari, scheme.Iast) == "nepāla"
assert v.lipi.detect_scheme("नेपाल") == scheme.Devanagari
assert v.akshar.is_svar("अ")
assert v.kosha.lookup("हामी").word == "हामी"
assert v.kosha.lookup("कखगघङ") is None

# Nested results, Unicode strings, optional returns, and policy arguments.
diag = v.parikshak.check_word("हामि")
assert diag.correction == "हामी"
assert diag.rule.code == diag.rule_code == "3(क)(ऊ)-7"
assert diag.alternate_reasons == []
text_diag = next(d for d in v.check_text("हामि") if d.incorrect == "हामि")
assert (text_diag.span_start, text_diag.span_end) == (0, len("हामि".encode()))
variant = next(d for d in v.check_text_with_options("संघीय", orthography_mode="common_editorial")
               if d.incorrect == "संघीय")
assert variant.kind == "Variant"
for check in (v.check_text_with_options, v.parikshak.check_text_with_options):
    # v0.1.0's fourth positional argument must stay include_noop_heuristics.
    legacy = next(d for d in check("संघीय", False, "strict", False) if d.incorrect == "संघीय")
    assert legacy.kind == "Error"
    editorial = next(d for d in check("संघीय", False, "strict", False,
                                     orthography_mode="common-editorial") if d.incorrect == "संघीय")
    assert editorial.kind == "Variant"
try:
    v.check_text_with_options("हामी", orthography_mode="invalid")
except ValueError:
    pass
else:
    raise AssertionError("invalid policy must raise ValueError")

for word in ("हेर", "हेर्"):
    assert v.prakriya.derive(word).output == word
assert v.shabda.decompose("संघीय").root != "घीय"
assert "ई" not in v.shabda.decompose("हामी").suffixes
assert v.shabda.decompose("तिनी").root == "तिन"
assert v.sandhi.apply("अति", "अधिक").output == "अत्यधिक"
with open(sys.argv[1], encoding="utf-8") as fixture_file:
    cases = json.load(fixture_file)["cases"]
for case in cases:
    actual = v.check_text_with_options(case["text"], **case["options"])
    expected = case["expected_diagnostics"]
    assert len(actual) == len(expected), case["id"]
    for found, wanted in zip(actual, expected):
        for key in ("span_start", "span_end", "incorrect", "correction", "rule_code",
                    "category_code", "kind", "explanation"):
            assert getattr(found, key) == wanted[key], (case["id"], key)
        assert math.isclose(found.confidence, wanted["confidence"], abs_tol=1e-6)
        assert [reason.rule_code for reason in found.alternate_reasons] == [
            reason["rule_code"] for reason in wanted.get("alternate_reasons", [])]
print(f"Python: {len(cases)} shared diagnostic fixtures passed.")
print("Python wheel import and API smoke checks passed.")
PY
