# Varnavinyas

[![CI](https://github.com/nepalibhasha/varnavinyas/actions/workflows/ci.yml/badge.svg)](https://github.com/nepalibhasha/varnavinyas/actions/workflows/ci.yml)

Nepali orthography tooling based on Nepal Academy standards.

Varnavinyas checks Nepali text for spelling, punctuation, and writing-convention issues. It turns the Academy's written rules into portable software for editors, websites, scripts, and language tooling.

This repository contains the core Rust engine, a browser UI, a CLI, editor support, and bindings for other platforms. This README will show you how to build the project, run the checker, and find the deeper implementation notes.

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
git clone https://github.com/nepalibhasha/varnavinyas.git
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

By default the checker uses Academy-strict orthography. To treat reviewed
common forms such as `संघीय`, `संसद`, and `कांग्रेस` as editorial variants
rather than hard errors, pass:

```bash
cargo run -p varnavinyas -- check path/to/document.txt --orthography-mode common-editorial
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

The [docs](docs/README.md) folder covers architecture, development, rule policy, datasets, and current backlog. Rule implementation details live near the code: start with [crates/prakriya](crates/prakriya/README.md) for token-level corrections, [crates/parikshak](crates/parikshak/README.md) for text-level diagnostics, and [crates/bindings-wasm](crates/bindings-wasm/README.md) for browser/JavaScript exports.

The Academy references used for rule alignment are kept in [docs/Notices-pages-77-99.md](docs/Notices-pages-77-99.md) and [docs/PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md](docs/PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md).

## Contributing

Both linguistic reports and code contributions are welcome. Good issue reports with examples, expected corrections, and source citations are especially useful.

For the contribution process, see [.github/CONTRIBUTING.md](.github/CONTRIBUTING.md). For support, security, and community expectations, see [.github/SUPPORT.md](.github/SUPPORT.md), [.github/SECURITY.md](.github/SECURITY.md), and [.github/CODE_OF_CONDUCT.md](.github/CODE_OF_CONDUCT.md).

## License

Varnavinyas is dual-licensed under MIT or Apache-2.0.

- [LICENSE-MIT](LICENSE-MIT)
- [LICENSE-APACHE](LICENSE-APACHE)
