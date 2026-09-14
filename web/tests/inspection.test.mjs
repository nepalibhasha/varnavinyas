import test from 'node:test';
import assert from 'node:assert/strict';
import { applyTextContext } from '../js/inspection-context.js';
import { wrapRuleTooltip } from '../js/rules-data.js';

const raw = { word: 'आवाजमा', is_correct: false, correction: 'आबाजमा',
  rule_notes: [{ rule: 'raw rule', explanation: 'raw explanation' }], alternate_rule_notes: [] };

test('contextual acceptance removes isolated corrections and corrective notes', () => {
  const shown = applyTextContext(raw, { available: true, diagnostic: null });
  assert.equal(shown.correction, null);
  assert.deepEqual(shown.rule_notes, []);
  assert.equal(shown.statusLabel, 'यस सन्दर्भमा त्रुटि भेटिएन');
  assert.equal(raw.correction, 'आबाजमा');
});

test('accepted lexical notes survive contextual acceptance', () => {
  const accepted = { ...raw, is_correct: true, correction: null };
  assert.deepEqual(applyTextContext(accepted, { available: true }).rule_notes, accepted.rule_notes);
});

test('contextual winner and its distinct reasons replace isolated analysis', () => {
  const diagnostic = { correction: 'सम्धिनी', kind: 'Error', rule_code: '3(ई)',
    alternate_reasons: [{ rule_code: '3(क)(ऊ)-3' }] };
  const shown = applyTextContext(raw, { available: true, diagnostic });
  assert.equal(shown.correction, 'सम्धिनी');
  assert.deepEqual(shown.rule_notes, [diagnostic]);
  assert.deepEqual(shown.alternate_rule_notes, diagnostic.alternate_reasons);
});

test('ambiguous, variant, overlapping phrase, and unavailable results are not called errors', () => {
  for (const kind of ['Ambiguous', 'Variant']) {
    const shown = applyTextContext(raw, { available: true, diagnostic: { kind, correction: 'x' } });
    assert.equal(shown.statusClass, 'uncertain');
    assert.equal(shown.canShowDerivation, false);
    assert.equal(shown.correction, kind === 'Variant' ? 'x' : null);
  }
  for (const context of [{ available: false }, { available: true, overlap: { correction: 'phrase' } }]) {
    const shown = applyTextContext(raw, context);
    assert.equal(shown.statusClass, 'uncertain');
    assert.equal(shown.correction, null);
  }
});

test('informational diagnostics never become inspector fix buttons', () => {
  const shown = applyTextContext(raw, { available: true, canApply: false, informational: true,
    diagnostic: { kind: 'Variant', correction: 'split + parts' } });
  assert.equal(shown.correction, null);
  assert.equal(shown.statusLabel, 'जानकारी');
});

// Minimal DOM host for the reference renderers; real browser coverage accompanies this test.
const referenceHost = { innerHTML: '', querySelector: () => null, querySelectorAll: () => [] };
globalThis.document = {
  getElementById: (id) => id === 'reference-content' ? referenceHost : null,
  createElement: () => ({ textContent: '', get innerHTML() {
    return String(this.textContent).replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;');
  } }),
};
const { setReferenceContext } = await import('../js/reference.js');

test('rule links use the precise citation while retaining the readable label', () => {
  const html = wrapRuleTooltip('ह्रस्व/दीर्घ स्वर नियम', null, { ruleCode: '3(क)(ऊ)-3', word: 'भाउजू' });
  assert.match(html, /data-target="ka-uu"/);
  assert.match(html, />ह्रस्व\/दीर्घ स्वर नियम<\/span>/);
  assert.match(wrapRuleTooltip('पदयोग/पदवियोग नियम', 'ShuddhaTable', { ruleCode: '3(घ)' }), /data-target="padayog"/);
  assert.match(wrapRuleTooltip('शैक्षणिक व्याकरण ७(क) — तिर्यक् रूपको प्रयोग', 'ShuddhaTable', { ruleCode: 'PS-Saisanik-7(क)-तिर्यक्' }), /data-target="tiryak-ka"/);
});

test('accepted reference context has neither an error claim nor a self-correction arrow', () => {
  setReferenceContext({ word: 'भाउजू', correction: 'भाउजू', categoryCode: 'HrasvaDirgha' });
  assert.match(referenceHost.innerHTML, /यो शब्दको वर्णविन्यासबारे/);
  assert.doesNotMatch(referenceHost.innerHTML, /reference-context-arrow|reference-context-wrong/);
  setReferenceContext({ incorrect: 'भाउजु', correction: 'भाउजू', categoryCode: 'HrasvaDirgha' });
  assert.match(referenceHost.innerHTML, /यो सुधार किन सुझाइयो/);
  assert.match(referenceHost.innerHTML, /reference-context-arrow/);
});
