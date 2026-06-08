from __future__ import annotations

import importlib.util
import inspect
import json
import sys
from pathlib import Path

from .macromodel import Macromodel


class MacroLibrary:
    """
    Registry of reusable macromodel folders.

    Expected folder structure:

        analoglib/macros/my_macro/
            macro.json
            compose.py

    Where:
        - macro.json contains metadata and optionally a custom compose file path
        - compose.py exposes a function named `compose` (preferred) or
          `build_macro` returning a Macromodel instance
    """

    def __init__(self, name: str, primitive_library=None):
        self.name = name
        self.primitive_library = primitive_library
        self._registry: dict[str, Path] = {}

    def register(self, folder: str | Path) -> None:
        folder = Path(folder)
        json_path = folder / "macro.json"

        if not json_path.exists():
            raise FileNotFoundError(
                f"Cannot register '{folder}': no macro.json found."
            )

        with open(json_path) as f:
            meta = json.load(f)

        name = meta["name"]
        self._registry[name] = folder
        print(f"[MacroLibrary:{self.name}] registered '{name}' <- {folder}")

    def register_all(self, macros_dir: str | Path) -> None:
        macros_dir = Path(macros_dir)
        for subfolder in sorted(macros_dir.iterdir()):
            if subfolder.is_dir() and (subfolder / "macro.json").exists():
                self.register(subfolder)

    def _load_descriptor(self, folder: Path) -> dict:
        with open(folder / "macro.json") as f:
            return json.load(f)

    def _load_compose_fn(self, folder: Path, descriptor: dict):
        files = descriptor.get("files", {})
        compose_file = files.get("compose", "compose.py")
        compose_path = folder / compose_file

        if not compose_path.exists():
            raise FileNotFoundError(
                f"Compose file '{compose_path}' not found for macro "
                f"'{descriptor.get('name', folder.name)}'."
            )

        module_name = f"sstadex._dynmacro.{descriptor['name']}"
        spec = importlib.util.spec_from_file_location(module_name, compose_path)
        module = importlib.util.module_from_spec(spec)
        sys.modules[module_name] = module
        spec.loader.exec_module(module)

        compose_fn = getattr(module, "compose", None)
        if compose_fn is None:
            compose_fn = getattr(module, "build_macro", None)

        if compose_fn is None:
            raise AttributeError(
                f"Macro '{descriptor['name']}' compose file must define "
                "'compose' or 'build_macro'."
            )

        return compose_fn

    def get(self, macro_name: str, **kwargs) -> Macromodel:
        if macro_name not in self._registry:
            raise KeyError(
                f"Macro '{macro_name}' not in macro library '{self.name}'. "
                f"Available: {self.list()}"
            )

        folder = self._registry[macro_name]
        descriptor = self._load_descriptor(folder)
        compose_fn = self._load_compose_fn(folder, descriptor)

        available_kwargs = {
            "descriptor": descriptor,
            "folder": folder,
            "primitive_library": self.primitive_library,
            "macro_library": self,
            **kwargs,
        }

        signature = inspect.signature(compose_fn)
        if any(
            param.kind == inspect.Parameter.VAR_KEYWORD
            for param in signature.parameters.values()
        ):
            compose_kwargs = available_kwargs
        else:
            compose_kwargs = {
                key: value
                for key, value in available_kwargs.items()
                if key in signature.parameters
            }

        macro = compose_fn(**compose_kwargs)

        if not isinstance(macro, Macromodel):
            raise TypeError(
                f"Macro factory for '{macro_name}' must return a Macromodel, "
                f"got {type(macro).__name__}."
            )

        return macro

    def list(self) -> list[str]:
        return list(self._registry.keys())

    def __repr__(self) -> str:
        return f"<MacroLibrary '{self.name}': {self.list()}>"
