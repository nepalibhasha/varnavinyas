# varnavinyas-lsp

Language Server Protocol implementation for editor integrations.

## What This Crate Owns

This crate exposes Varnavinyas diagnostics to editors through LSP. It turns core diagnostic results into:

- published diagnostics
- hover explanations
- code actions

It is the editor-facing runtime used by the VS Code extension and compatible editors.

## Main Responsibilities

- track open documents
- run `varnavinyas-parikshak` on document changes
- convert byte spans into LSP ranges
- publish editor diagnostics
- provide hover text with rule context

## Example

Run the language server binary and connect to it from an editor client:

```bash
cargo run -p varnavinyas-lsp
```

In practice, the server will call into `varnavinyas-parikshak` whenever the editor opens or changes a document, then publish diagnostics back through LSP.

## Depends On

- `varnavinyas-parikshak`
- `tower-lsp`

## Design Notes

- This crate should not implement language rules itself.
- It is a transport and presentation layer over core diagnostics.

## Current Limits

- Uses full-document sync rather than incremental diff-aware analysis.
- Richer analysis tiers should be surfaced more explicitly as the core model evolves.

## Status

Working editor integration layer with room for performance and UX refinement.
