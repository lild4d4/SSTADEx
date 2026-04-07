import numpy as np
import pandas as pd
from sstadex.models import Transistor


def build(primitive, ref_current=None) -> pd.DataFrame:
    cfg = primitive.lut_config
    length_arr = cfg.lengths

    ax0 = tuple(cfg.axes[0].values)
    ax1 = tuple(cfg.axes[1].values)
    id_ = primitive.il

    ports = primitive.ports

    voutp = ports["VOUTP"].dc_voltage
    vss = ports["VSS"].dc_voltage
    vinp = ports["VINP"].dc_voltage

    voutp = np.atleast_1d(voutp)
    vss = np.atleast_1d(vss)
    vinp = np.atleast_1d(vinp)

    pd_aux = {
        "length": [],
        "width_m1": [],
        "width_m2": [],
        "gm": [],
        "gds": [],
        "gdsid": [],
        "Ro": [],
        "cgg": [],
        "cgs": [],
        "cgd": [],
    }

    for vds_cs in voutp:
        vgs_cs = np.linspace(vds_cs-0.2, vds_cs+0.6, 5)
        print(f"vds_cs: {vds_cs}")
        print(f"vgs_cs: {vgs_cs}")

        tr = Transistor(
        lookup_table_file=primitive.lut_file,
        mos_type=primitive.transistor_type,
        vsb=0,
        vds=vds_cs,
        vgs=ax1,
        lengths=length_arr,
        dof=["length", "vgs"],
        dof_values=[length_arr, vgs_cs],
        )

        l_col = np.repeat(cfg.lengths, np.size(vgs_cs))
        il = id_
        id = il
        w_m1 = id / tr.jd
        w_m2 = ref_current / tr.jd
        gdsid = tr.gds / tr.id
        gds = gdsid * id
        ro = 1.0 / gds
        gm = tr.gmid * id
        cgg = (w_m1 * tr.cgg) / cfg.lut_w
        cgs = (w_m1 * tr.cgs) / cfg.lut_w
        cgd = (w_m1 * tr.cgd) / cfg.lut_w

        pd_aux["length"].append(np.asarray(l_col).flatten())
        pd_aux["width_m1"].append(np.asarray(w_m1).flatten())
        pd_aux["width_m2"].append(np.asarray(w_m2).flatten())
        pd_aux["gm"].append(np.asarray(gm).flatten())
        pd_aux["gds"].append(np.asarray(gds).flatten())
        pd_aux["gdsid"].append(np.asarray(gdsid).flatten())
        pd_aux["Ro"].append(np.asarray(ro).flatten())
        pd_aux["cgg"].append(np.asarray(cgg).flatten())
        pd_aux["cgs"].append(np.asarray(cgs).flatten())
        pd_aux["cgd"].append(np.asarray(cgd).flatten())

    return pd.DataFrame(
        {
            "length": np.asarray(pd_aux["length"]).flatten(),
            "width_m1": np.asarray(pd_aux["width_m1"]).flatten(),
            "width_m2": np.asarray(pd_aux["width_m2"]).flatten(),
            "gm": np.asarray(pd_aux["gm"]).flatten(),
            "gds": np.asarray(pd_aux["gds"]).flatten(),
            "gdsid": np.asarray(pd_aux["gdsid"]).flatten(),
            "Ro": np.asarray(pd_aux["Ro"]).flatten(),
            "cgg": np.asarray(pd_aux["cgg"]).flatten(),
            "cgs": np.asarray(pd_aux["cgs"]).flatten(),
            "cgd": np.asarray(pd_aux["cgd"]).flatten(),
        }
    )
