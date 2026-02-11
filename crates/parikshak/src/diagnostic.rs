use varnavinyas_prakriya::Rule;

/// Category of a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCategory {
    HrasvaDirgha,
    Chandrabindu,
    ShaShaS,
    RiKri,
    Halanta,
    YaE,
    KshaChhya,
    Sandhi,
    Punctuation,
    ShuddhaTable,
}

impl DiagnosticCategory {
    /// Infer category from a Rule.
    pub fn from_rule(rule: &Rule) -> Self {
        match rule {
            Rule::ShuddhaAshuddha(_) => DiagnosticCategory::ShuddhaTable,
            Rule::ChihnaNiyam(_) => DiagnosticCategory::Punctuation,
            Rule::VarnaVinyasNiyam(code) => {
                if code.contains("ह्रस्व") || code.contains("दीर्घ") || code.contains("3(क)")
                {
                    DiagnosticCategory::HrasvaDirgha
                } else if code.contains("चन्द्रबिन्दु") || code.contains("3(ख)")
                {
                    DiagnosticCategory::Chandrabindu
                } else if code.contains("श/ष/स") || code.contains("3(ग)") {
                    DiagnosticCategory::ShaShaS
                } else if code.contains("ऋ") || code.contains("कृ") {
                    DiagnosticCategory::RiKri
                } else if code.contains("हलन्त") {
                    DiagnosticCategory::Halanta
                } else if code.contains("य/ए") {
                    DiagnosticCategory::YaE
                } else if code.contains("क्ष") || code.contains("छ्य") {
                    DiagnosticCategory::KshaChhya
                } else if code.contains("सन्धि") || code.contains("sandhi") {
                    DiagnosticCategory::Sandhi
                } else {
                    DiagnosticCategory::ShuddhaTable
                }
            }
            Rule::Vyakaran(code) => {
                if code.contains("सन्धि") || code.contains("sandhi") {
                    DiagnosticCategory::Sandhi
                } else {
                    DiagnosticCategory::ShuddhaTable
                }
            }
        }
    }
}

impl std::fmt::Display for DiagnosticCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HrasvaDirgha => write!(f, "ह्रस्व/दीर्घ"),
            Self::Chandrabindu => write!(f, "चन्द्रबिन्दु"),
            Self::ShaShaS => write!(f, "श/ष/स"),
            Self::RiKri => write!(f, "ऋ/कृ"),
            Self::Halanta => write!(f, "हलन्त"),
            Self::YaE => write!(f, "य/ए"),
            Self::KshaChhya => write!(f, "क्ष/छ्य"),
            Self::Sandhi => write!(f, "सन्धि"),
            Self::Punctuation => write!(f, "चिह्न"),
            Self::ShuddhaTable => write!(f, "शुद्ध-अशुद्ध"),
        }
    }
}

/// A spell-check diagnostic.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// Byte offset span (start, end) in the original text.
    pub span: (usize, usize),
    /// The incorrect form found.
    pub incorrect: String,
    /// The suggested correction.
    pub correction: String,
    /// The rule that was applied.
    pub rule: Rule,
    /// Human-readable explanation.
    pub explanation: String,
    /// Category of the issue.
    pub category: DiagnosticCategory,
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} → {} ({})",
            self.category, self.incorrect, self.correction, self.explanation
        )
    }
}
