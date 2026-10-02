import { escapeHtml } from './utils.js';
import { canApplyDiagnostic } from './corrections.js';

/** Reviewed variants are a policy choice, independent of confidence. */
export function diagnosticDisplayState(diagnostic, { punctuationStrict = true } = {}) {
  if (diagnostic.rule_code === 'samasa-heuristic') return 'info';
  if (diagnostic.kind === 'Variant') return 'variant';
  if ((!punctuationStrict && diagnostic.category_code === 'Punctuation')
      || diagnostic.kind !== 'Error' || !(diagnostic.confidence >= 0.8)) return 'suggestion';
  return 'error';
}

export function diagnosticCountLabel(diagnostics, options) {
  const counts = { error: 0, variant: 0, suggestion: 0, info: 0 };
  for (const diagnostic of diagnostics) counts[diagnosticDisplayState(diagnostic, options)]++;
  const parts = [`${counts.error} त्रुटि`];
  if (counts.variant) parts.push(`${counts.variant} वैकल्पिक रूप`);
  if (counts.suggestion) parts.push(`${counts.suggestion} सुझाव`);
  if (counts.info) parts.push(`${counts.info} जानकारी`);
  return parts.join(', ');
}

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
    if (diagnostic.kind === 'Variant') {
      return `<div class="diag-correction">
        <span class="diag-source">${escapeHtml(diagnostic.incorrect)}</span>
        <span class="diag-arrow">→</span>
        <span class="diag-source">${escapeHtml(diagnostic.correction)}</span>
      </div>`;
    }
    return `<div class="diag-correction">
      <span class="diag-incorrect">${escapeHtml(diagnostic.incorrect)}</span>
      <span class="diag-arrow">→</span>
      <span class="diag-correct">${escapeHtml(diagnostic.correction)}</span>
    </div>`;
  }
  return `<div class="diag-correction"><span class="diag-incorrect">${escapeHtml(diagnostic.incorrect)}</span></div>`;
}
