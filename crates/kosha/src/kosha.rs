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

        Kosha { fst, headwords }
    }

    /// Check if a word exists in the lexicon.
    pub fn contains(&self, word: &str) -> bool {
        self.fst.contains(word)
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
}
