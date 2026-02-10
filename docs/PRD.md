# Varnavinyas PRD — Product Requirements Document

**Status**: Master implementation guide
**Audience**: Solo developer (Rust beginner)
**Last updated**: 2026-02-10

---

## 1. Document Map

This PRD suite is the **master implementation blueprint** for Varnavinyas. It translates the project vision into concrete, buildable specifications.

### Document Relationships

| Document | Role | Status |
|----------|------|--------|
| **PRD.md** (this file) | Master blueprint — architecture, dependencies, testing, CI/CD | Authoritative |
| [VISION.md](VISION.md) | Philosophy, motivation, design principles, use cases | Reference (read-only) |
| [ROADMAP.md](ROADMAP.md) | Demo-driven roadmap, verified test data (gold.toml) | Reference (test data) |
| [RUST_GUIDE.md](RUST_GUIDE.md) | Rust onboarding companion for beginners | Companion |

### Phase Files (graduated detail)

| Phase | File | Scope | Detail Level |
|-------|------|-------|-------------|
| 0 | [phases/PHASE_0.md](phases/PHASE_0.md) | akshar, lipi, CI/CD | Full specs + acceptance criteria |
| 1 | [phases/PHASE_1.md](phases/PHASE_1.md) | shabda, sandhi, prakriya, Python | Detailed requirements |
| 2 | [phases/PHASE_2.md](phases/PHASE_2.md) | kosha, lekhya, parikshak, WASM | High-level + key decisions |
| 3-4 | [phases/PHASE_3_4.md](phases/PHASE_3_4.md) | LSP, extensions, community | Bullet points |

### Reading Order

1. **VISION.md** — understand *why* this project exists
2. **PRD.md** (this file) — understand *what* to build and *how* it fits together
3. **RUST_GUIDE.md** — set up your development environment
4. **PHASE_0.md** — start building

---

## 2. Technical Foundations

### Rust Edition & Toolchain

| Setting | Value | Rationale |
|---------|-------|-----------|
| Rust edition | 2024 | Latest edition; requires Rust 1.85+ |
| MSRV | 1.85.0 | New project, no legacy consumers |
| Toolchain pin | `rust-toolchain.toml` | Reproducible builds across machines |

**`rust-toolchain.toml`**:
```toml
[toolchain]
channel = "1.85.0"
components = ["rustfmt", "clippy"]
```

### Naming & License

| Setting | Value |
|---------|-------|
| Crate prefix | `varnavinyas-{name}` |
| Workspace layout | `crates/` directory (flat) |
| License | MIT OR Apache-2.0 (dual, Rust standard) |
| Repository | Single workspace, all crates in one repo |

---

## 3. Workspace Architecture

### Root `Cargo.toml` (Virtual Manifest)

```toml
[workspace]
resolver = "3"
members = [
    "crates/akshar",
    "crates/shabda",
    "crates/sandhi",
    "crates/prakriya",
    "crates/kosha",
    "crates/lipi",
    "crates/lekhya",
    "crates/parikshak",
    "crates/bindings-python",
    "crates/bindings-wasm",
]

[workspace.package]
edition = "2024"
rust-version = "1.85.0"
license = "MIT OR Apache-2.0"
repository = "https://github.com/varnavinyas/varnavinyas"
description = "Nepali language orthography toolkit"

[workspace.dependencies]
# Core
unicode-segmentation = "1.12"
thiserror = "2.0"
rustc-hash = "2.1"

# Lexicon
fst = "0.4"
serde = { version = "1.0", features = ["derive"] }
rmp-serde = "1.3"

# Bindings
pyo3 = { version = "0.23", features = ["abi3-py310"] }
wasm-bindgen = "0.2.100"

# Dev
proptest = "1.6"
criterion = { version = "0.5", features = ["html_reports"] }

# Internal
varnavinyas-akshar = { path = "crates/akshar" }
varnavinyas-shabda = { path = "crates/shabda" }
varnavinyas-sandhi = { path = "crates/sandhi" }
varnavinyas-prakriya = { path = "crates/prakriya" }
varnavinyas-kosha = { path = "crates/kosha" }
varnavinyas-lipi = { path = "crates/lipi" }
varnavinyas-lekhya = { path = "crates/lekhya" }
varnavinyas-parikshak = { path = "crates/parikshak" }
```

