import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
const [directory, expectedCommit] = process.argv.slice(2);
const manifest = JSON.parse(fs.readFileSync(path.join(directory, 'manifest.json'), 'utf8'));
assert.equal(manifest.manifest_version, 1);
assert.equal(manifest.runtime_offline, true);
assert.match(manifest.source_commit, /^[0-9a-f]{40}$/);
if (expectedCommit) assert.equal(manifest.source_commit, expectedCommit);
const names = new Set();
for (const file of manifest.files) {
  assert.ok(!names.has(file.path), `duplicate ${file.path}`);
  assert.ok(!path.isAbsolute(file.path) && !file.path.split('/').includes('..'));
  names.add(file.path);
  const content = fs.readFileSync(path.join(directory, file.path));
  assert.equal(content.length, file.bytes, file.path);
  assert.equal(createHash('sha256').update(content).digest('hex'), file.sha256, file.path);
}
assert.ok(names.has(manifest.fixtures.path));
assert.equal(manifest.files.find(f => f.path === manifest.fixtures.path).sha256, manifest.fixtures.sha256);
assert.ok(names.has(manifest.origin_fixtures.path));
assert.equal(manifest.files.find(f => f.path === manifest.origin_fixtures.path).sha256, manifest.origin_fixtures.sha256);
assert.equal(manifest.origin_classification.provenance_available, true);
assert.equal(manifest.origin_classification.unknown_origin, null);
assert.deepEqual(manifest.orthography_modes, ['academy-strict', 'common-editorial']);
if (manifest.platform === 'ios') {
  assert.ok(names.has('bindings/varnavinyas_bindings_uniffi.swift'));
  assert.ok(names.has('bindings/varnavinyas_bindings_uniffiFFI.h'));
  assert.ok(names.has('bindings/varnavinyas_bindings_uniffiFFI.modulemap'));
  assert.ok(names.has('evaluation/evaluate.swift'));
  assert.equal([...names].filter(n => n.endsWith('/Headers/module.modulemap')).length, 2);
  assert.equal([...names].filter(n => n.endsWith('.a')).length, 2);
} else if (manifest.platform === 'android') {
  assert.ok(names.has(manifest.bindings));
  assert.ok(names.has('evaluation/Evaluate.kt'));
  for (const abi of ['arm64-v8a', 'armeabi-v7a', 'x86_64']) {
    const library = `jniLibs/${abi}/libvarnavinyas_bindings_uniffi.so`;
    assert.ok(names.has(library));
    assert.equal(fs.readFileSync(path.join(directory, library)).subarray(0, 4).toString('hex'), '7f454c46');
  }
}
console.log(`Verified ${manifest.artifact} ${manifest.artifact_version}: ${manifest.source_commit}`);
