/**
 * WASM bridge — loads the module and re-exports functions with JSON parsing.
 * Handles byte-offset to char-index conversion for Devanagari text.
 */
import init, {
  check_text,
  check_text_with_options,
  check_text_value,
  check_word,
  check_word_value,
  transliterate as wasmTransliterate,
  derive,
  derive_value,
  analyze_word,
  analyze_word_value,
  decompose_word,
  decompose_word_value,
  sandhi_apply,
  sandhi_split,
} from '../pkg/varnavinyas_bindings_wasm.js';

let initialized = false;

/**
 * Initialize the WASM module. Must be called before any other function.
 */
export async function initialize() {
  if (initialized) return;
  await init();
  initialized = true;
}

/**
 * Convert a UTF-8 byte offset to a JavaScript string char index.
 * Devanagari chars are typically 3 bytes in UTF-8 but 1 code unit in UTF-16.
 */
export function byteOffsetToCharIndex(text, byteOffset) {
  let bytePos = 0;
  for (let i = 0; i < text.length; i++) {
    if (bytePos >= byteOffset) return i;
    const cp = text.codePointAt(i);
    if (cp <= 0x7f) bytePos += 1;
    else if (cp <= 0x7ff) bytePos += 2;
    else if (cp <= 0xffff) bytePos += 3;
    else {
      bytePos += 4;
      i++; // surrogate pair
    }
  }
  return text.length;
}

/**
 * Check a full text for spelling/punctuation issues.
 * Returns an array of diagnostics with char-index spans.
 */
export function checkText(text, options = {}) {
  const { grammar = false } = options;

  let raw;
  try {
    raw = check_text_value(text, grammar);
  } catch (_err) {
    raw = grammar
      ? JSON.parse(check_text_with_options(text, grammar))
      : JSON.parse(check_text(text));
  }

  return raw.map((d) => ({
    ...d,
    charStart: byteOffsetToCharIndex(text, d.span_start),
    charEnd: byteOffsetToCharIndex(text, d.span_end),
  }));
}

/**
 * Check a single word.
 * Returns a diagnostic object or null.
 */
export function checkWord(word) {
  let raw;
  try {
    raw = check_word_value(word);
  } catch (_err) {
    raw = JSON.parse(check_word(word));
  }

  if (!raw) return null;
  return {
    ...raw,
    charStart: byteOffsetToCharIndex(word, raw.span_start),
    charEnd: byteOffsetToCharIndex(word, raw.span_end),
  };
}

/**
 * Transliterate text between scripts.
 * @param {string} input
 * @param {string} from - "Devanagari" or "Iast"
 * @param {string} to - "Devanagari" or "Iast"
 * @returns {string}
 */
export function transliterate(input, from, to) {
  return wasmTransliterate(input, from, to);
}

/**
 * Derive the correct form of a word with step tracing.
 * Returns { input, output, is_correct, steps: [{rule, description, before, after}] }
 */
export function deriveWord(word) {
  try {
    return derive_value(word);
  } catch (_err) {
    return JSON.parse(derive(word));
  }
}

/**
 * Analyze a word: get origin, correction, and explanatory rule notes.
 * Returns { word, origin, is_correct, correction, rule_notes: [{rule, explanation}] }
 */
export function analyzeWord(word) {
  try {
    return analyze_word_value(word);
  } catch (_err) {
    return JSON.parse(analyze_word(word));
  }
}

/**
 * Decompose a word into morphemes: root, prefixes, suffixes, and origin.
 * Returns { root, prefixes: string[], suffixes: string[], origin }
 */
export function decomposeWord(word) {
  try {
    return decompose_word_value(word);
  } catch (_err) {
    return JSON.parse(decompose_word(word));
  }
}

/**
 * Apply sandhi: join two morphemes.
 * Returns { output, sandhi_type, rule_citation } or { error: "..." }
 */
export function sandhiApply(first, second) {
  return JSON.parse(sandhi_apply(first, second));
}

/**
 * Split a word at sandhi boundaries.
 * Returns [{ left, right, output, sandhi_type, rule_citation }, ...]
 */
export function sandhiSplit(word) {
  return JSON.parse(sandhi_split(word));
}
