use varnavinyas_lekhya::check_punctuation;
use varnavinyas_prakriya::{DiagnosticKind, Rule};

use crate::diagnostic::{Diagnostic, DiagnosticCategory};

pub(crate) fn punctuation_diagnostics(
    text: &str,
    kind: DiagnosticKind,
    confidence: f32,
) -> Vec<Diagnostic> {
    check_punctuation(text)
        .into_iter()
        .map(|lekhya_diag| Diagnostic {
            span: lekhya_diag.span,
            incorrect: lekhya_diag.found,
            correction: lekhya_diag.expected,
            rule: Rule::ChihnaNiyam("Section 5"),
            explanation: lekhya_diag.rule.to_string(),
            category: DiagnosticCategory::Punctuation,
            kind,
            confidence,
            alternate_reasons: Vec::new(),
        })
        .collect()
}
