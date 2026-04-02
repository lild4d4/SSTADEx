import numpy as np
from mosplot.plot import load_lookup_table, Mosfet, Expression

# Agrega esto para diagnosticar
lookup_table = load_lookup_table("../../LUTs/ihp-sg13g2/lv_10w_nmos.npz")

nmos = Mosfet(lookup_table=lookup_table, mos="sg13_lv_nmos", vbs=0.0, vds=(0,1.2), vgs=(0, 1.2), length=6.4e-6)

gmid = nmos.lookup_expression_from_table(
    length=6.4e-6,
    vbs = 0,
    vds = 0.256,
    vgs = 0.156,
    primary="vds",
    expression=nmos.gmid_expression,
) 

jd = nmos.lookup_expression_from_table(
    length=6.4e-6,
    vbs = 0,
    vds = 0.256,
    vgs = 0.156,
    primary="vds",
    expression=nmos.current_density_expression,
) 

W = 10e-6/jd

print("gmid: ", gmid)
print("jd: ", jd)

print("W: ", W)