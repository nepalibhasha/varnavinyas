/**
 * WASM adapter — loads varnavinyas WASM module in the popup context
 * and exposes stable analysis functions.
 *
 * Based on Phase 0 spike findings:
 * - Popup async init() via chrome.runtime.getURL() works reliably
 * - Cold init: ~120ms, well under 1s target
 * - Idempotent: safe to call ensureInit() on every popup open
 */

import init, {
  analyze_word_value,
  check_word_value,
  decompose_word_value,
  sandhi_split_value,
  analyze_compound_value,
} from '../pkg/varnavinyas_bindings_wasm.js';

let initialized = false;
let initPromise = null;

/**
 * Initialize the WASM module. Idempotent — safe to call multiple times.
 * Returns the init time in milliseconds.
 */
export async function ensureInit() {
  if (initialized) return 0;

  // Coalesce concurrent callers
  if (initPromise) return initPromise;

  initPromise = (async () => {
    const wasmUrl = chrome.runtime.getURL(
      'pkg/varnavinyas_bindings_wasm_bg.wasm'
    );
    const t0 = performance.now();
    try {
      await init({ module_or_path: wasmUrl });
      const elapsed = performance.now() - t0;
      initialized = true;
      return elapsed;
    } finally {
      // Allow retry after a failed init attempt.
      initPromise = null;
    }
  })();

  return initPromise;
}

/**
 * Analyze a word: origin, correction, rule notes.
 * Returns { word, origin, is_correct, correction, rule_notes }
 */
export function analyzeWord(word) {
  try {
    return analyze_word_value(word);
  } catch (err) {
    return { word, error: err.message };
  }
}

/**
 * Check a word for spelling/punctuation issues.
 * Returns diagnostic object or null if correct.
 */
export function checkWord(word) {
  try {
    return check_word_value(word);
  } catch (err) {
    return null;
  }
}

/**
 * Decompose a word into morphemes.
 * Returns { root, prefixes, suffixes, origin }
 */
export function decomposeWord(word) {
  try {
    return decompose_word_value(word);
  } catch (err) {
    return { word, error: err.message };
  }
}

/**
 * Split a word at sandhi boundaries.
 * Returns [{ left, right, sandhi_type, rule_citation }, ...]
 */
export function sandhiSplit(word) {
  try {
    return sandhi_split_value(word);
  } catch (err) {
    return [];
  }
}

/**
 * Analyze a word as a potential compound (samasa).
 * Returns [{ left, right, samasa_type, score, vigraha }, ...]
 */
export function analyzeCompound(word) {
  try {
    return analyze_compound_value(word);
  } catch (err) {
    return [];
  }
}

/**
 * Normalize a word for dictionary lookup.
 * Strips suffixes to find the base/lemma form.
 */
export function normalizeQuery(word) {
  const decomposition = decomposeWord(word);
  if (decomposition.root && decomposition.root !== word) {
    return decomposition.root;
  }

  const analysis = analyzeWord(word);
  if (analysis.correction) {
    return analysis.correction;
  }

  return word;
}
