use crate::origin::{Origin, classify};
use crate::tables;
use std::collections::HashSet;
use varnavinyas_kosha::{Kosha, kosha};

const MAX_PREFIX_DEPTH: usize = 2;
const MAX_SUFFIX_DEPTH: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AffixKind {
    Prefix,
    PluralMarker,
    CaseMarker,
    Particle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AffixSegment {
    pub text: String,
    pub kind: AffixKind,
}

#[derive(Clone, Copy)]
struct SupportGroup {
    items: &'static [&'static str],
    kind: AffixKind,
    repeatable: bool,
}

const SUPPORT_SUFFIX_GROUPS: &[SupportGroup] = &[
    SupportGroup {
        items: tables::PARTICLES,
        kind: AffixKind::Particle,
        repeatable: false,
    },
    SupportGroup {
        items: tables::CASE_MARKERS,
        kind: AffixKind::CaseMarker,
        repeatable: true,
    },
    SupportGroup {
        items: tables::PLURAL_MARKERS,
        kind: AffixKind::PluralMarker,
        repeatable: false,
    },
];

#[derive(Clone, Copy, Default)]
struct LexicalSupport {
    known_word: bool,
    known_headword: bool,
}

impl LexicalSupport {
    fn is_exact(self) -> bool {
        self.known_word || self.known_headword
    }
}

fn lexical_support(word: &str, lex: &Kosha) -> LexicalSupport {
    LexicalSupport {
        known_word: lex.contains(word),
        known_headword: lex.lookup(word).is_some(),
    }
}

#[derive(Clone, Copy, Default)]
struct BaseSupport {
    lexical: LexicalSupport,
    suffixed_sibling: bool,
    prefixed_sibling: bool,
}

impl BaseSupport {
    fn is_supported(self) -> bool {
        self.lexical.is_exact() || self.suffixed_sibling || self.prefixed_sibling
    }
}

fn has_attested_suffixed_sibling(word: &str, lex: &Kosha) -> bool {
    tables::PARTICLES
        .iter()
        .chain(tables::CASE_MARKERS.iter())
        .chain(tables::PLURAL_MARKERS.iter())
        .any(|suffix| {
            let candidate = format!("{word}{suffix}");
            lex.contains(&candidate) || lex.lookup(&candidate).is_some()
        })
}

fn has_attested_prefixed_sibling(word: &str, lex: &Kosha) -> bool {
    tables::PREFIX_FORMS.iter().any(|&(_, sandhi_form, _)| {
        let candidate = format!("{sandhi_form}{word}");
        lex.contains(&candidate) || lex.lookup(&candidate).is_some()
    })
}

fn base_support(word: &str, lex: &Kosha) -> BaseSupport {
    if word.is_empty() {
        return BaseSupport::default();
    }

    BaseSupport {
        lexical: lexical_support(word, lex),
        suffixed_sibling: has_attested_suffixed_sibling(word, lex),
        prefixed_sibling: has_attested_prefixed_sibling(word, lex),
    }
}

fn normalize_suffix_label(sfx: &str) -> String {
    match sfx {
        "ि" => "इ".to_string(),
        "ी" => "ई".to_string(),
        "ु" => "उ".to_string(),
        "ू" => "ऊ".to_string(),
        _ => sfx.to_string(),
    }
}

fn affix_score(
    surface: &str,
    prefixes: &[String],
    suffixes: &[AffixSegment],
    support: BaseSupport,
) -> u16 {
    let mut score = 0;
    if support.lexical.known_word {
        score += 100;
    }
    if support.lexical.known_headword {
        score += 150;
    }
    if support.suffixed_sibling {
        score += 60;
    }
    if support.prefixed_sibling {
        score += 60;
    }
    score += (prefixes.len() as u16) * 20;
    score += (suffixes.len() as u16) * 15;
    if !prefixes.is_empty() || !suffixes.is_empty() {
        score += 10;
    }
    score += surface.chars().count() as u16;
    score
}

