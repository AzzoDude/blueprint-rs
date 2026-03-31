# blueprint-rs — Python Entry Point

This file serves as the main registration point for the `blueprint_rs` Rust extension. It uses **PyO3** to expose high-performance logic to Python.

## Module Details

| Component | Description |
|---|---|
| Module Name | `blueprint_rs` |
| Library Type | `cdylib` |

## Registered Classes

The following classes are exposed to Python through this entry point:

- `Node`
- `Port`
- `Connection`
- `DataType`

## Usage in Python

```python
import blueprint_rs

# You can now instantiate any of the registered classes
```
