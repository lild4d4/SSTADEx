from sstadex import spice_sim
import numpy as np

values = {
    "XM1": [
        np.linspace(1, 100, 5),
        np.linspace(1, 6.4, 5),
        np.asarray([1, 1, 1, 1, 1]),
        "nfet",
    ],
    "XM2": [
        np.linspace(1, 100, 5),
        np.linspace(1, 6.4, 5),
        np.asarray([1, 1, 1, 1, 1]),
        "nfet",
    ],
    "XM3": [
        np.linspace(1, 100, 5),
        np.linspace(1, 6.4, 5),
        np.asarray([1, 1, 1, 1, 1]),
        "pfet",
    ],
    "XM4": [
        np.linspace(1, 100, 5),
        np.linspace(1, 6.4, 5),
        np.asarray([1, 1, 1, 1, 1]),
        "pfet",
    ],
}

spice_sim("ota_cap", values)
