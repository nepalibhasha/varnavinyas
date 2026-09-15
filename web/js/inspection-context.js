import { ORIGIN_LABELS } from './utils.js';

/** Reconcile isolated word analysis with the editor's contextual checker result. */
export function applyTextContext(analysis, context) {
  if (!context) return analysis;
  const diagnostic = context.diagnostic;
  if (!context.available) {
    return { ...analysis, correction: null, rule_notes: [], alternate_rule_notes: [],
      statusLabel: 'जाँच उपलब्ध छैन', statusClass: 'uncertain' };
  }
  if (!diagnostic && context.overlap) {
    return { ...analysis, correction: null, rule_notes: [context.overlap], alternate_rule_notes: [],
      statusLabel: 'पदावलीको सन्दर्भ जाँच्नुहोस्', statusClass: 'uncertain' };
  }
  if (!diagnostic) {
    return { ...analysis, is_correct: true, correction: null,
      rule_notes: analysis?.is_correct ? analysis.rule_notes : [], alternate_rule_notes: [],
      statusLabel: 'यस सन्दर्भमा त्रुटि भेटिएन', statusClass: 'correct' };
  }
  const ambiguous = diagnostic.kind === 'Ambiguous';
  const variant = diagnostic.kind === 'Variant';
  return { ...analysis, is_correct: false,
    canShowDerivation: !ambiguous && !variant && !context.informational,
    correction: ambiguous || context.canApply === false ? null : diagnostic.correction,
    rule_notes: [diagnostic], alternate_rule_notes: diagnostic.alternate_reasons || [],
    statusLabel: context.informational ? 'जानकारी' : ambiguous ? 'सन्दर्भ जाँच्नुहोस्' : variant ? 'वैकल्पिक रूप' : 'अशुद्ध',
    statusClass: context.informational || ambiguous || variant ? 'uncertain' : 'incorrect' };
}

/** Only dictionary tags and reviewed overrides establish origin. */
export function originPresentation(analysis) {
  if (!analysis || analysis.origin_source === 'unknown' || analysis.origin === 'unknown'
      || !analysis.origin_source) {
    return { label: 'उत्पत्ति अज्ञात', cssClass: 'origin-unknown',
      explanation: 'यस शब्दको उत्पत्तिको प्रमाण उपलब्ध छैन।' };
  }
  const label = ORIGIN_LABELS[analysis.origin] || analysis.origin;
  if (analysis.origin_source === 'heuristic') {
    return { label: `${label} (अनुमानित)`, cssClass: `origin-${analysis.origin}`,
      explanation: 'अक्षरको ढाँचाबाट गरिएको अनुमान; शब्दकोशबाट पुष्टि भएको होइन।' };
  }
  return { label, cssClass: `origin-${analysis.origin}`,
    explanation: analysis.origin_source === 'kosha' ? 'शब्दकोशको उत्पत्ति सूचनामा आधारित।' : 'समीक्षा गरिएको उत्पत्ति सूचनामा आधारित।' };
}
