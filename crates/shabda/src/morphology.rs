use crate::origin::{Origin, classify};
use crate::tables;
use std::collections::HashSet;
use varnavinyas_kosha::kosha;

fn normalize_suffix_label(sfx: &str) -> String {
    match sfx {
        "ि" => "इ".to_string(),
        "ी" => "ई".to_string(),
        "ु" => "उ".to_string(),
        "ू" => "ऊ".to_string(),
        _ => sfx.to_string(),
    }
}

fn candidate_score(
    original: &str,
    prefixes: &[String],
    suffixes: &[String],
    known_word: bool,
    known_headword: bool,
) -> u16 {
    let mut score = 0;
    if known_word {
        score += 100;
    }
    if known_headword {
        score += 150;
    }
    score += (prefixes.len() as u16) * 20;
    score += (suffixes.len() as u16) * 15;
    if !prefixes.is_empty() || !suffixes.is_empty() {
        score += 10;
    }
    score += original.chars().count() as u16;
    score
}

fn push_candidate(
    candidates: &mut Vec<RootCandidate>,
    seen: &mut HashSet<String>,
    original: &str,
    root: &str,
    prefixes: &[String],
    suffixes: &[String],
) {
    let lex = kosha();
    let known_word = lex.contains(root);
    let known_headword = lex.lookup(root).is_some();
    if !known_word && !known_headword {
        return;
    }

    let key = format!("{}|{}|{}", root, prefixes.join("+"), suffixes.join("+"));
    if !seen.insert(key) {
        return;
    }

    candidates.push(RootCandidate {
        root: root.to_string(),
        prefixes: prefixes.to_vec(),
        suffixes: suffixes.to_vec(),
        origin: classify(root),
        known_word,
        known_headword,
        score: candidate_score(original, prefixes, suffixes, known_word, known_headword),
    });
}

fn recurse_suffix_groups(
    original: &str,
    current: &str,
    prefixes: &[String],
    suffixes: &mut Vec<String>,
    group_index: usize,
    groups: &[SuffixGroup],
    candidates: &mut Vec<RootCandidate>,
    seen: &mut HashSet<String>,
) {
    if group_index >= groups.len() {
        return;
    }

    recurse_suffix_groups(
        original,
        current,
        prefixes,
        suffixes,
        group_index + 1,
        groups,
        candidates,
        seen,
    );

    let min_root_chars = if prefixes.is_empty() { 1 } else { 4 };
    for &suffix in groups[group_index].items {
        let Some(rest) = current.strip_suffix(suffix) else {
            continue;
        };
        if rest.chars().count() < min_root_chars {
            continue;
        }

        suffixes.push(normalize_suffix_label(suffix));
        push_candidate(candidates, seen, original, rest, prefixes, suffixes);

        if groups[group_index].repeatable {
            recurse_suffix_groups(
                original,
                rest,
                prefixes,
                suffixes,
                group_index,
                groups,
                candidates,
                seen,
            );
        }

        recurse_suffix_groups(
            original,
            rest,
            prefixes,
            suffixes,
            group_index + 1,
            groups,
            candidates,
            seen,
        );
        suffixes.pop();
    }
}

fn sorted_candidates(mut candidates: Vec<RootCandidate>) -> Vec<RootCandidate> {
    candidates.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| b.known_headword.cmp(&a.known_headword))
            .then_with(|| b.known_word.cmp(&a.known_word))
            .then_with(|| b.root.chars().count().cmp(&a.root.chars().count()))
            .then_with(|| a.root.as_bytes().cmp(b.root.as_bytes()))
            .then_with(|| a.prefixes.cmp(&b.prefixes))
            .then_with(|| a.suffixes.cmp(&b.suffixes))
    });
    candidates
}

/// Morphological decomposition of a word.
#[derive(Debug, Clone)]
pub struct Morpheme {
    /// The root form after stripping prefixes and suffixes.
    pub root: String,
    /// उपसर्ग (prefixes) found.
    pub prefixes: Vec<String>,
    /// प्रत्यय (suffixes) found.
    pub suffixes: Vec<String>,
    /// Origin classification.
    pub origin: Origin,
}

/// A lexicon-backed root candidate for dictionary fallback and lookup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootCandidate {
    /// The proposed root form.
    pub root: String,
    /// Prefixes stripped from the input.
    pub prefixes: Vec<String>,
    /// Suffixes stripped from the input.
    pub suffixes: Vec<String>,
    /// Origin classification for the root form.
    pub origin: Origin,
    /// Whether the root exists in the kosha word-form lexicon.
    pub known_word: bool,
    /// Whether the root exists as a headword with metadata.
    pub known_headword: bool,
    /// A stable ranking score. Higher is better.
    pub score: u16,
}

#[derive(Clone, Copy)]
struct SuffixGroup {
    items: &'static [&'static str],
    repeatable: bool,
}

