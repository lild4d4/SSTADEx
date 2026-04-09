import pandas as pd
from pathlib import Path
import sys
import numpy as np
from sympy import Symbol
from sympy import lambdify
import matplotlib.pyplot as plt

from sstadex import Macromodel, Test, dfs, Testbench, VoltageSource, CurrentSource, Resistor, Capacitor

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from sstadex.models import Library

N_points = 10
lib = Library(
    name      = "ihp_sg13g2",
    lut_files = {
        "nmos": str(ROOT / "LUTs/ihp-sg13g2/lv_10w_nmos.npz"),
        "pmos": str(ROOT / "LUTs/ihp-sg13g2/lv_10w_pmos.npz"),
    },
)

lib.register_all(ROOT / "analoglib/primitives/")
print(lib)

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
gain_condition = 80
estability_condition = 60
size_condition = 1e-3 

I_bias_ref = 5e-6
I_amp_1stage = 20e-6 
I_amp_2stage = 20e-6 

# Iq_max = IL*(1-efficiency)
# Ib_out = Iq_max-I_amp

R1 = (Vout-Vref)/1e-6
R2 = Vref*R1/(Vout-Vref)

print(f"R1: {R1:.2f} Ohm")
print(f"R2: {R2:.2f} Ohm")

########################## PRIMITIVES #########################################s

vout_1stage = np.linspace(Vout-0.8, Vout, N_points)

# pmos common source
commonsource = lib.get("simplecommonsource", il=I_amp_2stage)

commonsource.set_port_voltages({
    "VIN": vout_1stage,
    "VOUT": Vout,
    "VDD": Vin
})

print(commonsource.summary())
commonsource_df = commonsource.build()
print("\n--- DataFrame ---")
print(f"shape: {commonsource_df.shape}")

commonsource.parameters = {
    Symbol('g_gm_xcos_m1'): commonsource_df['gm'].values,
    Symbol('R_gds_xcos_m1'): commonsource_df['Ro'].values,
}

commonsource.outputs = {
    Symbol("W_cos"): commonsource_df["width"].values,
    Symbol("L_cos"): commonsource_df["length"].values,
}

commonsource.interface_variables={
    'vout_1stage': np.repeat(vout_1stage, len(lengths))
}

commonsource_df.to_csv('commonsource.csv')

########################## MACROS ############################################

# OTA_1stage_macro 
OTA_2stage_macro = Macromodel(
    name = 'OTA_2stage_macro',
    ports=["VINP", "VINN", "VOUT", "VDD", "VSS"],
    outputs = [
        Symbol("W_cos"), Symbol("L_cos"),
        ],
    electrical_parameters = {
        "Vdd": Vin,
        "Vneg": Vref,
        "Vout": Vout,
        "Il": I_amp_2stage},
    macromodel_parameters={
        Symbol('Ra'): np.logspace(3, 7, N_points),
        Symbol('ga'): np.logspace(-5, -2, N_points),
        }
)

OTA_2stage_macro.interface_variables=[
    "vout_1stage"
]

# OTA_1stage_macro 
OTA_1stage_macro = Macromodel(
    name = 'OTA_1stage_macro',
    ports=["VINP", "VINN", "VOUT", "VDD", "VSS"],
    electrical_parameters = {
        "Vdd": OTA_2stage_macro.electrical_parameters['Vdd'],
        "Vneg": OTA_2stage_macro.electrical_parameters['Vneg'],
        "Vout": vout_1stage,
        "Il": I_amp_1stage},
    model="""Ra {VOUT} {VDD} 1
Ga {VOUT} {VDD} {VINP} {VINN} 1""",
    macromodel_parameters={
        Symbol('Ra'): np.logspace(3, 7, N_points),
        Symbol('ga'): np.logspace(-5, -2, N_points)}
)

OTA_2stage_macro.add_instance(
    "xcos",
    commonsource,
    {
        "VIN": "VOUT_1STAGE",
        "VOUT": "VOUT",
        "VDD": "VDD",
    },
    index=0,
    netlist_params={
        "W_cos": Symbol("W_cos"),
        "L_cos": Symbol("L_cos"),
    },
)

OTA_2stage_macro.add_instance(
    'xota_1stage',
    OTA_1stage_macro,
    {
        "VINP": "VINP",
        "VINN": "VINN",
        "VOUT": "VOUT_1STAGE",
        "VDD": "VDD",
        "VSS": "VSS"
    },
    index=0,
)

#################### TESTBENCHES ##########################

