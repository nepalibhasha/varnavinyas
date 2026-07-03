# Backlog

Last reviewed: 2026-07-03

This backlog is aligned against:

- `docs/Notices-pages-77-99.md`
- `docs/PS-Saisanik-Vyakaran-Varnavinyas-Page-327-349.md`
- `docs/RULE_SOURCE_POLICY.md`
- current implementation in `crates/prakriya`, `crates/parikshak`, `crates/lekhya`, `crates/sandhi`, `crates/samasa`, and `crates/vyakaran`

The source policy remains: prefer explicit Academy rules, prefer `PS-Saisanik...` when the two markdown sources conflict, and prefer rule-layer implementations over correction-table growth.

## Current State Notes

- The previous “Next 4-8 Weeks” window is expired.
- Current state now lives here instead of in a separate status page.
- `docs/tests/gold.toml` now has 110 fixture entries, not the older 91-entry shape.
- The old `अध्यन -> अध्ययन` implementation item is no longer open as an implementation bug: it exists in `docs/tests/gold.toml`, `crates/prakriya/src/correction_table.rs`, and the CLI flags it. The remaining work is source/audit cleanup because it is a stopgap, not a direct niyama derivation.
- `parikshak` now has a corpus snapshot regression gate and a documented arbitration contract in `crates/parikshak/ARBITRATION.md`.
- `kosha` now has lazy lexicon-tier overrides for attested forms that are unsafe as correction targets.
- Growing cited rule example lists have TSV inventory pilots under `data/rule_inventories/`.

## Current Implementation Snapshot

- Core utilities (`akshar`, `lipi`, `types`) are stable.
- `kosha` is stable and uses compile-time lexical assets: ~207k surface forms and ~132k headwords.
- `prakriya` is active and owns token-level orthography under `src/varna_vinyasa/`.
- `parikshak` is active and owns the production text checker, phrase/context passes, and stable diagnostic category codes.
- `lekhya` punctuation diagnostics are stable.
- `sandhi`, `vyakaran`, and `samasa` are active but still need broader eval coverage before being treated as mature language analysis layers.
- User-facing surfaces are active: CLI, web app, LSP, WASM, Python, C, and UniFFI bindings.
- No new user-facing diagnostic category codes were added by the arbitration, lexicon-tier, or ajanta-inventory work.

## Near-Term Priorities

### 1. Section 3(क) Hrasva/Dirgha Closure

Focus on the rule gaps explicitly marked in `crates/prakriya/src/varna_vinyasa/hrasva_dirgha/*`.

- [ ] Implement or formally defer verb-sensitive hrasva rules:
  - `3(क)(अ)-8`: धातुहरू
  - `3(क)(अ)-9`: क्रियापदहरू
  - `3(क)(आ)-7`: बिचका क्रियापद
  - `3(क)(आ)-8`: कर्म/भाव वाच्यका क्रियापद
  - `3(क)(इ)-8`: `नु`/`छु` प्रत्यय भएका क्रियापद
- [ ] Improve partially covered hrasva classes:
  - `3(क)(आ)-2`: suffix-family coverage
  - `3(क)(इ)-6`: विभक्ति final hrasva coverage
- [ ] Close or explicitly document final-dirgha semantic blockers:
  - `3(क)(ऊ)-4`: स्त्रीलिङ्गी विशेषण
  - `3(क)(ऊ)-6`: ईकारान्त निर्जीव नाम
  - `3(क)(ऊ)-10`: विध्यर्थक र स्त्रीलिङ्गी क्रियापद
- [ ] Add targeted tests for each new numbered subrule, including nearest false-positive examples.

### 2. Section 3(घ) / PS-Saisanik Spacing Consolidation

`पदयोग/पदवियोग` is now the highest-risk expansion area because it is context-sensitive and the two source documents have known policy differences.

- [ ] Pause broad new phrase-family expansion until existing generalized passes have stronger eval coverage.
- [ ] Re-audit unresolved policy-sensitive families:
  - `सरह`: notice-backed join evidence, not explicit in current `PS-Saisanik` split family
  - `थरी`: notice-backed classifier split with a legacy explicit rewrite for `सयथरी -> सय थरी`; generalized classifier handling remains deferred and `थरी` is not explicit in the current `PS-Saisanik` extract
  - broader `पदयोग-२` suffix inventory beyond current conservative `ज्यू` handling
  - broad `पदवियोग-१` baseline “each word separate” principle
