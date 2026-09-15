#!/usr/bin/env bash
set -euo pipefail

# Run against an installed wheel; PYTHON can select an isolated test environment.
"${PYTHON:-python3}" - <<'PY'
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
print("Python wheel import and API smoke checks passed.")
PY