tb_gain = Testbench(
    name="ota_2stage_gain",
    dut=OTA_2stage_macro,
    view="small_signal",
    elements=[
        CurrentSource("Ibias", "VOUT", "VSS", 0),
        VoltageSource("Vdd", "VDD", "VSS", 0),
        VoltageSource("Vss", "VSS", "VSS", 0),
        VoltageSource("V_n", "VINN", "VSS", 0),
        VoltageSource("V_p", "VINP", "VSS", 1),
    ],
    tf=("VOUT", "VINP"),
    parameter_map={
        Symbol("Vdd"): 0,
        Symbol("Vss"): 0,
        Symbol("V_n"): 0,
        Symbol("V_p"): 1,
        Symbol("Ibias"): 0,
        Symbol("s"): 0,
    },
)

gain_2stage = tb_gain.make_test(
    name="gain_2stage",
    opt_goal="max",
    conditions={"min": [10 ** (-100 / 20)]},
)

OTA_2stage_macro.specifications = [gain_2stage]
OTA_2stage_macro.opt_specifications = [gain_2stage]
OTA_2stage_macro.primitives = [commonsource]
OTA_2stage_macro.submacromodels = [OTA_1stage_macro]
OTA_2stage_macro.num_level_exp = 1
OTA_2stage_macro.is_primitive = 0
OTA_2stage_macro.run_pareto = False

_, _, _, ota_2stage_df, mask = dfs(OTA_2stage_macro, debug = False)

ota_2stage_df.to_csv('ota_2stage_df.csv')

ota_2stage_df["gain"] = 20*np.log10(ota_2stage_df["gain_2stage"])
ota_2stage_df["gain_1stage"] = 20*np.log10(ota_2stage_df[Symbol('Ra')]*ota_2stage_df[Symbol('ga')])
fig, ax = plt.subplots()
ax.scatter(ota_2stage_df["area"], ota_2stage_df["gain"])
ax.set_xscale('log')
fig.tight_layout()
fig.savefig("gain.png", dpi=300)
plt.close(fig)

ota_1stage_df_filtered = ota_2stage_df[((ota_2stage_df[Symbol("W_cos")]>1e-6) & (ota_2stage_df[Symbol("W_cos")]<100e-6) & (ota_2stage_df[Symbol("L_cos")]>1e-6) & (ota_2stage_df["gain"]>70))]
ota_1stage_df_filtered.to_csv('ota_2stage_df_filtered.csv')

point = {
    Symbol("W_cos"): ota_1stage_df_filtered[Symbol("W_cos")].to_numpy(),
    Symbol("L_cos"): ota_1stage_df_filtered[Symbol("L_cos")].to_numpy(),
}

# simulations = OTA_1stage_macro.ngspice_sim(
#     point,
#     variables=["gain", "vout"],
#     extra_spice = {
#         "pre": [
#             "**",
#             "x1 net1 vn vout vdd vss OTA_2stage_macro", # VINP VINN VOUTP VDD IBIAS
#             "R1 vfb vout 100000000 m=1",
#             "C2 vfb vss 10 m=1",
#             "R2 vfb vss 900000000 m=1",
#             "V1 net1 vfb dc 0 ac 1",
#             "I0 vout vss 20e-6",
#             ".lib /home/daniel/SSTADEX-prev/IHP-Open-PDK/ihp-sg13g2/libs.tech/ngspice/models/cornerMOSlv.lib mos_tt",
#             "Vref vn 0 0.9",
#             "Vdd vdd 0 1.5",
#             "Vss vss 0 0",
#             ".control",
#             "pre_osdi /home/daniel/SSTADEX-prev/IHP-Open-PDK/ihp-sg13g2/libs.tech/ngspice/osdi/psp103_nqs.osdi",
#             "ac dec 10 1 1G",
#             "meas ac gain find vdb(vout) at=1000",
#             "op",
#             "print v(vout)",
#             ".endc",
#             ".end"
#         ]
#     })

# print(simulations)

# sim_results_df = pd.DataFrame(
#     [
#         {
#             **sim_result["params"],
#             **{
#                 f"sim_{name}": value
#                 for name, value in sim_result.get("variables", {}).items()
#             },
#             "run_name": sim_result["run_name"],
#             "returncode": sim_result["returncode"],
#         }
#         for sim_result in simulations
#     ]
# )

# ota_1stage_comparison_df = pd.concat(
#     [
#         ota_1stage_df_filtered.reset_index(drop=True),
#         sim_results_df[
#             ["run_name", "returncode", "sim_gain", "sim_vout"]
#         ].reset_index(drop=True),
#     ],
#     axis=1,
# )

# ota_1stage_comparison_df["gain_error"] = 100*np.abs(ota_1stage_comparison_df["gain"] - ota_1stage_comparison_df["sim_gain"])/ota_1stage_comparison_df["gain"]

# ota_1stage_comparison_df.to_csv("ota_2stage_comparison.csv", index=False)
# print(ota_1stage_comparison_df)