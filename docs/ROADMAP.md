# Varnavinyas — Demo-Driven Roadmap

> **Implementation guide**: The [PRD](PRD.md) is the master implementation document for Varnavinyas.
> It provides concrete specifications, architecture details, and step-by-step build instructions.
> This ROADMAP remains the authoritative source for **test data** (gold.toml references)
> and **demo scenarios**. The PRD references this document for test fixtures.

Every phase ends with a **concrete, runnable demo** — not just "crate stubs done, trust me."
You should be able to feed in sample Nepali text and see mistakes identified, with rule
citations explaining *why* each word is wrong.

## Guiding Principle

> At any point in this project, I should be able to give you a Nepali sentence with mistakes
> and you should tell me: (1) which words are wrong, (2) what the correct form is,
> (3) which rule was broken, and (4) why.

The scope of what we can catch grows with each phase, but the **explain-ability** is there
from day one.

---

## Test Data Integrity

All test fixtures are split into two verified datasets:
- **`docs/tests/gold.toml`** — High-confidence pairs with page references (91 entries)
- **`docs/tests/needs_review.toml`** — Disputed/ambiguous pairs needing manual verification (21 entries)

Every entry includes a `page` field referencing `docs/Notices-pages-77-99.pdf` so it can be
manually verified. Phase 1 builds against `gold.toml` ONLY. Needs-review pairs are resolved
incrementally and promoted to gold as they are confirmed.

**Audit summary (from cross-referencing subdocs against the Academy standard):**
- 5 WRONG entries in original subdoc fixtures → fixed below
- 4 internal contradictions → resolved below
- 21 ambiguous/context-dependent entries → moved to `needs_review.toml`

---

## Test Fixtures: Source Material

All test data comes from official government sources, ensuring our ground truth is
authoritative:

### Primary Source: Nepal Academy Orthography Standard
`docs/Notices-pages-77-99.pdf` — The canonical rule book.

### Training Materials (from LDTA शुद्ध लेखन प्रशिक्षण)
Located in `docs/subdocs/`:

| File | Contents | Test Value |
|------|----------|------------|
| `_01-prashikshan-margadarshan.pdf` | Training guide, session structure | Context for rule ordering |
| `_02-prashikshan-yojana.pdf` | Session plans by topic | Rule groupings by difficulty |
| `_03-satra-yojana-abhyas-patra.pdf` | Session exercises & worksheets | Exercise-format test cases |
| `_04-prastuti-samagri-slides.pdf` | Presentation slides with examples | Rich correct/incorrect pairs by rule category |
| `_05-sahabhagi-adhyayan-samagri.pdf` | Participant study materials | Complete rule reference with examples |
| `_06-mulyankan-aujhar.pdf` | **Evaluation exam (पूर्णाङ्क: ५०)** | Gold-standard test suite |

### Key Test Cases from `_06` (Exam Paper)

**Q3 — Identify the correct form (शुद्ध रूप):**
```
बुढो → बूढो  ← (NEEDS_REVIEW: compensatory lengthening, see p5)
मीठो → मिठो  ← (FIXED: was "मिठा", correct target is मिठो, p5)
अगाडि → अगाडी  (p7, postposition dirgha)
पहाडि → पहाडी  (p7, adjectival dirgha)
मुख्याई → मुख्याईँ  ← (NEEDS_REVIEW: chandrabindu placement, see p9)
पुर्वेली ← correct form (FIXED: पूर्वेली is wrong, suffix -एली triggers hrasva, p6-7)
जातीय ← correct form (जातिय is wrong, -ईय suffix preserves dirgha, p7)
सभापती → सभापति  ← (NEEDS_REVIEW: context-dependent masculine/feminine, see p7)
स्विकार्नु ← correct form (FIXED: स्वीकार्नु is wrong, suffix -नु triggers hrasva, p6-7)
दुध → दूध  ← (NEEDS_REVIEW: compensatory lengthening, see p5)
```

