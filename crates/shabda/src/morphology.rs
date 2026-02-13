use crate::origin::{Origin, classify};
use crate::tables;
use varnavinyas_kosha::kosha;

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
            let min_root = if sandhi_form.chars().count() <= 1 { 4 } else { 2 };
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
                        suffixes.push(sfx.to_string());
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
                    suffixes.push(sfx.to_string());
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
                        suffixes.push(sfx.to_string());
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
                    suffixes.push(suffix.to_string());
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
