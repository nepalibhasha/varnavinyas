/**
 * App entry point — WASM init, tab routing, sample text, rule navigation.
 */
import { initialize } from './wasm-bridge.js';
import { initChecker, setText } from './checker.js';
import { initTransliterator } from './transliterator.js';
import { initDeriver } from './deriver.js';
import { initReference, highlightCard } from './reference.js';

const SAMPLE_TEXT =
  'नेपाल एक सुन्दर देश हो। यहाँको प्रसाशन राम्रो हुनुपर्छ। अत्याधिक खर्च गर्नु हुँदैन। राजनैतिक स्थिरता आवश्यक छ।';

/** Stores the tab to return to when the user clicks "back" from reference. */
let returnState = null;

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
  initReference();

  // Set up tab routing
  initTabs();

  // Set up rule-ref click navigation
  initRuleNavigation();

  // Load sample text
  setText(SAMPLE_TEXT);
}

function initTabs() {
  const tabs = document.querySelectorAll('.tab');
  const panels = document.querySelectorAll('.tab-panel');

  tabs.forEach((tab) => {
    tab.addEventListener('click', () => {
      const target = tab.dataset.tab;
      switchToTab(target);
      // Clear return state when user navigates tabs manually
      if (target !== 'reference') {
        hideBackButton();
      }
    });
  });
}

/**
 * Switch to a tab by name. Used by tab buttons and rule navigation.
 */
function switchToTab(tabName) {
  const tabs = document.querySelectorAll('.tab');
  const panels = document.querySelectorAll('.tab-panel');

  tabs.forEach((t) => {
    const isTarget = t.dataset.tab === tabName;
    t.classList.toggle('active', isTarget);
    t.setAttribute('aria-selected', isTarget ? 'true' : 'false');
  });

  panels.forEach((p) => {
    const isTarget = p.id === `tab-${tabName}`;
    p.classList.toggle('active', isTarget);
    p.hidden = !isTarget;
  });
}

/**
 * Get the currently active tab name.
 */
function getActiveTab() {
  const active = document.querySelector('.tab.active');
  return active ? active.dataset.tab : null;
}

/**
 * Set up delegated click handler for .rule-ref elements.
 * Clicking navigates to the reference tab and scrolls to the matching card.
 */
function initRuleNavigation() {
  document.addEventListener('click', (e) => {
    const ruleRef = e.target.closest('.rule-ref');
    if (!ruleRef) return;

    const categoryCode = ruleRef.dataset.category;
    if (!categoryCode) return;

    // Don't navigate if already on reference tab
    const currentTab = getActiveTab();
    if (currentTab === 'reference') {
      highlightCard(categoryCode);
      return;
    }

    // Save return state
    returnState = { tab: currentTab, scrollY: window.scrollY };

    // Switch to reference tab and scroll to the card
    switchToTab('reference');
    showBackButton();

    // Small delay to let the panel render before scrolling
    requestAnimationFrame(() => {
      highlightCard(categoryCode);
    });
  });

  // Back button handler
  const backBtn = document.getElementById('ref-back-btn');
  if (backBtn) {
    backBtn.addEventListener('click', goBack);
  }
}

function showBackButton() {
  const btn = document.getElementById('ref-back-btn');
  if (btn) btn.hidden = false;
}

function hideBackButton() {
  const btn = document.getElementById('ref-back-btn');
  if (btn) btn.hidden = true;
  returnState = null;
}

function goBack() {
  if (!returnState) return;
  const { tab, scrollY } = returnState;
  switchToTab(tab);
  hideBackButton();
  requestAnimationFrame(() => {
    window.scrollTo({ top: scrollY, behavior: 'auto' });
  });
}

main();
