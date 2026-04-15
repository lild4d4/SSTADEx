from sympy import Symbol
from sstadex import Macromodel, MacroLibrary, Library, Testbench, VoltageSource, CurrentSource, Resistor
import numpy as np

def compose(
    N_points,
    lengths,
    electrical_parameters,
    ROOT,
    ports,
    gain_condition,
    model=None,
    **kwargs,
):  
    
    primitive_lib = Library(
    name="ihp_sg13g2",
    lut_files={
        "nmos": str(ROOT / "LUTs/ihp-sg13g2/lv_5w_nmos.npz"),
        "pmos": str(ROOT / "LUTs/ihp-sg13g2/lv_5w_pmos.npz"),
    },
)
    primitive_lib.register_all(ROOT / "analoglib/primitives")

    macro_lib = MacroLibrary(name="ihp_sg13g2_macros", primitive_library=primitive_lib)
    macro_lib.register_all(ROOT / "analoglib/macros")

    Vin = electrical_parameters["Vdd"]
    Vref = electrical_parameters["Vref"]
    Vout = electrical_parameters["Vout"]
    vs = electrical_parameters["vs"]
    I_amp_1stage = electrical_parameters["I_amp_1stage"]
    I_amp_2stage = electrical_parameters["I_amp_2stage"]
    vout_1stage = electrical_parameters["vout_1stage"]

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
        model="""Ra {VOUT} {VDD} 1
Ga {VOUT} {VDD} {VINP} {VINN} 1""",
        ports=["VINP", "VINN", "VOUT", "VBIAS", "VDD", "VSS"]
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

    OTA_2stage_macro = Macromodel(
        name = 'OTA_2stage_macro',
        ports=ports,
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
        },
        model=model
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
        'xota_1stage',
        ota_1stage,
        {
            "VINP": "VINP",
            "VINN": "VINN",
            "VOUT": "VOUT_1STAGE",
            "VBIAS": "VBIAS_1STAGE",
            "VDD": "VDD",
            "VSS": "VSS",
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

    ota_1stage.num_level_exp = -1
    ota_1stage.run_pareto = False

    OTA_2stage_macro.specifications = [gain_2stage]
    OTA_2stage_macro.opt_specifications = [gain_2stage]
    OTA_2stage_macro.primitives = [commonsource]
    OTA_2stage_macro.submacromodels = [ota_1stage]


    return OTA_2stage_macro
