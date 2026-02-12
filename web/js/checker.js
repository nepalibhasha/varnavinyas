/**
 * Spell Checker module — mirror-div pattern with diagnostics panel.
 *
 * All category keying uses `d.category_code` (stable Rust enum variant name),
 * while `d.category` is the human-readable Nepali label.
 */
import { checkText, analyzeWord } from './wasm-bridge.js';
import { debounce, escapeHtml, CATEGORY_COLORS, CATEGORY_LABELS, ORIGIN_LABELS } from './utils.js';
import { getTooltipForRule, getCategoryForRule, RULE_TOOLTIPS } from './rules-data.js';

/** Feature flag for word analysis panel. Set to true to enable. */
const FEATURE_ANALYSIS = true;

let diagnostics = [];
let hiddenCategories = new Set();
let activeCardIndex = -1;

const editorInput = document.getElementById('editor-input');
const editorBackdrop = document.getElementById('editor-backdrop');
const diagnosticsList = document.getElementById('diagnostics-list');
const errorCount = document.getElementById('error-count');
const fixAllBtn = document.getElementById('fix-all-btn');
const categoryFilters = document.getElementById('category-filters');

/**
 * Initialize the spell checker module.
 */
export function initChecker() {
  editorInput.addEventListener('input', debouncedCheck);
  editorInput.addEventListener('scroll', syncScroll);
  editorInput.addEventListener('click', onEditorClick);
  fixAllBtn.addEventListener('click', fixAll);
}

const debouncedCheck = debounce(() => runCheck(), 300);

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

function runCheck() {
  const text = editorInput.value;
  if (!text.trim()) {
    diagnostics = [];
    renderBackdrop(text);
    renderDiagnostics();
    renderFilters();
    return;
  }

  try {
    diagnostics = checkText(text);
  } catch {
    diagnostics = [];
  }

  activeCardIndex = -1;
  renderBackdrop(text);
  renderDiagnostics();
  renderFilters();
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
  const visibleCount = diagnostics.filter(
    (d) => !hiddenCategories.has(d.category_code)
  ).length;

  errorCount.textContent = `${visibleCount} त्रुटि`;
  fixAllBtn.disabled = visibleCount === 0;

  if (diagnostics.length === 0) {
    diagnosticsList.innerHTML =
      '<p class="diag-empty">कुनै त्रुटि भेटिएन।</p>';
    return;
  }

  diagnosticsList.innerHTML = diagnostics
    .map((d, i) => {
      const hidden = hiddenCategories.has(d.category_code) ? ' hidden' : '';
      const active = i === activeCardIndex ? ' active' : '';
      const code = escapeHtml(d.category_code);
      const label = CATEGORY_LABELS[d.category_code] || d.category;
      return `
      <div class="diag-card${hidden}${active}" data-index="${i}" data-category="${code}">
        <span class="diag-badge" data-category="${code}">${escapeHtml(label)}</span>
        <div class="diag-correction">
          <span class="diag-incorrect">${escapeHtml(d.incorrect)}</span>
          <span class="diag-arrow">→</span>
          <span class="diag-correct">${escapeHtml(d.correction)}</span>
        </div>
        <div class="diag-explanation">${escapeHtml(d.explanation)}</div>
        <div class="diag-rule">${wrapRuleTooltip(d.rule, d.category_code)}</div>
        <button class="btn btn-sm btn-primary diag-fix" data-index="${i}">सच्याउनुहोस्</button>
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
 * Handle click in editor — find diagnostic at cursor position, or analyze clicked word.
 */
function onEditorClick() {
  const pos = editorInput.selectionStart;
  const idx = diagnostics.findIndex(
    (d) => pos >= d.charStart && pos < d.charEnd && !hiddenCategories.has(d.category_code)
  );
  if (idx >= 0) {
    activeCardIndex = idx;
    renderDiagnostics();
    // Scroll card into view
    const card = diagnosticsList.querySelector(`[data-index="${idx}"]`);
    if (card) card.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
  }

  // Feature-flagged: analyze word at cursor
  if (FEATURE_ANALYSIS) {
    const word = getWordAtCursor(editorInput.value, pos);
    if (word) {
      renderAnalysisPanel(word);
    }
  }
}

/**
 * Extract the Devanagari word at a given cursor position.
 */
function getWordAtCursor(text, pos) {
  if (!text || pos < 0 || pos > text.length) return null;
  // Walk left and right to find word boundaries.
  // Includes Devanagari letters, matras, signs (0900-0963) but excludes
  // danda/double-danda (0964-0965) and digits (0966-096F).
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
  return text.slice(start, end);
}

/**
 * Render the word analysis panel below the diagnostics.
 */
function renderAnalysisPanel(word) {
  let panel = document.getElementById('analysis-panel');
  if (!panel) {
    panel = document.createElement('div');
    panel.id = 'analysis-panel';
    panel.className = 'analysis-panel';
    diagnosticsList.parentElement.appendChild(panel);
  }

  try {
    const analysis = analyzeWord(word);
    const originLabel = ORIGIN_LABELS[analysis.origin] || analysis.origin;
    const originClass = `origin-${analysis.origin}`;
    const statusIcon = analysis.is_correct ? 'correct' : 'incorrect';
    const statusLabel = analysis.is_correct ? 'शुद्ध' : 'अशुद्ध';

    let html = `
      <div class="analysis-header">
        <span class="analysis-word">${escapeHtml(analysis.word)}</span>
        <span class="origin-badge ${originClass}">${escapeHtml(originLabel)}</span>
        <span class="analysis-status ${statusIcon}">${statusLabel}</span>
      </div>`;

    if (analysis.correction) {
      html += `
      <div class="analysis-correction">
        <span class="diag-incorrect">${escapeHtml(analysis.word)}</span>
        <span class="diag-arrow">\u2192</span>
        <span class="diag-correct">${escapeHtml(analysis.correction)}</span>
      </div>`;
    }

    if (analysis.rule_notes && analysis.rule_notes.length > 0) {
      html += '<div class="analysis-notes">';
      for (const note of analysis.rule_notes) {
        html += `
        <div class="analysis-note">
          <span class="analysis-note-rule">${wrapRuleTooltip(note.rule)}</span>
          <span class="analysis-note-text">${escapeHtml(note.explanation)}</span>
        </div>`;
      }
      html += '</div>';
    }

    panel.innerHTML = html;
    panel.hidden = false;
  } catch {
    panel.hidden = true;
  }
}

/**
 * Wrap a rule citation in a tooltip-enabled span.
 */
function wrapRuleTooltip(ruleText, categoryCode) {
  const cat = categoryCode || getCategoryForRule(ruleText);
  const tooltip = (cat && RULE_TOOLTIPS[cat]) || getTooltipForRule(ruleText);
  if (tooltip && cat) {
    return `<span class="rule-ref" data-tooltip="${escapeHtml(tooltip)}" data-category="${escapeHtml(cat)}">${escapeHtml(ruleText)}</span>`;
  }
  if (tooltip) {
    return `<span class="rule-ref" data-tooltip="${escapeHtml(tooltip)}">${escapeHtml(ruleText)}</span>`;
  }
  return escapeHtml(ruleText);
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
