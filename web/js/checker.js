/**
 * Spell Checker module — mirror-div pattern with diagnostics panel.
 *
 * All category keying uses `d.category_code` (stable Rust enum variant name),
 * while `d.category` is the human-readable Nepali label.
 */
import { checkText } from './wasm-bridge.js';
import { debounce, escapeHtml, CATEGORY_COLORS, CATEGORY_LABELS } from './utils.js';
import { wrapRuleTooltip } from './rules-data.js';
import { initInspector, showInspector, hideInspector, isInspectorActive } from './inspector.js';

let diagnostics = [];
let hiddenCategories = new Set();
let activeCardIndex = -1;
let runtimeErrorMessage = null;
const mobileDiagOverlay = document.getElementById('mobile-diag-overlay');

const editorInput = document.getElementById('editor-input');
const editorBackdrop = document.getElementById('editor-backdrop');
const diagnosticsList = document.getElementById('diagnostics-list');
const errorCount = document.getElementById('error-count');
const fixAllBtn = document.getElementById('fix-all-btn');
const categoryFilters = document.getElementById('category-filters');
const panelCol = document.getElementById('panel-col');
const grammarToggle = document.getElementById('grammar-toggle');
const punctuationStrictToggle = document.getElementById('punctuation-strict-toggle');
const punctuationModeNote = document.getElementById('punctuation-mode-note');
const grammarCoverage = document.getElementById('grammar-coverage');

/**
 * Initialize the spell checker module.
 */
export function initChecker() {
  editorInput.addEventListener('input', debouncedCheck);
  editorInput.addEventListener('scroll', syncScroll);
  editorInput.addEventListener('click', onEditorClick);
  fixAllBtn.addEventListener('click', fixAll);
  grammarToggle?.addEventListener('change', () => runCheck());
  punctuationStrictToggle?.addEventListener('change', () => {
    renderPunctuationModeNote();
    runCheck();
  });
  renderPunctuationModeNote();

  // Initialize the inspector on the panel column
  const panelContent = document.getElementById('panel-content');
  if (panelContent) {
    initInspector(panelContent, {
      onFix: handleInspectorFix,
      onBack: handleInspectorBack,
    });
  }
}

const debouncedCheck = debounce(() => runCheck(), 300);

const HEURISTIC_RULE_LABELS = {
  "samasa-heuristic": "समास",
  "morph-ambiguity": "अस्पष्ट",
  "quantifier-plural-redundancy": "बहुवचन",
  "ergative-le-intransitive": "ले-कारक",
  "genitive-mismatch-plural": "सम्बन्ध",
  "section4-phrase-style": "शैली",
};

function syncScroll() {
  editorBackdrop.scrollTop = editorInput.scrollTop;
  editorBackdrop.scrollLeft = editorInput.scrollLeft;
}

/**
 * Set editor text and run check (used for sample text).
 */
export function setText(text) {
  editorInput.value = text;
  runCheck();
}

function isGrammarEnabled() {
  return Boolean(grammarToggle?.checked);
}

function isPunctuationStrictEnabled() {
  return punctuationStrictToggle?.checked !== false;
}

function isPunctuationStyleDiagnostic(diag) {
  return diag.category_code === "Punctuation" && !isPunctuationStrictEnabled();
}

function isHeuristicDiagnostic(diag) {
  if (isPunctuationStyleDiagnostic(diag)) {
    return true;
  }
  return !(diag.kind === "Error" && diag.confidence >= 0.8);
}

function renderPunctuationModeNote() {
  if (!punctuationModeNote) return;
  punctuationModeNote.textContent = isPunctuationStrictEnabled()
    ? "कडा मोड: विरामचिह्न त्रुटि रूपमा देखाइन्छ।"
    : "शैली मोड: विरामचिह्न सुझाव हुन्, अनिवार्य गल्ती होइनन्।";
}

function getHeuristicRuleLabel(ruleCode) {
  if (!ruleCode) return "heuristic";
  if (HEURISTIC_RULE_LABELS[ruleCode]) return HEURISTIC_RULE_LABELS[ruleCode];
  if (ruleCode.startsWith("section4-")) return "शैली";
  return "heuristic";
}

