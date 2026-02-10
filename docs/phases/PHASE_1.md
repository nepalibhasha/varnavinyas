# Phase 1: Core Rules Engine

**Goal**: Rule engine capable of word origin classification, sandhi operations, and hrasva/dirgha correction with full rule tracing.
**Demo**: Feed in a word, get a traced derivation explaining why it's correct or incorrect.
**Prerequisite**: Phase 0 complete (akshar, lipi working).

---

## Crate: `varnavinyas-shabda`

### Purpose

Classify Nepali words by origin (tatsam/tadbhav/deshaj/aagantuk) and decompose them into morphological components (root + prefix + suffix). Origin classification is fundamental because different origin classes follow different spelling rules.

### Dependencies

```toml
[dependencies]
varnavinyas-akshar = { workspace = true }
thiserror = { workspace = true }
rustc-hash = { workspace = true }
```

### Key Types

```rust
/// Word origin classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Origin {
    /// तत्सम — direct Sanskrit borrowing, retains original form.
    Tatsam,
    /// तद्भव — modified Sanskrit, follows Nepali phonology.
    Tadbhav,
    /// देशज — native Nepali word.
    Deshaj,
    /// आगन्तुक — foreign loanword (English, Arabic, Hindi, etc.).
    Aagantuk,
}

/// Morphological decomposition of a word.
#[derive(Debug, Clone)]
pub struct Morpheme {
    pub root: String,
    pub prefixes: Vec<String>,   // उपसर्ग
    pub suffixes: Vec<String>,   // प्रत्यय
    pub origin: Origin,
}
```

### Classification Algorithm

The algorithm uses heuristics derived from Section 2 of the Academy standard:

1. **Tatsam markers**: Presence of ऋ, श, ष, क्ष, ज्ञ, visarga, or specific conjunct patterns strongly suggests tatsam (e.g., विज्ञान, ऋषि, प्रकृति)
2. **Tadbhav patterns**: Simplified phonology from Sanskrit origins (e.g., अग्नि→आगो, हस्त→हात)
3. **Aagantuk indicators**: Anglicized consonant clusters, retroflex ट/ड without Sanskrit context (e.g., कम्प्युटर, रजिस्टर)
4. **Deshaj fallback**: Native words not matching the above patterns (e.g., टोपी, भाका, चुला)

For Phase 1, classification primarily uses a **lookup table** from the Academy's word tables, supplemented by pattern-based heuristics. A full heuristic classifier is a stretch goal.

### Acceptance Criteria

| # | Criterion | Test |
|---|-----------|------|
| S1 | Classifies विज्ञान as Tatsam | `classify("विज्ञान").origin == Tatsam` |
| S2 | Classifies आगो as Tadbhav | `classify("आगो").origin == Tadbhav` |
| S3 | Classifies टोपी as Deshaj | `classify("टोपी").origin == Deshaj` |
| S4 | Classifies कम्प्युटर as Aagantuk | `classify("कम्प्युटर").origin == Aagantuk` |
| S5 | Decomposes प्रशासन | `decompose("प्रशासन").prefixes == ["प्र"], .root == "शासन"` |
| S6 | Decomposes उल्लिखित | `decompose("उल्लिखित").prefixes == ["उत्"], .root == "लिखित"` (sandhi applied) |
| S7 | All gold.toml word_form entries pass | Classification matches expected origin where derivable from the entry rule |

---

## Crate: `varnavinyas-sandhi`

### Purpose

Implement sandhi (sound-change) operations: the rules governing what happens when morphemes combine. Supports both forward application (combining) and reverse splitting.

### Dependencies

```toml
[dependencies]
varnavinyas-akshar = { workspace = true }
thiserror = { workspace = true }
```

### Key Types

```rust
/// Categories of sandhi rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandhiType {
    /// Vowel sandhi (अच् सन्धि): vowels combining at morpheme boundaries.
    VowelSandhi,
    /// Visarga sandhi (विसर्ग सन्धि): visarga transformations.
    VisargaSandhi,
    /// Consonant sandhi (हल् सन्धि): consonant assimilations.
    ConsonantSandhi,
}

/// Result of a sandhi operation.
#[derive(Debug, Clone)]
pub struct SandhiResult {
    pub output: String,
    pub sandhi_type: SandhiType,
    pub rule_citation: &'static str,
}
```

