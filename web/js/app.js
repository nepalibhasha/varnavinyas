/**
 * App entry point — WASM init, view switching, sample text, rule navigation.
 */
import { initialize } from './wasm-bridge.js';
import { initChecker, setText } from './checker.js';
import { initReference, highlightCard } from './reference.js';

const SAMPLE_TEXT =
  'नेपाल एक सुन्दर देश हो। यहाँको प्रसाशन राम्रो हुनुपर्छ। अत्याधिक खर्च गर्नु हुँदैन। राजनैतिक स्थिरता आवश्यक छ।';

/** Stores the scroll position to return to when the user clicks "back" from reference. */
let returnScrollY = null;

async function main() {
  await renderBuildInfo();

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
  initReference();

  // Set up view switching
  initViewSwitching();

  // Set up rule-ref click navigation
  initRuleNavigation();

  // Load sample text
  setText(SAMPLE_TEXT);
}

async function renderBuildInfo() {
  const el = document.getElementById('footer-build');
  if (!el) return;

  try {
    const res = await fetch('build-info.json', { cache: 'no-store' });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const info = await res.json();
    const sha = info?.git_sha || 'unknown';
    const builtAt = info?.built_at_utc || 'unknown';
    el.textContent = `Build: ${sha} · ${builtAt}`;
  } catch (_err) {
    el.textContent = 'Build: unavailable';
  }
}

/**
 * Initialize view switching: header rules button and back button.
 */
function initViewSwitching() {
  const rulesBtn = document.getElementById('nav-rules-btn');
  if (rulesBtn) {
    rulesBtn.addEventListener('click', () => {
      returnScrollY = window.scrollY;
      switchToView('reference');
    });
  }

  const backBtn = document.getElementById('ref-back-btn');
  if (backBtn) {
    backBtn.addEventListener('click', goBack);
  }
}

/**
 * Switch to a view by name ('editor' or 'reference').
 */
function switchToView(name) {
  const views = document.querySelectorAll('.view');
  views.forEach((v) => {
    const isTarget = v.id === `view-${name}`;
    v.classList.toggle('active', isTarget);
    v.hidden = !isTarget;
  });

  // Show/hide the rules button based on current view
  const rulesBtn = document.getElementById('nav-rules-btn');
  if (rulesBtn) {
    rulesBtn.hidden = name === 'reference';
  }
}

/**
 * Get the currently active view name.
 */
function getActiveView() {
  const active = document.querySelector('.view.active');
  if (!active) return null;
  return active.id.replace('view-', '');
}

/**
 * Set up delegated click handler for .rule-ref elements.
 * Clicking navigates to the reference view and scrolls to the matching card.
 */
function initRuleNavigation() {
  document.addEventListener('click', (e) => {
    const ruleRef = e.target.closest('.rule-ref');
    if (!ruleRef) return;

    const categoryCode = ruleRef.dataset.category;
    if (!categoryCode) return;

    // Don't navigate if already on reference view
    const currentView = getActiveView();
    if (currentView === 'reference') {
      highlightCard(categoryCode);
      return;
    }

    // Save return state
    returnScrollY = window.scrollY;

    // Switch to reference view and scroll to the card
    switchToView('reference');

    // Small delay to let the panel render before scrolling
    requestAnimationFrame(() => {
      highlightCard(categoryCode);
    });
  });
}

function goBack() {
  switchToView('editor');
  if (returnScrollY != null) {
    requestAnimationFrame(() => {
      window.scrollTo({ top: returnScrollY, behavior: 'auto' });
    });
  }
  returnScrollY = null;
}

main();
