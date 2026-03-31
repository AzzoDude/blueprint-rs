# blueprint-rs — Port Identification (`port.rs`)

`Port` structs define the input and output points of a `Node`. They are responsible for conveying data or execution flow.

## Attributes

| Attribute | Type | Description |
|---|---|---|
| `id` | `Uuid` | Unique V4 UUID for identification. |
| `name` | `String` | Name for display. |
| `data_type` | `DataType` | The specific logical type this port conveys. |

## Methods & Python API

### `Port.new(name: str, data_type: DataType)` → `Port`
Creates a brand-new port with a generated UUID.

---

## Python Example

```python
from blueprint_rs import Port, DataType

port = Port("X", DataType.Float)
print(f"Port {port.name} has ID: {port.id}")
```
