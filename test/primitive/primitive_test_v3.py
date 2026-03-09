"""
example.py
==========
Tests the folder-based primitive system with simplediffpair / IHP SG13G2.

Expected output:
    - Library registers simplediffpair from its folder
    - build() returns a DataFrame with shape (n_lengths * n_gmid_points, n_cols)
    - parameters and outputs dicts are populated on the primitive
    - engine interface (primitive.name, .parameters, .outputs) is accessible
"""

from pathlib import Path
import sys
import numpy as np

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from sstadex.models import Library

# ---------------------------------------------------------------------------
# 1. Create library for IHP SG13G2
#    Adjust lut paths to match your local setup
# ---------------------------------------------------------------------------
lib = Library(
    name      = "ihp_sg13g2",
    lut_files = {
        "nmos": str(ROOT / "LUTs/ihp-sg13g2/lv_10w_nmos.npz"),
        "pmos": str(ROOT / "LUTs/ihp-sg13g2/lv_10w_pmos.npz"),
    },
)

# register individual folder
lib.register(ROOT / "analoglib/primitives/simplediffpair")

# alternatively, register all primitives in the folder at once:
# lib.register_all("analoglib/primitives/")

print(lib)
# Expected: <Library 'ihp_sg13g2': ['simplediffpair']>

# ---------------------------------------------------------------------------
# 2. Instantiate primitive and run LUT sweep
# ---------------------------------------------------------------------------
dp = lib.get("simplediffpair", il=100e-6)
print(dp)
# Expected: <Primitive 'simplediffpair' v1.0 | lut=path/to/LUTs/IHP_LUT_nmos.npy>
dp.set_port_voltages({
    "VINP":  [0.9, 0.9, 0.9],
    "VINN":  0.9,
    "VOUTP": [0.9, 1, 1.1],
    "VOUTN": 1,
    "VTAIL": np.linspace(0.1, 0.9, 10)
})
print(dp.summary())

df = dp.build()

print("\n--- DataFrame ---")
print(f"shape: {df.shape}")
print(df.head())


dp.set_port_voltages({
    "VINP":  0.9,
    "VINN":  0.9,
    "VOUTP": 1,
    "VOUTN": 1,
    "VTAIL": np.linspace(0.1, 0.9, 10)
})
print(dp.summary())

df = dp.build()

print("\n--- DataFrame ---")
print(f"shape: {df.shape}")
print(df.head())

# ---------------------------------------------------------------------------
# 3. Check engine interface
#    These are the attributes sstadex.py and conditions.py consume
# ---------------------------------------------------------------------------
print("\n--- Engine interface ---")
print(f"name       : {dp.name}")
print(f"parameters : {list(dp.parameters.keys())}")
print(f"outputs    : {list(dp.outputs.keys())}")

# parameters should contain small-signal values: gm, gds, Ro, cgg, cgs, cgd, gdsid
# outputs should contain sizing values: length, width

# ---------------------------------------------------------------------------
# 4. Quick sanity check on values
# ---------------------------------------------------------------------------
import numpy as np

gm  = dp.parameters.get("gm")
Ro  = dp.parameters.get("Ro")
W   = dp.outputs.get("width")
L   = dp.outputs.get("length")

if gm is not None:
    print(f"\ngm  range : {np.nanmin(gm):.4e} -- {np.nanmax(gm):.4e} [A/V]")
    print(f"Ro  range : {np.nanmin(Ro):.4e} -- {np.nanmax(Ro):.4e} [Ohm]")
    print(f"W   range : {np.nanmin(W):.4e}  -- {np.nanmax(W):.4e}  [m]")
    print(f"L   values: {np.unique(L)}")
