import test from 'node:test';
import assert from 'node:assert/strict';
import { ORTHOGRAPHY_STORAGE_KEY, loadOrthographyMode, saveOrthographyMode, orthographyModeNote } from '../js/orthography-policy.js';

test('strict remains the default; a selected editorial preference survives reload', () => {
  const values = new Map();
  const storage = { getItem: key => values.get(key), setItem: (key, value) => values.set(key, value) };
  assert.equal(loadOrthographyMode(storage), 'academy-strict');
  saveOrthographyMode(storage, 'common-editorial');
  assert.equal(loadOrthographyMode(storage), 'common-editorial');
  saveOrthographyMode(storage, 'academy-strict');
  assert.equal(loadOrthographyMode(storage), 'academy-strict');
  values.set(ORTHOGRAPHY_STORAGE_KEY, 'unknown');
  assert.equal(loadOrthographyMode(storage), 'academy-strict');
});

test('blocked storage does not prevent checking with a session choice', () => {
  const blocked = { getItem() { throw new Error('blocked'); }, setItem() { throw new Error('blocked'); } };
  assert.equal(loadOrthographyMode(blocked), 'academy-strict');
  assert.doesNotThrow(() => saveOrthographyMode(blocked, 'common-editorial'));
});

test('mode descriptions explain example spellings and optional bulk-correction behavior', () => {
  for (const mode of ['academy-strict', 'common-editorial']) {
    for (const word of ['संघीय', 'कांग्रेस', 'संकेत']) assert.ok(orthographyModeNote(mode).includes(word));
  }
  assert.match(orthographyModeNote('common-editorial'), /सबै त्रुटि सच्याउने कार्यले यी रूप बदल्दैन/);
  assert.match(orthographyModeNote('academy-strict'), /काङ्ग्रेस शब्दकोशीय रूप/);
});
