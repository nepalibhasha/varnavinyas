/**
 * Popup script — orchestrates WASM loading, selection reading,
 * local analysis, and dictionary lookup display.
 *
 * State machine: idle → loading → result | not-found | error
 */

import { ensureInit, analyzeWord, checkWord, decomposeWord, sandhiSplit, analyzeCompound, normalizeQuery } from './wasm-adapter.js';

// ── DOM refs ──

const wasmStatusEl = document.getElementById('wasm-status');
const footerEl = document.getElementById('footer');

const states = {
  idle: document.getElementById('state-idle'),
  loading: document.getElementById('state-loading'),
  result: document.getElementById('state-result'),
  notFound: document.getElementById('state-not-found'),
  error: document.getElementById('state-error'),
};

function showState(name) {
  for (const el of Object.values(states)) {
    el.classList.remove('active');
  }
  if (states[name]) {
    states[name].classList.add('active');
  }
}

// ── State ──

let wasmReady = false;
let currentWord = '';
let currentDefinitions = [];
let activeLookupToken = 0;
let lastPopupSelection = '';

// ── Theme ──

const THEME_KEY = 'themePreference';
const THEME_VALUES = new Set(['auto', 'light', 'dark']);
const darkModeQuery = window.matchMedia('(prefers-color-scheme: dark)');
let themePreference = 'auto';
const HISTORY_KEY = 'recentLookups';
const HISTORY_LIMIT = 10;
let recentLookups = [];

function normalizeThemePreference(value) {
  return THEME_VALUES.has(value) ? value : 'auto';
}

function applyThemePreference(pref) {
  const root = document.documentElement;
  themePreference = normalizeThemePreference(pref);
  const effectiveTheme =
    themePreference === 'auto'
      ? (darkModeQuery.matches ? 'dark' : 'light')
      : themePreference;

  root.setAttribute('data-theme', themePreference);
  root.setAttribute('data-theme-effective', effectiveTheme);
  root.style.colorScheme = effectiveTheme;
}

async function initTheme() {
  const pref = await new Promise((resolve) => {
    chrome.storage.local.get(THEME_KEY, (result) => {
      if (chrome.runtime.lastError) {
        resolve('auto');
        return;
      }
      resolve(normalizeThemePreference(result[THEME_KEY]));
    });
  });
  applyThemePreference(pref);
}

chrome.storage.onChanged.addListener((changes, area) => {
  if (area !== 'local' || !changes[THEME_KEY]) return;
  const next = normalizeThemePreference(changes[THEME_KEY].newValue);
  applyThemePreference(next);
});

darkModeQuery.addEventListener('change', () => {
  if (themePreference === 'auto') {
    applyThemePreference('auto');
  }
});

async function startup() {
  await initTheme();
  await initHistory();
  try {
    const initMs = await ensureInit();
    wasmReady = true;
    wasmStatusEl.textContent = initMs > 0 ? `WASM ${initMs.toFixed(0)}ms` : 'WASM ready';
  } catch (err) {
    wasmStatusEl.textContent = 'WASM error';
    console.error('[varnavinyas] WASM init failed:', err);
  }

  // Read the latest selection from background
  chrome.runtime.sendMessage({ type: 'GET_SELECTION' }, (response) => {
    if (chrome.runtime.lastError) {
      // No response — content script may not have sent a selection yet
      showState('idle');
      return;
    }
    const text = response?.text;
    if (text) {
      processWord(text);
    } else {
      showState('idle');
    }
  });
}

// ── Analysis pipeline ──

async function processWord(text) {
  showState('loading');

  // Extract first word if multi-word selection
  const word = text.split(/\s+/)[0];
  if (!word) {
    showState('idle');
    return;
  }

  // Sync search input
  searchInput.value = word;
  const lookupToken = ++activeLookupToken;
  addToHistory(word);

  try {
    // Local WASM analysis
    const t0 = performance.now();
    const analysis = wasmReady ? analyzeWord(word) : null;
    const check = wasmReady ? checkWord(word) : null;
    const decomposition = wasmReady ? decomposeWord(word) : null;
    const splits = wasmReady ? sandhiSplit(word) : [];
    const compounds = wasmReady ? analyzeCompound(word) : [];
    const elapsed = performance.now() - t0;

    if (analysis && !analysis.error) {
      renderResult(word, analysis, check, decomposition, splits, compounds);
      footerEl.textContent = `${elapsed.toFixed(1)}ms`;
    } else {
      // WASM not available or error — show word with minimal info
      renderNotFound(word);
    }

    // Request dictionary lookup from background (async, updates UI when ready)
    const normalized = wasmReady ? normalizeQuery(word) : word;
    chrome.runtime.sendMessage(
      { type: 'LOOKUP_REQUEST', payload: { query: normalized } },
      (response) => {
        if (chrome.runtime.lastError) return;
        if (lookupToken !== activeLookupToken) return;
        if (response?.type === 'LOOKUP_RESULT') {
          renderDictionary(response.data);
        } else if (response?.type === 'LOOKUP_NOT_FOUND') {
          renderDictionary({ definitions: [] });
        } else if (response?.type === 'LOOKUP_ERROR') {
          renderDictionary({ error: response.error });
        }
      }
    );
  } catch (err) {
    renderError(err.message);
  }
}

