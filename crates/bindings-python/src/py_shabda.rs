use pyo3::prelude::*;
use varnavinyas_shabda::{self as shabda_core, AffixKind, Origin};

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

#[pyclass(name = "AffixKind", eq, frozen, hash)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PyAffixKind {
    Prefix,
    PluralMarker,
    CaseMarker,
    Particle,
}

impl From<AffixKind> for PyAffixKind {
    fn from(kind: AffixKind) -> Self {
        match kind {
            AffixKind::Prefix => PyAffixKind::Prefix,
            AffixKind::PluralMarker => PyAffixKind::PluralMarker,
            AffixKind::CaseMarker => PyAffixKind::CaseMarker,
            AffixKind::Particle => PyAffixKind::Particle,
        }
    }
}

#[pyclass(name = "AffixSegment", get_all, frozen)]
#[derive(Clone)]
pub struct PyAffixSegment {
    pub text: String,
    pub kind: PyAffixKind,
}

#[pyclass(name = "AffixAnalysis", get_all, frozen)]
#[derive(Clone)]
pub struct PyAffixAnalysis {
    pub surface: String,
    pub stem: String,
    pub root: String,
    pub prefixes: Vec<String>,
    pub prefix_segments: Vec<PyAffixSegment>,
    pub suffixes: Vec<String>,
    pub suffix_segments: Vec<PyAffixSegment>,
    pub score: u16,
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

impl From<shabda_core::AffixSegment> for PyAffixSegment {
    fn from(segment: shabda_core::AffixSegment) -> Self {
        Self {
            text: segment.text,
            kind: segment.kind.into(),
        }
    }
}

impl From<shabda_core::AffixAnalysis> for PyAffixAnalysis {
    fn from(analysis: shabda_core::AffixAnalysis) -> Self {
        Self {
            surface: analysis.surface,
            stem: analysis.stem,
            root: analysis.root,
            prefixes: analysis.prefixes,
            prefix_segments: analysis
                .prefix_segments
                .into_iter()
                .map(Into::into)
                .collect(),
            suffixes: analysis.suffixes,
            suffix_segments: analysis
                .suffix_segments
                .into_iter()
                .map(Into::into)
                .collect(),
            score: analysis.score,
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

/// Collect conservative affix analyses for a word.
#[pyfunction]
pub fn analyze_affixes(word: &str) -> Vec<PyAffixAnalysis> {
    shabda_core::analyze_affixes(word)
        .into_iter()
        .map(Into::into)
        .collect()
}

/// Return the best affix analysis for a word.
#[pyfunction]
pub fn best_analysis(word: &str) -> Option<PyAffixAnalysis> {
    shabda_core::best_analysis(word).map(Into::into)
}

/// Return whether the word has any supported affix analysis.
#[pyfunction]
pub fn has_supported_analysis(word: &str) -> bool {
    shabda_core::has_supported_analysis(word)
}

#[pymodule]
pub fn shabda(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyOrigin>()?;
    m.add_class::<PyAffixKind>()?;
    m.add_class::<PyAffixSegment>()?;
    m.add_class::<PyAffixAnalysis>()?;
    m.add_class::<PyMorpheme>()?;
    m.add_class::<PyRootCandidate>()?;
    m.add_function(wrap_pyfunction!(classify, m)?)?;
    m.add_function(wrap_pyfunction!(decompose, m)?)?;
    m.add_function(wrap_pyfunction!(lookup_root_candidates, m)?)?;
    m.add_function(wrap_pyfunction!(has_known_root, m)?)?;
    m.add_function(wrap_pyfunction!(best_root, m)?)?;
    m.add_function(wrap_pyfunction!(analyze_affixes, m)?)?;
    m.add_function(wrap_pyfunction!(best_analysis, m)?)?;
    m.add_function(wrap_pyfunction!(has_supported_analysis, m)?)?;
    Ok(())
}
