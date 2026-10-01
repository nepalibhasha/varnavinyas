// Test the downstream package itself, including its compatibility with our UI.
// Usage: node web/test-artifact.mjs [unpacked-artifact-directory]
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { morphologySupportedByAffix, originPresentation, sandhiSupportedByCompound } from './js/inspection-context.js';
import { applyCorrections, canBulkApplyDiagnostic } from './js/corrections.js';
import { getReferenceTargetForRule } from './js/rules-data.js';

const directory = path.resolve(process.argv[2] || 'web/dist/varnavinyas-browser-artifact');
const manifest = JSON.parse(await readFile(path.join(directory, 'manifest.json'), 'utf8'));
const info = JSON.parse(await readFile(path.join(directory, manifest.build_info), 'utf8'));
assert.equal(manifest.git_sha, info.git_sha);
assert.equal(manifest.artifact_api_version, 1);
assert.deepEqual(manifest.capabilities.affix_segment_kinds,
  ['prefix', 'plural_marker', 'case_marker', 'particle', 'postposition', 'comparison_marker']);
assert.equal(manifest.capabilities.word_analysis_origin_provenance, true);
assert.deepEqual(manifest.capabilities.word_analysis_origin_sources,
  ['kosha', 'override', 'heuristic', 'unknown']);

const wasm = await import(pathToFileURL(path.join(directory, manifest.entry_js)));
wasm.initSync({ module: await readFile(path.join(directory, manifest.entry_wasm)) });
// The same diagnostic payload is consumed by Python and generated mobile bindings.
const sharedFixtures = JSON.parse(await readFile(new URL('../docs/tests/mobile_diagnostics.json', import.meta.url), 'utf8'));
const normalizeFixture = value => Array.isArray(value) ? value.map(normalizeFixture)
  : value && typeof value === 'object' ? Object.fromEntries(Object.entries(value)
    .map(([key, item]) => [key, normalizeFixture(item)]))
  : typeof value === 'number' ? Math.round(value * 1e6) / 1e6 : value;
