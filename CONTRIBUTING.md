# Contributing to Varnavinyas

Thanks for helping improve Nepali orthography tooling.
This project welcomes both technical and non-technical contributors.

## Two ways to contribute

## 1. Report a linguistic issue (non-technical)
Use GitHub Issues and choose `Linguistic issue`.

Please include:
- What text/word/sentence looks incorrect
- What you expected instead
- Why you believe it is incorrect (if known)
- Source or citation (optional but very helpful)

Good linguistic reports are as valuable as code contributions.

## 2. Contribute code/docs/tests (technical)
Use GitHub Issues for discussion first when possible, then open a PR.

## Development setup
Prerequisites:
- Rust 1.85.0+
- Cargo

Setup and validate locally:
```bash
cargo build --workspace
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace -q
```

Web app (optional):
```bash
bash web/build.sh
bash web/smoke-test.sh
```

See `docs/RUST_GUIDE.md` for deeper Rust onboarding.

## Project conventions
- Rust edition: 2024
- Naming: `snake_case` for functions/modules, `PascalCase` for types
- Keep rules/corrections traceable to Academy sections
- Add regression tests for every fix (false-positive and false-negative paths)
- Treat `docs/tests/gold.toml` as canonical test data

## Pull request process
1. Keep each PR scoped to one concern.
2. Use Conventional Commits in commit titles (`feat:`, `fix:`, `docs:`, `test:`, `refactor:`, `chore:`).
3. Ensure all quality gates pass locally.
4. Fill out the PR template completely.

PRs should clearly include:
- Problem statement
- Design decisions/tradeoffs
- Impacted crates/files
- Test/lint evidence (commands + results)

## Reporting bugs
For software bugs, use the `Bug report` issue form and include:
- Steps to reproduce
- Expected vs actual behavior
- Environment (OS, Rust version)
- Minimal input text that reproduces the issue

## Suggesting features
Use the `Feature request` issue form.
Describe user value and whether the change targets linguists, developers, or both.

## Code of Conduct
By participating, you agree to follow `CODE_OF_CONDUCT.md`.

## Security
Do not open public issues for sensitive vulnerabilities.
See `SECURITY.md` for responsible disclosure instructions.
