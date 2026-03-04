# Backlog

## Near-Term Priorities (Next 4-8 Weeks)

### 1. Linguistic Core (Phase A Verification)
*   [ ] **Hrasva/Dirgha Rules**: Complete implementation of all 16 categories in Section 3(क).
*   [ ] **Gold Dataset Expansion**: Expand `gold.toml` reference pairs beyond the initial 91 entries to verify more edge cases.
*   [ ] **Sandhi Splitting**: Improve the brute-force split algorithm in `varnavinyas-sandhi`.
*   [ ] **Sandhi label localization in extension UI**: Replace internal enum labels like `VowelSandhi` with user-facing Nepali labels (e.g., `स्वर सन्धि`) in `extensions/browser/src/popup.js` / WASM adapter mapping.
*   [ ] **`अध्यन` misclassification**: `अध्यन` (misspelling) classified as `deshaj` / `correct: true`; should flag with correction `अध्ययन`.
    - Add `अध्यन` → `अध्ययन` to correction table (`prakriya/src/correction_table.rs`)
    - Add gold test in `docs/tests/gold.toml`
    - Add `ध्य` to tatsam markers in `shabda/src/origin.rs` (currently only has `द्य`)
    - Broader gap: unknown words (absent from lexicon + no correction rule) silently pass — `_in_lexicon` in `parikshak/src/checker.rs` is computed but unused

### 2. User-Facing Tools
*   [ ] **WASM**: Optimize `varnavinyas-wasm` bundle size (Target: < 2MB).
*   [ ] **Python**: Publish initial `varnavinyas` package to PyPI.

### 3. Documentation & Testing
*   [ ] **Migration**: Complete migration of old docs to new structure (Vision/Arch/Datasets).
*   [ ] **Fixture Audit**: Review `needs_review.toml` items and resolve top 5 ambiguities.

## Future / On Deck
*   **Browser Extension polish**: Side panel auto-update testing, Firefox QA pass, Chrome Web Store submission.
*   **Community Contribution**: Format for submitting new words via GitHub Issues.
