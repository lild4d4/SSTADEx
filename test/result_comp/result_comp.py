from sstadex import Macromodel, bfs, Primitive
import numpy as np
from sympy import Symbol

XSCHEM_RCFILE = "/opt/pdks/sky130A/libs.tech/xschem/xschemrc"
SPICE_DIR = "./spice/"
OUTPUT_DIR = "./output/"
XSCHEM_DIR = "./xschem/"

OTA_macro = Macromodel(name = "ota",
                       macromodel_parameters={Symbol('Ra'): np.linspace(100, 100000, 10),
                                              Symbol('gma'): np.linspace(0.001, 0.1, 10)},
                       req_tfs = [["vout", "vneg"]])

pt = Primitive(name = "passTransistor", 
               parameters = {Symbol('gm_pt'): np.asarray([100]), 
                             Symbol('Ro_pt'): np.asarray([100])})

LDO_macro = Macromodel(name = "ldo", 
                       submacromodels = [OTA_macro],
                       req_tfs = [["vout", "net1"]], 
                       primitives = [pt], 
                       electrical_parameters={Symbol("V1"): 1, 
                                              Symbol("V2"): 0, 
                                              Symbol("R2"): 100000, 
                                              Symbol("R3"): 300000})

bfs(LDO_macro)