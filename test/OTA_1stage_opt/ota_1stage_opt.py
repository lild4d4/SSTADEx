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

Vout = 1                                    # LDO output voltage
Vin = 1.5                                      # LDO supply voltage
Vref = 0.9                                      # LDO voltage reference
IL = 1e-3                                      # Load current
CL = 1e-12                                    # Load capacitance
RL = Vout/IL

## LDO specifications

efficiency = 0.99
gain_condition = 80
estability_condition = 60
size_condition = 1e-3

I_bias_ref = 5e-6
I_amp = 20e-6 

Iq_max = IL*(1-efficiency)
Ib_out = Iq_max-I_amp

#R1 = (Vout-Vref)/1e-6
#R2 = Vref*R1/(Vout-Vref)

#print(f"R1: {R1:.2f} Ohm")
#print(f"R2: {R2:.2f} Ohm")

#LDO = pd.DataFrame.from_dict({'Vout': [Vout], 'Vin': [Vin], 'Vref': [Vref], 'IL': [IL], 'CL': [CL], 'RL': [RL], 'Iq_max': [Iq_max], 'Ib_out': [Ib_out], 'R1': [R1], 'R2': [R2]}, orient='index', columns=['Value'])


########################## PRIMITIVES ############################################

vs = np.linspace(0.2, Vout-0.1, N_points)

# diff pair
diffpair = lib.get("simplediffpair", il=I_amp)
print(diffpair)

diffpair.set_port_voltages({
    "VINP":  Vref,
    "VINN":  Vref,
    "VOUTP": Vout,
    "VOUTN": Vout,
    "VTAIL": vs
})
print(diffpair.summary())

diffpair_df = diffpair.build()

print("\n--- DataFrame ---")
print(f"shape: {diffpair_df.shape}")
print(diffpair_df.head())

#diffpair_mask = (diffpair_df["width"]>3e-6) & (diffpair_df["width"]<3e-4)
#diffpair_df = diffpair_df[diffpair_mask]

diffpair.parameters = {
    Symbol('g_gm_xdp_m1'): diffpair_df['gm'].values,
    Symbol('R_gds_xdp_m1'): diffpair_df['Ro'].values,
}

diffpair.outputs = {
    Symbol("W_diff"): diffpair_df["width"].values,
    Symbol("L_diff"): diffpair_df["length"].values,
}

diffpair_df["vs"] = np.tile(vs, len(lengths))
diffpair.interface_variables={
    'vs_diff': np.tile(vs, len(lengths))
}

diffpair_df.to_csv('diffpair.csv')

# current mirror
currentmirror = lib.get("simplecurrentmirror", il=I_amp)
print(currentmirror)

currentmirror.set_port_voltages({
    "VINP":  Vout,
    "VINN":  Vout,
    "VOUTP": Vout,
    "VOUTN": Vout,
    "VDD": Vin
})
print(currentmirror.summary())

currentmirror_df = currentmirror.build()

print("\n--- DataFrame ---")
print(f"shape: {currentmirror_df.shape}")
print(currentmirror_df.head())

#currentmirror_mask = (currentmirror_df["width"]>3e-6) & (currentmirror_df["width"]<3e-4)
#currentmirror_df = currentmirror_df[currentmirror_mask]

currentmirror.parameters = {
    Symbol('g_gm_xcm_m1'): currentmirror_df['gm'].values,
    Symbol('R_gds_xcm_m1'): currentmirror_df['Ro'].values,
}

currentmirror.outputs = {
    Symbol("W_al"): currentmirror_df["width"].values,
    Symbol("L_al"): currentmirror_df["length"].values,
}

currentmirror_df.to_csv('currentmirror.csv')

currentsource = lib.get('simplecurrentsource', il=I_amp)
print(currentsource)

currentsource.set_port_voltages({
    "VOUTP": vs,
    "VSS": 0,
    "VINP": vs, 
})
print(currentsource.summary())

currentsource_df = currentsource.build(ref_current=I_amp)

print("\n--- DataFrame ---")
print(f"shape: {currentsource_df.shape}")
print(currentsource_df.head())

currentsource.parameters = {
    Symbol('g_gm_cs_m1'): currentsource_df['gm'].values,
    Symbol('R_gds_cs_m1'): currentsource_df['Ro'].values
}

currentsource.outputs = {
    Symbol("W_cs_m1"): currentsource_df["width_m1"].values,
    Symbol("W_cs_m2"): currentsource_df["width_m2"].values,
    Symbol("L_cs"): currentsource_df["length"].values,
}

currentsource.interface_variables={
    #'vs_cs': np.repeat(np.tile(vs, len(lengths)), 5),
    'vs_cs': np.tile(vs, len(lengths)),
    'vgs_cs': currentsource_df["vgs_cs"].values,
}

print('vs_cs: ', currentsource.interface_variables['vs_cs'])
print('vgs_cs: ', currentsource.interface_variables['vgs_cs'])

