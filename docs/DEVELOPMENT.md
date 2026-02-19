# Development Guide

## Prerequisites

- **Rust**: 1.85.0+ (Required for Edition 2024)
- **Cargo**: Standard installation
- **Optional**: `wasm-pack` (for web builds), `maturin` (for Python bindings)

## Build Commands

| Command | Description |
|---------|-------------|
| `cargo build --workspace` | Build all crates |
| `cargo test --workspace` | Run all unit and integration tests |
| `cargo fmt --all` | Format code |
| `cargo clippy --workspace --all-targets` | Run linter |

### Cargo Aliases
Check `.cargo/config.toml` for shortcuts:
- `cargo t` -> Run tests
- `cargo c` -> Run clippy
- `cargo f` -> Run format

## Testing Strategy

We rely on a multi-layered testing approach to ensure linguistic correctness.

### 1. Unit Tests
Located in `src/` of each crate. Test individual functions and corner cases.

### 2. Integration Tests
Located in `tests/` of each crate. Verify cross-module interactions (e.g., `parikshak` using `kosha`).

### 3. Gold Dataset Tests
**Critical for Linguistic Integrity.**
We maintain a "Gold" dataset in `docs/tests/gold.toml`. These are 91+ verified correct/incorrect pairs directly from the Academy standard.
*   **Rule**: ALL entries in `gold.toml` must pass.
*   **Run**: `cargo test -p varnavinyas-parikshak --test gold` (or specifically `gold_incorrect_forms_detected`).

### 4. Property-Based Tests
We use `proptest` to verify invariants, such as:
- `transliterate(transliterate(x)) == x` (Round-trip)
- `normalize(normalize(x)) == normalize(x)` (Idempotence)

## CI/CD Pipeline

Our GitHub Actions pipeline (`.github/workflows/ci.yml`) enforces:
1.  **Tests**: Runs on Ubuntu-latest (`cargo test`, `cargo bench`, `cargo test -p varnavinyas-eval`).
2.  **MSRV**: Checks compatibility with Rust 1.85.0.
3.  **Formatting & Clippy**: Zero tolerance for warnings (`cargo fmt`, `cargo clippy`).
4.  **Dependencies**: Audits using `cargo-deny`.
