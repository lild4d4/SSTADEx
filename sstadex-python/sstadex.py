from collections import deque
from pathlib import Path
from sstadex import mna, mna_solve, mna_tf, Macromodel, Primitive
from .opt import filter_conditions, get_new_conditions, run_pareto
import sympy as sym
import numpy as np
import matplotlib.pyplot as plt
import time
from sstadex.utils.flowsavings import FlowPaths
from sstadex.utils.timing import record_timing

def bfs():
    pass


def _format_condition_entry(condition):
    kind = condition.get("kind", "range")

    if kind == "range":
        column = condition.get("column")
        limits = condition.get("condition", {})
        parts = []
        if "min" in limits:
            parts.append(f"min={limits['min']:.6g}")
        if "max" in limits:
            parts.append(f"max={limits['max']:.6g}")
        return f"{column} ({', '.join(parts)})"

    if kind == "allowed_values":
        column = condition.get("column")
        values = np.asarray(condition.get("values", []))
        return f"{column} (allowed_values={values.size})"

    if kind == "metric":
        return f"metric={condition.get('metric')}"

    if kind == "expression":
        return "expression"

    return str(condition)


def _format_condition_group(conditions):
    formatted = []
    for condition in conditions.get("direct", []):
        formatted.append(_format_condition_entry(condition))
    for condition in conditions.get("derived", []):
        formatted.append(_format_condition_entry(condition))
    return formatted or ["none"]


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


def derive_submacro_conditions(parent_macro, parent_df):
    derived_conditions = {}

    if parent_df is None or len(parent_df.index) == 0:
        return derived_conditions

    submacro_by_name = {
        submacro.name: submacro
        for submacro in getattr(parent_macro, "submacromodels", [])
    }

    for target, rules in getattr(parent_macro, "submacro_condition_rules", {}).items():
        target_macro = (
            target
            if isinstance(target, Macromodel)
            else submacro_by_name.get(str(target))
        )
        target_name = target.name if isinstance(target, Macromodel) else str(target)
        target_conditions = {"direct": [], "derived": []}

        for rule in rules:
            kind = rule.get("kind")

            if kind == "allowed_values_from_parent":
                source_column = rule.get("source_column", rule.get("column"))
                target_column = rule.get("target_column", source_column)

                if source_column in parent_df:
                    target_conditions["direct"].append(
                        {
                            "kind": "allowed_values",
                            "column": target_column,
                            "values": np.unique(parent_df[source_column].to_numpy()),
                        }
                    )

            elif kind == "range_from_parent":
                source_column = rule.get("source_column", rule.get("column"))
                target_column = rule.get("target_column", source_column)

                if source_column in parent_df:
                    target_conditions["direct"].append(
                        {
                            "kind": "range",
                            "column": target_column,
                            "condition": {
                                "min": parent_df[source_column].min(),
                                "max": parent_df[source_column].max(),
                            },
                        }
                    )

            elif kind == "range":
                target_conditions["direct"].append(
                    {
                        "kind": "range",
                        "column": rule["column"],
                        "condition": rule.get("condition", {}),
                    }
                )

            elif kind == "range_from_submacro_metric":
                if target_macro is None:
                    continue

                metric_series = np.asarray(
                    target_macro.evaluate_derived_metric(rule["metric"], parent_df)
                )
                if metric_series.size == 0:
                    continue

                metric_min = float(np.min(metric_series))
                metric_max = float(np.max(metric_series))
                bound = rule.get("bound", "min")
                margin_factor = rule.get("margin_factor", 1.0)
                margin_offset = rule.get("margin_offset", 0.0)
                target_column = rule.get("target_column", rule["metric"])

                condition = {}
                if bound in ("min", "both"):
                    condition["min"] = metric_min * margin_factor + margin_offset
                if bound in ("max", "both"):
                    condition["max"] = metric_max * margin_factor + margin_offset

                target_conditions["direct"].append(
                    {
                        "kind": "range",
                        "column": target_column,
                        "condition": condition,
                    }
                )

            elif kind == "metric":
                target_conditions["derived"].append(
                    {
                        "kind": "metric",
                        "metric": rule["metric"],
                        "condition": rule.get("condition", {}),
                    }
                )

            elif kind == "expression":
                target_conditions["derived"].append(
                    {
                        "kind": "expression",
                        "expr": rule["expr"],
                        "condition": rule.get("condition", {}),
                    }
                )

        derived_conditions[target_name] = target_conditions
        print(
            f"[FLOW] Derived conditions {parent_macro.name} -> {target_name}: "
            + "; ".join(_format_condition_group(target_conditions))
        )

    return derived_conditions

