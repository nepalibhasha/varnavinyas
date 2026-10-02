import test from 'node:test';
import assert from 'node:assert/strict';
import { renderDiagnosticComposition, diagnosticDisplayState, diagnosticCountLabel } from '../js/diagnostic-presentation.js';

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

test('reviewed spellings are optional variants at any confidence, without error strike-through', () => {
  for (const incorrect of ['संघीय', 'कांग्रेस', 'संकेत']) {
    for (const confidence of [0.72, 1]) {
      const diagnostic = { incorrect, correction: 'कडा रूप', kind: 'Variant', confidence };
      assert.equal(diagnosticDisplayState(diagnostic), 'variant');
      assert.equal(diagnosticCountLabel([diagnostic]), '0 त्रुटि, 1 वैकल्पिक रूप');
      const html = renderDiagnosticComposition(diagnostic);
      assert.match(html, /diag-source/);
      assert.doesNotMatch(html, /diag-incorrect|diag-correct"/);
    }
  }
});

test('counts distinguish strict errors, variants, uncertain suggestions, and information', () => {
  const error = { incorrect: 'संकेत', correction: 'सङ्केत', kind: 'Error', confidence: 1 };
  const diagnostics = [error, { ...error, kind: 'Variant' }, { ...error, kind: 'Ambiguous' },
    { ...error, kind: 'Variant', rule_code: 'samasa-heuristic' }];
  assert.equal(diagnosticCountLabel(diagnostics), '1 त्रुटि, 1 वैकल्पिक रूप, 1 सुझाव, 1 जानकारी');
  assert.equal(diagnosticCountLabel([{ ...error, category_code: 'Punctuation' }],
    { punctuationStrict: false }), '0 त्रुटि, 1 सुझाव');
  assert.equal(diagnosticDisplayState({ ...error, confidence: undefined }), 'suggestion');
});
