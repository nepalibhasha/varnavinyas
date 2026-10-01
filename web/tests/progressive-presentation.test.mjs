import assert from 'node:assert/strict';
import test from 'node:test';
import { renderProgressiveAnalysis } from '../js/progressive-presentation.js';

globalThis.document = {
  createElement: () => ({ textContent: '', get innerHTML() {
    return String(this.textContent).replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;');
  } }),
};

const analysis = { surface: 'खोजिरहेको', main_form: 'खोजि', main_lemma: 'खोज्नु',
  auxiliary_form: 'रहेको', auxiliary_lemma: 'रहनु', negative: false };

test('progressive construction shows surface forms and dictionary lemmas without a fix', () => {
  const html = renderProgressiveAnalysis(analysis);
  for (const text of ['खोजि', 'खोज्नु', 'रहेको', 'रहनु', 'मुख्य क्रिया', 'सहायक क्रिया']) {
    assert.ok(html.includes(text), text);
  }
  assert.ok(!html.includes('diag-arrow'));
  assert.ok(!html.includes('<button'));
  assert.ok(!html.includes('विभक्ति'));
  assert.equal(renderProgressiveAnalysis(null), '');
});

test('progressive presentation escapes backend text and labels a negative reading', () => {
  const html = renderProgressiveAnalysis({ ...analysis, main_form: '<img src=x>', negative: true });
  assert.ok(!html.includes('<img'));
  assert.ok(html.includes('&lt;img'));
  assert.ok(html.includes('निषेध'));
});
