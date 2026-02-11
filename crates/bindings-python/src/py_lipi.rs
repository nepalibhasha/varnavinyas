use pyo3::prelude::*;
use varnavinyas_lipi::{self as lipi_core, Scheme};

#[pyclass(name = "Scheme", eq, frozen, hash)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PyScheme {
    Devanagari,
    Iast,
}

impl From<PyScheme> for Scheme {
    fn from(s: PyScheme) -> Self {
        match s {
            PyScheme::Devanagari => Scheme::Devanagari,
            PyScheme::Iast => Scheme::Iast,
        }
    }
}

impl From<Scheme> for PyScheme {
    fn from(s: Scheme) -> Self {
        match s {
            Scheme::Devanagari => PyScheme::Devanagari,
            Scheme::Iast => PyScheme::Iast,
        }
    }
}

#[pymethods]
impl PyScheme {
    fn __repr__(&self) -> String {
        match self {
            PyScheme::Devanagari => "Scheme.Devanagari".to_string(),
            PyScheme::Iast => "Scheme.Iast".to_string(),
        }
    }
}

/// Transliterate text between scripts.
#[pyfunction]
pub fn transliterate(input: &str, from: PyScheme, to: PyScheme) -> PyResult<String> {
    lipi_core::transliterate(input, from.into(), to.into())
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

/// Detect the script of input text.
#[pyfunction]
pub fn detect_scheme(input: &str) -> Option<PyScheme> {
    lipi_core::detect_scheme(input).map(|s| s.into())
}

#[pymodule]
pub fn lipi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyScheme>()?;
    m.add_function(wrap_pyfunction!(transliterate, m)?)?;
    m.add_function(wrap_pyfunction!(detect_scheme, m)?)?;
    Ok(())
}
