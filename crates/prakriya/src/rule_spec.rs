use crate::prakriya::Prakriya;
use crate::rule::Rule;

/// Diagnostic severity for a rule violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    /// Definite error — the word is wrong.
    Error,
    /// Valid variant — both forms may be acceptable.
    Variant,
    /// Ambiguous — needs manual review.
    Ambiguous,
}

impl DiagnosticKind {
    /// Stable machine-readable code for serialization.
    pub fn as_code(&self) -> &'static str {
        match self {
            Self::Error => "Error",
            Self::Variant => "Variant",
            Self::Ambiguous => "Ambiguous",
        }
    }
}

/// Category grouping for pattern rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCategory {
    HrasvaDirgha,
    Chandrabindu,
    ShaShaS,
    RiKri,
    Halanta,
    Structural,
    Sandhi,
    AadhiVriddhi,
    YaE,
    KshaChhya,
}

/// Metadata for a single pattern rule.
#[derive(Debug, Clone, Copy)]
pub struct RuleSpec {
    /// Unique identifier (e.g., "struct-shri").
    pub id: &'static str,
    /// Category grouping.
    pub category: RuleCategory,
    /// Diagnostic severity.
    pub kind: DiagnosticKind,
    /// Evaluation priority (lower = higher priority).
    pub priority: u16,
    /// Academy standard citation.
    pub citation: Rule,
    /// Example (incorrect, correct) pairs.
    pub examples: &'static [(&'static str, &'static str)],
}

/// A pattern rule with metadata and apply function.
pub struct PatternRule {
    /// Rule metadata.
    pub spec: RuleSpec,
    /// The rule function: takes input, returns corrected Prakriya if applicable.
    pub apply: fn(&str) -> Option<Prakriya>,
}
