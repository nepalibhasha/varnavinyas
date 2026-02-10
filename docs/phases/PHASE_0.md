# Phase 0: Foundation

**Goal**: Working character utilities, transliteration, and project infrastructure.
**Demo**: Feed in any Nepali text, get back structured character-level analysis.
**Prerequisite**: [RUST_GUIDE.md](../RUST_GUIDE.md) for Rust setup and concepts.

---

## Prerequisites & Setup

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version   # should print 1.85.0 or higher
cargo --version
```

### 2. Install Development Tools

```bash
# Linting & auditing
cargo install cargo-deny
cargo install cargo-llvm-cov

# WASM tooling (needed later, but install now)
cargo install wasm-bindgen-cli

# Optional but recommended
cargo install cargo-watch   # auto-rebuild on save
```

### 3. Create Workspace

```bash
mkdir -p varnavinyas/{crates,.cargo,.github/workflows,docs}
cd varnavinyas

# Create rust-toolchain.toml, deny.toml, rustfmt.toml, clippy.toml
# Create .cargo/config.toml

# Initialize crates for Phase 0
cargo init --lib crates/akshar --name varnavinyas-akshar
cargo init --lib crates/lipi --name varnavinyas-lipi

# Verify it compiles
cargo build --workspace
cargo test --workspace
```

**Important**: The root `Cargo.toml` for Phase 0 should list only the crates that exist. Start with this and add members as new crates are created in later phases:

```toml
[workspace]
resolver = "3"
members = [
    "crates/akshar",
    "crates/lipi",
]

# ... rest of workspace config from PRD.md Section 3
# (workspace.package, workspace.dependencies, etc.)
```

The full member list in PRD.md Section 3 shows the final state. Add crates to `members` as you create them in each phase.

---

## Crate: `varnavinyas-akshar`

The foundation layer. Every other crate depends on this.

### Purpose

Devanagari script processing: character classification, vowel analysis, syllable segmentation, and Unicode normalization. This crate answers the question: "What are the building blocks of this Nepali text?"

### Cargo.toml

```toml
[package]
name = "varnavinyas-akshar"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
description = "Devanagari character classification and syllable segmentation"

[dependencies]
unicode-segmentation = { workspace = true }
thiserror = { workspace = true }
rustc-hash = { workspace = true }

[dev-dependencies]
proptest = { workspace = true }
```

### Public API

```rust
// ===== Types =====

/// Classification of a Devanagari character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharType {
    Svar,           // स्वर (vowel): अ आ इ ई उ ऊ ऋ ए ऐ ओ औ
    Vyanjan,        // व्यञ्जन (consonant): क ख ग ... ह
    Matra,          // मात्रा (vowel sign): ा ि ी ु ू ृ े ै ो ौ
    Halanta,        // हलन्त (virama): ्
    Chandrabindu,   // चन्द्रबिन्दु: ँ
    Shirbindu,      // शिरबिन्दु (anusvara): ं
    Visarga,        // विसर्ग: ः
    Nukta,          // नुक्ता: ़
    Avagraha,       // अवग्रह: ऽ
    Numeral,        // अंक: ० १ २ ... ९
    Danda,          // दण्ड: । ॥
}

/// Vowel length classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SvarType {
    Hrasva,   // ह्रस्व (short): अ इ उ ऋ
    Dirgha,   // दीर्घ (long): आ ई ऊ ए ऐ ओ औ
}

/// Consonant group (varga).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Varga {
    KaVarga,    // क ख ग घ ङ
    ChaVarga,   // च छ ज झ ञ
    TaVarga,    // ट ठ ड ढ ण (retroflex)
    TaVarga2,   // त थ द ध न (dental)
    PaVarga,    // प फ ब भ म
    Antastha,   // य र ल व (semivowels)
    Ushma,      // श ष स (sibilants)
    Other,      // ह, क्ष, त्र, ज्ञ
}

/// A single syllable unit (akshara).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Akshara {
    /// The text of this akshara.
    pub text: String,
    /// Starting byte offset in the original string.
    pub start: usize,
    /// Ending byte offset in the original string.
    pub end: usize,
}

/// Detailed classification of a Devanagari character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DevanagariChar {
    pub char_type: CharType,
    pub varga: Option<Varga>,
    pub is_panchham: bool,   // ङ ञ ण न म
}

