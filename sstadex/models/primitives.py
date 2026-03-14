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
from abc import ABC, abstractmethod

import numpy as np
import pandas as pd

from enum import Enum, auto

class PortRole(Enum):
    INPUT   = auto()   # AC/signal input
    OUTPUT  = auto()   # AC/signal output
    BIAS    = auto()   # DC bias (set from outside)
    SUPPLY  = auto()   # VDD / VSS
    INTERNAL = auto()  # internal node (not exposed)

# ---------------------------------------------------------------------------
# Internal config dataclasses (loaded from primitive.json)
# ---------------------------------------------------------------------------

@dataclass
class Port:
    """
    A port of a primitive.

    Parameters
    ----------
    name : str
        Port name (must match the token in the netlist template).
    role : PortRole
        Functional role of the port.
    dc_voltage : float | None
        DC operating voltage applied from outside.
        Must be set before calling `build()`.
    description : str
        Human-readable description.
    """
    name: str
    role: PortRole
    dc_voltage: float | np.array | None = None
    description: str = ""

    def set_voltage(self, v: float | np.array) -> None:
        self.dc_voltage = v

    def __repr__(self) -> str:
        v = f"{self.dc_voltage:}V" if self.dc_voltage is not None else "unset"
        return f"Port({self.name!r}, {self.role.name}, {v})"

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
        self.description     = descriptor.get("description", "")
        self.version         = descriptor.get("version", "1.0")
        self.transistor_type = descriptor.get("transistor_type", "nmos")
        self.ports = {name: Port(name, PortRole[p["role"]], description=p.get("description"))
                      for name, p in descriptor.get("ports", {}).items()
        }

        self.subckt_name = descriptor.get("subckt_name", self.name)
        self.pin_order = descriptor.get("pin_order", list(descriptor.get("ports", {}).keys()))
        self._netlist_template = None

        self.small_signal = descriptor.get("small_signal", {})

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

    @property
    def netlist_template(self) -> str:
        if self._netlist_template is None:
            self._netlist_template = self._load_text_file("netlist", required=True)
        return self._netlist_template

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

    def _load_text_file(self, file_key: str, required: bool = True) -> str | None:
        filename = self._files.get(file_key)
        if filename is None:
            if required:
                raise FileNotFoundError(
                    f"Primitive '{self.name}' has no file entry for '{file_key}'."
                )
            return None

        filepath = self._folder / filename
        if not filepath.exists():
            if required:
                raise FileNotFoundError(
                    f"Required file '{filepath}' not found in primitive '{self.name}'."
                )
            return None

        return filepath.read_text()

    def render_subckt(self, index: int | None = None) -> str:
        params = self.get_netlist_params(index=index)
        print(f"rendering subckt with params: {params}")
        tokens = {
            "SUBCKT_NAME": self.subckt_name,
            **params,
        }

        return self.netlist_template.format(**tokens)

    def render_instance(self, instance_name: str, net_map: dict[str, str], view: str = "physical") -> str:
        if view == "physical":
            return self.render_physical_instance(instance_name, net_map)
        if view == "small_signal":
            return self.render_small_signal_instance(instance_name, net_map)
        raise ValueError(f"Unknown view '{view}'. Expected 'physical' or 'small_signal'.")

    def render_physical_instance(self, instance_name: str, net_map: dict[str, str]) -> str:
        missing = [pin for pin in self.pin_order if pin not in net_map]
        if missing:
            raise KeyError(
                f"Primitive '{self.name}' missing nets for pins: {missing}"
            )

        nets = [net_map[pin] for pin in self.pin_order]
        return f"{instance_name} {' '.join(nets)} {self.subckt_name}"

    def small_signal_branches(self) -> list[dict[str, str]]:
        """
        Returns a list of branch descriptors for the simplified small-signal model.

        Each branch must define:
            - name: unique branch suffix inside the primitive
            - vd: drain/output-like port name
            - vg: control/input-like port name
            - vs: source/reference port name
        """
        raise NotImplementedError(
            f"Primitive '{self.name}' must implement small_signal_branches()."
        )

    def render_small_signal_instance(self, instance_name: str, net_map: dict[str, str]) -> str:
        lines = []
        branches = self.small_signal_branches()

        for branch in branches:
            required = [branch["vd"], branch["vg"], branch["vs"]]
            missing = [pin for pin in required if pin not in net_map]
            if missing:
                raise KeyError(
                    f"Primitive '{self.name}' instance '{instance_name}' missing nets for branch "
                    f"'{branch['name']}': {missing}"
                )

            vd = net_map[branch["vd"]]
            vg = net_map[branch["vg"]]
            vs = net_map[branch["vs"]]
            suffix = f"{instance_name}_{branch['name']}"

            lines.append(f"R_gds_{suffix} {vd} {vs} 1")
            lines.append(f"G_gm_{suffix} {vd} {vs} {vg} {vs} 1")

        return "\n".join(lines)


    def get_netlist_params(self, index: int | None = None) -> dict[str, Any]:
        params = {}
        idx = 0 if index is None else index

        for key, values in self.outputs.items():
            arr = np.asarray(values)
            if arr.ndim == 0:
                params[key] = arr.item()
            else:
                params[key] = arr[idx]

        return params

    def small_signal_branches(self) -> list[dict[str, str]]:
        branches = self.small_signal.get("branches", [])
        if not branches:
            raise ValueError(
                f"Primitive '{self.name}' has no small_signal branches defined."
            )
        return branches

    
    def set_port_voltage(self, port_name: str, voltage: float) -> None:
        """Set the DC voltage on a port (called from outside before build())."""
        if port_name not in self.ports:
            raise KeyError(f"Port {port_name!r} not found in primitive {self.name!r}.")
        self.ports[port_name].set_voltage(voltage)

    def set_port_voltages(self, voltages: dict[str, float]) -> None:
        """Bulk-set port voltages from a dict."""
        for name, v in voltages.items():
            self.set_port_voltage(name, v)

    def summary(self) -> str:
        lines = [
            f"{'='*60}",
            f"Primitive : {self.name}  (v{self.version})",
            f"Desc      : {self.description}",
            f"{'─'*60}",
            "Ports:",
        ]
        for p in self.ports.values():
            lines.append(f"  {p}")
        lines.append("Layout params:")
        # for lp in self.layout_params.values():
        #     val = f"{lp.default}" if lp.default is not None else "TBD"
        #     flag = " [computed]" if lp.computed else ""
        #     lines.append(f"  {lp.name} [{lp.unit}] = {val}{flag}  — {lp.description}")
        lines.append(f"{'='*60}")
        return "\n".join(lines)

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