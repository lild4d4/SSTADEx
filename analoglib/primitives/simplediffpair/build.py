"""
build.py — simplediffpair
=========================
LUT sweep for the simple differential pair.

Called by Primitive.build(), which passes itself as argument.
Returns a pd.DataFrame with one row per (gmid, length) point.

Available via `primitive`:
    primitive.lut_file       -- path to the .npy LUT for this PDK
    primitive.transistor_type
    primitive.lut_config     -- LUTConfig(lut_w, axes, lengths)
    primitive.il             -- tail current [A]
"""

import numpy as np
import pandas as pd
from sstadex.models import Transistor


def build(primitive) -> pd.DataFrame:
    cfg = primitive.lut_config

    # resolve sweep axes
    gmid_arr   = cfg.axes[0].resolve()   # e.g. arange(5, 25, 1)
    length_arr = cfg.axes[1].resolve()   # e.g. [4e-7, 8e-7, 1.6e-6, 3.2e-6]
    id_ = primitive.il / 2  # each branch carries half the tail current
    rows = []

    for length in length_arr:
        tr = Transistor(
            lookup_table_file = primitive.lut_file,
            mos_type          = primitive.transistor_type,
            vsb               = 0,
            vds               = (0.1, 1.2, 0.01),
            vgs               = (0.1, 1.2, 0.01),
            lengths           = [float(length)],
            dof               = ["vds", "vgs"],
            dof_values        = [(0.1, 1.2, 0.01), (0.1, 1.2, 0.01)],
        )

        gmid_lut = np.asarray(tr.gmid).flatten()
        jd_lut   = np.asarray(tr.jd).flatten()
        gds_lut  = np.asarray(tr.gds).flatten()
        id_lut   = np.asarray(tr.id).flatten()
        cgg_lut  = np.asarray(tr.cgg).flatten()
        cgs_lut  = np.asarray(tr.cgs).flatten()
        cgd_lut  = np.asarray(tr.cgd).flatten()

        valid = np.isfinite(gmid_lut) & np.isfinite(jd_lut) & np.isfinite(id_lut) & (id_lut != 0)
        gmid_v = gmid_lut[valid]
        jd_v   = jd_lut[valid]
        gds_v  = gds_lut[valid]
        id_v   = id_lut[valid]
        cgg_v  = cgg_lut[valid]
        cgs_v  = cgs_lut[valid]
        cgd_v  = cgd_lut[valid]

        for target_gmid in gmid_arr:
            idx = int(np.argmin(np.abs(gmid_v - target_gmid)))
            W = id_ / jd_v[idx]
            gdsid = gds_v[idx] / id_v[idx]
            gds = gdsid * id_
            rows.append(
                {
                    "length": float(length),
                    "width": float(W),
                    "gm": float(gmid_v[idx] * id_),
                    "gds": float(gds),
                    "gdsid": float(gdsid),
                    "Ro": float(1.0 / gds),
                    "cgg": float((W * cgg_v[idx]) / cfg.lut_w),
                    "cgs": float((W * cgs_v[idx]) / cfg.lut_w),
                    "cgd": float((W * cgd_v[idx]) / cfg.lut_w),
                }
            )

    return pd.DataFrame(rows)
