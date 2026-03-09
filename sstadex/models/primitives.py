"""
sstadex.primitives
==================
Folder-based primitive loader.

Each primitive lives in its own folder:

    primitives/simplediffpair/
        primitive.json   -- descriptor: ports, layout params, lut config
        build.py         -- function build(primitive) -> pd.DataFrame
        netlist/
            netlist.spi
        cdl/
            netlist.cdl
        symbol/
            symbol.sym
        pcell/
            ihp_sg13g2/
                layout.py
            sky130/
                layout.py

Engine interface (unchanged):
    primitive.name          str
    primitive.parameters    dict[Symbol, np.ndarray]
    primitive.outputs       dict[str, np.ndarray]
    primitive.build()       populates parameters and outputs from LUT
"""

from __future__ import annotations

import json
import importlib.util
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

import numpy as np
import pandas as pd


# ---------------------------------------------------------------------------
# Internal config dataclasses (loaded from primitive.json)
# ---------------------------------------------------------------------------

@dataclass
class SweepAxis:
    variable: str
    values:   Any
    type:     str = "array"

    def resolve(self) -> np.ndarray:
        if self.type == "arange" and len(self.values) == 3:
            return np.arange(*self.values)
        return np.asarray(self.values)


@dataclass
class LUTConfig:
    lut_w:   float
    axes:    list[SweepAxis]
    lengths: list[float]


# ---------------------------------------------------------------------------
# Primitive
# ---------------------------------------------------------------------------

class Primitive:
    """
    Analog primitive loaded from a folder.

    Engine-facing attributes (same contract as old hardcoded primitives):
        name        : primitive identifier
        parameters  : dict[Symbol, np.ndarray]  -- small-signal params after build()
        outputs     : dict[str, np.ndarray]      -- sizing outputs after build()
    """

    def __init__(
        self,
        descriptor: dict,
        folder:     Path,
        lut_file:   str,
        il:         float = 100e-6,
    ):
        self._folder  = Path(folder)
        self.lut_file = lut_file
        self.il       = il

        # identity
        self.name            = descriptor["name"]
        self.version         = descriptor.get("version", "1.0")
        self.transistor_type = descriptor.get("transistor_type", "nmos")

        # LUT config
        lc = descriptor["lut_config"]
        self.lut_config = LUTConfig(
            lut_w   = lc["lut_w"],
            axes    = [
                SweepAxis(
                    variable = ax["variable"],
                    values   = ax["values"],
                    type     = ax.get("type", "array"),
                )
                for ax in lc["axes"]
            ],
            lengths = lc["lengths"],
        )

        # file map (keys: build, netlist, symbol, layout)
        self._files: dict[str, str] = descriptor.get("files", {})

        # engine interface -- populated after build()
        self.parameters: dict = {}
        self.outputs:    dict = {}

        # cached callables
        self._build_fn = None

    # ------------------------------------------------------------------
    # Factory
    # ------------------------------------------------------------------

    @classmethod
    def from_folder(cls, folder: str | Path, lut_file: str, **kwargs) -> "Primitive":
        """Load a Primitive from its folder (must contain primitive.json)."""
        folder    = Path(folder)
        json_path = folder / "primitive.json"

        if not json_path.exists():
            raise FileNotFoundError(f"No primitive.json in '{folder}'.")

        with open(json_path) as f:
            descriptor = json.load(f)

        return cls(descriptor=descriptor, folder=folder, lut_file=lut_file, **kwargs)

    # ------------------------------------------------------------------
    # build() -- dispatches to the primitive's build.py
    # ------------------------------------------------------------------

    def build(self) -> pd.DataFrame:
        """
        Run LUT sweep via the primitive's build.py.
        Populates self.parameters and self.outputs from the returned DataFrame.
        """
        if self._build_fn is None:
            self._build_fn = self._load_fn("build", "build", required=True)

        df = self._build_fn(self)

        # map DataFrame columns -> engine dicts
        self._populate_from_df(df)

        return df

    # ------------------------------------------------------------------
    # Engine dict population
    # ------------------------------------------------------------------

    def _populate_from_df(self, df: pd.DataFrame) -> None:
        """
        Map build() DataFrame columns into self.parameters and self.outputs.

        Convention:
            - sizing columns (width, length, width_*, length_*) -> self.outputs
            - small-signal columns (gm, gds, Ro, cgg, cgs, cgd, ...) -> self.parameters

        Both dicts are also keyed by sympy Symbol when the engine needs symbolic
        substitution -- here we keep plain string keys; the macromodel is
        responsible for creating the Symbol mapping when building its equations.
        """
        sizing_keys = {c for c in df.columns if c.startswith(("width", "length", "W", "L"))}

        for col in df.columns:
            values = df[col].values
            if col in sizing_keys:
                self.outputs[col] = values
            else:
                self.parameters[col] = values

    # ------------------------------------------------------------------
    # Dynamic module loader
    # ------------------------------------------------------------------

    def _load_fn(self, file_key: str, fn_name: str, required: bool = True):
        """Dynamically import a function from a .py file inside the primitive folder."""
        filename = self._files.get(file_key, f"{file_key}.py")
        filepath = self._folder / filename

        if not filepath.exists():
            if required:
                raise FileNotFoundError(
                    f"Required file '{filepath}' not found in primitive '{self.name}'."
                )
            return None

        module_name = f"sstadex._dyn.{self.name}.{file_key}"
        spec   = importlib.util.spec_from_file_location(module_name, filepath)
        module = importlib.util.module_from_spec(spec)
        sys.modules[module_name] = module
        spec.loader.exec_module(module)

        fn = getattr(module, fn_name, None)
        if fn is None:
            raise AttributeError(f"'{filepath}' has no function '{fn_name}'.")
        return fn

    def __repr__(self) -> str:
        return f"<Primitive '{self.name}' v{self.version} | lut={self.lut_file}>"


