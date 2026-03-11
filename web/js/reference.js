/**
 * Rules Reference tab — renders Academy rule sections with examples.
 */
import { RULES_SECTIONS } from './rules-data.js';
import { escapeHtml, CATEGORY_COLORS, CATEGORY_LABELS } from './utils.js';

const container = document.getElementById('reference-content');

let currentContext = null;
let focusedCategoryCode = null;
let focusedTargetId = null;
let currentSearch = '';

/**
 * Initialize the reference tab by rendering all rule sections.
 */
export function initReference() {
  if (!container) return;
  renderReferenceView();
}

export function setReferenceContext(context) {
  currentContext = context && (context.incorrect || context.word) ? context : null;
  focusedCategoryCode = currentContext?.categoryCode || null;
  focusedTargetId = currentContext?.targetId || null;
  currentSearch = '';
  renderReferenceView();
}

/**
 * Scroll to and briefly highlight the ref-card or subsection for the given categoryCode.
 */
export function highlightCard(categoryCode, targetId = null) {
  if (!categoryCode) return;

  if (focusedCategoryCode !== categoryCode || focusedTargetId !== targetId) {
    focusedCategoryCode = categoryCode;
    focusedTargetId = targetId;
    renderReferenceView();
  }

  const target = targetId
    ? document.getElementById(`ref-${categoryCode}-${targetId}`)
    : null;
  const card = target || document.getElementById(`ref-${categoryCode}`);
  if (!card) return;

  if (target && typeof target.open === 'boolean') {
    target.open = true;
  }

  card.scrollIntoView({ behavior: 'smooth', block: 'start' });
  card.classList.add('ref-card-highlight');
  setTimeout(() => card.classList.remove('ref-card-highlight'), 1500);
}

function renderReferenceView() {
  if (!container) return;

  const sections = getVisibleSections();
  const focusedLabel = focusedCategoryCode
    ? (CATEGORY_LABELS[focusedCategoryCode] || focusedCategoryCode)
    : '';

  container.innerHTML = `
    ${renderContextBanner()}
    <div class="reference-tools">
      ${focusedCategoryCode
        ? `<div class="reference-focus-bar">
            <div class="reference-focus-copy">
              <div class="reference-focus-kicker">सम्बन्धित नियम</div>
              <div class="reference-focus-title">${escapeHtml(focusedLabel)}${focusedTargetId ? ' · लागू उपनियम' : ''}</div>
            </div>
            <button class="btn btn-sm" id="reference-show-all-btn">सबै नियम हेर्नुहोस्</button>
          </div>`
        : `<input
            type="search"
            id="reference-search"
            class="reference-search"
            placeholder="नियम, उदाहरण, वा श्रेणी खोज्नुहोस्…"
            aria-label="Search rules reference"
            value="${escapeHtml(currentSearch)}"
          />
          <div class="reference-jump" id="reference-jump">
            ${renderJumpChips()}
          </div>`
      }
    </div>
    <div class="reference-sections" id="reference-sections">
      ${sections.length > 0 ? sections.map((section) => renderSection(section)).join('') : '<p class="diag-empty">मिल्दो नियम भेटिएन।</p>'}
    </div>
  `;

  bindReferenceEvents();
}

function bindReferenceEvents() {
  container.querySelector('#reference-search')
    ?.addEventListener('input', onSearchInput);

  container.querySelectorAll('.reference-jump-chip')
    .forEach((chip) => {
      chip.addEventListener('click', () => {
        const categoryCode = chip.dataset.category;
        if (categoryCode) highlightCard(categoryCode);
      });
    });

  container.querySelector('#reference-show-all-btn')
    ?.addEventListener('click', () => {
      focusedCategoryCode = null;
      focusedTargetId = null;
      renderReferenceView();
    });
}

function renderContextBanner() {
  if (!currentContext || (!currentContext.incorrect && !currentContext.word)) {
    return '<div class="reference-context-banner" id="reference-context-banner" hidden></div>';
  }

  const shownWord = currentContext.incorrect || currentContext.word;
  const correction = currentContext.correction || '';
  const explanation = currentContext.explanation || '';
  const rule = currentContext.rule || '';

  return `
    <div class="reference-context-banner" id="reference-context-banner">
      <div class="reference-context-kicker">यो शब्द किन अशुद्ध/सुधारयोग्य देखियो?</div>
      <div class="reference-context-main">
        <span class="reference-context-wrong">${escapeHtml(shownWord)}</span>
        ${correction ? `<span class="reference-context-arrow">→</span><span class="reference-context-right">${escapeHtml(correction)}</span>` : ''}
      </div>
      ${rule ? `<div class="reference-context-rule">${escapeHtml(rule)}</div>` : ''}
      ${explanation ? `<div class="reference-context-text">${escapeHtml(explanation)}</div>` : ''}
    </div>
  `;
}

