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
        "nmos": str(ROOT / "LUTs/ihp-sg13g2/lv_5w_nmos.npz"),
        "pmos": str(ROOT / "LUTs/ihp-sg13g2/lv_5w_pmos.npz"),
    },
)

lib.register_all(ROOT / "analoglib/primitives/")
print(lib)

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

vs = np.linspace(0.2, Vout-0.1, N_points)

diffpair = lib.get("simplediffpair", il=I_amp)
currentmirror = lib.get("simplecurrentmirror", il=I_amp)
currentsource = lib.get('simplecurrentsource', il=I_amp)

diffpair.set_port_voltages({
    "VINP":  Vref,
    "VINN":  Vref,
    "VOUTP": Vout,
    "VOUTN": Vout,
    "VTAIL": vs
})
currentmirror.set_port_voltages({
    "VINP":  Vout,
    "VINN":  Vout,
    "VOUTP": Vout,
    "VOUTN": Vout,
    "VDD": Vdd
})
currentsource.set_port_voltages({
    "VOUTP": vs,
    "VSS": 0,
    "VINP": vs, 
})

print(diffpair.summary())
print(currentmirror.summary())
print(currentsource.summary())

diffpair_df = diffpair.build()
currentmirror_df = currentmirror.build()
currentsource_df = currentsource.build(ref_current=I_amp)

diffpair_df.to_csv('diffpair.csv')
currentmirror_df.to_csv('currentmirror.csv')
currentsource_df.to_csv('currentsource.csv')

diffpair.parameters = {
    Symbol('g_gm_xdp_m1'): diffpair_df['gm'].values,
    Symbol('R_gds_xdp_m1'): diffpair_df['Ro'].values,
}
currentmirror.parameters = {
    Symbol('g_gm_xcm_m1'): currentmirror_df['gm'].values,
    Symbol('R_gds_xcm_m1'): currentmirror_df['Ro'].values,
}
currentsource.parameters = {
    Symbol('g_gm_cs_m1'): currentsource_df['gm'].values,
    Symbol('R_gds_cs_m1'): currentsource_df['Ro'].values
}

diffpair.outputs = {
    Symbol("W_diff"): diffpair_df["width"].values,
    Symbol("L_diff"): diffpair_df["length"].values,
}
currentmirror.outputs = {
    Symbol("W_al"): currentmirror_df["width"].values,
    Symbol("L_al"): currentmirror_df["length"].values,
}
currentsource.outputs = {
    Symbol("W_cs_m1"): currentsource_df["width_m1"].values,
    Symbol("W_cs_m2"): currentsource_df["width_m2"].values,
    Symbol("L_cs"): currentsource_df["length"].values,
}

OTA_1stage_macro = Macromodel(
    name = 'OTA_1stage_macro',
    ports=["VINP", "VINN", "VOUT", "VDD", "IBIAS", "Vbias", "VSS"],
    outputs = [
        Symbol("W_diff"), Symbol("L_diff"), 
        Symbol("W_al"), Symbol("L_al")],
    electrical_parameters = {
        "Vdd": Vdd,
        "Vneg": Vref,
        "Vout": Vout,
        "Il": I_amp},
    macromodel_parameters={
        Symbol('Ra'): np.logspace(3, 7, N_points),
        Symbol('gma'): np.logspace(-5, -2, N_points)}
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
        'Ibias': I_amp,
        'Vdd': Vdd,
    },
    model="""I{INSTANCE} {VOUT} {VSS} 0""",
    macromodel_parameters = {
        Symbol('Ixcs_macro'): [I_amp]
    }
)


diffpair.interface_variables={
    'vs_diff': np.tile(vs, len(lengths))
}
currentsource.interface_variables={
    'vs_cs': np.tile(vs, len(lengths)),
}

