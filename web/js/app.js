/**
 * App entry point — WASM init, tab routing, sample text.
 */
import { initialize } from './wasm-bridge.js';
import { initChecker, setText } from './checker.js';
import { initTransliterator } from './transliterator.js';
import { initDeriver } from './deriver.js';

const SAMPLE_TEXT =
  'नेपाल एक सुन्दर देश हो। यहाँको प्रसाशन राम्रो हुनुपर्छ। अत्याधिक खर्च गर्नु हुँदैन। राजनैतिक स्थिरता आवश्यक छ।';

async function main() {
  // Initialize WASM
  const overlay = document.getElementById('loading-overlay');
  try {
    await initialize();
  } catch (e) {
    overlay.querySelector('.loading-text').textContent =
      `WASM लोड गर्न सकिएन: ${e.message || e}`;
    console.error('WASM init failed:', e);
    return;
  }

  // Hide loading overlay
  overlay.classList.add('hidden');
  setTimeout(() => overlay.remove(), 300);

  // Initialize modules
  initChecker();
  initTransliterator();
  initDeriver();

  // Set up tab routing
  initTabs();

  // Load sample text
  setText(SAMPLE_TEXT);
}

function initTabs() {
  const tabs = document.querySelectorAll('.tab');
  const panels = document.querySelectorAll('.tab-panel');

  tabs.forEach((tab) => {
    tab.addEventListener('click', () => {
      const target = tab.dataset.tab;

      tabs.forEach((t) => {
        t.classList.toggle('active', t === tab);
        t.setAttribute('aria-selected', t === tab ? 'true' : 'false');
      });

      panels.forEach((p) => {
        const isTarget = p.id === `tab-${target}`;
        p.classList.toggle('active', isTarget);
        p.hidden = !isTarget;
      });
    });
  });
}

main();
