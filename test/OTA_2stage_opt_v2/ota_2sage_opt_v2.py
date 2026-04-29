from pathlib import Path
import sys
import pandas as pd
import numpy as np
from sympy import Symbol
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from sstadex import Library, MacroLibrary, Macromodel, dfs, Testbench, VoltageSource, CurrentSource, Resistor


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

N_points=10
vs = np.linspace(0.2, Vout-0.1, N_points)
vout_1stage = np.linspace(Vref - 0.1, Vout+0.1, N_points)

ota_1stage = macro_lib.get(
    "ota_1stage_macro",
    N_points=10, 
    ROOT=ROOT,
    lengths=lengths,    
    electrical_parameters={
        "Vdd": Vin,
        "Vref": Vref,
        "Vout": vout_1stage,
        "I_amp": I_amp_1stage,
        "vs": vs
    },
    ports=["VINP", "VINN", "VOUT", "VBIAS", "VDD", "VSS"],
    model="""Ra {VOUT} {VDD} 1
Ga {VOUT} {VDD} {VINP} {VINN} 1"""
)



commonsource = primitive_lib.get("simplecommonsource", il=I_amp_2stage)
commonsource.set_port_voltages({"VIN": vout_1stage,"VOUT": Vout,"VDD": Vin})
commonsource_df = commonsource.build()
commonsource.parameters = {
    Symbol('g_gm_xcos_m1'): commonsource_df['gm'].values,
    Symbol('R_gds_xcos_m1'): commonsource_df['Ro'].values,
}
commonsource.outputs = {
    Symbol("W_cos"): commonsource_df["width"].values,
    Symbol("L_cos"): commonsource_df["length"].values,
}

currentsource = primitive_lib.get('simplecurrentsource', il=I_amp_2stage)
currentsource.set_port_voltages({"VOUTP": Vout, "VSS": 0, "VINP": Vout})
currentsource_df = currentsource.build(ref_current=I_bias_ref)
currentsource.parameters = {
    Symbol('g_gm_xcs_m1'): currentsource_df['gm'].values,
    Symbol('R_gds_xcs_m1'): currentsource_df['Ro'].values
}
currentsource.outputs = {
    Symbol("W_cs_2stage_m1"): currentsource_df["width_m1"].values,
    Symbol("W_cs_2stage_m2"): currentsource_df["width_m2"].values,
    Symbol("L_cs_2stage"): currentsource_df["length"].values,
}

currentsource_df.to_csv("currentsource.csv")