/// Error type for akshar operations.
#[derive(Debug, thiserror::Error)]
pub enum AksharError {
    #[error("invalid Devanagari codepoint: U+{0:04X}")]
    InvalidCodepoint(u32),

    #[error("empty input")]
    EmptyInput,

    #[error("normalization failed: {0}")]
    NormalizationError(String),
}

// ===== Classification Functions =====

/// Classify a character within the Devanagari Unicode block.
/// Returns `None` for non-Devanagari characters.
pub fn classify(c: char) -> Option<DevanagariChar>;

/// Check if the character is a Devanagari vowel (स्वर).
pub fn is_svar(c: char) -> bool;

/// Check if the character is a Devanagari consonant (व्यञ्जन).
pub fn is_vyanjan(c: char) -> bool;

/// Check if the character is a vowel sign (मात्रा).
pub fn is_matra(c: char) -> bool;

/// Check if the character is a virama (हलन्त).
pub fn is_halanta(c: char) -> bool;

/// Determine the vowel length of a svar or matra.
/// Returns `None` for non-vowel characters.
pub fn svar_type(c: char) -> Option<SvarType>;

/// Check if the character is a panchham varna (ङ ञ ण न म).
pub fn is_panchham(c: char) -> bool;

/// Get the consonant's varga classification.
pub fn varga(c: char) -> Option<Varga>;

// ===== Vowel Mapping =====

/// Convert a hrasva vowel/matra to its dirgha counterpart.
/// इ→ई, उ→ऊ, ि→ी, ु→ू
/// Returns `None` if the input is not a convertible hrasva.
pub fn hrasva_to_dirgha(c: char) -> Option<char>;

/// Convert a dirgha vowel/matra to its hrasva counterpart.
/// ई→इ, ऊ→उ, ी→ि, ू→ु
/// Returns `None` if the input is not a convertible dirgha.
pub fn dirgha_to_hrasva(c: char) -> Option<char>;

/// Get the matra form of a svar (vowel).
/// अ→(none), आ→ा, इ→ि, ई→ी, उ→ु, ऊ→ू, etc.
pub fn svar_to_matra(c: char) -> Option<char>;

/// Get the svar form of a matra (vowel sign).
/// ा→आ, ि→इ, ी→ई, ु→उ, ू→ऊ, etc.
pub fn matra_to_svar(c: char) -> Option<char>;

// ===== Segmentation =====

/// Split text into akshara (syllable) units.
///
/// An akshara is the minimal pronounceable unit:
/// - A consonant + optional halanta + consonant chains + vowel sign
/// - A standalone vowel
/// - Anusvara/chandrabindu attach to the preceding akshara
///
/// # Examples
///
/// ```
/// use varnavinyas_akshar::split_aksharas;
///
/// let result = split_aksharas("नमस्ते");
/// let texts: Vec<&str> = result.iter().map(|a| a.text.as_str()).collect();
/// assert_eq!(texts, vec!["न", "मस्", "ते"]);
/// ```
pub fn split_aksharas(text: &str) -> Vec<Akshara>;

// ===== Normalization =====

