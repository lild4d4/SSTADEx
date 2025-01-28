from collections import deque
from sstadex import mna, mna_solve, mna_tf, Macromodel, Primitive
import sympy as sym
import numpy as np
import matplotlib.pyplot as plt


def bfs(macromodel):
    if macromodel is None:
        return []

    queue = deque([macromodel])
    result = []

    while queue:
        current_macromodel = queue.popleft()
        result.append(current_macromodel)
        build(current_macromodel)

        for submacromodel in current_macromodel.submacromodels:
            queue.append(submacromodel)

    return result


def dfs(macromodel):
    print("Macro results of: ", macromodel.name)
    if macromodel.name == "activeLoad" or macromodel.name == "currentSource":
        return 0

    macro_results, exploration_axes, primmods_output = build(macromodel)
    # print(macro_results)

    for submacromodel in macromodel.submacromodels:
        submacro_results = dfs(submacromodel)
        # submacro_results.update(submacro_results)
        print("Update Macromodel: ", macromodel.name)
        repeat = False
        macro_results, exploration_axes, primmods_output = build(macromodel, repeat)

    # print(macro_results)
    if macromodel.name == "ota":
        macromodel.its_final = True
    return macro_results, exploration_axes, primmods_output


def topdown_prim_lookup(macromodel):
    if macromodel.hasPrimitive():
        return macromodel.primitives
    else:
        return topdown_prim_lookup(macromodel.submacromodels[0])


def params_flatten(macrmomodel):
    global_params = {}

    for submacro in macrmomodel.submacromodels:
        global_params[submacro] = submacro.macromodel_parameters

    for primitive in macrmomodel.primitives:
        global_params[primitive] = primitive.parameters

    print(global_params.keys())

    return global_params


def explore(macromodel, flatten_params, exp, debug=False):
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
    for mod, i in flatten_params.items():
        Y = []
        if type(mod) is Macromodel and not mod.its_final:
            values_list.append(list(i.values()))
        else:
            Y = list(i.values())
            primvalues_list.append(Y)
            primmods_list.append(mod)

    primmods_outputs = []
    # for prim in primmods_list:
    #    primmods_outputs.append([prim.output[out] for out in macromodel.output])

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

        if Y_2.shape[0] > 1:
            Y_2_aux = np.tile(primvalues_list[0], len(primvalues_list[1][0]))
            Y_2_aux_2 = np.repeat(
                primvalues_list[1], len(primvalues_list[0][0])
            ).reshape(len(primvalues_list[1]), -1)
            # Y_2 = Y_2.reshape(len(primvalues_list_aux), -1)
            Y_2 = [*Y_2_aux, *Y_2_aux_2]

        else:
            Y_2 = Y_2.reshape(len(primvalues_list_aux), -1)
        # print(np.asarray(Y_2).shape)
        # if len(np.asarray(Y_2))!=1:
        #    print(np.asarray(Y_2)[1, 5])

    if debug:
        print("Values_list, macro: ", values_list)
        print("Values_list, micro: ", primvalues_list)
        print("value micro Y: ", Y_2)

    values_list = [j for sub in values_list for j in sub]
    X = [[]]
    if len(values_list) != 0:
        X = np.meshgrid(*values_list)
        X = np.reshape(X, (len(values_list), np.prod([len(x) for x in values_list])))

    result = []
    results_axes = []
    results_2 = []
    if len(X[0]) != 0:
        for idx in range(len(X[0])):
            temp = exp(*X[:, idx], *Y_2)
            # db = 20 * np.log10(np.abs(temp))
            result.append(temp)

            results_axes.append([*X[:, idx], *Y_2])
            # if macromodel.name == "ota":
            #    results_2.append(
            #        (X[:, idx][0] * X[:, idx][3]) / (X[:, idx][0] + X[:, idx][3])
            #    )
    else:
        for idx in range(1):
            temp = exp(*Y_2)
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
    return np.asarray(result), results_axes, primmods_outputs


def build(macromodel, repeat=True, debug=False):
    XSCHEM_RCFILE = "/opt/pdks/sky130A/libs.tech/xschem/xschemrc"
    SPICE_DIR = "./spice/"
    OUTPUT_DIR = "./output/"
    XSCHEM_DIR = "./xschem/"

    report, df, df2, A, X, Z, nodes = mna(
        XSCHEM_RCFILE, XSCHEM_DIR, SPICE_DIR, OUTPUT_DIR, macromodel
    )

    sol = mna_solve(macromodel)
    tfs = mna_tf(macromodel)

    print("A: ", A)
    print(X)
    print(sol)
    print(df)

    flattened_params = params_flatten(macromodel)
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

    especificaciones = macromodel.especificaciones

    result = []
    for idx, exp in enumerate(tfs):
        proc = especificaciones[idx].out_def
        exp = exp.subs(especificaciones[idx].parametros)

        if list(proc.keys())[0] == "eval":
            exp = sym.lambdify(
                tuple([j for i in flattened_params.values() for j in i.keys()]), exp
            )
            eval, exploration_axes, primmods_output = explore(
                macromodel, flattened_params, exp, debug
            )
            result.append(eval)
        elif list(proc.keys())[0] == "diff":
            variables = especificaciones[idx].variables
            variable = list(variables.keys())[0]
            print(variables)
            print(variable)

            exps = []
            for idx, eval in enumerate(variables[variable]):
                print({variable: eval})
                exp_new = exp.subs({variable: eval})
                print(exp_new)
                exps.append(exp_new)

            exp = np.lamb(exps[0] - exps[1], especificaciones[idx], macromodel)
            eval = explore(macromodel, flattened_params, exp, debug)
            result.append(eval)

    return result, exploration_axes, primmods_output
