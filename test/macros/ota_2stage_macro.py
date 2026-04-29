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

Vout = 1.1  
Vin = 1.5                             
Vref = 0.9                       
IL = 1e-3                                      
CL = 1e-12                                   
RL = Vout/IL

## OTA 2stage specifications

efficiency = 0.99
gain_condition = 70
estability_condition = 60
size_condition = 1e-3 

I_bias_ref = 20e-6
I_amp_1stage = 20e-6 
I_amp_2stage = 20e-6 

# Iq_max = IL*(1-efficiency)
# Ib_out = Iq_max-I_amp

R1 = (Vout-Vref)/1e-6
R2 = Vref*R1/(Vout-Vref)

print(f"R1: {R1:.2f} Ohm")
print(f"R2: {R2:.2f} Ohm")

########################## PRIMITIVES #########################################
N_points = 10
vs = np.linspace(0.2, Vout-0.1, N_points)
vout_1stage = np.linspace(Vref - 0.1, Vout+0.1, N_points)

ota_2stage = macro_lib.get(
    "ota_2stage_macro",
    N_points=10, 
    ROOT=ROOT,
    lengths=lengths,
    electrical_parameters={
        "Vdd": Vin,
        "Vref": Vref,
        "Vout": Vout,
        "I_amp_1stage": I_amp_1stage,
        "I_amp_2stage": I_amp_2stage,
        "vs": vs,
        "vout_1stage": vout_1stage
    },
    ports=["VINP", "VINN", "VOUT", "VDD", "VSS"],
    gain_condition=gain_condition
)

from pathlib import Path

output = Path("output")
output.mkdir(parents=True, exist_ok=True)

ota_2stage.num_level_exp = -1
ota_2stage.is_primitive = 0
ota_2stage.run_pareto = False

_, _, _, ota_2stage_df, mask = dfs(ota_2stage, debug = False)

ota_2stage_df["gain"] = 20*np.log10(ota_2stage_df["gain_2stage"])
#ota_2stage_df["gbw"] = 1/(2*np.pi*ota_2stage_df["rout_1stage"]*CL)

ota_2stage_df.to_csv('ota_2stage_df.csv')

ota_2stage_df_filtered = ota_2stage_df[((ota_2stage_df[Symbol("W_diff")]>1e-6) & (ota_2stage_df[Symbol("W_al")]>1e-6) & (ota_2stage_df[Symbol("W_cs_m1")]>1e-6) & (ota_2stage_df[Symbol("W_cs_m2")]>1e-6))]
ota_2stage_df_filtered.to_csv('ota_2stage_df_filtered.csv')