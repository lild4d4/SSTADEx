import pandas as pd

lookup_table_nmos = "../../LUTs/ihp-sg13g2/lv_10w_nmos.npz"
lookup_table_pmos = "../../LUTs/ihp-sg13g2/lv_10w_pmos.npz"
lut_w = 10e-6

N_points = 10
lengths_nmos = [0.4e-06, 0.8e-06, 1.6e-06, 3.2e-06, 6.4e-6]
lengths_pmos = [0.4e-06, 0.8e-06, 1.6e-06, 3.2e-06, 6.4e-6]

## Electrical parameters

Vout = 0.9                                    # LDO output voltage
Vin = 1.5                                      # LDO supply voltage
Vref = 0.9                                      # LDO voltage reference
IL = 1e-3                                      # Load current
CL = 1e-12                                    # Load capacitance
RL = Vout/IL

## LDO specifications

efficiency = 0.99
gain_condition = 80
estability_condition = 60
size_condition = 1e-3

I_bias = 5e-6
I_amp = 10e-6 

Iq_max = IL*(1-efficiency)
Ib_out = Iq_max-I_amp

#R1 = (Vout-Vref)/Ib_out
#R2 = Vref*R1/(Vout-Vref)

#LDO = pd.DataFrame.from_dict({'Vout': [Vout], 'Vin': [Vin], 'Vref': [Vref], 'IL': [IL], 'CL': [CL], 'RL': [RL], 'Iq_max': [Iq_max], 'Ib_out': [Ib_out], 'R1': [R1], 'R2': [R2]}, orient='index', columns=['Value'])

########################## MACROMODELS ############################################
from sstadex import Macromodel, simplediffpair, cm_pmos
import numpy as np
from sympy import Symbol

Vota_1stage = np.linspace(Vout-0.3, Vout+0.5, 5)

OTA_1stage_macro = Macromodel(
    name = 'OTA_1stage_macro',
    outputs = [
        Symbol("W_2stage"), Symbol("L_2stage"),
        Symbol("vin_2stage"),
        Symbol('W_cs_2stage'), Symbol('L_cs_2stage'),
        Symbol('vgs_cs_2stage'),
        Symbol("W_diff"), Symbol("L_diff"), 
        Symbol("vout_1stage"),
        Symbol("W_al"), Symbol("L_al"),
        Symbol("W_cs"), Symbol("L_cs"),
        Symbol('vgs_cs'),
        Symbol("W_cc"), Symbol("L_cc"),
        Symbol("W_rc"), Symbol("L_rc")],
    electrical_parameters = {
        "Vdd": Vin,
        "Vneg": Vref,
        "Vout": Vout,
        "Il": I_amp},
    macromodel_parameters={
        Symbol('Ra'): np.logspace(3, 7, N_points),
        Symbol('gma'): np.logspace(-5, -2, N_points)}
    )


####################################################################################
vs = np.linspace(0.2, Vout-0.1, 10)
vds_diffpair = Vout-vs
vgs_diffpair = Vref-vs

diffpair = simplediffpair(
    lut_file=lookup_table_nmos,
    lut_w=lut_w,
    netlist='diffpair.spice',
    type='sg13_lv_nmos',
    inputs = { 
        'vds_lut': (0.1, 1.2, 0.01),
        'vgs_lut': (0.1, 1.2, 0.01), 
        'vds': vds_diffpair, 
        'vgs': vgs_diffpair, 
        'il': I_amp/2,
        'length': lengths_nmos, 
        '2d_var': ['vds', 'vgs']}
)

diffpair_df = diffpair.build()

diffpair_mask = (diffpair_df["width"]>3e-6) & (diffpair_df["width"]<3e-4)
diffpair_df = diffpair_df[diffpair_mask]

diffpair_df.to_csv('diffpair.csv')

####################################################################################

activeload = cm_pmos(
    lut_file=lookup_table_pmos,
    lut_w=lut_w,
    netlist='pmos_cs.spice',
    type='sg13_lv_pmos',
    inputs={
        'vds_lut': (-1.2, -0.1, 0.01), 
        'vgs_lut': (-1.2, -0.1, 0.01), 
        'vds': OTA_1stage_macro.electrical_parameters["Vout"]-OTA_1stage_macro.electrical_parameters["Vdd"],
        'vgs': OTA_1stage_macro.electrical_parameters["Vout"]-OTA_1stage_macro.electrical_parameters["Vdd"], 
        'il': I_amp/2,
        'length': lengths_pmos, 
        '2d_var': ['vds', 'vgs']})

activeload_df = activeload.build()
#activeload_mask = (activeload_df["width"]>3e-6) & (activeload_df["width"]<3e-4)
#activeload_df = activeload_df

activeload_df.to_csv('activeload.csv')