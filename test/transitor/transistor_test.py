from sstadex import Transistor
import numpy as np

lookup_table_nmos = "../../LUTs/ihp-sg13g2/lv_10w_nmos.npz"
lookup_table_pmos = "../../LUTs/ihp-sg13g2/lv_10w_pmos.npz"
lengths = [0.4e-06]
vds = (0.1, 1.2, 0.1)
vgs = (0.1, 1.2, 0.1)

pt_transistor = Transistor(
            lookup_table_file=  lookup_table_nmos,
            mos_type="sg13_lv_nmos",
            vsb=0,
            vds=vds,
            vgs=vgs,
            lengths=lengths,
            dof=['vds', 'vgs'],
            dof_values=[vds,vgs]
)

print(pt_transistor.gmid)

################################################################

vds_pmos = (-1.2, -0.1, 0.1)
vgs_pmos = (-1.2, -0.1, 0.1)

pt_transistor_pmos = Transistor(
            lookup_table_file=lookup_table_pmos,
            mos_type="sg13_lv_pmos",
            vsb=0,
            vds=vds_pmos,
            vgs=vgs_pmos,
            lengths=lengths,
            dof=['vds', 'vgs'],
            dof_values=[vds_pmos,vgs_pmos]
)

print(pt_transistor_pmos.gmid)