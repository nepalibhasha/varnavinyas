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

- **[VISION.md](VISION.md)**: Why this project exists, principles, and high-level scope.
- **[ARCHITECTURE.md](ARCHITECTURE.md)**: System design, crate capabilities, and data flow.
- **[DEVELOPMENT.md](DEVELOPMENT.md)**: Build instructions, testing guide, and dev workflows.
- **[DATASETS.md](DATASETS.md)**: Explanation of test data, gold standards, and provenance.
- **[RULES.md](RULES.md)**: Linguistic rules implementation and Academy standard mapping.
- **[STATUS.md](STATUS.md)**: Current features and implementation status.
- **[BACKLOG.md](BACKLOG.md)**: Near-term priorities and task list.
- **[RUST_GUIDE.md](RUST_GUIDE.md)**: Onboarding guide for Rust contributors.
- **[Notices-pages-77-99.md](Notices-pages-77-99.md)**: Academy notice excerpt reference (source: [MoFAGA notice](https://mofaga.gov.np/notice-file/Notices-20211029142422901.pdf)).
- **[PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md](PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md)**: Academy school-grammar reference used alongside the notice excerpt for rule implementation.
## Crate Architecture Docs

- **[../crates/prakriya/ARCHITECTURE.md](../crates/prakriya/ARCHITECTURE.md)**: Token-level orthography engine design, runtime dispatch, and rule-hit model.
- **[../crates/parikshak/ARCHITECTURE.md](../crates/parikshak/ARCHITECTURE.md)**: Text-checking pipeline, diagnostic model, and pass ownership.
