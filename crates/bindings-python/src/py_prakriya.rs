use pyo3::prelude::*;
use varnavinyas_prakriya::{self as prakriya_core, Rule};

#[pyclass(name = "Rule", get_all, frozen)]
#[derive(Clone)]
pub struct PyRule {
    pub source: String,
    pub code: String,
}

impl From<Rule> for PyRule {
    fn from(r: Rule) -> Self {
        PyRule {
            source: r.source_name().to_string(),
            code: r.code().to_string(),
        }
    }
}

#[pymethods]
impl PyRule {
    fn __repr__(&self) -> String {
        format!("Rule(source='{}', code='{}')", self.source, self.code)
    }

    fn __str__(&self) -> String {
        format!("{}: {}", self.source, self.code)
    }
}

#[pyclass(name = "Step", get_all, frozen)]
#[derive(Clone)]
pub struct PyStep {
    pub rule: PyRule,
    pub description: String,
    pub before: String,
    pub after: String,
}

#[pymethods]
impl PyStep {
    fn __repr__(&self) -> String {
        format!(
            "Step(rule={}, '{}' → '{}')",
            self.rule.__repr__(),
            self.before,
            self.after,
        )
    }
}

#[pyclass(name = "Prakriya", get_all, frozen)]
#[derive(Clone)]
pub struct PyPrakriya {
    pub input: String,
    pub output: String,
    pub steps: Vec<PyStep>,
    pub is_correct: bool,
}

#[pymethods]
impl PyPrakriya {
    fn __repr__(&self) -> String {
        if self.is_correct {
            format!("Prakriya(input='{}', correct=True)", self.input)
        } else {
            format!(
                "Prakriya(input='{}', output='{}', steps={})",
                self.input,
                self.output,
                self.steps.len(),
            )
        }
    }

    fn __str__(&self) -> String {
        if self.is_correct {
            format!("✓ {} (correct)", self.input)
        } else {
            format!("✗ {} → {}", self.input, self.output)
        }
    }
}

/// Derive the correct form of a word with rule tracing.
#[pyfunction]
pub fn derive(input: &str) -> PyPrakriya {
    let p = prakriya_core::derive(input);
    PyPrakriya {
        input: p.input,
        output: p.output.clone(),
        steps: p
            .steps
            .into_iter()
            .map(|s| PyStep {
                rule: s.rule.into(),
                description: s.description,
                before: s.before,
                after: s.after,
            })
            .collect(),
        is_correct: p.is_correct,
    }
}

#[pymodule]
pub fn prakriya(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyRule>()?;
    m.add_class::<PyStep>()?;
    m.add_class::<PyPrakriya>()?;
    m.add_function(wrap_pyfunction!(derive, m)?)?;
    Ok(())
}