**Q5 — Multiple choice: कुन शब्द शुद्ध हो?**
```
(क) प्रशासन ✓   (vs प्रसाशन, प्रषाशन, प्रसापन)
(ख) फाउण्डेशन ✓  (vs फाउन्डेसन, फाउण्डेसन, फाउन्डेषन)  ← NOTE: p18 table lists फाउण्डेसन→फाउन्डेसन; exam answer conflicts with Academy standard
(ग) ऋषिमुनि ✓   (vs ऋषिमुनी, रिषिमुनि, ऋषिमूनि)
(घ) विवेकशील ✓   (vs विवेकशिल, विवेक्शीन, विवेकसील)
(ङ) सुसूचित ✓    (vs शुसूचित, सुसुचित, सुशूचित)
(च) विदेशियो  ← (all options given: विदेशियो, विदेसियो, विदेशिय, विदेषियो)
```

**Q9 — Correct form pairs (शुद्ध रूप चिन्नुहोस्):**
```
उपरोक्त → उपर्युक्त/? (p18)  ← (NEEDS_REVIEW: standard lists two correct forms)
धैर्यता → धीरता/धैर्य ✓ (p18)  (two acceptable corrections)
अत्याधिक → अत्यधिक ✓ (p18)
व्यवहारिक → व्यावहारिक ✓ (p18)  (इक suffix triggers आदिवृद्धि: अ→आ)
राजनैतिक → राजनीतिक ✓ (p18)   उल्लेखित → उल्लिखित ✓ (p18)
पुनरावलोकन → पुनरवलोकन ✓ (p18)  व्यहोरा → बेहोरा ← (NEEDS_REVIEW: both forms exist, p18)
```

**Q10 — Paragraph with errors to correct:**
```
राम स्याम सिता गीता वगैचामा खेल्दै थिएँ त्यहाँ स्यामले पुतलि देखेर भन्यो आहा कति राम्रो
पूतली म समाऔँ सीताले भनीन अहँ त्यसो गर्नु हुँदैन बगैचा नै उस्को घर हो तिमि भनेको
मान्दैनौँ न मान तर सबै ले आफ्नो घर सन्सारमा आत्म सुरक्षाका अनुभूती गरेको हुन्छन् जस्तै
हामी हाम्रो घरमा रमाएका हुन्छौ
```

### Key Test Cases from `_04` (Slides)

**Session 4 — Hrasva/Dirgha (ह्रस्वदीर्घ) pairs:**
```
# Words that take HRASVA (इ/उ) (p5):
सिप (शिल्प), मिठो, ठुलो, तितो, मित, पिर, खिर, बिउ,
धुलो, भुल, भिड, भिर, धुवाँ, भिख, पिठो, पिरो, बिच, ठिक, बेठिक
# NEEDS_REVIEW — moved to needs_review.toml (compensatory lengthening ambiguity on p5):
# दुध (दुग्ध), बुढो (वृद्ध) — may take dirgha as exception

# Words that take DIRGHA (ई/ऊ):
मूल, जीवन, दीपक, कूप, शूल, श्रीषण, भूगोल, भूमि, सूचना, सूक्त, सूक्ति
उल्लेखनीय, केन्द्रीय, मननीय, नवीन, युगीन, एकीकरण, केन्द्रीकरण, समीभवन,
भष्मीभूत, एकीकृत

# Suffix-derived DIRGHA (p7):
पूर्व+ई/ईय = पूर्वी, पूर्वीय  (suffix -ई/-ईय preserves dirgha)
# Suffix-derived HRASVA (p6-7):
पूर्व+ए/एली = पुर्वे, पुर्वेली  (suffix -ए/-एली triggers hrasva)
मूर्ख+ता = मूर्खता
मूर्ख+याइँ = मुर्ख्याईँ
स्वीकार+य = स्वीकार्य  (suffix -य preserves dirgha)
स्वीकार+नु = स्विकार्नु  (suffix -नु triggers hrasva)
```

**Session 5 — Chandrabindu/Shirbindu/Panchham Varna pairs:**
```
# Correct → Incorrect (p8-9)
सिंह → सिँह ← (सिंह is correct — tatsam uses shirbindu, p9)
संवाद → सँवाद ← (संवाद is correct — tatsam uses shirbindu, p9)
सांस्कृतिक → साँस्कृतिक ← (NEEDS_REVIEW: both common, p9)
जान्छौँ → जान्छौ ← (chandrabindu required, p9)
आउँछ → आउछ ← (chandrabindu required, p9)
भएछु → भएँछु ← (NEEDS_REVIEW: chandrabindu placement ambiguous, p9)
संचालन → सञ्चालन ← (NEEDS_REVIEW: shirbindu vs panchham debate, p8-9)
संपूर्ण → सम्पूर्ण ← (NEEDS_REVIEW: both exist; context-dependent, p8-9)
```

