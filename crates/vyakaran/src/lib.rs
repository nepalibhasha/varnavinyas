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
}