/// Normalize Devanagari text to a canonical form (NFC).
///
/// - Applies Unicode NFC normalization
/// - Standardizes visually identical sequences
///
/// Invariant: `normalize(normalize(s)) == normalize(s)` (idempotent)
pub fn normalize(text: &str) -> String;
```

### Module Layout

| Module | File | Contents |
|--------|------|----------|
| `devanagari` | `devanagari.rs` | Character tables, `classify()`, `CharType`, `DevanagariChar` |
| `vowel` | `vowel.rs` | `SvarType`, `svar_type()`, `hrasva_to_dirgha()`, `dirgha_to_hrasva()`, matra↔svar conversion |
| `consonant` | `consonant.rs` | `Varga`, `varga()`, `is_panchham()` |
| `syllable` | `syllable.rs` | `Akshara`, `split_aksharas()` |
| `normalize` | `normalize.rs` | `normalize()` — NFC normalization |
| `lib` | `lib.rs` | Re-exports all public API; `AksharError` |

### Implementation Notes

**Devanagari Unicode block (U+0900-U+097F)**:
- U+0900-U+0903: chandrabindu, anusvara, visarga
- U+0904-U+0914: vowels (svar)
- U+0915-U+0939: consonants (vyanjan)
- U+093A-U+094F: matras (vowel signs)
- U+094D: virama (halanta) — **critical for conjunct detection**
- U+0950: OM
- U+0958-U+095F: nukta forms
- U+0960-U+0963: vocalic vowels (ॠ, ॡ)
- U+0964-U+0965: dandas (। ॥)
- U+0966-U+096F: numerals (०-९)
- U+0970: abbreviation sign (॰)
- U+0971: high spacing dot

**Virama (्) for conjunct detection**: The virama character (U+094D) signals that the preceding consonant has no inherent vowel and forms a conjunct with the following consonant. For `split_aksharas`, a sequence like `स्ते` should be parsed as one akshara containing the conjunct `स्त` + matra `े`. The algorithm: scan for virama, look ahead for the next consonant, and group them together.

**NFC vs NFD**: Devanagari text can be encoded in composed (NFC) or decomposed (NFD) forms. For example, `की` can be a single composed character or `क` + `ी` separately. Always normalize to NFC first for consistent comparison.

### Acceptance Criteria

| # | Criterion | Test |
|---|-----------|------|
| A1 | Classifies all Devanagari codepoints in U+0900-U+097F (128 positions) correctly | `classify(c)` returns correct `CharType` for every codepoint in the range |
| A2 | Maps all hrasva↔dirgha vowel pairs | `hrasva_to_dirgha('इ') == Some('ई')`, `dirgha_to_hrasva('ई') == Some('इ')`, same for उ↔ऊ and matra forms ि↔ी, ु↔ू |
| A3 | Syllable segmentation: simple word | `split_aksharas("काठमाडौं")` → 4 aksharas: `["का", "ठ", "मा", "डौं"]` |
| A4 | Syllable segmentation: conjuncts | `split_aksharas("नमस्ते")` → 3 aksharas: `["न", "मस्", "ते"]` |
| A5 | Conjuncts are one akshara | `split_aksharas("प्रशासन")` treats `प्र` as one akshara |
| A6 | Normalization idempotence | Property test: `normalize(normalize(s)) == normalize(s)` for any Devanagari string |
| A7 | Panchham varna identification | `is_panchham('ङ') == true`, `is_panchham('ञ') == true`, `is_panchham('ण') == true`, `is_panchham('न') == true`, `is_panchham('म') == true`, `is_panchham('क') == false` |
| A8 | Matra↔svar conversion roundtrip | For every matra in ा ि ी ु ू ृ े ै ो ौ: `svar_to_matra(matra_to_svar(m)) == Some(m)` |
| A9 | Returns `None` for non-Devanagari | `classify('A') == None`, `classify('中') == None` |
| A10 | Zero runtime dependencies beyond workspace | Only `unicode-segmentation`, `thiserror`, `rustc-hash` in `Cargo.toml` |

---

## Crate: `varnavinyas-lipi`

### Purpose

Script transliteration: convert text between Devanagari, IAST, ISO 15919, Nepali romanized, and legacy font encodings (Preeti, Kantipur). This crate answers: "How do I convert between writing systems?"

### Cargo.toml

```toml
[package]
name = "varnavinyas-lipi"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
description = "Devanagari transliteration and legacy font conversion"

[dependencies]
varnavinyas-akshar = { workspace = true }
thiserror = { workspace = true }

[dev-dependencies]
proptest = { workspace = true }
```

### Public API

```rust
/// Transliteration schemes supported by Varnavinyas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scheme {
    /// Devanagari Unicode script.
    Devanagari,
    /// International Alphabet of Sanskrit Transliteration.
    Iast,
    /// ISO 15919 romanization standard.
    Iso15919,
    /// Common informal Nepali romanization used online.
    NepaliRomanized,
    /// Preeti legacy font encoding.
    Preeti,
    /// Kantipur legacy font encoding.
    Kantipur,
}

/// Error type for transliteration operations.
#[derive(Debug, thiserror::Error)]
pub enum LipiError {
    #[error("unsupported transliteration: {from:?} → {to:?}")]
    UnsupportedPair { from: Scheme, to: Scheme },

    #[error("invalid input for scheme {scheme:?}: {detail}")]
    InvalidInput { scheme: Scheme, detail: String },

    #[error("unmappable character '{c}' in scheme {scheme:?}")]
    UnmappableChar { c: char, scheme: Scheme },
}

