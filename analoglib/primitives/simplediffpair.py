"""
analoglib.primitives.simplediffpair
====================================
Simple NMOS differential pair primitive.

Ports
-----
  VIN+   (INPUT)   — positive differential input
  VIN-   (INPUT)   — negative differential input
  VOUT+  (OUTPUT)  — positive output
  VOUT-  (OUTPUT)  — negative output
  VTAIL  (BIAS)    — tail current source bias
  VDD    (SUPPLY)
  VSS    (SUPPLY)

The build() sweeps (gmid, length) or (vgs, length) and returns a
DataFrame with all small-signal params scaled to the operating current.
"""

from __future__ import annotations
import numpy as np
import pandas as pd

# Import the Transistor LUT engine from your existing code
from sstadex import Transistor   # ← same Transistor class you already have
from sstadex.primitive import (
    Primitive, Port, PortRole, LayoutParam, LUTConfig, SweepAxis
)


class SimpleDiffPair(Primitive):
    """
    NMOS simple differential pair.

    Parameters
    ----------
    lut_file : str
        Path to the characterised transistor LUT.
    lut_w : float
        Reference transistor width in the LUT (default 10 µm).
    il : float
        Tail current (A). Each branch carries il/2.
    transistor_type : str
        'nmos' or 'pmos'.
    """

    name        = "simplediffpair"
    description = "Simple NMOS/PMOS differential pair (single-transistor per branch)"
    version     = "1.0"

    # ------------------------------------------------------------------
    # Constructor
    # ------------------------------------------------------------------

    def __init__(
        self,
        lut_file: str,
        lut_w: float = 10e-6,
        il: float = 100e-6,
        transistor_type: str = "nmos",
    ):
        self._lut_file = lut_file
        self._lut_w    = lut_w
        self._il       = il
        self._type     = transistor_type

        # ------ ports (mutable instances, one per object) ------
        self._ports = {
            "VINP":  Port("VINP",  PortRole.INPUT,   description="Positive diff. input"),
            "VINN":  Port("VINN",  PortRole.INPUT,   description="Negative diff. input"),
            "VOUTP": Port("VOUTP", PortRole.OUTPUT,  description="Positive output"),
            "VOUTN": Port("VOUTN", PortRole.OUTPUT,  description="Negative output"),
            "VTAIL": Port("VTAIL", PortRole.BIAS,    description="Tail node voltage"),
            "VDD":   Port("VDD",   PortRole.SUPPLY,  description="Positive supply"),
            "VSS":   Port("VSS",   PortRole.SUPPLY,  description="Ground / neg. supply"),
        }

        # ------ layout params (filled by build()) ------
        self._layout_params = {
            "W":  LayoutParam("W",  "m",   "Transistor width",          computed=True),
            "L":  LayoutParam("L",  "m",   "Transistor gate length"),
            "nf": LayoutParam("nf", "int", "Number of fingers", default=2),
        }

        # ------ LUT sweep config ------
        self._lut_config = LUTConfig(
            lut_file = lut_file,
            lut_w    = lut_w,
            axes     = [
                SweepAxis("vds",   (0.1, 1.2, 100)),     # gm/Id axis
                SweepAxis("vgs", (0.1, 1.2, 100)),
            ],
            lengths  = [0.4e-6, 0.8e-6, 1.6e-6, 3.2e-6],
        )

    # ------------------------------------------------------------------
    # Primitive interface — properties
    # ------------------------------------------------------------------

    @property
    def ports(self) -> dict[str, Port]:
        return self._ports

    @property
    def layout_params(self) -> dict[str, LayoutParam]:
        return self._layout_params

    @property
    def lut_config(self) -> LUTConfig:
        return self._lut_config

    @property
    def netlist_template(self) -> str:
        # SPICE subcircuit template.  {TOKEN} are replaced by render_netlist().
        return (
            ".subckt {INSTANCE} {VINP} {VINN} {VOUTP} {VOUTN} {VTAIL} {VDD} {VSS}\n"
            "M1 {VOUTN} {VINP} {VTAIL} {VSS} {type} W={W} L={L} nf={nf}\n"
            "M2 {VOUTP} {VINN} {VTAIL} {VSS} {type} W={W} L={L} nf={nf}\n"
            ".ends {INSTANCE}\n"
        ).replace("{type}", self._type)

    # ------------------------------------------------------------------
    # build()
    # ------------------------------------------------------------------

    def build(self) -> pd.DataFrame:
        """
        Sweep over (gmid × length) space and return small-signal parameters.

        The voltages that matter for the transistor operating point are:
            vds = VOUTN - VTAIL   (or VOUTP - VTAIL)
            vgs = VINP  - VTAIL

        If these ports have DC voltages set, they are used directly.
        Otherwise a default LUT range is swept.
        """
        # --- resolve sweep variables ---
        cfg   = self._lut_config
        axis0 = cfg.axes[0]   # gmid (or first axis)
        axis1 = cfg.axes[1]   # length (or second axis)

        arr0 = axis0.resolve()   # e.g. gmid values
        arr1 = axis1.resolve()   # e.g. length values

        # --- derive vds / vgs from ports if set ---
        vout = self._ports["VOUTP"].dc_voltage
        vtail = self._ports["VTAIL"].dc_voltage
        vinp  = self._ports["VINP"].dc_voltage

        # Provide fallback ranges when ports are not biased externally
        if vout is not None and vtail is not None:
            vds_sweep = float(vout - vtail)   # scalar — single operating point
        else:
            vds_sweep = np.linspace(0.1, 1.2, 10)     # default range for LUT interpolation

        if vinp is not None and vtail is not None:
            vgs_sweep = float(vinp - vtail)
        else:
            vgs_sweep = np.linspace(0.1, 1.2, 10)

        # --- run Transistor LUT interpolation ---
        tr = Transistor(
            lookup_table_file   = cfg.lut_file,
            mos_type     = self._type,
            vsb = 0,
            vds                 = axis0.values,
            vgs                 = axis1.values,
            lengths              = cfg.lengths,
            dof          = [axis0.variable, axis1.variable],
            dof_values        = [vds_sweep, vgs_sweep],
        )

        # --- meshgrid for sweep shape ---
        #mesh = np.meshgrid(arr0, arr1)

        # length column — repeat to match mesh shape
        L_col = np.repeat(cfg.lengths, np.size(vds_sweep))

        # --- scale small-signal params to operating current ---
        il = self._il           # total tail current
        id = il / 2             # each branch

        W     = id / tr.jd
        gdsid = tr.gds / tr.id
        gds   = gdsid * id
        Ro    = 1.0 / gds
        gm    = tr.gmid * id
        cgg   = (W * tr.cgg) / cfg.lut_w
        cgs   = (W * tr.cgs) / cfg.lut_w
        cgd   = (W * tr.cgd) / cfg.lut_w

        # --- update layout param W (representative / median) ---
        self._layout_params["W"].default = float(np.nanmedian(W))

        df = pd.DataFrame({
            "length": np.asarray(L_col).flatten(),
            "width":  np.asarray(W).flatten(),
            "gm":     np.asarray(gm).flatten(),
            "gds":    np.asarray(gds).flatten(),
            "gdsid":  np.asarray(gdsid).flatten(),
            "Ro":     np.asarray(Ro).flatten(),
            "cgg":    np.asarray(cgg).flatten(),
            "cgs":    np.asarray(cgs).flatten(),
            "cgd":    np.asarray(cgd).flatten(),
        })

        # Attach operating point metadata
        df.attrs["primitive"]   = self.name
        df.attrs["il"]          = il
        df.attrs["lut_file"]    = cfg.lut_file
        df.attrs["port_voltages"] = {
            k: p.dc_voltage for k, p in self._ports.items()
        }

        return df