# Documentation

```text
Start here:
  README.md
    ↓
  docs/ARCHITECTURE.md
    ↓
  docs/DEVELOPMENT.md
    ↓
  crate-level ARCHITECTURE.md files
```

## Overview

- **[ARCHITECTURE.md](ARCHITECTURE.md)**: System design, crate capabilities, and data flow.
- **[DEVELOPMENT.md](DEVELOPMENT.md)**: Build instructions, testing guide, and dev workflows.
- **[DATASETS.md](DATASETS.md)**: Explanation of test data, gold standards, and provenance.
- **[RULES.md](RULES.md)**: Linguistic rules implementation and Academy standard mapping.
- **[RULE_SOURCE_POLICY.md](RULE_SOURCE_POLICY.md)**: Source precedence, conflict handling, and stopgap policy.
- **[INTEGRATION_NOTES.md](INTEGRATION_NOTES.md)**: Downstream option and diagnostic-behavior notes.
- **[BACKLOG.md](BACKLOG.md)**: Current state, near-term priorities, and deferred work.
- **[Notices-pages-77-99.md](Notices-pages-77-99.md)**: Academy notice excerpt reference (source: [MoFAGA notice](https://mofaga.gov.np/notice-file/Notices-20211029142422901.pdf)).
- **[PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md](PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md)**: Academy school-grammar reference used alongside the notice excerpt for rule implementation.

## Crate Architecture Docs

- **[../crates/prakriya/README.md](../crates/prakriya/README.md)**: Token-level rule engine, rule coverage, and correction-table audit.
- **[../crates/prakriya/ARCHITECTURE.md](../crates/prakriya/ARCHITECTURE.md)**: Runtime dispatch, winner selection, and rule-hit model.
- **[../crates/parikshak/README.md](../crates/parikshak/README.md)**: Text-level checker, `(घ)` coverage, and `तिर्यक्` coverage.
- **[../crates/parikshak/ARCHITECTURE.md](../crates/parikshak/ARCHITECTURE.md)**: Diagnostic pipeline, pass ownership, and category contracts.
- **[../crates/bindings-wasm/README.md](../crates/bindings-wasm/README.md)**: Browser/JavaScript exports and downstream WASM contract.
- **[../web/README.md](../web/README.md)**: Web app and browser artifact packaging workflow.
