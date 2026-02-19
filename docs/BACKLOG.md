# Backlog

## Near-Term Priorities (Next 4-8 Weeks)

### 1. Linguistic Core (Phase A Verification)
*   [ ] **Hrasva/Dirgha Rules**: Complete implementation of all 16 categories in Section 3(क).
*   [ ] **Gold Dataset Expansion**: Expand `gold.toml` reference pairs beyond the initial 91 entries to verify more edge cases.
*   [ ] **Sandhi Splitting**: Improve the brute-force split algorithm in `varnavinyas-sandhi`.

### 2. User-Facing Tools
*   [ ] **WASM**: Optimize `varnavinyas-wasm` bundle size (Target: < 2MB).
*   [ ] **Python**: Publish initial `varnavinyas` package to PyPI.

### 3. Documentation & Testing
*   [ ] **Migration**: Complete migration of old docs to new structure (Vision/Arch/Datasets).
*   [ ] **Fixture Audit**: Review `needs_review.toml` items and resolve top 5 ambiguities.

## Future / On Deck
*   **Browser Extension**: WebExtension wrapper around WASM module.
*   **Community Contribution**: Format for submitting new words via GitHub Issues.
