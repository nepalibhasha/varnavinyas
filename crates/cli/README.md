# varnavinyas-cli

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

## Design Notes

- This crate should stay a thin wrapper over core crates.
- It should expose core semantics clearly, especially the difference between definite errors and softer suggestions.

## Depends On

- `varnavinyas-parikshak`
- `varnavinyas-akshar`
- `varnavinyas-lipi`

## Current Limits

- Advanced internal analyses are not yet surfaced in a rich review mode.
- JSON output is useful, but still flatter than the underlying future data model should be.

## Status

Usable command surface for local workflows and CI.