def save_csv(macromodel, flowpaths, df, sufix = None):
    file_path = flowpaths.csv(macromodel.name+"_"+sufix)
    df.to_csv(file_path, index=False)

def dfs(macromodel, debug=False, going_up=0, level=0):

    current_level = level+1

    print(f"[FLOW] Starting exploration: {macromodel.name}")

    XSCHEM_RCFILE = "/opt/pdks/sky130A/libs.tech/xschem/xschemrc"
    SPICE_DIR = "./spice/"
    OUTPUT_DIR = Path("./outputs/")
    XSCHEM_DIR = "./xschem/"
    
    flowpaths = FlowPaths(output_dir = "outputs")


    macro_results, exploration_axes, primmods_output = build(macromodel)
    # print("exploration axes:", exploration_axes)

    start_time = time.time()
    if macromodel.ext_mask is None:
        mask = filter_conditions(macromodel, macro_results, primmods_output)
    else:
        mask = macromodel.ext_mask
    record_timing("spec_filter", macromodel.name, time.time() - start_time)

    print(
        f"[FLOW] Exploration points for {macromodel.name}: "
        f"{len(mask)} -> {int(np.count_nonzero(mask))} after spec conditions"
    )

    start_time = time.time()
    final_df, new_conditions = get_new_conditions(
        macromodel,
        flowpaths,
        mask,
        macro_results,
        exploration_axes,
        macromodel.flattened_params,
        primmods_output,
    )
    record_timing("dataframe_assembly", macromodel.name, time.time() - start_time)

    print(
        f"[FLOW] Dataframe rows for {macromodel.name}: "
        f"{len(final_df.index)} after dataframe assembly"
    )

    if getattr(macromodel, "propagated_conditions", None):
        rows_before = len(final_df.index)
        start_time = time.time()
        filtered_df = macromodel.apply_propagated_conditions(final_df)
        record_timing(
            "propagated_filter", macromodel.name, time.time() - start_time
        )
        final_df = filtered_df
        save_csv(macromodel, flowpaths, final_df, "after_propagated_conditions")
        print(
            f"[FLOW] Dataframe rows for {macromodel.name}: "
            f"{rows_before} -> {len(final_df.index)} after propagated conditions"
        )

    for idx, mac in enumerate(macro_results):
        macro_results[idx] = macro_results[idx][mask]
        # print(macro_results[idx])

    if debug:
        print("Macro_results: ", macro_results)

    if macromodel.num_level_exp == 1 or going_up == 1:
        print(f"[FLOW] Finished exploration: {macromodel.name}")

        if macromodel.run_pareto:
            final_df, final_mask = run_pareto(macromodel, final_df)
            return (
                macro_results,
                exploration_axes,
                primmods_output,
                final_df,
                mask,
                final_mask,
            )
        else:
            return macro_results, exploration_axes, primmods_output, final_df, mask

    # if going_up == 1:
    #    print("End of the exploration of: ", macromodel.name)
    #    return macro_results, exploration_axes, primmods_output, final_df

    start_time = time.time()
    submacro_conditions = derive_submacro_conditions(macromodel, final_df)
    record_timing("condition_derivation", macromodel.name, time.time() - start_time)
    for submacromodel in macromodel.submacromodels:
        inherited_conditions = submacro_conditions.get(
            submacromodel.name,
            {"direct": [], "derived": []},
        )
        local_conditions = getattr(
            submacromodel,
            "propagated_conditions",
            {"direct": [], "derived": []},
        )
        submacromodel.propagated_conditions = {
            "direct": [
                *list(local_conditions.get("direct", [])),
                *list(inherited_conditions.get("direct", [])),
            ],
            "derived": [
                *list(local_conditions.get("derived", [])),
                *list(inherited_conditions.get("derived", [])),
            ],
        }
        print(
            f"[FLOW] Descending into {submacromodel.name} with conditions: "
            + "; ".join(_format_condition_group(submacromodel.propagated_conditions))
        )
        submacro_results = dfs(submacromodel, debug, level=current_level)
        macro_results = submacro_results[0]
        exploration_axes = submacro_results[1]
        primmods_output = submacro_results[2]
        final_df = submacro_results[3]
        save_csv(submacromodel, flowpaths, final_df, "going_down"+str(current_level))
        submacromodel.update(submacro_results)
        results_2 = dfs(macromodel, going_up=1, level=current_level)
        macro_results = results_2[0]
        exploration_axes = results_2[1]
        primmods_output = results_2[2]
        final_df = results_2[3]
        save_csv(submacromodel, flowpaths, final_df, "going_up"+str(current_level))

        if debug:
            print("Macro_results: ", macro_results)

    # print(macro_results)
    if macromodel.name == "ota":
        macromodel.its_final = True
    return macro_results, exploration_axes, primmods_output, final_df, mask


