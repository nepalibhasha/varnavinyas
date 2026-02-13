/**
 * Word Inspector module — contextual analysis panel for clicked words.
 *
 * Shows origin, correction, morphology, sandhi splits, derivation steps,
 * and rule notes for the selected word.
 */
import { analyzeWord, deriveWord, decomposeWord, sandhiSplit } from './wasm-bridge.js';
import { escapeHtml, ORIGIN_LABELS } from './utils.js';
import { wrapRuleTooltip } from './rules-data.js';

/** Feature flag for word inspector. Set to false to disable. */
const FEATURE_WORD_INSPECTOR = true;

const SANDHI_TYPE_LABELS = {
  VowelSandhi: '\u0938\u094D\u0935\u0930 \u0938\u0928\u094D\u0927\u093F',
  VisargaSandhi: '\u0935\u093F\u0938\u0930\u094D\u0917 \u0938\u0928\u094D\u0927\u093F',
  ConsonantSandhi: '\u0935\u094D\u092F\u091E\u094D\u091C\u0928 \u0938\u0928\u094D\u0927\u093F',
};

const SANDHI_TYPE_CLASS = {
  VowelSandhi: 'sandhi-type-vowel',
  VisargaSandhi: 'sandhi-type-visarga',
  ConsonantSandhi: 'sandhi-type-consonant',
};

/** Case markers (postpositions) — matches Rust tables::CASE_MARKERS */
const CASE_MARKERS = new Set([
  '\u092D\u093F\u0924\u094D\u0930', '\u0926\u0947\u0916\u093F',
  '\u0932\u093E\u0908', '\u092C\u093E\u091F', '\u0938\u0901\u0917',
  '\u0924\u093F\u0930', '\u0915\u093E', '\u0915\u0940',
  '\u0932\u0947', '\u0915\u094B', '\u092E\u093E',
]);

/** Plural markers — matches Rust tables::PLURAL_MARKERS */
const PLURAL_MARKERS = new Set(['\u0939\u0930\u0942', '\u0939\u0930\u0941']);

/**
 * Classify a suffix and return { cssClass, label }.
 */
function classifySuffix(s) {
  if (CASE_MARKERS.has(s)) return { cssClass: 'morpheme-case', label: '\u0935\u093F\u092D\u0915\u094D\u0924\u093F' };
  if (PLURAL_MARKERS.has(s)) return { cssClass: 'morpheme-plural', label: '\u092C\u0939\u0941\u0935\u091A\u0928' };
  return { cssClass: 'morpheme-suffix', label: '\u092A\u094D\u0930\u0924\u094D\u092F\u092F' };
}

let panelEl = null;
let callbacks = {};
let active = false;
let currentWord = '';

/**
 * Initialize the inspector on a given panel element.
 * @param {HTMLElement} el - The panel container element
 * @param {{ onFix: Function, onBack: Function }} cbs - Callbacks
 */
export function initInspector(el, cbs) {
  panelEl = el;
  callbacks = cbs || {};
}

/**
 * Show the inspector for a word.
 * @param {string} word - The clicked word
 * @param {number} start - Char start index in editor
 * @param {number} end - Char end index in editor
 */
export function showInspector(word, start, end) {
  if (!FEATURE_WORD_INSPECTOR || !panelEl) return;
  if (!word || !word.trim()) return;

  active = true;
  currentWord = word;

  let html = '';

  // Back button
  html += `<button class="btn btn-sm panel-back-btn" id="inspector-back">\u2190 \u0924\u094D\u0930\u0941\u091F\u093F \u0938\u0942\u091A\u0940</button>`;

  // --- Header: word + origin + correctness ---
  let analysis = null;
  try {
    analysis = analyzeWord(word);
  } catch {
    // no analysis available
  }

  html += '<div class="inspector-word-header">';
  html += `<span class="inspector-word">${escapeHtml(word)}</span>`;
  if (analysis) {
    const originLabel = ORIGIN_LABELS[analysis.origin] || analysis.origin;
    const originClass = `origin-${analysis.origin}`;
    html += ` <span class="origin-badge ${originClass}">${escapeHtml(originLabel)}</span>`;
    const statusIcon = analysis.is_correct ? 'correct' : 'incorrect';
    const statusLabel = analysis.is_correct ? '\u0936\u0941\u0926\u094D\u0927' : '\u0905\u0936\u0941\u0926\u094D\u0927';
    html += ` <span class="analysis-status ${statusIcon}">${statusLabel}</span>`;
  }
  html += '</div>';

  // --- Correction section ---
  if (analysis && analysis.correction) {
    html += `
    <div class="inspector-fix-section">
      <div class="analysis-correction">
        <span class="diag-incorrect">${escapeHtml(analysis.word)}</span>
        <span class="diag-arrow">\u2192</span>
        <span class="diag-correct">${escapeHtml(analysis.correction)}</span>
      </div>
      <button class="btn btn-sm btn-primary" id="inspector-fix" data-start="${start}" data-end="${end}" data-correction="${escapeHtml(analysis.correction)}">\u0938\u091A\u094D\u092F\u093E\u0909\u0928\u0941\u0939\u094B\u0938\u094D</button>
    </div>`;
  }

  // --- Morphology ---
  // Get morphological root so sandhi splitting operates on the stem,
  // not the inflected form (e.g., "रामसँग" → root "राम").
  let morphRoot = word;
  try {
    const m = decomposeWord(word);
    if (m.root) morphRoot = m.root;
  } catch { /* use full word as fallback */ }
  html += renderMorphologySection(word);

  // --- Sandhi splits (on morphological root) ---
  html += renderSandhiSection(morphRoot);

  // --- Derivation steps ---
  html += renderDerivationSection(word);

  // --- Rule notes ---
  if (analysis && analysis.rule_notes && analysis.rule_notes.length > 0) {
    html += '<div class="inspector-section">';
    html += `<div class="inspector-section-title">\u0928\u093F\u092F\u092E \u091F\u093F\u092A\u094D\u092A\u0923\u0940 <span class="inspector-section-label">Rule Notes</span></div>`;
    html += '<div class="analysis-notes">';
    for (const note of analysis.rule_notes) {
      html += `
      <div class="analysis-note">
        <span class="analysis-note-rule">${wrapRuleTooltip(note.rule)}</span>
        <span class="analysis-note-text">${escapeHtml(note.explanation)}</span>
      </div>`;
    }
    html += '</div></div>';
  }

  panelEl.innerHTML = `<div class="inspector-container">${html}</div>`;

  // Attach event handlers
  const backBtn = panelEl.querySelector('#inspector-back');
  if (backBtn) {
    backBtn.addEventListener('click', () => {
      hideInspector();
      if (callbacks.onBack) callbacks.onBack();
    });
  }

  const fixBtn = panelEl.querySelector('#inspector-fix');
  if (fixBtn) {
    fixBtn.addEventListener('click', () => {
      const s = parseInt(fixBtn.dataset.start);
      const e = parseInt(fixBtn.dataset.end);
      const correction = fixBtn.dataset.correction;
      hideInspector();
      if (callbacks.onFix) callbacks.onFix(s, e, correction);
    });
  }
}

