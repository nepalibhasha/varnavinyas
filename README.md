# Varnavinyas (वर्णविन्यास)

Open-source Nepali orthography toolkit based on Nepal Academy standards.

*शुद्ध नेपाली, सबैका लागि।*
*(Correct Nepali, for everyone.)*

**[Try it in your browser](https://nepalibhasha.github.io/varnavinyas/)** — no install required.

Varnavinyas is a Rust workspace for spell checking, orthographic normalization, punctuation diagnostics, and linguistic analysis for Nepali text.

## What It Includes

- Orthography correction pipeline with rule-based and table-based fixes (`parikshak` + `prakriya` + `kosha`)
- Punctuation checker aligned with Academy Section 5 conventions (`lekhya`)
- Devanagari text utilities: akshara splitting, normalization, transliteration, legacy font conversion (`akshar`, `lipi`)
- Sandhi, morphology/origin utilities, and evaluation harnesses (`sandhi`, `shabda`, `eval`)
- Multiple interfaces:
  - CLI (`crates/cli`)
  - LSP server (`crates/lsp`)
  - Web app with WASM bindings (`web/`, `crates/bindings-wasm`)
  - Python/C/UniFFI bindings (`crates/bindings-python`, `crates/bindings-c`, `crates/bindings-uniffi`)

## Workspace Layout

- `crates/akshar`, `crates/lipi`, `crates/shabda`, `crates/types`, `crates/sandhi`, `crates/prakriya`: core libraries
- `crates/kosha`: lexicon and headword metadata
- `crates/lekhya`: punctuation diagnostics
- `crates/parikshak`: end-to-end checker
- `crates/vyakaran`, `crates/samasa`: grammar/samasa libraries
- `crates/eval`: evaluation/test harnesses
- `web/`: browser UI + rules reference + WASM bridge
- `docs/tests/*.toml`: gold/eval fixtures

## Quick Start

### Prerequisites

- Rust 1.85.0+
- Cargo
- (Optional for web build) `wasm-pack` and `wasm-bindgen-cli`

### Build

```bash
cargo build --workspace
```

### Test and Lint

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace -q
```

### Run Web App

```bash
bash web/build.sh
python3 -m http.server 8080 --directory web/
```

Open `http://localhost:8080`.

### Web Smoke Test

```bash
bash web/smoke-test.sh
```

## Status

Current feature matrix: `docs/STATUS.md`

## Documentation

- `docs/README.md` — central index for all documentation
- `docs/VISION.md` — project philosophy and principles
- `docs/ARCHITECTURE.md` — system design and dependencies
- `docs/DEVELOPMENT.md` — build instructions and dev workflows
- `docs/DATASETS.md` — testing sources and provenance
- `docs/RULES.md` — linguistic rule implementations
- `docs/STATUS.md` — feature matrix and current progress
- `docs/BACKLOG.md` — near-term priorities
- `docs/RUST_GUIDE.md` — onboarding for Rust contributors
- `docs/Notices-pages-77-99.md` — Academy reference used for rules alignment (source: [MoFAGA notice](https://mofaga.gov.np/notice-file/Notices-20211029142422901.pdf))

## Contributing

Technical and non-technical contributions are welcome.

- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `SECURITY.md`
- `SUPPORT.md`
- Issue templates in `.github/ISSUE_TEMPLATE/` (including linguistic issue reporting)

## License

Dual-licensed under MIT or Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`).
