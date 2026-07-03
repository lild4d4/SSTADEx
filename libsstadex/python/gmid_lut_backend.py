#!/usr/bin/env python3
import json
import math
import sys
from collections import defaultdict

import numpy as np
from mosplot.plot import Mosfet, load_lookup_table


METADATA_KEYS = {
    "description",
    "simulator",
    "parameter_names",
    "device_parameters",
}


def main():
    payload = json.load(sys.stdin)
    lut_files = payload["lut_files"]
    queries = payload["queries"]
    cache = {}
    results = [None] * len(queries)
    groups = defaultdict(list)

    for index, query in enumerate(queries):
        device = query["device"]
        lut_file = lut_files.get(device)
        if lut_file is None:
            raise KeyError(f"missing LUT file for device '{device}'")

        length = float(query["length"])
        vbs = float(query.get("dof", {}).get("vbs", 0.0))
        groups[(lut_file, device, length, vbs)].append((index, query))

    for (lut_file, device, length, vbs), group in groups.items():
        if lut_file not in cache:
            cache[lut_file] = load_lookup_table(lut_file)

        lookup_table = cache[lut_file]
        mos_key = resolve_mos_key(lookup_table, device)
        vds_values = np.asarray(
            [float(required_dof(query, "vds")) for _, query in group], dtype=float
        )
        vgs_values = np.asarray(
            [float(required_dof(query, "vgs")) for _, query in group], dtype=float
        )

        mosfet = Mosfet(
            lookup_table=lookup_table,
            mos=mos_key,
            vbs=vbs,
            vds=(axis_min(vds_values), axis_max(vds_values)),
            vgs=(axis_min(vgs_values), axis_max(vgs_values)),
            length=length,
        )
        expressions = [
            mosfet.current_density_expression,
            mosfet.gmid_expression,
            mosfet.gds_expression,
            mosfet.cgg_expression,
            mosfet.cgs_expression,
            mosfet.cgd_expression,
            mosfet.vdsat_expression,
            mosfet.id_expression,
        ]
        values = mosfet.interpolate(
            x_expression=mosfet.vds_expression,
            x_value=vds_values,
            y_expression=mosfet.vgs_expression,
            y_value=vgs_values,
            z_expression=expressions,
        )

        jd, gmid, gds, cgg, cgs, cgd, vdsat, current = [
            pairwise_values(value, len(group)) for value in values
        ]

        for position, (index, _) in enumerate(group):
            results[index] = {
                "length": length,
                "id": as_float(current[position]),
                "jd": as_float(jd[position]),
                "gmid": as_float(gmid[position]),
                "gds": as_float(gds[position]),
                "cgg": as_float(cgg[position]),
                "cgs": as_float(cgs[position]),
                "cgd": as_float(cgd[position]),
                "vdsat": as_float(vdsat[position]),
            }

    if any(result is None for result in results):
        raise RuntimeError("internal error: missing result rows")

    json.dump({"results": results}, sys.stdout)


def resolve_mos_key(lookup_table, device):
    if device in lookup_table:
        return device

    device_keys = [key for key in lookup_table.keys() if key not in METADATA_KEYS]
    if device in {"nmos", "pmos"}:
        matches = [key for key in device_keys if device in key.lower()]
        if len(matches) == 1:
            return matches[0]

    if len(device_keys) == 1:
        return device_keys[0]

    raise KeyError(f"could not resolve device '{device}' in LUT keys {device_keys}")


def required_dof(query, name):
    dof = query.get("dof", {})
    if name not in dof:
        raise KeyError(f"query '{query.get('lut_name')}' missing dof '{name}'")
    return dof[name]


def axis_min(value):
    return min(0.0, float(np.min(value)))


def axis_max(value):
    return max(0.0, float(np.max(value)))


def pairwise_values(value, count):
    array = np.asarray(value, dtype=float)
    if count == 1:
        return array.ravel()
    return np.diag(array.reshape(count, count))


def as_float(value):
    if hasattr(value, "ravel"):
        value = value.ravel()[0]
    elif isinstance(value, (list, tuple)):
        value = value[0]

    value = float(value)
    if not math.isfinite(value):
        raise ValueError(f"non-finite LUT value {value}")
    return value


if __name__ == "__main__":
    main()