### Complete Directory Tree

```
varnavinyas/
├── Cargo.toml                          # Workspace root (virtual manifest)
├── rust-toolchain.toml                 # Pin Rust 1.85.0
├── deny.toml                           # cargo-deny configuration
├── rustfmt.toml                        # Formatter settings
├── clippy.toml                         # Linter settings
├── .cargo/
│   └── config.toml                     # Cargo aliases
├── .github/
│   └── workflows/
│       └── ci.yml                      # CI/CD pipeline
├── crates/
│   ├── akshar/                         # Character & script utilities
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── devanagari.rs           # Devanagari character tables
│   │   │   ├── vowel.rs               # Hrasva/dirgha classification
│   │   │   ├── consonant.rs           # Consonant classification
│   │   │   ├── syllable.rs            # Akshara segmentation
│   │   │   └── normalize.rs           # Unicode normalization
│   │   └── tests/
│   │       └── classification.rs       # Integration tests
│   ├── lipi/                           # Transliteration
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── scheme.rs              # Scheme definitions
│   │   │   ├── mapping.rs             # Mapping tables
│   │   │   └── legacy.rs              # Preeti/Kantipur conversion
│   │   └── tests/
│   │       └── roundtrip.rs
│   ├── shabda/                         # Word analysis
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── origin.rs              # Tatsam/tadbhav classification
│   │       ├── morphology.rs          # Root/prefix/suffix decomposition
│   │       └── gender.rs              # Gender marker analysis
│   ├── sandhi/                         # Sound change rules
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── conjunct.rs            # Consonant cluster rules
│   │       ├── vowel_sandhi.rs        # Vowel combination rules
│   │       └── prefix.rs             # Prefix attachment rules
│   ├── prakriya/                       # Derivation engine
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── prakriya.rs            # Core derivation state + history
│   │       ├── rule.rs                # Rule abstraction with citations
│   │       ├── hrasva_dirgha.rs       # Vowel-length resolution
│   │       └── step.rs               # Step recording (rule tracing)
│   ├── kosha/                          # FST lexicon
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── kosha.rs               # FST-backed word store
│   │       ├── packer.rs             # Semantic data packing
│   │       └── builder.rs            # Lexicon construction
│   ├── lekhya/                         # Punctuation & writing conventions
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── punctuation.rs         # Punctuation rules
│   │       └── padavali.rs           # Word-joining conventions
│   ├── parikshak/                      # Spell checker (integration crate)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── checker.rs             # Spell check pipeline
│   │       ├── linter.rs             # Orthography linting
│   │       └── diagnostic.rs         # Error reporting with citations
│   ├── bindings-python/                # PyO3 bindings
│   │   ├── Cargo.toml
│   │   ├── pyproject.toml             # maturin build config
│   │   └── src/
│   │       ├── lib.rs                 # Top-level module registration
│   │       ├── akshar.rs
│   │       ├── lipi.rs
│   │       └── shabda.rs
│   └── bindings-wasm/                  # WASM bindings
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
├── docs/
│   ├── PRD.md                          # This file
│   ├── VISION.md                       # Philosophy & motivation
│   ├── ROADMAP.md                      # Demo-driven roadmap
│   ├── RUST_GUIDE.md                   # Rust onboarding
│   ├── phases/
│   │   ├── PHASE_0.md
│   │   ├── PHASE_1.md
│   │   ├── PHASE_2.md
│   │   └── PHASE_3_4.md
│   └── tests/
│       ├── gold.toml                   # 91 verified test pairs
│       └── needs_review.toml           # 21 disputed pairs
└── reference/
    └── vidyut/                         # Vidyut source code (architectural reference)
```

