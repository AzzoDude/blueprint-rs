use pyo3::prelude::{pymodule, Bound, PyModule, PyModuleMethods, PyResult};

pub mod model;

#[pymodule]
fn blueprint_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<model::node::Node>()?;
    m.add_class::<model::port::Port>()?;
    m.add_class::<model::connection::Connection>()?;
    Ok(())
}