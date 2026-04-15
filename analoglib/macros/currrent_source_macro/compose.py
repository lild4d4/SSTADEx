from sympy import Symbol
from sstadex import Macromodel, MacroLibrary, Library, Testbench, VoltageSource, CurrentSource
import numpy as np

def compose(
    lengths,
    electrical_parameters,
    ROOT,
    **kwargs,
):  
    output_symbols = kwargs.get("output_symbols", {})
    w_cs_m1_symbol = output_symbols.get("W_cs_m1", Symbol("W_cs_m1"))
    w_cs_m2_symbol = output_symbols.get("W_cs_m2", Symbol("W_cs_m2"))
    l_cs_symbol = output_symbols.get("L_cs", Symbol("L_cs"))
    vs_cs_name = output_symbols.get("vs_cs", "vs_cs")
    
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

    vs = electrical_parameters["Vout"]
    I_amp = electrical_parameters["I_amp"]
    Vdd = electrical_parameters["Vdd"]

    currentsource = primitive_lib.get('simplecurrentsource', il=I_amp)

    currentsource.set_port_voltages({"VOUTP": vs, "VSS": 0, "VINP": vs})

    currentsource_df = currentsource.build(ref_current=I_amp)

    currentsource.parameters = {
        Symbol('g_gm_cs_m1'): currentsource_df['gm'].values,
        Symbol('R_gds_cs_m1'): currentsource_df['Ro'].values
    }

    currentsource.outputs = {
        w_cs_m1_symbol: currentsource_df["width_m1"].values,
        w_cs_m2_symbol: currentsource_df["width_m2"].values,
        l_cs_symbol: currentsource_df["length"].values,
    }

    current_source_macro = Macromodel(
        name = 'current_source_macro',
        ports = ['VOUT', 'VSS', 'Vbias'],
        outputs = [
            w_cs_m1_symbol,
            w_cs_m2_symbol,
            l_cs_symbol,
        ],
        electrical_parameters = {
            'Iref': I_amp,
            'Ibias': I_amp,
            'Vdd': Vdd,
        },
        model="""I{INSTANCE} {VOUT} {VSS} 0""",
        macromodel_parameters = {
            Symbol('Ixcs_macro'): [I_amp]
        }
    )

    currentsource.interface_variables={
        vs_cs_name: np.tile(vs, len(lengths)),
    }

    current_source_macro.interface_variables=[
        vs_cs_name,
    ]

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
        "W_cs_m1": w_cs_m1_symbol,
        "W_cs_m2": w_cs_m2_symbol,
        "L_cs": l_cs_symbol,
    }
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

    return current_source_macro
