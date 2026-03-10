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

function isInteractiveElement(el) {
  return Boolean(
    el.closest(
      'input, textarea, button, a, select, option, [contenteditable]:not([contenteditable="false"])'
    )
  );
}

function extractWordFromTextAtOffset(text, offset) {
  if (!text) return '';
  const chars = Array.from(text);
  if (!chars.length) return '';
  const i0 = Math.max(0, Math.min(offset, chars.length - 1));
  const isDev = (ch) => /[\u0900-\u097F]/.test(ch || '');
  let i = i0;

  if (!isDev(chars[i])) {
    if (isDev(chars[i - 1])) i -= 1;
    else return '';
  }

  let start = i;
  let end = i + 1;
  while (start > 0 && isDev(chars[start - 1])) start -= 1;
  while (end < chars.length && isDev(chars[end])) end += 1;
  return chars.slice(start, end).join('').trim();
}

function getClickedWord(event) {
  const target = event.target;
  if (!(target instanceof Element) || isInteractiveElement(target)) return '';

  if (document.caretPositionFromPoint) {
    const pos = document.caretPositionFromPoint(event.clientX, event.clientY);
    if (pos?.offsetNode?.nodeType === Node.TEXT_NODE) {
      return extractWordFromTextAtOffset(pos.offsetNode.textContent || '', pos.offset);
    }
  }

  if (document.caretRangeFromPoint) {
    const range = document.caretRangeFromPoint(event.clientX, event.clientY);
    if (range?.startContainer?.nodeType === Node.TEXT_NODE) {
      return extractWordFromTextAtOffset(
        range.startContainer.textContent || '',
        range.startOffset
      );
    }
  }

  return '';
}

/**
 * Debounced handler: reads selection, filters, and sends to background.
 */
function onSelectionChange() {
  clearTimeout(debounceTimer);
  debounceTimer = setTimeout(() => {
    const text = getSelectedText();

    if (!text) {
      lastSelection = '';
      return;
    }

    // Skip empty, non-Devanagari, or duplicate selections
    if (!hasDevanagari(text) || text === lastSelection) return;

    lastSelection = text;

    chrome.runtime.sendMessage({
      type: 'SELECTION_UPDATED',
      payload: { text },
    });
  }, DEBOUNCE_MS);
}

function onWordClick(event) {
  // If user has an explicit selection, selection handler will process it.
  const selected = getSelectedText();
  if (selected) return;

  const word = getClickedWord(event);
  if (!word || !hasDevanagari(word) || word === lastSelection) return;

  lastSelection = word;
  chrome.runtime.sendMessage({
    type: 'SELECTION_UPDATED',
    payload: { text: word },
  });
}

document.addEventListener('mouseup', onSelectionChange);
document.addEventListener('selectionchange', onSelectionChange);
document.addEventListener('click', onWordClick);
