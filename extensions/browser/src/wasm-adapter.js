/**
 * WASM adapter — loads varnavinyas WASM module in the popup context
 * and exposes stable analysis functions.
 *
 * Based on Phase 0 spike findings:
 * - Popup async init() via chrome.runtime.getURL() works reliably
 * - Cold init: ~120ms, well under 1s target
 * - Idempotent: safe to call ensureInit() on every popup open
 */

import init, * as wasmBindings from '../pkg/varnavinyas_bindings_wasm.js';

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
    if (typeof wasmBindings.analyze_word_value !== 'function') {
      return { word, error: 'analyze_word_value unavailable' };
    }
    return wasmBindings.analyze_word_value(word);
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
    if (typeof wasmBindings.check_word_value !== 'function') {
      return null;
    }
    return wasmBindings.check_word_value(word);
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
    if (typeof wasmBindings.decompose_word_value !== 'function') {
      return { word, error: 'decompose_word_value unavailable' };
    }
    return wasmBindings.decompose_word_value(word);
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
    if (typeof wasmBindings.sandhi_split_value !== 'function') {
      return [];
    }
    return wasmBindings.sandhi_split_value(word);
  } catch (err) {
    return [];
  }
}

/**
 * Return the single best compound-safe sandhi split, or null.
 * Applies lexicon and minimum-length guards (mirrors wasm-bridge.js).
 */
export function sandhiSplitBestForCompound(word) {
  try {
    if (typeof wasmBindings.sandhi_split_best_for_compound_value !== 'function') {
      return null;
    }
    return wasmBindings.sandhi_split_best_for_compound_value(word);
  } catch (err) {
    return null;
  }
}

/**
 * Analyze a word as a potential compound (samasa).
 * Returns [{ left, right, samasa_type, score, vigraha }, ...]
 */
export function analyzeCompound(word) {
  try {
    if (typeof wasmBindings.analyze_compound_value !== 'function') {
      return [];
    }
    return wasmBindings.analyze_compound_value(word);
  } catch (err) {
    return [];
  }
}

/**
 * Normalize a word for dictionary lookup.
 * Falls back to the decomposition root only when the original word is not
 * itself a known headword — prevents प्रयोग → योग mis-lookups where the
 * input is already a valid dictionary entry.
 */
export function normalizeQuery(word) {
  const analysis = analyzeWord(word);

  // If the word is a known lexicon headword, look it up directly.
  // origin_source === 'kosha' means it matched a dictionary entry — don't
  // strip it to a root (prevents प्रयोग → योग mis-lookups).
  if (analysis && !analysis.error && analysis.origin_source === 'kosha') {
    return word;
  }

  // Unknown word: try the decomposed root (suffix-stripped lemma).
  const decomposition = decomposeWord(word);
  if (decomposition.root && decomposition.root !== word) {
    return decomposition.root;
  }

  // Last resort: if there's a correction, look that up instead.
  if (analysis && analysis.correction) {
    return analysis.correction;
  }

  return word;
}