**Session 6 — श/ष/स pairs:**
```
शासन → सासन (incorrect)     शेष → सेष (incorrect)
इन्ष्टिच्युट → इन्स्टिच्युट (p18: ष→स, no dirgha ऊ; Session 6 had wrong direction)
एशिया → एसिया (incorrect)
ऋतु → रितु (incorrect)       ऋषि → रिषि (incorrect)
कृति → क्रिति (incorrect)     कृस्चियन → क्रिस्चियन
```

**Session 7 — य/ए, क्ष/छ्य, क्षे/छे, हलन्त/अजन्त pairs:**
```
एथार्थ → यथार्थ ← (incorrect)  यकिन → एकिन ← (both wrong forms)
गरिएको → गरियेको             एकता → यकता (incorrect)
लक्ष्य → लछ्य ← (trick)       छेत्र → क्षेत्र ✓
इच्छा → इक्षा (incorrect)     अर्थात् → अर्थात (halanta required)
```

**Session 8 — Padayog (पदयोग) pairs:**
```
घरभित्र → घर भित्र ← (should be one word? context-dependent)
आज्ञाअनुसार → आज्ञा अनुसार
सामाजिकस्थिति → सामाजिक स्थिति ← (should be two words)
एकताबद्ध → एकता बद्ध
```

### Paragraph-Level Test Fixtures

**From slides (क्रियाकलाप — Activity):**
```
INPUT (with errors):
तिथीमीति मीलेको पात्रो घरमा छ भने तिमि
हामि सबैको दैनीकी ठीक हुन्छ । दीदी, बहीनी,
भाउजु, फुपु सबैले मीठो दहिचाहीँ खाऊन् तर
पीरो खुर्सानि मूखमा नहालून् भनि दाजु, भाई,
सम्धि र जोगिले साथिसँग भनेको कुरा गायीका,
सम्धिनि र मीतिनिले सुने छन् ।

EXPECTED CORRECTIONS:
तिथीमीति → तिथिमिति    (hrasva: tadbhav)
मीलेको → मिलेको       (hrasva: tadbhav verb)
हामि → हामी           (dirgha: pronoun)
दैनीकी → दैनिकी       (hrasva in middle)
दीदी → दिदी           (hrasva: kinship tadbhav)
बहीनी → बहिनी         (hrasva: kinship tadbhav)
भाउजु → भाउजू        (dirgha: kinship)
फुपु → फुपू            (dirgha: kinship)
मीठो → मिठो           (hrasva: tadbhav)
दहिचाहीँ → [context]
पीरो → पिरो           (hrasva: tadbhav)
खुर्सानि → खुर्सानी     (dirgha: feminine noun)
मूखमा → मुखमा         (hrasva: tadbhav of मुख)
नहालून् → [context]
भनि → भनी            (dirgha: absolutive)
भाई → भाइ            (hrasva: kinship tadbhav)
गायीका → गाईका        (vowel form)
सम्धिनि → सम्धिनी     (dirgha: feminine)
मीतिनिले → मितिनीले    (mixed hrasva/dirgha)
```

**From exam Q10 (paragraph correction):**
```
INPUT:
राम स्याम सिता गीता वगैचामा खेल्दै थिएँ त्यहाँ स्यामले पुतलि देखेर भन्यो आहा कति राम्रो
पूतली म समाऔँ सीताले भनीन अहँ त्यसो गर्नु हुँदैन बगैचा नै उस्को घर हो तिमि भनेको
मान्दैनौँ न मान तर सबै ले आफ्नो घर सन्सारमा आत्म सुरक्षाका अनुभूती गरेको हुन्छन् जस्तै
हामी हाम्रो घरमा रमाएका हुन्छौ

KEY ERRORS:
वगैचामा → बगैँचामा     (ब not व; chandrabindu)
पुतलि → पुतली          (dirgha: feminine noun)
पूतली → पुतली          (hrasva: tadbhav)
सीताले → सीताले        (correct as-is, tatsam name)
भनीन → भनिन्          (hrasva + halanta)
बगैचा → बगैँचा         (chandrabindu)
सन्सारमा → संसारमा     (shirbindu, not na-halanta)
अनुभूती → अनुभूति      (hrasva: tatsam ending)
```

