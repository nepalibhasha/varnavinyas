# Varnavinyas Feature Status

| Crate | Feature | Status | Test Coverage | Notes |
|-------|---------|--------|---------------|-------|
| **akshar** | Character classification | ✅ Implemented | gold.toml | Devanagari block only |
| **akshar** | Syllable segmentation | ✅ Implemented | Unit tests | |
| **lipi** | Devanagari ↔ IAST | ✅ Implemented | Round-trip | |
| **lipi** | Preeti/Kantipur | ✅ Implemented | Unit tests | Legacy font support |
| **shabda** | Origin classification | ✅ Implemented | ~26K entries | Tatsam/Tadbhav/Aagantuk |
| **sandhi** | Dirgha/Guna/Yan | ✅ Implemented | Unit tests | |
| **sandhi** | Vriddhi/Visarga | ✅ Implemented | Unit tests | New in Phase 2 |
| **sandhi** | Split algorithm | ✅ Implemented | Unit tests | General brute-force |
| **prakriya** | Correction table | ✅ Implemented | 91 gold pairs | Authoritative overrides |
| **prakriya** | Hrasva/dirgha rules | ✅ Implemented | gold.toml | |
| **prakriya** | Sibilant (active) | ✅ Implemented | gold.toml | O7a |
| **prakriya** | B/V distinction | ✅ Implemented | gold.toml | O7b (via table) |
| **prakriya** | Halanta enforcement | ✅ Implemented | gold.toml | O7c (via table) |
| **prakriya** | Ksh/Chhya | ✅ Implemented | gold.toml | O7d (via table) |
| **kosha** | FST lexicon | ✅ Implemented | 51K headwords | Fast lookup |
| **lekhya** | Punctuation rules | ✅ Implemented | Unit tests | All 14 types (O9) |
| **parikshak** | Spell checker | ✅ Implemented | Integration | |
| **parikshak** | Smart tokenizer | ✅ Implemented | Unit tests | O8 suffix-aware |
| **vyakaran** | Morphology API | 🚧 Planned | — | Phase 3 (Stub only) |
