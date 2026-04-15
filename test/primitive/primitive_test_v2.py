"""
example_usage.py
================
Demonstrates the new analoglib primitive workflow.

Three usage patterns:
  1. Single-point sizing (one operating point, e.g. for hand-verification)
  2. Design space exploration (full LUT sweep → DataFrame)
  3. Netlist export
"""

# ─── 1. Build the technology library ─────────────────────────────────────────

import sys

from sstadex.primitive import Library


sys.path.append("/home/daniel/SSTADEx/analoglib/primitives")  # adjust path as needed
from simplediffpair import SimpleDiffPair
# from analoglib.primitives.cs_pmos        import CommonSourcePMOS   # same pattern
# from analoglib.primitives.cm_pmos        import CurrentMirrorPMOS  # same pattern

IHP_LUT_NMOS = "/home/daniel/SSTADEx/LUTs/ihp-sg13g2/lv_10w_nmos.npz"
IHP_LUT_PMOS = "/home/daniel/SSTADEx/LUTs/ihp-sg13g2/lv_10w_pmos.npz"
LUT_W        = 10e-6     # reference width used during characterisation

lib = Library("ihp_sg13g2")

lib.register(
    SimpleDiffPair,
    lut_file        = IHP_LUT_NMOS,
    lut_w           = LUT_W,
    transistor_type = "sg13_lv_nmos",
)

# ─── 2a. Design Space Exploration ────────────────────────────────────────────
#
#  No port voltages set → uses default LUT ranges, returns the full sweep table.

dp_sweep = lib.instantiate("simplediffpair", il=10e-6)
print(dp_sweep.summary())

df_sweep = dp_sweep.build()
print("\n[Exploration] Full design space — first 5 rows:")
print(df_sweep.head())
print(f"Shape: {df_sweep.shape}")

# ─── 2b. Single operating-point sizing ───────────────────────────────────────
#
#  Port voltages are set → LUT is evaluated at that specific bias.

dp_point = lib.instantiate("simplediffpair", il=10e-6)
dp_point.set_port_voltages({
    "VINP":  0.9,
    "VINN":  0.9,
    "VOUTP": 1,
    "VOUTN": 1,
    "VTAIL": 0.2,
    "VDD":   1.5,
    "VSS":   0.0,
})

df_point = dp_point.build()
print("\n[Single Point] Operating point sizing:")
print(df_point)

# ─── 3. Netlist export ───────────────────────────────────────────────────────

netlist = dp_point.render_netlist(
    net_map={
        "VINP":  "net_inp",
        "VINN":  "net_inn",
        "VOUTP": "net_outp",
        "VOUTN": "net_outn",
        "VTAIL": "net_tail",
        "VDD":   "VDD",
        "VSS":   "VSS",
    },
    instance_name="XDIFFPAIR",
)
print("\n[Netlist]")
print(netlist)

# ─── 4. Query library ────────────────────────────────────────────────────────

print("\n[Library] Available primitives:", lib.list())
print(lib.describe("simplediffpair"))