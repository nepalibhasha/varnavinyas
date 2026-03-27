use varnavinyas_kosha::Kosha;
use varnavinyas_kosha::kosha;
use varnavinyas_prakriya::is_in_correction_table;
use varnavinyas_shabda::{best_analysis, has_supported_analysis};

/// A token extracted from text.
#[derive(Debug, Clone)]
pub struct Token {
    /// The word text (without surrounding punctuation).
    pub text: String,
    /// Byte offset of the start of this token in the original text.
    pub start: usize,
    /// Byte offset of the end of this token in the original text.
    pub end: usize,
}

/// A token with suffix analysis — the stem and optional detached suffix.
#[derive(Debug, Clone)]
pub struct AnalyzedToken {
    /// The stem (after suffix detachment, or the full word if no suffix matched).
    pub stem: String,
    /// The detached suffix, if any.
    pub suffix: Option<String>,
    /// Byte offset of the start of the full token (stem+suffix) in the original text.
    pub start: usize,
    /// Byte offset of the end of the full token (stem+suffix) in the original text.
    pub end: usize,
}

pub(crate) fn is_supported_stem(stem: &str, lex: &Kosha) -> bool {
    if stem.is_empty() {
        return false;
    }

    if lex.contains(stem) || lex.lookup(stem).is_some() || is_in_correction_table(stem) {
        return true;
    }

    has_supported_analysis(stem)
}

struct SupportSuffixGroup {
    items: &'static [&'static str],
    repeatable: bool,
}

const SUPPORT_SUFFIX_GROUPS: &[SupportSuffixGroup] = &[
    SupportSuffixGroup {
        items: varnavinyas_shabda::tables::PARTICLES,
        repeatable: false,
    },
    SupportSuffixGroup {
        items: varnavinyas_shabda::tables::CASE_MARKERS,
        repeatable: true,
    },
    SupportSuffixGroup {
        items: varnavinyas_shabda::tables::PLURAL_MARKERS,
        repeatable: false,
    },
];

fn collect_detachments(
    current: &str,
    detached: &mut Vec<&'static str>,
    group_index: usize,
    best: &mut Option<(String, String)>,
    lex: &Kosha,
    require_supported_stem: bool,
) {
    if !detached.is_empty() && (!require_supported_stem || is_supported_stem(current, lex)) {
        let suffix = detached.iter().rev().copied().collect::<String>();
        let candidate = (current.to_string(), suffix);
        let replace = best
            .as_ref()
            .is_none_or(|(_, best_suffix)| candidate.1.len() > best_suffix.len());
        if replace {
            *best = Some(candidate);
        }
    }

    if group_index >= SUPPORT_SUFFIX_GROUPS.len() {
        return;
    }

    let group = &SUPPORT_SUFFIX_GROUPS[group_index];
    for &sfx in group.items {
        if let Some(rest) = current.strip_suffix(sfx) {
            if rest.is_empty() {
                continue;
            }
            detached.push(sfx);
            let next_group = if group.repeatable {
                group_index
            } else {
                group_index + 1
            };
            collect_detachments(
                rest,
                detached,
                next_group,
                best,
                lex,
                require_supported_stem,
            );
            collect_detachments(
                rest,
                detached,
                group_index + 1,
                best,
                lex,
                require_supported_stem,
            );
            detached.pop();
        }
    }

    collect_detachments(
        current,
        detached,
        group_index + 1,
        best,
        lex,
        require_supported_stem,
    );
}

pub(crate) fn best_supported_detachment(word: &str, lex: &Kosha) -> Option<(String, String)> {
    let mut best = None;
    collect_detachments(word, &mut Vec::new(), 0, &mut best, lex, true);
    best
}

pub(crate) fn best_detachment_candidate(word: &str, lex: &Kosha) -> Option<(String, String)> {
    let mut best = None;
    collect_detachments(word, &mut Vec::new(), 0, &mut best, lex, false);
    best
}

