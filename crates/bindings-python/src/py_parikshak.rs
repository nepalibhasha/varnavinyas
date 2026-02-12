use pyo3::prelude::*;
use varnavinyas_parikshak as parikshak_core;

use crate::py_prakriya::PyRule;

#[pyclass(name = "Diagnostic", get_all, frozen)]
#[derive(Clone)]
pub struct PyDiagnostic {
    pub span_start: usize,
    pub span_end: usize,
    pub incorrect: String,
    pub correction: String,
    pub rule: PyRule,
    pub explanation: String,
    pub category: String,
    pub kind: String,
    pub confidence: f32,
}

#[pymethods]
impl PyDiagnostic {
    fn __repr__(&self) -> String {
        format!(
            "Diagnostic(incorrect='{}', correction='{}', category='{}')",
            self.incorrect, self.correction, self.category
        )
    }

    fn __str__(&self) -> String {
        format!(
            "[{}] {} → {} ({})",
            self.category, self.incorrect, self.correction, self.explanation
        )
    }
}

/// Check a single word.
/// Returns a Diagnostic or None.
#[pyfunction]
pub fn check_word(word: &str) -> Option<PyDiagnostic> {
    parikshak_core::check_word(word).map(|d| PyDiagnostic {
        span_start: d.span.0,
        span_end: d.span.1,
        incorrect: d.incorrect,
        correction: d.correction,
        rule: d.rule.into(),
        explanation: d.explanation,
        category: d.category.to_string(),
        kind: d.kind.as_code().to_string(),
        confidence: d.confidence,
    })
}

/// Check a full text for spelling and punctuation issues.
/// Returns a list of Diagnostic objects.
#[pyfunction]
pub fn check_text(text: &str) -> Vec<PyDiagnostic> {
    parikshak_core::check_text(text)
        .into_iter()
        .map(|d| PyDiagnostic {
            span_start: d.span.0,
            span_end: d.span.1,
            incorrect: d.incorrect,
            correction: d.correction,
            rule: d.rule.into(),
            explanation: d.explanation,
            category: d.category.to_string(),
            kind: d.kind.as_code().to_string(),
            confidence: d.confidence,
        })
        .collect()
}

#[pymodule]
pub fn parikshak(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDiagnostic>()?;
    m.add_function(wrap_pyfunction!(check_word, m)?)?;
    m.add_function(wrap_pyfunction!(check_text, m)?)?;
    Ok(())
}