function heuristicLabel(diag) {
  if (!isHeuristicDiagnostic(diag)) {
    return null;
  }
  if (isPunctuationStyleDiagnostic(diag)) {
    return "विराम शैली";
  }
  return getHeuristicRuleLabel(diag.rule_code);
}

function runCheck() {
  hideMobileDiagOverlay();
  const text = editorInput.value;
  runtimeErrorMessage = null;

  if (!text.trim()) {
    diagnostics = [];
    renderBackdrop(text);
    renderDiagnostics();
    renderFilters();
    renderGrammarCoverage();
    return;
  }

  try {
    diagnostics = checkText(text, { grammar: isGrammarEnabled() });
  } catch (err) {
    console.error('checkText failed', err);
    runtimeErrorMessage = 'जाँच प्रक्रिया असफल भयो। कृपया पृष्ठ रिफ्रेस गरेर फेरि प्रयास गर्नुहोस्।';
    diagnostics = [];
  }

  activeCardIndex = -1;
  renderBackdrop(text);
  renderDiagnostics();
  renderFilters();
  renderGrammarCoverage();
}

function renderGrammarCoverage() {
  if (!grammarCoverage) return;

  const enabled = isGrammarEnabled();
  if (!enabled) {
    grammarCoverage.innerHTML = `
      <div class="grammar-coverage-head">
        <span>Grammar Coverage</span>
        <span class="grammar-coverage-label">Heuristics</span>
      </div>
      <p class="grammar-coverage-note">व्याकरण जाँच बन्द छ।</p>`;
    return;
  }

  if (runtimeErrorMessage) {
    grammarCoverage.innerHTML = `
      <div class="grammar-coverage-head">
        <span>Grammar Coverage</span>
        <span class="grammar-coverage-label">Heuristics</span>
      </div>
      <p class="grammar-coverage-note">${escapeHtml(runtimeErrorMessage)}</p>`;
    return;
  }

  const byRule = new Map();

  for (const d of diagnostics) {
    if (!isHeuristicDiagnostic(d)) continue;
    if (isPunctuationStyleDiagnostic(d)) continue;
    const ruleCode = d.rule_code || "heuristic-unknown";
    const current = byRule.get(ruleCode) || { count: 0, confidenceSum: 0 };
    current.count += 1;
    current.confidenceSum += Number.isFinite(d.confidence) ? d.confidence : 0;
    byRule.set(ruleCode, current);
  }

  if (byRule.size === 0) {
    grammarCoverage.innerHTML = `
      <div class="grammar-coverage-head">
        <span>Grammar Coverage</span>
        <span class="grammar-coverage-label">Heuristics</span>
      </div>
      <p class="grammar-coverage-note">अहिलेसम्म कुनै heuristic/style सुझाव भेटिएन।</p>`;
    return;
  }

  const chips = Array.from(byRule.entries())
    .sort((a, b) => b[1].count - a[1].count || a[0].localeCompare(b[0]))
    .map(([code, stats]) => {
      const avg = Math.round((stats.confidenceSum / stats.count) * 100);
      return `
      <span class="grammar-coverage-chip">
        ${escapeHtml(getHeuristicRuleLabel(code))}
        <span class="grammar-coverage-count">${stats.count}</span>
        <span class="grammar-coverage-avg">${avg}%</span>
      </span>`;
    })
    .join("");

  grammarCoverage.innerHTML = `
    <div class="grammar-coverage-head">
      <span>Grammar Coverage</span>
      <span class="grammar-coverage-label">Heuristics</span>
    </div>
    <p class="grammar-coverage-note">फेला परेका नियम संकेतहरू</p>
    <div class="grammar-coverage-list">${chips}</div>`;
}


/**
 * Render the backdrop with <mark> elements for each diagnostic.
 */