/// Vocative case markers. Only active behind `vocative-tokenization` feature.
#[cfg(feature = "vocative-tokenization")]
const VOCATIVE_SUFFIXES: &[&str] = &["ए", "ओ"];

/// Discourse particles (nipats). Only active behind `nipat-tokenization` feature.
/// Sorted longest-first. Single-char nipats (त, ल, नि) are risky — extra guard applied.
#[cfg(feature = "nipat-tokenization")]
const NIPATS: &[&str] = &["क्यारे", "नै", "पो", "रे", "खै", "नि", "ल", "त"];

/// Tokenize text into word tokens with byte offsets.
///
/// Splits on whitespace and punctuation boundaries.
/// Only returns tokens that contain at least one Devanagari character.
pub fn tokenize(text: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut start = None;

    for (idx, ch) in text.char_indices() {
        if ch.is_whitespace() || is_punctuation(ch) {
            if let Some(word_start) = start.take() {
                let word = &text[word_start..idx];
                if has_devanagari(word) {
                    tokens.push(Token {
                        text: word.to_string(),
                        start: word_start,
                        end: idx,
                    });
                }
            }
        } else if start.is_none() {
            start = Some(idx);
        }
    }

    if let Some(word_start) = start {
        let word = &text[word_start..];
        if has_devanagari(word) {
            tokens.push(Token {
                text: word.to_string(),
                start: word_start,
                end: text.len(),
            });
        }
    }

    tokens
}

/// Tokenize text into analyzed tokens with outer affix detachment.
///
/// For each whitespace-delimited token, uses the shared affix analyzer to detach
/// a conservative outer suffix/particle stack. If no valid split is found, the
/// full word becomes the stem with `suffix: None`.
pub fn tokenize_analyzed(text: &str) -> Vec<AnalyzedToken> {
    let tokens = tokenize(text);
    let lex = kosha();

    tokens
        .into_iter()
        .map(|tok| {
            if let Some(analysis) = best_analysis(&tok.text) {
                if !analysis.suffixes.is_empty() {
                    let detached = tok
                        .text
                        .strip_prefix(&analysis.stem)
                        .unwrap_or_default()
                        .to_string();
                    if !detached.is_empty() {
                        return AnalyzedToken {
                            stem: analysis.stem,
                            suffix: Some(detached),
                            start: tok.start,
                            end: tok.end,
                        };
                    }
                }
            }
            if !lex.contains(&tok.text)
                && lex.lookup(&tok.text).is_none()
                && !is_in_correction_table(&tok.text)
            {
                if let Some((stem, detached)) = best_supported_detachment(&tok.text, lex) {
                    return AnalyzedToken {
                        stem,
                        suffix: Some(detached),
                        start: tok.start,
                        end: tok.end,
                    };
                }
            }
            // Oblique form: stem ends in ा (oblique) but dictionary has ो form
            // e.g., "केटालाई" → stem "केटा", but kosha has "केटो"
            #[cfg(feature = "oblique-forms")]
            for sfx in varnavinyas_shabda::tables::CASE_MARKERS
                .iter()
                .chain(varnavinyas_shabda::tables::PLURAL_MARKERS.iter())
            {
                if let Some(stem) = tok.text.strip_suffix(sfx) {
                    if !stem.is_empty() {
                        if let Some(base) = stem.strip_suffix('ा') {
                            let candidate = format!("{base}ो");
                            if lex.contains(&candidate) {
                                return AnalyzedToken {
                                    stem: stem.to_string(),
                                    suffix: Some((*sfx).to_string()),
                                    start: tok.start,
                                    end: tok.end,
                                };
                            }
                        }
                    }
                }
            }
            // Vocative markers: single-char ए/ओ with triple guard
            #[cfg(feature = "vocative-tokenization")]
            for voc in VOCATIVE_SUFFIXES {
                if let Some(stem) = tok.text.strip_suffix(voc) {
                    // Guard 1: stem exists in kosha
                    // Guard 2: full word is NOT in kosha (avoid splitting real words)
                    // Guard 3: stem must end in vowel/matra (vocative attaches to vowel stems)
                    if !stem.is_empty()
                        && lex.contains(stem)
                        && !lex.contains(&tok.text)
                        && stem.chars().last().is_some_and(|c| {
                            varnavinyas_akshar::is_svar(c) || varnavinyas_akshar::is_matra(c)
                        })
                    {
                        return AnalyzedToken {
                            stem: stem.to_string(),
                            suffix: Some(voc.to_string()),
                            start: tok.start,
                            end: tok.end,
                        };
                    }
                }
            }
            // Nipat (discourse particle) detachment with triple guard
            #[cfg(feature = "nipat-tokenization")]
            for nip in NIPATS {
                if let Some(stem) = tok.text.strip_suffix(nip) {
                    // Guard 1: stem exists in kosha
                    // Guard 2: full word is NOT in kosha
                    // Guard 3: risky short nipats (≤2 chars, e.g. "नि", "त", "ल") require stem to end in vowel/matra
                    let is_risky = nip.chars().count() <= 2;
                    let vowel_ending = stem.chars().last().is_some_and(|c| {
                        varnavinyas_akshar::is_svar(c) || varnavinyas_akshar::is_matra(c)
                    });
                    if !stem.is_empty()
                        && lex.contains(stem)
                        && !lex.contains(&tok.text)
                        && (!is_risky || vowel_ending)
                    {
                        return AnalyzedToken {
                            stem: stem.to_string(),
                            suffix: Some(nip.to_string()),
                            start: tok.start,
                            end: tok.end,
                        };
                    }
                }
            }
            AnalyzedToken {
                stem: tok.text,
                suffix: None,
                start: tok.start,
                end: tok.end,
            }
        })
        .collect()
}

