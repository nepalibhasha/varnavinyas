# varnavinyas

Terminal interface for running Varnavinyas checks and utility commands.

## What This Crate Owns

This crate is the command-line surface for the workspace. It exists to make the core engines usable from shell workflows, CI, and quick manual inspection.

## Commands

The CLI currently exposes commands for:

- `check` -> run text diagnostics
- `akshar` -> inspect script/akshara behavior
- `lipi` -> transliterate text

## Example

```bash
varnavinyas check document.txt
varnavinyas akshar "शब्द"
varnavinyas lipi "नेपाल" --to IAST
```

```bash
echo "राजनैतिक" | varnavinyas check -
```

## Release Assets

CLI release assets are published from stable `cli-v*` tags. Each release includes
per-platform archives, per-archive SHA-256 checksum files, and an aggregate
`SHA256SUMS` file.

Editor integrations should use `varnavinyas check --format json`. The compatible
JSON output contract is documented in `docs/CLI_JSON_CONTRACT.md`.

## Design Notes

- This crate should stay a thin wrapper over core crates.
- It should expose core semantics clearly, especially the difference between definite errors and softer suggestions.

## Depends On

- `varnavinyas-parikshak`
- `varnavinyas-akshar`
- `varnavinyas-lipi`

## Current Limits

- Advanced internal analyses are not yet surfaced in a rich review mode.
- JSON output is intentionally flat for editor clients and may add fields compatibly over time.

## Status

Usable command surface for local workflows and CI.