/// Decompose a word into morphological components.
pub fn decompose(word: &str) -> Morpheme {
    if word.is_empty() {
        return Morpheme {
            root: String::new(),
            prefixes: Vec::new(),
            suffixes: Vec::new(),
            origin: Origin::Deshaj,
        };
    }

    let origin = classify(word);
    let mut remaining = word.to_string();
    let mut prefixes = Vec::new();
    let mut suffixes = Vec::new();
    let lex = kosha();

    // Strip known prefixes (including sandhi-ed forms)
    // For consonant assimilation like उत् + ल → उल्ल:
    // We strip "उल्" and the remaining starts with "ल" (the doubled consonant)
    for &(prefix, sandhi_form, _root_prefix) in tables::PREFIX_FORMS.iter() {
        if let Some(rest) = remaining.strip_prefix(sandhi_form) {
            // Short prefixes (≤1 Devanagari char, e.g., अ, आ) require longer roots
            // to prevent over-decomposition (e.g., आगो → prefix अ + root गो).
            let min_root = if sandhi_form.chars().count() <= 1 {
                4
            } else {
                2
            };
            if rest.chars().count() >= min_root && lex.contains(rest) {
                prefixes.push(prefix.to_string());
                remaining = rest.to_string();
                break; // Only strip one prefix for now
            }
        }
    }

    // Strip known suffixes.
    // When a prefix was already found, require the remaining root after suffix
    // stripping to have at least 4 chars (roughly 2 Devanagari syllables) to
    // prevent over-decomposition (e.g., उल्लिखित → root stays "लिखित", not "लिख").
    #[cfg(feature = "iterative-decompose")]
    {
        // 3-phase iterative: Case marker → Plural → Derivational
        let min_root_chars = if prefixes.is_empty() { 1 } else { 4 };
        // Phase 1: Case markers (postpositions) — loop to strip stacked markers
        // e.g., गाईप्रतिको → strip को → गाईप्रति → strip प्रति → गाई
        loop {
            let mut found = false;
            for &sfx in tables::CASE_MARKERS.iter() {
                if let Some(rest) = remaining.strip_suffix(sfx) {
                    if rest.chars().count() >= min_root_chars {
                        suffixes.push(normalize_suffix_label(sfx));
                        remaining = rest.to_string();
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                break;
            }
        }
        // Phase 2: Plural markers
        for &sfx in tables::PLURAL_MARKERS.iter() {
            if let Some(rest) = remaining.strip_suffix(sfx) {
                if rest.chars().count() >= min_root_chars {
                    suffixes.push(normalize_suffix_label(sfx));
                    remaining = rest.to_string();
                    break;
                }
            }
        }
        // Phase 3: Derivational suffixes
        // If case/plural markers were already stripped and the remaining root is a
        // valid dictionary word, skip derivational stripping to avoid over-decomposition
        // (e.g., गाईप्रतिको → गाई is the root, not गा + ई)
        let skip_derivational = !suffixes.is_empty() && lex.contains(&remaining);
        if !skip_derivational {
            for &sfx in tables::SUFFIXES.iter() {
                if let Some(rest) = remaining.strip_suffix(sfx) {
                    if rest.chars().count() >= min_root_chars && lex.contains(rest) {
                        suffixes.push(normalize_suffix_label(sfx));
                        remaining = rest.to_string();
                        break;
                    }
                }
            }
        }
        // Reverse so derivational is first, then plural, then case (inner → outer)
        suffixes.reverse();
    }
    #[cfg(not(feature = "iterative-decompose"))]
    {
        let min_root_chars = if prefixes.is_empty() { 1 } else { 4 };
        for &suffix in tables::SUFFIXES.iter() {
            if let Some(rest) = remaining.strip_suffix(suffix) {
                if rest.chars().count() >= min_root_chars && lex.contains(rest) {
                    suffixes.push(normalize_suffix_label(suffix));
                    remaining = rest.to_string();
                    break; // Only strip one suffix for now
                }
            }
        }
    }

    Morpheme {
        root: remaining,
        prefixes,
        suffixes,
        origin,
    }
}

/// Generate lexicon-backed root candidates by conservatively stripping known
/// prefixes and suffixes. Intended for dictionary lookup fallback paths.
pub fn lookup_root_candidates(word: &str) -> Vec<RootCandidate> {
    if word.is_empty() {
        return Vec::new();
    }

    let mut candidates = Vec::new();
    let mut seen = HashSet::new();

    push_candidate(&mut candidates, &mut seen, word, word, &[], &[]);

    let decomposed = decompose(word);
    push_candidate(
        &mut candidates,
        &mut seen,
        word,
        &decomposed.root,
        &decomposed.prefixes,
        &decomposed.suffixes,
    );

    #[cfg(feature = "iterative-decompose")]
    let groups = [
        SuffixGroup {
            items: tables::CASE_MARKERS,
            repeatable: true,
        },
        SuffixGroup {
            items: tables::PLURAL_MARKERS,
            repeatable: false,
        },
        SuffixGroup {
            items: tables::SUFFIXES,
            repeatable: false,
        },
    ];

    #[cfg(not(feature = "iterative-decompose"))]
    let groups = [SuffixGroup {
        items: tables::SUFFIXES,
        repeatable: false,
    }];

    let empty_prefixes: Vec<String> = Vec::new();
    recurse_suffix_groups(
        word,
        word,
        &empty_prefixes,
        &mut Vec::new(),
        0,
        &groups,
        &mut candidates,
        &mut seen,
    );

    for &(prefix, sandhi_form, _) in tables::PREFIX_FORMS.iter() {
        let Some(rest) = word.strip_prefix(sandhi_form) else {
            continue;
        };
        let min_root = if sandhi_form.chars().count() <= 1 {
            4
        } else {
            2
        };
        if rest.chars().count() < min_root {
            continue;
        }

        let prefixes = vec![prefix.to_string()];
        push_candidate(&mut candidates, &mut seen, word, rest, &prefixes, &[]);
        recurse_suffix_groups(
            word,
            rest,
            &prefixes,
            &mut Vec::new(),
            0,
            &groups,
            &mut candidates,
            &mut seen,
        );
    }

    sorted_candidates(candidates)
}

/// Return whether the word has at least one known root candidate.
pub fn has_known_root(word: &str) -> bool {
    !lookup_root_candidates(word).is_empty()
}

/// Return the highest-ranked known root candidate for the word.
pub fn best_root(word: &str) -> Option<RootCandidate> {
    lookup_root_candidates(word).into_iter().next()
}
