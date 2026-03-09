/**
 * Rules Reference tab — renders Academy rule sections with examples.
 */
import { RULES_SECTIONS } from './rules-data.js';
import { escapeHtml, CATEGORY_COLORS, CATEGORY_LABELS } from './utils.js';

const container = document.getElementById('reference-content');

/**
 * Initialize the reference tab by rendering all rule sections.
 */
export function initReference() {
  if (!container) return;
  container.innerHTML = `
    <div class="reference-context-banner" id="reference-context-banner" hidden></div>
    <div class="reference-tools">
      <input
        type="search"
        id="reference-search"
        class="reference-search"
        placeholder="नियम, उदाहरण, वा श्रेणी खोज्नुहोस्…"
        aria-label="Search rules reference"
      />
      <div class="reference-jump" id="reference-jump">
        ${renderJumpChips()}
      </div>
    </div>
    <div class="reference-sections" id="reference-sections">
      ${renderSections()}
    </div>
  `;

  container.querySelector('#reference-search')
    ?.addEventListener('input', onSearchInput);

  container.querySelectorAll('.reference-jump-chip')
    .forEach((chip) => {
      chip.addEventListener('click', () => {
        const categoryCode = chip.dataset.category;
        if (categoryCode) highlightCard(categoryCode);
      });
    });
}

export function setReferenceContext(context) {
  const banner = container?.querySelector('#reference-context-banner');
  if (!banner) return;

  if (!context || (!context.incorrect && !context.word)) {
    banner.hidden = true;
    banner.innerHTML = '';
    return;
  }

  const shownWord = context.incorrect || context.word;
  const correction = context.correction || '';
  const explanation = context.explanation || '';
  const rule = context.rule || '';

  banner.innerHTML = `
    <div class="reference-context-kicker">यो शब्द किन अशुद्ध/सुधारयोग्य देखियो?</div>
    <div class="reference-context-main">
      <span class="reference-context-wrong">${escapeHtml(shownWord)}</span>
      ${correction ? `<span class="reference-context-arrow">→</span><span class="reference-context-right">${escapeHtml(correction)}</span>` : ''}
    </div>
    ${rule ? `<div class="reference-context-rule">${escapeHtml(rule)}</div>` : ''}
    ${explanation ? `<div class="reference-context-text">${escapeHtml(explanation)}</div>` : ''}
  `;
  banner.hidden = false;
}

/**
 * Scroll to and briefly highlight the ref-card or subsection for the given categoryCode.
 */
export function highlightCard(categoryCode, targetId = null) {
  if (!categoryCode) return;
  const target = targetId
    ? document.getElementById(`ref-${categoryCode}-${targetId}`)
    : null;
  const card = target || document.getElementById(`ref-${categoryCode}`);
  if (!card) return;

  card.scrollIntoView({ behavior: 'smooth', block: 'center' });
  card.classList.add('ref-card-highlight');
  setTimeout(() => card.classList.remove('ref-card-highlight'), 1500);
}

function renderSections() {
  return RULES_SECTIONS.map((section) => {
    const color = CATEGORY_COLORS[section.categoryCode] || 'var(--cat-default)';

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
      (section.referenceTargets && section.referenceTargets.length > 0)
        ? `<div class="ref-targets">
            ${section.referenceTargets.map((t) => `
              <details class="ref-target" id="ref-${escapeHtml(section.categoryCode)}-${escapeHtml(t.id)}">
                <summary class="ref-target-summary">
                  <span class="ref-target-kicker">कहिले लागू हुन्छ?</span>
                  <span class="ref-target-title">${escapeHtml(t.label)}</span>
                </summary>
                <div class="ref-target-body">
                  ${t.summary ? `<p class="ref-target-text">${escapeHtml(t.summary)}</p>` : ''}
                  ${renderTargetExamples(t.examples)}
                </div>
              </details>
            `).join('')}
          </div>`
        : '';

    const label = CATEGORY_LABELS[section.categoryCode] || section.categoryCode;

    return `
      <article class="ref-card" id="ref-${escapeHtml(section.categoryCode)}" data-category="${escapeHtml(section.categoryCode)}" style="--ref-accent: ${color}">
        <div class="ref-card-header">
          <div class="ref-card-heading">
            <span class="ref-badge" style="background: ${color};">${escapeHtml(label)}</span>
            <h3 class="ref-title">${escapeHtml(section.title)}</h3>
          </div>
          <div class="ref-card-tools">
            <button class="btn btn-sm ref-jump-btn" data-category="${escapeHtml(section.categoryCode)}">यो खण्डमा जानुहोस्</button>
          </div>
        </div>
        <p class="ref-summary">${escapeHtml(section.summary)}</p>
        ${targetsHtml}
        ${examplesHtml}
        ${subRulesHtml}
      </article>`;
  }).join('');
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
  const q = (e.target.value || '').trim().toLowerCase();
  const sectionEls = container.querySelectorAll('.ref-card');

  sectionEls.forEach((card) => {
    const text = card.textContent.toLowerCase();
    card.hidden = q ? !text.includes(q) : false;
  });
}

document.addEventListener('click', (e) => {
  const btn = e.target.closest('.ref-jump-btn');
  if (!btn) return;
  const categoryCode = btn.dataset.category;
  if (categoryCode) {
    highlightCard(categoryCode);
  }
});
