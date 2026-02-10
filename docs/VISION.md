# वर्णविन्यास (Varnavinyas)

**The definitive open-source Nepali language infrastructure toolkit**

*शुद्ध नेपाली भाषाको लागि मुक्त-स्रोत प्रविधि*
*Open-source technology for correct Nepali language*

---

## 1. The Problem

Nepali is spoken by over 32 million people, serves as the official language of Nepal, and is
recognized in the Indian Constitution. Yet it lacks the most basic computational language
infrastructure that languages like English, Chinese, and even Sanskrit now enjoy.

### No open-source orthography engine

The Government of Nepal published the **नेपाली वर्णविन्यास (Nepali Orthography)** standard
through the प्राज्ञा प्रतिष्ठान (Nepal Academy), codifying hundreds of rules for correct
spelling — covering hrasva/dirgha vowel selection, chandrabindu usage, panchham varna
rules, sandhi operations, prefix/suffix conventions, and punctuation. Yet **no open-source
software implements these rules**. The standard exists only as a PDF document, inaccessible
to machines.

### The hrasva/dirgha crisis

The most pervasive orthographic challenge in Nepali is the correct use of short (ह्रस्व) and
long (दीर्घ) vowels — इ vs ई, उ vs ऊ, and their matra forms. The government standard
dedicates extensive sections to this: 16 categories of rules governing when words take dirgha
vowels based on word origin (tatsam vs tadbhav), suffix patterns, gender markers, verb forms,
and morphological context. Errors in hrasva/dirgha selection are the single most common
spelling mistake in Nepali writing, yet no automated tool can detect or correct them.

### Fragmented linguistic knowledge

Nepali orthography draws from multiple knowledge domains:

- **Word origin classification**: तत्सम (Sanskrit-origin, e.g., सत्य→सत्य), तद्भव
  (modified Sanskrit, e.g., अद्य→आज), देशज (native Nepali), and आगन्तुक (foreign
  loanwords) — each following different spelling rules
- **Sandhi rules**: Complex sound-change operations at morpheme boundaries
  (e.g., क्+ष = क्ष, द्+भ = द्ध, ह्+म = ह्म)
- **Prefix/suffix conventions**: Rules governing how उपसर्ग and प्रत्यय attach to roots
  and when they trigger vowel changes
- **Punctuation standards**: Nepali-specific punctuation marks (अल्पविराम, पूर्णविराम,
  etc.) and their usage rules

This knowledge is scattered across grammar textbooks, academy publications, and the minds
of expert linguists — never unified into a computational system.

### No Vidyut equivalent for Nepali