/**
 * Hide the inspector and clear the panel.
 */
export function hideInspector() {
  active = false;
  currentWord = '';
  if (panelEl) panelEl.innerHTML = '';
}

/**
 * Check if the inspector is currently active.
 */
export function isInspectorActive() {
  return active;
}

// --- Internal rendering helpers ---

function renderMorphologySection(word) {
  try {
    const m = decomposeWord(word);
    const hasPrefixes = m.prefixes && m.prefixes.length > 0;
    const hasSuffixes = m.suffixes && m.suffixes.length > 0;
    if (!hasPrefixes && !hasSuffixes) return '';

    let parts = '';
    for (const p of (m.prefixes || [])) {
      parts += `
        <span class="morpheme morpheme-prefix">
          ${escapeHtml(p)}
          <span class="morpheme-label">\u0909\u092A\u0938\u0930\u094D\u0917</span>
        </span>
        <span class="morpheme-sep">+</span>`;
    }
    parts += `
      <span class="morpheme morpheme-root">
        ${escapeHtml(m.root)}
        <span class="morpheme-label">\u092E\u0942\u0932</span>
      </span>`;
    for (const s of (m.suffixes || [])) {
      const { cssClass, label } = classifySuffix(s);
      parts += `
        <span class="morpheme-sep">+</span>
        <span class="morpheme ${cssClass}">
          ${escapeHtml(s)}
          <span class="morpheme-label">${escapeHtml(label)}</span>
        </span>`;
    }

    return `
    <div class="inspector-section">
      <div class="inspector-section-title">\u0936\u092C\u094D\u0926 \u0935\u093F\u0936\u094D\u0932\u0947\u0937\u0923 <span class="inspector-section-label">Morphology</span></div>
      <div class="morphology-display">${parts}</div>
    </div>`;
  } catch {
    return '';
  }
}

function renderSandhiSection(word) {
  try {
    const results = sandhiSplit(word);
    if (!results || results.length === 0) return '';

    const rows = results.map((r) => {
      const typeLabel = SANDHI_TYPE_LABELS[r.sandhi_type] || r.sandhi_type;
      const typeClass = SANDHI_TYPE_CLASS[r.sandhi_type] || '';
      return `
        <div class="sandhi-split-row">
          <span class="sandhi-split-parts">${escapeHtml(r.left)} + ${escapeHtml(r.right)}</span>
          <span class="sandhi-type-badge ${typeClass}">${escapeHtml(typeLabel)}</span>
          <span class="sandhi-citation">${escapeHtml(r.rule_citation)}</span>
        </div>`;
    }).join('');

    return `
    <div class="inspector-section">
      <div class="inspector-section-title">\u0938\u0928\u094D\u0927\u093F \u0935\u093F\u091A\u094D\u091B\u0947\u0926 <span class="inspector-section-label">Sandhi Splits</span></div>
      ${rows}
    </div>`;
  } catch {
    return '';
  }
}

function renderDerivationSection(word) {
  try {
    const result = deriveWord(word);
    if (!result.steps || result.steps.length === 0) return '';

    const rows = result.steps.map((s, i) => `
      <tr>
        <td>${i + 1}</td>
        <td class="rule-cell">${wrapRuleTooltip(s.rule)}</td>
        <td>${escapeHtml(s.description)}</td>
        <td>${escapeHtml(s.before)}</td>
        <td>${escapeHtml(s.after)}</td>
      </tr>`).join('');

    return `
    <div class="inspector-section">
      <div class="inspector-section-title">\u0928\u093F\u092F\u092E \u091A\u0930\u0923\u0939\u0930\u0942 <span class="inspector-section-label">Derivation Steps</span></div>
      <table class="steps-table">
        <thead>
          <tr>
            <th>#</th>
            <th>\u0928\u093F\u092F\u092E</th>
            <th>\u0935\u093F\u0935\u0930\u0923</th>
            <th>\u092A\u0939\u093F\u0932\u0947</th>
            <th>\u092A\u091B\u093F</th>
          </tr>
        </thead>
        <tbody>${rows}</tbody>
      </table>
    </div>`;
  } catch {
    return '';
  }
}
