use pyo3::prelude::*;
use varnavinyas_shabda::{self as shabda_core, Origin};

#[pyclass(name = "Origin", eq, frozen, hash)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PyOrigin {
    Tatsam,
    Tadbhav,
    Deshaj,
    Aagantuk,
}

impl From<Origin> for PyOrigin {
    fn from(o: Origin) -> Self {
        match o {
            Origin::Tatsam => PyOrigin::Tatsam,
            Origin::Tadbhav => PyOrigin::Tadbhav,
            Origin::Deshaj => PyOrigin::Deshaj,
            Origin::Aagantuk => PyOrigin::Aagantuk,
        }
    }
}

#[pymethods]
impl PyOrigin {
    fn __repr__(&self) -> String {
        match self {
            PyOrigin::Tatsam => "Origin.Tatsam".to_string(),
            PyOrigin::Tadbhav => "Origin.Tadbhav".to_string(),
            PyOrigin::Deshaj => "Origin.Deshaj".to_string(),
            PyOrigin::Aagantuk => "Origin.Aagantuk".to_string(),
        }
    }
}

#[pyclass(name = "Morpheme", get_all, frozen)]
#[derive(Clone)]
pub struct PyMorpheme {
    pub root: String,
    pub prefixes: Vec<String>,
    pub suffixes: Vec<String>,
    pub origin: PyOrigin,
}

#[pymethods]
impl PyMorpheme {
    fn __repr__(&self) -> String {
        format!(
            "Morpheme(root='{}', prefixes={:?}, suffixes={:?}, origin={:?})",
            self.root,
            self.prefixes,
            self.suffixes,
            self.origin.__repr__(),
        )
    }
}

/// Classify a word by its origin.
#[pyfunction]
pub fn classify(word: &str) -> PyOrigin {
    shabda_core::classify(word).into()
}

/// Decompose a word into morphological components.
#[pyfunction]
pub fn decompose(word: &str) -> PyMorpheme {
    let m = shabda_core::decompose(word);
    PyMorpheme {
        root: m.root,
        prefixes: m.prefixes,
        suffixes: m.suffixes,
        origin: m.origin.into(),
    }
}

#[pymodule]
pub fn shabda(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyOrigin>()?;
    m.add_class::<PyMorpheme>()?;
    m.add_function(wrap_pyfunction!(classify, m)?)?;
    m.add_function(wrap_pyfunction!(decompose, m)?)?;
    Ok(())
}
