from sstadex import Macromodel, bfs, Primitive, dfs
import numpy as np
from sympy import Symbol

XSCHEM_RCFILE = "/opt/pdks/sky130A/libs.tech/xschem/xschemrc"
SPICE_DIR = "./spice/"
OUTPUT_DIR = "./output/"
XSCHEM_DIR = "./xschem/"

active_load = Macromodel(name = "activeLoad",
                         macromodel_parameters = {Symbol("Raload_1"): np.linspace(100, 100000, 5),
                                                  Symbol("Raload_2"): np.linspace(100, 100000, 5),
                                                  Symbol("gaload"): np.linspace(0.001, 0.1, 5)})

diff_pair = Macromodel(name = "diffpair",
                       macromodel_parameters = {Symbol("gdif_1"): np.linspace(0.001, 0.1, 5),
                                                Symbol("Rdif_1"): np.linspace(100, 100000, 5)})

current_source = Macromodel(name = "currentSource",
                            macromodel_parameters = {Symbol("I2"): np.asarray([0])})

OTA_macro = Macromodel(name = "ota",
                       submacromodels = [active_load, diff_pair, current_source],
                       macromodel_parameters={Symbol('Ra'): np.linspace(100, 100000, 5),
                                              Symbol('gma'): np.linspace(0.001, 0.1, 5)},
                       req_tfs = [["vout", "vpos"]],
                       electrical_parameters={Symbol("gdif_2"): Symbol("gdif_1"),
                                              Symbol("Rdif_2"): Symbol("Rdif_1"),
                                              Symbol("V1"): 0,
                                              Symbol("V_n"): 0,
                                              Symbol("V_p"): 1})

pt = Primitive(name = "passTransistor", 
               parameters = {Symbol('gm_pt'): np.linspace(0.001, 0.1, 5), 
                             Symbol('Ro_pt'): np.linspace(100, 100000, 5)})

LDO_macro = Macromodel(name = "ldo", 
                       submacromodels = [OTA_macro],
                       req_tfs = [["vout", "net1"]], 
                       primitives = [pt], 
                       electrical_parameters={Symbol("V1"): 1, 
                                              Symbol("V2"): 0, 
                                              Symbol("R2"): 100000, 
                                              Symbol("R3"): 300000})

dfs(LDO_macro)