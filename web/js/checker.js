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
let dismissedDiagnosticKeys = new Set();
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
const reviewPrevBtn = document.getElementById('review-prev-btn');
const reviewNextBtn = document.getElementById('review-next-btn');
const reviewProgress = document.getElementById('review-progress');
const applyHardBtn = document.getElementById('apply-hard-btn');
const copyCorrectedBtn = document.getElementById('copy-corrected-btn');
const togglePreviewBtn = document.getElementById('toggle-preview-btn');
const previewPanel = document.getElementById('preview-panel');
let previewOpen = false;

/**
 * Initialize the spell checker module.
 */
export function initChecker() {
  editorInput.addEventListener('input', debouncedCheck);
  editorInput.addEventListener('scroll', syncScroll);
  editorInput.addEventListener('click', onEditorClick);
  fixAllBtn.addEventListener('click', fixAll);
  reviewPrevBtn?.addEventListener('click', goToPreviousDiagnostic);
  reviewNextBtn?.addEventListener('click', goToNextDiagnostic);
  applyHardBtn?.addEventListener('click', applyHardErrors);
  copyCorrectedBtn?.addEventListener('click', copyCorrectedText);
  togglePreviewBtn?.addEventListener('click', togglePreviewPanel);
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

const FILTER_CATEGORY_ORDER = [
  "HrasvaDirgha",
  "Chandrabindu",
  "Halanta",
  "Punctuation",
  "ShaShaS",
  "RiKri",
  "YaE",
  "KshaChhya",
  "GyaGyan",
  "AadhiVriddhi",
  "Sandhi",
  "ShuddhaTable",
];

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

function primaryCategoryLabel(diag) {
  switch (diag.category_code) {
    case "HrasvaDirgha":
      return "ह्रस्व/दीर्घ";
    case "Punctuation":
    case "Chandrabindu":
      return "चिह्न";
    case "ShuddhaTable":
    case "AadhiVriddhi":
      return "शुद्ध/अशुद्ध";
    case "Halanta":
      return "हलन्त/अजन्त";
    case "ShaShaS":
    case "RiKri":
    case "YaE":
    case "KshaChhya":
    case "GyaGyan":
      return "उस्तै उच्चारण";
    default:
      return CATEGORY_LABELS[diag.category_code] || diag.category;
  }
}

function diagnosticStateLabel(diag) {
  if (isHeuristicDiagnostic(diag)) return "सुझाव";
  if (diag.kind === "Variant") return "वैकल्पिक";
  return null;
}

function diagnosticKindClass(diag) {
  if (isHeuristicDiagnostic(diag)) return "suggestion";
  if (diag.kind === "Variant") return "variant";
  return "error";
}

function diagnosticGuidance(diag) {
  if (isHeuristicDiagnostic(diag)) {
    return "यो सन्दर्भअनुसार छान्ने सुझाव हो।";
  }
  if (diag.kind === "Variant") {
    return "यो अनिवार्य त्रुटि होइन; मानक वैकल्पिक रूपसम्बन्धी सूचना हो।";
  }
  return "यो मुख्य मानक त्रुटि हो। चाहनुहुन्छ भने सिधै सच्याउन सकिन्छ।";
}

function diagnosticKey(d) {
  return [
    d.charStart,
    d.charEnd,
    d.rule_code || '',
    d.rule || '',
    d.incorrect || '',
    d.correction || '',
  ].join('::');
}

function isDismissedDiagnostic(d) {
  return dismissedDiagnosticKeys.has(diagnosticKey(d));
}

function getVisibleDiagnosticsWithIndex() {
  return diagnostics
    .map((d, index) => ({ d, index }))
    .filter(({ d }) =>
      !hiddenCategories.has(d.category_code) && !isDismissedDiagnostic(d)
    );
}

function getActiveVisiblePosition() {
  if (activeCardIndex < 0) return -1;
  return getVisibleDiagnosticsWithIndex().findIndex(({ index }) => index === activeCardIndex);
}

function isHardDiagnostic(d) {
  return !isHeuristicDiagnostic(d);
}

function runCheck() {
  hideMobileDiagOverlay();
  const text = editorInput.value;
  runtimeErrorMessage = null;

  if (!text.trim()) {
    diagnostics = [];
    dismissedDiagnosticKeys.clear();
    renderBackdrop(text);
    renderDiagnostics();
    renderFilters();
    renderGrammarCoverage();
    renderReviewToolbar();
    renderPreviewPanel();
    return;
  }

  try {
    diagnostics = checkText(text, { grammar: isGrammarEnabled() });
  } catch (err) {
    console.error('checkText failed', err);
    runtimeErrorMessage = 'जाँच प्रक्रिया असफल भयो। कृपया पृष्ठ रिफ्रेस गरेर फेरि प्रयास गर्नुहोस्।';
    diagnostics = [];
  }

  dismissedDiagnosticKeys.clear();
  activeCardIndex = -1;
  renderBackdrop(text);
  renderDiagnostics();
  renderFilters();
  renderGrammarCoverage();
  renderReviewToolbar();
  renderPreviewPanel();
}

function renderReviewToolbar() {
  const visible = getVisibleDiagnosticsWithIndex();
  const hardVisible = visible.filter(({ d }) => isHardDiagnostic(d));
  const activePos = getActiveVisiblePosition();

  if (reviewProgress) {
    reviewProgress.textContent = visible.length > 0
      ? `${activePos >= 0 ? activePos + 1 : 0} / ${visible.length}`
      : '0 / 0';
  }
  if (reviewPrevBtn) reviewPrevBtn.disabled = visible.length <= 1;
  if (reviewNextBtn) reviewNextBtn.disabled = visible.length <= 1;
  if (applyHardBtn) applyHardBtn.disabled = hardVisible.length === 0;
  if (copyCorrectedBtn) copyCorrectedBtn.disabled = visible.length === 0;
  if (togglePreviewBtn) togglePreviewBtn.disabled = visible.length === 0;
}

function togglePreviewPanel() {
  previewOpen = !previewOpen;
  renderPreviewPanel();
}

function renderPreviewPanel() {
  if (!previewPanel) return;
  const visible = getVisibleDiagnosticsWithIndex().map(({ d }) => d);
  const corrected = buildCorrectedText({ hardOnly: false });
  const original = editorInput.value;

  if (visible.length === 0 || !previewOpen) {
    previewPanel.hidden = true;
    previewPanel.innerHTML = '';
    if (togglePreviewBtn) {
      togglePreviewBtn.textContent = 'पूर्वावलोकन';
    }
    return;
  }

  const changedCount = visible.filter((d) => d.incorrect !== d.correction).length;
  previewPanel.hidden = false;
  previewPanel.innerHTML = `
    <div class="preview-head">
      <div>
        <div class="preview-title">सच्याइएको पाठको पूर्वावलोकन</div>
        <div class="preview-note">${changedCount} वटा लागू सुधार, अहिले देखिएका नियमहरूका आधारमा</div>
      </div>
      <button class="btn btn-sm" id="preview-close-btn">बन्द गर्नुहोस्</button>
    </div>
    <div class="preview-grid">
      <div class="preview-block">
        <div class="preview-label">अहिलेको पाठ</div>
        <pre class="preview-text">${escapeHtml(original)}</pre>
      </div>
      <div class="preview-block">
        <div class="preview-label">सच्याइएको पाठ</div>
        <pre class="preview-text preview-text-corrected">${escapeHtml(corrected)}</pre>
      </div>
    </div>
  `;
  previewPanel.querySelector('#preview-close-btn')
    ?.addEventListener('click', () => {
      previewOpen = false;
      renderPreviewPanel();
    });

  if (togglePreviewBtn) {
    togglePreviewBtn.textContent = 'पूर्वावलोकन लुकाउनुहोस्';
  }
}

function goToDiagnosticByVisiblePosition(position) {
  const visible = getVisibleDiagnosticsWithIndex();
  if (visible.length === 0) return;
  const normalized = ((position % visible.length) + visible.length) % visible.length;
  const targetIndex = visible[normalized].index;
  setActiveCard(targetIndex);
  const card = diagnosticsList.querySelector(`[data-index="${targetIndex}"]`);
  if (card) card.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
}

function goToPreviousDiagnostic() {
  const visible = getVisibleDiagnosticsWithIndex();
  if (visible.length === 0) return;
  const activePos = getActiveVisiblePosition();
  goToDiagnosticByVisiblePosition(activePos <= 0 ? visible.length - 1 : activePos - 1);
}

function goToNextDiagnostic() {
  const visible = getVisibleDiagnosticsWithIndex();
  if (visible.length === 0) return;
  const activePos = getActiveVisiblePosition();
  goToDiagnosticByVisiblePosition(activePos < 0 ? 0 : activePos + 1);
}

function buildCorrectedText({ hardOnly = false } = {}) {
  const applicable = getVisibleDiagnosticsWithIndex()
    .map(({ d }) => d)
    .filter((d) => !hardOnly || isHardDiagnostic(d))
    .sort((a, b) => b.charStart - a.charStart);

  let text = editorInput.value;
  for (const d of applicable) {
    text = text.slice(0, d.charStart) + d.correction + text.slice(d.charEnd);
  }
  return text;
}

function applyHardErrors() {
  editorInput.value = buildCorrectedText({ hardOnly: true });
  runCheck();
}

async function copyCorrectedText() {
  const corrected = buildCorrectedText({ hardOnly: false });
  try {
    await navigator.clipboard.writeText(corrected);
    if (copyCorrectedBtn) {
      const oldText = copyCorrectedBtn.textContent;
      copyCorrectedBtn.textContent = 'कपी भयो';
      setTimeout(() => {
        copyCorrectedBtn.textContent = oldText;
      }, 1200);
    }
  } catch (_err) {
    const probe = document.createElement('textarea');
    probe.value = corrected;
    document.body.appendChild(probe);
    probe.select();
    document.execCommand('copy');
    probe.remove();
  }
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
    (d) => !hiddenCategories.has(d.category_code) && !isDismissedDiagnostic(d)
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
      const isHeuristic = isHeuristicDiagnostic(d);
      const label = primaryCategoryLabel(d);
      const heuristicClass = isHeuristic ? " heuristic" : "";
      const kindLabel = diagnosticStateLabel(d);
      const kindClass = diagnosticKindClass(d);
      const guidance = diagnosticGuidance(d);
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
      const dismissButton = `<button class="btn btn-sm diag-dismiss" data-index="${i}">\u0905\u0939\u093F\u0932\u0947 \u091B\u094B\u0921\u094D\u0928\u0941\u0939\u094B\u0938\u094D</button>`;
      const confidence = Number.isFinite(d.confidence) ? Math.round(d.confidence * 100) : 0;
      return `
      <div class="diag-card${hidden}${active}${heuristicClass}" data-index="${i}" data-category="${code}">
        <div class="diag-meta">
          <span class="diag-badge" data-category="${code}">${escapeHtml(label)}</span>
          ${kindLabel ? `<span class="diag-kind-chip diag-kind-${kindClass}">${escapeHtml(kindLabel)}</span>` : ""}
          <span class="diag-confidence">${confidence}%</span>
        </div>
        ${correctionRow}
        <div class="diag-guidance">${escapeHtml(guidance)}</div>
        <div class="diag-explanation">${escapeHtml(d.explanation)}</div>
        <div class="diag-rule">${wrapRuleTooltip(d.rule, d.category_code, {
          incorrect: d.incorrect,
          correction: d.correction,
          explanation: d.explanation,
        })}</div>
        ${renderAlternateReasons(d)}
        <div class="diag-actions">
          ${dismissButton}
          ${fixButton}
        </div>
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

  diagnosticsList.querySelectorAll('.diag-dismiss').forEach((btn) => {
    btn.addEventListener('click', (e) => {
      e.stopPropagation();
      dismissOne(parseInt(btn.dataset.index));
    });
  });

  renderReviewToolbar();
  renderPreviewPanel();
}

/**
 * Render category filter pills.
 */
function renderFilters() {
  const counts = {};
  for (const d of diagnostics) {
    if (isDismissedDiagnostic(d)) continue;
    counts[d.category_code] = (counts[d.category_code] || 0) + 1;
  }

  const presentExtras = Object.keys(counts).filter(
    (code) => !FILTER_CATEGORY_ORDER.includes(code)
  );
  const categories = [...FILTER_CATEGORY_ORDER, ...presentExtras].sort((a, b) => {
    const aIndex = FILTER_CATEGORY_ORDER.indexOf(a);
    const bIndex = FILTER_CATEGORY_ORDER.indexOf(b);
    const aRank = aIndex === -1 ? Number.MAX_SAFE_INTEGER : aIndex;
    const bRank = bIndex === -1 ? Number.MAX_SAFE_INTEGER : bIndex;
    return aRank - bRank || a.localeCompare(b);
  });
  if (categories.length === 0) {
    categoryFilters.innerHTML = '';
    return;
  }

  categoryFilters.innerHTML = categories
    .map((code) => {
      const count = counts[code] || 0;
      const empty = count === 0 ? ' empty' : '';
      const inactive = hiddenCategories.has(code) ? ' inactive' : '';
      const color = CATEGORY_COLORS[code] || 'var(--cat-default)';
      const label = CATEGORY_LABELS[code] || code;
      return `<button class="category-pill${inactive}${empty}" data-category="${escapeHtml(code)}" style="border-color: ${color}; color: ${color};" ${count === 0 ? 'disabled' : ''}>
        ${escapeHtml(label)}
        <span class="pill-count">${count}</span>
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
      renderReviewToolbar();
      renderPreviewPanel();
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
  const visible = getVisibleDiagnosticsWithIndex();
  const visiblePos = visible.findIndex(({ index }) => index === idx);
  const hasChange = d.incorrect !== d.correction;
  const label = primaryCategoryLabel(d);
  const code = escapeHtml(d.category_code);
  const kindLabel = diagnosticStateLabel(d);
  const kindClass = diagnosticKindClass(d);
  const guidance = diagnosticGuidance(d);
  mobileDiagOverlay.innerHTML = `
    <div class="diag-meta">
      <span class="diag-badge" data-category="${code}">${escapeHtml(label)}</span>
      ${kindLabel ? `<span class="diag-kind-chip diag-kind-${kindClass}">${escapeHtml(kindLabel)}</span>` : ""}
      <span class="mobile-diag-progress">${visiblePos >= 0 ? `${visiblePos + 1} / ${visible.length}` : ''}</span>
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
    <div class="diag-guidance">${escapeHtml(guidance)}</div>
    <div class="diag-explanation">${escapeHtml(d.explanation)}</div>
    <div class="diag-rule">${wrapRuleTooltip(d.rule, d.category_code, {
      incorrect: d.incorrect,
      correction: d.correction,
      explanation: d.explanation,
    })}</div>
    ${renderAlternateReasons(d)}
    <div class="mobile-diag-actions">
      <button class="btn btn-sm" id="mobile-prev-btn" ${visible.length <= 1 ? 'disabled' : ''}>&larr; अघिल्लो</button>
      <button class="btn btn-sm" id="mobile-next-btn" ${visible.length <= 1 ? 'disabled' : ''}>अर्को &rarr;</button>
      <button class="btn btn-sm" id="mobile-skip-btn">अहिले छोड्नुहोस्</button>
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

  mobileDiagOverlay.querySelector('#mobile-prev-btn')
    ?.addEventListener('click', (e) => {
      e.stopPropagation();
      if (visible.length === 0) return;
      const nextPos = visiblePos <= 0 ? visible.length - 1 : visiblePos - 1;
      const target = visible[nextPos];
      if (target) showMobileDiagOverlay(target.d, target.index);
    });

  mobileDiagOverlay.querySelector('#mobile-next-btn')
    ?.addEventListener('click', (e) => {
      e.stopPropagation();
      if (visible.length === 0) return;
      const nextPos = visiblePos < 0 || visiblePos >= visible.length - 1 ? 0 : visiblePos + 1;
      const target = visible[nextPos];
      if (target) showMobileDiagOverlay(target.d, target.index);
    });

  mobileDiagOverlay.querySelector('#mobile-skip-btn')
    ?.addEventListener('click', (e) => {
      e.stopPropagation();
      dismissOne(idx);
      const refreshed = getVisibleDiagnosticsWithIndex();
      if (refreshed.length === 0) {
        hideMobileDiagOverlay();
        return;
      }
      const target = refreshed[Math.min(visiblePos, refreshed.length - 1)];
      if (target) showMobileDiagOverlay(target.d, target.index);
    });

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

function renderAlternateReasons(d) {
  if (!d.alternate_reasons || d.alternate_reasons.length === 0) {
    return "";
  }

  const items = d.alternate_reasons.map((alt) => {
    const altLabel = CATEGORY_LABELS[alt.category_code] || alt.category;
    const altCorrection = alt.correction && alt.correction !== d.correction
      ? `<span class="diag-alt-correction">${escapeHtml(alt.correction)}</span>`
      : "";
    return `
      <div class="diag-alt-item">
        <div class="diag-alt-meta">
          <span class="diag-alt-category">${escapeHtml(altLabel)}</span>
          <span class="diag-alt-rule">${wrapRuleTooltip(alt.rule, alt.category_code, {
            incorrect: d.incorrect,
            correction: alt.correction || d.correction,
            explanation: alt.explanation,
          })}</span>
          ${altCorrection}
        </div>
        <div class="diag-alt-text">${escapeHtml(alt.explanation)}</div>
      </div>`;
  }).join('');

  return `
    <div class="diag-alternates">
      <div class="diag-alternates-title">अन्य लागू नियमहरू</div>
      ${items}
    </div>`;
}

/**
 * Handle click in editor — show inspector for clicked word, or highlight diagnostic.
 */
function onEditorClick() {
  const pos = editorInput.selectionStart;
  const text = editorInput.value;

  // Check if click is on a diagnostic
  const idx = diagnostics.findIndex(
    (d) =>
      pos >= d.charStart &&
      pos < d.charEnd &&
      !hiddenCategories.has(d.category_code) &&
      !isDismissedDiagnostic(d)
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

function dismissOne(index) {
  const d = diagnostics[index];
  dismissedDiagnosticKeys.add(diagnosticKey(d));
  if (activeCardIndex === index) {
    activeCardIndex = -1;
  }
  hideMobileDiagOverlay();
  renderBackdrop(editorInput.value);
  renderDiagnostics();
  renderFilters();
  renderReviewToolbar();
  renderPreviewPanel();
}

/**
 * Fix all visible diagnostics, applying in reverse offset order.
 */
function fixAll() {
  editorInput.value = buildCorrectedText({ hardOnly: false });
  runCheck();
}
