use crate::rule::Rule;

/// A single step in a derivation.
#[derive(Debug, Clone)]
pub struct Step {
    /// The rule that was applied.
    pub rule: Rule,
    /// Human-readable description of what happened.
    pub description: String,
    /// Text before this step.
    pub before: String,
    /// Text after this step.
    pub after: String,
}

impl Step {
    pub fn new(
        rule: Rule,
        description: impl Into<String>,
        before: impl Into<String>,
        after: impl Into<String>,
    ) -> Self {
        Self {
            rule,
            description: description.into(),
            before: before.into(),
            after: after.into(),
        }
    }
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} → {} ({})",
            self.rule, self.before, self.after, self.description
        )
    }
}
