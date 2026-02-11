use pyo3::prelude::*;
use varnavinyas_lekhya as lekhya_core;

#[pyclass(name = "LekhyaDiagnostic", get_all, frozen)]
#[derive(Clone)]
pub struct PyLekhyaDiagnostic {
    pub span_start: usize,
    pub span_end: usize,
    pub found: String,
    pub expected: String,
    pub rule: String,
}

#[pymethods]
impl PyLekhyaDiagnostic {
    fn __repr__(&self) -> String {
        format!(
            "LekhyaDiagnostic(found='{}', expected='{}', span=({}, {}))",
            self.found, self.expected, self.span_start, self.span_end
        )
    }

    fn __str__(&self) -> String {
        format!("{} → {} ({})", self.found, self.expected, self.rule)
    }
}

/// Check text for punctuation issues.
#[pyfunction]
pub fn check_punctuation(text: &str) -> Vec<PyLekhyaDiagnostic> {
    lekhya_core::check_punctuation(text)
        .into_iter()
        .map(|d| PyLekhyaDiagnostic {
            span_start: d.span.0,
            span_end: d.span.1,
            found: d.found,
            expected: d.expected,
            rule: d.rule.to_string(),
        })
        .collect()
}

#[pymodule]
pub fn lekhya(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyLekhyaDiagnostic>()?;
    m.add_function(wrap_pyfunction!(check_punctuation, m)?)?;
    Ok(())
}
