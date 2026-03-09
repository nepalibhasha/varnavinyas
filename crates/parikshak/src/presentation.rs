use serde::Serialize;

use crate::diagnostic::{Diagnostic, DiagnosticReason, diagnostic_reason_category};

/// Stable serializable diagnostic shape for JSON/binding adapters.
#[derive(Debug, Clone, Serialize)]
pub struct ApiDiagnostic {
    pub span_start: usize,
    pub span_end: usize,
    pub incorrect: String,
    pub correction: String,
    pub rule: String,
    pub rule_code: String,
    pub explanation: String,
    pub category: String,
    pub category_code: String,
    pub kind: String,
    pub confidence: f32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_reasons: Vec<ApiDiagnosticReason>,
}

/// Stable serializable alternate diagnostic reason shape.
#[derive(Debug, Clone, Serialize)]
pub struct ApiDiagnosticReason {
    pub rule: String,
    pub rule_code: String,
    pub explanation: String,
    pub category: String,
    pub category_code: String,
    pub correction: String,
}

impl From<&DiagnosticReason> for ApiDiagnosticReason {
    fn from(reason: &DiagnosticReason) -> Self {
        let category = diagnostic_reason_category(reason);
        Self {
            rule: reason.rule.to_string(),
            rule_code: reason.rule.code().to_string(),
            explanation: reason.explanation.clone(),
            category: category.to_string(),
            category_code: category.as_code().to_string(),
            correction: reason.correction.clone().unwrap_or_default(),
        }
    }
}

impl From<DiagnosticReason> for ApiDiagnosticReason {
    fn from(reason: DiagnosticReason) -> Self {
        Self::from(&reason)
    }
}

impl From<&Diagnostic> for ApiDiagnostic {
    fn from(diagnostic: &Diagnostic) -> Self {
        Self {
            span_start: diagnostic.span.0,
            span_end: diagnostic.span.1,
            incorrect: diagnostic.incorrect.clone(),
            correction: diagnostic.correction.clone(),
            rule: diagnostic.rule.to_string(),
            rule_code: diagnostic.rule.code().to_string(),
            explanation: diagnostic.explanation.clone(),
            category: diagnostic.category.to_string(),
            category_code: diagnostic.category.as_code().to_string(),
            kind: diagnostic.kind.as_code().to_string(),
            confidence: diagnostic.confidence,
            alternate_reasons: diagnostic
                .alternate_reasons
                .iter()
                .map(ApiDiagnosticReason::from)
                .collect(),
        }
    }
}

impl From<Diagnostic> for ApiDiagnostic {
    fn from(diagnostic: Diagnostic) -> Self {
        Self::from(&diagnostic)
    }
}
