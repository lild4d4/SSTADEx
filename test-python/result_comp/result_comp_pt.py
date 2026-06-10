from sstadex import Macromodel, bfs, Primitive
import numpy as np
from sympy import Symbol

XSCHEM_RCFILE = "/opt/pdks/sky130A/libs.tech/xschem/xschemrc"
SPICE_DIR = "./spice/"
OUTPUT_DIR = "./output/"
XSCHEM_DIR = "./xschem/"

OTA_macro = Macromodel(name = "ota",
                       macromodel_parameters={Symbol('Ra'): [np.asarray([1000])],
                                              Symbol('gma'): [np.asarray([0.001])]},
                       req_tfs = [["vout", "vneg"]])

pt = Primitive(name = "passTransistor", 
               parameters = {Symbol('gm_pt'): np.asarray([2.404e-5]), 
                             Symbol('Ro_pt'): np.asarray([1/4.389e-7])})

LDO_macro = Macromodel(name = "ldo", 
                       submacromodels = [OTA_macro],
                       req_tfs = [["vout", "net1"]], 
                       primitives = [pt], 
                       electrical_parameters={Symbol("V1"): 1, 
                                              Symbol("V2"): 0, 
                                              Symbol("R2"): 100000, 
                                              Symbol("R3"): 300000})

bfs(LDO_macro)