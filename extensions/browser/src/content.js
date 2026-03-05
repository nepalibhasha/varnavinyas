/**
 * Content script — captures selected Nepali text on any page.
 *
 * Listens for mouseup and selectionchange events, debounces, filters
 * empty/whitespace-only selections, and sends SELECTION_UPDATED to the
 * background service worker via chrome.runtime.sendMessage.
 */

const DEBOUNCE_MS = 250;

let debounceTimer = null;
let lastSelection = '';

/**
 * Check if a string contains at least one Devanagari character (U+0900–U+097F).
 */
function hasDevanagari(text) {
  return /[\u0900-\u097F]/.test(text);
}

/**
 * Get the current text selection, trimmed. Returns empty string if none.
 */
function getSelectedText() {
  const sel = window.getSelection();
  if (!sel || sel.isCollapsed) return '';
  return sel.toString().trim();
}

/**
 * Debounced handler: reads selection, filters, and sends to background.
 */
function onSelectionChange() {
  clearTimeout(debounceTimer);
  debounceTimer = setTimeout(() => {
    const text = getSelectedText();

    // Skip empty, non-Devanagari, or duplicate selections
    if (!text || !hasDevanagari(text) || text === lastSelection) return;

    lastSelection = text;

    chrome.runtime.sendMessage({
      type: 'SELECTION_UPDATED',
      payload: { text },
    });
  }, DEBOUNCE_MS);
}

document.addEventListener('mouseup', onSelectionChange);
document.addEventListener('selectionchange', onSelectionChange);
