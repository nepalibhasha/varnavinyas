#[cfg(any(test, feature = "test-seam"))]
use std::cell::RefCell;
use std::sync::LazyLock;

use fst::Set;

use crate::builder::build_fst_set;
use crate::origin_tag::{OriginTag, parse_origin_tag, parse_source_language};

/// Static word list (one word per line, byte-sorted).
static WORDS_DATA: &str = include_str!("../../../data/words.txt");

/// Static headword metadata (tab-separated: word \t pos_tags).
static HEADWORDS_DATA: &str = include_str!("../../../data/headwords.tsv");

/// Global singleton lexicon, built once on first access.
static KOSHA: LazyLock<Kosha> =
    LazyLock::new(|| Kosha::from_static_data(WORDS_DATA, HEADWORDS_DATA));

#[cfg(any(test, feature = "test-seam"))]
thread_local! {
    static TEST_KOSHA_OVERRIDE: RefCell<Option<&'static Kosha>> = const { RefCell::new(None) };
}

/// A metadata entry for a headword.
#[derive(Debug, Clone)]
pub struct WordEntry {
    /// The headword.
    pub word: &'static str,
    /// Part-of-speech tags (e.g., "[सं.] ना.", "वि.").
    pub pos: &'static str,
}

/// FST-based Nepali lexicon.
///
/// Uses an `fst::Set` for fast `contains()` checks over ~109K word forms,
/// and a sorted `Vec<WordEntry>` for headword metadata lookups.
pub struct Kosha {
    /// FST set for O(1) word existence checks.
    fst: Set<Vec<u8>>,
    /// Sorted full-word forms for nearby suggestion heuristics.
    words: Vec<&'static str>,
    /// Sorted headword entries for binary-search metadata lookup.
    headwords: Vec<WordEntry>,
}

impl Kosha {
    /// Build from the static embedded data files.
    fn from_static_data(words_data: &'static str, headwords_data: &'static str) -> Self {
        // Parse word list for FST
        let words: Vec<&str> = words_data.lines().filter(|l| !l.is_empty()).collect();
        let fst_bytes = build_fst_set(&words);
        let fst = Set::new(fst_bytes).expect("FST should be valid");

        // Parse headword metadata
        let mut headwords: Vec<WordEntry> = headwords_data
            .lines()
            .filter_map(|line| {
                let mut parts = line.splitn(2, '\t');
                let word = parts.next()?.trim();
                if word.is_empty() {
                    return None;
                }
                let pos = parts.next().unwrap_or("").trim();
                Some(WordEntry { word, pos })
            })
            .collect();
        headwords.sort_by(|a, b| a.word.as_bytes().cmp(b.word.as_bytes()));

        Kosha {
            fst,
            words,
            headwords,
        }
    }

    /// Check if a word exists in the lexicon.
    pub fn contains(&self, word: &str) -> bool {
        self.fst.contains(word)
    }

    /// Find one near-match candidate by character-level edit distance.
    ///
    /// This searches a bounded lexicographic window around the insertion point,
    /// avoiding a full-lexicon scan while keeping Unicode-aware matching.
    pub fn suggest_nearby(&self, word: &str, max_distance: usize) -> Option<String> {
        if word.is_empty() {
            return None;
        }

        let idx = self
            .words
            .binary_search_by(|w| w.as_bytes().cmp(word.as_bytes()))
            .unwrap_or_else(|i| i);
        const WINDOW: usize = 256;
        let start = idx.saturating_sub(WINDOW);
        let end = (idx + WINDOW).min(self.words.len());

        let mut best: Option<(&str, usize)> = None;
        for candidate in &self.words[start..end] {
            let clen = candidate.chars().count();
            let wlen = word.chars().count();
            if clen.abs_diff(wlen) > max_distance {
                continue;
            }

            if let Some(dist) = bounded_levenshtein_chars(word, candidate, max_distance) {
                match best {
                    None => best = Some((candidate, dist)),
                    Some((best_word, best_dist)) => {
                        if dist < best_dist || (dist == best_dist && candidate < &best_word) {
                            best = Some((candidate, dist));
                        }
                    }
                }
            }
        }

        best.map(|(w, _)| w.to_string())
    }

    /// Look up headword metadata (POS tags).
    /// Returns `None` if the word is not a known headword.
    pub fn lookup(&self, word: &str) -> Option<&WordEntry> {
        self.headwords
            .binary_search_by(|entry| entry.word.as_bytes().cmp(word.as_bytes()))
            .ok()
            .map(|idx| &self.headwords[idx])
    }

    /// Number of word forms in the FST.
    pub fn word_count(&self) -> usize {
        self.fst.len()
    }

    /// Number of headwords with metadata.
    pub fn headword_count(&self) -> usize {
        self.headwords.len()
    }

