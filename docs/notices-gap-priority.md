# Notices Rules Gap Priority (Pages 77-99)

Date: 2026-02-16

## P0 (fixed in this pass)

1. Section 5 निर्देशक (`: / - / :-`) mismatch
- Problem: checker forced `:` to `:-`, but spec allows all three forms.
- Fix: removed forced `:` -> `:-` diagnostic.
- Files: `crates/lekhya/src/punctuation.rs`, `crates/lekhya/tests/punctuation.rs`

2. Section 5 सङ्क्षेप over-flagging
- Problem: valid abbreviation chains like `अ. दु. अ. आ.` and `त्रि.वि.` could be flagged.
- Fix: abbreviation detection now supports chained/compact Devanagari abbreviation patterns while keeping sentence-ending `.` checks.
- Files: `crates/lekhya/src/punctuation.rs`, `crates/lekhya/tests/punctuation.rs`

## P1 (next highest value)

1. Section 3(ग)(ऊ): `ज्ञ/ग्याँ/ग्या` correction family
- Status: done in this pass.
- Added dedicated kosha-validated rule (`ortho-gya-gyan`) to handle common misspelling direction `ग्याँ/ग्या -> ज्ञा`.
- Added regressions for:
  - `अग्यान -> अज्ञान`
  - `प्रग्या -> प्रज्ञा`
  - keep valid loanword `ग्यारेज` unchanged.

2. Section 3(घ): पदयोग/पदवियोग baseline
- Status: baseline done in this pass.
- Added curated split->join phrase diagnostics with boundary-aware matching in `parikshak`.
- Currently covered high-frequency examples: `म सँग`, `आज्ञा अनुसार`, `तिमी भन्दा`, etc.
- Integrated into `check_text` pipeline with regression tests.

3. Section 5 punctuation coverage expansion
- Status: done for conservative baseline in this pass.
- Added conservative checks for:
  - तिर्यक् विराम spacing around विकल्प slash (`/`)
  - ऐजन comma-pair spacing normalization (`, ,` -> `,,`)
  - unmatched कोष्ठक balance sanity (`(` / `)`)
- Deep semantic/context checks for कोष्ठक usage remain pending.

## P2 (broader coverage)

1. Section 4 पदावली and वाक्य-level patterns
- Status: baseline implemented in this pass.
- Added opt-in Section 4 phrase/sentence suggestions as `Variant` diagnostics behind `CheckOptions { grammar: true }`.
- Default (`check_text`) remains unchanged to avoid false positives.
- Expanded with additional sentence-level patterns (style/word-order/caraka usage) from Section 4(ग).
- Includes complex long-sentence reorder suggestions from Section 4(ग) examples.

2. Section 3(ङ) हलन्त/अजन्त deeper subrules
- Status: done for current conservative scope.
- Added:
  - verb-form halanta suffix handling (`छस/छन/इस`) with kosha validation
  - ajanta-side terminal `...छ् -> ...छ` typo correction with kosha validation
- Future enhancement: broader imperative/subtype exception tables.

## Verification run for P0

- `cargo test -p varnavinyas-lekhya -q`
- `cargo test -p varnavinyas-parikshak -q`
- Result: all tests passed.