# ---------------------------------------------------------------------------
# Library -- registry that binds primitive folders to a PDK
# ---------------------------------------------------------------------------

class Library:
    """
    Technology-aware registry of primitive folders.

    Usage:
        lib = Library("ihp_sg13g2", lut_files={
            "nmos": "luts/IHP_LUT_nmos.npy",
            "pmos": "luts/IHP_LUT_pmos.npy",
        })
        lib.register("primitives/simplediffpair/")
        lib.register_all("primitives/")

        dp = lib.get("simplediffpair", il=100e-6)
        df = dp.build()
    """

    def __init__(self, name: str, lut_files: dict[str, str] = None):
        self.name      = name
        self.lut_files = lut_files or {}
        self._registry: dict[str, Path] = {}

    # ------------------------------------------------------------------
    # Registration
    # ------------------------------------------------------------------

    def register(self, folder: str | Path) -> None:
        """Register a single primitive folder."""
        folder    = Path(folder)
        json_path = folder / "primitive.json"

        if not json_path.exists():
            raise FileNotFoundError(f"Cannot register '{folder}': no primitive.json found.")

        with open(json_path) as f:
            meta = json.load(f)

        name = meta["name"]
        self._registry[name] = folder
        print(f"[Library:{self.name}] registered '{name}' <- {folder}")

    def register_all(self, primitives_dir: str | Path) -> None:
        """Auto-register every subfolder that contains a primitive.json."""
        primitives_dir = Path(primitives_dir)
        for subfolder in sorted(primitives_dir.iterdir()):
            if subfolder.is_dir() and (subfolder / "primitive.json").exists():
                self.register(subfolder)

    # ------------------------------------------------------------------
    # Instantiation
    # ------------------------------------------------------------------

    def get(self, primitive_name: str, **kwargs) -> Primitive:
        """
        Return a fresh Primitive with the correct LUT injected for this PDK.
        Extra kwargs (e.g. il=100e-6) are forwarded to Primitive.__init__.
        """
        if primitive_name not in self._registry:
            raise KeyError(
                f"Primitive '{primitive_name}' not in library '{self.name}'. "
                f"Available: {self.list()}"
            )

        folder = self._registry[primitive_name]

        with open(folder / "primitive.json") as f:
            meta = json.load(f)

        t_type   = meta.get("transistor_type", "nmos")
        lut_file = self.lut_files.get(t_type)

        if not lut_file:
            raise ValueError(
                f"No LUT registered for transistor type '{t_type}' "
                f"in library '{self.name}'."
            )

        return Primitive.from_folder(folder, lut_file=lut_file, **kwargs)

    # ------------------------------------------------------------------
    # Introspection
    # ------------------------------------------------------------------

    def list(self) -> list[str]:
        return list(self._registry.keys())

    def __repr__(self) -> str:
        return f"<Library '{self.name}': {self.list()}>"