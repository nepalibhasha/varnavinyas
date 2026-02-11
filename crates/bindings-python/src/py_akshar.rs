use pyo3::prelude::*;
use varnavinyas_akshar::{self as akshar_core, CharType, SvarType};

#[pyclass(name = "CharType", eq, frozen, hash)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PyCharType {
    Svar,
    Vyanjan,
    Matra,
    Halanta,
    Chandrabindu,
    Shirbindu,
    Visarga,
    Nukta,
    Avagraha,
    Numeral,
    Danda,
    OtherMark,
}

impl From<CharType> for PyCharType {
    fn from(ct: CharType) -> Self {
        match ct {
            CharType::Svar => PyCharType::Svar,
            CharType::Vyanjan => PyCharType::Vyanjan,
            CharType::Matra => PyCharType::Matra,
            CharType::Halanta => PyCharType::Halanta,
            CharType::Chandrabindu => PyCharType::Chandrabindu,
            CharType::Shirbindu => PyCharType::Shirbindu,
            CharType::Visarga => PyCharType::Visarga,
            CharType::Nukta => PyCharType::Nukta,
            CharType::Avagraha => PyCharType::Avagraha,
            CharType::Numeral => PyCharType::Numeral,
            CharType::Danda => PyCharType::Danda,
            CharType::OtherMark => PyCharType::OtherMark,
        }
    }
}

#[pymethods]
impl PyCharType {
    fn __repr__(&self) -> String {
        match self {
            PyCharType::Svar => "CharType.Svar".to_string(),
            PyCharType::Vyanjan => "CharType.Vyanjan".to_string(),
            PyCharType::Matra => "CharType.Matra".to_string(),
            PyCharType::Halanta => "CharType.Halanta".to_string(),
            PyCharType::Chandrabindu => "CharType.Chandrabindu".to_string(),
            PyCharType::Shirbindu => "CharType.Shirbindu".to_string(),
            PyCharType::Visarga => "CharType.Visarga".to_string(),
            PyCharType::Nukta => "CharType.Nukta".to_string(),
            PyCharType::Avagraha => "CharType.Avagraha".to_string(),
            PyCharType::Numeral => "CharType.Numeral".to_string(),
            PyCharType::Danda => "CharType.Danda".to_string(),
            PyCharType::OtherMark => "CharType.OtherMark".to_string(),
        }
    }
}

#[pyclass(name = "SvarType", eq, frozen, hash)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PySvarType {
    Hrasva,
    Dirgha,
}

impl From<SvarType> for PySvarType {
    fn from(st: SvarType) -> Self {
        match st {
            SvarType::Hrasva => PySvarType::Hrasva,
            SvarType::Dirgha => PySvarType::Dirgha,
        }
    }
}

#[pymethods]
impl PySvarType {
    fn __repr__(&self) -> String {
        match self {
            PySvarType::Hrasva => "SvarType.Hrasva".to_string(),
            PySvarType::Dirgha => "SvarType.Dirgha".to_string(),
        }
    }
}

#[pyclass(name = "Akshara", get_all, frozen)]
#[derive(Clone)]
pub struct PyAkshara {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

#[pymethods]
impl PyAkshara {
    fn __repr__(&self) -> String {
        format!("Akshara('{}')", self.text)
    }

    fn __str__(&self) -> &str {
        &self.text
    }
}

/// Classify a Devanagari character.
#[pyfunction]
pub fn classify(c: char) -> Option<PyCharType> {
    akshar_core::classify(c).map(|dc| dc.char_type.into())
}

/// Check if a character is a vowel (स्वर).
#[pyfunction]
pub fn is_svar(c: char) -> bool {
    akshar_core::is_svar(c)
}

/// Check if a character is a consonant (व्यञ्जन).
#[pyfunction]
pub fn is_vyanjan(c: char) -> bool {
    akshar_core::is_vyanjan(c)
}

/// Determine vowel type (hrasva/dirgha).
#[pyfunction]
pub fn svar_type(c: char) -> Option<PySvarType> {
    akshar_core::svar_type(c).map(|st| st.into())
}

/// Split text into aksharas (syllable units).
#[pyfunction]
pub fn split_aksharas(text: &str) -> Vec<PyAkshara> {
    akshar_core::split_aksharas(text)
        .into_iter()
        .map(|a| PyAkshara {
            text: a.text,
            start: a.start,
            end: a.end,
        })
        .collect()
}

/// Normalize Devanagari text.
#[pyfunction]
pub fn normalize(text: &str) -> String {
    akshar_core::normalize(text)
}

#[pymodule]
pub fn akshar(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCharType>()?;
    m.add_class::<PySvarType>()?;
    m.add_class::<PyAkshara>()?;
    m.add_function(wrap_pyfunction!(classify, m)?)?;
    m.add_function(wrap_pyfunction!(is_svar, m)?)?;
    m.add_function(wrap_pyfunction!(is_vyanjan, m)?)?;
    m.add_function(wrap_pyfunction!(svar_type, m)?)?;
    m.add_function(wrap_pyfunction!(split_aksharas, m)?)?;
    m.add_function(wrap_pyfunction!(normalize, m)?)?;
    Ok(())
}