for (const fixture of sharedFixtures.cases) {
  const actual = wasm.check_text_value_with_options(fixture.text, fixture.options.grammar,
    fixture.options.orthography_mode);
  assert.deepEqual(normalizeFixture(actual), normalizeFixture(fixture.expected_diagnostics), fixture.id);
  for (const diagnostic of actual.filter(d => d.kind === 'Ambiguous')) {
    assert.equal(canBulkApplyDiagnostic(diagnostic), false, fixture.id);
  }
}
// Progressive participles preserve main/auxiliary evidence and internal endings.
for (const [word, main, lemma, auxiliary] of [
  ['खोजिरहेको', 'खोजि', 'खोज्नु', 'रहेको'],
  ['खोजिरहेकी', 'खोजि', 'खोज्नु', 'रहेकी'],
  ['खोजिरहेका', 'खोजि', 'खोज्नु', 'रहेका'],
  ['खाइरहेको', 'खाइ', 'खानु', 'रहेको'],
  ['उर्लिरहेको', 'उर्लि', 'उर्लनु', 'रहेको'],
  ['ओर्लिरहेको', 'ओर्लि', 'ओर्लनु', 'रहेको'],
]) {
  const analysis = wasm.analyze_progressive_value(word);
  assert.equal(analysis.main_form, main);
  assert.equal(analysis.main_lemma, lemma);
  assert.equal(analysis.auxiliary_form, auxiliary);
  assert.equal(analysis.auxiliary_lemma, 'रहनु');
  assert.equal(analysis.negative, false);
  assert.deepEqual(JSON.parse(wasm.analyze_progressive(word)), analysis);
  assert.equal(wasm.check_word_value(word), null, word);
  assert.deepEqual(wasm.best_affix_analysis_value(word).suffixes, [], word);
}
for (const word of ['झझझिरहेकी', 'मइरहेको', 'खोजीरहेको', 'खोजिरहनेको']) {
  assert.equal(wasm.analyze_progressive_value(word), null, word);
}
assert.equal(wasm.best_affix_analysis_value('खोजिरहेकाले').stem, 'खोजिरहेका');
assert.deepEqual(wasm.best_affix_analysis_value('खोजिरहेकाले').suffixes, ['ले']);
assert.equal(wasm.best_affix_analysis_value('मानिससरह').suffix_segments[0].kind, 'comparison_marker');
assert.equal(wasm.best_affix_analysis_value('घरसम्म').suffix_segments[0].kind, 'postposition');
assert.equal(wasm.best_affix_analysis_value('घरको').suffix_segments[0].kind, 'case_marker');
// Public compound analysis needs reviewed formation evidence, not lexical coincidence.
for (const word of ['सवारीमा', 'दशकमा', 'आयात', 'आर्थिक', 'विकास', 'यातायात',
  'व्यवस्थापन', 'रिसाइकल', 'आधारमा', 'छलफल', 'विवरण', 'सबैलाई']) {
  const compounds = wasm.analyze_compound_value(word);
  assert.deepEqual(compounds, [], word);
  assert.deepEqual(JSON.parse(wasm.analyze_compound(word)), compounds, word);
  assert.equal(sandhiSupportedByCompound(wasm.sandhi_split_best_for_compound_value(word), compounds), false, word);
  for (const mode of ['academy-strict', 'common-editorial']) {
    for (const text of [word, `🙂 ${word} सम्बन्धी जानकारी उपलब्ध छ।`]) {
      assert.ok(!wasm.check_text_value_with_options(text, true, mode)
        .some(d => d.rule_code === 'samasa-heuristic'), text);
    }
  }
}
for (const [word, left, right] of [['सूर्योदय', 'सूर्य', 'उदय'],
  ['महोत्सव', 'महा', 'उत्सव'], ['पूर्वाधार', 'पूर्व', 'आधार'],
  ['मापदण्ड', 'माप', 'दण्ड'], ['एकचक्र', 'एक', 'चक्र']]) {
  const compounds = wasm.analyze_compound_value(word);
  assert.equal(compounds.length, 1, word);
  assert.equal(compounds[0].left, left, word);
  assert.equal(compounds[0].right, right, word);
  const split = wasm.sandhi_split_best_for_compound_value(word);
  if (split) assert.equal(sandhiSupportedByCompound(split, compounds), true, word);
}
for (const word of ['आयात', 'आर्थिक', 'आधार', 'आचार', 'आकाश']) {
  assert.ok(wasm.sandhi_split_value(word).every(c => c.left !== word && c.right !== word), word);
}
// Exercise contextual reading, UTF-8 spans and actual bulk policy together.
const converbRule = 'PS-Saisanik-ह्रस्वदीर्घ-(भ)-context-कृदन्त';
for (const mode of ['academy-strict', 'common-editorial']) {
  for (const grammar of [false, true]) {
    for (const [text, short, long, kind] of [
      ['🙂 काम पूरा पारि घर फर्कियो।', 'पारि', 'पारी', 'Error'],
      ['वृत्ताकार पारि बनाइएको चौतारो', 'पारि', 'पारी', 'Error'],
      ['कुरा मिलाइ नाफा खाने व्यक्ति', 'मिलाइ', 'मिलाई', 'Ambiguous'],
      ['खाना पकाइ खायो।', 'पकाइ', 'पकाई', 'Ambiguous'],
      ['टिको लगाइ दिएर घर गयो।', 'लगाइ', 'लगाई', 'Ambiguous'],
      ['खाना बनाइ राख्यो।', 'बनाइ', 'बनाई', 'Ambiguous'],
      ['निश्चिन्त भइ समय बिताई बस्नु', 'भइ', 'भई', 'Ambiguous'],
      ['🙂 चिठी लेखि पठायो।', 'लेखि', 'लेखी', 'Ambiguous'],
      ['🙂 पत्रहरूलाई लेखि पठाइन्।', 'लेखि', 'लेखी', 'Ambiguous'],
      ['चिठी लेखि पठाउँछ।', 'लेखि', 'लेखी', 'Ambiguous'],
    ]) {
      const diagnostic = wasm.check_text_value_with_options(text, grammar, mode)
        .find(d => d.incorrect === short);
      assert.equal(diagnostic?.correction, long, text);
      assert.equal(diagnostic.kind, kind, text);
      assert.equal(diagnostic.rule_code, converbRule);
      assert.equal(getReferenceTargetForRule(diagnostic.rule_code, 'HrasvaDirgha').targetId,
        'saishanik-final-i-verbs');
      const bytes = new TextEncoder().encode(text);
      assert.equal(new TextDecoder().decode(bytes.slice(diagnostic.span_start, diagnostic.span_end)), short);
      const charStart = new TextDecoder().decode(bytes.slice(0, diagnostic.span_start)).length;
      const choice = { ...diagnostic, charStart, charEnd: charStart + short.length };
      assert.equal(canBulkApplyDiagnostic(choice), kind === 'Error');
      assert.equal(applyCorrections(text, text, [choice].filter(canBulkApplyDiagnostic)),
        kind === 'Error' ? text.replace(short, long) : text);
      assert.equal(applyCorrections(text, text, [choice]), text.replace(short, long));
    }
    for (const text of ['यसको मिलाइ राम्रो छ।', 'खानाको पकाइ राम्रो भयो।',
      'तिम्रा लेखि उनी मरेबराबरै भए।', 'राम खोला पारि बस्छ।',
      'चिठी, लेखि पठायो।', 'चिठी लेखि\nपठायो।', 'पूरा ‘पारि’ घर फर्कियो।']) {
      assert.ok(!wasm.check_text_value_with_options(text, grammar, mode)
        .some(d => d.rule_code === converbRule), text);
    }
  }
}
for (const name of manifest.required_exports) assert.equal(typeof wasm[name], 'function', name);

