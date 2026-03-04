/// Categories of sandhi rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandhiType {
    /// Vowel sandhi (अच् सन्धि): vowels combining at morpheme boundaries.
    VowelSandhi,
    /// Visarga sandhi (विसर्ग सन्धि): visarga transformations.
    VisargaSandhi,
    /// Consonant sandhi (हल् सन्धि): consonant assimilations.
    ConsonantSandhi,
}

impl SandhiType {
    /// Canonical Devanagari label for display surfaces.
    pub fn display_label(self) -> &'static str {
        match self {
            Self::VowelSandhi => "स्वर सन्धि",
            Self::VisargaSandhi => "विसर्ग सन्धि",
            Self::ConsonantSandhi => "व्यञ्जन सन्धि",
        }
    }
}

/// Coarse-grained sandhi rule families.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleFamily {
    DirectJoin,
    VowelGuna,
    VowelVriddhi,
    Yan,
    Ayadi,
    VisargaR,
    VisargaSibilant,
    ConsonantAssimilation,
}

/// Confidence band for a reverse split candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuthorityTier {
    Exploratory,
    Plausible,
    Likely,
    Authoritative,
}

/// Lexical evidence strength for a candidate member.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LexicalStatus {
    Unknown,
    KnownSurface,
    KnownBoundForm,
    KnownHeadword,
}

/// Result of a forward sandhi operation.
#[derive(Debug, Clone)]
pub struct SandhiResult {
    pub output: String,
    pub sandhi_type: SandhiType,
    pub family: RuleFamily,
    pub rule_id: &'static str,
    pub rule_citation: &'static str,
}

impl SandhiResult {
    pub fn new(
        output: String,
        sandhi_type: SandhiType,
        family: RuleFamily,
        rule_id: &'static str,
        rule_citation: &'static str,
    ) -> Self {
        Self {
            output,
            sandhi_type,
            family,
            rule_id,
            rule_citation,
        }
    }
}

/// Evidence-carrying reverse split candidate.
#[derive(Debug, Clone)]
pub struct SandhiCandidate {
    pub surface: String,
    pub left: String,
    pub right: String,
    pub sandhi_type: SandhiType,
    pub family: RuleFamily,
    pub rule_id: &'static str,
    pub rule_citation: &'static str,
    pub forward_verified: bool,
    pub lexical_left: LexicalStatus,
    pub lexical_right: LexicalStatus,
    pub authority: AuthorityTier,
    pub confidence: f32,
}

/// Error type for sandhi operations.
#[derive(Debug, thiserror::Error)]
pub enum SandhiError {
    #[error("empty input")]
    EmptyInput,

    #[error("no sandhi rule applies for '{first}' + '{second}'")]
    NoRuleApplies { first: String, second: String },
}
