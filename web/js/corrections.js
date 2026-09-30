/** Optional suggestions require an individual choice before applying them. */
export function canApplyDiagnostic(d) {
  return Boolean(d) && d.incorrect !== d.correction && d.rule_code !== 'samasa-heuristic';
}

export function canBulkApplyDiagnostic(d, { punctuationStrict = false } = {}) {
  return canApplyDiagnostic(d) && d.kind === 'Error' && d.confidence >= 0.8
    && (d.category_code !== 'Punctuation' || punctuationStrict);
}

/** Offsets refer to the checked snapshot, in JavaScript UTF-16 code units. */
export function applyCorrections(text, checkedText, diagnostics) {
  if (text !== checkedText) return text;
  const ordered = [...diagnostics].sort((a, b) => b.charStart - a.charStart);
  let rightBoundary = text.length;
  for (const d of ordered) {
    if (!canApplyDiagnostic(d) || typeof d.correction !== 'string'
        || !Number.isInteger(d.charStart) || !Number.isInteger(d.charEnd)
        || d.charStart < 0 || d.charEnd <= d.charStart || d.charEnd > rightBoundary
        || text.slice(d.charStart, d.charEnd) !== d.incorrect) return text;
    rightBoundary = d.charStart;
  }
  return ordered.reduce((result, d) =>
    result.slice(0, d.charStart) + d.correction + result.slice(d.charEnd), text);
}