/// Structured result for conservative prefix/suffix analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AffixAnalysis {
    /// Original surface form.
    pub surface: String,
    /// Surface stem after removing only outer suffixes/particles.
    pub stem: String,
    /// Base root after removing recognized prefixes from `stem`.
    pub root: String,
    /// Recognized prefixes in outer-to-inner order.
    pub prefixes: Vec<String>,
    /// Recognized prefix segments with explicit kind metadata.
    pub prefix_segments: Vec<AffixSegment>,
    /// Recognized suffixes/particles in inner-to-outer order.
    pub suffixes: Vec<String>,
    /// Recognized suffix segments with explicit kind metadata.
    pub suffix_segments: Vec<AffixSegment>,
    /// Stable ranking score. Higher is better.
    pub score: u16,
}

fn push_affix_analysis(
    analyses: &mut Vec<AffixAnalysis>,
    seen: &mut HashSet<String>,
    surface: &str,
    stem: &str,
    root: &str,
    prefixes: &[String],
    suffixes: &[AffixSegment],
    support: BaseSupport,
) {
    if !support.is_supported() {
        return;
    }

    let stored_suffixes: Vec<String> = suffixes.iter().rev().map(|s| s.text.clone()).collect();
    let stored_suffix_segments: Vec<AffixSegment> = suffixes.iter().rev().cloned().collect();
    let stored_prefix_segments: Vec<AffixSegment> = prefixes
        .iter()
        .cloned()
        .map(|text| AffixSegment {
            text,
            kind: AffixKind::Prefix,
        })
        .collect();

    let key = format!(
        "{}|{}|{}|{}",
        stem,
        root,
        prefixes.join("+"),
        stored_suffixes.join("+")
    );
    if !seen.insert(key) {
        return;
    }

    analyses.push(AffixAnalysis {
        surface: surface.to_string(),
        stem: stem.to_string(),
        root: root.to_string(),
        prefixes: prefixes.to_vec(),
        prefix_segments: stored_prefix_segments,
        suffixes: stored_suffixes,
        suffix_segments: stored_suffix_segments,
        score: affix_score(surface, prefixes, suffixes, support),
    });
}

fn collect_prefixed_analyses(
    surface: &str,
    stem: &str,
    current: &str,
    prefixes: &mut Vec<String>,
    suffixes: &[AffixSegment],
    prefix_depth: usize,
    analyses: &mut Vec<AffixAnalysis>,
    seen: &mut HashSet<String>,
    lex: &Kosha,
) {
    let support = base_support(current, lex);
    push_affix_analysis(
        analyses, seen, surface, stem, current, prefixes, suffixes, support,
    );

    if prefix_depth >= MAX_PREFIX_DEPTH {
        return;
    }

    for &(prefix, sandhi_form, _) in tables::PREFIX_FORMS.iter() {
        let Some(rest) = current.strip_prefix(sandhi_form) else {
            continue;
        };
        let min_root_chars = if sandhi_form.chars().count() <= 1 {
            4
        } else {
            2
        };
        if rest.chars().count() < min_root_chars {
            continue;
        }

        prefixes.push(prefix.to_string());
        collect_prefixed_analyses(
            surface,
            stem,
            rest,
            prefixes,
            suffixes,
            prefix_depth + 1,
            analyses,
            seen,
            lex,
        );
        prefixes.pop();
    }
}

