use pyo3::{pyclass, pymethods};
use crate::model::datatype::DataType;

#[pyclass(get_all, set_all, from_py_object)]
#[derive(Debug, Clone)]
pub struct Port {
    pub id: uuid::Uuid,
    pub name: String,
    pub data_type: DataType,
}

#[pymethods]
impl Port {
    #[new]
    pub fn new(name: String, data_type: DataType) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            data_type,
        }
    }
}