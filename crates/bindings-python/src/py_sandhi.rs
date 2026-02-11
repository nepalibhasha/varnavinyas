use pyo3::prelude::*;
use varnavinyas_sandhi::{self as sandhi_core, SandhiType};

#[pyclass(name = "SandhiType", eq, frozen, hash)]
#[derive(Clone, PartialEq, Eq, Hash)]
#[allow(clippy::enum_variant_names)]
pub enum PySandhiType {
    VowelSandhi,
    VisargaSandhi,
    ConsonantSandhi,
}

impl From<SandhiType> for PySandhiType {
    fn from(st: SandhiType) -> Self {
        match st {
            SandhiType::VowelSandhi => PySandhiType::VowelSandhi,
            SandhiType::VisargaSandhi => PySandhiType::VisargaSandhi,
            SandhiType::ConsonantSandhi => PySandhiType::ConsonantSandhi,
        }
    }
}

#[pymethods]
impl PySandhiType {
    fn __repr__(&self) -> String {
        match self {
            PySandhiType::VowelSandhi => "SandhiType.VowelSandhi".to_string(),
            PySandhiType::VisargaSandhi => "SandhiType.VisargaSandhi".to_string(),
            PySandhiType::ConsonantSandhi => "SandhiType.ConsonantSandhi".to_string(),
        }
    }
}

#[pyclass(name = "SandhiResult", get_all, frozen)]
#[derive(Clone)]
pub struct PySandhiResult {
    pub output: String,
    pub sandhi_type: PySandhiType,
    pub rule_citation: String,
}

#[pymethods]
impl PySandhiResult {
    fn __repr__(&self) -> String {
        format!(
            "SandhiResult(output='{}', type={}, rule='{}')",
            self.output,
            self.sandhi_type.__repr__(),
            self.rule_citation,
        )
    }
}

/// Apply sandhi to combine two morphemes.
#[pyfunction]
pub fn apply(first: &str, second: &str) -> PyResult<PySandhiResult> {
    sandhi_core::apply(first, second)
        .map(|r| PySandhiResult {
            output: r.output,
            sandhi_type: r.sandhi_type.into(),
            rule_citation: r.rule_citation.to_string(),
        })
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

/// Split a word at sandhi boundaries.
#[pyfunction]
pub fn split(word: &str) -> Vec<(String, String, PySandhiResult)> {
    sandhi_core::split(word)
        .into_iter()
        .map(|(first, second, result)| {
            (
                first,
                second,
                PySandhiResult {
                    output: result.output,
                    sandhi_type: result.sandhi_type.into(),
                    rule_citation: result.rule_citation.to_string(),
                },
            )
        })
        .collect()
}

#[pymodule]
pub fn sandhi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySandhiType>()?;
    m.add_class::<PySandhiResult>()?;
    m.add_function(wrap_pyfunction!(apply, m)?)?;
    m.add_function(wrap_pyfunction!(split, m)?)?;
    Ok(())
}