OTA_2stage_macro = Macromodel(
    name = 'OTA_2stage_macro',
    ports=["VINP", "VINN", "VOUT", "VBIAS_1STAGE", "VBIAS_2STAGE", "VDD", "VSS"],
    outputs = [
        Symbol("W_cos"), Symbol("L_cos"),
        Symbol("W_cs_2stage_m1"), Symbol("W_cs_2stage_m2"),
        Symbol("L_cs_2stage"),
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

OTA_2stage_macro.add_instance(
    "xcs",
    currentsource,
    {
        "VOUTP": "VOUT",
        "VSS": "VSS", 
        "VOUTN": "VBIAS_2STAGE",
        "VINP": "VBIAS_2STAGE",
        "VINN": "VBIAS_2STAGE"
    },
    index=0,
    netlist_params={
        "W_cs_m1": Symbol("W_cs_2stage_m1"),
        "W_cs_m2": Symbol("W_cs_2stage_m2"),
        "L_cs": Symbol("L_cs_2stage"),
    }
)

OTA_2stage_macro.add_instance(
    'xota_1stage',
    ota_1stage,
    {
        "VINP": "VINP",
        "VINN": "VINN",
        "VOUT": "VOUT_1STAGE",
        "VBIAS": "VBIAS_1STAGE",
        "VDD": "VDD",
        "VSS": "VSS"
    },
    index=0,
)

commonsource.interface_variables={
    'vout_1stage_commonsource': np.repeat(vout_1stage, len(lengths))
}
OTA_2stage_macro.interface_variables=[
    "vout_1stage_commonsource"
]
OTA_2stage_macro.shared_nodes = {
"VOUT_node": ["vout_1stage_commonsource", "vout_1stage_diffpair"],
}


########################## CONDITIONS #####################################

ota_1stage.derived_metrics = {
    "gain_1stage_proxy": lambda df: df[Symbol("Ra")] * df[Symbol("ga")],
}

OTA_2stage_macro.submacro_condition_rules = {
    ota_1stage: [
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

ota_1stage.propagated_conditions = {
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

tb_gain_2stage = Testbench(
    name="ota_2stage_gain",
    dut=OTA_2stage_macro,
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
        Symbol("s"): 0,
    },
)

gain_2stage = tb_gain_2stage.make_test(
    name="gain_2stage",
    opt_goal="max",
    conditions={"min": [10 ** (gain_condition / 20)]},
)



from pathlib import Path

output = Path("output")
output.mkdir(parents=True, exist_ok=True)

ota_1stage.num_level_exp = -1
ota_1stage.run_pareto = False

OTA_2stage_macro.specifications = [gain_2stage]
OTA_2stage_macro.opt_specifications = [gain_2stage]
OTA_2stage_macro.primitives = [commonsource, currentsource]
OTA_2stage_macro.submacromodels = [ota_1stage]
OTA_2stage_macro.num_level_exp = -1
OTA_2stage_macro.run_pareto = True

_, _, _, OTA_2stage_macro_df, mask = dfs(OTA_2stage_macro, debug = False)

OTA_2stage_macro_df["gain"] = 20*np.log10(OTA_2stage_macro_df["gain_2stage"])
#OTA_2stage_macro_df["gbw"] = 1/(2*np.pi*OTA_2stage_macro_df["rout_2stage"]*CL)

OTA_2stage_macro_df.to_csv('ota_2stage_df.csv')

OTA_2stage_macro_df_filtered = OTA_2stage_macro_df[
    (
        (OTA_2stage_macro_df[Symbol("W_diff")] > 1e-6) & (OTA_2stage_macro_df[Symbol("W_diff")] < 1000e-6)
        & (OTA_2stage_macro_df[Symbol("W_al")] > 1e-6) & (OTA_2stage_macro_df[Symbol("W_al")] < 1000e-6)
        & (OTA_2stage_macro_df[Symbol("W_cs_1stage_m1")] > 1e-6) & (OTA_2stage_macro_df[Symbol("W_cs_1stage_m1")] < 1000e-6)
        & (OTA_2stage_macro_df[Symbol("W_cs_1stage_m2")] > 1e-6) & (OTA_2stage_macro_df[Symbol("W_cs_1stage_m2")] < 1000e-6)
    )
]
OTA_2stage_macro_df_filtered.to_csv('OTA_2stage_macro_df_filtered.csv')

fig, ax = plt.subplots()
ax.scatter(OTA_2stage_macro_df_filtered["area"], OTA_2stage_macro_df_filtered["gain"])
ax.set_xscale('log')
fig.tight_layout()
fig.savefig("gain.png", dpi=300)
plt.close(fig)

simulation_wcos = OTA_2stage_macro_df_filtered[Symbol("W_cos")].to_numpy()
simulation_wdiff = OTA_2stage_macro_df_filtered[Symbol("W_diff")].to_numpy()
simulation_wal = OTA_2stage_macro_df_filtered[Symbol("W_al")].to_numpy()

wcos_max = 5e-6
ng_cos = np.ceil(simulation_wcos / wcos_max).astype(int)
ng_diff = np.ceil(simulation_wdiff / wcos_max).astype(int)
ng_al = np.ceil(simulation_wal / wcos_max).astype(int)

print("ng_cos", ng_cos)
print("ng_cos", ng_diff)
print("ng_cos", ng_al)

point = {
    Symbol("W_diff"): OTA_2stage_macro_df_filtered[Symbol("W_diff")].to_numpy(),
    Symbol("L_diff"): OTA_2stage_macro_df_filtered[Symbol("L_diff")].to_numpy(),
    Symbol("ng_diff"): ng_diff,
    Symbol("W_al"): OTA_2stage_macro_df_filtered[Symbol("W_al")].to_numpy(),
    Symbol("L_al"): OTA_2stage_macro_df_filtered[Symbol("L_al")].to_numpy(),
    Symbol("ng_al"): ng_al,
    Symbol("W_cos"): OTA_2stage_macro_df_filtered[Symbol("W_cos")].to_numpy(),
    Symbol("L_cos"): OTA_2stage_macro_df_filtered[Symbol("L_cos")].to_numpy(),
    Symbol("ng_cos"): ng_cos,
    Symbol("W_cs_1stage_m1"): OTA_2stage_macro_df_filtered[Symbol("W_cs_1stage_m1")].to_numpy(),
    Symbol("W_cs_1stage_m2"): OTA_2stage_macro_df_filtered[Symbol("W_cs_1stage_m2")].to_numpy(),
    Symbol("L_cs_1stage"): OTA_2stage_macro_df_filtered[Symbol("L_cs_1stage")].to_numpy(),
    Symbol("W_cs_2stage_m1"): OTA_2stage_macro_df_filtered[Symbol("W_cs_2stage_m1")].to_numpy(),
    Symbol("W_cs_2stage_m2"): OTA_2stage_macro_df_filtered[Symbol("W_cs_2stage_m2")].to_numpy(),
    Symbol("L_cs_2stage"): OTA_2stage_macro_df_filtered[Symbol("L_cs_2stage")].to_numpy(),
}

simulations = OTA_2stage_macro.ngspice_sim(
    point,
    variables=["gain", "vout"],
    extra_spice = {
        "pre": [
            "**",
            "x1 vref vret vout vbias_1stage vbias_2stage vdd vss OTA_2stage_macro", # VINP VINN VOUTP VDD IBIAS
            "R1 vfb vout 200000000 m=1",
            "C2 vfb vss 10 m=1",
            "R2 vfb vss 900000000 m=1",
            "V1 vret vfb dc 0 ac 1",
            "I0 vdd vbias_1stage 20e-6",
            "I1 vdd vbias_2stage 20e-6",
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
        OTA_2stage_macro_df_filtered.reset_index(drop=True),
        sim_results_df[
            ["run_name", "returncode", "sim_gain", "sim_vout"]
        ].reset_index(drop=True),
    ],
    axis=1,
)

ota_2stage_comparison_df["gain_error"] = 100*np.abs(ota_2stage_comparison_df["gain"] - ota_2stage_comparison_df["sim_gain"])/ota_2stage_comparison_df["gain"]

ota_2stage_comparison_df.to_csv("ota_2stage_comparison.csv", index=False)
print(ota_2stage_comparison_df)