use crate::tables;
pub use varnavinyas_types::Origin;

/// Provenance for origin classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OriginSource {
    /// From the local override table.
    Override,
    /// From kosha dictionary origin tags.
    Kosha,
    /// From heuristic fallback rules.
    Heuristic,
}

/// Origin decision with provenance metadata.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OriginDecision {
    pub origin: Origin,
    pub source: OriginSource,
    pub confidence: f32,
}

/// Classify a Nepali word by its origin.
///
/// Three-tier lookup:
/// 1. Override table (small, for edge cases where dictionary/heuristic fails)
/// 2. Kosha dictionary lookup (~26K words with origin tags from Brihat Shabdakosha)
/// 3. Heuristic classification (phonological pattern matching)
pub fn classify(word: &str) -> Origin {
    classify_with_provenance(word).origin
}

/// Classify a word with provenance and confidence metadata.
pub fn classify_with_provenance(word: &str) -> OriginDecision {
    if word.is_empty() {
        return OriginDecision {
            origin: Origin::Deshaj,
            source: OriginSource::Heuristic,
            confidence: 0.0,
        };
    }

    // 1. Override table (small set of manually verified edge cases)
    if let Some(origin) = tables::lookup_origin(word) {
        return OriginDecision {
            origin,
            source: OriginSource::Override,
            confidence: 1.0,
        };
    }

    // 2. Kosha dictionary lookup (~26K words with origin tags)
    if let Some(tag) = varnavinyas_kosha::kosha().origin_of(word) {
        return OriginDecision {
            origin: tag,
            source: OriginSource::Kosha,
            confidence: 0.95,
        };
    }

    // 3. Heuristic classification
    OriginDecision {
        origin: classify_heuristic(word),
        source: OriginSource::Heuristic,
        confidence: 0.65,
    }
}

fn classify_heuristic(word: &str) -> Origin {
    let chars: Vec<char> = word.chars().collect();

    // Aagantuk indicators: foreign consonant clusters, nukta forms
    if has_aagantuk_markers(&chars) {
        return Origin::Aagantuk;
    }

    // Tatsam markers: ऋ, ष, क्ष, ज्ञ, visarga, specific conjuncts
    if has_tatsam_markers(word, &chars) {
        return Origin::Tatsam;
    }

    // Tadbhav patterns: simplified phonology
    if has_tadbhav_markers(word, &chars) {
        return Origin::Tadbhav;
    }

    // Default: Deshaj (native Nepali)
    Origin::Deshaj
}

fn has_aagantuk_markers(chars: &[char]) -> bool {
    // Nukta forms (क़ ख़ ग़ ज़ ड़ ढ़ फ़)
    for c in chars {
        if matches!(
            c,
            '\u{0958}'..='\u{095F}' // Precomposed nukta consonants
        ) {
            return true;
        }
    }

    // Check for nukta combining character
    for window in chars.windows(2) {
        if window[1] == '\u{093C}' {
            // ़ (nukta) following consonant
            return true;
        }
    }

    false
}

fn has_tatsam_markers(word: &str, chars: &[char]) -> bool {
    // Direct tatsam vowel: ऋ
    if chars.contains(&'ऋ') || chars.contains(&'ृ') {
        return true;
    }

    // ष (retroflex sibilant) — strong tatsam marker
    if chars.contains(&'ष') {
        return true;
    }

    // Visarga ः
    if chars.contains(&'ः') {
        return true;
    }

    // Conjuncts: क्ष, ज्ञ
    if word.contains("क्ष") || word.contains("ज्ञ") || word.contains("क्त") || word.contains("त्म")
    {
        return true;
    }

    // श्र (common tatsam conjunct, but not exclusive)
    // Additional tatsam conjuncts
    if word.contains("त्र")
        || word.contains("त्त")
        || word.contains("द्ध")
        || word.contains("द्य")
        || word.contains("द्व")
    {
        return true;
    }

    false
}

/// Look up the source language for a word (e.g., "फारसी", "अरबी", "संस्कृत").
///
/// Uses the kosha dictionary's origin tags. Returns `None` if the word has no
/// recognized language tag or is not a known headword.
pub fn source_language(word: &str) -> Option<&'static str> {
    varnavinyas_kosha::kosha().source_language_of(word)
}

fn has_tadbhav_markers(word: &str, chars: &[char]) -> bool {
    // Common tadbhav endings: -ो, -ा with simplified consonants
    let last = chars.last().copied().unwrap_or('\0');
    let second_last = if chars.len() >= 2 {
        chars[chars.len() - 2]
    } else {
        '\0'
    };

    // Common tadbhav verb endings
    if word.ends_with("नु") || word.ends_with("ने") || word.ends_with("को") {
        return true;
    }

    // Tadbhav diminutives/informal endings
    if last == 'ो' || (second_last != '\0' && matches!(last, 'ो' | 'ा')) {
        // Words ending in -ठो, -ठा etc. are often tadbhav
        if matches!(second_last, 'ठ' | 'ड' | 'ढ') {
            return true;
        }
    }

    false
}
