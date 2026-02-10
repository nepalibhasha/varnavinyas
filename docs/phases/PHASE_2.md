# Phase 2: Lexicon & Spell Checker

**Goal**: Working spell checker with FST-based lexicon lookup and full rule-based correction for paragraphs.
**Demo**: Feed in a paragraph with errors, get all errors flagged with corrections and rule citations.
**Prerequisite**: Phase 1 complete (shabda, sandhi, prakriya working).

---

## Crate: `varnavinyas-kosha`

### Purpose

A memory-efficient FST (Finite State Transducer) based lexicon encoding the Nepal Academy's correct word list and the ~2000+ correct/incorrect pairs from Section 4. Sub-microsecond lookup.

### Approach

Following Vidyut's FST pattern using the `fst` crate:

- Words stored in an `fst::Map` with semantic metadata packed into 64-bit values
- A `Packer` maps between packed integers and full semantic records (word origin, gender, correct form reference)
- Duplicate handling via 2-byte key extension (supporting up to 4,225 homographs)
- Build-time FST construction: the FST is built at compile time (or at first run) from embedded word tables

### Key Dependencies

```toml
[dependencies]
varnavinyas-akshar = { workspace = true }
fst = { workspace = true }
serde = { workspace = true }
rmp-serde = { workspace = true }
thiserror = { workspace = true }
rustc-hash = { workspace = true }
```

### Key Decision: Build-time vs Runtime FST

**Build-time generation** (recommended for Phase 2):
- Embed the Section 4 word table as a Rust `include_bytes!` resource
- Build the FST during `build.rs` or include a pre-built FST binary in the repo
- Advantage: zero startup cost, deterministic
- Disadvantage: rebuilds needed for lexicon changes

**Runtime loading** (future, for community-contributed lexicons):
- Load FST from a file path at runtime
- Needed when the lexicon grows beyond what's practical to embed

### Packing Strategy

Metadata packed into a `u64`:

```text
OOGG__________________IIIIIIIIIIIIIIIIII (40 bits used)

O: Origin (2 bits = 4 values: Tatsam/Tadbhav/Deshaj/Aagantuk)
G: Gender (2 bits = 4 values: Masculine/Feminine/Neuter/Unknown)
I: Correct-form index (18 bits = 262,144 possible entries)
_: Reserved for future use
```

If the correct-form index is 0, the word is itself correct. Non-zero index points into a string table of correct forms.

### Performance Targets

| Metric | Target |
|--------|--------|
| Single word lookup | < 1 μs |
| FST memory footprint | < 5 MB for ~2000 entries |
| Full lexicon (future) | < 50 MB |

### Acceptance Criteria

| # | Criterion |
|---|-----------|
| K1 | All ~2000 entries from Section 4 are queryable |
| K2 | Lookup returns correct/incorrect status + correction |
| K3 | Lookup performance < 1 μs (criterion benchmark) |
| K4 | Unknown words return `None` (not false positives) |
| K5 | FST builds deterministically from source data |

---

## Crate: `varnavinyas-lekhya`

### Purpose

Implements Section 5 of the Academy standard: Nepali punctuation rules (14 mark types), padavali (word-joining) conventions, and number formatting.

### Scope: 14 Punctuation Marks

From Section 5 of the Academy standard:

| # | Mark | Nepali Name | Unicode |
|---|------|-------------|---------|
| 1 | , | अल्पविराम | U+002C |
| 2 | । | पूर्णविराम | U+0964 |
| 3 | ? | प्रश्नवाचक | U+003F |
| 4 | ! | विस्मयबोधक | U+0021 |
| 5 | :- | निर्देशक | — |
| 6 | ' ' | एकल उद्धरण | U+2018/2019 |
| 7 | " " | दोहोरो उद्धरण | U+201C/201D |
| 8 | ( ) | कोष्ठक | U+0028/0029 |
| 9 | - | योजक | U+002D |
| 10 | . | सङ्क्षेप चिह्न | U+002E |
| 11 | ,, | ऐजन | — |
| 12 | / | तिर्यक् विराम | U+002F |
| 13 | ; | अर्धविराम | U+003B |
| 14 | ... | ऐजन बिन्दु | U+2026 |

### Additional Scope

- **Padavali conventions**: When words join vs. separate (e.g., "घरभित्र" vs "घर भित्र")
- **Number formatting**: Nepali numeral conventions
- **Halanta rules**: When halanta (्) is required at word end (e.g., अर्थात्, संसद्)

### Acceptance Criteria

