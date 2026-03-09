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
    pub rule_code: String,
    pub explanation: String,
    pub category: String,
    pub category_code: String,
    pub kind: String,
    pub confidence: f32,
    pub alternate_reasons: Vec<PyDiagnosticReason>,
}

#[pyclass(name = "DiagnosticReason", get_all, frozen)]
#[derive(Clone)]
pub struct PyDiagnosticReason {
    pub rule: PyRule,
    pub rule_code: String,
    pub explanation: String,
    pub category: String,
    pub category_code: String,
    pub correction: String,
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
    parikshak_core::check_word(word).map(py_diagnostic_from_core)
}

/// Check a full text for spelling and punctuation issues.
/// Returns a list of Diagnostic objects.
#[pyfunction]
pub fn check_text(text: &str) -> PyResult<Vec<PyDiagnostic>> {
    check_text_with_options(text, false, "strict", false)
}

fn parse_punctuation_mode(mode: &str) -> PyResult<varnavinyas_parikshak::PunctuationMode> {
    match mode {
        "strict" => Ok(varnavinyas_parikshak::PunctuationMode::Strict),
        "normalized_editorial" => Ok(varnavinyas_parikshak::PunctuationMode::NormalizedEditorial),
        other => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Unknown punctuation_mode '{other}'. Use 'strict' or 'normalized_editorial'."
        ))),
    }
}

/// Check full text with runtime options.
#[pyfunction]
#[pyo3(signature = (text, grammar=false, punctuation_mode="strict", include_noop_heuristics=false))]
pub fn check_text_with_options(
    text: &str,
    grammar: bool,
    punctuation_mode: &str,
    include_noop_heuristics: bool,
) -> PyResult<Vec<PyDiagnostic>> {
    let punctuation_mode = parse_punctuation_mode(punctuation_mode)?;
    let diagnostics = parikshak_core::check_text_with_options(
        text,
        parikshak_core::CheckOptions {
            grammar,
            punctuation_mode,
            include_noop_heuristics,
        },
    );
    Ok(diagnostics
        .into_iter()
        .map(py_diagnostic_from_core)
        .collect())
}

fn py_diagnostic_from_core(d: parikshak_core::Diagnostic) -> PyDiagnostic {
    PyDiagnostic {
        span_start: d.span.0,
        span_end: d.span.1,
        incorrect: d.incorrect,
        correction: d.correction,
        rule_code: d.rule.code().to_string(),
        rule: d.rule.into(),
        explanation: d.explanation,
        category: d.category.to_string(),
        category_code: d.category.as_code().to_string(),
        kind: d.kind.as_code().to_string(),
        confidence: d.confidence,
        alternate_reasons: d
            .alternate_reasons
            .into_iter()
            .map(|r| PyDiagnosticReason {
                rule_code: r.rule.code().to_string(),
                rule: r.rule.into(),
                explanation: r.explanation,
                category: r.category.to_string(),
                category_code: r.category.as_code().to_string(),
                correction: r.correction,
            })
            .collect(),
    }
}

#[pymodule]
pub fn parikshak(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDiagnostic>()?;
    m.add_class::<PyDiagnosticReason>()?;
    m.add_function(wrap_pyfunction!(check_word, m)?)?;
    m.add_function(wrap_pyfunction!(check_text, m)?)?;
    m.add_function(wrap_pyfunction!(check_text_with_options, m)?)?;
    Ok(())
}
