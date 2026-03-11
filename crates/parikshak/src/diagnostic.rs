use varnavinyas_prakriya::{DiagnosticKind, Rule, RuleCategory};

/// Category of a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCategory {
    HrasvaDirgha,
    Chandrabindu,
    ShaShaS,
    BaVa,
    RiKri,
    Halanta,
    AadhiVriddhi,
    YaE,
    KshaChhya,
    GyaGyan,
    Sandhi,
    Punctuation,
    ShuddhaTable,
}

impl DiagnosticCategory {
    /// Stable machine-readable code for serialization to JS/JSON consumers.
    /// This is an explicit API contract — do not rename without updating web/js/utils.js.
    pub fn as_code(&self) -> &'static str {
        match self {
            Self::HrasvaDirgha => "HrasvaDirgha",
            Self::Chandrabindu => "Chandrabindu",
            Self::ShaShaS => "ShaShaS",
            Self::BaVa => "BaVa",
            Self::RiKri => "RiKri",
            Self::Halanta => "Halanta",
            Self::AadhiVriddhi => "AadhiVriddhi",
            Self::YaE => "YaE",
            Self::KshaChhya => "KshaChhya",
            Self::GyaGyan => "GyaGyan",
            Self::Sandhi => "Sandhi",
            Self::Punctuation => "Punctuation",
            Self::ShuddhaTable => "ShuddhaTable",
        }
    }

    /// Infer category from a Rule.
    pub fn from_rule(rule: &Rule) -> Self {
        match rule {
            Rule::ShuddhaAshuddha(_) => DiagnosticCategory::ShuddhaTable,
            Rule::ChihnaNiyam(_) => DiagnosticCategory::Punctuation,
            Rule::VarnaVinyasNiyam(code) => {
                if code.contains("3(क)-इक-प्रत्यय") {
                    DiagnosticCategory::AadhiVriddhi
                } else if code.contains("ऋ")
                    || code.contains("कृ")
                    || code.contains("3(ग)-ऋ")
                    || code.contains("3(ग)-कृ")
                {
                    DiagnosticCategory::RiKri
                } else if code.contains("ब/व")
                    || code.contains("3(ग)-बव")
                    || code.contains("3(ग)(आ)")
                {
                    DiagnosticCategory::BaVa
                } else if code.contains("क्ष") || code.contains("छ्य") || code.contains("3(छ)-क्ष")
                {
                    DiagnosticCategory::KshaChhya
                } else if code.contains("ज्ञ") || code.contains("ग्य") {
                    DiagnosticCategory::GyaGyan
                } else if code.contains("य/ए") || *code == "3(छ)" {
                    DiagnosticCategory::YaE
                } else if code.contains("ह्रस्व") || code.contains("दीर्घ") || code.contains("3(क)")
                {
                    DiagnosticCategory::HrasvaDirgha
                } else if code.contains("चन्द्रबिन्दु") || code.contains("3(ख)")
                {
                    DiagnosticCategory::Chandrabindu
                } else if code.contains("श/ष/स") || code.contains("3(ग)") {
                    DiagnosticCategory::ShaShaS
                } else if code.contains("हलन्त") || code.contains("3(ङ)") {
                    DiagnosticCategory::Halanta
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

    /// Typed mapping from prakriya rule categories.
    pub fn from_rule_category(category: RuleCategory) -> Self {
        match category {
            RuleCategory::ShuddhaTable => DiagnosticCategory::ShuddhaTable,
            RuleCategory::HrasvaDirgha => DiagnosticCategory::HrasvaDirgha,
            RuleCategory::Chandrabindu => DiagnosticCategory::Chandrabindu,
            RuleCategory::ShaShaS => DiagnosticCategory::ShaShaS,
            RuleCategory::BaVa => DiagnosticCategory::BaVa,
            RuleCategory::RiKri => DiagnosticCategory::RiKri,
            RuleCategory::Halanta => DiagnosticCategory::Halanta,
            RuleCategory::Sandhi => DiagnosticCategory::Sandhi,
            RuleCategory::AadhiVriddhi => DiagnosticCategory::AadhiVriddhi,
            RuleCategory::YaE => DiagnosticCategory::YaE,
            RuleCategory::KshaChhya => DiagnosticCategory::KshaChhya,
            RuleCategory::GyaGyan => DiagnosticCategory::GyaGyan,
            RuleCategory::Structural => DiagnosticCategory::ShuddhaTable,
        }
    }
}

/// Prefer typed prakriya metadata, but upgrade generic ShuddhaTable to a more
/// specific family when the winning rule itself clearly identifies one.
pub fn choose_diagnostic_category(
    category: Option<RuleCategory>,
    rule: &Rule,
) -> DiagnosticCategory {
    match category.map(DiagnosticCategory::from_rule_category) {
        Some(DiagnosticCategory::ShuddhaTable) => {
            let from_rule = DiagnosticCategory::from_rule(rule);
            if from_rule == DiagnosticCategory::ShuddhaTable {
                DiagnosticCategory::ShuddhaTable
            } else {
                from_rule
            }
        }
        Some(specific) => specific,
        None => DiagnosticCategory::from_rule(rule),
    }
}

impl std::fmt::Display for DiagnosticCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HrasvaDirgha => write!(f, "ह्रस्व/दीर्घ"),
            Self::Chandrabindu => write!(f, "चन्द्रबिन्दु"),
            Self::ShaShaS => write!(f, "श/ष/स"),
            Self::BaVa => write!(f, "ब/व"),
            Self::RiKri => write!(f, "ऋ/कृ"),
            Self::Halanta => write!(f, "हलन्त"),
            Self::AadhiVriddhi => write!(f, "आदिवृद्धि"),
            Self::YaE => write!(f, "य/ए"),
            Self::KshaChhya => write!(f, "क्ष/छ्य"),
            Self::GyaGyan => write!(f, "ज्ञ/ग्य"),
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
    /// Severity kind (error, variant, ambiguous).
    pub kind: DiagnosticKind,
    /// Confidence score (0.0–1.0).
    pub confidence: f32,
    /// Other applicable Academy reasons for the same token, if any.
    pub alternate_reasons: Vec<DiagnosticReason>,
}

/// An alternate applicable reason for the same token.
pub type DiagnosticReason = varnavinyas_prakriya::Explanation;

pub fn diagnostic_reason_category(reason: &DiagnosticReason) -> DiagnosticCategory {
    choose_diagnostic_category(reason.category, &reason.rule)
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
