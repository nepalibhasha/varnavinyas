# Varnavinyas (वर्णविन्यास)

**The definitive open-source Nepali language orthography toolkit.**

*शुद्ध नेपाली, सबैका लागि।*  
*(Correct Nepali, for everyone.)*

Varnavinyas is a high-performance Rust library designed to digitize the Nepal Academy's official orthography standard (नेपाली वर्णविन्यास). It provides the foundational infrastructure for spell checkers, grammar tools, and linguistic analysis.

## Features (Phase 0)

*   **Character Analysis**: Devanagari character classification (Vowel, Consonant, Matra, etc.)
*   **Syllable Segmentation**: Accurate splitting of text into Aksharas (syllables), handling complex conjuncts.
*   **Transliteration**:
    *   Devanagari ↔ IAST (International Alphabet of Sanskrit Transliteration)
    *   Legacy Font Support (Preeti/Kantipur → Unicode)
*   **Normalization**: Unicode NFC normalization for consistent text processing.

## Architecture

The project is organized as a Cargo workspace:

*   **`crates/akshar`**: Core character utilities, segmentation, and normalization.
*   **`crates/lipi`**: Transliteration and script conversion.
*   **`docs/`**: Detailed documentation, including the Product Requirements Document (PRD) and roadmap.

## Usage

### Prerequisites

*   Rust 1.85.0 or higher
*   Cargo

### Building

```bash
cargo build --workspace
```

### Testing

Run the full test suite, including gold-standard verification:

```bash
cargo test --workspace
```

### Legacy Font Support

To enable support for legacy fonts like Preeti and Kantipur, use the `legacy` feature:

```bash
cargo test -p varnavinyas-lipi --features legacy
```

## Documentation

*   [**Vision**](docs/VISION.md): Why this project exists.
*   [**PRD**](docs/PRD.md): Technical specifications and architecture.
*   [**Roadmap**](docs/ROADMAP.md): Development phases and milestones.
*   [**Rust Guide**](docs/RUST_GUIDE.md): Guide for contributors new to Rust.

## License

MIT or Apache-2.0