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

    # matrices 2D alineadas
    vds = voutp[:, None] - vss[None, :]
    vgs = vinp[:, None]  - vss[None, :]

    vds_sweep = vds.ravel()
    vgs_sweep = vgs.ravel()

    tr = Transistor(
        lookup_table_file = primitive.lut_file,
        mos_type          = primitive.transistor_type,
        vsb               = 0,
        vds               = ax0, # -> (0.1, 1.2, 10) is faster than (0.1, 1.2, 0.01) for some reason. Need to check.
        vgs               = ax1,
        lengths           = length_arr,
        dof               = ["vds", "vgs"],
        dof_values        = [vds_sweep, vgs_sweep],
    )
    print('[DEBUG] jd: ', tr.jd)

    # --- meshgrid for sweep shape ---
    mesh = np.meshgrid(ax0, ax1)
    # length column — repeat to match mesh shape
    L_col = np.repeat(cfg.lengths, np.size(vds_sweep))
    # --- scale small-signal params to operating current ---
    id = id_             # each branch
    W     = id / tr.jd
    gdsid = tr.gds / tr.id
    gds   = gdsid * id
    Ro    = 1.0 / gds
    gm    = tr.gmid * id
    cgg   = (W * tr.cgg) / cfg.lut_w
    cgs   = (W * tr.cgs) / cfg.lut_w
    cgd   = (W * tr.cgd) / cfg.lut_w
    # --- update layout param W (representative / median) ---
    #self._layout_params["W"].default = float(np.nanmedian(W))
    df = pd.DataFrame({
        "length": np.asarray(L_col).flatten(),
        "width_m1":  np.asarray(W).flatten(),
        "width_m2":  np.asarray(W).flatten(),
        "gm":     np.asarray(gm).flatten(),
        "gds":    np.asarray(gds).flatten(),
        "gdsid":  np.asarray(gdsid).flatten(),
        "Ro":     np.asarray(Ro).flatten(),
        "cgg":    np.asarray(cgg).flatten(),
        "cgs":    np.asarray(cgs).flatten(),
        "cgd":    np.asarray(cgd).flatten(),
        "vgs_cs":    np.tile(vgs_sweep, len(length_arr))
    })

    return df
