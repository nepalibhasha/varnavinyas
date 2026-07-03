# Rule Source Policy

> **Last reviewed**: 2026-07-03

This project treats the two source markdowns under `docs/` as normative linguistic references.

Normative linguistic sources

- `docs/PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md`
- `docs/Notices-pages-77-99.md`

UI/reference alignment source

- `web/js/rules-data.js`
  - source of truth for the browser rules-reference tab and tooltip/category mapping
  - not an independent linguistic authority

Operational fixture source

- `docs/tests/gold.toml`

Supporting coverage and audit docs

- `crates/prakriya/README.md` for token-level rule coverage and correction-table audit notes
- `crates/parikshak/README.md` for text-level `(घ)` and `तिर्यक्` coverage notes
- `data/rule_inventories/*.tsv` for reviewed, provenance-carrying rule inventories compiled into rule modules
- `data/lexicon_overrides.tsv` for reviewed lexicon-tier overrides that keep raw attestation from becoming an unsafe correction target

Policy

1. Prefer rule implementation when a correction is justified by an explicit niyama in the Academy markdown.
2. When `PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md` and `Notices-pages-77-99.md` conflict, prefer `PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md`.
3. Target state: use `crates/prakriya/src/correction_table.rs` only for:
   - entries explicitly present in the Academy's Section 4 shuddha/ashuddha table
   - temporary stopgaps that are clearly documented and tracked for later replacement
   Existing rule-backed holdouts are tracked in `crates/prakriya/README.md` until they can move safely into first-class rules.
4. Every new spelling fix must cite one of:
   - exact Academy section/rule
   - exact shuddha/ashuddha table entry
   - explicit stopgap justification recorded in `crates/prakriya/README.md`
5. When rule support is weak or ambiguous, prefer no diagnosis over speculative correction.
6. Do not remove a correction-table entry just because each component rule exists separately; if the final output needs multi-step composition that `derive()` cannot yet produce, keep the entry and document the gap in `crates/prakriya/README.md`.
7. Prefer generalized rule layers over one-off table entries for:
   - `तिर्यक्` forms
   - `पदयोग/पदवियोग`
   - context-sensitive phrase or sentence behavior
8. Common-editorial orthography mode is not frequency-based. Only reviewed
   common-vs-strict forms in the curated checker registry may be downgraded
   from `Error` to `Variant`; unreviewed rule hits remain errors.
9. Raw lexicon attestation is not enough to make a form a safe correction
   target. When a rule uses lexical validation for a proposed output, prefer
   `kosha::is_correction_target()` unless the rule has a stronger source-backed
   reason to accept any attested form.
10. Growing source example lists should move toward schema-checked TSV
    inventories with mandatory source and review-status fields instead of
    scattered Rust constants.

Current conflict resolutions

- reviewed common-vs-strict orthographic variants
  - `Notices-pages-77-99.md` explicitly prefers strict forms such as
    `सङ्घ`, `सङ्घीय`, `सञ्चार`, `सङ्केत`, and `संसद्`.
  - modern public and literary writing commonly uses forms such as `संघीय`,
    `संघ`, `संचार`, `संकेत`, `संसद`, and the political name `कांग्रेस`.
  - current policy: keep `academy-strict` as the compatibility default; in
    `common-editorial` mode, downgrade only curated reviewed cases to
    `Variant` with the strict form retained as `correction`.
  - current implementation: `crates/parikshak/src/checker.rs`

- `जस्तो/जस्तै/जत्रो/जसरी`
  - `Notices-pages-77-99.md` treats this family under `पदयोग` joining examples.
  - `PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md` explicitly requires them to be written separately before a preceding name/pronoun phrase.
  - current policy: prefer the `PS-Saisanik...` split behavior
  - current implementation: `crates/parikshak/src/checker/padayog.rs`
