from pathlib import Path
import sys

import numpy as np
from sympy import Symbol

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from sstadex import Library, MacroLibrary, Macromodel, dfs, Testbench, VoltageSource
from sstadex.utils.flowsavings import FlowPaths

flowpaths = FlowPaths("outputs")

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

gain_condition = 60

I_bias_ref = 20e-6
I_amp_1stage = 20e-6 
I_amp_2stage = 20e-6 

#Iq_max = IL*(1-efficiency)
#Ib_pt = Iq_max-I_amp1-I_amp2-I_bias

#R1 = (Vout-Vref)/Ib_pt
#R2 = Vref*R1/(Vout-Vref)

vs = np.linspace(0.2, Vout-0.1, N_points)
vout_1stage = np.linspace(Vref - 0.1, Vout+0.1, 3)
vout_2stage = np.linspace(Vref - 0.1, Vout+0.1, 3)

ota_2stage = macro_lib.get(
    "ota_2stage_macro",
    N_points=10, 
    ROOT=ROOT,
    lengths=lengths,
    electrical_parameters={
        "Vdd": Vin,
        "Vref": Vref,
        "Vout": vout_2stage,
        "I_amp_1stage": I_amp_1stage,
        "I_amp_2stage": I_amp_2stage,
        "vs": vs,
        "vout_1stage": vout_1stage
    },
    ports=["VINP", "VINN", "VOUT", "VDD", "VSS"],
    gain_condition=gain_condition,
    model="""Ra {VOUT} {VDD} 1
Ga {VOUT} {VDD} {VINP} {VINN} 1"""
)

pt = primitive_lib.get("simplecommonsource", il=IL)
pt.set_port_voltages({"VIN": vout_1stage,"VOUT": Vout,"VDD": Vin})
pt_df =pt.build()
pt.parameters = {
    Symbol('g_gm_xcos_m1'): pt_df['gm'].values,
    Symbol('R_gds_xcos_m1'): pt_df['Ro'].values,
    Symbol('vdsat_xcos_m1'): pt_df['vdsat'].values,
}
pt.outputs = {
    Symbol("W_cos_pt"): pt_df["width"].values,
    Symbol("L_cos_pt"): pt_df["length"].values,
}

pt_df.to_csv("outputs/pt_df.csv", index=False)

LDO_macro = Macromodel(
    name = 'LDO_macro', 
    ports = ["VINP", "VINN", "VOUT", "VOUT_2STAGE", "VDD", "VSS"],
    outputs = [
        Symbol("W_cos_pt"), Symbol("L_cos_pt"),
        ],
    electrical_parameters = {
        "Vdd": Vin,
        "Vneg": Vref,
        "Vout": Vout},
)

LDO_macro.add_instance(
    "xcos",
    pt,
    {
        "VIN": "VOUT_2STAGE",
        "VOUT": "VOUT",
        "VDD": "VDD"
    },
    index=0,
    netlist_params={
        "W_cos": Symbol("W_cos_pt"),
        "L_cos": Symbol("L_cos_pt"),
        "ng_cos": Symbol("ng_cos_pt"),
    }
)

LDO_macro.add_instance(
    "xota_2stage",
    ota_2stage,
    {
        "VINP": "VINP",
        "VINN": "VINN",
        "VOUT": "VOUT_2STAGE",
        "VDD": "VDD",
        "VSS": "VSS"
    },
    index=0
)

pt.interface_variables={
    'vin_pt': np.repeat(vout_1stage, len(lengths))
}
LDO_macro.interface_variables={
    'vin_pt',
    'vout_2stage'
}

LDO_macro.shared_nodes = {
    "vout_2stagae_node": ['vin_pt', 'vout_2stage']
}

tb_gain_ldo = Testbench(
    name="LDO_gain",
    dut=LDO_macro,
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

gain_ldo = tb_gain_ldo.make_test(
    name="gain_ldo",
    opt_goal="max",
    conditions={"min": [10 ** (70 / 20)]},
)

ota_2stage.derived_metrics = {
        "gain_2stage_proxy": lambda df: df[Symbol("Ra")] * df[Symbol("ga")],
    }
LDO_macro.submacro_condition_rules = {
        ota_2stage: [
            {
                "kind": "range_from_submacro_metric",
                "metric": "gain_2stage_proxy",
                "target_column": "gain_2stage",
                "bound": "min",
                "margin_factor": 1.0,
            },
        ]
    }

LDO_macro.propagated_conditions = {
    "direct": [
        {
            "kind": "range",
            "column": Symbol("W_cos_pt"),
            "condition": {"min": 1e-6, "max": 1000e-6},
        },
        {
            "kind": "range",
            "column": Symbol('vdsat_xcos_m1'),
            "condition": {"min": 0, "max": 200e-3},
        },

    ],
    "derived": [],
}
ota_2stage.propagated_conditions = {
    "direct": [
        {
            "kind": "range",
            "column": Symbol("W_cos"),
            "condition": {"min": 1e-6, "max": 1000e-6},
        },
    ],
    "derived": [],
}


ota_2stage.num_level_exp = -1
ota_2stage.is_primitive = 0
ota_2stage.run_pareto = False

LDO_macro.specifications = [gain_ldo]
LDO_macro.opt_specifications = [gain_ldo]
LDO_macro.primitives = [pt]
LDO_macro.submacromodels = [ota_2stage]
LDO_macro.num_level_exp = -1
LDO_macro.run_pareto = False

_, _, _, LDO_macro_df, mask = dfs(LDO_macro, debug = False)

LDO_macro_df.to_csv("LDO_final.csv", index=False)