function renderBackdrop(text) {
  if (diagnostics.length === 0) {
    editorBackdrop.textContent = text;
    return;
  }

  // Sort by charStart ascending
  const sorted = [...diagnostics]
    .map((d, i) => ({ ...d, index: i }))
    .sort((a, b) => a.charStart - b.charStart);

  let html = '';
  let pos = 0;

  for (const d of sorted) {
    if (d.charStart < pos) continue; // skip overlaps

    // Text before this error
    if (d.charStart > pos) {
      html += escapeHtml(text.slice(pos, d.charStart));
    }

    const markHidden = hiddenCategories.has(d.category_code) ? ' class="mark-hidden"' : '';
    html += `<mark data-category="${escapeHtml(d.category_code)}" data-index="${d.index}"${markHidden}>${escapeHtml(text.slice(d.charStart, d.charEnd))}</mark>`;
    pos = d.charEnd;
  }

  // Remaining text
  if (pos < text.length) {
    html += escapeHtml(text.slice(pos));
  }

  editorBackdrop.innerHTML = html;
}

/**
 * Render the diagnostics panel.
 */
function renderDiagnostics() {
  if (runtimeErrorMessage) {
    errorCount.textContent = 'जाँच त्रुटि';
    fixAllBtn.disabled = true;
    diagnosticsList.innerHTML = `<p class="diag-empty">${escapeHtml(runtimeErrorMessage)}</p>`;
    return;
  }

  const visibleDiagnostics = diagnostics.filter(
    (d) => !hiddenCategories.has(d.category_code)
  );
  const visibleErrorCount = visibleDiagnostics.filter(
    (d) => !isHeuristicDiagnostic(d)
  ).length;
  const visibleSuggestionCount = visibleDiagnostics.length - visibleErrorCount;

  errorCount.textContent = visibleSuggestionCount > 0
    ? `${visibleErrorCount} त्रुटि, ${visibleSuggestionCount} शैली सुझाव`
    : `${visibleErrorCount} \u0924\u094D\u0930\u0941\u091F\u093F`;
  fixAllBtn.disabled = visibleDiagnostics.length === 0;

  if (diagnostics.length === 0) {
    diagnosticsList.innerHTML =
      '<p class="diag-empty">\u0915\u0941\u0928\u0948 \u0924\u094D\u0930\u0941\u091F\u093F \u092D\u0947\u091F\u093F\u090F\u0928\u0964</p>';
    return;
  }

  diagnosticsList.innerHTML = diagnostics
    .map((d, i) => {
      const hidden = hiddenCategories.has(d.category_code) ? ' hidden' : '';
      const active = i === activeCardIndex ? ' active' : '';
      const code = escapeHtml(d.category_code);
      const tagLabel = heuristicLabel(d);
      const isHeuristic = Boolean(tagLabel);
      const label = isHeuristic
        ? "विवरण"
        : (CATEGORY_LABELS[d.category_code] || d.category);
      const heuristicClass = isHeuristic ? " heuristic" : "";
      const heuristicTag = isHeuristic
        ? `<span class="diag-heuristic-tag">${escapeHtml(tagLabel)}</span>`
        : "";
      const badgeClass = isHeuristic
        ? "diag-badge diag-badge-suggestion"
        : "diag-badge";
      const badgeAttr = isHeuristic ? "" : ` data-category="${code}"`;
      const hasChange = d.incorrect !== d.correction;
      const correctionRow = hasChange
        ? `<div class="diag-correction">
          <span class="diag-incorrect">${escapeHtml(d.incorrect)}</span>
          <span class="diag-arrow">\u2192</span>
          <span class="diag-correct">${escapeHtml(d.correction)}</span>
        </div>`
        : `<div class="diag-correction">
          <span class="diag-incorrect">${escapeHtml(d.incorrect)}</span>
        </div>`;
      const fixButton = hasChange
        ? `<button class="btn btn-sm btn-primary diag-fix" data-index="${i}">\u0938\u091A\u094D\u092F\u093E\u0909\u0928\u0941\u0939\u094B\u0938\u094D</button>`
        : "";
      const confidence = Number.isFinite(d.confidence) ? Math.round(d.confidence * 100) : 0;
      return `
      <div class="diag-card${hidden}${active}${heuristicClass}" data-index="${i}" data-category="${code}">
        <div class="diag-meta">
          <span class="${badgeClass}"${badgeAttr}>${escapeHtml(label)}</span>
          ${heuristicTag}
          <span class="diag-confidence">${confidence}%</span>
        </div>
        ${correctionRow}
        <div class="diag-explanation">${escapeHtml(d.explanation)}</div>
        <div class="diag-rule">${wrapRuleTooltip(d.rule, d.category_code)}</div>
        ${fixButton}
      </div>`;
    })
    .join('');

  // Attach card click handlers
  diagnosticsList.querySelectorAll('.diag-card').forEach((card) => {
    card.addEventListener('click', (e) => {
      if (e.target.classList.contains('diag-fix')) return;
      const idx = parseInt(card.dataset.index);
      setActiveCard(idx);
    });
  });

  // Attach fix button handlers
  diagnosticsList.querySelectorAll('.diag-fix').forEach((btn) => {
    btn.addEventListener('click', (e) => {
      e.stopPropagation();
      fixOne(parseInt(btn.dataset.index));
    });
  });
}

