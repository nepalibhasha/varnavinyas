# वर्णविन्यास (Varnavinyas)

**The definitive open-source Nepali language infrastructure toolkit.**

*शुद्ध नेपाली भाषाको लागि मुक्त-स्रोत प्रविधि*
*(Open-source technology for correct Nepali language)*

---

## 1. The Problem

Nepali is spoken by over 32 million people, yet it lacks the basic computational language infrastructure that languages like English or Sanskrit enjoy.

*   **No open-source orthography engine**: The official *Nepali Orthography Standard* (published by Nepal Academy) exists only as a PDF ruleset, inaccessible to machines.
*   **The Hrasva/Dirgha Crisis**: Errors in short/long vowels (इ/ई, उ/ऊ) are pervasively common, yet no tool exists to correct them based on grammatical rules.
*   **Fragmented Knowledge**: Linguistic rules are scattered across textbooks and expert intuition, not unified in code.

## 2. Vision

**Varnavinyas aims to be the digital foundation for the Nepali language.**

We are building a complete, high-performance toolkit that encodes the Nepal Academy's standards into efficient, portable software. Just as *Vidyut* did for Sanskrit, *Varnavinyas* provides the building blocks—spell checkers, grammar engines, transliterators—that every Nepali application needs.

## 3. Principles

*   **शुद्धता (Fidelity)**: Every rule traces back to the Nepal Academy standard. We do not invent rules; we encode authority.
*   **गति (Performance)**: Operations should be measured in microseconds. Suitable for real-time editors and large-scale data processing.
*   **सर्वव्यापकता (Portability)**: Written in Rust, deployed everywhere (Python, WebAssembly, Mobile, C).
*   **स्वतन्त्र (Offline-First)**: Zero runtime dependencies. No API calls. Works in remote schools and secure government offices.
*   **पारदर्शिता (Transparency)**: Not a black box. The system explains *why* a word is wrong by citing the specific rule it violated.

## 4. Scope

Varnavinyas is a monorepo workspace providing:

*   **Character Utilities**: Devanagari analysis, Unicode normalization (`akshar`).
*   **Lexicon**: FST-based comprehensive word store (`kosha`).
*   **Rule Engine**: Derivation logic for spelling and grammar (`prakriya`, `sandhi`, `shabda`).
*   **Spell Checker**: High-level diagnostic pipelining (`parikshak`).
*   **Transliteration**: Script conversion (`lipi`).

---

**See Also:**
*   [STATUS.md](STATUS.md) for current feature support.
*   [BACKLOG.md](BACKLOG.md) for upcoming priorities.
*   [ARCHITECTURE.md](ARCHITECTURE.md) for technical design.
