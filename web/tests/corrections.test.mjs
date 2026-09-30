import test from 'node:test';
import assert from 'node:assert/strict';
import { applyCorrections, canApplyDiagnostic, canBulkApplyDiagnostic } from '../js/corrections.js';

const text = '🙂 मीलेको संघीय।';
const error = { incorrect: 'मीलेको', correction: 'मिलेको', kind: 'Error', confidence: 1,
  category_code: 'HrasvaDirgha', charStart: 3, charEnd: 9 };
const variant = { incorrect: 'संघीय', correction: 'सङ्घीय', kind: 'Variant', confidence: 1,
  category_code: 'Chandrabindu', charStart: 10, charEnd: 15 };

test('common-editorial bulk actions leave optional forms unchanged, strict mode corrects them', () => {
  assert.equal(applyCorrections(text, text, [error, variant].filter(canBulkApplyDiagnostic)),
    '🙂 मिलेको संघीय।');
  assert.equal(applyCorrections(text, text,
    [error, { ...variant, kind: 'Error' }].filter(canBulkApplyDiagnostic)), '🙂 मिलेको सङ्घीय।');
  assert.equal(canApplyDiagnostic(variant), true);
  assert.equal(applyCorrections(text, text, [variant]), '🙂 मीलेको सङ्घीय।');
});

test('bulk actions require clear errors and respect punctuation style policy', () => {
  for (const d of [{ ...error, kind: 'Ambiguous' }, { ...error, confidence: 0.79 },
    variant, { ...error, rule_code: 'samasa-heuristic' }, { ...error, correction: error.incorrect }]) {
    assert.equal(canBulkApplyDiagnostic(d), false);
  }
  const punctuation = { ...error, category_code: 'Punctuation' };
  assert.equal(canBulkApplyDiagnostic(punctuation), false);
  assert.equal(canBulkApplyDiagnostic(punctuation, { punctuationStrict: true }), true);
});

test('stale offsets cannot modify newly typed text, including a same-span edit elsewhere', () => {
  for (const edited of ['आज ' + text, text + ' नयाँ', text.replace('संघीय', 'समाचार')]) {
    assert.equal(applyCorrections(edited, text, [error]), edited);
    assert.equal(applyCorrections(edited, text, [variant]), edited);
  }
});

test('invalid or overlapping corrections fail without partially applying others', () => {
  for (const invalid of [{ ...variant, charStart: -1 }, { ...variant, charEnd: 99 },
    { ...variant, charStart: 10.5 }, { ...variant, incorrect: 'पुरानो' },
    { ...variant, correction: null }, error]) {
    assert.equal(applyCorrections(text, text, [error, invalid]), text);
  }
  assert.equal(applyCorrections(text, text, []), text);
});
