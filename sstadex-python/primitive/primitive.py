"""
analoglib.core.primitive
========================
Base class for all analog primitives in the library.

A Primitive is a self-contained design unit that knows:
  - Its ports and their roles (input, output, bias, supply)
  - Its SPICE netlist template
  - Its layout parameters (parametric PCells)
  - How to build its LUT given external port voltages
  - Its transistor-level composition
"""

from __future__ import annotations
from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Any
import numpy as np
import pandas as pd


# ---------------------------------------------------------------------------
# Port model
# ---------------------------------------------------------------------------

class PortRole(Enum):
    INPUT   = auto()   # AC/signal input
    OUTPUT  = auto()   # AC/signal output
    BIAS    = auto()   # DC bias (set from outside)
    SUPPLY  = auto()   # VDD / VSS
    INTERNAL = auto()  # internal node (not exposed)


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
    dc_voltage: float | None = None
    description: str = ""

    def set_voltage(self, v: float) -> None:
        self.dc_voltage = float(v)

    def __repr__(self) -> str:
        v = f"{self.dc_voltage:.3f}V" if self.dc_voltage is not None else "unset"
        return f"Port({self.name!r}, {self.role.name}, {v})"


# ---------------------------------------------------------------------------
# Layout parameter descriptor
# ---------------------------------------------------------------------------

@dataclass
class LayoutParam:
    """
    Describes one parametric dimension of the layout PCell.

    Parameters
    ----------
    name : str
        Parameter name (e.g. 'W', 'L', 'nf', 'rows').
    unit : str
        SI unit string (e.g. 'm', 'int').
    description : str
        Human-readable description.
    default : float | int | None
        Default value before sizing.
    computed : bool
        If True, this parameter is filled automatically by `build()`.
    """
    name: str
    unit: str = "m"
    description: str = ""
    default: float | int | None = None
    computed: bool = False


# ---------------------------------------------------------------------------
# LUT sweep configuration
# ---------------------------------------------------------------------------

@dataclass
class SweepAxis:
    """
    One axis of the LUT sweep space.

    Parameters
    ----------
    variable : str
        Name of the sweep variable. Must be one of:
        'gmid', 'vgs', 'vds', 'length', 'jd'.
    values : np.ndarray | tuple
        Array of values OR a (start, stop, step) tuple (like np.arange).
    """
    variable: str
    values: np.ndarray | tuple

    def resolve(self) -> np.ndarray:
        if isinstance(self.values, tuple) and len(self.values) == 3:
            return np.arange(*self.values)
        return np.asarray(self.values)


@dataclass
class LUTConfig:
    """
    Full LUT sweep configuration for a primitive.

    Parameters
    ----------
    lut_file : str
        Path to the pre-characterized transistor LUT (.npy).
    lut_w : float
        Reference width used during LUT characterization (meters).
    axes : list[SweepAxis]
        Ordered list of sweep axes (at least two for a 2-D sweep).
    lengths : list[float]
        Gate lengths to sweep (meters).
    """
    lut_file: str
    lut_w: float
    axes: list[SweepAxis]
    lengths: list[float]


# ---------------------------------------------------------------------------
# Base Primitive
# ---------------------------------------------------------------------------

class Primitive(ABC):
    """
    Abstract base class for all analog primitives.

    Subclasses must implement:
        - `ports`        → dict[str, Port]
        - `layout_params`→ dict[str, LayoutParam]
        - `lut_config`   → LUTConfig
        - `netlist_template` → str  (SPICE template with {port_name} tokens)
        - `build()`      → pd.DataFrame  (runs LUT sweep, fills layout_params)
    """

    # ------------------------------------------------------------------
    # Identity
    # ------------------------------------------------------------------
    name: str = ""
    description: str = ""
    version: str = "0.1"

    # ------------------------------------------------------------------
    # Must be defined by subclass (class-level descriptors)
    # ------------------------------------------------------------------

    @property
    @abstractmethod
    def ports(self) -> dict[str, Port]:
        """Return the port map for this primitive."""

    @property
    @abstractmethod
    def layout_params(self) -> dict[str, LayoutParam]:
        """Return the layout parameter map."""

    @property
    @abstractmethod
    def lut_config(self) -> LUTConfig:
        """Return the LUT sweep configuration."""

    @property
    @abstractmethod
    def netlist_template(self) -> str:
        """
        SPICE netlist template string.
        Use {PORT_NAME} tokens that will be replaced by net names on instantiation.
        Use {PARAM_NAME} tokens for layout parameters (W, L, nf, ...).
        Example:
            'M1 {DRAIN} {GATE} {SOURCE} {BULK} nmos W={W} L={L} nf={nf}'
        """

    # ------------------------------------------------------------------
    # Core API
    # ------------------------------------------------------------------

    @abstractmethod
    def build(self) -> pd.DataFrame:
        """
        Run the LUT-based sweep for this primitive given the current port voltages.

        Returns
        -------
        pd.DataFrame
            One row per operating point in the design space.
            Columns always include: length, width, gm, gds, Ro, cgg, cgs, cgd.
            Plus any primitive-specific columns.
        """

    def set_port_voltage(self, port_name: str, voltage: float) -> None:
        """Set the DC voltage on a port (called from outside before build())."""
        if port_name not in self.ports:
            raise KeyError(f"Port {port_name!r} not found in primitive {self.name!r}.")
        self.ports[port_name].set_voltage(voltage)

    def set_port_voltages(self, voltages: dict[str, float]) -> None:
        """Bulk-set port voltages from a dict."""
        for name, v in voltages.items():
            self.set_port_voltage(name, v)

    def render_netlist(self, net_map: dict[str, str], instance_name: str = "X1") -> str:
        """
        Render the SPICE netlist for one sized instance.

        Parameters
        ----------
        net_map : dict[str, str]
            Mapping from port name → actual net name in the schematic.
        instance_name : str
            SPICE instance identifier.

        Returns
        -------
        str
            SPICE netlist string ready to concatenate into a deck.
        """
        tokens: dict[str, Any] = {}
        # Instance name
        tokens["INSTANCE"] = instance_name
        # Port nets
        for pname, port in self.ports.items():
            net = net_map.get(pname, pname)
            tokens[pname] = net
        # Layout params
        for lpname, lp in self.layout_params.items():
            tokens[lpname] = lp.default if lp.default is not None else 0
        return f"* {instance_name}: {self.name}\n" + self.netlist_template.format(**tokens)

    def check_ports_set(self, required_roles: list[PortRole] | None = None) -> None:
        """Raise ValueError if any required port voltages are unset."""
        roles = required_roles or [PortRole.BIAS, PortRole.SUPPLY]
        missing = [
            p.name for p in self.ports.values()
            if p.role in roles and p.dc_voltage is None
        ]
        if missing:
            raise ValueError(
                f"Primitive {self.name!r}: port voltages not set for: {missing}. "
                "Call set_port_voltages() before build()."
            )

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
        for lp in self.layout_params.values():
            val = f"{lp.default}" if lp.default is not None else "TBD"
            flag = " [computed]" if lp.computed else ""
            lines.append(f"  {lp.name} [{lp.unit}] = {val}{flag}  — {lp.description}")
        lines.append(f"{'='*60}")
        return "\n".join(lines)

    def __repr__(self) -> str:
        return f"<Primitive: {self.name}>"