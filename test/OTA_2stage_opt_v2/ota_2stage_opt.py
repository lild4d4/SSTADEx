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

vs = np.linspace(0.2, Vout-0.1, N_points)
vout_1stage = np.linspace(Vref - 0.1, Vout+0.1, N_points)

#Instantiation
diffpair = lib.get("simplediffpair", il=I_amp_1stage)
currentmirror = lib.get("simplecurrentmirror", il=I_amp_1stage)
commonsource = lib.get("simplecommonsource", il=I_amp_2stage)

#Ports voltages definition
diffpair.set_port_voltages({
    "VINP":  Vref,
    "VINN":  Vref,
    "VOUTP": vout_1stage,
    "VOUTN": vout_1stage,
    "VTAIL": vs
})

currentmirror.set_port_voltages({
    "VINP":  vout_1stage,
    "VINN":  vout_1stage,
    "VOUTP": vout_1stage,
    "VOUTN": vout_1stage,
    "VDD": Vin
})

commonsource.set_port_voltages({
    "VIN": vout_1stage,
    "VOUT": Vout,
    "VDD": Vin
})

#Builds
diffpair_df = diffpair.build()
currentmirror_df = currentmirror.build()
commonsource_df = commonsource.build()

diffpair_df.to_csv("diffpair.csv")
currentmirror_df.to_csv("currentmirror.csv")
commonsource_df.to_csv("commonsource.csv")

#Parameters definition
diffpair.parameters = {
    Symbol('g_gm_xdp_m1'): diffpair_df['gm'].values,
    Symbol('R_gds_xdp_m1'): diffpair_df['Ro'].values,
}

currentmirror.parameters = {
    Symbol('g_gm_xcm_m1'): currentmirror_df['gm'].values,
    Symbol('R_gds_xcm_m1'): currentmirror_df['Ro'].values,
}

commonsource.parameters = {
    Symbol('g_gm_xcos_m1'): commonsource_df['gm'].values,
    Symbol('R_gds_xcos_m1'): commonsource_df['Ro'].values,
}

#Outputs definition
diffpair.outputs = {
    Symbol("W_diff"): diffpair_df["width"].values,
    Symbol("L_diff"): diffpair_df["length"].values,
}

currentmirror.outputs = {
    Symbol("W_al"): currentmirror_df["width"].values,
    Symbol("L_al"): currentmirror_df["length"].values,
}

commonsource.outputs = {
    Symbol("W_cos"): commonsource_df["width"].values,
    Symbol("L_cos"): commonsource_df["length"].values,
}

########################## MACROS #########################################

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

OTA_1stage_macro = Macromodel(
    name = 'OTA_1stage_macro',
    ports=["VINP", "VINN", "VOUT", "VDD", "VSS"],
    outputs = [
        Symbol("W_diff"), Symbol("L_diff"), 
        Symbol("W_al"), Symbol("L_al")],
    electrical_parameters = {
        "Vdd": OTA_2stage_macro.electrical_parameters["Vdd"],
        "Vneg": OTA_2stage_macro.electrical_parameters["Vneg"],
        "Vout": vout_1stage,
        "Il": I_amp_1stage},
    model="""Ra {VOUT} {VDD} 1
Ga {VOUT} {VDD} {VINP} {VINN} 1""",
    macromodel_parameters={
        Symbol('Ra'): np.logspace(3, 7, N_points),
        Symbol('ga'): np.logspace(-5, -2, N_points)
    }
)

current_source_macro = Macromodel(
    name = 'current_source_macro',
    ports = ['VOUT', 'VSS', 'Vbias'],
    outputs = [
        Symbol("W_cs_m1"),
        Symbol("W_cs_m2"),
        Symbol("L_cs"),
    ],
    electrical_parameters = {
        'Iref': I_bias_ref,
        'Ibias': I_amp_1stage,
        'Vdd': Vin,
    },
    model="""I{INSTANCE} {VOUT} {VSS} 20e-6""",
    macromodel_parameters = {
        Symbol('Ixcs_macro'): [I_bias_ref]
    }
)

########################## INSTANCES #####################################

#Primitive Instances

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
        "ng_cos": Symbol("ng_cos"),
    },
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
        "ng_diff": Symbol("ng_diff"),
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
        "ng_al": Symbol("ng_al"),
    },
)

