# blueprint-rs — Node Implementation (`node.rs`)

The `Node` struct is the fundamental building block of the blueprint logic engine. It maintains its own state, and is responsible for managing its connected inputs and outputs.

## Attributes

| Attribute | Type | Description |
|---|---|---|
| `id` | `Uuid` | Unique identifier for each node instance. |
| `name` | `String` | Human-readable identifier. |
| `position` | `(f32, f32)` | Graphical (x, y) coordinates for visual layout. |
| `inputs` | `Vec<Port>` | A list of incoming data or execution ports. |
| `outputs` | `Vec<Port>` | A list of outgoing data or execution ports. |

## Methods & Python API

### `Node.new(name: str, position: tuple)` → `Node`
Creates a new node instance with a unique V4 UUID.

### `Node.set_name(name: str)`
Updates the node's name, with a validation rule: **Name must be 30 characters or less**.

### `Node.get_port(id: Uuid)` → `Optional[Port]`
Searches through all inputs and outputs to find a port matching the specified UUID.

---

## Python Example

```python
from blueprint_rs import Node, DataType, Port

node = Node("Addition", (100.0, 200.0))
print(f"Node ID: {node.id}")
```
