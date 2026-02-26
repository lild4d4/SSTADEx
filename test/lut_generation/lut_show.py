import numpy as np
from mosplot.plot import load_lookup_table, Mosfet, Expression

lookup_table = load_lookup_table("../../LUTs/ihp-sg13g2/lv_10w.npz")

nmos = Mosfet(lookup_table=lookup_table, mos="sg13_lv_nmos", vbs=0.0, vds=0.6, vgs=(0.01, 1.10))
#pmos = Mosfet(lookup_table=lookup_table, mos="sg13_lv_pmos", vbs=0.0, vds=-0.6, vgs=(-1.2, -0.01))

print(lookup_table['sg13_lv_nmos']['cgg'])

nmos.plot_by_expression(
    x_expression = nmos.gmid_expression,
    y_expression = nmos.current_density_expression,
    filtered_values = nmos.length,
    y_scale="log",
    save_fig="./figures/nmos_current_density.svg"
)