// ── Renderers ──

function renderResult(word, analysis, check, decomposition, splits, compounds) {
  showState('result');

  currentWord = analysis.word || word;
  currentDefinitions = [];
  document.getElementById('result-word').textContent = currentWord;

  // Set open link
  const openBtn = document.getElementById('btn-open');
  openBtn.href = `https://sabdasakha.com/word/${encodeURIComponent(currentWord)}`;
  document.getElementById('actions').style.display = 'flex';

  // Origin badge
  const originEl = document.getElementById('result-origin');
  const origin = (analysis.origin || 'unknown').toLowerCase();
  originEl.textContent = analysis.origin || '';
  originEl.className = `origin-badge ${origin}`;

  // Correction
  const correctionEl = document.getElementById('result-correction');
  if (!analysis.is_correct && analysis.correction) {
    correctionEl.style.display = 'flex';
    document.getElementById('correction-wrong').textContent = word;
    document.getElementById('correction-right').textContent = analysis.correction;
  } else if (check && check.suggestion) {
    correctionEl.style.display = 'flex';
    document.getElementById('correction-wrong').textContent = word;
    document.getElementById('correction-right').textContent = check.suggestion;
  } else {
    correctionEl.style.display = 'none';
  }

  // Rules
  const rulesEl = document.getElementById('result-rules');
  rulesEl.innerHTML = '';
  if (analysis.rule_notes && analysis.rule_notes.length > 0) {
    for (const note of analysis.rule_notes) {
      const li = document.createElement('li');
      const tag = document.createElement('span');
      tag.className = 'rule-tag';
      tag.textContent = note.rule;
      li.appendChild(tag);
      li.appendChild(document.createTextNode(` ${note.explanation}`));
      rulesEl.appendChild(li);
    }
  }

  // Morphology (prefix/root/suffix)
  const morphSection = document.getElementById('morph-section');
  const morphContent = document.getElementById('morph-content');
  morphContent.innerHTML = '';
  const hasPrefixes = decomposition && decomposition.prefixes && decomposition.prefixes.length > 0;
  const hasSuffixes = decomposition && decomposition.suffixes && decomposition.suffixes.length > 0;
  if (hasPrefixes || hasSuffixes) {
    morphSection.style.display = 'block';
    const row = document.createElement('div');
    row.className = 'morph-row';
    if (hasPrefixes) {
      for (const p of decomposition.prefixes) {
        const tag = document.createElement('span');
        tag.className = 'morph-prefix';
        tag.textContent = p;
        tag.title = 'उपसर्ग';
        row.appendChild(tag);
      }
      row.appendChild(document.createTextNode(' + '));
    }
    const rootSpan = document.createElement('span');
    rootSpan.className = 'morph-root';
    rootSpan.textContent = decomposition.root;
    rootSpan.title = 'मूल शब्द';
    row.appendChild(rootSpan);
    if (hasSuffixes) {
      row.appendChild(document.createTextNode(' + '));
      for (const s of decomposition.suffixes) {
        const tag = document.createElement('span');
        tag.className = 'morph-suffix';
        tag.textContent = s;
        tag.title = 'प्रत्यय';
        row.appendChild(tag);
      }
    }
    morphContent.appendChild(row);
  } else {
    morphSection.style.display = 'none';
  }

  // Compound (samasa) + sandhi splits — combined section
  const splitSection = document.getElementById('split-section');
  const splitContent = document.getElementById('split-content');
  splitContent.innerHTML = '';
  const allCompounds = Array.isArray(compounds) ? compounds : [];
  const isUnknownSamasa = (samasaType) => {
    const t = String(samasaType || '').trim().toLowerCase();
    return t === 'अज्ञात' || t === 'unknown';
  };
  const knownCompounds = allCompounds.filter((c) => !isUnknownSamasa(c.samasa_type));
  const unknownCompounds = allCompounds.filter((c) => isUnknownSamasa(c.samasa_type));
  const hasCompounds = knownCompounds.length > 0;
  const hasSandhi = splits && splits.length > 0;
  const showUnknownFallback = !hasCompounds && !hasSandhi && unknownCompounds.length > 0;
  if (hasCompounds || hasSandhi || showUnknownFallback) {
    splitSection.style.display = 'block';
    // Compound candidates (top 2)
    if (hasCompounds) {
      const top = knownCompounds.slice(0, 2);
      for (const c of top) {
        const li = document.createElement('li');
        const parts = document.createElement('span');
        parts.className = 'sandhi-parts';
        parts.textContent = `${c.left} + ${c.right}`;
        li.appendChild(parts);
        const type = document.createElement('span');
        type.className = 'sandhi-type';
        type.textContent = c.samasa_type;
        li.appendChild(type);
        if (c.vigraha) {
          const vig = document.createElement('span');
          vig.className = 'split-vigraha';
          vig.textContent = c.vigraha;
          li.appendChild(vig);
        }
        splitContent.appendChild(li);
      }
    }
    if (showUnknownFallback) {
      const c = unknownCompounds[0];
      const li = document.createElement('li');
      li.className = 'split-tentative';
      const marker = document.createElement('span');
      marker.className = 'tentative-marker';
      marker.textContent = '?';
      marker.title = 'अनुमानित परिणाम';
      marker.setAttribute('aria-label', 'अनुमानित परिणाम');
      li.appendChild(marker);
      const parts = document.createElement('span');
      parts.className = 'sandhi-parts';
      parts.textContent = `${c.left} + ${c.right}`;
      li.appendChild(parts);
      const type = document.createElement('span');
      type.className = 'sandhi-type';
      type.textContent = 'अनुमानित';
      li.appendChild(type);
      if (c.vigraha) {
        const vig = document.createElement('span');
        vig.className = 'split-vigraha';
        vig.textContent = c.vigraha;
        li.appendChild(vig);
      }
      splitContent.appendChild(li);
    }
    // Sandhi splits (that aren't duplicates of compound candidates)
    if (hasSandhi) {
      const compoundKeys = new Set(
        knownCompounds.map((c) => `${c.left}|${c.right}`)
      );
      for (const s of splits) {
        if (compoundKeys.has(`${s.left}|${s.right}`)) continue;
        const li = document.createElement('li');
        const parts = document.createElement('span');
        parts.className = 'sandhi-parts';
        parts.textContent = `${s.left} + ${s.right}`;
        li.appendChild(parts);
        if (s.sandhi_type) {
          const type = document.createElement('span');
          type.className = 'sandhi-type';
          type.textContent = s.sandhi_type;
          li.appendChild(type);
        }
        splitContent.appendChild(li);
      }
    }
  } else {
    splitSection.style.display = 'none';
  }

  // Reset dictionary section to stub
  document.getElementById('dict-content').innerHTML =
    '<span class="dict-stub">शब्दकोश खोज्दै…</span>';
}

