from mosplot.lookup_table_generator.simulators import NgspiceSimulator
from mosplot.lookup_table_generator import TransistorSweep, LookupTableGenerator
import os


PDK_ROOT=os.environ['PDK_ROOT']
PDK=os.environ['PDK']

# One of `include_paths` or `lib_mappings` must be specified.
# The rest are optional.

ngspice = NgspiceSimulator(
    # Provide path to simulator if simulator is not in system path.
    simulator_path="ngspice",

    # Default simulation temperature. Override if needed.
    temperature=27,

    # All parameters are saved by default. Override if needed.
    #parameters_to_save=["id", "vth", "vdsat", "gm"],

    # Files to include with `.INCLUDE`.
    #include_paths=[
    #    "./NMOS_VTH.inc",
    #    "./PMOS_VTH.inc"
    #],

    # Files to include with `.LIB`.
    lib_mappings = [
        (PDK_ROOT+'/'+PDK+"/libs.tech/ngspice/models/cornerMOSlv.lib", "mos_tt")
    ],

    # If the transistor is defined inside a subcircuit in
    # the library files, you must specify the symbol used (first entry)
    # and the hierarchical name (second entry). Override if needed.
    mos_spice_symbols = ("XM1", "n.xm1.nsg13_lv_pmos"),

    # Additional spice code that may be needed for the simulation goes here:
    #raw_spice = [
    #    "line 1",
    #    "line 2",
    #]

    # Specify the width. For devices that do not take a width,
    # you can specify other parameters such as the number of fingers.
    # The keys are exactly those recognized by the model.
    device_parameters = {
        "w": 10e-6,
    }
)

# Define a sweep object for PMOS transistors.
pmos_sweep = TransistorSweep(
    mos_type="sg13_lv_pmos",
    vgs=(-0.1, -1.2, -0.01),
    vds=(-0.1, -1.2, -0.01),
    vbs=(0, 1.0, 0.1),
    length=[0.4e-6, 0.8e-6, 1.6e-6, 3.2e-6, 6.4e-6]
)


obj = LookupTableGenerator(
    description="OpenPDK ihp-sg13g2",

    # Pass the simulator object
    simulator=ngspice,

    # Pass the sweep object, specifying the models they apply to.
    model_sweeps={
        "sg13_lv_pmos": pmos_sweep
    },

    # Specify the number of processes to use to build the table faster.
    # Note: `ngspice` already applies parallel processing for certain
    # models. Setting `n_process` to values other than 1 may result in
    # slow downs.
    n_process=4,
)

# Optionally, run an op simulation to check outputs.
# obj.op_simulation()

# Build and store the table.
obj.build("../../LUTs/ihp-sg13g2/lv_10w_pmos")
