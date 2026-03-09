# Varnavinyas (वर्णविन्यास)

Open-source Nepali orthography tooling based on Nepal Academy standards.

*शुद्ध नेपाली, सबैका लागि।*  
*(Correct Nepali, for everyone.)*

**[Try it in your browser](https://nepalibhasha.github.io/varnavinyas/)** — no install required.

## What This Project Is

Varnavinyas is a Rust workspace for checking and normalizing Nepali text.

It is built for three kinds of users:
- writers, editors, teachers, students, and institutions that need standard Nepali spelling and punctuation
- developers who want a reusable Nepali orthography engine
- contributors who want Academy-aligned rules implemented transparently and auditable in code

The project focuses on:
- word-level orthography correction
- punctuation diagnostics
- rule tracing with Academy citations
- web, CLI, editor, and binding surfaces on top of the same core engine

Diagnostics use stable `category_code` values across the web app, CLI, LSP, and bindings so filtering and highlighting stay consistent.

## Use Varnavinyas

### Browser

Use the hosted web app:

- <https://nepalibhasha.github.io/varnavinyas/>

The web app includes:
- text checker
- word inspector
- rules reference

### CLI

Build the workspace:

```bash
cargo build --workspace
```

Run the checker:

```bash
cargo run -p varnavinyas -- check
```

Run with JSON output:

```bash
cargo run -p varnavinyas -- check --format json
```

### Editor / LSP

The workspace includes an LSP server in `crates/lsp` for editor integrations and diagnostic surfacing.

### Bindings

Public bindings exist for:
- WebAssembly: `crates/bindings-wasm`
- Python: `crates/bindings-python`
- C: `crates/bindings-c`
- UniFFI: `crates/bindings-uniffi`

## Architecture At A Glance

```mermaid
flowchart LR
    A[kosha<br/>lexicon + metadata]
    B[prakriya<br/>token-level correction]
    C[parikshak<br/>text-level checking]
    D[CLI / LSP / Web / Bindings]

    A --> B
    B --> C
    C --> D
```

The core flow is:

1. `kosha` provides lexicon lookup and metadata
2. `prakriya` decides token-level standard form and rule trace
3. `parikshak` runs text-level checking, span handling, padayog/padabiyog passes, punctuation, and heuristics
4. CLI, web, LSP, and bindings present those diagnostics to users

The main crates are:
- `crates/prakriya`: token-level orthography engine
- `crates/parikshak`: end-to-end text checker
- `crates/kosha`: lexicon and headword metadata
- `crates/lekhya`: punctuation diagnostics
- `web/`: browser UI backed by WASM

Inside `crates/prakriya`:
- `src/varna_vinyasa/` owns Academy orthography families
- `src/usage_fixes/` owns later cleanup-style rules
- `src/runtime.rs` assembles and caches runtime rule dispatch
- `src/model/` owns core derivation types

Inside `crates/parikshak`:
- `src/checker/word_level.rs` owns token-level integration
- `src/checker/padayog.rs` owns join/split text passes
- `src/checker/punctuation.rs` owns punctuation diagnostics
- `src/checker/style_variants.rs` and `src/checker/grammar.rs` own higher-level heuristics

## Workspace Layout

```text
crates/
  core:       akshar lipi shabda sandhi types
  checking:   kosha prakriya lekhya parikshak
  analysis:   vyakaran samasa eval
  surfaces:   cli lsp bindings-*
web/
docs/
```

- `crates/akshar`, `crates/lipi`: Devanagari text utilities
- `crates/shabda`, `crates/sandhi`, `crates/types`: morphology and shared language/data types
- `crates/prakriya`, `crates/parikshak`, `crates/lekhya`, `crates/kosha`: core checking stack
- `crates/vyakaran`, `crates/samasa`: grammar and samasa libraries
- `crates/eval`: evaluation harnesses
- `web/`: web UI and WASM bridge
- `docs/tests/*.toml`: gold and eval fixtures

## Build And Test

### Prerequisites

- Rust 1.85.0+
- Cargo
- optional for web builds: `wasm-pack` and `wasm-bindgen-cli`

### Main Commands

```bash
cargo build --workspace
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace -q
```

Build the web app:

```bash
bash web/build.sh
```

Smoke-test the web app:

```bash
bash web/smoke-test.sh
```

Serve the built web app locally:

```bash
python3 -m http.server 8080 --directory web/
```

## Documentation

Start here:
- [docs/README.md](docs/README.md)

Key docs:
- [docs/VISION.md](docs/VISION.md) — why the project exists
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) — system design and crate boundaries
- [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) — build and test workflow
- [docs/DATASETS.md](docs/DATASETS.md) — datasets and provenance
- [docs/RULES.md](docs/RULES.md) — rule implementation notes
- [docs/STATUS.md](docs/STATUS.md) — current feature matrix
- [docs/Notices-pages-77-99.md](docs/Notices-pages-77-99.md) — Academy reference used for rule alignment

Crate-specific architecture docs:
- [crates/prakriya/ARCHITECTURE.md](crates/prakriya/ARCHITECTURE.md)
- [crates/parikshak/ARCHITECTURE.md](crates/parikshak/ARCHITECTURE.md)

## Contributing

Technical and non-technical contributions are welcome.

Start with:
- [CONTRIBUTING.md](CONTRIBUTING.md)
- [docs/RUST_GUIDE.md](docs/RUST_GUIDE.md)
- [docs/BACKLOG.md](docs/BACKLOG.md)

Community and process files:
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
- [SECURITY.md](SECURITY.md)
- [SUPPORT.md](SUPPORT.md)

## License

Dual-licensed under MIT or Apache-2.0.

- [LICENSE-MIT](LICENSE-MIT)
- [LICENSE-APACHE](LICENSE-APACHE)
