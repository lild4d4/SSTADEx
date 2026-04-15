Expected macro library structure
===============================

Each reusable macromodel lives in its own folder:

    analoglib/macros/my_macro/
        macro.json
        compose.py

Minimal `macro.json` example:

```json
{
  "name": "current_source_macro",
  "version": "1.0",
  "description": "Reusable hierarchical current source macro",
  "files": {
    "compose": "compose.py"
  }
}
```

Expected `compose.py` entrypoint:

```python
from sstadex import Macromodel


def compose(primitive_library=None, macro_library=None, **kwargs):
    macro = Macromodel(
        name="current_source_macro",
        ports=["VOUT", "VSS"],
        outputs=[],
    )
    return macro
```

`MacroLibrary.get(...)` injects these optional keyword arguments into the
factory when supported by the function signature:

- `descriptor`
- `folder`
- `primitive_library`
- `macro_library`

This allows a macro to instantiate primitives from the primitive library or
reuse other registered macros.
