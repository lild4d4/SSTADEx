from sstadex import Transistor
import numpy as np

lookup_table = "../../LUTs/ihp-sg13g2/lv_10w.npz"
lengths = [0.4e-06]
vds = (0.1, 1.2, 0.1)
vgs = (0.1, 1.2, 0.1)

pt_transistor = Transistor(
            lookup_table_file=lookup_table,
            mos_type="sg13_lv_nmos",
            vsb=0,
            vds=vds,
            vgs=vgs,
            lengths=lengths,
            dof=['vds', 'vgs'],
            dof_values=[vds,vgs]
)

print(pt_transistor.gmid)