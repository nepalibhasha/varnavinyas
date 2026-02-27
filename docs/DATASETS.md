# Datasets & Test Fixtures

## Overview

Linguistic correctness is the primary goal of Varnavinyas. We manage our test data rigorously to separate "proven facts" from "debated interpretations."

## File Structure

Test data is located in `docs/tests/`:

*   **`gold.toml`** (The Ground Truth)
    *   Contains verified Correct/Incorrect pairs.
    *   **Source**: Directly cited from the Nepal Academy Orthography Standard.
    *   **Usage**: CI tests fail if any entry here is not handled correctly.

*   **`needs_review.toml`** (The Holding Area)
    *   Contains ambiguous, disputed, or context-dependent pairs requiring expert linguistic review.

## Evaluation & Corpus Datasets

*   **`samasa_gold.toml` & `morph_gold.toml`**
    *   Gold standards for compound word (Samasa) classification and morphological decomposition.
    *   **Usage**: Used by `varnavinyas-eval` to track heuristic precision and recall regressions.

*   **`grammar_sentences.toml`**
    *   Sentence-level pairs testing contextual grammar diagnostic bounds.

*   **`data/headwords.tsv`**
    *   Canonical headword list with POS metadata (`word<TAB>pos`) used by `varnavinyas-kosha`.
    *   Current scale: ~132k headwords.

*   **`data/words.txt`**
    *   Surface-form lexicon used to build the fast containment index for spell-checking.
    *   Current scale: ~207k entries.

## Lexicon Provenance

`data/words.txt` and `data/headwords.tsv` are derived from the Sabdasakha dictionary database, whose Nepali lexicon is anchored in:

1.  **नेपाली बृहत् शब्दकोश** (Nepali Brihat Shabdakosh), Nepal Academy.
2.  **प्रज्ञा नेपाली बृहत् शब्दकोश** (Pragya Nepali Brihat Shabdakosh), Nepal Academy.

Usage in Varnavinyas:

1.  `words.txt` powers the compiled FST for fast existence checks.
2.  `headwords.tsv` provides headword-level metadata (POS/origin-tag parsing).

## Provenance Policy

Every entry in our datasets must have a traceback to an authoritative source.

1.  **Key Source**: *Nepal Academy Orthography Standard* — published by MoFAGA ([PDF](https://mofaga.gov.np/notice-file/Notices-20211029142422901.pdf)). Local copy: `docs/Notices-pages-77-99.pdf`.
2.  **Secondary Sources**: *LDTA Training Materials* (Government training docs).

### Promotion Flow

How a pair moves from `needs_review.toml` to `gold.toml`:

1.  **Identify**: A discrepancy or ambiguity is found (e.g., *फाउण्डेशन* vs *फाउन्डेसन* where the exam key conflicts with the standard).
2.  **Isolate**: Add it to `needs_review.toml` with a comment explaining the conflict.
3.  **Review**: Consult the *Linguistic Advisory Group* (or check updated prints of the standard).
4.  **Resolve**:
    *   If a specific rule clarifies it, move to `gold.toml`.
    *   If both are valid, mark as such (support multiple correct forms).
5.  **Test**: Ensure the code handles the new case, then commit.
