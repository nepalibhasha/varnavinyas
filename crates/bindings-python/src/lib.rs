use pyo3::prelude::*;
use pyo3::wrap_pymodule;

mod py_akshar;
mod py_lipi;
mod py_prakriya;
mod py_sandhi;
mod py_shabda;

#[pymodule]
fn varnavinyas(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_wrapped(wrap_pymodule!(py_akshar::akshar))?;
    m.add_wrapped(wrap_pymodule!(py_lipi::lipi))?;
    m.add_wrapped(wrap_pymodule!(py_shabda::shabda))?;
    m.add_wrapped(wrap_pymodule!(py_sandhi::sandhi))?;
    m.add_wrapped(wrap_pymodule!(py_prakriya::prakriya))?;
    Ok(())
}
