# blueprint-rs — Node Implementation (`node.rs`)

The `Node` struct is the fundamental building block of the blueprint logic engine. It handles its state, manages ports, and can perform its own execution logic.

## Attributes

| Attribute | Type | Description |
|---|---|---|
| `id` | `Uuid` | Unique identifier for each node instance. |
| `name` | `String` | Human-readable identifier. |
| `position` | `(f32, f32)` | Graphical (x, y) coordinates for visual layout. |
| `inputs` | `Vec<Port>` | A list of incoming data or execution ports. |
| `outputs` | `Vec<Port>` | A list of outgoing data or execution ports. |
| `value` | `NodeValue` | The persistent data stored within the node. |

## Methods & Python API

### `Node.new(name: str, position: tuple)` → `Node`
Creates a brand-new node instance with a unique V4 UUID.

### `Node.set_name(name: str)`
Updates the node's name, with a validation rule: **Name must be 30 characters or less**.

### `Node.execute()`
Triggers the node's internal logic. Currently, this prints the node's name and its **`NodeValue`** to the console.

### `Node.add_input_port(name: str, data_type: DataType)`
Adds a new input port to the node.

### `Node.add_output_port(name: str, data_type: DataType)`
Adds a new output port to the node.

### `Node.get_port(id: Uuid)` → `Optional[Port]`
Searches through all inputs and outputs to find a port matching the specified UUID.

---

## Python Example

```python
from blueprint_rs import Node, NodeValue

node = Node("Greeting", (10.0, 20.0))
node.value = NodeValue.String("Hello from the visual script!")

# Execute the node logic
node.execute() 
# Output: [Node Greeting] Value: Hello from the visual script!
```