### Core Operations

```rust
/// Apply sandhi: combine two morphemes.
pub fn apply(first: &str, second: &str) -> Result<SandhiResult, SandhiError>;

/// Split a word at potential sandhi boundaries.
/// Returns all valid decompositions.
pub fn split(word: &str) -> Vec<(String, String, SandhiResult)>;
```

### Rule Categories

**Vowel sandhi** (from Section 3):
- यण् sandhi: इ/ई + vowel → य (e.g., अति + अधिक = अत्यधिक)
- गुण sandhi: अ + इ/ई → ए, अ + उ/ऊ → ओ
- वृद्धि sandhi: आ + इ/ई → ऐ, आ + उ/ऊ → औ
- दीर्घ sandhi: अ + अ → आ (e.g., स + अङ्ग = साङ्ग)

**Visarga sandhi** (from Section 4 table):
- पुनः + स्थापना = पुनःस्थापना (visarga retained before स)
- पुनः + अवलोकन = पुनरवलोकन (visarga → र before vowel)

**Consonant sandhi**:
- Prefix assimilation: उत् + लिखित = उल्लिखित, उत् + चारण = उच्चारण
- Conjunct formation: सम् + कलन = सङ्कलन

### Acceptance Criteria

| # | Criterion | Test |
|---|-----------|------|
| D1 | Vowel sandhi: apply | `apply("अति", "अधिक") == "अत्यधिक"` |
| D2 | Visarga sandhi: apply | `apply("पुनः", "अवलोकन") == "पुनरवलोकन"` |
| D3 | Visarga retained | `apply("पुनः", "स्थापना") == "पुनःस्थापना"` |
| D4 | Consonant assimilation | `apply("उत्", "लिखित") == "उल्लिखित"` |
| D5 | Sandhi split | `split("अत्यधिक")` contains `("अति", "अधिक")` |
| D6 | Sandhi split: visarga | `split("पुनरवलोकन")` contains `("पुनः", "अवलोकन")` |
| D7 | Gold.toml sandhi entries | All sandhi-category entries in gold.toml pass |

---

## Crate: `varnavinyas-prakriya`

### Purpose

The derivation engine. Given a word, trace step-by-step how rules apply to determine whether it's correct, and if not, what the correct form is. Every step records which Academy standard rule was applied. Modeled on Vidyut's `Prakriya` struct.

### Dependencies

```toml
[dependencies]
varnavinyas-akshar = { workspace = true }
varnavinyas-shabda = { workspace = true }
varnavinyas-sandhi = { workspace = true }
thiserror = { workspace = true }
```

### Key Types

```rust
/// A rule from an authoritative source.
/// Modeled after Vidyut's Rule enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rule {
    /// Nepal Academy Orthography Standard section reference.
    /// e.g., "3(क)" for hrasva/dirgha vowel rules.
    VarnaVinyasNiyam(&'static str),

    /// Nepal Academy Grammar reference.
    Vyakaran(&'static str),

    /// Specific word table entry from Section 4.
    /// (correct/incorrect word pairs)
    ShuddhaAshuddha(&'static str),

    /// Punctuation rule from Section 5.
    ChihnaNiyam(&'static str),
}

/// A single step in a derivation.
#[derive(Debug, Clone)]
pub struct Step {
    pub rule: Rule,
    pub description: String,
    pub before: String,
    pub after: String,
}

/// The derivation state, tracking history.
#[derive(Debug, Clone)]
pub struct Prakriya {
    pub input: String,
    pub output: String,
    pub steps: Vec<Step>,
    pub is_correct: bool,
}
```

### Core Engine: Hrasva/Dirgha Resolution

The 16 categories from Section 3(क) of the Academy standard:

