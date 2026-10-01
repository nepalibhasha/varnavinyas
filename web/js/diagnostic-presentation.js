import { escapeHtml } from './utils.js';
import { canApplyDiagnostic } from './corrections.js';

/** Structural information must not look like a replacement to apply. */
export function renderDiagnosticComposition(diagnostic) {
  if (diagnostic.rule_code === 'samasa-heuristic') {
    return `<div class="diag-correction diag-analysis-row">
      <span class="diag-source">${escapeHtml(diagnostic.incorrect)}</span>
      <div class="diag-analysis-details">
        <span class="diag-analysis-label">बनोट:</span>
        <span class="diag-analysis">${escapeHtml(diagnostic.correction)}</span>
      </div>
    </div>`;
  }
  if (canApplyDiagnostic(diagnostic)) {
    return `<div class="diag-correction">
      <span class="diag-incorrect">${escapeHtml(diagnostic.incorrect)}</span>
      <span class="diag-arrow">→</span>
      <span class="diag-correct">${escapeHtml(diagnostic.correction)}</span>
    </div>`;
  }
  return `<div class="diag-correction"><span class="diag-incorrect">${escapeHtml(diagnostic.incorrect)}</span></div>`;
}
