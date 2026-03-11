/**
 * App entry point — WASM init, view switching, sample text, rule navigation.
 */
import { initialize } from './wasm-bridge.js';
import { initChecker, setText } from './checker.js';
import { initReference, highlightCard, setReferenceContext } from './reference.js';

// Sample text is intentionally built from Academy notice examples so the default
// UI exercises rule families users can cross-check in the Rules Reference.
const SAMPLE_TEXT = [
  'सरकारी प्रसाशनमा अत्याधिक ढिलाइ र राजनैतिक अस्थिरता हुँदा औपचारिक नेपाली लेखन बिग्रन्छ।',
  'अभीमान, सूमार्ग र भौतीक जस्ता रूपभन्दा अभिमान, सुमार्ग र भौतिक मानक मानिन्छन्।',
  'अहीले धेरैले आउछ, निम्ती, त्यती, नीती, दुइ र साठि जस्ता रूप लेख्छन्।',
  'महान लेख्दा हलन्त छुट्न सक्छ, नेपालि, गरिबि, कोहि र थारु जस्ता रूप भेटिन्छन्।',
  'एशिया, छेत्र, अग्यान, यकता, रिषि र व्यवहारिक जस्ता रूपले उस्तै उच्चारण र आदिवृद्धिमा पनि अन्योल ल्याउँछन्।',
  'अंक जस्तो तत्सम शब्दमा पञ्चम वर्णको प्रयोग चाहिन्छ, र “उनले भने", नेपाल सुन्दर छ. जस्ता वाक्यमा विरामचिह्न मिलाउनुपर्छ।',
].join(' ');

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
      setReferenceContext(null);
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
    const targetId = ruleRef.dataset.target || null;
    if (!categoryCode) return;
    const context = {
      categoryCode,
      targetId,
      word: ruleRef.dataset.word || '',
      incorrect: ruleRef.dataset.incorrect || '',
      correction: ruleRef.dataset.correction || '',
      explanation: ruleRef.dataset.explanation || '',
      rule: ruleRef.dataset.rule || ruleRef.textContent || '',
    };

    // Don't navigate if already on reference view
    const currentView = getActiveView();
    if (currentView === 'reference') {
      setReferenceContext(context);
      highlightCard(categoryCode, targetId);
      return;
    }

    // Save return state
    returnScrollY = window.scrollY;

    // Switch to reference view and scroll to the card
    switchToView('reference');

    // Small delay to let the panel render before scrolling
    requestAnimationFrame(() => {
      setReferenceContext(context);
      highlightCard(categoryCode, targetId);
    });
  });
}

function goBack() {
  setReferenceContext(null);
  switchToView('editor');
  if (returnScrollY != null) {
    requestAnimationFrame(() => {
      window.scrollTo({ top: returnScrollY, behavior: 'auto' });
    });
  }
  returnScrollY = null;
}

main();
