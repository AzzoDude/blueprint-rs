use pyo3::pyclass;

#[pyclass(from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Exec,
    Int,
    Float,
    String,
    Bool
}