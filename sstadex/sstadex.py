from collections import deque
from sstadex import mna, mna_solve, mna_tf, Macromodel, Primitive
from .opt import filter_conditions, get_new_conditions, run_pareto
import sympy as sym
import numpy as np
import matplotlib.pyplot as plt
import time


def bfs():
    pass


def dfs(macromodel, debug=False, going_up=0):
    print("############################################")
    print("Starting the exploration of: ", macromodel.name)

    macro_results, exploration_axes, primmods_output = build(macromodel)
    # print("exploration axes:", exploration_axes)

    if macromodel.ext_mask is None:
        mask = filter_conditions(macromodel, macro_results, primmods_output)
    else:
        mask = macromodel.ext_mask
    # print("Masks: ", mask.shape)

    final_df, new_conditions = get_new_conditions(
        macromodel,
        mask,
        macro_results,
        exploration_axes,
        macromodel.flattened_params,
        primmods_output,
    )

    for idx, mac in enumerate(macro_results):
        macro_results[idx] = macro_results[idx][mask]
        # print(macro_results[idx])

    if debug:
        print("Macro_results: ", macro_results)

    if macromodel.num_level_exp == 1 or going_up == 1:
        print("End of the exploration of: ", macromodel.name)

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

    for submacromodel in macromodel.submacromodels:
        print(macromodel.submacromodels)
        print("Going into the Macromodel: ", submacromodel.name)
        print("Going into the Macromodel with code: ", submacromodel)
        submacro_results = dfs(submacromodel, debug)
        print(macromodel.submacromodels)
        macro_results = submacro_results[0]
        exploration_axes = submacro_results[1]
        primmods_output = submacro_results[2]
        final_df = submacro_results[3]
        final_df.to_csv(submacromodel.name + ".csv")
        submacromodel.update(submacro_results)
        results_2 = dfs(macromodel, going_up=1)
        macro_results = results_2[0]
        exploration_axes = results_2[1]
        primmods_output = results_2[2]
        final_df = results_2[3]
        final_df.to_csv(submacromodel.name + "_2" + ".csv")

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
    print("#### creating the primods_list ####")
    for mod, i in flatten_params.items():
        # print("mod: ", mod)
        Y = []
        if type(mod) is Macromodel and not mod.its_final:
            if mod.is_primitive:
                # print("mod is primitive !!")
                Y = list(i.values())
                primvalues_list.append(Y)
                primmods_list.append(mod)
            else:
                values_list.append(list(i.values()))
        else:
            Y = list(i.values())
            if debug:
                print("Y: ", Y)
            primvalues_list.append(Y)
            primmods_list.append(mod)

    primmods_outputs_aux = {}
    primmods_outputs = []

    macro_prim_size = 0
    prim_size = 0

    new_macromodel_outputs = []
    for prim in primmods_list:
        if type(prim) is Macromodel:
            print("macro as prim")
            primmods_outputs_aux.update(prim.output_results)
            for out in prim.outputs:
                # primmods_outputs_aux.append(prim.output_results[out])
                if out not in new_macromodel_outputs:
                    new_macromodel_outputs.append(out)
        else:
            for out in macromodel.outputs:
                # print("all macromodel outputs: ", out)
                if out in prim.outputs:
                    # print("macromodel output in primitives output: ", out)
                    primmods_outputs_aux[out] = prim.outputs[
                        out
                    ]  ### this could be removed if we elimante the macromodel.outputs completly

    for i in reversed(new_macromodel_outputs):
        if i not in macromodel.outputs:
            macromodel.outputs.insert(0, i)

    # print("macromodel outputs: ", macromodel.outputs)
    print("primmods_outputs_aux: ", primmods_outputs_aux)
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
                    for jdx, output in enumerate(prim.outputs):
                        primmods_outputs.append(
                            np.tile(
                                primmods_outputs_aux[output], len(primvalues_list[1][0])
                            )
                        )
                        pos = pos + 1
                else:
                    for jdx, output in enumerate(prim.outputs):
                        primmods_outputs.append(
                            np.repeat(
                                primmods_outputs_aux[output], len(primvalues_list[0][0])
                            )
                        )
                        pos = pos + 1

        elif Y_2.shape[0] > 2:
            pos_list = []
            meshgrid = []
            for idx, prim in enumerate(primvalues_list):
                pos_list.append(list(range(len(prim[0]))))
            print(pos_list)
            meshgrid = np.meshgrid(*pos_list)
            print(meshgrid)

            Y_aux = []
            for idx, prim in enumerate(primvalues_list):
                for jdx, prim_in in enumerate(prim):
                    print("prim_in: ", prim_in)
                    Y_aux.append(np.asarray(prim_in)[tuple(meshgrid[idx].flatten()),])

            Y_2 = [*Y_aux]

            # print("Y_2: ", Y_2)

            for idx, prim in enumerate(primmods_list):
                for jdx, output in enumerate(prim.outputs):
                    print("primods outputs: ", output)
                    primmods_outputs.append(
                        primmods_outputs_aux[output][tuple(meshgrid[idx].flatten()),]
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
                for jdx, output in enumerate(prim.outputs):
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
    print(len(X[0]))
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
        print("finished for operation")
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
        print("finished for operation")
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
    print("starting the return")
    return (
        np.asarray(result).flatten(),
        np.hstack(results_axes),
        np.asarray(primmods_outputs),
    )


def build(macromodel, repeat=True, debug=False):
    XSCHEM_RCFILE = "/opt/pdks/sky130A/libs.tech/xschem/xschemrc"
    SPICE_DIR = "./spice/"
    OUTPUT_DIR = "./output/"
    XSCHEM_DIR = "./xschem/"

    MNA_times = {}
    explore_times = {}

    tfs = []
    for spec in macromodel.specifications:
        macromodel.name = spec.netlist
        print("Netlist: ", macromodel.name)

        start_time = time.time()
        report, df, df2, A, X, Z, nodes = mna(
            XSCHEM_RCFILE, XSCHEM_DIR, SPICE_DIR, OUTPUT_DIR, macromodel
        )
        mna_time = time.time() - start_time
        print(f"MNA of {spec.name} took: {mna_time}")

        MNA_times[spec.name] = mna_time

        print(nodes)

        sol = mna_solve(macromodel)
        tfs.append(*mna_tf(macromodel, spec))

        # print("A: ", A)
        # print(X)
        # print(sol)
        # print(df)

    print("older order of submacros: ", macromodel.submacromodels)

    updated_submacrolist = list(macromodel.submacromodels)
    for submacro in updated_submacrolist:
        if submacro.is_primitive:
            updated_submacrolist.append(
                updated_submacrolist.pop(updated_submacrolist.index(submacro))
            )

    print("new order of submacros: ", updated_submacrolist)
    print(macromodel.submacromodels)

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
        proc = specifications[idx].out_def
        if len(specifications[idx].parametros) != 0:
            exp = exp.subs(specifications[idx].parametros)

        if specifications[idx].composed == 1:
            if list(proc.keys())[0] == "divide":
                print("in divide")
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
                            print("found numerator with index: ", jdx)
                            num_index = jdx
                    except:
                        print("numerator equal 1")
                    if spec.name == denominator.name:
                        print("found denominator with index: ", jdx)
                        den_index = jdx

                if numerator != 1:
                    numerator = result[num_index]

                result.append(numerator / result[den_index])

        if list(proc.keys())[0] == "eval":
            print("in eval")
            print(exp)
            exp = sym.lambdify(
                tuple([j for i in flattened_params.values() for j in i.keys()]), exp
            )
            print(
                "lambdify variables:",
                tuple([j for i in flattened_params.values() for j in i.keys()]),
                exp,
            )
            start_time = time.time()
            eval, exploration_axes, primmods_output = explore(
                macromodel, flattened_params, exp, debug
            )
            explore_time = time.time() - start_time
            print(f"Explore time: {explore_time}")
            explore_times[specifications[idx].name] = explore_time
            print("finished explore operation")
            eval = np.abs(eval)
            if specifications[idx].lamd != None:
                eval = specifications[idx].lamd(eval)
            print("evaluation shape: ", eval.shape)
            if len(eval) == 1:
                eval = np.repeat(eval[0], exploration_axes.shape[1])
            result.append(np.abs(eval))
            print("abs done")
        elif list(proc.keys())[0] == "diff":
            print("in diff")
            variables = specifications[idx].variables
            variable = list(variables.keys())[0]
            print(variables)
            print(variable)

            exps = []
            for idx, eval in enumerate(variables[variable]):
                print({variable: eval})
                exp_new = exp.subs({variable: eval})
                print(exp_new)
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
            print(f"Explore time: {explore_time}")
            explore_times[specifications[idx].name] = explore_time
            print("evaluation shape: ", eval.shape)
            result.append(np.abs(eval))

        elif list(proc.keys())[0] == "frec":
            print("in frec")
            exp = sym.lambdify(
                tuple([j for i in flattened_params.values() for j in i.keys()]), exp
            )
            start_time = time.time()
            eval, exploration_axes, primmods_output = explore(
                macromodel, flattened_params, exp, debug
            )
            explore_time = time.time() - start_time
            print(f"Explore time: {explore_time}")
            explore_times[specifications[idx].name] = explore_time

            print("eval: ", eval)

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
            print(gbw_final)
            result.append(gbw_final)

        elif list(proc.keys())[0] == "pm":
            print("in frec")
            exp = sym.lambdify(
                tuple([j for i in flattened_params.values() for j in i.keys()]), exp
            )
            start_time = time.time()
            eval, exploration_axes, primmods_output = explore(
                macromodel, flattened_params, exp, debug
            )
            explore_time = time.time() - start_time
            print(f"Explore time: {explore_time}")
            explore_times[specifications[idx].name] = explore_time

            print("eval: ", eval)

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
            print(phase_final)
            result.append(phase_final)

    print(MNA_times)
    print(explore_times)
    return result, exploration_axes, primmods_output
