// Test the downstream package itself, including its compatibility with our UI.
// Usage: node web/test-artifact.mjs [unpacked-artifact-directory]
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { originPresentation } from './js/inspection-context.js';

const directory = path.resolve(process.argv[2] || 'web/dist/varnavinyas-browser-artifact');
const manifest = JSON.parse(await readFile(path.join(directory, 'manifest.json'), 'utf8'));
const info = JSON.parse(await readFile(path.join(directory, manifest.build_info), 'utf8'));
assert.equal(manifest.git_sha, info.git_sha);
assert.equal(manifest.artifact_api_version, 1);
assert.equal(manifest.capabilities.word_analysis_origin_provenance, true);
assert.deepEqual(manifest.capabilities.word_analysis_origin_sources,
  ['kosha', 'override', 'heuristic', 'unknown']);

const wasm = await import(pathToFileURL(path.join(directory, manifest.entry_js)));
wasm.initSync({ module: await readFile(path.join(directory, manifest.entry_wasm)) });
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
console.log(`Browser artifact ${manifest.artifact_version} (${manifest.git_sha}): runtime and origin presentation checks passed.`);
