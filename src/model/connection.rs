use pyo3::{pyclass, pymethods};
use uuid::Uuid;

#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct Connection {
    #[pyo3(get)]
    pub from_node: Uuid,

    #[pyo3(get)]
    pub from_port: Uuid,

    #[pyo3(get)]
    pub to_node: Uuid,

    #[pyo3(get)]
    pub to_port: Uuid,
}

#[pymethods]
impl Connection {
    #[new]
    pub fn new(from_node: Uuid, from_port: Uuid, to_node: Uuid, to_port: Uuid) -> Self {
        Self {
            from_node,
            from_port,
            to_node,
            to_port,
        }
    }
}