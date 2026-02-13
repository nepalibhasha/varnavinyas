use thiserror::Error;

/// Error type for vyakaran operations.
#[derive(Debug, Error)]
pub enum VyakaranError {
    #[error("morphological analysis not implemented")]
    NotImplemented,
}

/// Grammatical gender (लिंग).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    /// Masculine (पुलिंग)
    Masculine,
    /// Feminine (स्त्रीलिंग)
    Feminine,
    /// Neutral (नपुंसक लिंग) - rare/historical in Nepali but structurally present
    Neuter,
}

/// Grammatical number (वचन).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Number {
    /// Singular (एकवचन)
    Singular,
    /// Plural (बहुवचन)
    Plural,
}

/// Grammatical case (कारक).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Case {
    /// Nominative (कर्ता)
    Nominative,
    /// Accusative (कर्म)
    Accusative,
    /// Instrumental (करण)
    Instrumental,
    /// Dative (सम्प्रदान)
    Dative,
    /// Ablative (अपादान)
    Ablative,
    /// Genitive (सम्बन्ध)
    Genitive,
    /// Locative (अधिकरण)
    Locative,
    /// Vocative (सम्बोधन)
    Vocative,
}

/// Grammatical person (पुरुष).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Person {
    /// First person (प्रथम पुरुष)
    First,
    /// Second person (द्वितीय पुरुष)
    Second,
    /// Third person (तृतीय पुरुष)
    Third,
}

/// Verb tense/aspect (काल/पक्ष).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tense {
    /// Present (वर्तमान)
    Present,
    /// Past (भूत)
    Past,
    /// Future (भविष्यत्)
    Future,
    /// Unknown/Other
    Unknown,
}

/// Grammatical features of a word.
#[derive(Debug, Clone, Default)]
pub struct Features {
    pub gender: Option<Gender>,
    pub number: Option<Number>,
    pub case: Option<Case>,
    pub tense: Option<Tense>,
    pub person: Option<Person>,
}

/// Morphological analysis result for a single word.
#[derive(Debug, Clone)]
pub struct MorphAnalysis {
    /// The dictionary form (lemma)
    pub lemma: String,
    /// Prefix, if detached
    pub prefix: Option<String>,
    /// Suffix/inflection, if detached
    pub suffix: Option<String>,
    /// Grammatical features
    pub features: Features,
}

/// Analyze a word into its morphological components.
pub trait MorphAnalyzer {
    fn analyze(&self, word: &str) -> Result<Vec<MorphAnalysis>, VyakaranError>;
}

/// Stub implementation for Phase 2.
pub struct StubAnalyzer;

impl MorphAnalyzer for StubAnalyzer {
    fn analyze(&self, _word: &str) -> Result<Vec<MorphAnalysis>, VyakaranError> {
        Err(VyakaranError::NotImplemented)
    }
}

/// Rule-based analyzer MVP implementation.
#[cfg(feature = "vyakaran-mvp")]
pub struct RuleBasedAnalyzer;

#[cfg(feature = "vyakaran-mvp")]
impl MorphAnalyzer for RuleBasedAnalyzer {
    fn analyze(&self, word: &str) -> Result<Vec<MorphAnalysis>, VyakaranError> {
        if word.is_empty() {
            return Ok(Vec::new());
        }

        let mut analyses = Vec::new();

        if let Some(analysis) = analyze_nominal(word) {
            analyses.push(analysis);
        }
        if let Some(analysis) = analyze_verbal(word) {
            analyses.push(analysis);
        }

        if analyses.is_empty() {
            analyses.push(MorphAnalysis {
                lemma: word.to_string(),
                prefix: None,
                suffix: None,
                features: Features::default(),
            });
        }

        Ok(analyses)
    }
}

#[cfg(feature = "vyakaran-mvp")]
const CASE_SUFFIXES: &[(&str, Case)] = &[
    ("देखि", Case::Ablative),
    ("बाट", Case::Ablative),
    ("सँग", Case::Instrumental),
    ("लाई", Case::Dative),
    ("तिर", Case::Locative),
    ("का", Case::Genitive),
    ("की", Case::Genitive),
    ("को", Case::Genitive),
    ("ले", Case::Instrumental),
    ("मा", Case::Locative),
];

#[cfg(feature = "vyakaran-mvp")]
const PLURAL_SUFFIXES: &[&str] = &["हरू", "हरु"];