function renderDictionary(data) {
  const el = document.getElementById('dict-content');
  el.innerHTML = '';

  if (data.error) {
    el.innerHTML = `<span class="dict-stub">${data.error}</span>`;
    return;
  }

  if (!data.definitions || data.definitions.length === 0) {
    el.innerHTML = '<span class="dict-stub">शब्दकोशमा भेटिएन</span>';
    currentDefinitions = [];
    return;
  }

  currentDefinitions = data.definitions;

  // Part of speech
  if (data.partOfSpeech) {
    const pos = document.createElement('div');
    pos.className = 'dict-pos';
    pos.textContent = data.partOfSpeech;
    el.appendChild(pos);
  }

  // Definitions
  const ol = document.createElement('ol');
  ol.className = 'dict-defs';
  for (const def of data.definitions) {
    const li = document.createElement('li');
    li.textContent = typeof def === 'string' ? def : def.text;
    ol.appendChild(li);
  }
  el.appendChild(ol);

  // Source attribution
  if (data.source === 'sabdasakha') {
    const src = document.createElement('div');
    src.className = 'dict-source';
    src.textContent = 'शब्दसखा';
    el.appendChild(src);
  }
}

function renderNotFound(word) {
  showState('notFound');
  currentWord = word;
  currentDefinitions = [];
  document.getElementById('notfound-word').textContent = word;
}

function renderError(message) {
  showState('error');
  currentDefinitions = [];
  document.getElementById('error-message').textContent = message;
}