---

## Phase 0: "I Can Read Devanagari" (Months 1–2)

### What You Can Demo

Feed in any Nepali text → get back structured character-level analysis.

```
$ varnavinyas akshar analyze "विज्ञान"

Characters:
  व (व्यञ्जन, ka-varga)
  ि (मात्रा, hrasva-i)
  ज (व्यञ्जन, cha-varga)
  ् (हलन्त)
  ञ (व्यञ्जन, cha-varga, panchham)
  ा (मात्रा, dirgha-aa)
  न (व्यञ्जन, ta-varga, panchham)

Syllables: वि · ज्ञा · न
Conjuncts: ज्ञ (ज् + ञ)
```

```
$ varnavinyas akshar hrasva-dirgha "पुतली"
  पु: उ-मात्रा (hrasva)
  ली: ई-मात्रा (dirgha)
```

### Deliverables

- [ ] Rust workspace initialized with all crate stubs (compiles, tests pass)
- [ ] `varnavinyas-akshar` implemented:
  - Full Devanagari character table (स्वर, व्यञ्जन, मात्रा, चिह्न)
  - Hrasva/dirgha vowel classification (इ↔ई, उ↔ऊ, matra forms)
  - Syllable segmentation
  - Conjunct decomposition (ज्ञ → ज्+ञ, क्ष → क्+ष, etc.)
  - Unicode normalization
- [ ] `varnavinyas-lipi` implemented:
  - Devanagari ↔ IAST transliteration
  - Preeti/Kantipur → Unicode conversion
- [ ] CLI tool: `varnavinyas akshar analyze <text>`
- [ ] CI/CD pipeline running
- [ ] Test suite: character classification tests from orthography standard Section 3(क)

### Test Fixture Coverage

From the standard:
- All Devanagari vowels/consonants classified correctly
- Conjunct decomposition for all combinations in Section 3(ख):
  क्ष, ज्ञ, त्र, क्त, द्ध, ह्न, ह्र, ट्ठ, etc.

---

## Phase 1: "I Know Which Words Are Wrong" (Months 2–5)

### What You Can Demo

Feed in the exam paper's correct/incorrect pairs → get a lookup verdict for each.

```
$ varnavinyas kosha check "अत्याधिक"

❌ अत्याधिक
   Correct form: अत्यधिक
   Rule: ShuddhaAshuddha(Section-4)
   Source: Nepal Academy Orthography Standard, शुद्ध-अशुद्ध तालिका
```

```
$ varnavinyas kosha check "प्रशासन"
✅ प्रशासन — शुद्ध
```

```
$ varnavinyas kosha check "प्रसाशन"
❌ प्रसाशन
   Correct form: प्रशासन
   Rule: ShuddhaAshuddha(Section-4), श/ष/स-नियम
```

### Deliverables

- [ ] `varnavinyas-kosha` implemented:
  - FST-based word store with all ~2000+ correct/incorrect pairs from Section 4
  - Lookup returns: is_correct, suggested_correction, rule_citation
  - Semantic packing for word metadata (origin class, etc.)
- [ ] `varnavinyas-shabda` — basic word origin classification:
  - Given a word, classify as तत्सम/तद्भव/देशज/आगन्तुक
  - Based on tatsam identification rules from Section 2
    (श/ष usage = tatsam, ऋ/ऋ usage = tatsam, conjunct patterns, etc.)
- [ ] CLI tool: `varnavinyas check <word>` and `varnavinyas check --file <path>`
- [ ] Python bindings for kosha + shabda
- [ ] Test suite: All pairs from `docs/tests/gold.toml` pass; Q5 and Q9 subsets verified

### Test Fixture Coverage

Test data sourced from `docs/tests/gold.toml`. NEEDS_REVIEW pairs in
`docs/tests/needs_review.toml` are excluded until manually verified.

**Must pass (from exam Q5):**
- `प्रशासन` → ✅ correct
- `प्रसाशन` → ❌ (श/ष confusion)
- `ऋषिमुनि` → ✅ correct
- `रिषिमुनि` → ❌ (ऋ→रि incorrect)
- `विवेकशील` → ✅ correct
- `विवेकशिल` → ❌ (hrasva/dirgha)

