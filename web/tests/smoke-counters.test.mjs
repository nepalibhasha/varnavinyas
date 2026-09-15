import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';

const script = readFileSync(new URL('../smoke-test.sh', import.meta.url), 'utf8');

for (const [helper, counter] of [['pass', 'PASS'], ['fail', 'FAIL']]) {
  test(`${helper} counter commands succeed from zero and preserve reporting`, () => {
    const definition = script.split('\n').find(line => line.startsWith(`${helper}() {`));
    assert.ok(definition);
    // Observe each command's status even on macOS Bash 3.2, which does not
    // apply errexit to arithmetic commands the way CI's Bash does.
    const result = spawnSync('bash', ['-c', `
      set -uT
      trap 'if [ "$?" -ne 0 ]; then exit 97; fi' DEBUG
      ${counter}=0
      ${definition}
      ${helper} first
      ${helper} second
      test "$${counter}" -eq 2
    `], { encoding: 'utf8' });
    assert.equal(result.status, 0, result.stderr);
    const output = helper === 'pass' ? result.stdout : result.stderr;
    assert.match(output, new RegExp(`${counter}: first`));
    assert.match(output, new RegExp(`${counter}: second`));
  });
}
