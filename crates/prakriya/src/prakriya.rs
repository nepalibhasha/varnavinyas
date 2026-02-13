use crate::rule_spec::{DiagnosticKind, RuleCategory};
use crate::step::Step;

/// The derivation state, tracking history.
#[derive(Debug, Clone)]
pub struct Prakriya {
    /// The original input word.
    pub input: String,
    /// The corrected output word.
    pub output: String,
    /// Steps applied during derivation.
    pub steps: Vec<Step>,
    /// Whether the input was already correct.
    pub is_correct: bool,
    /// Typed diagnostic category propagated from rule metadata.
    pub category: Option<RuleCategory>,
    /// Typed diagnostic severity propagated from rule metadata.
    pub kind: DiagnosticKind,
}

impl Prakriya {
    /// Create a new Prakriya indicating the word is already correct.
    pub fn correct(word: &str) -> Self {
        Self {
            input: word.to_string(),
            output: word.to_string(),
            steps: Vec::new(),
            is_correct: true,
            category: None,
            kind: DiagnosticKind::Error,
        }
    }

    /// Create a new Prakriya with a correction.
    pub fn corrected(input: &str, output: &str, steps: Vec<Step>) -> Self {
        Self {
            input: input.to_string(),
            output: output.to_string(),
            steps,
            is_correct: false,
            category: None,
            kind: DiagnosticKind::Error,
        }
    }

    /// Attach typed metadata from rule dispatch.
    pub fn with_metadata(mut self, category: RuleCategory, kind: DiagnosticKind) -> Self {
        self.category = Some(category);
        self.kind = kind;
        self
    }
}

impl std::fmt::Display for Prakriya {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_correct {
            write!(f, "✓ {} (correct)", self.input)
        } else {
            writeln!(f, "✗ {} → {}", self.input, self.output)?;
            for (i, step) in self.steps.iter().enumerate() {
                writeln!(f, "  Step {}: {step}", i + 1)?;
            }
            Ok(())
        }
    }
}
