from sstadex import Macromodel, bfs, Primitive
import numpy as np
from sympy import Symbol

XSCHEM_RCFILE = "/opt/pdks/sky130A/libs.tech/xschem/xschemrc"
SPICE_DIR = "./spice/"
OUTPUT_DIR = "./output/"
XSCHEM_DIR = "./xschem/"

#### OTA
electrical_parameters = {'Vdd': 1.8, 'Vout': 1.2, 'V_n': 0.9, 'V_p': 0.9, 'ibias': 40e-6,}
netlist = "ota" 
macromodel_parameters = {Symbol('Ra'): np.linspace(100, 100000, 10),
             Symbol('gma'): np.linspace(0.001, 0.1, 10)}

OTA_macro = Macromodel(
    name = "ota",
    electrical_parameters=electrical_parameters,
    netlist=netlist,
    macromodel_parameters=macromodel_parameters,
    req_tfs = [["vout", "vneg"]]
)

# LDO macro
pt = Primitive(name = "passTransistor", parameters = {Symbol('gm_pt'): np.linspace(100, 100000, 10)
                                                      , Symbol('Ro_pt'):np.linspace(100, 100000, 10)})
LDO_macro = Macromodel(name = "ldo", submacromodels = [OTA_macro], req_tfs = [["vout", "net1"]]
                       , primitives = [pt]
                       , electrical_parameters={Symbol("V1"): 1, Symbol("V2"): 0, Symbol("R2"): 100000, Symbol("R3"): 300000})

bfs(LDO_macro)