function getVisibleSections() {
  let sections = RULES_SECTIONS;

  if (focusedCategoryCode) {
    sections = sections.filter((section) => section.categoryCode === focusedCategoryCode);
  }

  if (!focusedCategoryCode && currentSearch) {
    const q = currentSearch.toLowerCase();
    sections = sections.filter((section) => sectionText(section).includes(q));
  }

  return sections;
}

function sectionText(section) {
  const targets = (section.referenceTargets || [])
    .flatMap((t) => [t.label, t.summary || '', ...(t.examples || [])]);
  return [
    section.title,
    section.summary,
    ...(section.subRules || []),
    ...(section.examples || []).flatMap((ex) => [ex.wrong, ex.correct]),
    ...targets,
    CATEGORY_LABELS[section.categoryCode] || section.categoryCode,
  ].join(' ').toLowerCase();
}

function renderSection(section) {
  const color = CATEGORY_COLORS[section.categoryCode] || 'var(--cat-default)';
  const label = CATEGORY_LABELS[section.categoryCode] || section.categoryCode;

  const filteredTargets = focusedCategoryCode && focusedTargetId
    ? (section.referenceTargets || []).filter((t) => t.id === focusedTargetId)
    : (section.referenceTargets || []);

  const examplesHtml =
    section.examples.length > 0
      ? `<table class="ref-examples">
          <thead>
            <tr><th>अशुद्ध</th><th></th><th>शुद्ध</th></tr>
          </thead>
          <tbody>
            ${section.examples
              .map(
                (ex) => `<tr>
                <td class="ref-wrong">${escapeHtml(ex.wrong)}</td>
                <td class="ref-arrow">\u2192</td>
                <td class="ref-right">${escapeHtml(ex.correct)}</td>
              </tr>`
              )
              .join('')}
          </tbody>
        </table>`
      : '';

  const subRulesHtml =
    section.subRules.length > 0
      ? `<ul class="ref-subrules">
          ${section.subRules.map((r) => `<li>${escapeHtml(r)}</li>`).join('')}
        </ul>`
      : '';

  const targetsHtml =
    filteredTargets.length > 0
      ? `<div class="ref-targets-kicker">कहिले लागू हुन्छ?</div>
        <div class="ref-targets">
          ${filteredTargets.map((t) => `
            <details
              class="ref-target ref-target-status-${t.status || 'unknown'}"
              id="ref-${escapeHtml(section.categoryCode)}-${escapeHtml(t.id)}"
              ${focusedCategoryCode ? 'open' : ''}
            >
              <summary class="ref-target-summary">
                <span class="ref-target-title">${escapeHtml(t.label)}</span>
                <span class="ref-status-dot ref-status-${t.status || 'unknown'}"></span>
              </summary>
              <div class="ref-target-body">
                ${t.summary ? `<p class="ref-target-text">${escapeHtml(t.summary)}</p>` : ''}
                ${renderTargetExamples(t.examples)}
              </div>
            </details>
          `).join('')}
        </div>`
      : '';

  return `
    <article class="ref-card" id="ref-${escapeHtml(section.categoryCode)}" data-category="${escapeHtml(section.categoryCode)}" style="--ref-accent: ${color}">
      <div class="ref-card-header">
        <div class="ref-card-heading">
          <span class="ref-badge" style="background: ${color};">${escapeHtml(label)}</span>
          <h3 class="ref-title">${escapeHtml(section.title)}</h3>
        </div>
      </div>
      <p class="ref-summary">${escapeHtml(section.summary)}</p>
      ${subRulesHtml}
      ${targetsHtml}
      ${examplesHtml}
    </article>`;
}

function renderJumpChips() {
  return RULES_SECTIONS.map((section) => {
    const label = CATEGORY_LABELS[section.categoryCode] || section.categoryCode;
    return `
      <button class="reference-jump-chip" data-category="${escapeHtml(section.categoryCode)}">
        ${escapeHtml(label)}
      </button>
    `;
  }).join('');
}

function renderTargetExamples(examples = []) {
  if (!examples || examples.length === 0) return '';
  return `
    <div class="ref-target-examples-wrap">
      <div class="ref-target-examples-title">उदाहरण</div>
      <ul class="ref-target-examples">
      ${examples.map((ex) => `<li>${escapeHtml(ex)}</li>`).join('')}
      </ul>
    </div>
  `;
}

function onSearchInput(e) {
  currentSearch = (e.target.value || '').trim();
  renderReferenceView();
}

document.addEventListener('click', (e) => {
  const btn = e.target.closest('.ref-jump-btn');
  if (!btn) return;
  const categoryCode = btn.dataset.category;
  if (categoryCode) {
    highlightCard(categoryCode);
  }
});
