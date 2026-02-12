/**
 * Rules Reference tab — renders Academy rule sections with examples.
 */
import { RULES_SECTIONS } from './rules-data.js';
import { escapeHtml, CATEGORY_COLORS } from './utils.js';

const container = document.getElementById('reference-content');

/**
 * Initialize the reference tab by rendering all rule sections.
 */
export function initReference() {
  if (!container) return;
  container.innerHTML = renderSections();
}

/**
 * Scroll to and briefly highlight the ref-card for the given categoryCode.
 */
export function highlightCard(categoryCode) {
  if (!categoryCode) return;
  const card = document.getElementById(`ref-${categoryCode}`);
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

    return `
      <div class="ref-card" id="ref-${escapeHtml(section.categoryCode)}">
        <div class="ref-card-header">
          <span class="ref-badge" style="background: ${color};">${escapeHtml(section.categoryCode)}</span>
          <span class="ref-code">${escapeHtml(section.code)}</span>
          <h3 class="ref-title">${escapeHtml(section.title)}</h3>
        </div>
        <p class="ref-summary">${escapeHtml(section.summary)}</p>
        ${examplesHtml}
        ${subRulesHtml}
      </div>`;
  }).join('');
}