/// Transliterate text from one scheme to another.
///
/// # Examples
///
/// ```
/// use varnavinyas_lipi::{transliterate, Scheme};
///
/// let result = transliterate("namaste", Scheme::Iast, Scheme::Devanagari).unwrap();
/// assert_eq!(result, "नमस्ते");
/// ```
pub fn transliterate(input: &str, from: Scheme, to: Scheme) -> Result<String, LipiError>;

/// Attempt to detect the scheme of the input text.
///
/// Returns `None` if the scheme cannot be determined.
pub fn detect_scheme(input: &str) -> Option<Scheme>;
```

### Module Layout

| Module | File | Contents |
|--------|------|----------|
| `scheme` | `scheme.rs` | `Scheme` enum, detection logic |
| `mapping` | `mapping.rs` | Mapping tables between schemes (const arrays) |
| `legacy` | `legacy.rs` | Preeti → Unicode, Kantipur → Unicode lookup tables |
| `lib` | `lib.rs` | `transliterate()`, `detect_scheme()`, re-exports |

### Implementation Notes

**Mapping table structure**: Each scheme pair is defined as a pair of ordered arrays. Transliteration scans input left-to-right, matching the longest token first (greedy matching). For Devanagari↔IAST, the mapping covers:

- 14 vowels: अ↔a, आ↔ā, इ↔i, ई↔ī, उ↔u, ऊ↔ū, ऋ↔ṛ, ए↔e, ऐ↔ai, ओ↔o, औ↔au, अं↔aṃ, अः↔aḥ, ँ↔m̐
- 33 consonants: क↔ka, ख↔kha, ग↔ga ... ह↔ha
- 10 numerals: ०↔0, १↔1 ... ९↔9
- Special: virama (्), avagraha (ऽ)

**Preeti font encoding**: Preeti is a widely-used legacy Nepali font that maps Devanagari glyphs to ASCII code points. For example, Preeti ASCII character `g` maps to the Devanagari glyph `ा`, `]` maps to `ू`, etc. The mapping is a fixed lookup table of ~100 entries. Source: Preeti font documentation and community-maintained mapping tables.

**WASM future-proofing**: This crate will later be compiled to WASM. Keep the API simple (no `std::fs`, no threading). The `Cargo.toml` should not include `crate-type = ["cdylib"]` yet — that's added only in `bindings-wasm`.

### Acceptance Criteria

| # | Criterion | Test |
|---|-----------|------|
| L1 | Devanagari→IAST roundtrip | `transliterate(transliterate(s, Dev, IAST), IAST, Dev) == s` for valid Nepali text |
| L2 | All vowels transliterate correctly | Each of the 14 vowels maps correctly in both directions |
| L3 | All consonants transliterate correctly | Each of the 33 consonants maps correctly in both directions |
| L4 | Conjuncts transliterate correctly | `transliterate("क्ष", Dev, IAST) == "kṣa"` (or appropriate IAST) |
| L5 | Numerals transliterate | `transliterate("१२३", Dev, IAST) == "123"` |
| L6 | Preeti decode | A known Preeti-encoded string decodes to the correct Unicode Devanagari |
| L7 | Empty string handling | `transliterate("", any, any) == Ok("")` |
| L8 | Mixed-script handling | Non-target characters pass through unchanged |
| L9 | Scheme detection | `detect_scheme("नमस्ते") == Some(Devanagari)`, `detect_scheme("namaste") == Some(Iast)` (heuristic) |
| L10 | Property test: reversibility | `proptest` confirms roundtrip for Devanagari↔IAST on random Devanagari strings |

---

## CI/CD Setup

### Step-by-step

1. **Copy** the `.github/workflows/ci.yml` from [PRD.md Section 7](../PRD.md#7-cicd-pipeline) into your repository.

2. **Create** `deny.toml` at the workspace root (copy from PRD.md Section 7).

3. **Create** `rustfmt.toml` and `clippy.toml` (copy from PRD.md Section 3).

4. **Create** `rust-toolchain.toml` (copy from PRD.md Section 2).

5. **Verify locally** before pushing:

```bash
cargo fmt --all --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo deny check
cargo doc --workspace --no-deps
```

6. **Push** and verify all CI jobs pass.

---

## Step-by-Step Build Order

This is the exact sequence to follow. Each step builds on the previous.

### Step 1: Workspace Setup

```bash
# Create the directory structure from PRD.md Section 3
# Initialize each crate with cargo init --lib
# Copy Cargo.toml, rust-toolchain.toml, deny.toml, etc.
# Verify: cargo build --workspace && cargo test --workspace
```

**Done when**: `cargo test --workspace` passes with empty crates.

### Step 2: akshar — `devanagari.rs`

Implement character tables and `classify()`:
- Define the `CharType`, `DevanagariChar` types
- Write the classification function covering U+0900-U+097F
- Write unit tests for every character in the range

**Done when**: `classify()` returns correct `CharType` for all Devanagari codepoints (criterion A1, A9).

### Step 3: akshar — `vowel.rs`

Implement vowel analysis:
- `SvarType`, `svar_type()`, `is_svar()`, `is_matra()`
- `hrasva_to_dirgha()`, `dirgha_to_hrasva()`
- `svar_to_matra()`, `matra_to_svar()`

**Done when**: All hrasva↔dirgha mappings work (criterion A2, A8).

### Step 4: akshar — `consonant.rs`

Implement consonant classification:
- `Varga`, `varga()`, `is_vyanjan()`
- `is_panchham()`

**Done when**: Varga classification and panchham detection work (criterion A7).

### Step 5: akshar — `syllable.rs`

Implement akshara segmentation:
- `Akshara` struct, `split_aksharas()`
- Handle virama-based conjuncts
- Handle chandrabindu/anusvara attachment

**Done when**: Syllable segmentation passes criteria A3, A4, A5.

### Step 6: akshar — `normalize.rs`

Implement Unicode normalization:
- `normalize()` using NFC
- Property test for idempotence

**Done when**: Criterion A6 passes (property test).

### Step 7: akshar — Integration Tests

Write `crates/akshar/tests/classification.rs`:
- Test all Devanagari characters
- Test edge cases (empty string, mixed scripts, U+0000)
- Run proptest for normalization

**Done when**: `cargo test -p varnavinyas-akshar` all green.

### Step 8: lipi — `scheme.rs` + `mapping.rs`

Implement transliteration:
- `Scheme` enum, mapping tables
- `transliterate()` with greedy longest-match
- `detect_scheme()` heuristic

**Done when**: Devanagari↔IAST roundtrip works (criteria L1-L5).

### Step 9: lipi — `legacy.rs`

Implement Preeti/Kantipur conversion:
- Preeti→Unicode lookup table
- Kantipur→Unicode lookup table

**Done when**: Known Preeti strings decode correctly (criterion L6).

### Step 10: lipi — Integration Tests

Write `crates/lipi/tests/roundtrip.rs`:
- Roundtrip tests for all scheme pairs
- Property tests for reversibility
- Edge cases

**Done when**: `cargo test -p varnavinyas-lipi` all green.

### Step 11: CI/CD Setup

- Copy workflow YAML, deny.toml, config files
- Push to GitHub, verify all jobs pass
- Fix any issues

**Done when**: All CI jobs green on GitHub.

### Step 12: Gold Dataset Test Runner

Write an integration test that loads `docs/tests/gold.toml` and verifies that akshar operations work correctly on the gold data (character classification, syllable segmentation of gold entry words).

This is a *partial* test — full gold.toml verification comes in Phase 1 when the rule engine exists. For Phase 0, we verify that:
- All gold.toml words can be loaded and parsed
- All words can be segmented into aksharas without errors
- All words can be normalized without errors

**Done when**: `cargo test --workspace` passes including gold.toml smoke test.

---

## Phase 0 Completion Checklist

- [ ] `cargo build --workspace` succeeds
- [ ] `cargo test --workspace` passes (all akshar and lipi tests)
- [ ] `cargo clippy --workspace -- -D warnings` passes
- [ ] `cargo fmt --all --check` passes
- [ ] `cargo deny check` passes
- [ ] `cargo doc --workspace --no-deps` builds without warnings
- [ ] All 10 akshar acceptance criteria (A1-A10) verified
- [ ] All 10 lipi acceptance criteria (L1-L10) verified
- [ ] CI/CD pipeline green on GitHub
- [ ] Gold dataset smoke test passes

### Next: [Phase 1 — Core Rules Engine](PHASE_1.md)
