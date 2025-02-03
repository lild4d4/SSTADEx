import numpy as np
import pandas as pd
from sstadex import Macromodel


def filter_conditions(macromodel, macro_results):
    masks = []

    for idx, spec in enumerate(macromodel.specifications):
        mask = np.full(macro_results[idx].shape, True)
        for idx, res in enumerate(macro_results[idx].flatten()):
            if "max" in spec.conditions.keys():
                for cond in spec.conditions["max"]:
                    if res < cond:
                        mask[idx] = True
                    else:
                        mask[idx] = False
            elif "min" in spec.conditions.keys():
                for cond in spec.conditions["min"]:
                    if res > cond:
                        mask[idx] = True
                    else:
                        mask[idx] = False
        masks.append(mask)

    mask = np.full(macro_results[0].shape, True)
    for m in masks:
        mask = mask & m

    return mask


def get_new_conditions(
    macromodel, mask, macro_results, exploration_axes, flattened_params
):
    flattened_submacro_params = []

    for model in flattened_params.keys():
        if type(model) == Macromodel:
            for i in model.macromodel_parameters.keys():
                flattened_submacro_params.append(i)
        else:
            for i in model.parameters.keys():
                flattened_submacro_params.append(i)

    print("flattened submacro params: ", flattened_submacro_params)

    final_dict = {}
    for idx, axe in enumerate(exploration_axes):
        final_dict[flattened_submacro_params[idx]] = axe[mask]

    spec_names = []
    for idx, result in enumerate(macro_results):
        spec_names.append(macromodel.specifications[idx].name)
        final_dict[macromodel.specifications[idx].name] = result[mask]

    df = pd.DataFrame.from_dict(final_dict)

    df.sort_values(by=flattened_submacro_params)

    return df, df.iloc[0]
