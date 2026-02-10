# Phases 3-4: Ecosystem & Refinement

**Scope**: Editor integration, platform bindings, community tooling, and long-term sustainability.

---

## Phase 3: Ecosystem & Bindings (Months 11-15)

### LSP Server
- Implement using `tower-lsp` crate
- Real-time diagnostics as the user types
- Hover information: rule explanation and Academy citation
- Code actions: quick-fix to apply corrections
- Document-wide diagnostics on save
- Configuration: enable/disable specific rule categories

### VS Code Extension
- Language client wrapping the LSP server
- Extension published to VS Code Marketplace
- Activate for `.np`, `.txt` files with Devanagari content
- Settings UI for rule category toggles
- Status bar showing error count

### CLI Tool
- Built with `clap` crate
- Commands:
  - `varnavinyas check <file|text>` — spell check
  - `varnavinyas check --explain` — spell check with rule explanations
  - `varnavinyas check --format=json` — structured output for tooling
  - `varnavinyas akshar analyze <text>` — character analysis
  - `varnavinyas lipi convert <text> --from <scheme> --to <scheme>` — transliteration
- Exit code: 0 = clean, 1 = errors found
- Supports stdin pipe for integration with other tools

### UniFFI Bindings (Kotlin/Swift)
- Generate Kotlin bindings for Android apps
- Generate Swift bindings for iOS apps
- Expose: `checkText()`, `transliterate()`, `classify()`
- Package as Android AAR and iOS framework

### C ABI (cbindgen)
- Generate C header via `cbindgen`
- Expose core functions: check, transliterate, classify
- Memory management: caller frees returned strings
- Opaque handle pattern for stateful objects

### Web Application
- Static site (no server required)
- WASM-powered, runs entirely in the browser
- Text input area with real-time error highlighting
- Educational mode: click an error to see the rule explanation
- Deployable to GitHub Pages or any static host

---

## Phase 4: Refinement & Community (Months 15-18)

### Performance Optimization
- Profile with `cargo flamegraph`
- Optimize hot paths: kosha lookup, rule matching
- Benchmark: 1000-word document check < 50ms
- WASM bundle size audit: target < 5MB gzipped
- Memory usage profiling for large documents

### Lexicon Expansion
- Community-contributed word lists
- Build pipeline for adding new words to FST
- Versioned lexicon releases
- Quality gate: every new entry needs a source citation

### Dialectal Variations
- Support for regional Nepali spelling preferences
- Configuration for strict (Academy) vs. tolerant mode
- Track common acceptable alternatives (e.g., संचालन vs सञ्चालन)

### Documentation
- API documentation in Nepali and English
- Tutorial: "Building a Nepali spell checker" walkthrough
- Guide: "How to encode a new orthography rule"
- Contributor guide: architecture overview, testing conventions

### Community Governance
- Linguistic advisory group for rule validation
- Contribution guidelines with citation requirements
- Regular community calls
- Issue templates for rule disputes, word additions, bug reports

### Future Scope (Beyond Phase 4)
- Grammar checking beyond orthography
- Style suggestions and readability analysis
- Input method support (keyboard integration)
- Screen reader pronunciation verification
- Nepali text-to-speech integration hooks

---

### Previous: [Phase 2 — Lexicon & Spell Checker](PHASE_2.md)
