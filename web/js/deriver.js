/**
 * Deriver module — educational word derivation with step trace.
 */
import { deriveWord, analyzeWord, decomposeWord } from './wasm-bridge.js';
import { debounce, escapeHtml, ORIGIN_LABELS } from './utils.js';
import { getTooltipForRule, getCategoryForRule } from './rules-data.js';

/** Feature flag for word analysis in deriver. Set to true to enable. */
const FEATURE_ANALYSIS = true;

const inputEl = document.getElementById('deriver-input');
const resultEl = document.getElementById('deriver-result');
const emptyEl = document.getElementById('deriver-empty');
const summaryEl = document.getElementById('deriver-summary');
const stepsContainer = document.getElementById('deriver-steps-container');
const stepsTbody = document.getElementById('steps-tbody');

/**
 * Initialize the deriver module.
 */
export function initDeriver() {
  inputEl.addEventListener('input', debouncedDerive);
}

const debouncedDerive = debounce(() => runDerive(), 300);

function runDerive() {
  const word = inputEl.value.trim();
  if (!word) {
    resultEl.hidden = true;
    emptyEl.hidden = false;
    return;
  }

  let result;
  try {
    result = deriveWord(word);
  } catch {
    resultEl.hidden = true;
    emptyEl.hidden = false;
    return;
  }

  emptyEl.hidden = true;
  resultEl.hidden = false;

  // Summary
  const indicatorClass = result.is_correct ? 'correct' : 'incorrect';
  summaryEl.innerHTML = `
    <span class="indicator ${indicatorClass}"></span>
    <span class="word-input">${escapeHtml(result.input)}</span>
    <span class="word-arrow">→</span>
    <span class="word-output">${escapeHtml(result.output)}</span>
    <span style="margin-left: auto; font-size: 0.8rem; color: var(--color-text-secondary);">
      ${result.is_correct ? 'शुद्ध' : 'अशुद्ध'}
    </span>
  `;

  // Steps table
  if (result.steps && result.steps.length > 0) {
    stepsContainer.hidden = false;
    stepsTbody.innerHTML = result.steps
      .map(
        (s, i) => `
      <tr>
        <td>${i + 1}</td>
        <td class="rule-cell">${wrapRuleTooltip(s.rule)}</td>
        <td>${escapeHtml(s.description)}</td>
        <td>${escapeHtml(s.before)}</td>
        <td>${escapeHtml(s.after)}</td>
      </tr>`
      )
      .join('');
  } else {
    stepsContainer.hidden = true;
  }

  // Feature-flagged: word analysis with origin and rule notes
  if (FEATURE_ANALYSIS) {
    renderDeriverAnalysis(word);
  }
}

/**
 * Wrap a rule citation in a tooltip-enabled span.
 */
function wrapRuleTooltip(ruleText) {
  const cat = getCategoryForRule(ruleText);
  const tooltip = getTooltipForRule(ruleText);
  if (tooltip && cat) {
    return `<span class="rule-ref" data-tooltip="${escapeHtml(tooltip)}" data-category="${escapeHtml(cat)}">${escapeHtml(ruleText)}</span>`;
  }
  if (tooltip) {
    return `<span class="rule-ref" data-tooltip="${escapeHtml(tooltip)}">${escapeHtml(ruleText)}</span>`;
  }
  return escapeHtml(ruleText);
}

function renderDeriverAnalysis(word) {
  let panel = document.getElementById('deriver-analysis');
  if (!panel) {
    panel = document.createElement('div');
    panel.id = 'deriver-analysis';
    panel.className = 'deriver-analysis';
    resultEl.appendChild(panel);
  }

  try {
    const analysis = analyzeWord(word);
    const originLabel = ORIGIN_LABELS[analysis.origin] || analysis.origin;
    const originClass = `origin-${analysis.origin}`;

    let html = `
      <div class="analysis-header">
        <span class="origin-badge ${originClass}">${escapeHtml(originLabel)}</span>
      </div>`;

    // Morphology decomposition
    html += renderMorphology(word);

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
 * Render morphology decomposition (prefixes + root + suffixes).
 */
function renderMorphology(word) {
  try {
    const m = decomposeWord(word);
    const hasPrefixes = m.prefixes && m.prefixes.length > 0;
    const hasSuffixes = m.suffixes && m.suffixes.length > 0;

    // Only show if there's actual decomposition
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
      parts += `
        <span class="morpheme-sep">+</span>
        <span class="morpheme morpheme-suffix">
          ${escapeHtml(s)}
          <span class="morpheme-label">\u092A\u094D\u0930\u0924\u094D\u092F\u092F</span>
        </span>`;
    }

    return `<div class="morphology-display">${parts}</div>`;
  } catch {
    return '';
  }
}
