# blueprint-rs — Type System (`datatype.rs`)

`DataType` is the enumeration of all supported types in the blueprint engine.

## Variants

| Variant | Purpose |
|---|---|
| `Exec` | Signal to trigger execution flow on the subsequent node. |
| `Int` | Integer numeric value. |
| `Float` | Floating-point numeric value. |
| `String` | Text-based data. |
| `Bool` | Boolean logic (True/False). |

---

## Python Example

```python
from blueprint_rs import DataType

print(f"Supported Type: {DataType.Exec}")
```