/**
 * Render category filter pills.
 */
function renderFilters() {
  const counts = {};
  for (const d of diagnostics) {
    counts[d.category_code] = (counts[d.category_code] || 0) + 1;
  }

  const categories = Object.keys(counts).sort();
  if (categories.length === 0) {
    categoryFilters.innerHTML = '';
    return;
  }

  categoryFilters.innerHTML = categories
    .map((code) => {
      const inactive = hiddenCategories.has(code) ? ' inactive' : '';
      const color = CATEGORY_COLORS[code] || 'var(--cat-default)';
      const label = CATEGORY_LABELS[code] || code;
      return `<button class="category-pill${inactive}" data-category="${escapeHtml(code)}" style="border-color: ${color}; color: ${color};">
        ${escapeHtml(label)}
        <span class="pill-count">${counts[code]}</span>
      </button>`;
    })
    .join('');

  categoryFilters.querySelectorAll('.category-pill').forEach((pill) => {
    pill.addEventListener('click', () => {
      const code = pill.dataset.category;
      if (hiddenCategories.has(code)) {
        hiddenCategories.delete(code);
      } else {
        hiddenCategories.add(code);
      }
      renderBackdrop(editorInput.value);
      renderDiagnostics();
      renderFilters();
    });
  });
}

/**
 * Set the active diagnostic card and scroll editor to that error.
 */
function setActiveCard(index) {
  activeCardIndex = activeCardIndex === index ? -1 : index;
  renderDiagnostics();

  if (activeCardIndex >= 0) {
    const d = diagnostics[activeCardIndex];
    editorInput.focus();
    editorInput.setSelectionRange(d.charStart, d.charEnd);
  }
}

/**
 * Whether we're on a narrow (mobile) viewport where the panel is hidden.
 */
function isMobileView() {
  return window.innerWidth <= 768;
}

/**
 * Show a single diagnostic as an overlay card inside the editor (mobile).
 */
function showMobileDiagOverlay(d, idx) {
  if (!mobileDiagOverlay) return;
  const hasChange = d.incorrect !== d.correction;
  const label = CATEGORY_LABELS[d.category_code] || d.category;
  const code = escapeHtml(d.category_code);
  mobileDiagOverlay.innerHTML = `
    <div class="diag-meta">
      <span class="diag-badge" data-category="${code}">${escapeHtml(label)}</span>
      <button class="mobile-diag-dismiss" aria-label="Close">&times;</button>
    </div>
    ${hasChange
      ? `<div class="diag-correction">
          <span class="diag-incorrect">${escapeHtml(d.incorrect)}</span>
          <span class="diag-arrow">\u2192</span>
          <span class="diag-correct">${escapeHtml(d.correction)}</span>
        </div>`
      : `<div class="diag-correction"><span class="diag-incorrect">${escapeHtml(d.incorrect)}</span></div>`
    }
    <div class="diag-explanation">${escapeHtml(d.explanation)}</div>
    <div class="diag-rule">${wrapRuleTooltip(d.rule, d.category_code)}</div>
    <div class="mobile-diag-actions">
      ${hasChange ? `<button class="btn btn-sm btn-primary" id="mobile-fix-btn">सच्याउनुहोस्</button>` : ''}
    </div>`;
  mobileDiagOverlay.classList.add('visible');

  // Fix button
  const fixBtn = mobileDiagOverlay.querySelector('#mobile-fix-btn');
  if (fixBtn) {
    fixBtn.addEventListener('click', (e) => {
      e.stopPropagation();
      fixOne(idx);
      hideMobileDiagOverlay();
    });
  }

  // Dismiss button
  mobileDiagOverlay.querySelector('.mobile-diag-dismiss')
    ?.addEventListener('click', (e) => {
      e.stopPropagation();
      hideMobileDiagOverlay();
    });
}