- `सरह`
  - `सरह` appears in `Notices-pages-77-99.md` with the older join-style comparison family.
  - `PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md` does not currently list `सरह` in the explicit split family (`जस्तो/जस्तै/जत्रो/जसरी` only).
  - current policy: keep the Notice-default join behavior for `सरह`; do not fold it into the `PS-Saisanik` split comparison rule unless a future source explicitly adds it there.
  - current implementation: `crates/parikshak/src/checker/padayog_rules.rs` (`बुद्धि सरह -> बुद्धिसरह`), pinned by `sarah_join_rule_still_applies`.
- honorific `ज्यू` vs `ज्यु`
  - `PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md` explicitly uses `ज्यू`
  - current policy: `ज्यू` is the preferred honorific suffix form for generalized `पदयोग-२` joining
  - caveat: `ज्यु` is still an attested lexical noun in the lexicon, so normalization to `ज्यू` is only applied in honorific-suffix context after a plausible host word; it is not a blanket word-level rewrite
  - current implementation: `crates/parikshak/src/checker/padayog.rs`
- `विम्ब` / `बिम्ब` family
  - `Notices-pages-77-99.md` uses `विम्ब` in पञ्चम-वर्ण and `ब` examples.
  - `PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md` ४(घ) explicitly lists Sanskrit-व् words that take `ब` in Nepali: `बिन्दु`, `बिम्ब`, `बेला`, `कुबेला`, `सुबेला`, `बार`, `आइतबार`, `बुधबार`, `बिना`.
  - current policy: prefer the `PS-Saisanik...` ब-forms for this listed family, even when raw lexicon assets contain both ब/व variants.
  - current implementation: `crates/prakriya/src/varna_vinyasa/ustai_ucharan_varnaharu/ba_va.rs` (`3(ग)(आ)-PS-Saisanik-4(घ)-ब`)
- `बधू` / `वधू`
  - `Notices-pages-77-99.md` lists `बधू` under a broad द/ध/ल/ह-before-ब pattern.
  - `PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md` lists `वधू`, and the Sanskrit tatsam principle also supports `वधू`.
  - current policy: prefer `वधू`; treat the Notice `बधू` listing as a source defect for this word, not as a reason to generalize the broad ब-pattern over tatsam `वधू`.
  - current implementation status: pending explicit rule/guard if normalization is added.
- final-dirgha exception inventory
  - `Notices-pages-77-99.md` states final `ति/धि/नि/टि/पि` classes broadly as hrasva.
  - `PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md` adds exceptions whose word-final इ is conventionally dirgha: `श्रेणी`, `युवती`, `सूची`, `अञ्जली` / `श्रद्धाञ्जली`, `आवली` / `शब्दावली`, `औषधी`.
  - current policy: prefer the `PS-Saisanik...` dirgha exceptions; these forms must not be normalized to final hrasva.
  - current implementation: explicit `PS_FINAL_DIRGHA_EXCEPTIONS` inventory in `crates/prakriya/src/varna_vinyasa/hrasva_dirgha/helpers.rs`
- loanword-ajanta (`आगन्तुक` words pronounced halanta)
  - `PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md` ३(ग) explicitly says loanwords pronounced halanta are written ajanta, with examples such as `कोट`, `टेलिफोन`, `कम्प्युटर`, `रिजल्ट`, `सिमेन्ट`, `बल्ब`, and `फिल्ड`.
  - current policy: implement the explicit PS examples as reviewed ajanta corrections, but do not generalize by "strip terminal halanta if the stripped form is attested" because verb roots and other legitimate halanta forms can be false positives.
  - current implementation: `data/rule_inventories/ajanta_halanta.tsv` consumed by `crates/prakriya/src/varna_vinyasa/halanta_ra_ajanta/ajanta.rs`; context-free checker diagnostics suppress ambiguous verb-root ajanta examples in `crates/parikshak/src/checker/word_level.rs`.
