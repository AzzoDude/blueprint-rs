use pyo3::{pyclass, pymethods, PyResult, exceptions::PyValueError};
use uuid::Uuid;
use crate::model::port::Port;

#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct Node {
	#[pyo3(get)]
	pub id: Uuid,

	#[pyo3(get, set)]
	pub name: String,

	#[pyo3(get, set)]
	pub position: (f32, f32),

	#[pyo3(get)]
	pub inputs: Vec<Port>,

	#[pyo3(get)]
	pub outputs: Vec<Port>
}

#[pymethods]
impl Node {
	#[new]
	pub fn new(name: String, position: (f32, f32)) -> PyResult<Self> {
		if name.len() > 30 {
			return Err(PyValueError::new_err("Name must be 30 characters or less"));
		}

		Ok(Self {
			id: Uuid::new_v4(),
			name,
			position,
			inputs: Vec::new(),
			outputs: Vec::new(),
		})
	}

	#[setter]
	pub fn set_name(&mut self, name: String) -> PyResult<()> {
		if name.len() > 30 {
			return Err(PyValueError::new_err("Name must be 30 characters or less"));
		}
		self.name = name;
		Ok(())
	}

	pub fn get_port(&self, id: Uuid) -> Option<Port> {
        self.inputs.iter()
            .chain(self.outputs.iter())
            .find(|p| p.id == id)
            .cloned()
    }
}