import numpy as np
import pandas as pd
from sstadex import Macromodel
from sympy import Symbol


def get_block_output_symbols(block):
    if isinstance(block, Macromodel):
        return [
            *list(block.outputs),
            *list(getattr(block, "interface_variables", [])),
        ]

    return [
        *list(getattr(block, "outputs", {}).keys()),
        *list(getattr(block, "interface_variables", {}).keys()),
    ]


def get_sizing_symbol_order(flattened_params):
    primmods_list = []

    for model in flattened_params.keys():
        if type(model) is Macromodel and not model.its_final:
            if model.is_primitive:
                primmods_list.append(model)
        else:
            primmods_list.append(model)

    sizing_symbols = []
    output_symbols = []
    interface_symbols = []

    for block in primmods_list:
        block_output_symbols = get_block_output_symbols(block)
        sizing_symbols.extend(block_output_symbols)

        if isinstance(block, Macromodel):
            output_symbols.extend(list(block.outputs))
            interface_symbols.extend(list(getattr(block, "interface_variables", [])))
        else:
            output_symbols.extend(list(getattr(block, "outputs", {}).keys()))
            interface_symbols.extend(
                list(getattr(block, "interface_variables", {}).keys())
            )

    return sizing_symbols, set(output_symbols), set(interface_symbols)


def filter_conditions(macromodel, macro_results, sizing):
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
        final_dict[macromodel.specifications[idx].name] = result[mask]

    sizing_symbols, output_symbols, _ = get_sizing_symbol_order(flattened_params)

    print(
        f"[FLOW] Building dataframe for {macromodel.name}: "
        f"sizing_shape={getattr(sizing, 'shape', None)}, "
        f"kept_points={int(np.count_nonzero(mask))}/{len(mask)}"
    )

    if len(sizing) != 0:
        area = np.full(sizing[0][mask].shape, 0)
        for idx, output in enumerate(sizing_symbols):
            final_dict[output] = sizing[idx][mask]
            if output in output_symbols:
                area = area + sizing[idx][mask]

        final_dict["area"] = area

    df = pd.DataFrame.from_dict(final_dict)
    df_name = 'conditions_df_'+macromodel.name+'.csv'
    df.to_csv(df_name)
    print(
        f"[FLOW] Dataframe built for {macromodel.name}: "
        f"rows={len(df.index)}, columns={len(df.columns)}"
    )

    for shared_node_variables in getattr(macromodel, "shared_nodes", {}).values():
        if not shared_node_variables:
            continue

        if all(variable in df for variable in shared_node_variables):
            rows_before = len(df.index)
            reference_variable = shared_node_variables[0]
            for variable in shared_node_variables[1:]:
                df = df[df[reference_variable] == df[variable]]
            print(
                f"[FLOW] Shared node filter {macromodel.name} "
                f"{shared_node_variables}: {rows_before} -> {len(df.index)}"
            )

    # df.sort_values(by=flattened_submacro_params)

    return df, 0
