use pyo3::{pyclass, pymethods, PyResult, exceptions::PyValueError};
use crate::model::datatype::NodeValue;
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
	pub outputs: Vec<Port>,

	#[pyo3(get, set)]
	pub value: NodeValue
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
			value: NodeValue::None(),
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

	pub fn execute(&self) {
		match &self.value {
			NodeValue::None() => println!("[Node {}] Value: None", self.name),
			NodeValue::Int(v) => println!("[Node {}] Value: {}", self.name, v),
			NodeValue::Float(v) => println!("[Node {}] Value: {}", self.name, v),
			NodeValue::String(v) => println!("[Node {}] Value: {}", self.name, v),
			NodeValue::Bool(v) => println!("[Node {}] Value: {}", self.name, v),
		}
	}

	pub fn add_port(&mut self, name: String, data_type: crate::model::datatype::DataType, is_output: bool) {
		let port = Port::new(name, data_type);
		if is_output {
			self.outputs.push(port);
		} else {
			self.inputs.push(port);
		}
	}

	pub fn get_port(&self, id: Uuid) -> Option<Port> {
        self.inputs.iter()
            .chain(self.outputs.iter())
            .find(|p| p.id == id)
            .cloned()
    }
}