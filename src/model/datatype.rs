use pyo3::{pyclass, pymethods, PyResult};

#[pyclass(from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Exec,
    Int,
    Float,
    String,
    Bool
}

#[pyclass(from_py_object)]
#[derive(Debug, Clone, PartialEq)]
pub enum NodeValue {
    None(),
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

#[pymethods]
impl NodeValue {
    fn __repr__(&self) -> String {
        match self {
            NodeValue::None() => "NodeValue.None".to_string(),
            NodeValue::Int(v) => format!("NodeValue.Int({})", v),
            NodeValue::Float(v) => format!("NodeValue.Float({})", v),
            NodeValue::String(v) => format!("NodeValue.String(\"{}\")", v),
            NodeValue::Bool(v) => format!("NodeValue.Bool({})", v),
        }
    }

    pub fn __add__(&self, other: &Self) -> PyResult<Self> {
        match (self, other) {
            (NodeValue::Int(a), NodeValue::Int(b)) => Ok(NodeValue::Int(a + b)),
            (NodeValue::Float(a), NodeValue::Float(b)) => Ok(NodeValue::Float(a + b)),
            (NodeValue::Int(a), NodeValue::Float(b)) => Ok(NodeValue::Float(*a as f64 + b)),
            (NodeValue::Float(a), NodeValue::Int(b)) => Ok(NodeValue::Float(a + *b as f64)),
            (NodeValue::String(a), NodeValue::String(b)) => Ok(NodeValue::String(format!("{}{}", a, b))),
            _ => Err(pyo3::exceptions::PyTypeError::new_err("Invalid types for addition")),
        }
    }

    fn __sub__(&self, other: &Self) -> PyResult<Self> {
        match (self, other) {
            (NodeValue::Int(a), NodeValue::Int(b)) => Ok(NodeValue::Int(a - b)),
            (NodeValue::Float(a), NodeValue::Float(b)) => Ok(NodeValue::Float(a - b)),
            (NodeValue::Int(a), NodeValue::Float(b)) => Ok(NodeValue::Float(*a as f64 - b)),
            (NodeValue::Float(a), NodeValue::Int(b)) => Ok(NodeValue::Float(a - *b as f64)),
            _ => Err(pyo3::exceptions::PyTypeError::new_err("Invalid types for subtraction")),
        }
    }

    fn __mul__(&self, other: &Self) -> PyResult<Self> {
        match (self, other) {
            (NodeValue::Int(a), NodeValue::Int(b)) => Ok(NodeValue::Int(a * b)),
            (NodeValue::Float(a), NodeValue::Float(b)) => Ok(NodeValue::Float(a * b)),
            (NodeValue::Int(a), NodeValue::Float(b)) => Ok(NodeValue::Float(*a as f64 * b)),
            (NodeValue::Float(a), NodeValue::Int(b)) => Ok(NodeValue::Float(a * *b as f64)),
            _ => Err(pyo3::exceptions::PyTypeError::new_err("Invalid types for multiplication")),
        }
    }

    fn __truediv__(&self, other: &Self) -> PyResult<Self> {
        match (self, other) {
            (NodeValue::Int(a), NodeValue::Int(b)) => {
                if *b == 0 { return Err(pyo3::exceptions::PyZeroDivisionError::new_err("Division by zero")); }
                Ok(NodeValue::Float(*a as f64 / *b as f64))
            },
            (NodeValue::Float(a), NodeValue::Float(b)) => {
                if *b == 0.0 { return Err(pyo3::exceptions::PyZeroDivisionError::new_err("Division by zero")); }
                Ok(NodeValue::Float(a / b))
            },
            (NodeValue::Int(a), NodeValue::Float(b)) => {
                if *b == 0.0 { return Err(pyo3::exceptions::PyZeroDivisionError::new_err("Division by zero")); }
                Ok(NodeValue::Float(*a as f64 / b))
            },
            (NodeValue::Float(a), NodeValue::Int(b)) => {
                if *b == 0 { return Err(pyo3::exceptions::PyZeroDivisionError::new_err("Division by zero")); }
                Ok(NodeValue::Float(a / *b as f64))
            },
            _ => Err(pyo3::exceptions::PyTypeError::new_err("Invalid types for division")),
        }
    }
}