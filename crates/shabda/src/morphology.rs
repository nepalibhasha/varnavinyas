use crate::origin::{Origin, classify};
use crate::tables;

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

    // Strip known prefixes (including sandhi-ed forms)
    // For consonant assimilation like उत् + ल → उल्ल:
    // We strip "उल्" and the remaining starts with "ल" (the doubled consonant)
    for &(prefix, sandhi_form, _root_prefix) in tables::PREFIX_FORMS.iter() {
        if let Some(rest) = remaining.strip_prefix(sandhi_form) {
            // Short prefixes (≤1 Devanagari char, e.g., अ, आ) require longer roots
            // to prevent over-decomposition (e.g., आगो → prefix अ + root गो).
            let min_root = if sandhi_form.chars().count() <= 1 { 4 } else { 2 };
            if rest.chars().count() >= min_root {
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
    let min_root_chars = if prefixes.is_empty() { 1 } else { 4 };
    for &suffix in tables::SUFFIXES.iter() {
        if let Some(rest) = remaining.strip_suffix(suffix) {
            if rest.chars().count() >= min_root_chars {
                suffixes.push(suffix.to_string());
                remaining = rest.to_string();
                break; // Only strip one suffix for now
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