#[cfg(feature = "vyakaran-mvp")]
fn analyze_nominal(word: &str) -> Option<MorphAnalysis> {
    let mut stem = word;
    let mut suffix_parts: Vec<&str> = Vec::new();
    let mut features = Features {
        number: Some(Number::Singular),
        case: Some(Case::Nominative),
        ..Default::default()
    };

    // Right-to-left: case marker first.
    for &(sfx, case) in CASE_SUFFIXES {
        if let Some(rest) = stem.strip_suffix(sfx) {
            if !rest.is_empty() {
                stem = rest;
                suffix_parts.push(sfx);
                features.case = Some(case);
            }
            break;
        }
    }

    // Then plural marker on the remaining stem.
    for &pl in PLURAL_SUFFIXES {
        if let Some(rest) = stem.strip_suffix(pl) {
            if !rest.is_empty() {
                stem = rest;
                suffix_parts.push(pl);
                features.number = Some(Number::Plural);
            }
            break;
        }
    }

    if stem == word {
        return None;
    }

    let lemma = nominal_lemma_from_stem(stem, features.case);
    suffix_parts.reverse();
    let suffix = if suffix_parts.is_empty() {
        None
    } else {
        Some(suffix_parts.concat())
    };

    Some(MorphAnalysis {
        lemma,
        prefix: None,
        suffix,
        features,
    })
}

#[cfg(feature = "vyakaran-mvp")]
fn nominal_lemma_from_stem(stem: &str, case: Option<Case>) -> String {
    let lex = varnavinyas_kosha::kosha();

    // Oblique recovery: for case-marked forms, try stem ा -> lemma ो
    // when the ो form is lexically known.
    if case.is_some_and(|c| c != Case::Nominative) {
        if let Some(base) = stem.strip_suffix('ा') {
            let candidate = format!("{base}ो");
            if lex.contains(&candidate) {
                return candidate;
            }
        }
    }

    if lex.contains(stem) {
        return stem.to_string();
    }

    // Fallback to shabda decomposition root when direct lexicon lookup misses.
    varnavinyas_shabda::decompose(stem).root
}

#[cfg(feature = "vyakaran-mvp")]
fn analyze_verbal(word: &str) -> Option<MorphAnalysis> {
    // Infinitive: -नु
    if let Some(stem) = word.strip_suffix("नु") {
        if !stem.is_empty() {
            return Some(MorphAnalysis {
                lemma: word.to_string(),
                prefix: None,
                suffix: Some("नु".to_string()),
                features: Features {
                    tense: Some(Tense::Unknown),
                    ..Default::default()
                },
            });
        }
    }

    // Progressive markers: ...दै + present ending.
    for &(ending, person) in &[
        ("छु", Person::First),
        ("छौ", Person::Second),
        ("छ", Person::Third),
    ] {
        if word.ends_with(ending) && word.contains("दै") {
            return Some(MorphAnalysis {
                lemma: word.to_string(),
                prefix: None,
                suffix: Some(ending.to_string()),
                features: Features {
                    tense: Some(Tense::Present),
                    person: Some(person),
                    ..Default::default()
                },
            });
        }
    }

    // Simple present negative cue.
    if word.ends_with("छैन") {
        return Some(MorphAnalysis {
            lemma: word.to_string(),
            prefix: None,
            suffix: Some("छैन".to_string()),
            features: Features {
                tense: Some(Tense::Present),
                ..Default::default()
            },
        });
    }

    // Simple past negative cue.
    if word.ends_with("एन") {
        return Some(MorphAnalysis {
            lemma: word.to_string(),
            prefix: None,
            suffix: Some("एन".to_string()),
            features: Features {
                tense: Some(Tense::Past),
                ..Default::default()
            },
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_returns_error() {
        let analyzer = StubAnalyzer;
        assert!(matches!(
            analyzer.analyze("नेपाल"),
            Err(VyakaranError::NotImplemented)
        ));
    }

    #[cfg(feature = "vyakaran-mvp")]
    #[test]
    fn nominal_case_and_plural_detected() {
        let analyzer = RuleBasedAnalyzer;
        let analyses = analyzer
            .analyze("केटाहरूलाई")
            .expect("analysis should succeed");
        let m = analyses
            .iter()
            .find(|a| a.features.case == Some(Case::Dative))
            .expect("expected nominal dative analysis");
        assert_eq!(m.features.number, Some(Number::Plural));
        assert_eq!(m.suffix.as_deref(), Some("हरूलाई"));
    }

    #[cfg(feature = "vyakaran-mvp")]
    #[test]
    fn oblique_o_to_a_recovers_lemma() {
        let analyzer = RuleBasedAnalyzer;
        let analyses = analyzer.analyze("केटालाई").expect("analysis should succeed");
        let m = analyses
            .iter()
            .find(|a| a.features.case == Some(Case::Dative))
            .expect("expected nominal dative analysis");
        assert_eq!(m.lemma, "केटो");
    }

    #[cfg(feature = "vyakaran-mvp")]
    #[test]
    fn verbal_infinitive_detected() {
        let analyzer = RuleBasedAnalyzer;
        let analyses = analyzer.analyze("लेखनु").expect("analysis should succeed");
        assert!(
            analyses
                .iter()
                .any(|a| a.suffix.as_deref() == Some("नु")
                    && a.features.tense == Some(Tense::Unknown))
        );
    }
}
