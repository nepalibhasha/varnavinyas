/**
 * Deriver module — educational word derivation with step trace.
 */
import { deriveWord } from './wasm-bridge.js';
import { debounce, escapeHtml } from './utils.js';

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
        <td class="rule-cell">${escapeHtml(s.rule)}</td>
        <td>${escapeHtml(s.description)}</td>
        <td>${escapeHtml(s.before)}</td>
        <td>${escapeHtml(s.after)}</td>
      </tr>`
      )
      .join('');
  } else {
    stepsContainer.hidden = true;
  }
}