### Cargo Aliases — `.cargo/config.toml`

```toml
[alias]
t = "test --workspace"
c = "clippy --workspace -- -D warnings"
f = "fmt --all"
d = "doc --workspace --no-deps --open"
ta = "test --workspace -- --include-ignored"
```

### Formatter — `rustfmt.toml`

```toml
# Keep defaults — Rust community standard
# Only override if needed
max_width = 100
use_field_init_shorthand = true
```

### Linter — `clippy.toml`

```toml
# Clippy configuration
msrv = "1.85.0"
```

---

## 4. Dependency Inventory

All external crates used across the workspace:

| Crate | Version | Purpose | Used By | Category |
|-------|---------|---------|---------|----------|
| `unicode-segmentation` | 1.12 | Unicode grapheme/word segmentation | akshar | Core |
| `thiserror` | 2.0 | Structured error types via derive macros | all crates | Core |
| `rustc-hash` | 2.1 | Fast non-cryptographic hash map/set | akshar, kosha | Core |
| `fst` | 0.4 | Finite state transducer for lexicon | kosha | Lexicon |
| `serde` | 1.0 | Serialization framework | kosha | Lexicon |
| `rmp-serde` | 1.3 | MessagePack (compact binary serialization) | kosha | Lexicon |
| `pyo3` | 0.23 | Python bindings via abi3 | bindings-python | Bindings |
| `wasm-bindgen` | 0.2.100 | WASM JavaScript interop | bindings-wasm | Bindings |
| `proptest` | 1.6 | Property-based testing | all (dev-dependency) | Testing |
| `criterion` | 0.5 | Benchmarking framework | all (dev-dependency) | Testing |

### Dependency Principles

1. **Minimal**: Each crate declares only the dependencies it needs
2. **Workspace versions**: All dependency versions declared in workspace root
3. **No duplicate purposes**: One crate per job (e.g., `thiserror` only, not `anyhow`)
4. **Dev vs runtime**: `proptest` and `criterion` are dev-only, never compiled into releases

---

## 5. Crate Dependency Graph

```
varnavinyas-parikshak (integration / spell checker)
├── varnavinyas-prakriya (derivation engine)
│   ├── varnavinyas-akshar
│   ├── varnavinyas-shabda
│   │   └── varnavinyas-akshar
│   └── varnavinyas-sandhi
│       └── varnavinyas-akshar
├── varnavinyas-kosha (FST lexicon)
│   └── varnavinyas-akshar
├── varnavinyas-lekhya (punctuation)
│   └── varnavinyas-akshar
└── varnavinyas-lipi (transliteration)
    └── varnavinyas-akshar

varnavinyas-akshar (leaf crate — no internal deps)
```

### Key observations

- **akshar** is the foundation — every other crate depends on it
- **parikshak** is the top-level integrator — it composes everything
- **lipi** is independent of the rule engine (transliteration stands alone)
- **prakriya** orchestrates shabda + sandhi for rule application
- No circular dependencies; strict DAG

---

## 6. Testing Strategy

### Test Pyramid

| Layer | Location | Tool | Purpose |
|-------|----------|------|---------|
| Unit tests | `crates/{name}/src/*.rs` | `#[cfg(test)] mod tests` | Test individual functions |
| Integration tests | `crates/{name}/tests/*.rs` | `cargo test -p {name}` | Test cross-module behavior |
| Doc tests | `/// # Examples` on public items | `cargo test --doc` | Ensure examples compile & run |
| Property tests | Inside unit/integration tests | `proptest` | Verify invariants hold for all inputs |
| Gold dataset | `docs/tests/gold.toml` | Custom test harness | Verify all 91 Academy pairs |
| Benchmarks | `crates/{name}/benches/*.rs` | `criterion` | Performance regression detection |

### Gold Dataset Tests

The 91 verified pairs in `docs/tests/gold.toml` serve as ground truth:

