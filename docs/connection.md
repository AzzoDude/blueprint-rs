# blueprint-rs — Relationship Mapping (`connection.rs`)

`Connection` mapping is used to link two ports on two distinct nodes. This struct provides the backbone of the blueprint graph's edges.

## Attributes

| Attribute | Type | Description |
|---|---|---|
| `from_node` | `Uuid` | The origin Node ID. |
| `from_port` | `Uuid` | The origin Port ID. |
| `to_node` | `Uuid` | The destination Node ID. |
| `to_port` | `Uuid` | The destination Port ID. |

## Methods & Python API

### `Connection.new(from_node: Uuid, from_port: Uuid, to_node: Uuid, to_port: Uuid)` → `Connection`
Creates a static relationship record between two specific ports.

---

## Python Example

```python
from blueprint_rs import Connection

# All IDs must be valid Uuid objects
conn = Connection(node_a.id, port_a.id, node_b.id, port_b.id)
```