**Must pass (from exam Q9):**
- `उपर्युक्त` → ✅ (not उपरोक्त)
- `अत्यधिक` → ✅ (not अत्याधिक)
- `राजनीतिक` → ✅ (not राजनैतिक)
- `उल्लिखित` → ✅ (not उल्लेखित)

---

## Phase 2: "I Can Explain WHY It's Wrong" (Months 5–8)

### What You Can Demo

Feed in a word → get a rule-traced derivation explaining the correction.

```
$ varnavinyas prakriya explain "मीठो"

❌ मीठो → मिठो
   Derivation trace:
   1. मीठो — input word
   2. Root: मिष्ट (Sanskrit: sweet) — tatsam origin
   3. Tadbhav transformation: मिष्ट → मिठो
      Rule: VarnaVinyasNiyam("3(क)-12")
      "क्षतिपूर्ति दीर्घीभवन: ह्रस्व — अनेक अर्थ नलाग्ने (एउटा मात्र अर्थ दिने)
       शब्दमा इकार/उकार ह्रस्व"
   4. मीठो uses दीर्घ ई-मात्रा, but tadbhav words with single meaning take ह्रस्व
   5. Correct form: मिठो (with ह्रस्व इ-मात्रा)
```

```
$ varnavinyas prakriya explain "सांस्कृतिक"

⚠️ सांस्कृतिक → साँस्कृतिक (preferred) OR सांस्कृतिक (acceptable)
   Derivation trace:
   1. Root: संस्कृति (tatsam)
   2. Rule: VarnaVinyasNiyam("3(ख)")
      "चन्द्रविन्दुको प्रयोग तत्सम शब्दमा हुँदैन, तद्भव र आगन्तुक शब्दमा मात्र हुन्छ"
   3. But: सांस्कृतिक retains shirbindu (ं) — tatsam form
   4. Tadbhav adaptation uses chandrabindu (ँ) → साँस्कृतिक
   5. Note: Both forms in common use; standard prefers shirbindu for tatsam
```

### Deliverables

- [ ] `varnavinyas-prakriya` implemented:
  - Prakriya (derivation) state with step history (modeled on Vidyut)
  - Rule enum with Nepal Academy citations
  - Hrasva/dirgha resolution engine (all 16 categories from Section 3)
  - Step-by-step trace output
- [ ] `varnavinyas-sandhi` implemented:
  - Conjunct formation rules (Section 3(ख) — all combinations)
  - Prefix sandhi (उत्+लिखित=उल्लिखित, etc.)
  - Vowel sandhi at morpheme boundaries
- [ ] Enhanced `varnavinyas-shabda`:
  - Morphological decomposition (root + prefix + suffix)
  - Gender marker detection for dirgha rules
- [ ] CLI tool: `varnavinyas explain <word>`
- [ ] Python bindings with history access
- [ ] Test suite: all slide examples from sessions 3–5 (hrasva/dirgha, chandrabindu, panchham varna)

### Test Fixture Coverage

**Must explain correctly (from slides session 4):**
- `मीठो` → मिठो: hrasva because tadbhav of मिष्ट, single-meaning word (p5)
- `दुध/दूध` → NEEDS_REVIEW: compensatory lengthening ambiguity (p5, see needs_review.toml)
- `बुढो/बूढो` → NEEDS_REVIEW: compensatory lengthening ambiguity (p5, see needs_review.toml)
- `स्विकार्नु` ← correct (hrasva): suffix -नु triggers hrasva (स्वीकार + नु = स्विकार्नु, p6-7)
- `पुर्वेली` ← correct (hrasva): suffix -एली triggers hrasva (पूर्व + एली = पुर्वेली, p6-7)

**Must trace conjuncts (from standard Section 3(ख)):**
- क्ष = क् + ष, ज्ञ = ज् + ञ, द्ध = द् + ध, ह्र = ह् + र

---

## Phase 3: "I Can Check a Full Paragraph" (Months 8–12)

### What You Can Demo

Feed in the exam paragraph (Q10) or the slide activity text → get all errors flagged with
explanations.