#Macros Instances as submacros

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

OTA_1stage_macro.add_instance(
    "xcs_macro",
    current_source_macro,
    {
        "VOUT": "IBIAS",
        "VSS": "VSS",
        "Vbias": "Vbias"
    },
    index=0,
    netlist_params={
        'Ibias': Symbol('Ibias')
    }
)

########################## Shared Nodes #####################################

currentmirror.interface_variables={
    'vout_1stage_currentmirror': np.tile(vout_1stage, len(lengths))
}

diffpair.interface_variables={
    'vout_1stage_diffpair': np.tile(np.repeat(vout_1stage, N_points), len(lengths))
}

commonsource.interface_variables={
    'vout_1stage_commonsource': np.repeat(vout_1stage, len(lengths))
}

OTA_2stage_macro.interface_variables=[
    "vout_1stage_commonsource"
]

OTA_1stage_macro.interface_variables=[
    "vout_1stage_currentmirror",
    'vout_1stage_diffpair'
]

OTA_1stage_macro.shared_nodes = {
    "IBIAS_node": ["vout_1stage_currentmirror", "vout_1stage_diffpair"],
}

OTA_2stage_macro.shared_nodes = {
    "IBIAS_node": ["vout_1stage_diffpair", "vout_1stage_commonsource"],
}


########################## CONDITIONS #####################################

OTA_1stage_macro.derived_metrics = {
    "gain_1stage_proxy": lambda df: df[Symbol("Ra")] * df[Symbol("ga")],
}

OTA_2stage_macro.submacro_condition_rules = {
    OTA_1stage_macro: [
        {
            "kind": "range_from_submacro_metric",
            "metric": "gain_1stage_proxy",
            "target_column": "gain_1stage",
            "bound": "min",
            "margin_factor": 1.0,
        },
    ]
}

OTA_2stage_macro.propagated_conditions = {
    "direct": [
        {
            "kind": "range",
            "column": Symbol("W_cos"),
            "condition": {"min": 1e-6, "max": 1000e-6},
        },
    ],
    "derived": [],
}

OTA_1stage_macro.propagated_conditions = {
    "direct": [
        {
            "kind": "range",
            "column": Symbol("W_al"),
            "condition": {"min": 1e-6, "max": 1000e-6},
        },
        {
            "kind": "range",
            "column": Symbol("W_diff"),
            "condition": {"min": 1e-6, "max": 1000e-6},
        },
    ],
    "derived": [],
}

########################## TEST BENCH #####################################

