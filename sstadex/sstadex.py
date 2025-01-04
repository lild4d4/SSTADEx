from collections import deque
from sstadex import mna, mna_solve, mna_tf, Macromodel, Primitive
import sympy as sym
import numpy as np

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

    return global_params

def explore(macromodel, flatten_params, tf):

    tf = tf.subs(macromodel.electrical_parameters)

    symbols = [j for i in flatten_params.values() for j in i.keys()]
    exp = sym.lambdify(tuple(symbols), tf)

    Xs = []
    Ys = []
    values_list = []
    for mod, i in flatten_params.items():
        
        Y = []
        if type(mod) is Macromodel:
            values_list.append(list(i.values()))
        else:
            Y = list(i.values())
            Ys.append(Y)

    values_list = [j for sub in values_list for j in sub]
    X = np.meshgrid(*values_list)
    X = np.reshape(X, (len(values_list), 
                       np.prod([len(x) for x in values_list])))

    result = []
    results_axes = []
    for idx, value in enumerate(X[0]):
        temp = exp(*X[:, idx], *Y)
        result.append(20*np.log10(np.abs(temp)))
        
        results_axes.append([*X[:, idx], *Y])

    return np.asarray(result), results_axes

def build(macromodel):
        
        XSCHEM_RCFILE = "/opt/pdks/sky130A/libs.tech/xschem/xschemrc"
        SPICE_DIR = "./spice/"
        OUTPUT_DIR = "./output/"
        XSCHEM_DIR = "./xschem/"

        report, df, df2, A, X, Z, nodes = mna(XSCHEM_RCFILE, XSCHEM_DIR, SPICE_DIR, OUTPUT_DIR, macromodel) 
        sol = mna_solve(macromodel)
        tfs = mna_tf(macromodel)

        flattened_params = params_flatten(macromodel)
        exploration_results, exploration_axes = explore(macromodel, flattened_params, tfs[0])

        print(exploration_results[0])
        print(exploration_axes[0])