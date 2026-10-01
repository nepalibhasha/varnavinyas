use pyo3::prelude::*;
use varnavinyas_shabda::{self as shabda_core, AffixKind, Origin};

#[pyclass(from_py_object, name = "Origin", eq, frozen, hash)]
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

#[pyclass(from_py_object, name = "Morpheme", get_all, frozen)]
#[derive(Clone)]
pub struct PyMorpheme {
    pub root: String,
    pub prefixes: Vec<String>,
    pub suffixes: Vec<String>,
    pub origin: PyOrigin,
}

/// Dictionary-backed main verb and auxiliary reading.
#[pyclass(from_py_object, name = "ProgressiveAnalysis", get_all, frozen)]
#[derive(Clone)]
pub struct PyProgressiveAnalysis {
    pub surface: String,
    pub main_form: String,
    pub main_lemma: String,
    pub auxiliary_form: String,
    pub auxiliary_lemma: String,
    pub negative: bool,
}

#[pyfunction]
pub fn analyze_progressive(word: &str) -> Option<PyProgressiveAnalysis> {
    shabda_core::analyze_progressive(word).map(|analysis| PyProgressiveAnalysis {
        surface: analysis.surface,
        main_form: analysis.main_form,
        main_lemma: analysis.main_lemma,
        auxiliary_form: analysis.auxiliary_form,
        auxiliary_lemma: analysis.auxiliary_lemma,
        negative: analysis.negative,
    })
}

/// Evidence source; Unknown means no origin evidence is available.
#[pyclass(from_py_object, name = "OriginSource", eq, frozen, hash)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PyOriginSource {
    Override,
    Kosha,
    Heuristic,
    Unknown,
}

impl From<shabda_core::OriginSource> for PyOriginSource {
    fn from(source: shabda_core::OriginSource) -> Self {
        match source {
            shabda_core::OriginSource::Override => Self::Override,
            shabda_core::OriginSource::Kosha => Self::Kosha,
            shabda_core::OriginSource::Heuristic => Self::Heuristic,
            shabda_core::OriginSource::Unknown => Self::Unknown,
        }
    }
}

/// Origin with evidence. `origin` is None when `source` is Unknown.
#[pyclass(from_py_object, name = "OriginDecision", get_all, frozen)]
#[derive(Clone)]
pub struct PyOriginDecision {
    pub origin: Option<PyOrigin>,
    pub source: PyOriginSource,
    pub confidence: f32,
}

/// Classify with evidence; missing evidence is never presented as Deshaj.
#[pyfunction]
pub fn classify_with_provenance(word: &str) -> PyOriginDecision {
    let decision = shabda_core::classify_with_provenance(word);
    PyOriginDecision {
        origin: decision.supported_origin().map(Into::into),
        source: decision.source.into(),
        confidence: decision.confidence,
    }
}

#[pyclass(from_py_object, name = "AffixKind", eq, frozen, hash)]
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

#[pyclass(from_py_object, name = "AffixSegment", get_all, frozen)]
#[derive(Clone)]
pub struct PyAffixSegment {
    pub text: String,
    pub kind: PyAffixKind,
}

#[pyclass(from_py_object, name = "AffixAnalysis", get_all, frozen)]
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

#[pyclass(from_py_object, name = "RootCandidate", get_all, frozen)]
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
    m.add_class::<PyOriginSource>()?;
    m.add_class::<PyOriginDecision>()?;
    m.add_class::<PyAffixKind>()?;
    m.add_class::<PyAffixSegment>()?;
    m.add_class::<PyAffixAnalysis>()?;
    m.add_class::<PyMorpheme>()?;
    m.add_class::<PyProgressiveAnalysis>()?;
    m.add_class::<PyRootCandidate>()?;
    m.add_function(wrap_pyfunction!(classify, m)?)?;
    m.add_function(wrap_pyfunction!(analyze_progressive, m)?)?;
    m.add_function(wrap_pyfunction!(classify_with_provenance, m)?)?;
    m.add_function(wrap_pyfunction!(decompose, m)?)?;
    m.add_function(wrap_pyfunction!(lookup_root_candidates, m)?)?;
    m.add_function(wrap_pyfunction!(has_known_root, m)?)?;
    m.add_function(wrap_pyfunction!(best_root, m)?)?;
    m.add_function(wrap_pyfunction!(analyze_affixes, m)?)?;
    m.add_function(wrap_pyfunction!(best_analysis, m)?)?;
    m.add_function(wrap_pyfunction!(has_supported_analysis, m)?)?;
    Ok(())
}