function hideMobileDiagOverlay() {
  if (mobileDiagOverlay) {
    mobileDiagOverlay.classList.remove('visible');
    mobileDiagOverlay.innerHTML = '';
  }
}

/**
 * Handle click in editor — show inspector for clicked word, or highlight diagnostic.
 */
function onEditorClick() {
  const pos = editorInput.selectionStart;
  const text = editorInput.value;

  // Check if click is on a diagnostic
  const idx = diagnostics.findIndex(
    (d) => pos >= d.charStart && pos < d.charEnd && !hiddenCategories.has(d.category_code)
  );

  if (idx >= 0) {
    if (isMobileView()) {
      // Mobile: show overlay card inside editor
      showMobileDiagOverlay(diagnostics[idx], idx);
      return;
    }
    activeCardIndex = idx;
    renderDiagnostics();
    const card = diagnosticsList.querySelector(`[data-index="${idx}"]`);
    if (card) card.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
  } else if (isMobileView()) {
    hideMobileDiagOverlay();
  }

  // Show inspector for clicked word (desktop only — on mobile the panel is hidden)
  if (isMobileView()) return;

  const wordInfo = getWordAtCursor(text, pos);
  if (wordInfo) {
    // Hide diagnostics panel content, show inspector
    hideDiagnosticsPanel();
    showInspector(wordInfo.word, wordInfo.start, wordInfo.end);
  } else if (isInspectorActive()) {
    // Clicked whitespace — restore diagnostics
    hideInspector();
    restoreDiagnosticsPanel();
  }
}

/**
 * Extract the Devanagari word at a given cursor position, with start/end indices.
 */
function getWordAtCursor(text, pos) {
  if (!text || pos < 0 || pos > text.length) return null;
  const isDevanagariWord = (c) => {
    if (!c) return false;
    const cp = c.charCodeAt(0);
    return cp >= 0x0900 && cp <= 0x0963;
  };
  let start = pos;
  let end = pos;
  while (start > 0 && isDevanagariWord(text[start - 1])) start--;
  while (end < text.length && isDevanagariWord(text[end])) end++;
  if (start === end) return null;
  return { word: text.slice(start, end), start, end };
}

/**
 * Hide the diagnostics panel elements (diag header + list).
 */
function hideDiagnosticsPanel() {
  const header = panelCol?.querySelector('.diag-header');
  if (header) header.style.display = 'none';
  if (diagnosticsList) diagnosticsList.style.display = 'none';
}

/**
 * Restore the diagnostics panel elements.
 */
function restoreDiagnosticsPanel() {
  const header = panelCol?.querySelector('.diag-header');
  if (header) header.style.display = '';
  if (diagnosticsList) diagnosticsList.style.display = '';
}

/**
 * Handle fix from inspector — apply correction, restore diagnostics, re-run check.
 */
function handleInspectorFix(start, end, correction) {
  const text = editorInput.value;
  editorInput.value = text.slice(0, start) + correction + text.slice(end);
  restoreDiagnosticsPanel();
  runCheck();
}

/**
 * Handle back from inspector — restore diagnostics panel.
 */
function handleInspectorBack() {
  restoreDiagnosticsPanel();
}

/**
 * Fix a single diagnostic: replace the incorrect span with correction.
 */
function fixOne(index) {
  const d = diagnostics[index];
  const text = editorInput.value;
  editorInput.value =
    text.slice(0, d.charStart) + d.correction + text.slice(d.charEnd);
  runCheck();
}

/**
 * Fix all visible diagnostics, applying in reverse offset order.
 */
function fixAll() {
  const visible = diagnostics
    .filter((d) => !hiddenCategories.has(d.category_code))
    .sort((a, b) => b.charStart - a.charStart);

  let text = editorInput.value;
  for (const d of visible) {
    text = text.slice(0, d.charStart) + d.correction + text.slice(d.charEnd);
  }
  editorInput.value = text;
  runCheck();
}