```rust
// Example test runner pattern (in parikshak integration tests)
#[test]
fn test_gold_shuddha_table() {
    let gold: GoldData = load_gold("docs/tests/gold.toml");
    for entry in &gold.shuddha_table {
        let result = check(&entry.incorrect);
        assert_eq!(result.correction, entry.correct,
            "Failed on: {} (rule: {})", entry.incorrect, entry.rule);
    }
}
```

**Categories in gold.toml** (91 entries total):
- `shuddha_table` — Section 4 correct/incorrect pairs
- `hrasva_dirgha` — Section 3(क) vowel length
- `chandrabindu` — Section 3(ख) nasalization
- `sha_sha_sa` — Section 3(ग) sibilant confusion
- `ri_kri` — Section 3(ग) ऋ/कृ errors
- `halanta` — Section 3(ङ) virama rules
- `ya_e` — Section 3(छ) य/ए confusion
- `ksha_chhya` — Section 3(छ) क्ष/छ्य confusion
- `paragraph_correction` — Cross-rule corrections

**Excluded**: `needs_review.toml` (21 disputed entries) — not used in automated tests until resolved.

### Property Tests

Key invariants to verify with `proptest`:

| Invariant | Crate | Property |
|-----------|-------|----------|
| Normalization idempotence | akshar | `normalize(normalize(s)) == normalize(s)` |
| Transliteration roundtrip | lipi | `transliterate(transliterate(s, A, B), B, A) == s` |
| Sandhi reversibility | sandhi | `split(apply(a, b))` contains `(a, b)` |
| Classification totality | akshar | Every Devanagari char maps to exactly one category |

### Coverage Target

- **Line coverage**: > 90% across all crates
- Tool: `cargo-llvm-cov` → upload to Codecov
- Coverage tracked per PR, regressions blocked

---

## 7. CI/CD Pipeline

### GitHub Actions — `.github/workflows/ci.yml`

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
  schedule:
    - cron: '0 6 * * *'

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: "-D warnings"

jobs:
  test:
    name: Test (${{ matrix.os }})
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --workspace

  msrv:
    name: MSRV (1.85.0)
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@1.85.0
      - uses: Swatinem/rust-cache@v2
      - run: cargo check --workspace

  fmt:
    name: Formatting
    runs-on: ubuntu-latest
    # NOTE: nightly rustfmt is standard Rust ecosystem practice.
    # rustfmt features stabilize slowly; nightly gives access to
    # rustfmt.toml options like imports_granularity. Pin a specific
    # nightly date if formatting drift becomes an issue:
    #   dtolnay/rust-toolchain@nightly with toolchain: nightly-2025-01-15
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@nightly
        with:
          components: rustfmt
      - run: cargo +nightly fmt --all --check

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo clippy --workspace --all-targets -- -D warnings

  deny:
    name: Cargo Deny
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: EmbarkStudios/cargo-deny-action@v2

  coverage:
    name: Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: llvm-tools-preview
      - uses: taiki-e/install-action@cargo-llvm-cov
      - uses: Swatinem/rust-cache@v2
      - run: cargo llvm-cov --workspace --lcov --output-path lcov.info
      - uses: codecov/codecov-action@v4
        with:
          files: lcov.info
          fail_ci_if_error: false

  docs:
    name: Documentation
    runs-on: ubuntu-latest
    env:
      RUSTDOCFLAGS: "-D warnings"
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo doc --workspace --no-deps

  wasm:
    name: WASM Build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      - uses: Swatinem/rust-cache@v2
      - run: cargo build -p varnavinyas-bindings-wasm --target wasm32-unknown-unknown

  audit:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: rustsec/audit-check@v2
```

### `deny.toml`

```toml
[advisories]
vulnerability = "deny"
unmaintained = "warn"
yanked = "deny"

[licenses]
allow = [
    "MIT",
    "Apache-2.0",
    "Unicode-3.0",
    "Unicode-DFS-2016",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Zlib",
]
confidence-threshold = 0.8

[bans]
multiple-versions = "warn"
wildcards = "deny"

