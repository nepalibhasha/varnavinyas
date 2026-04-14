use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};

mod py_akshar;
mod py_kosha;
mod py_lekhya;
mod py_lipi;
mod py_parikshak;
pub(crate) mod py_prakriya;
mod py_sandhi;
mod py_shabda;

#[pymodule]
fn varnavinyas(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    // Top-level convenience exports: from varnavinyas import check_text
    m.add_function(wrap_pyfunction!(py_parikshak::check_text, m)?)?;
    m.add_function(wrap_pyfunction!(py_parikshak::check_text_with_options, m)?)?;
    m.add_class::<py_parikshak::PyDiagnostic>()?;
    m.add_class::<py_parikshak::PyDiagnosticReason>()?;
    // Submodules
    m.add_wrapped(wrap_pymodule!(py_akshar::akshar))?;
    m.add_wrapped(wrap_pymodule!(py_lipi::lipi))?;
    m.add_wrapped(wrap_pymodule!(py_shabda::shabda))?;
    m.add_wrapped(wrap_pymodule!(py_sandhi::sandhi))?;
    m.add_wrapped(wrap_pymodule!(py_prakriya::prakriya))?;
    m.add_wrapped(wrap_pymodule!(py_kosha::kosha))?;
    m.add_wrapped(wrap_pymodule!(py_lekhya::lekhya))?;
    m.add_wrapped(wrap_pymodule!(py_parikshak::parikshak))?;
    Ok(())
}