OTA_1stage_macro.interface_variables = [
    "vs_diff",
]
current_source_macro.interface_variables=[
    "vs_cs",
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

current_source_macro.add_instance(
    "xcs",
    currentsource,
    {
        "VOUTP": "VOUT",
        "VSS": "VSS", 
        "VOUTN": "Vbias",
        "VINP": "Vbias",
        "VINN": "Vbias"
    },
    index=0,
    netlist_params={
        "W_cs_m1": Symbol("W_cs_m1"),
        "W_cs_m2": Symbol("W_cs_m2"),
        "L_cs": Symbol("L_cs"),
    }
)

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

# tb_phase_margin = Testbench(
#     name="ota_1stage_pm",
#     dut=OTA_1stage_macro,
#     view="small_signal",
#     elements=[
#         VoltageSource("Vdd", "VDD", "VSS", 0),
#         VoltageSource("Vss", "VSS", "VSS", 0),
#         VoltageSource("V_n", "VINN", "VSS", 0),
#         VoltageSource("V_p", "VINP", "VSS", 1),
#         Capacitor("Cout", "VOUT", "VSS", CL),
#     ],
#     tf=["VOUT", "VINP"],
#     parameter_map={
#         Symbol("Vdd"): 0,
#         Symbol("Vss"): 0,
#         Symbol("V_n"): 0,
#         Symbol("V_p"): 1,
#         Symbol("Cout"): CL,
#         Symbol("g_gm_xdp_m2"): Symbol("g_gm_xdp_m1"),
#         Symbol("R_gds_xdp_m2"): Symbol("R_gds_xdp_m1"),
#         Symbol("g_gm_xcm_m2"): Symbol("g_gm_xcm_m1"),
#         Symbol("R_gds_xcm_m2"): Symbol("R_gds_xcm_m1"),
#     },
# )

# phase_margin_1stage = tb_phase_margin.make_test(
#     name="pm_1stage",
#     opt_goal="max",
#     conditions={"min": [0]},
#     out_def = {"pm": tb_phase_margin.tf}
# )

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

from pathlib import Path

osdi_dir = Path("output")
osdi_dir.mkdir(parents=True, exist_ok=True)

current_source_macro.specifications=[ibias_currentsource]
current_source_macro.primitives = [currentsource]
current_source_macro.num_level_exp = 1
current_source_macro.run_pareto = False

OTA_1stage_macro.specifications = [gain_1stage, rout_1stage, gm_1stage]
OTA_1stage_macro.opt_specifications = [gain_1stage]
OTA_1stage_macro.primitives = [diffpair, currentmirror]
OTA_1stage_macro.submacromodels = [current_source_macro]
OTA_1stage_macro.num_level_exp = -1
OTA_1stage_macro.is_primitive = 0
OTA_1stage_macro.run_pareto = False

_, _, _, ota_1stage_df, mask = dfs(OTA_1stage_macro, debug = False)

ota_1stage_df["gain"] = 20*np.log10(ota_1stage_df["gain_1stage"])
ota_1stage_df["gbw"] = 1/(2*np.pi*ota_1stage_df["rout_1stage"]*CL)

ota_1stage_df.to_csv('ota_1stage_df.csv')

ota_1stage_df_filtered = ota_1stage_df[((ota_1stage_df[Symbol("W_diff")]>1e-6) & (ota_1stage_df[Symbol("W_al")]>1e-6) & (ota_1stage_df[Symbol("W_cs_m1")]>1e-6) & (ota_1stage_df[Symbol("W_cs_m2")]>1e-6))]
ota_1stage_df_filtered.to_csv('ota_1stage_df_filtered.csv')

simulation_wdiff = ota_1stage_df_filtered[Symbol("W_diff")].to_numpy()
simulation_wal = ota_1stage_df_filtered[Symbol("W_al")].to_numpy()

wcos_max = 5e-6
ng_diff = np.ceil(simulation_wdiff / wcos_max).astype(int)
ng_al = np.ceil(simulation_wal / wcos_max).astype(int)

point = {
    Symbol("W_diff"): ota_1stage_df_filtered[Symbol("W_diff")].to_numpy(),
    Symbol("L_diff"): ota_1stage_df_filtered[Symbol("L_diff")].to_numpy(),
    Symbol("ng_diff"): ng_diff,
    Symbol("W_al"): ota_1stage_df_filtered[Symbol("W_al")].to_numpy(),
    Symbol("L_al"): ota_1stage_df_filtered[Symbol("L_al")].to_numpy(),
    Symbol("ng_al"): ng_al,
    Symbol("W_cs_m1"): ota_1stage_df_filtered[Symbol("W_cs_m1")].to_numpy(),
    Symbol("W_cs_m2"): ota_1stage_df_filtered[Symbol("W_cs_m2")].to_numpy(),
    Symbol("L_cs"): ota_1stage_df_filtered[Symbol("L_cs")].to_numpy(),
}

simulations = OTA_1stage_macro.ngspice_sim(
    point,
    variables=["gain", "vout", "ibias", "vbias", "gbw"],
    extra_spice = {
        "pre": [
            "**",
            "x1 net1 vn vout vdd ibias vbias vss OTA_1stage_macro", # VINP VINN VOUTP VDD IBIAS
            "R1 vfb vout 100000000 m=1",
            "C2 vfb vss 10 m=1",
            "R2 vfb vss 900000000 m=1",
            "V1 net1 vfb dc 0 ac 1",
            "I0 vdd vbias 20e-6",
            "Cout vout vss 1e-12",
            ".lib /home/daniel/SSTADEX-prev/IHP-Open-PDK/ihp-sg13g2/libs.tech/ngspice/models/cornerMOSlv.lib mos_tt",
            "Vref vn 0 0.9",
            "Vdd vdd 0 1.5",
            "Vss vss 0 0",
            ".control",
            "pre_osdi /home/daniel/SSTADEX-prev/IHP-Open-PDK/ihp-sg13g2/libs.tech/ngspice/osdi/psp103_nqs.osdi",
            "ac dec 10 1 1G",
            "meas ac gain find vdb(vout) at=100",
            "let gain_fc = gain-3",
            "meas ac gbw WHEN vdb(vout)=gain_fc",
            "op",
            "print v(vout)",
            "print v(ibias)",
            "print v(vbias)",
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

ota_1stage_comparison_df = pd.concat(
    [
        ota_1stage_df_filtered.reset_index(drop=True),
        sim_results_df[
            ["run_name", "returncode", "sim_gain", "sim_vout", "sim_ibias", "sim_vbias", "sim_gbw"]
        ].reset_index(drop=True),
    ],
    axis=1,
)

ota_1stage_comparison_df["gain_error"] = 100*np.abs(ota_1stage_comparison_df["gain"] - ota_1stage_comparison_df["sim_gain"])/ota_1stage_comparison_df["gain"]
ota_1stage_comparison_df["gbw_error"] = 100*np.abs(ota_1stage_comparison_df["gbw"] - ota_1stage_comparison_df["sim_gbw"])/ota_1stage_comparison_df["gbw"]

ota_1stage_comparison_df["ibias_error"] = 100*np.abs(ota_1stage_comparison_df["vs_diff"] - ota_1stage_comparison_df["sim_ibias"])/ota_1stage_comparison_df["vs_diff"]

ota_1stage_comparison_df.to_csv("ota_1stage_comparison.csv", index=False)
print(ota_1stage_comparison_df)