# Varnavinyas Feature Status

> **Last Updated**: 2026-02-19

## Core Features

| Crate | Feature | Status | Test Coverage |
|-------|---------|--------|---------------|
| **akshar** | Character classification | ✅ Stable | gold.toml |
| **lipi** | Devanagari ↔ IAST | ✅ Stable | Round-trip |
| **lipi** | Preeti/Kantipur Legacy | ✅ Stable | Unit tests |
| **kosha** | FST Lexicon (~51k words) | ✅ Stable | Section 4 tables |
| **prakriya** | Hrasva/Dirgha Rules | 🚧 In Progress | gold.toml (Partial) |
| **prakriya** | Rule Tracing | ✅ Implemented | - |
| **lekhya** | Punctuation (Section 5) | ✅ Implemented | Unit tests |
| **parikshak** | Spell Check Pipeline | ✅ Stable | Integration |

## Bindings & Tools

| Component | Status | Notes |
|-----------|--------|-------|
| **CLI** | ✅ Beta | Basic checking works |
| **WASM** | ✅ Alpha | Browser bindings active |
| **Python** | 🚧 WIP | `varnavinyas` package stub |
| **LSP** | 🚧 WIP | Editor integration scaffolded |
| **Vyakaran** | 🚧 MVP | Basic morphology analysis |
