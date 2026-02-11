/**
 * Transliterator module — two textareas with live conversion.
 */
import { transliterate } from './wasm-bridge.js';

let fromScheme = 'Devanagari';
let toScheme = 'Iast';

const inputEl = document.getElementById('translit-input');
const outputEl = document.getElementById('translit-output');
const directionBtn = document.getElementById('translit-direction');
const swapBtn = document.getElementById('translit-swap');
const copyBtn = document.getElementById('translit-copy');
const fromLabel = document.getElementById('translit-from-label');
const toLabel = document.getElementById('translit-to-label');
const inputLabel = document.getElementById('translit-input-label');
const outputLabel = document.getElementById('translit-output-label');

/**
 * Initialize the transliterator module.
 */
export function initTransliterator() {
  inputEl.addEventListener('input', convert);
  directionBtn.addEventListener('click', toggleDirection);
  swapBtn.addEventListener('click', swap);
  copyBtn.addEventListener('click', copyOutput);
  updateLabels();
}

function convert() {
  const text = inputEl.value;
  if (!text.trim()) {
    outputEl.value = '';
    return;
  }
  try {
    outputEl.value = transliterate(text, fromScheme, toScheme);
  } catch (e) {
    outputEl.value = `Error: ${e.message || e}`;
  }
}

function toggleDirection() {
  [fromScheme, toScheme] = [toScheme, fromScheme];
  updateLabels();
  // Re-convert with new direction
  convert();
}

function swap() {
  [fromScheme, toScheme] = [toScheme, fromScheme];
  const temp = inputEl.value;
  inputEl.value = outputEl.value;
  outputEl.value = temp;
  updateLabels();
}

function updateLabels() {
  const fromDisplay = fromScheme === 'Devanagari' ? 'देवनागरी' : 'IAST';
  const toDisplay = toScheme === 'Devanagari' ? 'देवनागरी' : 'IAST';
  fromLabel.textContent = fromDisplay;
  toLabel.textContent = toDisplay;
  inputLabel.textContent = fromDisplay;
  outputLabel.textContent = toDisplay;
  inputEl.placeholder =
    fromScheme === 'Devanagari' ? 'यहाँ लेख्नुहोस्…' : 'Type here…';
  outputEl.placeholder =
    toScheme === 'Devanagari'
      ? 'रूपान्तरित पाठ यहाँ देखिनेछ…'
      : 'Transliterated text appears here…';
}

async function copyOutput() {
  try {
    await navigator.clipboard.writeText(outputEl.value);
    const original = copyBtn.textContent;
    copyBtn.textContent = 'Copied!';
    setTimeout(() => {
      copyBtn.textContent = original;
    }, 1500);
  } catch {
    // Fallback: select and prompt user
    outputEl.select();
  }
}