const unknown = wasm.analyze_word_value('कखगघङ');
assert.equal(unknown.origin, 'unknown');
assert.equal(unknown.origin_source, 'unknown');
assert.equal(unknown.origin_confidence, 0);
assert.equal(originPresentation(unknown).label, 'उत्पत्ति अज्ञात');
const inferred = wasm.analyze_word_value('क़लम');
assert.equal(inferred.origin_source, 'heuristic');
assert.match(originPresentation(inferred).label, /अनुमानित/);
const documented = wasm.analyze_word_value('हामी');
assert.ok(['kosha', 'override'].includes(documented.origin_source));
assert.equal(originPresentation(documented).label, 'तद्भव');

for (const word of ['हेर', 'हेर्']) assert.equal(wasm.derive_value(word).output, word);
assert.notEqual(wasm.decompose_word_value('संघीय').root, 'घीय');
assert.ok(!wasm.decompose_word_value('हामी').suffixes.includes('ई'));
assert.equal(wasm.decompose_word_value('तिनी').root, 'तिन');
assert.ok(wasm.decompose_word_value('तिनी').suffixes.includes('ई'));
const pronoun = wasm.check_word_value('हामि');
assert.equal(pronoun.correction, 'हामी');
assert.equal(pronoun.rule_code, '3(क)(ऊ)-7');
assert.equal((pronoun.alternate_reasons || []).length, 0);
const variant = wasm.check_text_value_with_options('संघीय', false, 'common-editorial')
  .find(diagnostic => diagnostic.incorrect === 'संघीय');
assert.equal(variant.kind, 'Variant');
// Exercise source-approved abbreviations through the actual browser WASM
// exports, including UTF-8 spans and both downstream orthography modes.
for (const mode of ['academy-strict', 'common-editorial']) {
  for (const text of ['बी.बी.सी.', 'सी.डी.ओ.\nनेपाल',
    'बी.बी.सी. समाचार प्रसारण गर्छ।', 'एस.एल.सी. परीक्षा भयो।', '🙂 ‘बी.बी.सी.’ समाचार']) {
    assert.deepEqual(wasm.check_text_value_with_options(text, false, mode), [], text);
  }
  const nearby = wasm.check_text_value_with_options('हामि बी.बी.सी. समाचार पढ्छौँ.', false, mode);
  assert.ok(nearby.some(d => d.incorrect === 'हामि' && d.correction === 'हामी'));
  assert.ok(nearby.some(d => d.incorrect === '.' && d.correction === '।'));
  assert.ok(!nearby.some(d => d.incorrect === 'सी'));
  for (const [wrong, correct] of [['मीलेको', 'मिलेको'], ['प्रभू', 'प्रभु'],
    ['साधू', 'साधु'], ['श्रद्धालू', 'श्रद्धालु'], ['बधू', 'वधू']]) {
    const text = `🙂 ${wrong}।`;
    const diagnostics = wasm.check_text_value_with_options(text, false, mode);
    assert.equal(diagnostics.length, 1, text);
    assert.equal(diagnostics[0].correction, correct);
    assert.equal(diagnostics[0].kind, 'Error');
    const bytes = new TextEncoder().encode(text);
    const { span_start: start, span_end: end } = diagnostics[0];
    assert.equal(new TextDecoder().decode(bytes.slice(start, end)), wrong);
    assert.equal(wasm.check_word_value(wrong).correction, correct);
  }
  for (const word of ['मिलेको', 'प्रभु', 'साधु', 'श्रद्धालु', 'वधू', 'रामकोपनि', 'संसदमा']) {
    assert.deepEqual(wasm.check_text_value_with_options(word, false, mode), [], word);
  }
}
assert.ok(wasm.analyze_word_value('प्रभु').rule_notes
  .some(note => note.rule_code === '3(क)(इ)-PS-Saisanik-(छ)'));
