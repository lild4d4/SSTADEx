import numpy as np
import pandas as pd
from sstadex.models import Transistor


def build(primitive) -> pd.DataFrame:

    cfg = primitive.lut_config
    length_arr = cfg.lengths

    ax0 = tuple(cfg.axes[0].values)
    ax1 = tuple(cfg.axes[1].values)
    id_ = primitive.il
    rows = []

    ports = primitive.ports

    voutp = ports["VOUT"].dc_voltage 
    vdd = ports["VDD"].dc_voltage 
    vinp = ports["VIN"].dc_voltage 

    vout = np.atleast_1d(voutp)
    vdd = np.atleast_1d(vdd)
    vin = np.atleast_1d(vinp)

    # matrices 2D alineadas
    vds = vout[:, None] - vdd[None, :]
    vgs = vin[:, None]  - vdd[None, :]

    vds_sweep = vds.ravel()
    vgs_sweep = vgs.ravel()
    
    pd_aux = {
        "length": [],
        "width": [],
        "gm": [],
        "gds": [],
        "gdsid": [],
        "Ro": [],
        "cgg": [],
        "cgs": [],
        "cgd": [],
        "vdsat": [],
    }

    for vds in vds_sweep:

        tr = Transistor(
        lookup_table_file=primitive.lut_file,
        mos_type=primitive.transistor_type,
        vsb=0,
        vds=vds,
        vgs=ax1,
        lengths=length_arr,
        dof=["length", "vgs"],
        dof_values=[length_arr, vgs_sweep],
        )

        l_col = np.tile(cfg.lengths, np.size(vgs_sweep))
        il = id_
        id = il
        w = id / tr.jd
        gdsid = tr.gds / tr.id
        gds = gdsid * id
        ro = 1.0 / gds
        gm = tr.gmid * id
        cgg = (w * tr.cgg) / cfg.lut_w
        cgs = (w * tr.cgs) / cfg.lut_w
        cgd = (w * tr.cgd) / cfg.lut_w
        vdsat = tr.vdsat

        pd_aux["length"].append(np.asarray(l_col).flatten())
        pd_aux["width"].append(np.asarray(w).flatten())
        pd_aux["gm"].append(np.asarray(gm).flatten())
        pd_aux["gds"].append(np.asarray(gds).flatten())
        pd_aux["gdsid"].append(np.asarray(gdsid).flatten())
        pd_aux["Ro"].append(np.asarray(ro).flatten())
        pd_aux["cgg"].append(np.asarray(cgg).flatten())
        pd_aux["cgs"].append(np.asarray(cgs).flatten())
        pd_aux["cgd"].append(np.asarray(cgd).flatten())
        pd_aux["vdsat"].append(np.asarray(vdsat).flatten())

    return pd.DataFrame(
        {
            "length": np.asarray(pd_aux["length"]).flatten(),
            "width": np.asarray(pd_aux["width"]).flatten(),
            "gm": np.asarray(pd_aux["gm"]).flatten(),
            "gds": np.asarray(pd_aux["gds"]).flatten(),
            "gdsid": np.asarray(pd_aux["gdsid"]).flatten(),
            "Ro": np.asarray(pd_aux["Ro"]).flatten(),
            "cgg": np.asarray(pd_aux["cgg"]).flatten(),
            "cgs": np.asarray(pd_aux["cgs"]).flatten(),
            "cgd": np.asarray(pd_aux["cgd"]).flatten(),
            "vdsat": np.asarray(pd_aux["vdsat"]).flatten(),
        }
    )
