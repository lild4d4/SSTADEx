import numpy as np
from mosplot.plot import load_lookup_table, Mosfet, Expression

# Agrega esto para diagnosticar
lookup_table = load_lookup_table("../../LUTs/ihp-sg13g2/lv_10w.npz")

nmos = Mosfet(lookup_table=lookup_table, mos="sg13_lv_nmos", vbs=0.0, vds=(0,1.2), vgs=(0, 1.2), length=0.4e-6)

x = nmos.lookup_expression_from_table(
    length=4e-7,
    vbs = 0,
    vds = (0.1, 1.2, 0.1),
    vgs = (0.1, 1.2, 0.1),
    primary="vds",
    expression=nmos.gmid_expression,
) 

print(x)

gmid = nmos.interpolate(
        x_expression=nmos.vds_expression,
        x_value=(0.1, 1, 0.01),
        y_expression=nmos.vgs_expression,
        y_value=(0.1, 1, 0.01),
        z_expression=nmos.gmid_expression
)

print(gmid)