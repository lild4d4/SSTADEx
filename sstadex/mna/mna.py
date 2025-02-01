import subprocess
import pandas as pd
import SymMNA as symmna
import sympy as sym

from sstadex.utils import spice2ptxt


def mna(XSCHEM_RCFILE, XSCHEM_DIR, SPICE_DIR, OUTPUT_DIR, macromodel):
    print("Running MNA... ")

    design_name = macromodel.name
    # Generation of the .spice from the schematic.
    subprocess.run(
        [
            "xschem",
            "--netlist",
            "--no_x",
            "--quit",
            "--rcfile",
            XSCHEM_RCFILE,
            "-o",
            SPICE_DIR,
            XSCHEM_DIR + design_name + ".sch",
        ]
    )

    # the input netlist of the mna must be plain netlist and not spice, here the convertion is made.
    nodes = spice2ptxt(SPICE_DIR, OUTPUT_DIR, design_name)

    # read the mna input file genereated in the last function
    mna_input_file = open(OUTPUT_DIR + design_name + ".txt", "r")
    lines = mna_input_file.readlines()
    content = lines
    report, df, df2, A, X, Z = symmna.smna(content)

    macromodel.A, macromodel.X, macromodel.Z, macromodel.nodes = A, X, Z, nodes

    return report, df, df2, A, X, Z, nodes


def mna_solve(macromodel):
    eq = sym.Eq(macromodel.A * sym.Matrix(macromodel.X), sym.Matrix(macromodel.Z))
    sol = sym.solve(eq, macromodel.X)

    macromodel.eq_solutions = sol

    return sol


def mna_tf(macromodel):
    sol = macromodel.eq_solutions
    nodes = macromodel.nodes
    # req_tfs = macromodel.req_tfs
    req_tfs = [i.tf for i in macromodel.specifications]
    print(req_tfs)

    tfs = []
    for idx, req_tf in enumerate(req_tfs):
        tfs.append(
            sol[sym.sympify("v" + str(nodes[req_tf[0]].iloc[0]))]
            / sol[sym.sympify("v" + str(nodes[req_tf[1]].iloc[0]))]
        )

    macromodel.tfs_sol = tfs

    return tfs
