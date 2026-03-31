use pyo3::{pyclass, pymethods, PyResult};
use uuid::Uuid;
use crate::model::node::Node;
use crate::model::connection::Connection;
use crate::model::datatype::NodeValue;
use std::collections::HashMap;

#[pyclass]
pub struct Graph {
    #[pyo3(get)]
    pub nodes: HashMap<Uuid, Node>,
    #[pyo3(get)]
    pub connections: Vec<Connection>,
}

#[pymethods]
impl Graph {
    #[new]
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_connection(&mut self, connection: Connection) {
        self.connections.push(connection);
    }

    /// Performs a real calculation for a node by pulling values from its inputs
    pub fn compute(&mut self, node_id: Uuid) -> PyResult<NodeValue> {
        // 1. Find the target node
        let node = self.nodes.get(&node_id).ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("Node not found")
        })?;

        // 2. Special Logic for "Arithmetic_Add"
        if node.name == "Arithmetic_Add" {
            let mut values = Vec::new();
            
            // Find all connections pointing to this node's inputs
            for conn in &self.connections {
                if conn.to_node == node_id {
                    // Pull the value from the source node
                    if let Some(source_node) = self.nodes.get(&conn.from_node) {
                        values.push(source_node.value.clone());
                    }
                }
            }

            // Perform the addition across all found source values
            if values.len() >= 2 {
                let mut result = values[0].clone();
                for i in 1..values.len() {
                    // Use the __add__ implementation we wrote earlier!
                    result = result.__add__(&values[i])?;
                }
                
                // Update the node's internal value with the result
                if let Some(n) = self.nodes.get_mut(&node_id) {
                    n.value = result.clone();
                }
                return Ok(result);
            }
        }

        // Default: return the current value of the node
        Ok(node.value.clone())
    }
}