def topdown_prim_lookup(macromodel):
    if macromodel.hasPrimitive():
        return macromodel.primitives
    else:
        return topdown_prim_lookup(macromodel.submacromodels[0])


def params_flatten(macrmomodel, updated_subamcrolist):
    global_params = {}

    for submacro in updated_subamcrolist:
        global_params[submacro] = submacro.macromodel_parameters

    for primitive in macrmomodel.primitives:
        global_params[primitive] = primitive.parameters

    return global_params


def explore(macromodel, flatten_params, expr, debug=False):
    # tf = tf.subs(macromodel.electrical_parameters)

    # symbols = [j for i in flatten_params.values() for j in i.keys()]
    # exp = sym.lambdify(tuple(symbols), tf)

    if debug:
        macros_num = 0
        primitives_num = 0
        for mod, i in flatten_params.items():
            if type(mod) is Macromodel and not mod.its_final:
                macros_num = macros_num + 1
            else:
                primitives_num = primitives_num + 1

        print("macros_num: ", macros_num)
        print("primitives_num: ", primitives_num)

    Xs = []
    Ys = []
    values_list = []
    primvalues_list = []
    primmods_list = []
    primitive_entries = []
    for mod, i in flatten_params.items():
        # print("mod: ", mod)
        Y = []
        if type(mod) is Macromodel and not mod.its_final:
            if mod.is_primitive:
                # print("mod is primitive !!")
                Y = list(i.values())
                primvalues_list.append(Y)
                primmods_list.append(mod)
                primitive_entries.append((mod, Y))
            else:
                values_list.append(list(i.values()))
        else:
            Y = list(i.values())
            if debug:
                print("Y: ", Y)
            primvalues_list.append(Y)
            primmods_list.append(mod)
            primitive_entries.append((mod, Y))

    primmods_outputs_aux = {}
    primmods_outputs = []

    macro_prim_size = 0
    prim_size = 0

    new_macromodel_outputs = []
    new_macromodel_interface_variables = []
    for prim in primmods_list:
        if type(prim) is Macromodel:
            primmods_outputs_aux.update(prim.output_results)
            primmods_outputs_aux.update(prim.interface_results)
            for out in prim.outputs:
                # primmods_outputs_aux.append(prim.output_results[out])
                if out not in new_macromodel_outputs:
                    new_macromodel_outputs.append(out)
            for interface_variable in prim.interface_variables:
                if interface_variable not in new_macromodel_interface_variables:
                    new_macromodel_interface_variables.append(interface_variable)
        else:
            for out in macromodel.outputs:
                # print("all macromodel outputs: ", out)
                if out in prim.outputs:
                    # print("macromodel output in primitives output: ", out)
                    primmods_outputs_aux[out] = prim.outputs[
                        out
                    ]  ### this could be removed if we elimante the macromodel.outputs completly
            for interface_variable in getattr(macromodel, "interface_variables", []):
                if interface_variable in prim.interface_variables:
                    primmods_outputs_aux[interface_variable] = prim.interface_variables[
                        interface_variable
                    ]

    # print("macromodel outputs: ", macromodel.outputs)
    # print("primvalues_list: * ", *primvalues_list)

    Y_2 = [[]]
    if len(primvalues_list) != 0:
        Y_2 = np.meshgrid(*primvalues_list)
        Y_2_aux = np.meshgrid(*primvalues_list, indexing="ij")
        primvalues_list_aux = [j for sub in primvalues_list for j in sub]
        Y_2 = np.asarray(Y_2)

        if debug:
            print("beggining Y_2: ", Y_2)
            print("primvalues_aux: ", primvalues_list_aux)
            print("Y_2.shape: ", Y_2.shape)

        if Y_2.shape[0] > 1 and Y_2.shape[0] <= 2:
            # print("Y_2 shape: ", Y_2.shape)
            Y_2_aux = np.tile(primvalues_list[0], len(primvalues_list[1][0]))
            Y_2_aux_2 = np.repeat(
                primvalues_list[1], len(primvalues_list[0][0])
            ).reshape(len(primvalues_list[1]), -1)
            # Y_2 = Y_2.reshape(len(primvalues_list_aux), -1)
            Y_2 = [*Y_2_aux, *Y_2_aux_2]
            # print("Y_2: ", Y_2)

            pos = 0
            for idx, prim in enumerate(primmods_list):
                if idx == 0:
                    for jdx, output in enumerate(get_block_output_symbols(prim)):
                        primmods_outputs.append(
                            np.tile(
                                primmods_outputs_aux[output], len(primvalues_list[1][0])
                            )
                        )
                        pos = pos + 1
                else:
                    for jdx, output in enumerate(get_block_output_symbols(prim)):
                        primmods_outputs.append(
                            np.repeat(
                                primmods_outputs_aux[output], len(primvalues_list[0][0])
                            )
                        )
                        pos = pos + 1

        elif Y_2.shape[0] > 2:
            pos_list = []
            meshgrid = []
            for _, prim_values in primitive_entries:
                pos_list.append(list(range(len(prim_values[0]))))
            meshgrid = np.meshgrid(*pos_list, indexing="ij")

            Y_aux = []
            for idx, (_, prim_values) in enumerate(primitive_entries):
                for prim_in in prim_values:
                    Y_aux.append(np.asarray(prim_in)[tuple(meshgrid[idx].flatten()),])

            Y_2 = [*Y_aux]

            # print("Y_2: ", Y_2)

            for idx, (prim, _) in enumerate(primitive_entries):
                for output in get_block_output_symbols(prim):
                    primmods_outputs.append(
                        np.asarray(primmods_outputs_aux[output])[
                            tuple(meshgrid[idx].flatten()),
                        ]
                    )

            # primmods_outputs.append(
            #    np.tile(primmods_outputs_aux[0], len(primmods_outputs_aux[2]))
            # )
            # primmods_outputs.append(
            #    np.tile(primmods_outputs_aux[1], len(primmods_outputs_aux[2]))
            # )
            # primmods_outputs.append(
            #    np.repeat(primmods_outputs_aux[2], len(primmods_outputs_aux[0]))
            # )
            # primmods_outputs.append(
            #    np.repeat(primmods_outputs_aux[3], len(primmods_outputs_aux[0]))
            # )

        else:
            Y_2 = Y_2.reshape(len(primvalues_list_aux), -1)
            for idx, prim in enumerate(primmods_list):
                for jdx, output in enumerate(get_block_output_symbols(prim)):
                    primmods_outputs.append(primmods_outputs_aux[output])
        # print(np.asarray(Y_2).shape)
        # if len(np.asarray(Y_2))!=1:
        #    print(np.asarray(Y_2)[1, 5])

    # print("Primods_output_aux 2: ", np.asarray(primmods_outputs).shape)

    if debug:
        print("Values_list, macro: ", values_list)
        print("Values_list, micro: ", primvalues_list)
        print("value micro Y: ", Y_2)

    values_list = [j for sub in values_list for j in sub]
    X = [[]]
    if len(values_list) != 0:
        X = np.meshgrid(*values_list)
        if debug:
            print("lengths of values list: ", [len(x) for x in values_list])
            print("mult: ", np.prod([len(x) for x in values_list]))
        X = np.reshape(X, (len(values_list), np.prod([len(x) for x in values_list])))

    result = []
    results_axes = []
    results_2 = []
    primmods_outputs_aux_2 = [[], []]
    exploration_points = len(X[0]) if len(X) != 0 else 0
    primitive_points = len(Y_2[0]) if len(Y_2) != 0 else 0
    print(
        f"[FLOW] Explore core for {macromodel.name}: "
        f"macro_points={exploration_points}, primitive_points={primitive_points}"
    )
    if len(X[0]) != 0 and len(Y_2[0]) != 0:
        # print("here !")
        for idx in range(len(X[0])):
            # print(idx)
            # print(*X[:, idx])
            # print(*Y_2)
            temp = expr(*X[:, idx], *Y_2)
            # print(type(temp))
            if not isinstance(temp, np.ndarray):
                temp = np.repeat(temp, len(Y_2[0]))
            # if not isinstance(temp, list):
            #
            # db = 20 * np.log10(np.abs(temp))
            result.append(temp)
            # print(temp)
            results_axes.append(
                [*np.repeat(X[:, idx], len(Y_2[0])).reshape(len(X[:, idx]), -1), *Y_2]
            )

            # primmods_outputs_aux_2[0].append(
            #    np.asarray(primmods_outputs_aux[0]).flatten()
            # )
            # primmods_outputs_aux_2[1].append(
            #    np.asarray(primmods_outputs_aux[1]).flatten()
            # )

            # for primmods in primmods_outputs_aux:
            #    primmods_outputs_aux_2.append(np.asarray(primmods).flatten())
            # if macromodel.name == "ota":
            #    results_2.append(
            #        (X[:, idx][0] * X[:, idx][3]) / (X[:, idx][0] + X[:, idx][3])
            #    )
        primmods_outputs = np.tile(np.asarray(primmods_outputs), len(X[0]))

        # primmods_outputs_aux_2[0] = np.asarray(primmods_outputs_aux_2[0]).flatten()
        # primmods_outputs_aux_2[1] = np.asarray(primmods_outputs_aux_2[1]).flatten()
        # for i in primmods_outputs_aux_2:
        #    primmods_outputs.append(i)
    elif len(X[0]) != 0 and Y_2[0] == []:
        for idx in range(len(X[0])):
            # print(idx)
            temp = expr(*X[:, idx])
            # db = 20 * np.log10(np.abs(temp))
            result.append(temp)
            # print(temp)
            results_axes.append([*(X[:, idx].reshape(len(X[:, idx]), -1))])
        primmods_outputs = np.tile(np.asarray(primmods_outputs), len(X[0]))
    else:
        for idx in range(1):
            temp = expr(*Y_2)
            # db = 20 * np.log10(np.abs(temp))
            result.append(temp)

            results_axes.append([*Y_2])
            # if macromodel.name == "ota":
            #    results_2.append(
            #        (X[:, idx][0] * X[:, idx][3]) / (X[:, idx][3] + X[:, idx][3])
            #    )

    # if macromodel.name == "ota":
    #    return (
    #        [np.asarray(result), np.asarray(results_2)],
    #        results_axes,
    #        primmods_outputs,
    #    )
    # else:
    #    return np.asarray(result), results_axes, primmods_outputs
    return (
        np.asarray(result).flatten(),
        np.hstack(results_axes),
        np.asarray(primmods_outputs),
    )