/// Check if a character is punctuation (for tokenization purposes).
fn is_punctuation(c: char) -> bool {
    matches!(
        c,
        '.' | ','
            | '!'
            | '?'
            | ';'
            | ':'
            | '-'
            | '('
            | ')'
            | '['
            | ']'
            | '{'
            | '}'
            | '"'
            | '\''
            | '/'
            | '–'
            | '—'
            | '।'
            | '…'
            | '“'
            | '”'
            | '‘'
            | '’'
    )
}

/// Check if a string contains any Devanagari character.
fn has_devanagari(s: &str) -> bool {
    s.chars().any(|c| ('\u{0900}'..='\u{097F}').contains(&c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_tokenization() {
        let tokens = tokenize("नेपाल राम्रो देश हो।");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].text, "नेपाल");
        assert_eq!(tokens[3].text, "हो");
    }

    #[test]
    fn strips_trailing_danda() {
        let tokens = tokenize("देश हो।");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[1].text, "हो");
    }

    #[test]
    fn strips_smart_quotes() {
        let tokens = tokenize("“अत्याधिक”");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].text, "अत्याधिक");
    }

    #[test]
    fn splits_on_internal_punctuation_boundaries() {
        let tokens = tokenize("नेकपा (माओवादी केन्द्र)को");
        let words: Vec<_> = tokens.iter().map(|t| t.text.as_str()).collect();
        assert_eq!(words, vec!["नेकपा", "माओवादी", "केन्द्र", "को"]);
    }

    #[test]
    fn splits_on_unicode_dash_boundaries() {
        let tokens = tokenize("राम भने– ‘घर जाऔँ।’");
        let words: Vec<_> = tokens.iter().map(|t| t.text.as_str()).collect();
        assert_eq!(words, vec!["राम", "भने", "घर", "जाऔँ"]);
    }

    #[test]
    fn empty_input() {
        assert!(tokenize("").is_empty());
        assert!(tokenize("   ").is_empty());
    }

    #[test]
    fn skips_english_tokens() {
        let tokens = tokenize("hello नेपाल world");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].text, "नेपाल");
    }

    #[test]
    fn preserves_byte_offsets() {
        let text = "नेपाल राम्रो";
        let tokens = tokenize(text);
        assert_eq!(tokens.len(), 2);
        assert_eq!(&text[tokens[0].start..tokens[0].end], "नेपाल");
        assert_eq!(&text[tokens[1].start..tokens[1].end], "राम्रो");
    }

    // --- O8 acceptance tests: suffix-aware tokenizer ---

    /// O8.1: "रामलाई" → stem "राम", suffix "लाई"
    #[test]
    fn o8_1_detach_laai() {
        let tokens = tokenize_analyzed("रामलाई");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].stem, "राम");
        assert_eq!(tokens[0].suffix.as_deref(), Some("लाई"));
    }

    /// O8.2: "घरहरु" → stem "घर", suffix "हरु"
    #[test]
    fn o8_2_detach_haru() {
        let tokens = tokenize_analyzed("घरहरु");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].stem, "घर");
        assert_eq!(tokens[0].suffix.as_deref(), Some("हरु"));
    }

    /// O8.3: "नेपालमा" → stem "नेपाल", suffix "मा"
    #[test]
    fn o8_3_detach_maa() {
        let tokens = tokenize_analyzed("नेपालमा");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].stem, "नेपाल");
        assert_eq!(tokens[0].suffix.as_deref(), Some("मा"));
    }

    /// O8.4: Unknown stem keeps original token unsplit.
    #[test]
    fn o8_4_unknown_stem_unsplit() {
        let tokens = tokenize_analyzed("ज्ञपतमा");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].stem, "ज्ञपतमा");
        assert_eq!(tokens[0].suffix, None);
    }

    /// O8.5: Longest suffix wins — "घरभित्र" matches "भित्र" (15 bytes), not shorter.
    #[test]
    fn o8_5_longest_suffix_wins() {
        let tokens = tokenize_analyzed("घरभित्र");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].stem, "घर");
        assert_eq!(tokens[0].suffix.as_deref(), Some("भित्र"));
    }

    /// O8.6: tokenize() still returns Vec<Token> unchanged.
    #[test]
    fn o8_6_tokenize_unchanged() {
        let tokens = tokenize("रामलाई नेपालमा");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].text, "रामलाई");
        assert_eq!(tokens[1].text, "नेपालमा");
    }

    /// O8.7: tokenize_analyzed() compiles and returns Vec<AnalyzedToken>.
    #[test]
    fn o8_7_returns_analyzed_tokens() {
        let tokens: Vec<AnalyzedToken> = tokenize_analyzed("राम नेपाल");
        assert_eq!(tokens.len(), 2);
    }

    /// O8.8: Byte offsets cover the full original unsplit form.
    #[test]
    fn o8_8_byte_offsets_cover_full_token() {
        let text = "रामलाई नेपालमा";
        let tokens = tokenize_analyzed(text);
        assert_eq!(tokens.len(), 2);
        assert_eq!(&text[tokens[0].start..tokens[0].end], "रामलाई");
        assert_eq!(&text[tokens[1].start..tokens[1].end], "नेपालमा");
    }

    #[test]
    fn detaches_suffix_for_stem_supported_by_attested_sibling_form() {
        let tokens = tokenize_analyzed("मच्छिन्द्रनाथको");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].stem, "मच्छिन्द्रनाथ");
        assert_eq!(tokens[0].suffix.as_deref(), Some("को"));
    }

    #[test]
    fn detaches_stacked_suffixes_via_shared_analysis() {
        let tokens = tokenize_analyzed("रामकोपनि");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].stem, "राम");
        assert_eq!(tokens[0].suffix.as_deref(), Some("कोपनि"));
    }

    #[test]
    fn preserves_prefix_in_stem_when_detaching_case_suffix() {
        let tokens = tokenize_analyzed("निराशाबाट");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].stem, "निराशा");
        assert_eq!(tokens[0].suffix.as_deref(), Some("बाट"));
    }
}