| # | Category | Rule | Example |
|---|----------|------|---------|
| 1 | Tatsam retains original | Tatsam words keep their Sanskrit vowel length | मुख (hrasva in Sanskrit = hrasva in Nepali) |
| 2 | Tadbhav single-meaning → hrasva | Single-meaning tadbhav words take hrasva | मिष्ट→मिठो (not मीठो) |
| 3 | Kinship tadbhav | Initial/medial hrasva, final dirgha for feminine kinship | दिदी, बहिनी, भाउजू, फुपू |
| 4 | Masculine names → hrasva | All masculine kinship/names take hrasva | दाजु, बाबु, भाइ |
| 5 | Feminine nouns → dirgha | Feminine noun endings take dirgha ई | खुर्सानी, सम्धिनी |
| 6 | Pronouns → dirgha | Pronouns take dirgha | हामी (not हामि) |
| 7 | Postpositions → dirgha | Postposition endings take dirgha | अगाडी, पछाडी |
| 8 | Adjectival → dirgha | Demonym/adjectival endings take dirgha | पहाडी |
| 9 | Absolutive → dirgha | Absolutive verb form (पूर्वकालिक) takes dirgha | भनी (not भनि) |
| 10 | Suffix -ई/-ईय preserves dirgha | Suffixes -ई and -ईय preserve root dirgha | पूर्वी, पूर्वीय |
| 11 | Suffix -ए/-एली triggers hrasva | Suffixes -ए and -एली trigger root hrasva | पुर्वेली (not पूर्वेली) |
| 12 | Suffix -नु triggers hrasva | Verb suffix -नु triggers root hrasva | स्विकार्नु (not स्वीकार्नु) |
| 13 | Suffix -य preserves dirgha | Suffix -य preserves root dirgha | स्वीकार्य |
| 14 | -ईकरण suffix | Suffix -ईकरण: base stays hrasva | औद्योगिकीकरण (not औद्योगीकरण) |
| 15 | -इक suffix + आदिवृद्धि | -इक triggers आदिवृद्धि (अ→आ) | व्यावहारिक (not व्यवहारिक) |
| 16 | Redundant -ता | Abstract nouns ending in -य/-र्य don't take -ता | सौन्दर्य (not सौन्दर्यता) |

### Step Recording

Every correction produces a traceable derivation:

```rust
// Example derivation for "मीठो" → "मिठो"
let p = derive("मीठो");
assert_eq!(p.steps.len(), 3);
// Step 1: Classify origin → Tadbhav (from मिष्ट)
// Step 2: Apply Rule 3(क)-12: tadbhav single-meaning → hrasva
// Step 3: Replace ई-मात्रा with इ-मात्रा → "मिठो"
assert_eq!(p.output, "मिठो");
assert!(p.steps.iter().any(|s|
    matches!(s.rule, Rule::VarnaVinyasNiyam("3(क)-12"))
));
```

### Acceptance Criteria

| # | Criterion | Test |
|---|-----------|------|
| P1 | Corrects अत्याधिक → अत्यधिक | `derive("अत्याधिक").output == "अत्यधिक"` with Section 4 citation |
| P2 | Corrects मीठो → मिठो | `derive("मीठो").output == "मिठो"` citing Rule 3(क)-12 |
| P3 | Corrects हामि → हामी | `derive("हामि").output == "हामी"` citing pronoun rule |
| P4 | Accepts प्रशासन as correct | `derive("प्रशासन").is_correct == true` |
| P5 | All 91 gold.toml entries | Every entry produces correct output with a non-empty rule citation |
| P6 | Step trace non-empty | Every correction has `steps.len() > 0` |
| P7 | Suffix rules work | `derive("स्वीकार्नु").output == "स्विकार्नु"` with suffix rule citation |
| P8 | Multiple corrections | `derive("पूर्वेली").output == "पुर्वेली"` |

---

## Python Bindings

### Setup

```toml
# crates/bindings-python/Cargo.toml
[package]
name = "varnavinyas"
version = "0.1.0"
edition.workspace = true

[lib]
name = "varnavinyas"
crate-type = ["cdylib"]

[dependencies]
pyo3 = { workspace = true }
varnavinyas-akshar = { workspace = true }
varnavinyas-lipi = { workspace = true }
varnavinyas-shabda = { workspace = true }
varnavinyas-sandhi = { workspace = true }
varnavinyas-prakriya = { workspace = true }
```