```
$ varnavinyas check --explain << 'EOF'
तिथीमीति मीलेको पात्रो घरमा छ भने तिमि
हामि सबैको दैनीकी ठीक हुन्छ । दीदी, बहीनी,
भाउजु, फुपु सबैले मीठो दहिचाहीँ खाऊन् तर
पीरो खुर्सानि मूखमा नहालून् भनि दाजु, भाई,
सम्धि र जोगिले साथिसँग भनेको कुरा गायीका,
सम्धिनि र मीतिनिले सुने छन् ।
EOF

Found 14 errors:

 1. तिथीमीति → तिथिमिति
    Rule: VarnaVinyasNiyam("3(क)-12") — tadbhav, hrasva for single-meaning
    Category: hrasva/dirgha

 2. मीलेको → मिलेको
    Rule: VarnaVinyasNiyam("3(क)-12") — tadbhav verb root, hrasva
    Category: hrasva/dirgha

 3. हामि → हामी
    Rule: VarnaVinyasNiyam("3(ई)-1") — pronoun takes dirgha
    Category: hrasva/dirgha

 4. दैनीकी → दैनिकी
    Rule: VarnaVinyasNiyam("3(क)-12") — medial vowel hrasva
    Category: hrasva/dirgha

 5. दीदी → दिदी
    Rule: VarnaVinyasNiyam("3(इ)-kinship") — kinship tadbhav, hrasva
    Category: hrasva/dirgha

 ... (9 more errors with full traces)

Summary: 14 errors found (12 hrasva/dirgha, 1 chandrabindu, 1 vowel form)
```

### Deliverables

- [ ] `varnavinyas-parikshak` implemented:
  - Full spell-check pipeline composing all lower crates
  - Text tokenization → word lookup → rule-based analysis → diagnostic output
  - Batch mode for full documents
  - Structured diagnostic output (position, error, correction, rule, explanation)
- [ ] `varnavinyas-lekhya` implemented:
  - Punctuation rules from Section 5 (14 mark types)
  - Padavali conventions (word joining/separation rules)
- [ ] CLI tool: `varnavinyas check <text|file>` with `--explain` flag
- [ ] WASM bindings (wasm-pack) for browser deployment
- [ ] Python bindings for full parikshak pipeline
- [ ] Test suite: both paragraphs (exam Q10 + slide activity) fully passing

### Test Fixture Coverage

**Must catch all errors in exam Q10 paragraph**
**Must catch all errors in slide क्रियाकलाप paragraph**
**Must handle mixed error types (hrasva/dirgha + chandrabindu + श/ष/स)**

---

## Phase 4: "I Work in Your Editor" (Months 12–15)

### What You Can Demo

Real-time squiggly underlines in VS Code / Neovim while typing Nepali text. Hover for
rule explanation. Quick-fix to apply correction.

### Deliverables

- [ ] LSP server wrapping `varnavinyas-parikshak`
- [ ] VS Code extension published
- [ ] Web application (static site, WASM-powered, no server)
- [ ] UniFFI bindings for Kotlin/Swift
- [ ] C ABI via cbindgen
- [ ] CLI with `--format=json` for tool integration

---

## Phase 5: "Community-Ready" (Months 15–18)

### What You Can Demo

Contributors can add new rules, new word entries, and new test fixtures. The test suite
catches regressions. Documentation in Nepali makes the tool accessible to linguists.

### Deliverables

- [ ] Performance optimization (benchmark: 1000-word doc < 50ms)
- [ ] Expanded lexicon beyond the Academy's ~2000 pairs
- [ ] Documentation in Nepali + English
- [ ] Linguistic advisory group established
- [ ] Contribution guide: how to encode a new rule with citations and tests

---

## Progress Tracking

After each phase, we run the **full test suite** and report:

| Phase | Exam Q3 | Exam Q5 | Exam Q9 | Exam Q10 | Slide Activities | Paragraph Check |
|-------|---------|---------|---------|----------|-----------------|-----------------|
| 0     | —       | —       | —       | —        | —               | —               |
| 1     | partial | ✅ all   | ✅ all   | —        | —               | —               |
| 2     | ✅ all   | ✅ all   | ✅ all   | partial  | partial         | —               |
| 3     | ✅ all   | ✅ all   | ✅ all   | ✅ all    | ✅ all           | ✅ all           |
| 4     | ✅ all   | ✅ all   | ✅ all   | ✅ all    | ✅ all           | ✅ all (in-editor)|
| 5     | ✅ all   | ✅ all   | ✅ all   | ✅ all    | ✅ all           | ✅ all + community|

Each cell should be verifiable by running `cargo test` at any time.
