import assert from 'node:assert/strict';
import fs from 'node:fs';
import { spawnSync } from 'node:child_process';
const fixtures = JSON.parse(fs.readFileSync('docs/tests/mobile_diagnostics.json', 'utf8'));
for (const entry of fixtures.cases) {
  const args = ['check', '--format', 'json', '--orthography-mode', entry.options.orthography_mode];
  if (entry.options.grammar) args.push('--grammar');
  const result = spawnSync(process.argv[2], args, { input: entry.text, encoding: 'utf8' });
  assert.ifError(result.error);
  assert.equal(result.status, entry.expected_diagnostics.some(d => d.kind === 'Error') ? 1 : 0,
    `${entry.id}: ${result.stderr}`);
  const actual = JSON.parse(result.stdout);
  assert.equal(actual.length, entry.expected_diagnostics.length, entry.id);
  entry.expected_diagnostics.forEach((expected, index) => {
    const found = actual[index];
    for (const key of ['incorrect', 'correction', 'rule_code', 'category_code', 'kind', 'explanation']) {
      assert.equal(found[key], expected[key], `${entry.id}: ${key}`);
    }
    assert.ok(Math.abs(found.confidence - expected.confidence) < 1e-6);
    const prefix = Buffer.from(entry.text).subarray(0, expected.span_start).toString('utf8');
    const lines = prefix.split('\n');
    assert.equal(found.line, lines.length);
    assert.equal(found.column, Array.from(lines.at(-1)).length + 1);
    assert.deepEqual((found.alternate_reasons || []).map(r => r.rule_code),
      (expected.alternate_reasons || []).map(r => r.rule_code));
  });
}
console.log(`CLI: ${fixtures.cases.length} shared diagnostic fixtures passed`);
const origins = JSON.parse(fs.readFileSync('docs/tests/origin_classification.json', 'utf8'));
for (const entry of origins.cases) {
  const result = spawnSync(process.argv[2], ['classify', entry.word, '--format', 'json'], { encoding: 'utf8' });
  assert.ifError(result.error);
  assert.equal(result.status, 0, result.stderr);
  const actual = JSON.parse(result.stdout);
  assert.equal(actual.origin, entry.expected.origin, entry.id);
  assert.equal(actual.source, entry.expected.source, entry.id);
  assert.ok(Math.abs(actual.confidence - entry.expected.confidence) < 1e-6, entry.id);
}
console.log(`CLI: ${origins.cases.length} shared origin fixtures passed`);