- [ ] Expand fixtures for implemented `PS-Saisanik` families:
  - comparison splits: `जस्तो/जस्तै/जत्रो/जसरी`
  - `... स्वरूप` joins
  - middle-name joins
  - one-meaning compound joins
  - institutional/title splits
  - `निपात`, `जना`, meaningful reduplication, nominal-verb, and multi-word samasa splits
- [ ] Add false-positive fixtures for lexicon-attested joined forms that source policy still requires splitting.

### 3. Correction Table Audit And Migration

Keep `crates/prakriya/src/correction_table.rs` from becoming a second rule engine.

- [ ] Migrate remaining Section 3-backed entries out of the correction table once rule-path parity tests exist.
- [ ] Keep true Section 4 shuddha/ashuddha entries in the table.
- [ ] Revisit current stopgaps:
  - `अध्यन -> अध्ययन`: source confirmation or replacement with a rule-backed path
  - `बिद्वान -> विद्वान्`: needs safe multi-step composition (`ब/व` plus halanta)
  - `भएकोमा -> भएकामा`: decide whether this belongs in lower `prakriya` derivation or only higher-level `तिर्यक्` handling
- [ ] Ensure every non-Section-4 table entry is documented in the `crates/prakriya/README.md` audit section with a removal/replacement path.

### 4. Other Rule Families

- [ ] Section 3(ख): broaden chandrabindu/shirbindu/panchham coverage, especially non-tatsam over-Sanskritized variants where pronunciation-based inference is currently conservative.
- [ ] Section 3(ग): harden exception handling for `श/ष/स`, `ब/व`, `य/ए`, `ऋ/रि`, `क्ष/छ्य`, and `ज्ञ/ग्या/ग्याँ` so rule fallbacks do not flag valid inflected or derivational forms.
- [ ] Section 3(ङ): continue halanta/ajanta context handling for ambiguous imperative-like forms and productive verb paradigms; PS loanword-ajanta example coverage is implemented through `data/rule_inventories/ajanta_halanta.tsv`, but broad origin-gated generalization remains deferred.
- [ ] Section 5: keep punctuation stable, but add broader positive/negative examples from the notice document for quote, slash, abbreviation, hyphen, ellipsis, and spacing behavior.
- [ ] Section 3(च): decide whether lipi-specific guidance needs a dedicated module and crate-level coverage note or should remain scattered across existing utilities.

### 5. Evaluation And Fixture Coverage

- [ ] Resolve the 21 entries in `docs/tests/needs_review.toml` into either `gold.toml`, explicit deferrals, or source-policy notes.
- [ ] Expand `docs/tests/grammar_sentences.toml`; it currently has only 7 sentence fixtures and two disabled expectations.
- [ ] Decide whether to promote, keep suppressed, or remove disabled low-confidence grammar expectations:
  - `ergative-le-intransitive`
  - `genitive-mismatch-plural`
- [ ] Expand `docs/tests/samasa_gold.toml`; it currently has only 3 compound-analysis fixtures.
- [ ] Expand `docs/tests/morph_gold.toml` beyond the initial MVP morphology set.
- [ ] Add source-alignment tests that sample each numbered Academy subsection, not only known correction examples.

## User-Facing And Release Work

- [ ] Keep web rule data aligned with Rust diagnostic categories whenever rule coverage changes.
- [ ] Re-check WASM bundle size after rule/data changes; optimize only if bundle growth becomes a real release blocker.
- [ ] Finish Python packaging/release workflow if the Python bindings are intended for public consumption soon.
- [ ] Keep the browser artifact versioning workflow aligned with downstream consumers.

## Done / Superseded From Previous Backlog

- [x] README onboarding refresh: completed in `docs: simplify README onboarding`.
- [x] `अध्यन -> अध्ययन` implementation path: present as a correction-table stopgap with gold coverage.
- [x] Browser extension ownership moved downstream; this repo now tracks browser artifact packaging and contract work instead.
