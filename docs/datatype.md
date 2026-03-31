# blueprint-rs — Type System (`datatype.rs`)

`blueprint-rs` uses two core data structures to define and manage data: **`DataType`** for identity and **`NodeValue`** for storage and computation.

---

## 1. `DataType` (Enum)
This is the identity registry, used by **Ports** to declare what kind of data they expect.

| Variant | Purpose |
|---|---|
| `Exec` | Signal to trigger execution flow on the subsequent node. |
| `Int` | Integer numeric value. |
| `Float` | Floating-point numeric value. |
| `String` | Text-based data. |
| `Bool` | Boolean logic (True/False). |

---

## 2. `NodeValue` (Enum)
This is the **storage and computation engine** that actually holds the data inside a Node.

### Variants
- `None`: Represents an empty or uninitialized state.
- `Int(i64)`: Stores integer numbers.
- `Float(f64)`: Stores decimal numbers.
- `String(String)`: Stores text data.
- `Bool(bool)`: Stores boolean states.

### Arithmetic Support
`NodeValue` implements standard Python arithmetic magic methods for flexible computation:

| Operator | Action | Supported Types |
|---|---|---|
| `+` | Add / Concatenate | `Int + Int`, `Float + Float`, `String + String` |
| `-` | Subtract | `Int - Int`, `Float - Float` |
| `*` | Multiply | `Int * Int`, `Float * Float` |
| `/` | Divide | `Int / Int` (returns Float), `Float / Float` |

---

## Python Example

```python
from blueprint_rs import DataType, NodeValue

# Create typed values
val_a = NodeValue.Int(10)
val_b = NodeValue.Float(5.5)

# Perform arithmetic safely
result = val_a + val_b  # NodeValue.Float(15.5)
print(f"Computed Result: {result}")
```