# currentsource_mask = (currentsource_df["width_m1"]>1e-6) & (currentsource_df["width_m2"]>1e-6)
# currentsource_df = currentsource_df[currentsource_mask]

currentsource_df.to_csv('currentsource.csv')

########################## MACROMODELS ############################################

# OTA_1stage_macro 
OTA_1stage_macro = Macromodel(
    name = 'OTA_1stage_macro',
    ports=["VINP", "VINN", "VOUT", "VDD", "IBIAS"],
    outputs = [
        Symbol("W_diff"), Symbol("L_diff"), 
        Symbol("W_al"), Symbol("L_al")],
    electrical_parameters = {
        "Vdd": Vin,
        "Vneg": Vref,
        "Vout": Vout,
        "Il": I_amp},
    macromodel_parameters={
        Symbol('Ra'): np.logspace(3, 7, N_points),
        Symbol('gma'): np.logspace(-5, -2, N_points)}
    )

OTA_1stage_macro.interface_variables = [
    "vs_diff",
]

OTA_1stage_macro.shared_nodes = {
    "IBIAS_node": ["vs_diff", "vs_cs"],
}

OTA_1stage_macro.add_instance(
    "xdp",
    diffpair,
    {
        "VINP": "VINP",
        "VINN": "VINN",
        "VOUTP": "VOUT",
        "VOUTN": "N1",
        "VTAIL": "IBIAS",
    },
    index=0,
    netlist_params={
        "W_diff": Symbol("W_diff"),
        "L_diff": Symbol("L_diff"),
    },
)

OTA_1stage_macro.add_instance(
    "xcm",
    currentmirror,
    {
        "VINP": "N1",
        "VINN": "N1",
        "VOUTP": "VOUT",
        "VOUTN": "N1",
        "VDD": "VDD",
    },
    index=0,
    netlist_params={
        "W_al": Symbol("W_al"),
        "L_al": Symbol("L_al"),
    },
)

current_source_macro = Macromodel(
    name = 'current_source_macro',
    ports = ['VOUT', 'VSS'],
    outputs = [
        Symbol("W_cs_m1"),
        Symbol("W_cs_m2"),
        Symbol("L_cs"),
    ],
    electrical_parameters = {
        'Iref': I_bias_ref,
        'Ibias': I_amp,
        'Vdd': Vin,
    },
    model="""I{INSTANCE} {VOUT} {VSS} 0""",
    macromodel_parameters = {
        Symbol('Ics'): [I_amp]
    }
)

current_source_macro.interface_variables=[
    "vs_cs",
    "vgs_cs"
]

OTA_1stage_macro.add_instance(
    "cs",
    current_source_macro,
    {
        "VOUT": "IBIAS",
        "VSS": "VSS"
    },
    index=0,
    netlist_params={
        'Ibias': Symbol('Ibias')
    }
)

current_source_macro.add_instance(
    "cs",
    currentsource,
    {
        "VOUTP": "VOUT",
        "VSS": "VSS", 
        "VOUTN": "Vbias",
        "VINP": "Vbias"
    },
    index=0,
    netlist_params={
        "W_cs_m1": Symbol("W_cs_m1"),
        "W_cs_m2": Symbol("W_cs_m2"),
        "L_cs": Symbol("L_cs"),
    }
)

#################### TESTBENCHES ##########################

tb_gain = Testbench(
    name="ota_1stage_gain",
    dut=OTA_1stage_macro,
    view="small_signal",
    elements=[
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
        Symbol("g_gm_xdp_m2"): Symbol("g_gm_xdp_m1"),
        Symbol("R_gds_xdp_m2"): Symbol("R_gds_xdp_m1"),
        Symbol("g_gm_xcm_m2"): Symbol("g_gm_xcm_m1"),
        Symbol("R_gds_xcm_m2"): Symbol("R_gds_xcm_m1"),
        Symbol("s"): 0,
    },
)

gain_1stage = tb_gain.make_test(
    name="gain_1stage",
    opt_goal="max",
    conditions={"min": [10 ** (-100 / 20)]},
)

tb_rout = Testbench(
    name="ota_1stage_rout",
    dut=OTA_1stage_macro,
    view="small_signal",
    elements=[
        VoltageSource("Vdd", "VDD", "VSS", 0),
        VoltageSource("Vss", "VSS", "VSS", 0),
        VoltageSource("V_n", "VINN", "VSS", 0),
        VoltageSource("V_p", "VINP", "VSS", 0),
        VoltageSource("Vr", "VR", "VSS", 1),
        Resistor("Rr", "VR", "VOUT", 1000),
    ],
    tf=("VOUT", "VR"),
    parameter_map={
        Symbol("Vdd"): 0,
        Symbol("Vss"): 0,
        Symbol("V_n"): 0,
        Symbol("V_p"): 0,
        Symbol("Vr"): 1,
        Symbol("Rr"): 1000,
        Symbol("g_gm_xdp_m2"): Symbol("g_gm_xdp_m1"),
        Symbol("R_gds_xdp_m2"): Symbol("R_gds_xdp_m1"),
        Symbol("g_gm_xcm_m2"): Symbol("g_gm_xcm_m1"),
        Symbol("R_gds_xcm_m2"): Symbol("R_gds_xcm_m1"),
        Symbol("s"): 0,
    },
)

