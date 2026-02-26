/**
 * Background service worker — stores selections, manages cache,
 * and handles dictionary lookups.
 *
 * No WASM here — all local analysis happens in the popup context
 * (validated in Phase 0 spike: 120ms popup init).
 */

const CACHE_TTL_MS = 7 * 24 * 60 * 60 * 1000; // 7 days
const CACHE_SCHEMA_VERSION = 1;

// ── Lifecycle ──

chrome.runtime.onInstalled.addListener((details) => {
  console.log('[varnavinyas] installed:', details.reason);
  if (details.reason === 'update') {
    invalidateStaleCache();
  }

  // On Chrome: disable popup so onClicked fires → opens side panel.
  // On Firefox: keep default_popup (sidePanel API doesn't exist).
  if (chrome.sidePanel) {
    chrome.action.setPopup({ popup: '' });
  }
});

if (chrome.sidePanel) {
  chrome.action.onClicked.addListener((tab) => {
    chrome.sidePanel.open({ tabId: tab.id });
  });
}

// ── Message handler ──

chrome.runtime.onMessage.addListener((message, _sender, sendResponse) => {
  if (message.type === 'SELECTION_UPDATED') {
    handleSelectionUpdated(message.payload);
    return false; // synchronous
  }

  if (message.type === 'GET_SELECTION') {
    handleGetSelection(sendResponse);
    return true; // async response
  }

  if (message.type === 'LOOKUP_REQUEST') {
    handleLookupRequest(message.payload, sendResponse);
    return true; // async response
  }

  return false;
});

// ── Selection storage ──

// Firefox 115+ supports storage.session; fall back to storage.local
const sessionStore = chrome.storage.session || chrome.storage.local;

function handleSelectionUpdated(payload) {
  sessionStore.set({ latestSelection: payload.text });

  // Forward to side panel so it auto-updates
  chrome.runtime.sendMessage({
    type: 'SELECTION_CHANGED',
    payload: { text: payload.text },
  }).catch(() => {
    // Side panel / popup not open — ignore
  });
}

function handleGetSelection(sendResponse) {
  sessionStore.get('latestSelection', (result) => {
    sendResponse({ text: result.latestSelection || '' });
  });
}

// ── Sabdasakha Dictionary API ──

const API_BASE = 'https://sabdasakha.com/api/v1';
const FETCH_TIMEOUT_MS = 5000;

async function handleLookupRequest(payload, sendResponse) {
  const { query } = payload;

  try {
    // Check cache first
    const cached = await getCachedEntry(query);
    if (cached) {
      sendResponse({ type: 'LOOKUP_RESULT', source: 'cache', data: cached });
      return;
    }

    // Fetch from Sabdasakha API
    const url = `${API_BASE}/dictionary/word?q=${encodeURIComponent(query)}`;
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), FETCH_TIMEOUT_MS);

    let response;
    try {
      response = await fetch(url, { signal: controller.signal });
    } finally {
      clearTimeout(timeout);
    }

    if (response.status === 404) {
      sendResponse({ type: 'LOOKUP_NOT_FOUND', data: { query } });
      return;
    }

    if (!response.ok) {
      sendResponse({
        type: 'LOOKUP_ERROR',
        error: `API returned ${response.status}`,
      });
      return;
    }

    const data = await response.json();
    const result = {
      query,
      word: data.word,
      partOfSpeech: data.part_of_speech,
      definitions: data.definitions || [],
      source: 'sabdasakha',
    };

    // Cache the result
    await setCachedEntry(query, result);

    sendResponse({ type: 'LOOKUP_RESULT', source: 'api', data: result });
  } catch (err) {
    if (err.name === 'AbortError') {
      sendResponse({ type: 'LOOKUP_ERROR', error: 'Request timed out' });
    } else {
      console.error('[varnavinyas] lookup error:', err);
      sendResponse({ type: 'LOOKUP_ERROR', error: err.message });
    }
  }
}

// ── Cache helpers ──

function cacheKey(word) {
  return `cache:v${CACHE_SCHEMA_VERSION}:${word}`;
}

async function getCachedEntry(word) {
  const key = cacheKey(word);
  return new Promise((resolve) => {
    chrome.storage.local.get(key, (result) => {
      const entry = result[key];
      if (!entry) return resolve(null);
      if (Date.now() - entry.timestamp > CACHE_TTL_MS) {
        chrome.storage.local.remove(key);
        return resolve(null);
      }
      resolve(entry.data);
    });
  });
}

async function setCachedEntry(word, data) {
  const key = cacheKey(word);
  return chrome.storage.local.set({
    [key]: { data, timestamp: Date.now(), schemaVersion: CACHE_SCHEMA_VERSION },
  });
}

function invalidateStaleCache() {
  chrome.storage.local.get(null, (items) => {
    const staleKeys = Object.keys(items).filter((k) => {
      if (!k.startsWith('cache:')) return false;
      const entry = items[k];
      return (
        !entry.schemaVersion ||
        entry.schemaVersion < CACHE_SCHEMA_VERSION ||
        Date.now() - entry.timestamp > CACHE_TTL_MS
      );
    });
    if (staleKeys.length > 0) {
      chrome.storage.local.remove(staleKeys);
    }
  });
}

// Export for future use
export { setCachedEntry };
