use crate::model::rule::Rule;
use crate::model::rule_spec::RuleCategory;

/// Shared outward-facing explanation for rule-based analysis surfaces.
#[derive(Debug, Clone)]
pub struct Explanation {
    /// The cited rule.
    pub rule: Rule,
    /// Human-readable explanation.
    pub explanation: String,
    /// Suggested correction for this explanation, if any.
    pub correction: Option<String>,
    /// Typed rule category when known.
    pub category: Option<RuleCategory>,
}

impl Explanation {
    pub fn new(rule: Rule, explanation: impl Into<String>) -> Self {
        Self {
            rule,
            explanation: explanation.into(),
            correction: None,
            category: None,
        }
    }

    pub fn with_correction(mut self, correction: impl Into<String>) -> Self {
        self.correction = Some(correction.into());
        self
    }

    pub fn with_category(mut self, category: RuleCategory) -> Self {
        self.category = Some(category);
        self
    }
}
