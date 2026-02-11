use pyo3::prelude::*;
use varnavinyas_kosha as kosha_core;

#[pyclass(name = "WordEntry", get_all, frozen)]
#[derive(Clone)]
pub struct PyWordEntry {
    pub word: String,
    pub pos: String,
}

#[pymethods]
impl PyWordEntry {
    fn __repr__(&self) -> String {
        format!("WordEntry(word='{}', pos='{}')", self.word, self.pos)
    }
}

/// Check if a word exists in the lexicon.
#[pyfunction]
pub fn contains(word: &str) -> bool {
    kosha_core::kosha().contains(word)
}

/// Look up headword metadata (POS tags).
/// Returns a WordEntry or None.
#[pyfunction]
pub fn lookup(word: &str) -> Option<PyWordEntry> {
    kosha_core::kosha().lookup(word).map(|e| PyWordEntry {
        word: e.word.to_string(),
        pos: e.pos.to_string(),
    })
}

/// Return the number of word forms in the lexicon.
#[pyfunction]
pub fn word_count() -> usize {
    kosha_core::kosha().word_count()
}

/// Return the number of headwords with metadata.
#[pyfunction]
pub fn headword_count() -> usize {
    kosha_core::kosha().headword_count()
}

#[pymodule]
pub fn kosha(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyWordEntry>()?;
    m.add_function(wrap_pyfunction!(contains, m)?)?;
    m.add_function(wrap_pyfunction!(lookup, m)?)?;
    m.add_function(wrap_pyfunction!(word_count, m)?)?;
    m.add_function(wrap_pyfunction!(headword_count, m)?)?;
    Ok(())
}
