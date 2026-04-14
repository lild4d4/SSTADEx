from sympy import Symbol
from sstadex import Macromodel, MacroLibrary, Library, Testbench, VoltageSource, CurrentSource, Resistor
import numpy as np

def compose(
    N_points,
    lengths,
    electrical_parameters,
    ROOT,
    **kwargs,
):  
    
    primitive_lib = Library(
    name      = "ihp_sg13g2",
    lut_files = {
        "nmos": str(ROOT / "LUTs/ihp-sg13g2/lv_5w_nmos.npz"),
        "pmos": str(ROOT / "LUTs/ihp-sg13g2/lv_5w_pmos.npz"),
    },
    )

    primitive_lib.register_all(ROOT / "analoglib/primitives/")

    macro_lib = MacroLibrary(name="ihp_sg13g2_macros", primitive_library=primitive_lib)
    macro_lib.register_all(ROOT / "analoglib/macros")

    Vdd = electrical_parameters["Vdd"]
    Vref = electrical_parameters["Vref"]
    Vout = electrical_parameters["Vout"]
    vs = electrical_parameters["vs"]
    I_amp = electrical_parameters["I_amp"]

    diffpair = primitive_lib.get("simplediffpair", il=I_amp)
    currentmirror = primitive_lib.get("simplecurrentmirror", il=I_amp)

    current_source_macro_eparameters = {"Vout": vs, "I_amp": I_amp, "Vdd": Vdd}

    current_source_macro = macro_lib.get(
        "current_source_macro",
        lengths=lengths,
        electrical_parameters=current_source_macro_eparameters,
        ROOT=ROOT,
    )

    diffpair.set_port_voltages({"VINP":  Vref, "VINN":  Vref, "VOUTP": Vout, "VOUTN": Vout, "VTAIL": vs})
    currentmirror.set_port_voltages({"VINP":  Vout, "VINN":  Vout, "VOUTP": Vout, "VOUTN": Vout, "VDD": Vdd})

    diffpair_df = diffpair.build()
    currentmirror_df = currentmirror.build()

    diffpair.parameters = {
        Symbol('g_gm_xdp_m1'): diffpair_df['gm'].values,
        Symbol('R_gds_xdp_m1'): diffpair_df['Ro'].values,
    }
    currentmirror.parameters = {
        Symbol('g_gm_xcm_m1'): currentmirror_df['gm'].values,
        Symbol('R_gds_xcm_m1'): currentmirror_df['Ro'].values,
    }

    diffpair.outputs = {
        Symbol("W_diff"): diffpair_df["width"].values,
        Symbol("L_diff"): diffpair_df["length"].values,
    }
    currentmirror.outputs = {
        Symbol("W_al"): currentmirror_df["width"].values,
        Symbol("L_al"): currentmirror_df["length"].values,
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

    diffpair.interface_variables={
    'vs_diff': np.tile(vs, len(lengths))
    }

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

    from pathlib import Path

    
    current_source_macro.num_level_exp = 1
    current_source_macro.run_pareto = False

    OTA_1stage_macro.specifications = [gain_1stage, rout_1stage, gm_1stage]
    OTA_1stage_macro.opt_specifications = [gain_1stage]
    OTA_1stage_macro.primitives = [diffpair, currentmirror]
    OTA_1stage_macro.submacromodels = [current_source_macro]

    return OTA_1stage_macro
