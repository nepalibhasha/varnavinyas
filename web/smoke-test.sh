#!/usr/bin/env bash
#
# Smoke test for the varnavinyas web app.
# Validates: WASM artifacts exist, JS/CSS category keys are consistent,
# .mark-hidden rule exists, and the local server can serve all assets.
#
set -euo pipefail

cd "$(dirname "$0")"

PASS=0
FAIL=0

pass() { ((PASS++)); echo "  PASS: $1"; }
fail() { ((FAIL++)); echo "  FAIL: $1" >&2; }
warn() { echo "  WARN: $1" >&2; }

echo "=== Varnavinyas Web Smoke Test ==="
echo ""

# --- 1. WASM artifacts exist ---
echo "[1] WASM build artifacts"
if [ -f pkg/varnavinyas_bindings_wasm.js ] && [ -f pkg/varnavinyas_bindings_wasm_bg.wasm ]; then
  pass "WASM JS and .wasm files present in web/pkg/"
else
  fail "WASM artifacts missing — run: bash web/build.sh"
fi

# --- 2. WASM JS exports the expected functions ---
echo "[2] WASM JS exports"
CORE_EXPORTS="check_text check_word transliterate derive"
TYPED_EXPORTS="check_text_value check_word_value derive_value analyze_word_value decompose_word_value sandhi_apply_value sandhi_split_value"

missing_exports() {
  local exports="$1"
  local missing=""
  for fn in $exports; do
    if ! grep -q "export function ${fn}" pkg/varnavinyas_bindings_wasm.js 2>/dev/null; then
      missing="$missing $fn"
    fi
  done
  echo "$missing"
}

# Always enforce core exports from built pkg.
for fn in $CORE_EXPORTS; do
  if grep -q "export function ${fn}" pkg/varnavinyas_bindings_wasm.js 2>/dev/null; then
    pass "export function ${fn} found"
  else
    fail "export function ${fn} missing from WASM JS"
  fi
done

# Typed exports are enforced in pkg only when rebuild succeeds.
ENFORCE_TYPED_PKG=1
MISSING_TYPED=$(missing_exports "$TYPED_EXPORTS")
if [ -n "${MISSING_TYPED// }" ]; then
  if command -v wasm-pack >/dev/null 2>&1; then
    echo "  INFO: typed exports missing in pkg; rebuilding web/pkg via web/build.sh"
    BUILD_LOG=$(mktemp)
    if ./build.sh >"$BUILD_LOG" 2>&1; then
      pass "Rebuilt web/pkg before export checks"
    else
      ENFORCE_TYPED_PKG=0
      warn "Failed to rebuild web/pkg via web/build.sh; skipping pkg typed-export enforcement"
      sed -n '1,40p' "$BUILD_LOG" >&2 || true
    fi
    rm -f "$BUILD_LOG"
  else
    ENFORCE_TYPED_PKG=0
    warn "wasm-pack unavailable; skipping pkg typed-export enforcement"
  fi
fi

if [ "$ENFORCE_TYPED_PKG" -eq 1 ]; then
  for fn in $TYPED_EXPORTS; do
    if grep -q "export function ${fn}" pkg/varnavinyas_bindings_wasm.js 2>/dev/null; then
      pass "export function ${fn} found"
    else
      fail "export function ${fn} missing from WASM JS"
    fi
  done
fi

# Always enforce typed exports at Rust source level.
for fn in $TYPED_EXPORTS; do
  if grep -q "pub fn ${fn}" ../crates/bindings-wasm/src/lib.rs 2>/dev/null; then
    pass "Rust export ${fn} found"
  else
    fail "Rust export ${fn} missing in bindings-wasm/src/lib.rs"
  fi
done

# --- 3. Category code consistency across Rust → JS → CSS ---
echo "[3] Category code consistency"
# Expected codes from Rust DiagnosticCategory enum (via Debug format)
EXPECTED_CODES="ShuddhaTable HrasvaDirgha Chandrabindu ShaShaS RiKri Halanta YaE KshaChhya Sandhi Punctuation"

for code in $EXPECTED_CODES; do
  # Check JS CATEGORY_COLORS has this key
  if grep -q "  ${code}:" js/utils.js 2>/dev/null; then
    pass "JS CATEGORY_COLORS has ${code}"
  else
    fail "JS CATEGORY_COLORS missing ${code}"
  fi

  # Check CSS has mark[data-category="..."] selector
  if grep -q "data-category=\"${code}\"" css/style.css 2>/dev/null; then
    pass "CSS data-category selector for ${code}"
  else
    fail "CSS data-category selector missing for ${code}"
  fi
done

# --- 4. .mark-hidden CSS rule exists ---
echo "[4] mark-hidden CSS rule"
if grep -q "\.mark-hidden" css/style.css 2>/dev/null; then
  pass ".mark-hidden rule exists in CSS"
else
  fail ".mark-hidden rule missing from CSS"
fi

# --- 5. checker.js uses category_code (not category) for keying ---
echo "[5] checker.js uses category_code"
if grep -q "d\.category_code" js/checker.js 2>/dev/null; then
  pass "checker.js references category_code"
else
  fail "checker.js does not reference category_code"
fi
# Ensure d.category is only used as label fallback (not for filtering/keying)
# Exclude comments (// and *) and the intentional "|| d.category" label fallback
BARE_CATEGORY_KEYING=$(grep -v '^\s*//' js/checker.js | grep -v '^\s*\*' | grep -v '|| d\.category' | grep -c 'd\.category\b' 2>/dev/null || true)
if [ "$BARE_CATEGORY_KEYING" -eq 0 ]; then
  pass "checker.js does not use bare d.category for filtering/keying"
else
  fail "checker.js uses bare d.category for filtering in $BARE_CATEGORY_KEYING places"
fi

# --- 6. Rust WASM bindings have category_code field ---
echo "[6] Rust category_code field"
if grep -q "category_code" ../crates/bindings-wasm/src/lib.rs 2>/dev/null; then
  pass "category_code field in JsDiagnostic struct"
else
  fail "category_code field missing from Rust WASM bindings"
fi

# --- 7. HTTP server test (quick start/stop) ---
echo "[7] HTTP serving"
PORT=18080
python3 -m http.server $PORT --directory . &>/dev/null &
SERVER_PID=$!
sleep 1

serve_check() {
  local path=$1
  local label=$2
  local status
  status=$(curl -s -o /dev/null -w "%{http_code}" "http://localhost:${PORT}/${path}" 2>/dev/null || echo "000")
  if [ "$status" = "200" ]; then
    pass "HTTP 200 for ${label}"
  else
    fail "HTTP ${status} for ${label} (expected 200)"
  fi
}

serve_check "index.html" "index.html"
serve_check "css/style.css" "style.css"
serve_check "js/app.js" "app.js"
serve_check "js/wasm-bridge.js" "wasm-bridge.js"
serve_check "js/checker.js" "checker.js"
serve_check "js/inspector.js" "inspector.js"
serve_check "js/reference.js" "reference.js"
serve_check "js/rules-data.js" "rules-data.js"
serve_check "pkg/varnavinyas_bindings_wasm.js" "WASM JS module"
serve_check "pkg/varnavinyas_bindings_wasm_bg.wasm" "WASM binary"

kill $SERVER_PID 2>/dev/null || true

# --- Summary ---
echo ""
TOTAL=$((PASS + FAIL))
echo "=== Results: ${PASS}/${TOTAL} passed, ${FAIL} failed ==="
[ "$FAIL" -eq 0 ] && exit 0 || exit 1
