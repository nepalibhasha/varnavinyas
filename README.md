# Varnavinyas

[![CI](https://github.com/varnavinyas/varnavinyas/actions/workflows/ci.yml/badge.svg)](https://github.com/varnavinyas/varnavinyas/actions/workflows/ci.yml)

Nepali orthography tooling based on Nepal Academy standards.

Varnavinyas checks Nepali text for spelling, punctuation, and writing-convention issues. This repository contains the core Rust engine, a browser UI, a CLI, editor support, and bindings for other platforms.

## Contents

- Quickstart
- Web app
- Architecture
- Documentation
- Contributing

## Quickstart

This quickstart will show you how to:

- build the Rust workspace
- run the command-line checker
- run the test suite

### Building Varnavinyas

Install Rust 1.85.0 or newer, then build the workspace:

```bash
git clone https://github.com/varnavinyas/varnavinyas.git
cd varnavinyas
cargo build --workspace
```

The first build may take a little while because the workspace includes the checker engine, CLI, LSP server, and several binding crates.

### Running the checker

You can check a file:

```bash
cargo run -p varnavinyas -- check path/to/document.txt
```

Or pipe text directly into the checker:

```bash
printf "राजनैतिक गल्ति छ.\n" | cargo run -q -p varnavinyas -- check - --explain
```

Use JSON output when integrating with scripts or other tools:

```bash
cargo run -p varnavinyas -- check path/to/document.txt --format json
```

The CLI also includes small utility commands:

```bash
cargo run -p varnavinyas -- akshar "शब्द"
cargo run -p varnavinyas -- lipi "नेपाल" --to IAST
```

### Running tests

Run the main test suite:

```bash
cargo test --workspace -q
```

Before opening a pull request, also run formatting and clippy:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

## Web app

The browser app includes a text checker, word inspector, and rules reference. If you just want to try Varnavinyas, use the hosted app linked from the repository homepage.

To build and serve the web app locally:

```bash
bash web/build.sh
python3 -m http.server 8080 --directory web/
```

Then open `http://localhost:8080`.

The web build uses `wasm-pack` and `wasm-bindgen-cli`. If the installed `wasm-bindgen-cli` version does not match the generated package, `web/build.sh` prints the exact version to install.

For a static consistency check of the web app and WASM bundle, run:

```bash
bash web/smoke-test.sh
```

See [web/README.md](web/README.md) for more detail.

## Architecture

The core flow is intentionally small:

```text
kosha      lexicon lookup and word metadata
prakriya   token-level correction and rule tracing
parikshak  full-text diagnostics with spans and categories
surfaces   CLI, LSP, web UI, and platform bindings
```

The most important crates are:

- `crates/kosha`: FST-backed lexicon and headword metadata
- `crates/prakriya`: token-level orthography engine
- `crates/parikshak`: end-to-end text checker
- `crates/lekhya`: punctuation diagnostics
- `crates/cli`: command-line interface
- `crates/lsp`: language server
- `crates/bindings-*`: Python, WebAssembly, C, and UniFFI bindings

For the full design, read [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md). The two most useful crate-level guides are [crates/prakriya/ARCHITECTURE.md](crates/prakriya/ARCHITECTURE.md) and [crates/parikshak/ARCHITECTURE.md](crates/parikshak/ARCHITECTURE.md).

## Documentation

Start with [docs/README.md](docs/README.md), which links to the main project documents.

Useful entry points:

- [docs/VISION.md](docs/VISION.md): project goals and scope
- [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md): build, test, and development workflow
- [docs/RULES.md](docs/RULES.md): rule implementation notes
- [docs/DATASETS.md](docs/DATASETS.md): datasets and provenance
- [docs/STATUS.md](docs/STATUS.md): current feature status
- [docs/RUST_GUIDE.md](docs/RUST_GUIDE.md): Rust onboarding notes

Academy reference material used for rule alignment lives in:

- [docs/Notices-pages-77-99.md](docs/Notices-pages-77-99.md)
- [docs/PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md](docs/PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md)

## Contributing

Both linguistic reports and code contributions are welcome. Good issue reports with examples, expected corrections, and source citations are especially useful.

For the contribution process, see [.github/CONTRIBUTING.md](.github/CONTRIBUTING.md). For support, security, and community expectations, see [.github/SUPPORT.md](.github/SUPPORT.md), [.github/SECURITY.md](.github/SECURITY.md), and [.github/CODE_OF_CONDUCT.md](.github/CODE_OF_CONDUCT.md).

## License

Varnavinyas is dual-licensed under MIT or Apache-2.0.

- [LICENSE-MIT](LICENSE-MIT)
- [LICENSE-APACHE](LICENSE-APACHE)