// Semantic classes come from reviewed source rows, not a vowel/POS guess.
const semanticInventory = await readFile(new URL('../data/rule_inventories/final_ii_semantic_classes.tsv', import.meta.url), 'utf8');
for (const row of semanticInventory.trim().split('\n').slice(1)) {
  const [correct, semanticClass] = row.split('\t');
  const code = { feminine_adjective: '3(क)(ऊ)-4', inanimate_noun: '3(क)(ऊ)-6',
    ps_inanimate_noun: 'PS-Saisanik-ह्रस्वदीर्घ-(थ)',
    ps_animate_noun: 'PS-Saisanik-ह्रस्वदीर्घ-(थ)-सजीव' }[semanticClass];
  const wrong = correct.slice(0, -1) + 'ि';
  assert.equal(wasm.derive_value(wrong).output, correct);
  assert.ok(wasm.analyze_word_value(correct).rule_notes.some(note => note.rule_code === code));
  for (const mode of ['academy-strict', 'common-editorial']) {
    const text = `🙂 ${wrong}।`;
    const diagnostics = wasm.check_text_value_with_options(text, false, mode);
    assert.equal(diagnostics.length, 1, text);
    assert.equal(diagnostics[0].correction, correct);
    assert.equal(diagnostics[0].rule_code, code);
    assert.equal(diagnostics[0].kind, 'Error');
    const bytes = new TextEncoder().encode(text);
    assert.equal(new TextDecoder().decode(bytes.slice(diagnostics[0].span_start, diagnostics[0].span_end)), wrong);
    assert.deepEqual(wasm.check_text_value_with_options(correct, false, mode), [], correct);
  }
}
for (const word of ['माथि', 'नाति', 'समिति', 'गाडीमा', 'फर्सीको']) {
  assert.deepEqual(wasm.check_text_value(word, false), [], word);
}
for (const [surface, left, right] of [['अत्याचार', 'अति', 'आचार'],
  ['प्रत्यादेश', 'प्रति', 'आदेश'], ['स्वागत', 'सु', 'आगत']]) {
  assert.equal(wasm.sandhi_apply_value(left, right).output, surface);
  const candidate = wasm.sandhi_split_value(surface).find(c => c.left === left && c.right === right);
  assert.ok(candidate, surface);
  assert.notEqual(candidate.authority, 'Exploratory');
  assert.equal(wasm.sandhi_apply_value(candidate.left, candidate.right).output, surface);
}
assert.deepEqual(wasm.sandhi_split_value('नेपाल'), []);
for (const word of ['फर्सी', 'विद्यार्थी', 'कखगघङी']) {
  assert.equal(morphologySupportedByAffix(wasm.best_affix_analysis_value(word), wasm.decompose_word_value(word)), false, word);
}
const studentStem = wasm.best_affix_analysis_value('विद्यार्थी').root;
assert.equal(studentStem, 'विद्यार्थी');
assert.ok(wasm.sandhi_split_value(studentStem).some(c => c.left === 'विद्या' && c.right === 'अर्थी'));
assert.equal(morphologySupportedByAffix(wasm.best_affix_analysis_value('रामको'), wasm.decompose_word_value('रामको')), true);
console.log(`Browser artifact ${manifest.artifact_version} (${manifest.git_sha}): runtime and origin presentation checks passed.`);
