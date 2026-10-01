import { escapeHtml } from './utils.js';

/** Show surface construction and lemma evidence without suggesting a rewrite. */
export function renderProgressiveAnalysis(analysis) {
  if (!analysis) return '';
  return `<div class="inspector-section">
    <div class="inspector-section-head">
      <h3 class="inspector-section-title">क्रियाको बनोट</h3>
      <span class="inspector-section-label" lang="en">Progressive construction</span>
    </div>
    <div class="morphology-display">
      <span class="morpheme morpheme-root">${escapeHtml(analysis.main_form)}
        <span class="morpheme-label">मुख्य क्रिया · ${escapeHtml(analysis.main_lemma)}</span>
      </span>
      <span class="morpheme-sep">+</span>
      <span class="morpheme morpheme-suffix">${escapeHtml(analysis.auxiliary_form)}
        <span class="morpheme-label">सहायक क्रिया · ${escapeHtml(analysis.auxiliary_lemma)}</span>
      </span>
    </div>
    <p>मुख्य क्रियासँग रहेको/रहेकी/रहेका जोडिँदा चलिरहेको काम जनाउँछ।</p>
    ${analysis.negative ? '<p>निषेध जनाउने न मुख्य क्रियासँग जोडिएको छ।</p>' : ''}
  </div>`;
}
