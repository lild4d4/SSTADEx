import numpy as np
from sstadex import simplediffpair

lookup_table = "../../LUTs/ihp-sg13g2/lv_10w.npz"

lengths_nmos = [0.4e-06]

vs = np.linspace(0, 0.8, 100)
vds_diffpar = 1.1 - vs
vgs_diffpar = 0.9 - vs
I_amp1 = 5e-6

diffpair = simplediffpair(
    lut_file=lookup_table,
    netlist='diffpair.spice',
    type='sg13_lv_nmos',
    inputs = { 
        'vds_lut': (0.1, 1.1, 0.01),
        'vgs_lut': (0.1, 1.1, 0.01), 
        'vds': (0.1, 1, 0.1), 
        'vgs': (0.1, 1, 0.1), 
        'il': I_amp1/2,
        'length': lengths_nmos, 
        '2d_var': ['vds', 'vgs']}
)

diffpair=diffpair.build()
print(diffpair)

