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

ROOT = Path(__file__).resolve().parents[3]
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
lib.register(ROOT / "analoglib/primitives/simplecurrentmirror")

# alternatively, register all primitives in the folder at once:
# lib.register_all("analoglib/primitives/")

print(lib)
# Expected: <Library 'ihp_sg13g2': ['simplecurrentmirror']>

# ---------------------------------------------------------------------------
# 2. Instantiate primitive and run LUT sweep
# ---------------------------------------------------------------------------
dp = lib.get("simplecurrentmirror", il=100e-6)
print(dp)
# Expected: <Primitive 'simplecurrentmirror' v1.0 | lut=path/to/LUTs/IHP_LUT_nmos.npy>
dp.set_port_voltages({
    "VINP":  1.1,
    "VINN":  1.1,
    "VOUTP": 1.1,
    "VOUTN": 1.1,
    "VDD": 1.8
})
print(dp.summary())

df = dp.build()

print("\n--- DataFrame ---")
print(f"shape: {df.shape}")
print(df.head())