fn collect_suffix_group_analyses(
    surface: &str,
    current: &str,
    suffixes: &mut Vec<AffixSegment>,
    group_index: usize,
    analyses: &mut Vec<AffixAnalysis>,
    seen: &mut HashSet<String>,
    lex: &Kosha,
) {
    let mut prefixes = Vec::new();
    collect_prefixed_analyses(
        surface,
        current,
        current,
        &mut prefixes,
        suffixes,
        0,
        analyses,
        seen,
        lex,
    );

    if group_index >= SUPPORT_SUFFIX_GROUPS.len() || suffixes.len() >= MAX_SUFFIX_DEPTH {
        return;
    }

    collect_suffix_group_analyses(
        surface,
        current,
        suffixes,
        group_index + 1,
        analyses,
        seen,
        lex,
    );

    for &suffix in SUPPORT_SUFFIX_GROUPS[group_index].items {
        for rest in suffix_rest_candidates(current, suffix) {
            suffixes.push(AffixSegment {
                text: suffix.to_string(),
                kind: SUPPORT_SUFFIX_GROUPS[group_index].kind,
            });
            if SUPPORT_SUFFIX_GROUPS[group_index].repeatable {
                collect_suffix_group_analyses(
                    surface,
                    rest,
                    suffixes,
                    group_index,
                    analyses,
                    seen,
                    lex,
                );
            }
            collect_suffix_group_analyses(
                surface,
                rest,
                suffixes,
                group_index + 1,
                analyses,
                seen,
                lex,
            );
            suffixes.pop();
        }
    }
}

fn suffix_rest_candidates<'a>(current: &'a str, suffix: &str) -> Vec<&'a str> {
    let mut candidates = Vec::new();
    let current_len = current.chars().count();
    let suffix_len = suffix.chars().count();

    if let Some(rest) = current.strip_suffix(suffix) {
        if !rest.is_empty() {
            candidates.push(rest);
        }
    }

    let mut suffix_chars = suffix.chars();
    let Some(onset) = suffix_chars.next() else {
        return candidates;
    };
    let tail = suffix_chars.as_str();
    if tail.is_empty() {
        return candidates;
    }

    // Shared-onset form: the suffix starts with the same consonant the stem
    // already ends with, so the surface may carry only the suffix tail.
    if current_len > suffix_len {
        if let Some(rest) = current.strip_suffix(tail) {
            if !rest.is_empty() && rest.ends_with(onset) && !candidates.contains(&rest) {
                candidates.push(rest);
            }
        }
    }

    candidates
}

fn sorted_affix_analyses(mut analyses: Vec<AffixAnalysis>) -> Vec<AffixAnalysis> {
    analyses.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| b.suffixes.len().cmp(&a.suffixes.len()))
            .then_with(|| b.prefixes.len().cmp(&a.prefixes.len()))
            .then_with(|| b.stem.chars().count().cmp(&a.stem.chars().count()))
            .then_with(|| a.stem.as_bytes().cmp(b.stem.as_bytes()))
            .then_with(|| a.root.as_bytes().cmp(b.root.as_bytes()))
    });
    analyses
}

/// Collect conservative affix analyses for a surface form.
pub fn analyze_affixes(word: &str) -> Vec<AffixAnalysis> {
    if word.is_empty() {
        return Vec::new();
    }

    let lex = kosha();
    let mut analyses = Vec::new();
    let mut seen = HashSet::new();
    collect_suffix_group_analyses(
        word,
        word,
        &mut Vec::new(),
        0,
        &mut analyses,
        &mut seen,
        lex,
    );
    sorted_affix_analyses(analyses)
}

/// Return the highest-ranked affix analysis for a surface form.
pub fn best_analysis(word: &str) -> Option<AffixAnalysis> {
    analyze_affixes(word).into_iter().next()
}

/// Return whether a surface form is supported by exact lexicon evidence or by a
/// conservative prefix/suffix analysis over a supported stem.
pub fn has_supported_analysis(word: &str) -> bool {
    best_analysis(word).is_some()
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
    let support = lexical_support(root, lex);
    let known_word = support.known_word;
    let known_headword = support.known_headword;
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
        for rest in suffix_rest_candidates(current, suffix) {
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
                for rest in suffix_rest_candidates(&remaining, sfx) {
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
            for rest in suffix_rest_candidates(&remaining, sfx) {
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
                for rest in suffix_rest_candidates(&remaining, sfx) {
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
            for rest in suffix_rest_candidates(&remaining, suffix) {
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