- `श`-initial / `श`-bearing proper names and surnames
  - `Notices-pages-77-99.md` and broad तद्भव/आगन्तुक श/ष/स normalization can otherwise suggest स-forms for common proper names.
  - `PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md` examples preserve forms such as `शेर्पा`; current local policy treats reviewed proper names/surnames as lexically protected even when a broad sibilant rule would otherwise apply.
  - current implementation: `PROPER_NOUN_SH_BASES` guard in `crates/prakriya/src/varna_vinyasa/ustai_ucharan_varnaharu/sibilant.rs` for `कुशवाह`, `जोशी`, `शाह`, `शेर्पा`, `शेरचन` with common suffixes.
- `तिर्यक्`
  - `PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md` adds an explicit `तिर्यक् रूपको प्रयोग` rule family that is not a numbered Section 3 notice rule in `Notices-pages-77-99.md`
  - current policy: treat `तिर्यक्` as a first-class checker rule family, not as correction-table growth
  - current implementation: `crates/parikshak/src/checker/tiryak.rs`
- `परराष्ट्र मन्त्रालय` / `नेपाल सरकार`-type institutional phrases
  - `PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md` explicitly keeps many such institutional/topic phrases split even when joined compounds are attested in raw lexicon assets
  - current policy: prefer the `PS-Saisanik...` split behavior for the implemented institutional/title inventories
  - current implementation: `crates/parikshak/src/checker/padayog.rs`
- `प्रधान मन्त्री` / `शुभ कामना` / `कीर्ति पुर`-type one-meaning compounds
  - `PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md` explicitly joins one-meaning place names and two-word one-meaning compounds
  - current policy: join conservatively only when the combined form is already attested and the right-hand member is in the curated school-grammar-backed inventory
  - current implementation: `crates/parikshak/src/checker/padayog.rs`

Review checklist for new fixes

1. Which exact source and subrule justify this change?
2. If the two normative markdowns differ, which one wins and why?
3. Can this be implemented as a rule instead of a one-off correction-table entry?
4. If it is a stopgap, is it recorded in `crates/prakriya/README.md` with a removal path?
5. Does the change add a regression test for both the intended correction and the nearest false-positive risk?

Current consolidation gate

- The recent `PS-Saisanik...` work has expanded beyond isolated fixes into a broader phrase-rule layer.
- Further implementation should now default to consolidation first:
  - update local source-policy and crate coverage notes
  - expand broader eval fixtures, not only targeted regressions
  - review false-positive risk before adding new inventories or segmentation paths
- Preferred stopping rule for the next phase:
  - do not add another phrase family unless the source is explicit, the rule can be generalized conservatively, and there is a clear eval-fixture path for it
- If a candidate change mainly requires growing curated token inventories without stronger structural validation, treat it as `defer until audited` rather than `implement immediately`.

Current local consolidation priorities

1. Broaden eval/gold coverage for the newly implemented `PS-Saisanik` phrase families.
2. Re-review unresolved policy-sensitive families:
   - `थरी`
   - broader `पदयोग-२` suffix inventory
   - broader `पदवियोग-१` baseline splitting
3. Keep `web/build-info.json` out of commits unless intentionally packaging; commit markdown policy notes only when intentionally updating project docs.

Repository expectations

- `docs/tests/gold.toml` is the curated fixture set used by tests, not the normative authority by itself.
- `crates/prakriya/src/correction_table.rs` should stay aligned with `docs/tests/gold.toml` for Section 4-backed entries.
- Any entry that is not directly backed by the Academy markdown must be called out explicitly in the `crates/prakriya/README.md` audit section rather than silently mixed into the table.
- `PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md` may introduce rule families that are not explicit in the notice document, such as `तिर्यक्`; implement those as first-class rules rather than as growing correction-table exceptions.
- When a family is only partially shared across the two sources, record that distinction explicitly:
  - `जस्तो/जस्तै/जत्रो/जसरी` are currently school-grammar-backed split forms
  - `सरह` is currently a Notice-backed join form, not a school-grammar-backed split override
  - `जना` is school-grammar-backed
  - `थरी` is currently notice-backed only
- When a `PS-Saisanik...` override changes already-attested lexical spellings in `data/words.txt` or `data/headwords.tsv`, prefer the rule policy over raw lexicon acceptance and document the override here.
