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
    *   The primary lexicon containing ~51k+ correctly verified headwords.
    *   **Usage**: Compiled into the FST (Finite State Transducer) by `varnavinyas-kosha` for sub-microsecond validation.

## Provenance Policy

Every entry in our datasets must have a traceback to an authoritative source.

1.  **Primary Source**: *Nepal Academy Orthography Standard* (PDF).
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
