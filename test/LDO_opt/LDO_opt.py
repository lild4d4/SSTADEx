from pathlib import Path
import sys

import numpy as np
from sympy import Symbol

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from sstadex import Library, MacroLibrary, Macromodel, dfs


primitive_lib = Library(
    name="ihp_sg13g2",
    lut_files={
        "nmos": str(ROOT / "LUTs/ihp-sg13g2/lv_5w_nmos.npz"),
        "pmos": str(ROOT / "LUTs/ihp-sg13g2/lv_5w_pmos.npz"),
    },
)
primitive_lib.register_all(ROOT / "analoglib/primitives")

macro_lib = MacroLibrary(
    name="ihp_sg13g2_macros",
    primitive_library=primitive_lib,
)
macro_lib.register_all(ROOT / "analoglib/macros")

lengths = [4e-7, 8e-7, 1.6e-6, 3.2e-6, 6.4e-6]

## Exploration parameters

N_points = 10
lengths_nmos = [0.4e-06, 0.8e-06, 1.6e-06, 3.2e-06, 6.4e-6]
lengths_pmos = [0.4e-06, 0.8e-06, 1.6e-06, 3.2e-06, 6.4e-6]

lut_file = "../../LUTs/ihp_lut_lv_10w.npy"
lut_w = 10e-6

## Electrical parameters

Vout = 1.2                                      # LDO output voltage
Vin = 1.8                                       # LDO supply voltage
Vref = 0.9                                      # LDO voltage reference
IL = 5e-3                                     # Load current
CL = 0.5e-12                                    # Load capacitance
RL = Vout/IL

## LDO specifications

efficiency = 0.98
psrr_condition = -70
estability_condition = 60
linereg_condition = 0.5
loadreg_condition = 0.7
#iq_condition = 20e-6
size_condition = 1e-3

I_bias = 5e-6
I_amp1 = 10e-6 # considering the two branches of differential pair
I_amp2 = 10e-6

Iq_max = IL*(1-efficiency)
Ib_pt = Iq_max-I_amp1-I_amp2-I_bias

R1 = (Vout-Vref)/Ib_pt
R2 = Vref*R1/(Vout-Vref)