The Sanskrit ecosystem now has [Vidyut](https://github.com/ambuda-org/vidyut) — an ambitious
open-source toolkit providing word generation (prakriya), morphological analysis, sandhi
operations, transliteration, and an FST-based lexicon — all in pure Rust with Python
bindings. Nepali, despite sharing Devanagari script and significant Sanskrit heritage, has
**nothing comparable**. Existing Nepali NLP tools are either proprietary, incomplete, or
abandoned.

### Government mandate without tooling

Nepal's government mandates शुद्ध (correct) Nepali in official documents, education, and
media. Schools teach orthography rules. The Academy publishes correction tables. But
without software tooling, compliance depends entirely on human expertise — which is
expensive, inconsistent, and unscalable.

---

## 2. Vision

**Varnavinyas aims to be the definitive, open-source computational infrastructure for the
Nepali language** — a complete toolkit that encodes the Nepal Academy's orthography
standard, Nepali grammar rules, and linguistic knowledge into efficient, portable,
offline-capable software.

Just as Vidyut brought rigorous computational linguistics to Sanskrit, Varnavinyas will do the
same for Nepali: providing the foundational building blocks that every Nepali language
application needs — from spell checkers and grammar tools to educational software and
document processing pipelines.

The name **वर्णविन्यास** itself means "orthography" — the arrangement (विन्यास) of
characters (वर्ण) — reflecting our core mission: making correct Nepali writing accessible
to everyone through technology.

---

## 3. Design Principles

### शुद्धता (Fidelity)

Every rule implemented must trace back to the Nepal Academy's orthography standard or
established grammatical authority. Rules are not invented or approximated — they are
encoded faithfully, with citations. When the standard provides a correct/incorrect word pair,
Varnavinyas must agree.

### गति (Performance)

Spell-checking a document should feel instant. Word derivation should complete in
microseconds. The toolkit must be fast enough to power real-time editor integrations
(keystroke-level latency) and batch processing of large document corpora alike.

### सर्वव्यापकता (Portability)

A single Rust core with bindings for every major platform:

- **Python** (PyO3) — for researchers, data scientists, NLP pipelines
- **WebAssembly** (wasm-pack) — for browser-based editors and web apps
- **Kotlin/Swift** (UniFFI) — for Android and iOS applications
- **C** (cbindgen) — for system integration and any FFI-capable language

Write the rules once, run them everywhere.

### स्वतन्त्र (Offline-First)

No API calls. No cloud dependencies. No internet required. The entire toolkit — rules, lexicon,
and all — ships as a self-contained binary. This is essential for:

- Schools in rural Nepal with limited connectivity
- Government offices with restricted networks
- Privacy-sensitive document processing
- Embedding in desktop and mobile applications

### विस्तारयोग्यता (Extensibility)

The crate architecture allows consumers to use only what they need. Need just
transliteration? Pull in `varnavinyas-lipi`. Need the full spell-checker? Pull in
`varnavinyas-parikshak` which composes all the lower-level crates.

### पारदर्शिता (Transparency)

Every correction comes with an explanation. When Varnavinyas flags a word as incorrect, it
reports *which rule* was violated, *what the correct form* is, and *why* — citing the specific
section of the orthography standard. This is not a black-box spell checker; it is an
educational tool.

---

## 4. Scope & Components

Varnavinyas is organized as a Rust workspace of focused crates, following the architectural
pattern established by Vidyut:

### Core Crates

#### `varnavinyas-akshar` — Character & Script Utilities

The foundation layer for Devanagari script processing.

- Unicode-aware character classification (स्वर, व्यञ्जन, मात्रा, चिह्न)
- Hrasva/dirgha vowel identification and mapping (इ↔ई, उ↔ऊ, and matra forms)
- Chandrabindu (ँ), shirbindu (ं), and panchham varna (ङ, ञ, ण, न, म) handling
- Conjunct consonant (संयुक्त वर्ण) decomposition and analysis
- Devanagari syllable (अक्षर) segmentation
- Character normalization (handling Unicode composition variants)

#### `varnavinyas-shabda` — Word Analysis

Classification and analysis of Nepali words by origin and morphological type.

- **Word origin classification**:
  - तत्सम (Tatsam) — direct Sanskrit borrowings retaining original form
    (e.g., सत्य, विज्ञान, प्रकृति)
  - तद्भव (Tadbhav) — modified Sanskrit words following Nepali phonology
    (e.g., अद्य→आज, हस्त→हात, अग्नि→आगो)
  - देशज (Deshaj) — native Nepali words (e.g., भाका, टोपी, चुला)
  - आगन्तुक (Aagantuk) — foreign loanwords from Hindi, English, Arabic, etc.
- **Morphological decomposition**: Identify root, prefix (उपसर्ग), and suffix (प्रत्यय)
- **Gender and number markers**: Recognize पुल्लिङ्गी/स्त्रीलिङ्गी patterns and their
  vowel implications (all masculine names are hrasva; feminine names take dirgha)

#### `varnavinyas-sandhi` — Sound Change Rules

Implements sandhi operations: the rules governing what happens when morphemes combine.

- **Conjunct formation**: Encoding the combinatory rules for Nepali consonant clusters
  (e.g., क्+ष=क्ष, द्+भ=द्ध, ह्+न=ह्न, ह्+र=ह्र, ट्+ठ=ट्ठ)
- **Vowel sandhi**: Rules for vowel changes at morpheme boundaries
  (e.g., अति+अधिक=अत्यधिक, प्र+ईक्षा=प्रेक्षा)
- **Visarga sandhi**: Rules for visarga transformations
- **Prefix sandhi**: How prefixes like अप, सम्, उत्, परि attach to roots
  (e.g., उत्+लिखित=उल्लिखित, उत्+चारण=उच्चारण, सम्+अप+अङ्ग=सपाङ्ग)
- **Sandhi splitting**: Reverse operation — decomposing a combined form

#### `varnavinyas-prakriya` — Word Derivation Engine

The rule engine that traces how a word arrives at its correct form, inspired by Vidyut's
prakriya system.

- **Derivation tracing**: Step-by-step record of which rules applied to produce a word
  form, with each step citing its authority (orthography standard section number)
- **Rule application**: Mandatory and optional rule handling with decision tracking
- **Hrasva/dirgha resolution**: The core engine applying the 16+ categories of
  vowel-length rules from the orthography standard:
  - तत्सम शब्दमा संस्कृतबाट आएका यी शब्दहरू दीर्घ (words from Sanskrit that
    take dirgha based on original form)
  - ई/ऊकारको प्रयोग (when ई/ऊ matra applies based on suffix, gender, etc.)
  - प्रत्यय लागेर बनेका शब्द (suffix-derived words and their vowel rules)
- **Padavali (पदावली) rules**: Correct/incorrect usage of particles (को, का, लाई, etc.)

#### `varnavinyas-kosha` — FST-Based Compact Lexicon

A memory-efficient lexicon encoding the Nepal Academy's correct word list and the
correct/incorrect word tables from Section 4 of the orthography standard.

- **FST (Finite State Transducer) storage**: Following Vidyut's approach using the `fst`
  crate — storing words with shared prefixes and suffixes for extreme compression
- **Semantic packing**: Word metadata (origin class, gender, correct form, rule citation)
  packed into compact integer representations
- **Prefix/suffix tables**: Paradigm tables for common Nepali morphological patterns
- **Lookup**: Sub-microsecond word lookup with full metadata retrieval
- **Correct/incorrect mapping**: The ~2000+ correct↔incorrect word pairs from the
  government standard encoded as a lookup table
  (e.g., अत्याधिक→अत्यधिक, बाग्मती→बागमती, इन्स्टिच्युट→इन्स्टिच्यूट)

#### `varnavinyas-lipi` — Transliteration

Script conversion between Devanagari and romanization schemes.

- **Devanagari ↔ IAST** (International Alphabet of Sanskrit Transliteration)
- **Devanagari ↔ ISO 15919**
- **Devanagari ↔ Hunterian** (used in Indian government romanization)
- **Devanagari ↔ Nepali Romanized** (common informal romanization used online)
- **Preeti/Kantipur ↔ Unicode**: Legacy Nepali font encoding to Unicode conversion
  (critical for digitizing older documents)

#### `varnavinyas-lekhya` — Punctuation & Writing Conventions

Implements Section 5 of the orthography standard: Nepali punctuation and formatting rules.

- **Punctuation marks**: Rules for अल्पविराम (,), पूर्णविराम (।), प्रश्नवाचक (?),
  विस्मयबोधक (!), निर्देशक (:–), उद्धरण (' ' / " "), कोष्ठक (), योजक (–),
  सङ्क्षेप (.), ऐजन (,,), तिर्यक् विराम (/)
- **Padavali conventions**: When to use हलन्त vs not, when words join vs separate
- **Number formatting**: Nepali numeral conventions
- **Abbreviation rules**: सङ्क्षेप चिह्न usage (e.g., अ. दु. अ. = abbreviation patterns)

#### `varnavinyas-parikshak` — Spell Checker & Linter (Integration Crate)

The top-level integration crate that composes all lower crates into a complete spell-checking
and linting pipeline.

- **Spell checking**: Flag misspelled words with correction suggestions
- **Orthography linting**: Detect rule violations beyond simple misspelling —
  hrasva/dirgha errors, incorrect sandhi, wrong punctuation usage
- **Diagnostic reporting**: Each error includes the violated rule, correction, and
  citation from the orthography standard
- **Batch mode**: Process entire documents or corpora
- **LSP integration support**: Structured output suitable for Language Server Protocol

### Language Bindings

#### Python Bindings (PyO3)

Following Vidyut's pattern of submodule-based Python bindings:

```python
import varnavinyas

# Spell check
parikshak = varnavinyas.parikshak.Parikshak()
results = parikshak.check("यो बाक्यमा अशुध्द शब्दहरू छन्")
for error in results:
    print(f"{error.word} → {error.suggestion} (Rule: {error.rule_code})")

# Word analysis
shabda = varnavinyas.shabda.analyze("विज्ञान")
print(f"Origin: {shabda.origin}")  # Origin: Tatsam

# Transliteration
nepali = varnavinyas.lipi.transliterate("namaste", Scheme.IAST, Scheme.Devanagari)
print(nepali)  # नमस्ते
```

#### WebAssembly (wasm-pack)

```javascript
import init, { Parikshak } from 'varnavinyas-wasm';

await init();
const checker = new Parikshak();
const errors = checker.check("यो बाक्यमा अशुध्द शब्दहरू छन्");
errors.forEach(e => console.log(`${e.word} → ${e.suggestion}`));
```

#### UniFFI (Kotlin / Swift / Ruby)

```kotlin
// Android / Kotlin
val parikshak = Parikshak()
val errors = parikshak.check("यो बाक्यमा अशुध्द शब्दहरू छन्")
errors.forEach { println("${it.word} → ${it.suggestion}") }
```

#### C ABI (cbindgen)

```c
#include "varnavinyas.h"

VarnavinyasParikshak *checker = varnavinyas_parikshak_new();
VarnavinyasErrors *errors = varnavinyas_check(checker, "...");
// iterate errors...
varnavinyas_errors_free(errors);
varnavinyas_parikshak_free(checker);
```

---

## 5. Architecture Overview

### Workspace Structure

```
varnavinyas/
├── Cargo.toml                    # Workspace root
├── varnavinyas-akshar/           # Character/script utilities
│   └── src/
│       ├── lib.rs
│       ├── devanagari.rs         # Devanagari character tables
│       ├── vowel.rs              # Hrasva/dirgha classification
│       ├── syllable.rs           # Syllable segmentation
│       └── normalize.rs          # Unicode normalization
├── varnavinyas-shabda/           # Word analysis
│   └── src/
│       ├── lib.rs
│       ├── origin.rs             # Tatsam/tadbhav/deshaj classification
│       ├── morphology.rs         # Root/prefix/suffix decomposition
│       └── gender.rs             # Gender marker analysis
├── varnavinyas-sandhi/           # Sound change rules
│   └── src/
│       ├── lib.rs
│       ├── conjunct.rs           # Consonant cluster rules
│       ├── vowel_sandhi.rs       # Vowel combination rules
│       └── prefix.rs             # Prefix attachment rules
├── varnavinyas-prakriya/         # Derivation engine
│   └── src/
│       ├── lib.rs
│       ├── prakriya.rs           # Core derivation state + history
│       ├── rule.rs               # Rule abstraction with citations
│       ├── hrasva_dirgha.rs      # Vowel-length resolution engine
│       └── step.rs               # Step recording (rule tracing)
├── varnavinyas-kosha/            # FST lexicon
│   └── src/
│       ├── lib.rs
│       ├── kosha.rs              # FST-backed word store
│       ├── packer.rs             # Semantic data packing
│       └── builder.rs            # Lexicon construction
├── varnavinyas-lipi/             # Transliteration
│   └── src/
│       ├── lib.rs
│       ├── scheme.rs             # Transliteration scheme definitions
│       └── legacy.rs             # Preeti/Kantipur font conversion
├── varnavinyas-lekhya/           # Punctuation & writing conventions
│   └── src/
│       ├── lib.rs
│       ├── punctuation.rs        # Punctuation rules
│       └── padavali.rs           # Word-joining conventions
├── varnavinyas-parikshak/        # Spell checker (integration crate)
│   └── src/
│       ├── lib.rs
│       ├── checker.rs            # Spell check pipeline
│       ├── linter.rs             # Orthography linting
│       └── diagnostic.rs         # Error reporting with citations
├── bindings-python/              # PyO3 bindings
│   └── src/
│       ├── lib.rs                # Top-level module registration
│       ├── parikshak.rs          # Python spell checker API
│       ├── shabda.rs             # Python word analysis API
│       └── lipi.rs               # Python transliteration API
├── bindings-wasm/                # wasm-pack bindings
├── bindings-uniffi/              # UniFFI bindings (Kotlin/Swift)
└── bindings-c/                   # C ABI via cbindgen
```

### Crate Dependency Graph

```
varnavinyas-parikshak (integration / spell checker)
├── varnavinyas-prakriya (derivation engine)
│   ├── varnavinyas-akshar (character utilities)
│   ├── varnavinyas-shabda (word analysis)
│   │   └── varnavinyas-akshar
│   └── varnavinyas-sandhi (sound changes)
│       └── varnavinyas-akshar
├── varnavinyas-kosha (FST lexicon)
│   └── varnavinyas-akshar
├── varnavinyas-lekhya (punctuation)
│   └── varnavinyas-akshar
└── varnavinyas-lipi (transliteration)
    └── varnavinyas-akshar

bindings-python
├── varnavinyas-parikshak
├── varnavinyas-kosha
├── varnavinyas-lipi
└── (all transitive deps)
```

### Data Flow: Spell-Check Operation

When a user submits text like `"यो बाक्यमा अशुध्द शब्दहरू छन्"` for checking:

```
Input text
    │
    ▼
[varnavinyas-akshar] ── Tokenize into words, normalize Unicode
    │
    ▼
[varnavinyas-kosha] ── Look up each word in FST lexicon
    │                    ├── Found & correct → PASS
    │                    ├── Found in incorrect table → map to correction
    │                    └── Not found → continue analysis
    │
    ▼
[varnavinyas-shabda] ── Classify word origin (tatsam/tadbhav/deshaj)
    │                    Decompose into root + affixes
    │
    ▼
[varnavinyas-prakriya] ── Apply orthography rules to determine correct form
    │                      ├── Hrasva/dirgha rules (16 categories)
    │                      ├── Sandhi rules (via varnavinyas-sandhi)
    │                      └── Prefix/suffix conventions
    │                      Each step recorded with rule citation
    │
    ▼
[varnavinyas-lekhya] ── Check punctuation and formatting
    │
    ▼
[varnavinyas-parikshak] ── Compile diagnostics
    │                       Each diagnostic contains:
    │                       - Position in text
    │                       - Incorrect form
    │                       - Suggested correction
    │                       - Rule code + citation
    │                       - Explanation (in Nepali)
    │
    ▼
Output: List of diagnostics
```

---

## 6. Data Strategy

### Code-Driven Rules with Citations

Following Vidyut's approach, orthography rules are encoded directly in Rust — not in
external data files. Each rule function cites its source:

```rust
/// Orthography Standard, Section 3(क): ह्रस्वदीर्घ वर्ण र मात्रा
/// Rule: All masculine names (पुलिङ्गी नाम) take hrasva vowel.
/// Examples: दाजु, बाबु, भिनाजु, फुपाजु, भाइ, गुरु, गोरु
pub(crate) fn rule_3ka_masculine_names(prakriya: &mut Prakriya) -> bool {
    // ...
}
```

This approach ensures:

- **Auditability**: Every rule can be traced to its authoritative source
- **No external data dependencies**: Rules compile into the binary
- **Type safety**: The Rust compiler catches rule logic errors at compile time
- **Testability**: Each rule is independently unit-testable against known correct/incorrect
  pairs from the standard

### Rule Abstraction

Modeled after Vidyut's `Rule` enum, our rule system attributes each operation to its source:

```rust
pub enum Rule {
    /// Nepal Academy Orthography Standard section reference
    /// e.g., "3(क)" for hrasva/dirgha vowel rules
    VarnaVinyasNiyam(&'static str),

    /// Nepal Academy Grammar reference
    Vyakaran(&'static str),

    /// Specific word table entry from Section 4
    /// (correct/incorrect word pairs)
    ShuddhaAshuddha(&'static str),

    /// Punctuation rule from Section 5
    ChihnaNiyam(&'static str),
}
```

### Embedded Lookup Tables

The correct/incorrect word tables from Section 4 of the orthography standard (~2000+ entries)
are embedded as compile-time data:

```rust
// Generated from Section 4: शुद्ध-अशुद्ध शब्द तालिका
static SHUDDHA_ASHUDDHA: &[(&str, &str)] = &[
    ("अत्यधिक", "अत्याधिक"),      // correct, incorrect
    ("बागमती", "बाग्मती"),
    ("पुनरवलोकन", "पुनरावलोकन"),
    ("उल्लिखित", "उल्लेखित"),
    // ... 2000+ entries
];
```

### FST Lexicon Construction

The kosha crate follows Vidyut's FST pattern for the main word store:

- Words stored in a `fst::Map` with semantic metadata packed into 64-bit values
- A `Packer` maps between packed integers and full semantic records
  (word origin, gender, number, correct form reference)
- Duplicate handling via 2-byte key extension (supporting up to 4,225 homographs)
- Target: entire Nepali lexicon in <50MB with sub-microsecond lookup

---

## 7. Use Cases

### Editor Plugin (LSP)

A Language Server Protocol implementation powered by `varnavinyas-parikshak`:

- Real-time spell checking with red underlines in VS Code, Neovim, etc.
- Hover to see rule explanation and citation
- Quick-fix actions to apply corrections
- Works offline, runs locally

### Web Application

A browser-based Nepali writing assistant using the WASM bindings:

- Paste or type Nepali text, get instant orthography feedback
- Educational mode showing *why* each correction is needed
- No server required — everything runs in the browser
- Deployable as a static site

### API Service

For organizations processing Nepali documents at scale:

- REST/gRPC API wrapping the Rust core
- Batch document checking
- Integration with document management systems
- Government agencies ensuring compliance with orthography standards

### Educational Tool

For students learning correct Nepali writing:

- Interactive exercises based on the orthography standard
- Progressive difficulty following the standard's section structure
- Derivation visualization showing how rules produce correct forms
- Available offline for schools with limited connectivity

### Document Processing Pipeline

For publishers, media houses, and government offices:

- Batch check entire document corpora
- Generate correction reports
- Automated pre-publication orthography review
- Legacy font (Preeti/Kantipur) to Unicode conversion pipeline

### Accessibility

- Screen reader integration: ensure correct pronunciation by ensuring correct spelling
- Input method support: help users type correct Nepali
- Mobile keyboard integration via UniFFI bindings

---

## 8. Phased Roadmap

### Phase 0: Foundation (Months 1–3)

**Goal**: Working character utilities and project infrastructure.

- [ ] Initialize Rust workspace with all crate stubs
- [ ] Implement `varnavinyas-akshar`: full Devanagari character classification,
      hrasva/dirgha vowel tables, syllable segmentation
- [ ] Implement `varnavinyas-lipi`: Devanagari ↔ IAST transliteration,
      Preeti/Kantipur → Unicode conversion
- [ ] Set up CI/CD pipeline, benchmarks, property-based testing
- [ ] Publish initial crates to crates.io

### Phase 1: Core Rules Engine (Months 3–7)

**Goal**: Rule engine capable of hrasva/dirgha correction with full tracing.

- [ ] Implement `varnavinyas-shabda`: word origin classification using the
      tatsam identification rules from Section 2 of the standard
- [ ] Implement `varnavinyas-sandhi`: conjunct consonant formation rules,
      prefix attachment rules
- [ ] Implement `varnavinyas-prakriya`: core derivation engine with step
      recording, rule abstraction, hrasva/dirgha resolution (all 16 categories)
- [ ] Encode correct/incorrect word tables from Section 4 as test fixtures
- [ ] Python bindings (PyO3) for akshar, shabda, sandhi, prakriya

### Phase 2: Lexicon & Spell Checker (Months 7–11)

**Goal**: Working spell checker with lexicon lookup and rule-based correction.

- [ ] Implement `varnavinyas-kosha`: FST-based lexicon with semantic packing,
      embed correct/incorrect word tables
- [ ] Implement `varnavinyas-lekhya`: punctuation rules from Section 5
- [ ] Implement `varnavinyas-parikshak`: spell-check pipeline composing all crates,
      diagnostic reporting with rule citations
- [ ] Python bindings for kosha, lekhya, parikshak
- [ ] WASM bindings (wasm-pack) for browser deployment

### Phase 3: Ecosystem & Bindings (Months 11–15)

**Goal**: Full binding coverage and editor integration.

- [ ] LSP server implementation for editor integration
- [ ] VS Code extension
- [ ] UniFFI bindings for Kotlin/Swift (Android/iOS)
- [ ] C ABI via cbindgen
- [ ] Web application (static site using WASM bindings)
- [ ] CLI tool for batch document checking

### Phase 4: Refinement & Community (Months 15–18)

**Goal**: Production-quality toolkit with community governance.

- [ ] Performance optimization: benchmark against real document corpora
- [ ] Expand lexicon coverage with community contributions
- [ ] Handle dialectal variations and stylistic preferences
- [ ] Documentation: comprehensive API docs, tutorials, and guides in Nepali
- [ ] Linguistic advisory group for rule validation
- [ ] Explore advanced features: grammar checking beyond orthography,
      style suggestions, readability analysis

---

## 9. Success Metrics

### Rule Coverage

| Metric | Target |
|--------|--------|
| Hrasva/dirgha rules from Section 3 encoded | 100% of 16 categories |
| Correct/incorrect word pairs from Section 4 | 100% of ~2000+ entries |
| Punctuation rules from Section 5 | 100% of 14 mark types |
| Sandhi rules (conjunct formation) | 100% of standard combinations |
| Prefix/suffix rules | All patterns documented in Sections 3–4 |

### Performance

| Operation | Target |
|-----------|--------|
| Single word lookup (kosha) | < 1 μs |
| Single word spell-check | < 100 μs |
| Full document check (1000 words) | < 50 ms |
| Lexicon memory footprint | < 50 MB |
| WASM bundle size | < 5 MB (gzipped) |

### Accuracy

| Metric | Target |
|--------|--------|
| Precision on Academy word tables | 100% (these are ground truth) |
| Hrasva/dirgha correction accuracy | > 95% on real-world text |
| False positive rate | < 2% |

### Adoption (18-month targets)

| Metric | Target |
|--------|--------|
| crates.io downloads | 1,000+ |
| PyPI downloads | 5,000+ |
| npm downloads (WASM) | 2,000+ |
| GitHub stars | 500+ |
| Contributors | 20+ |
| Editor plugin installs | 1,000+ |

---

## 10. Community & Governance

### License

**MIT License** — maximizing adoption. The orthography rules are public knowledge published
by the Nepal government; encoding them in software should be freely available to all.

### Contribution Guidelines

- All rule implementations must cite their source (standard section, grammar reference)
- Correct/incorrect examples from the standard serve as mandatory test cases
- Performance benchmarks must not regress
- Code review by at least one maintainer with Nepali linguistic knowledge
- Documentation in both English and Nepali

### Linguistic Advisory Group

A panel of Nepali language experts to:

- Validate rule implementations against the standard
- Resolve ambiguities where the standard is unclear
- Advise on dialectal variations and evolving usage
- Review community-contributed rules and word lists

### Communication

- GitHub Issues for bug reports and feature requests
- GitHub Discussions for linguistic questions and design decisions
- Periodic community calls for roadmap alignment

---

## 11. Inspirations & Acknowledgements

- **[Vidyut](https://github.com/ambuda-org/vidyut)** by Ambuda — the architectural inspiration for
  this project. Varnavinyas adapts Vidyut's workspace structure, FST lexicon approach,
  rule tracing pattern, and Python binding strategy for the Nepali language context.
- **Nepal Academy (नेपाल प्राज्ञा प्रतिष्ठान)** — for publishing the orthography standard
  that serves as our primary rule source.
- **Dr. Devi Nepal** — author of the नेपाली वर्णविन्यास guide published through
  प्राज्ञा परिषद् सदस्य, नेपाल प्राज्ञा-प्रतिष्ठान, काठमाडौँ.

---

*वर्णविन्यास — शुद्ध नेपाली, सबैका लागि।*
*Varnavinyas — Correct Nepali, for everyone.*
