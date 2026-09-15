// Add common source provenance, contracts, and fixtures to a release directory.
// node release/package-metadata.mjs <python|cli|ios|android> <vX.Y.Z> <directory> [target]
import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';

const [platform, version, directory, target] = process.argv.slice(2);
if (!['python', 'cli', 'ios', 'android'].includes(platform) || !/^v\d+\.\d+\.\d+$/.test(version || '') || !directory) {
  throw new Error('Expected platform, vX.Y.Z version, directory, and optional target');
}
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
fs.mkdirSync(directory, { recursive: true });
for (const [source, destination] of [
  ['docs/tests/mobile_diagnostics.json', 'fixtures/diagnostics.json'],
  ['docs/MOBILE_EVALUATION.md', 'MOBILE_EVALUATION.md'],
  ['docs/INTEGRATION_NOTES.md', 'INTEGRATION_NOTES.md'],
  ['docs/releases/native-v0.1.1.md', 'RELEASE_NOTES.md'],
  ['LICENSE-MIT', 'LICENSE-MIT'], ['LICENSE-APACHE', 'LICENSE-APACHE'],
]) {
  const output = path.join(directory, destination);
  fs.mkdirSync(path.dirname(output), { recursive: true });
  fs.copyFileSync(source, output);
}
const files = [];
function inventory(relative = '') {
  for (const entry of fs.readdirSync(path.join(directory, relative), { withFileTypes: true })) {
    const name = path.posix.join(relative, entry.name);
    if (entry.isDirectory()) inventory(name);
    else if (entry.isFile() && name !== 'manifest.json') {
      const data = fs.readFileSync(path.join(directory, name));
      files.push({ path: name, bytes: data.length, sha256: digest(data) });
    }
  }
}
inventory();
files.sort((a, b) => a.path.localeCompare(b.path));
const mobile = ['ios', 'android'].includes(platform);
const manifest = {
  manifest_version: 1, artifact: `varnavinyas-${platform}${mobile ? '-evaluation' : ''}`,
  artifact_version: version,
  source_commit: execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim(),
  platform, target: target || null, runtime_offline: true,
  diagnostic_schema_version: 1,
  span_unit: platform === 'cli' ? 'one-based-line-and-character-column' : 'utf8-bytes',
  fixtures: { path: 'fixtures/diagnostics.json', schema_version: 1,
    sha256: digest(fs.readFileSync('docs/tests/mobile_diagnostics.json')) },
  orthography_modes: ['academy-strict', 'common-editorial'],
  default_orthography_mode: 'academy-strict',
  default_punctuation_mode: 'strict', grammar_pass_available: true,
  ...(mobile ? {
    binding_generator: { name: 'uniffi', version: '0.28.3' },
    diagnostic_transport: 'JSON string',
    api: ['check_text', 'check_text_with_options', 'check_text_with_all_options', 'check_word', 'classify', 'transliterate'],
    origin_classification: { categories: ['Tatsam', 'Tadbhav', 'Deshaj', 'Aagantuk'], provenance_available: false },
    targets: platform === 'ios' ? ['aarch64-apple-ios', 'aarch64-apple-ios-sim', 'x86_64-apple-ios']
      : ['aarch64-linux-android', 'armv7-linux-androideabi', 'x86_64-linux-android'],
    ...(platform === 'ios' ? { minimum_ios: '13.0', framework: 'VarnavinyasBindingsUniFFI.xcframework', bindings: 'bindings/varnavinyas_bindings_uniffi.swift' }
      : { minimum_android_api: 21, libraries: 'jniLibs', bindings: 'bindings/uniffi/varnavinyas_bindings_uniffi/varnavinyas_bindings_uniffi.kt',
        dependencies: ['net.java.dev.jna:jna:5.13.0@aar'] }),
  } : {}),
  files,
};
fs.writeFileSync(path.join(directory, 'manifest.json'), JSON.stringify(manifest, null, 2) + '\n');
console.log(`${platform} ${version}: ${manifest.source_commit}, ${files.length} checksummed files`);
