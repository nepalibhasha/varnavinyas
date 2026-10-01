mod inventory;

use varnavinyas_kosha::kosha;

/// Samasa taxonomy for reviewed interpretations, not automatic POS inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamasaType {
    Tatpurusha,
    Karmadharaya,
    Dvigu,
    Bahuvrihi,
    Dvandva,
    Avyayibhava,
    Unknown,
}

/// A reviewed compound interpretation. `score` is a ranking weight, not a probability.
#[derive(Debug, Clone, PartialEq)]
pub struct SamasaCandidate {
    pub left: String,
    pub right: String,
    pub samasa_type: SamasaType,
    pub score: f32,
    pub vigraha: String,
}

/// Return supported samasa interpretations of an uninflected word.
///
/// Two dictionary entries and a forward sandhi round trip establish only a
/// possible spelling boundary. Public analysis additionally requires reviewed
/// evidence for this word's members, grammatical relationship and vigraha.
/// An empty result means no supported analysis, not that the word is incorrect.
/// Prefix/suffix derivations belong to morphology; raw reverse-sandhi candidates
/// remain available through `varnavinyas_sandhi::split` for exploration.
pub fn analyze_compound(word: &str) -> Vec<SamasaCandidate> {
    let Some(entry) = inventory::lookup(word) else {
        return Vec::new();
    };
    let lex = kosha();
    if lex.lookup(entry.left).is_none() || lex.lookup(entry.right).is_none() {
        return Vec::new();
    }
    let direct_join = format!("{}{}", entry.left, entry.right) == word;
    if !direct_join
        && !varnavinyas_sandhi::apply_all(entry.left, entry.right)
            .iter()
            .any(|result| result.output == word)
    {
        return Vec::new();
    }

    // Retain legacy ranking weights for supported interpretations. These are
    // deliberately not statistical confidence estimates.
    let base_score: f32 = match entry.samasa_type {
        SamasaType::Tatpurusha => 0.82,
        SamasaType::Karmadharaya => 0.84,
        SamasaType::Dvigu => 0.92,
        SamasaType::Bahuvrihi => 0.74,
        SamasaType::Dvandva => 0.80,
        SamasaType::Avyayibhava => 0.86,
        SamasaType::Unknown => 0.50,
    };
    vec![SamasaCandidate {
        left: entry.left.to_string(),
        right: entry.right.to_string(),
        samasa_type: entry.samasa_type,
        score: base_score - if direct_join { 0.05 } else { 0.0 },
        vigraha: entry.vigraha.to_string(),
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reviewed_pairs_are_the_only_winners_and_have_explicit_vigraha() {
        for (word, left, right, samasa_type) in [
            ("सूर्योदय", "सूर्य", "उदय", SamasaType::Tatpurusha),
            ("महोत्सव", "महा", "उत्सव", SamasaType::Karmadharaya),
            ("एकचक्र", "एक", "चक्र", SamasaType::Dvigu),
            ("पूर्वाधार", "पूर्व", "आधार", SamasaType::Karmadharaya),
            ("मापदण्ड", "माप", "दण्ड", SamasaType::Tatpurusha),
        ] {
            let candidates = analyze_compound(word);
            assert_eq!(candidates.len(), 1, "{word}: {candidates:?}");
            let top = &candidates[0];
            assert_eq!((top.left.as_str(), top.right.as_str()), (left, right));
            assert_eq!(top.samasa_type, samasa_type, "{word}");
            assert!(!top.vigraha.is_empty(), "{word}");
        }
    }

    #[test]
    fn lexical_coincidences_and_derivations_are_not_compound_evidence() {
        for word in [
            "सवारीमा",
            "दशकमा",
            "आयात",
            "आर्थिक",
            "विकास",
            "यातायात",
            "व्यवस्थापन",
            "रिसाइकल",
            "आधारमा",
            "छलफल",
            "विवरण",
            "सबैलाई",
            "नेपाली",
            "धार्मिक",
            "",
        ] {
            assert!(analyze_compound(word).is_empty(), "{word}");
        }
    }
}
