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

## Electrical parameters

Vout = 1
Vref = 0.9                                       
Vdd = 1.5                           
CL = 1e-12 

## OTA specifications

gain_condition = 30
estability_condition = 60
size_condition = 1e-3

I_bias_ref = 20e-6
I_amp = 20e-6 

N_points=10
vs = np.linspace(0.2, Vout-0.1, N_points)

ota_1stage = macro_lib.get(
    "ota_1stage_macro",
    N_points=10, 
    ROOT=ROOT,
    lengths=lengths,
    electrical_parameters={
        "Vdd": Vdd,
        "Vref": Vref,
        "Vout": Vout,
        "I_amp": I_amp,
        "vs": vs
    },
    ports=["VINP", "VINN", "VOUT", "VDD", "VSS"]
)

from pathlib import Path

output = Path("output")
output.mkdir(parents=True, exist_ok=True)

ota_1stage.num_level_exp = -1
ota_1stage.run_pareto = False

_, _, _, ota_1stage_df, mask = dfs(ota_1stage, debug = False)

ota_1stage_df["gain"] = 20*np.log10(ota_1stage_df["gain_1stage"])
ota_1stage_df["gbw"] = 1/(2*np.pi*ota_1stage_df["rout_1stage"]*CL)

ota_1stage_df.to_csv('ota_1stage_df.csv')

ota_1stage_df_filtered = ota_1stage_df[((ota_1stage_df[Symbol("W_diff")]>1e-6) & (ota_1stage_df[Symbol("W_al")]>1e-6) & (ota_1stage_df[Symbol("W_cs_m1")]>1e-6) & (ota_1stage_df[Symbol("W_cs_m2")]>1e-6))]
ota_1stage_df_filtered.to_csv('ota_1stage_df_filtered.csv')