[sources]
allow-git = []
```

---

## 8. Error Handling Strategy

### Pattern: Per-crate `thiserror` enums

Every crate defines its own error type using `thiserror 2.0`:

```rust
// crates/akshar/src/lib.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AksharError {
    #[error("invalid Devanagari codepoint: U+{0:04X}")]
    InvalidCodepoint(u32),

    #[error("empty input")]
    EmptyInput,

    #[error("normalization failed: {0}")]
    NormalizationError(String),
}
```

### Error types per crate

| Crate | Error Type | Key Variants |
|-------|-----------|--------------|
| akshar | `AksharError` | `InvalidCodepoint`, `EmptyInput` |
| lipi | `LipiError` | `UnsupportedScheme`, `InvalidInput`, `MappingError` |
| shabda | `ShabdaError` | `UnknownWord`, `ClassificationFailed` |
| sandhi | `SandhiError` | `InvalidComponents`, `NoRuleApplies` |
| prakriya | `PrakriyaError` | `NoDerivation`, `AmbiguousRule` |
| kosha | `KoshaError` | `FstError`, `PackingError`, `NotFound` |
| lekhya | `LekhyaError` | `InvalidPunctuation` |
| parikshak | `ParikshakError` | wraps all lower crate errors |

### Binding conversions

```rust
// PyO3 — convert crate errors to Python exceptions
impl From<AksharError> for pyo3::PyErr {
    fn from(err: AksharError) -> pyo3::PyErr {
        pyo3::exceptions::PyValueError::new_err(err.to_string())
    }
}

// WASM — convert to JsError
impl From<AksharError> for wasm_bindgen::JsError {
    fn from(err: AksharError) -> wasm_bindgen::JsError {
        wasm_bindgen::JsError::new(&err.to_string())
    }
}
```

---

## 9. Risk Assessment

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| **Unicode edge cases** (ZWJ, ZWNJ, composed vs decomposed Devanagari) | High | High | Use `unicode-segmentation` for grapheme boundaries; normalize to NFC early; extensive property tests |
| **Tatsam/tadbhav classification boundary** | Medium | High | Start with lookup table from Academy standard; heuristic classification is Phase 1 stretch goal |
| **FST size for full lexicon** | Medium | Low | Initial Phase 2 lexicon is ~2000 entries (tiny); monitor size if community lexicon grows beyond 100K |
| **WASM bundle size** | Medium | Medium | Target <5MB gzipped; strip debug info; use `wasm-opt -Oz`; defer kosha FST loading to async |
| **needs_review.toml entries** (21 disputed pairs) | Low | Certain | Resolve incrementally; never include in automated tests until promoted to gold.toml |
| **Preeti/Kantipur encoding** | Medium | Medium | Underdocumented legacy formats; start with Preeti (most common); test against known documents |
| **Rust learning curve** | Medium | Certain | RUST_GUIDE.md companion; start with akshar (string processing = Rust's strength); use clippy as mentor |
| **Academy standard ambiguity** | Medium | Medium | Some rules have exceptions or context-dependent forms; document ambiguities in needs_review.toml |

---

## 10. Phase Overview

| Phase | Crates | Milestone | Detail Level | Timeline Guide |
|-------|--------|-----------|-------------|---------------|
| **0** | akshar, lipi | "I can read Devanagari" — character classification, transliteration, CI/CD | Full specs | Months 1-3 |
| **1** | shabda, sandhi, prakriya, bindings-python | "I know which words are wrong" — rule engine with tracing | Detailed | Months 3-7 |
| **2** | kosha, lekhya, parikshak, bindings-wasm | "I can check a paragraph" — FST lexicon, spell checker | High-level | Months 7-11 |
| **3-4** | LSP, VS Code extension, UniFFI, C, CLI | "I work in your editor" + community | Bullet points | Months 11-18 |

### Start here: [Phase 0 — Foundation](phases/PHASE_0.md)

---

*वर्णविन्यास — शुद्ध नेपाली, सबैका लागि।*
