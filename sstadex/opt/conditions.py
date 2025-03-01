import numpy as np
import pandas as pd
from sstadex import Macromodel


def filter_conditions(macromodel, macro_results, sizing):
    area_mask = np.full(sizing[0].shape, True)

    size_cond = 1
    for idx, prim_size in enumerate(sizing):
        # print("prim size: ", prim_size)

        if "max" in macromodel.area_conditions:
            size_cond = macromodel.area_conditions["max"][idx]
            # print("size cond max: ", size_cond)
            for jdx, size in enumerate(prim_size):
                if size < size_cond:
                    area_mask[jdx] = area_mask[jdx] & True
                else:
                    area_mask[jdx] = area_mask[jdx] & False
        if "min" in macromodel.area_conditions:
            size_cond = macromodel.area_conditions["min"][idx]
            # print("size cond min: ", size_cond)
            for jdx, size in enumerate(prim_size):
                if size > size_cond:
                    area_mask[jdx] = area_mask[jdx] & True
                else:
                    area_mask[jdx] = area_mask[jdx] & False

        # if idx < 1:
        #    size_cond = macromodel.area_conditions[idx]
        # else:
        #    size_cond = 1
        # print("size cond: ", size_cond)
        # for jdx, size in enumerate(
        #    prim_size
        # ):  ## when the L area is added then should be sizing[idx]
        #    if size < size_cond:
        #        area_mask[jdx] = area_mask[jdx] & True
        #    else:
        #        area_mask[jdx] = area_mask[jdx] & False

    masks = []

    for idx, spec in enumerate(macromodel.specifications):
        mask = np.full(macro_results[idx].shape, True)
        for jdx, res in enumerate(macro_results[idx].flatten()):
            if res == None:
                res = 0
            else:
                res = np.abs(res)
            if "max" in spec.conditions.keys():
                for cond in spec.conditions["max"]:
                    if res < cond:
                        mask[jdx] = True
                    else:
                        mask[jdx] = False
            elif "min" in spec.conditions.keys():
                for cond in spec.conditions["min"]:
                    if res > cond:
                        mask[jdx] = True
                    else:
                        mask[jdx] = False
        masks.append(mask)

    masks.append(area_mask)
    mask = np.full(macro_results[0].shape, True)
    for m in masks:
        # print("partial mask: ", m)
        mask = mask & m

    return mask


def get_new_conditions(
    macromodel, mask, macro_results, exploration_axes, flattened_params, sizing
):
    flattened_submacro_params = []

    for model in flattened_params.keys():
        if type(model) == Macromodel:
            for i in model.macromodel_parameters.keys():
                flattened_submacro_params.append(i)
        else:
            for i in model.parameters.keys():
                flattened_submacro_params.append(i)

    final_dict = {}
    for idx, axe in enumerate(exploration_axes):
        final_dict[flattened_submacro_params[idx]] = axe[mask]

    spec_names = []
    for idx, result in enumerate(macro_results):
        spec_names.append(macromodel.specifications[idx].name)
        final_dict[macromodel.specifications[idx].name] = np.abs(result[mask])

    area = np.full(sizing[0][mask].shape, 0)
    for idx, output in enumerate(macromodel.outputs):
        # print("output name: ", output)
        final_dict[output] = sizing[idx][mask]
        area = area + sizing[idx][mask]

    final_dict["area"] = area

    df = pd.DataFrame.from_dict(final_dict)

    # df.sort_values(by=flattened_submacro_params)

    return df, 0