// ── Popup-local selection lookup ──

function readPopupSelectedText() {
  const sel = window.getSelection();
  if (!sel || sel.isCollapsed) return '';
  const active = document.activeElement;
  if (active && (active.tagName === 'INPUT' || active.tagName === 'TEXTAREA')) {
    return '';
  }
  const text = sel.toString().trim();
  if (!text) return '';
  // Strip non-Devanagari edges (quotes/punctuation) for cleaner lookup.
  return text.replace(/^[^\u0900-\u097F]+|[^\u0900-\u097F]+$/g, '');
}

function handlePopupSelectionLookup() {
  const text = readPopupSelectedText();
  if (!text || text === lastPopupSelection) return;
  if (!/[\u0900-\u097F]/.test(text)) return;
  lastPopupSelection = text;
  processWord(text);
}

// ── Actions ──

const copyBtn = document.getElementById('btn-copy');
const copyLabel = (text) => {
  // Preserve the SVG icon, only update the text node
  const svg = copyBtn.querySelector('svg');
  copyBtn.textContent = '';
  if (svg) copyBtn.appendChild(svg);
  copyBtn.appendChild(document.createTextNode(` ${text}`));
};

copyBtn.addEventListener('click', async () => {
  const lines = [];
  if (currentWord) lines.push(currentWord);
  for (const def of currentDefinitions) {
    const text = typeof def === 'string' ? def : def.text;
    lines.push(text);
  }
  if (lines.length === 0) return;

  try {
    await navigator.clipboard.writeText(lines.join('\n'));
    copyLabel('कपी भयो!');
    copyBtn.classList.add('copied');
    setTimeout(() => {
      copyLabel('कपी');
      copyBtn.classList.remove('copied');
    }, 1500);
  } catch (err) {
    console.error('[varnavinyas] copy failed:', err);
    copyLabel('कपी असफल');
    setTimeout(() => {
      copyLabel('कपी');
    }, 1500);
  }
});

// ── Manual search ──

const searchInput = document.getElementById('search-input');
const searchBtn = document.getElementById('search-btn');
const historyRow = document.getElementById('history-row');
const historyListEl = document.getElementById('history-list');
const historyClearBtn = document.getElementById('history-clear');

async function initHistory() {
  recentLookups = await new Promise((resolve) => {
    chrome.storage.local.get(HISTORY_KEY, (result) => {
      if (chrome.runtime.lastError) {
        resolve([]);
        return;
      }
      const list = Array.isArray(result[HISTORY_KEY]) ? result[HISTORY_KEY] : [];
      resolve(list);
    });
  });
  renderHistory();
}

function isValidHistoryWord(word) {
  return Boolean(word) && word.length >= 2 && /[\u0900-\u097F]/.test(word);
}

function renderHistory() {
  if (!historyRow || !historyListEl) return;
  historyListEl.innerHTML = '';
  if (!recentLookups.length) {
    historyRow.style.display = 'none';
    return;
  }

  historyRow.style.display = 'block';
  for (const entry of recentLookups) {
    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'history-chip';
    btn.textContent = entry.word;
    btn.addEventListener('click', () => processWord(entry.word));
    historyListEl.appendChild(btn);
  }
}

function persistHistory() {
  chrome.storage.local.set({ [HISTORY_KEY]: recentLookups });
}

function addToHistory(word) {
  if (!isValidHistoryWord(word)) return;
  const now = Date.now();
  recentLookups = recentLookups
    .filter((entry) => entry.word !== word)
    .slice(0, HISTORY_LIMIT - 1);
  recentLookups.unshift({ word, ts: now });
  renderHistory();
  persistHistory();
}

function clearHistory() {
  recentLookups = [];
  renderHistory();
  persistHistory();
}

function doSearch() {
  const word = searchInput.value.trim();
  if (word) processWord(word);
}

searchBtn.addEventListener('click', doSearch);
searchInput.addEventListener('keydown', (e) => {
  if (e.key === 'Enter') doSearch();
});
historyClearBtn?.addEventListener('click', clearHistory);

document.addEventListener('mouseup', handlePopupSelectionLookup);
document.addEventListener('keyup', (e) => {
  if (e.key === 'Shift' || e.key.startsWith('Arrow')) {
    handlePopupSelectionLookup();
  }
});

// ── Live updates (side panel mode) ──

chrome.runtime.onMessage.addListener((message) => {
  if (message.type === 'SELECTION_CHANGED' && message.payload?.text) {
    processWord(message.payload.text);
  }
});

// ── Go ──

startup();