    /// Look up a word's origin from its dictionary metadata tags.
    ///
    /// Parses the `[सं.]`, `[फा.]`, `[अङ्.]` etc. tags from the headword's
    /// POS field using the Nepali Brihat Shabdakosha abbreviation legend.
    /// Returns `None` if the word is not a headword or has no origin tag.
    pub fn origin_of(&self, word: &str) -> Option<OriginTag> {
        let entry = self.lookup(word)?;
        parse_origin_tag(entry.pos)
    }

    /// Look up a word's source language from its dictionary metadata tags.
    ///
    /// Returns the human-readable language name (e.g., "फारसी", "अरबी", "संस्कृत").
    /// Returns `None` if the word is not a headword or has no recognized language tag.
    pub fn source_language_of(&self, word: &str) -> Option<&'static str> {
        let entry = self.lookup(word)?;
        parse_source_language(entry.pos)
    }
}

#[cfg(any(test, feature = "test-seam"))]
struct TestKoshaResetGuard {
    previous: Option<&'static Kosha>,
}

#[cfg(any(test, feature = "test-seam"))]
impl Drop for TestKoshaResetGuard {
    fn drop(&mut self) {
        TEST_KOSHA_OVERRIDE.with(|slot| {
            slot.replace(self.previous);
        });
    }
}

/// Run a closure with a scoped test lexicon override.
///
/// This is available only for tests (or with `test-seam` feature) and resets
/// automatically when the closure exits.
///
/// Note: this helper intentionally leaks the constructed `Kosha` instance via
/// `Box::leak` to satisfy `'static` storage in the thread-local override.
/// This is acceptable for short-lived test execution, but should not be used in
/// long-running loops in production-like processes.
#[cfg(any(test, feature = "test-seam"))]
pub fn with_test_kosha<R>(
    words_data: &'static str,
    headwords_data: &'static str,
    f: impl FnOnce() -> R,
) -> R {
    let custom = Box::leak(Box::new(Kosha::from_static_data(
        words_data,
        headwords_data,
    )));
    let previous = TEST_KOSHA_OVERRIDE.with(|slot| slot.replace(Some(custom)));
    let _reset = TestKoshaResetGuard { previous };
    f()
}

/// Get a reference to the global lexicon singleton.
pub fn kosha() -> &'static Kosha {
    #[cfg(any(test, feature = "test-seam"))]
    {
        if let Some(override_kosha) = TEST_KOSHA_OVERRIDE.with(|slot| *slot.borrow()) {
            return override_kosha;
        }
    }

    &KOSHA
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kosha_override_is_scoped() {
        const WORD: &str = "टेस्टशब्दनिश्चित";
        assert!(!kosha().contains(WORD));

        with_test_kosha(
            "टेस्टशब्दनिश्चित\n",
            "टेस्टशब्दनिश्चित\tना.\n",
            || {
                assert!(kosha().contains(WORD));
            },
        );

        assert!(!kosha().contains(WORD));
    }

    #[test]
    fn test_kosha_override_supports_nesting() {
        const WORD1: &str = "पहिलोनमूना";
        const WORD2: &str = "दोस्रोनमूना";

        with_test_kosha(
            "पहिलोनमूना\n",
            "पहिलोनमूना\tना.\n",
            || {
                assert!(kosha().contains(WORD1));
                assert!(!kosha().contains(WORD2));

                with_test_kosha(
                    "दोस्रोनमूना\n",
                    "दोस्रोनमूना\tना.\n",
                    || {
                        assert!(!kosha().contains(WORD1));
                        assert!(kosha().contains(WORD2));
                    },
                );

                assert!(kosha().contains(WORD1));
                assert!(!kosha().contains(WORD2));
            },
        );
    }

    #[test]
    fn test_suggest_nearby_returns_close_match() {
        with_test_kosha(
            "अध्ययन\nआकाश\n",
            "अध्ययन\tना.\nआकाश\tना.\n",
            || {
                let hit = kosha().suggest_nearby("अध्यन", 1);
                assert_eq!(hit.as_deref(), Some("अध्ययन"));
            },
        );
    }
}

fn bounded_levenshtein_chars(a: &str, b: &str, max_distance: usize) -> Option<usize> {
    if a == b {
        return Some(0);
    }
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    if a_chars.len().abs_diff(b_chars.len()) > max_distance {
        return None;
    }

    let mut prev: Vec<usize> = (0..=b_chars.len()).collect();
    let mut curr: Vec<usize> = vec![0; b_chars.len() + 1];

    for (i, &ac) in a_chars.iter().enumerate() {
        curr[0] = i + 1;
        let mut row_min = curr[0];
        for (j, &bc) in b_chars.iter().enumerate() {
            let cost = if ac == bc { 0 } else { 1 };
            curr[j + 1] = (prev[j + 1] + 1).min(curr[j] + 1).min(prev[j] + cost);
            row_min = row_min.min(curr[j + 1]);
        }
        if row_min > max_distance {
            return None;
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    let dist = prev[b_chars.len()];
    (dist <= max_distance).then_some(dist)
}
