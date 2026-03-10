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

#[pyclass(name = "RootCandidate", get_all, frozen)]
#[derive(Clone)]
pub struct PyRootCandidate {
    pub root: String,
    pub prefixes: Vec<String>,
    pub suffixes: Vec<String>,
    pub origin: PyOrigin,
    pub known_word: bool,
    pub known_headword: bool,
    pub score: u16,
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

#[pymethods]
impl PyRootCandidate {
    fn __repr__(&self) -> String {
        format!(
            "RootCandidate(root='{}', prefixes={:?}, suffixes={:?}, known_word={}, known_headword={}, score={})",
            self.root,
            self.prefixes,
            self.suffixes,
            self.known_word,
            self.known_headword,
            self.score,
        )
    }
}

impl From<shabda_core::RootCandidate> for PyRootCandidate {
    fn from(candidate: shabda_core::RootCandidate) -> Self {
        Self {
            root: candidate.root,
            prefixes: candidate.prefixes,
            suffixes: candidate.suffixes,
            origin: candidate.origin.into(),
            known_word: candidate.known_word,
            known_headword: candidate.known_headword,
            score: candidate.score,
        }
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

/// Generate lexicon-backed root candidates for a word.
#[pyfunction]
pub fn lookup_root_candidates(word: &str) -> Vec<PyRootCandidate> {
    shabda_core::lookup_root_candidates(word)
        .into_iter()
        .map(Into::into)
        .collect()
}

/// Return whether the word has at least one known root candidate.
#[pyfunction]
pub fn has_known_root(word: &str) -> bool {
    shabda_core::has_known_root(word)
}

/// Return the highest-ranked root candidate for a word.
#[pyfunction]
pub fn best_root(word: &str) -> Option<PyRootCandidate> {
    shabda_core::best_root(word).map(Into::into)
}

#[pymodule]
pub fn shabda(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyOrigin>()?;
    m.add_class::<PyMorpheme>()?;
    m.add_class::<PyRootCandidate>()?;
    m.add_function(wrap_pyfunction!(classify, m)?)?;
    m.add_function(wrap_pyfunction!(decompose, m)?)?;
    m.add_function(wrap_pyfunction!(lookup_root_candidates, m)?)?;
    m.add_function(wrap_pyfunction!(has_known_root, m)?)?;
    m.add_function(wrap_pyfunction!(best_root, m)?)?;
    Ok(())
}