```toml
# crates/bindings-python/pyproject.toml
[build-system]
requires = ["maturin>=1.8,<2.0"]
build-backend = "maturin"

[project]
name = "varnavinyas"
requires-python = ">=3.10"

[tool.maturin]
features = ["pyo3/abi3-py310"]
```

### Submodule Pattern

Following Vidyut's approach with `wrap_pymodule!`:

```rust
// crates/bindings-python/src/lib.rs
use pyo3::prelude::*;

mod akshar;
mod lipi;
mod shabda;

#[pymodule]
fn varnavinyas(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(akshar::akshar))?;
    m.add_wrapped(wrap_pymodule!(lipi::lipi))?;
    m.add_wrapped(wrap_pymodule!(shabda::shabda))?;
    Ok(())
}
```

### Python API

```python
import varnavinyas

# Character analysis
result = varnavinyas.akshar.classify('क')  # → CharType.Vyanjan
aksharas = varnavinyas.akshar.split_aksharas("नमस्ते")  # → ["न", "मस्", "ते"]

# Transliteration
text = varnavinyas.lipi.transliterate("namaste", "iast", "devanagari")

# Word classification
origin = varnavinyas.shabda.classify("विज्ञान")  # → Origin.Tatsam
```

### Type Stubs

Provide `.pyi` files for IDE autocompletion:

```
varnavinyas/
├── __init__.pyi
├── akshar.pyi
├── lipi.pyi
└── shabda.pyi
```

### Build & Test

```bash
cd crates/bindings-python
maturin develop    # Build and install in current venv
python -c "import varnavinyas; print(varnavinyas.akshar.split_aksharas('नमस्ते'))"
```

---

## Key Technical Decisions

### How to encode the 16 hrasva/dirgha rule categories

Use a `RuleCategory` enum with one variant per category. Each category is a function that takes a `Prakriya` and returns whether it applies:

```rust
pub(crate) fn apply_hrasva_dirgha_rules(prakriya: &mut Prakriya) {
    // Rules are tried in priority order
    if rule_tatsam_preservation(prakriya) { return; }
    if rule_tadbhav_single_meaning(prakriya) { return; }
    if rule_kinship_tadbhav(prakriya) { return; }
    if rule_feminine_noun(prakriya) { return; }
    if rule_pronoun_dirgha(prakriya) { return; }
    // ... etc.
}
```

Each rule function records a `Step` with its `Rule` citation when it fires.

### Pattern matching vs lookup table

**Hybrid approach**:
- Section 4 word pairs (शुद्ध-अशुद्ध table): **lookup table** (exact match, ~2000 entries)
- Section 3 rules (hrasva/dirgha, chandrabindu, etc.): **pattern matching** (structural rules applied via code)
- Lookup is tried first (fast path). If no exact match, fall back to rule-based analysis.

### Whether shabda needs an embedded lexicon

For Phase 1: **heuristics only** + the Academy's word table as lookup. No separate lexicon file.

The tatsam identification heuristics (presence of ऋ, श, ष, conjuncts like क्ष/ज्ञ) cover a large fraction of cases. For the remaining words, the Phase 2 FST lexicon (kosha) will provide full coverage.

---

## Phase 1 Completion Checklist

- [ ] `varnavinyas-shabda` classifies word origins correctly (S1-S7)
- [ ] `varnavinyas-sandhi` applies and splits sandhi (D1-D7)
- [ ] `varnavinyas-prakriya` traces derivations with rule citations (P1-P8)
- [ ] All 91 gold.toml entries produce correct corrections
- [ ] Python bindings build with `maturin develop`
- [ ] Python smoke test passes: `import varnavinyas; ...`
- [ ] `cargo test --workspace` all green
- [ ] CI/CD pipeline green

### Next: [Phase 2 — Lexicon & Spell Checker](PHASE_2.md)
