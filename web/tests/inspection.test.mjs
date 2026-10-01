import test from 'node:test';
import assert from 'node:assert/strict';
import { applyTextContext, morphologySupportedByAffix, originPresentation } from '../js/inspection-context.js';
import { getReferenceTargetForRule, getRuleSummary, RULES_SECTIONS, wrapRuleTooltip } from '../js/rules-data.js';

const raw = { word: 'आवाजमा', is_correct: false, correction: 'आबाजमा',
  rule_notes: [{ rule: 'raw rule', explanation: 'raw explanation' }], alternate_rule_notes: [] };

test('joining reference explains reviewed relational roles and guarded hosts', () => {
  const target = RULES_SECTIONS.flatMap(s => s.referenceTargets || []).find(t => t.id === 'padayog');
  assert.match(target.summary, /सरह.*३\(घ\)-११/);
  assert.match(target.summary, /सम्म.*५\(अ\)\(ख\)/);
  assert.match(target.summary, /घरको सम्म र मानिसको सरह/);
  assert.match(target.summary, /जस्तो\/जस्तै\/जत्रो\/जसरी/);
  assert.ok(target.examples.includes('घर सम्म -> घरसम्म'));
  assert.ok(target.examples.includes('मानिस सरह -> मानिससरह'));
});

test('legacy morphology cannot override the supported stem, including lexical coincidences', () => {
  assert.equal(morphologySupportedByAffix({ root: 'फर्सी', stem: 'फर्सी' }, { root: 'फर्स', suffixes: ['ई'] }), false);
  assert.equal(morphologySupportedByAffix({ root: 'विद्यार्थी' }, { root: 'विद्यार्थ', suffixes: ['ई'] }), false);
  assert.equal(morphologySupportedByAffix(null, { root: 'कखगघङ', suffixes: ['ई'] }), false);
  assert.equal(morphologySupportedByAffix({ root: 'राम' }, { root: 'राम', suffixes: ['को'] }), true);
  assert.equal(morphologySupportedByAffix({ root: 'शासन', stem: 'अनुशासन' }, { root: 'अनुशासन', suffixes: ['मा'] }), true);
});

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

test('reviewed semantic-class citations open the final-dirgha reference with bounded coverage', () => {
  for (const code of ['3(क)(ऊ)-4', '3(क)(ऊ)-6', 'PS-Saisanik-ह्रस्वदीर्घ-(थ)', 'PS-Saisanik-ह्रस्वदीर्घ-(थ)-सजीव']) {
    assert.match(wrapRuleTooltip('ह्रस्व/दीर्घ स्वर नियम', 'HrasvaDirgha', { ruleCode: code }), /data-target="ka-uu"/);
    assert.match(getRuleSummary(code, 'HrasvaDirgha'), /सूचीकृत स्त्रीलिङ्गी विशेषण र निर्जीव नाम/);
    assert.match(getRuleSummary(code, 'HrasvaDirgha'), /सम्धी, जोगी, खसी र हात्ती/);
    assert.match(getRuleSummary(code, 'HrasvaDirgha'), /हात्तीको, खसीलाई/);
  }
});

test('feminine predicate and converb citations open their own explanation with lexical limits', () => {
  const section = RULES_SECTIONS.find(s => s.categoryCode === 'HrasvaDirgha');
  for (const code of ['PS-Saisanik-ह्रस्वदीर्घ-(ब)', 'PS-Saisanik-ह्रस्वदीर्घ-(भ)', 'PS-Saisanik-ह्रस्वदीर्घ-(ब)/(भ)', 'PS-Saisanik-ह्रस्वदीर्घ-(भ)-context-कृदन्त']) {
    const target = getReferenceTargetForRule(code, 'HrasvaDirgha');
    assert.equal(target.targetId, 'saishanik-final-i-verbs');
    assert.ok(section.referenceTargets.some(t => t.id === target.targetId));
    assert.match(wrapRuleTooltip('ह्रस्व/दीर्घ स्वर नियम', 'HrasvaDirgha', { ruleCode: code }), /data-target="saishanik-final-i-verbs"/);
    const summary = getRuleSummary(code, 'HrasvaDirgha');
    assert.match(summary, /स्त्रीलिङ्गी समापक क्रिया/);
    assert.match(summary, /पढी = पढेर/);
    for (const word of ['मिलाइ', 'पारि', 'लेखि']) assert.ok(summary.includes(word));
    assert.match(summary, /वाक्यको सन्दर्भ चाहिन्छ/);
    assert.match(summary, /सबै छोटा इकारान्त शब्दलाई दीर्घ बनाइँदैन/);
    assert.match(summary, /खोला पारि बस्छ/);
    assert.match(summary, /यो निश्चित त्रुटि होइन/);
    assert.match(summary, /सबै सच्याउने कार्यले यसलाई बदल्दैन/);
    assert.match(summary, /पत्रहरूलाई लेखि पठाइन्/);
  }
  setReferenceContext({ incorrect: 'नभइ', correction: 'नभई', categoryCode: 'HrasvaDirgha',
    targetId: 'saishanik-final-i-verbs', rule: 'PS-Saisanik-ह्रस्वदीर्घ-(ब)/(भ)' });
  assert.match(referenceHost.innerHTML, /id="ref-HrasvaDirgha-saishanik-final-i-verbs"/);
  assert.match(referenceHost.innerHTML, /पढी = पढेर/);
  assert.doesNotMatch(referenceHost.innerHTML, /id="ref-HrasvaDirgha-ka-uu"/);
});

test('origin badges distinguish missing evidence, inference, and documented origin', () => {
  for (const analysis of [null, { origin: 'deshaj' },
    { origin: 'deshaj', origin_source: 'future-source' },
    { origin: 'future-origin', origin_source: 'kosha' }]) {
    assert.equal(originPresentation(analysis).label, 'उत्पत्ति अज्ञात');
  }
  assert.equal(originPresentation({ origin: 'deshaj', origin_source: 'unknown' }).label, 'उत्पत्ति अज्ञात');
  assert.equal(originPresentation({ origin: 'unknown', origin_source: 'unknown' }).cssClass, 'origin-unknown');
  assert.equal(originPresentation({ origin: 'aagantuk', origin_source: 'heuristic' }).label, 'आगन्तुक (अनुमानित)');
  assert.equal(originPresentation({ origin: 'tatsam', origin_source: 'kosha' }).label, 'तत्सम');
  assert.equal(originPresentation({ origin: 'tadbhav', origin_source: 'override' }).label, 'तद्भव');
});

test('inspector sandhi rows need matching reviewed compound members, not a spelling score', async () => {
  const { sandhiSupportedByCompound } = await import('../js/inspection-context.js');
  assert.equal(sandhiSupportedByCompound({ left: 'वि', right: 'कास', confidence: 0.9 }, []), false);
  assert.equal(sandhiSupportedByCompound({ left: 'याता', right: 'आयात' }, []), false);
  const compounds = [{ left: 'सूर्य', right: 'उदय' }];
  assert.equal(sandhiSupportedByCompound({ left: 'सूर्य', right: 'उदय' }, compounds), true);
  assert.equal(sandhiSupportedByCompound({ left: 'सूर्या', right: 'उदय' }, compounds), false);
  assert.equal(sandhiSupportedByCompound(null, compounds), false);
});
