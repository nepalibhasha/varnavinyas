import test from 'node:test';
import assert from 'node:assert/strict';
import { renderDiagnosticComposition } from '../js/diagnostic-presentation.js';

globalThis.document = {
  createElement: () => ({ textContent: '', get innerHTML() {
    return String(this.textContent).replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;');
  } }),
};

test('compound information is labeled composition without a correction arrow', () => {
  const html = renderDiagnosticComposition({ rule_code: 'samasa-heuristic',
    incorrect: 'सूर्योदयमा', correction: 'सूर्य + उदय + मा', confidence: 0.82 });
  assert.match(html, /बनोट:/);
  assert.match(html, /सूर्य \+ उदय \+ मा/);
  assert.doesNotMatch(html, /diag-arrow|→|diag-correct"|82%/);
});

test('actual corrections retain their arrow and every composition field is escaped', () => {
  const html = renderDiagnosticComposition({ incorrect: 'हामि', correction: 'हामी' });
  assert.match(html, /diag-arrow/);
  assert.match(html, /diag-correct"/);
  for (const rule_code of ['samasa-heuristic', '3(क)(ऊ)-7']) {
    const unsafe = renderDiagnosticComposition({ rule_code,
      incorrect: '<img src=x onerror=alert(1)>', correction: '<script>x</script>' });
    assert.doesNotMatch(unsafe, /<img|<script/);
    assert.match(unsafe, /&lt;/);
  }
});