| # | Criterion |
|---|-----------|
| Y1 | Detects incorrect punctuation usage |
| Y2 | Suggests correct Nepali punctuation marks |
| Y3 | Flags halanta omission (from gold.toml halanta entries) |
| Y4 | Padavali rules flag incorrect word joining/separation |

---

## Crate: `varnavinyas-parikshak`

### Purpose

The top-level integration crate composing all lower crates into a spell-checking and linting pipeline. This is the crate that end-users interact with.

### Pipeline

```
Input text
    │
    ├─ 1. Normalize (akshar)
    ├─ 2. Tokenize into words (akshar)
    ├─ 3. For each word:
    │     ├─ 3a. Lookup in kosha (exact match: fast path)
    │     ├─ 3b. If not found: classify origin (shabda)
    │     ├─ 3c. Apply rules (prakriya) → get correction + trace
    │     └─ 3d. Record diagnostic
    ├─ 4. Check punctuation (lekhya)
    └─ 5. Return diagnostics
```

### Diagnostic Struct

```rust
/// A single diagnostic (error report).
pub struct Diagnostic {
    /// Byte offset span in the original text.
    pub span: (usize, usize),
    /// The incorrect form as it appears in text.
    pub incorrect: String,
    /// The suggested correct form.
    pub correction: String,
    /// The rule that was violated.
    pub rule: Rule,
    /// Human-readable explanation (in Nepali).
    pub explanation: String,
    /// Error category for grouping/filtering.
    pub category: DiagnosticCategory,
}

pub enum DiagnosticCategory {
    HrasvaDirgha,
    Chandrabindu,
    ShaShaS,
    RiKri,
    Halanta,
    YaE,
    KshaChhya,
    Sandhi,
    Punctuation,
    ShuddhaTable,
}
```

### Batch Mode

```rust
/// Check a single word.
pub fn check_word(word: &str) -> Option<Diagnostic>;

/// Check an entire text, returning all diagnostics.
pub fn check_text(text: &str) -> Vec<Diagnostic>;
```

### Acceptance Criteria

| # | Criterion |
|---|-----------|
| C1 | Processes the slide paragraph fixture with all expected errors found |
| C2 | Processes the exam Q10 paragraph with all expected errors found |
| C3 | Structured diagnostics include position, correction, rule, explanation |
| C4 | Batch mode handles multi-paragraph documents |
| C5 | Performance: 1000-word document < 50ms |
| C6 | No false positives on correctly-spelled gold.toml "correct" forms |

---

## WASM Bindings

### Setup

```toml
# crates/bindings-wasm/Cargo.toml
[package]
name = "varnavinyas-bindings-wasm"
version = "0.1.0"
edition.workspace = true

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = { workspace = true }
varnavinyas-parikshak = { workspace = true }
varnavinyas-lipi = { workspace = true }
varnavinyas-akshar = { workspace = true }
```

### Build Pipeline

```bash
# Build WASM
cargo build -p varnavinyas-bindings-wasm --target wasm32-unknown-unknown --release

# Generate JS bindings
wasm-bindgen target/wasm32-unknown-unknown/release/varnavinyas_bindings_wasm.wasm \
    --out-dir pkg --target web

# Optimize
wasm-opt pkg/varnavinyas_bindings_wasm_bg.wasm -Oz -o pkg/varnavinyas_bindings_wasm_bg.wasm
```

### Web API

```javascript
import init, { checkText, transliterate } from './pkg/varnavinyas_bindings_wasm.js';

await init();

const diagnostics = checkText("अत्याधिक शब्द");
diagnostics.forEach(d => {
    console.log(`${d.incorrect} → ${d.correction} (${d.rule})`);
});
```

### Bundle Size Target

- **Target**: < 5 MB gzipped
- **Strategy**: Strip debug info, use `wasm-opt -Oz`, defer kosha FST loading if needed
- **Monitoring**: CI job measures WASM bundle size on every PR

---

## Phase 2 Completion Checklist

- [ ] `varnavinyas-kosha` stores and queries all Section 4 entries (K1-K5)
- [ ] `varnavinyas-lekhya` detects punctuation errors (Y1-Y4)
- [ ] `varnavinyas-parikshak` checks full paragraphs with diagnostics (C1-C6)
- [ ] WASM bindings build and run in browser
- [ ] Python bindings cover kosha, lekhya, parikshak
- [ ] Both paragraph fixtures (slide + exam Q10) pass
- [ ] Performance benchmarks established with criterion

### Next: [Phases 3-4 — Ecosystem & Refinement](PHASE_3_4.md)
