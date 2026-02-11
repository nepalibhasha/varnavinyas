/**
 * Debounce a function by the given delay in milliseconds.
 */
export function debounce(fn, delay) {
  let timer;
  return (...args) => {
    clearTimeout(timer);
    timer = setTimeout(() => fn(...args), delay);
  };
}

/**
 * Escape HTML special characters to prevent XSS.
 */
export function escapeHtml(str) {
  const div = document.createElement('div');
  div.textContent = str;
  return div.innerHTML;
}

/**
 * Map stable category_code values (Rust enum variant names) to CSS custom properties.
 */
export const CATEGORY_COLORS = {
  ShuddhaTable: 'var(--cat-shuddha-table)',
  HrasvaDirgha: 'var(--cat-hrasva-dirgha)',
  Chandrabindu: 'var(--cat-chandrabindu)',
  ShaShaS: 'var(--cat-sha-sha-s)',
  RiKri: 'var(--cat-ri-kri)',
  Halanta: 'var(--cat-halanta)',
  YaE: 'var(--cat-ya-e)',
  KshaChhya: 'var(--cat-ksha-chhya)',
  Sandhi: 'var(--cat-sandhi)',
  Punctuation: 'var(--cat-punctuation)',
};

/**
 * Human-readable origin labels keyed by origin code.
 */
export const ORIGIN_LABELS = {
  tatsam: 'तत्सम',
  tadbhav: 'तद्भव',
  deshaj: 'देशज',
  aagantuk: 'आगन्तुक',
};

/**
 * Human-readable category labels keyed by category_code.
 */
export const CATEGORY_LABELS = {
  ShuddhaTable: 'शुद्ध/अशुद्ध',
  HrasvaDirgha: 'ह्रस्व/दीर्घ',
  Chandrabindu: 'चन्द्रबिन्दु',
  ShaShaS: 'श/ष/स',
  RiKri: 'ऋ/कृ',
  Halanta: 'हलन्त',
  YaE: 'य/ए',
  KshaChhya: 'क्ष/छ',
  Sandhi: 'सन्धि',
  Punctuation: 'चिह्न',
};