tb_gain_1stage = Testbench(
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

gain_1stage = tb_gain_1stage.make_test(
    name="gain_1stage",
    opt_goal="max",
    conditions={"min": [10 ** (-100 / 20)]},
)

tb_rout_1stage = Testbench(
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

rout_1stage = tb_rout_1stage.make_test(
    name="rout_1stage",
    opt_goal="max",
    target_param=Symbol("Ra"),
    conditions={"min": [1]},
    lamd=lambda x: x * 1000 / (1 - x),
)

tb_gm_1stage = Testbench(
    name="ota_1stage_gm",
    dut=OTA_1stage_macro,
    tf=("VOUT", "VINP"),
)

gm_1stage = tb_gm_1stage.make_test(
    name="gm_1stage",
    opt_goal="max",
    composed=1,
    out_def = {"divide": [gain_1stage, rout_1stage]},
    conditions={"min": [1e-10]},
    target_param=Symbol("ga"),
)

tb_gain_2stage = Testbench(
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

gain_2stage = tb_gain_2stage.make_test(
    name="gain_2stage",
    opt_goal="max",
    conditions={"min": [10 ** (gain_condition / 20)]},
)

 ######### 

OTA_1stage_macro.specifications = [gain_1stage, rout_1stage, gm_1stage]
OTA_1stage_macro.opt_specifications = [gain_1stage]
OTA_1stage_macro.primitives = [diffpair, currentmirror]
OTA_1stage_macro.submacromodels = [current_source_macro]
OTA_1stage_macro.num_level_exp = 1
OTA_1stage_macro.is_primitive = 0
OTA_1stage_macro.run_pareto = False

OTA_2stage_macro.specifications = [gain_2stage]
OTA_2stage_macro.opt_specifications = [gain_2stage]
OTA_2stage_macro.primitives = [commonsource]
OTA_2stage_macro.submacromodels = [OTA_1stage_macro]
OTA_2stage_macro.num_level_exp = -1
OTA_2stage_macro.is_primitive = 0
OTA_2stage_macro.run_pareto = True

_, _, _, ota_2stage_df, mask = dfs(OTA_2stage_macro, debug = False)

ota_2stage_df["1stage_gain"] = 20*np.log10(ota_2stage_df[Symbol("Ra")] * ota_2stage_df[Symbol("ga")])
ota_2stage_df["gain"] = 20*np.log10(ota_2stage_df["gain_2stage"])

ota_2stage_df.to_csv('ota_2stage_df.csv')

fig, ax = plt.subplots()
ax.scatter(ota_2stage_df["area"], ota_2stage_df["gain"])
ax.set_xscale('log')
fig.tight_layout()
fig.savefig("gain.png", dpi=300)
plt.close(fig)

simulation_wcos = ota_2stage_df[Symbol("W_cos")].to_numpy()
simulation_wdiff = ota_2stage_df[Symbol("W_diff")].to_numpy()
simulation_wal = ota_2stage_df[Symbol("W_al")].to_numpy()

wcos_max = 10e-6
ng_cos = np.ceil(simulation_wcos / wcos_max).astype(int)
ng_diff = np.ceil(simulation_wdiff / wcos_max).astype(int)
ng_al = np.ceil(simulation_wal / wcos_max).astype(int)

point = {
    Symbol("W_diff"): ota_2stage_df[Symbol("W_diff")].to_numpy(),
    Symbol("L_diff"): ota_2stage_df[Symbol("L_diff")].to_numpy(),
    Symbol("ng_diff"): ng_diff,
    Symbol("W_al"): ota_2stage_df[Symbol("W_al")].to_numpy(),
    Symbol("L_al"): ota_2stage_df[Symbol("L_al")].to_numpy(),
    Symbol("ng_al"): ng_al,
    Symbol("W_cos"): ota_2stage_df[Symbol("W_cos")].to_numpy(),
    Symbol("L_cos"): ota_2stage_df[Symbol("L_cos")].to_numpy(),
    Symbol("ng_cos"): ng_cos
}

simulations = OTA_2stage_macro.ngspice_sim(
    point,
    variables=["gain", "vout"],
    extra_spice = {
        "pre": [
            "**",
            "x1 vref vret vout vdd vss OTA_2stage_macro", # VINP VINN VOUTP VDD IBIAS
            "R1 vfb vout 200000000 m=1",
            "C2 vfb vss 10 m=1",
            "R2 vfb vss 900000000 m=1",
            "V1 vret vfb dc 0 ac 1",
            "I0 vout vss 20e-6",
            ".lib /home/daniel/SSTADEX-prev/IHP-Open-PDK/ihp-sg13g2/libs.tech/ngspice/models/cornerMOSlv.lib mos_tt",
            "Vref vref 0 0.9",
            "Vdd vdd 0 1.5",
            "Vss vss 0 0",
            ".control",
            "pre_osdi /home/daniel/SSTADEX-prev/IHP-Open-PDK/ihp-sg13g2/libs.tech/ngspice/osdi/psp103_nqs.osdi",
            "ac dec 10 1 1G",
            "meas ac gain find vdb(vout) at=1000",
            "op",
            "print v(vout)",
            ".endc",
            ".end"
        ]
    })

print(simulations)

sim_results_df = pd.DataFrame(
    [
        {
            **sim_result["params"],
            **{
                f"sim_{name}": value
                for name, value in sim_result.get("variables", {}).items()
            },
            "run_name": sim_result["run_name"],
            "returncode": sim_result["returncode"],
        }
        for sim_result in simulations
    ]
)

ota_2stage_comparison_df = pd.concat(
    [
        ota_2stage_df.reset_index(drop=True),
        sim_results_df[
            ["run_name", "returncode", "sim_gain", "sim_vout"]
        ].reset_index(drop=True),
    ],
    axis=1,
)

ota_2stage_comparison_df["gain_error"] = 100*np.abs(ota_2stage_comparison_df["gain"] - ota_2stage_comparison_df["sim_gain"])/ota_2stage_comparison_df["gain"]

ota_2stage_comparison_df.to_csv("ota_2stage_comparison.csv", index=False)
print(ota_2stage_comparison_df)