def build(macromodel, repeat=True, debug=False):
    XSCHEM_RCFILE = "/opt/pdks/sky130A/libs.tech/xschem/xschemrc"
    SPICE_DIR = "./spice/"
    OUTPUT_DIR = Path("./outputs/")
    XSCHEM_DIR = "./xschem/"

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    MNA_times = {}
    explore_times = {}

    tfs = []
    for spec in macromodel.specifications:
        if spec.composed == 1:
            tfs.append(None)
            MNA_times[spec.name] = 0
            continue

        if getattr(spec, "testbench", None) is not None:
            testbench = spec.testbench
            macromodel.name = testbench.name
            spice_path = Path(SPICE_DIR) / f"{macromodel.name}.spice"
            spice_path.parent.mkdir(parents=True, exist_ok=True)
            spice_path.write_text(testbench.gen_netlist())
        else:
            macromodel.name = spec.netlist
        print(f"[FLOW] Netlist/source for {spec.name}: {macromodel.name}")

        start_time = time.time()
        report, df, df2, A, X, Z, nodes = mna(
            XSCHEM_RCFILE, XSCHEM_DIR, SPICE_DIR, OUTPUT_DIR, macromodel
        )
        mna_time = time.time() - start_time
        print(f"[FLOW] MNA {spec.name}: {mna_time:.3f}s")

        MNA_times[spec.name] = mna_time
        record_timing("mna_generation", spec.name, mna_time)

        start_time = time.time()
        sol = mna_solve(macromodel)
        tfs.append(*mna_tf(macromodel, spec))
        record_timing("mna_solve_tf", spec.name, time.time() - start_time)

        # print("A: ", A)
        # print(X)
        # print(sol)
        # print(df)

    updated_submacrolist = list(macromodel.submacromodels)
    for submacro in updated_submacrolist:
        if submacro.is_primitive:
            updated_submacrolist.append(
                updated_submacrolist.pop(updated_submacrolist.index(submacro))
            )

    print(
        f"[FLOW] Flattened hierarchy for {macromodel.name}: "
        f"submacros={len(updated_submacrolist)}, primitives={len(macromodel.primitives)}"
    )

    flattened_params = params_flatten(macromodel, updated_submacrolist)
    macromodel.flattened_params = flattened_params

    # print("flattened params: ", flattened_params)

    specifications = macromodel.specifications

    if len(specifications) == 0:
        result, exploration_axes, primmods_output = explore(
            macromodel, flattened_params, tfs[0], debug
        )

    # exploration_results, exploration_axes, primmods_output = explore(
    #    macromodel, flattened_params, tfs[0], debug
    # )

    # if macromodel.name == "ota":
    #    macromodel.macromodel_parameters[sym.Symbol("Ra")] = exploration_results[
    #        1
    #    ].flatten()
    #    macromodel.macromodel_parameters[sym.Symbol("gma")] = (
    #        exploration_results[0].flatten() / exploration_results[1].flatten()
    #    )

    result = []
    for idx, exp in enumerate(tfs):
        spec = specifications[idx]
        proc = spec.out_def

        if spec.composed == 1:
            if list(proc.keys())[0] == "divide":
                numerator = proc["divide"][0]
                denominator = proc["divide"][1]
                if debug:
                    print("numerator name: ", numerator.name)
                    print("denominator name: ", denominator.name)

                num_index = 0
                den_index = 0
                for jdx, spec in enumerate(specifications):
                    if debug:
                        print("Spec name: ", spec.name)
                    try:
                        if spec.name == numerator.name:
                            num_index = jdx
                    except:
                        pass
                    if spec.name == denominator.name:
                        den_index = jdx

                if numerator != 1:
                    numerator = result[num_index]

                result.append(numerator / result[den_index])
                print(
                    f"[FLOW] Composed spec {specifications[idx].name}: "
                    f"{getattr(numerator, 'name', 'value')} / {denominator.name}"
                )
            continue

        if len(spec.parametros) != 0:
            exp = exp.subs(spec.parametros)

        if list(proc.keys())[0] == "eval":
            exp = sym.lambdify(
                tuple([j for i in flattened_params.values() for j in i.keys()]), exp
            )
            start_time = time.time()
            eval, exploration_axes, primmods_output = explore(
                macromodel, flattened_params, exp, debug
            )
            explore_time = time.time() - start_time
            print(f"[FLOW] Explore {specifications[idx].name}: {explore_time:.3f}s")
            explore_times[specifications[idx].name] = explore_time
            record_timing("exploration", specifications[idx].name, explore_time)
            eval = np.abs(eval)
            if specifications[idx].lamd != None:
                eval = specifications[idx].lamd(eval)
            if len(eval) == 1:
                eval = np.repeat(eval[0], exploration_axes.shape[1])
            result.append(np.abs(eval))
        elif list(proc.keys())[0] == "diff":
            variables = specifications[idx].variables
            variable = list(variables.keys())[0]

            exps = []
            for idx, eval in enumerate(variables[variable]):
                exp_new = exp.subs({variable: eval})
                exps.append(exp_new)

            # exp = np.lamb(exps[0] - exps[1], specifications[idx], macromodel)
            exp = sym.lambdify(
                tuple([j for i in flattened_params.values() for j in i.keys()]),
                (exps[0] - exps[1]) / exps[0],
            )
            start_time = time.time()
            eval, exploration_axes, primmods_output = explore(
                macromodel, flattened_params, exp, debug
            )
            explore_time = time.time() - start_time
            print(f"[FLOW] Explore {specifications[idx].name}: {explore_time:.3f}s")
            explore_times[specifications[idx].name] = explore_time
            record_timing("exploration", specifications[idx].name, explore_time)
            result.append(np.abs(eval))

        elif list(proc.keys())[0] == "frec":
            exp = sym.lambdify(
                tuple([j for i in flattened_params.values() for j in i.keys()]), exp
            )
            start_time = time.time()
            eval, exploration_axes, primmods_output = explore(
                macromodel, flattened_params, exp, debug
            )
            explore_time = time.time() - start_time
            print(f"[FLOW] Explore {specifications[idx].name}: {explore_time:.3f}s")
            explore_times[specifications[idx].name] = explore_time
            record_timing("exploration", specifications[idx].name, explore_time)

            frec = sym.Symbol("frec")

            gbw_j = []

            for i in eval.flatten():
                temp = i.subs({sym.Symbol("s"): frec * 2j * np.pi})
                gbw_j.append(temp)

            f = np.logspace(1, 9, 200)
            gbw_final = []
            phase_final = []

            for i in gbw_j:
                gbw_lamb = sym.lambdify(frec, i)
                raw = gbw_lamb(f)
                abs_values = np.abs(20 * np.log10(np.abs(raw)))
                if type(abs_values) is np.ndarray:
                    isnan = np.isnan(abs_values[0])
                else:
                    isnan = np.isnan(abs_values)
                if isnan:
                    gbw_final.append(None)
                    phase_final.append(None)
                else:
                    abs_values = abs_values - (abs_values[0] - 3)
                    cross = np.argmin(np.abs(abs_values))
                    gbw_final.append(f[cross])
                    phase_final.append(np.angle(raw[cross]) * 180 / (np.pi))

            gbw_final = np.asarray(gbw_final)
            result.append(gbw_final)

        elif list(proc.keys())[0] == "pm":
            exp = sym.lambdify(
                tuple([j for i in flattened_params.values() for j in i.keys()]), exp
            )
            start_time = time.time()
            eval, exploration_axes, primmods_output = explore(
                macromodel, flattened_params, exp, debug
            )
            explore_time = time.time() - start_time
            print(f"[FLOW] Explore {specifications[idx].name}: {explore_time:.3f}s")
            explore_times[specifications[idx].name] = explore_time
            record_timing("exploration", specifications[idx].name, explore_time)

            frec = sym.Symbol("frec")

            gbw_j = []

            for i in eval.flatten():
                temp = i.subs({sym.Symbol("s"): frec * 2j * np.pi})
                gbw_j.append(temp)

            f = np.logspace(1, 9, 500)
            gbw_final = []
            phase_final = []

            for i in gbw_j:
                gbw_lamb = sym.lambdify(frec, i)
                raw = gbw_lamb(f)
                abs_values = np.abs(20 * np.log10(np.abs(raw)))
                if type(abs_values) is np.ndarray:
                    isnan = np.isnan(abs_values[0])
                else:
                    isnan = np.isnan(abs_values)
                if isnan:
                    gbw_final.append(None)
                    phase_final.append(None)
                else:
                    cross = np.argmin(np.abs(abs_values))
                    gbw_final.append(f[cross])
                    phase_final.append(np.angle(raw[cross]) * 180 / (np.pi))
                    # print("angle:", np.angle(raw[cross]) * 180 / (np.pi))

            phase_final = np.asarray(phase_final)
            result.append(phase_final)

    print(f"[FLOW] MNA timings for {macromodel.name}: {MNA_times}")
    print(f"[FLOW] Explore timings for {macromodel.name}: {explore_times}")
    return result, exploration_axes, primmods_output