rout_1stage = tb_rout.make_test(
    name="rout_1stage",
    opt_goal="max",
    conditions={"min": [1]},
    lamd=lambda x: x * 1000 / (1 - x),
)

tb_ibias = Testbench(
    name="currentsource_ibas",
    dut=current_source_macro,
    view="small_signal",
    elements=[
        VoltageSource("Vdd", "VDD", "VSS", 0),
        VoltageSource("Vss", "VSS", "VSS", 0),
        CurrentSource("I0", "Vbias", "VSS", 0),
    ],
    tf=("VOUT", "Vbias"),
    parameter_map={
        Symbol("Vdd"): 0,
        Symbol("Vss"): 0,
        Symbol("I0"): 0,
        Symbol("s"): 0,
        Symbol('g_gm_cs_m2'): Symbol('g_gm_cs_m1'),
        Symbol('R_gds_cs_m2'): Symbol('R_gds_cs_m1')
    }
)

ibias_currentsource = tb_ibias.make_test(
    name="ibias_currentsource",
    opt_goal="max",
    conditions={"min": [0]},
)

current_source_macro.specifications=[ibias_currentsource]
current_source_macro.primitives = [currentsource]
current_source_macro.num_level_exp = 1
current_source_macro.run_pareto = False

OTA_1stage_macro.specifications = [gain_1stage, rout_1stage]
OTA_1stage_macro.opt_specifications = [gain_1stage]
OTA_1stage_macro.primitives = [diffpair, currentmirror]
OTA_1stage_macro.submacromodels = [current_source_macro]
OTA_1stage_macro.num_level_exp = -1
OTA_1stage_macro.is_primitive = 0
OTA_1stage_macro.run_pareto = False

_, _, _, ota_1stage_df, mask = dfs(OTA_1stage_macro, debug = False)

ota_1stage_df.to_csv('ota_1stage_df.csv')

ota_1stage_df["gain"] = 20*np.log10(ota_1stage_df["gain_1stage"])
fig, ax = plt.subplots()
ax.scatter(ota_1stage_df["area"], ota_1stage_df["gain"])
ax.set_xscale('log')
fig.tight_layout()
fig.savefig("gain.png", dpi=300)
plt.close(fig)


ota_1stage_df_filtered = ota_1stage_df[((ota_1stage_df[Symbol("W_diff")]>1e-6) & (ota_1stage_df[Symbol("W_al")]>1e-6) & (ota_1stage_df[Symbol("W_cs_m1")]>1e-6) & (ota_1stage_df[Symbol("W_cs_m2")]>1e-6))]
ota_1stage_df_filtered.to_csv('ota_1stage_df_filtered.csv')

print("w_diff", ota_1stage_df_filtered[Symbol("W_diff")].to_numpy())

# point = {
#     Symbol("W_diff"): ota_1stage_df_filtered[Symbol("W_diff")].to_numpy(),
#     Symbol("L_diff"): ota_1stage_df_filtered[Symbol("L_diff")].to_numpy(),
#     Symbol("W_al"): ota_1stage_df_filtered[Symbol("W_al")].to_numpy(),
#     Symbol("L_al"): ota_1stage_df_filtered[Symbol("L_al")].to_numpy(),
# }

# simulations = OTA_1stage_macro.ngspice_sim(
#     point,
#     variables=["gain", "vout"],
#     extra_spice = {
#         "pre": [
#             "**",
#             "x1 net1 vn vout vdd ibias OTA_1stage_macro", # VINP VINN VOUTP VDD IBIAS
#             "R1 vfb vout 100000000 m=1",
#             "C2 vfb vss 10 m=1",
#             "R2 vfb vss 900000000 m=1",
#             "V1 net1 vfb dc 0 ac 1",
#             "I1 ibias vss 20e-6",
#             ".lib /home/designer/shared/SSTADEx/IHP-Open-PDK/ihp-sg13g2/libs.tech/ngspice/models/cornerMOSlv.lib mos_tt",
#             "Vref vn 0 0.9",
#             "Vdd vdd 0 1.5",
#             "Vss vss 0 0",
#             ".control",
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

# ota_1stage_comparison_df["error"] = np.abs(ota_1stage_comparison_df["gain"] - ota_1stage_comparison_df["sim_gain"])

# ota_1stage_comparison_df.to_csv("ota_1stage_comparison.csv", index=False)
# print(ota_1stage_comparison_df)
