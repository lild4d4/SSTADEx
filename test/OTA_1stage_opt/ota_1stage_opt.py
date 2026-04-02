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

N_points = 100
lib = Library(
    name      = "ihp_sg13g2",
    lut_files = {
        "nmos": str(ROOT / "LUTs/ihp-sg13g2/lv_10w_nmos.npz"),
        "pmos": str(ROOT / "LUTs/ihp-sg13g2/lv_10w_pmos.npz"),
    },
)

lib.register_all(ROOT / "analoglib/primitives/")
print(lib)

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

I_bias = 5e-6
I_amp = 20e-6 

Iq_max = IL*(1-efficiency)
Ib_out = Iq_max-I_amp

#R1 = (Vout-Vref)/Ib_out
#R2 = Vref*R1/(Vout-Vref)

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
        "VOUTN": "NC",
        "VDD": "VDD",
    },
    index=0,
    netlist_params={
        "W_al": Symbol("W_al"),
        "L_al": Symbol("L_al"),
    },
)

#################### TESTBENCHES ##########################

tb_gain = Testbench(
    name="ota_1stage_gain",
    dut=OTA_1stage_macro,
    view="small_signal",
    elements=[
        CurrentSource("I2", "IBIAS", "VSS", 0),
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
        Symbol("I2"): 0,
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
        CurrentSource("I2", "IBIAS", "VSS", 0),
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
        Symbol("I2"): 0,
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

OTA_1stage_macro.specifications = [gain_1stage, rout_1stage]
OTA_1stage_macro.opt_specifications = [gain_1stage]
OTA_1stage_macro.primitives = [diffpair, currentmirror]
OTA_1stage_macro.submacromodels = []
OTA_1stage_macro.num_level_exp = -1
OTA_1stage_macro.is_primitive = 0
OTA_1stage_macro.run_pareto = True

_, _, _, ota_1stage_df, mask = dfs(OTA_1stage_macro, debug = False)

ota_1stage_df.to_csv('ota_1stage_df.csv')

ota_1stage_df["gain"] = 20*np.log10(ota_1stage_df["gain_1stage"])
fig, ax = plt.subplots()
ax.scatter(ota_1stage_df["area"], ota_1stage_df["gain"])
ax.set_xscale('log')
fig.tight_layout()
fig.savefig("gain.png", dpi=300)
plt.close(fig)


ota_1stage_df_filtered = ota_1stage_df[((ota_1stage_df[Symbol("W_diff")]>1e-6) & (ota_1stage_df[Symbol("W_al")]>1e-6))]
ota_1stage_df_filtered.to_csv('ota_1stage_df_filtered.csv')

point = {
    Symbol("W_diff"): 12e-6,
    Symbol("L_diff"): 0.4e-6,
    Symbol("W_al"): 8e-6,
    Symbol("L_al"): 0.4e-6,
}

netlist_text = OTA_1stage_macro.gen_netlist_for_params(point)
print(netlist_text)