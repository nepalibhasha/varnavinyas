# Linguistic Rules

Varnavinyas encodes Nepali orthography rules as auditable Rust code. This document maps the local Academy references to the implementation layers.

## Sources

The rule layer currently uses two local Academy references:

1. Nepal Academy orthography notice excerpt:
   - source PDF: <https://mofaga.gov.np/notice-file/Notices-20211029142422901.pdf>
   - local excerpt: `docs/Notices-pages-77-99.md`
2. School-grammar reference for additional and sometimes conflicting spacing/morphosyntactic guidance:
   - local excerpt: `docs/PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md`

When these two sources conflict, follow `docs/RULE_SOURCE_POLICY.md`. The current policy prefers `PS-Saisanik...` for the known conflict set.

## Implementation Model

```text
Single-token orthography       -> prakriya
Multi-token spacing/context    -> parikshak
Punctuation                    -> lekhya
Lexical plausibility/metadata  -> kosha + shabda
Surface-specific presentation  -> CLI / LSP / Web / Bindings
```

Rules are plain Rust functions plus small schema-checked TSV inventories that
are compiled into the owning rule modules. There is no external JSON/YAML rule
engine. Shared metadata and browser rule labels live outside the rule engine,
but production decisions remain in code and reviewed inventories.

## Rule Categories

| Source Area | Description | Current Implementation | Status |
|---|---|---|---|
| Section 3 `(क)` | ह्रस्व/दीर्घ vowel length | `crates/prakriya/src/varna_vinyasa/hrasva_dirgha.rs` and `hrasva_dirgha/{a,aa,i,u,uu}.rs` | Partial / expanded |
| Section 3 `(ख)` | चन्द्रविन्दु, शिरविन्दु, पञ्चम वर्ण | `crates/prakriya/src/varna_vinyasa/chandrabindu_shirbindu.rs`, `chandrabindu_shirbindu/*`, `panchham.rs` | Partial / expanded |
| Section 3 `(ग)` | श/ष/स, ऋ/रि, ब/व, य/ए, क्ष/छ्य, ज्ञ/gya families | `crates/prakriya/src/varna_vinyasa/ustai_ucharan_varnaharu.rs` and submodules | Partial |
| Section 3 `(घ)` | पदयोग/पदवियोग | `crates/parikshak/src/checker/padayog.rs`, `padayog_rules.rs`, and phrase-specific checker passes | Partial / active |
| Section 3 `(ङ)` | हलन्त/अजन्त | `crates/prakriya/src/varna_vinyasa/halanta_ra_ajanta.rs` and submodules | Partial / expanded |
| Section 3 `(च)` | लिपिगत विशिष्टता and related notes | Scattered/partial handling; no full dedicated module yet | Missing / partial |
| Section 4 | शुद्ध-अशुद्ध table | `crates/prakriya/src/correction_table.rs` plus rule-backed exceptions | Active, not a pure table mirror |
| Section 5 | punctuation and formatting | `crates/lekhya/src/punctuation.rs`, integrated through `parikshak` | Stable |
| `PS-Saisanik` section 7 | तिर्यक् रूपको प्रयोग | `crates/parikshak/src/checker/tiryak.rs` | Partial / active |

## Current Coverage Notes

- Section 3 `(क)` has broad coverage for many initial, medial, and final hrasva-dirgha classes. Remaining gaps are mostly verb-sensitive classes and semantic classes that need stronger morphology/lexicon signals.
- Section 3 `(घ)` and `PS-Saisanik` spacing rules live in `parikshak` because they need neighboring tokens, spacing, punctuation, or phrase context.
- `parikshak` arbitrates overlapping text diagnostics explicitly; see `crates/parikshak/ARBITRATION.md` for the current `kind > specificity > pass > confidence` contract.
- Section 3 `(ङ)` includes inventory-backed ajanta coverage for the Notice example lists and the `PS-Saisanik` loanword-ajanta examples such as `कोट् -> कोट`.
- Section 4 is not simply a lexicon lookup. `correction_table.rs` currently contains 81 entries: 38 Section 4-style entries, 42 rule-backed holdouts, and 1 documented stopgap. See `crates/prakriya/README.md`.
- `तिर्यक्`, comparison spacing, institutional/title splits, and similar school-grammar phrase behavior should be first-class checker rules, not correction-table growth.

## Example Rule Shape

```rust
// crates/prakriya/src/varna_vinyasa/hrasva_dirgha/a.rs

pub fn rule_suffix_nu_hrasva(input: &str) -> Option<Prakriya> {
    // Verbal suffix families such as -नु are handled with lexical and
    // derivational guards before returning a correction path.
}
```

## Diagnostics

When a rule is violated, user-facing surfaces receive a diagnostic with:

1. incorrect text and byte span
2. suggested correction
3. rule citation and stable `rule_code`
4. human-readable explanation
5. stable `category_code`
6. confidence and optional alternate reasons

Stable diagnostic categories are defined in `crates/parikshak/src/diagnostic.rs` and are consumed by the web UI, CLI JSON, LSP, and bindings.

## Alignment Strategy

- Use the two source markdowns for authority and citations.
- Use `docs/RULE_SOURCE_POLICY.md` when sources conflict.
- Keep `docs/tests/gold.toml` as the regression ground truth, not as independent linguistic authority.
- Prefer first-class rules over one-off table entries.
- Keep broad fallbacks below specific numbered rules and suppress duplicate alternate hits when a specific rule already explains the same correction.
- Prefer schema-checked inventories with provenance fields for growing cited example lists, especially when the alternative is scattered Rust constants.
- Treat raw lexicon attestation as plausibility evidence, not proof that a form is a safe correction target.
