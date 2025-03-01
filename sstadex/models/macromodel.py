import numpy as np


class Macromodel:
    def __init__(
        self,
        name="",
        netlist=None,
        electrical_parameters={},
        specifications={},
        electrical_variables={},
        macromodel_parameters={},
        submacromodels=[],
        primitives=[],
        A=[],
        X=[],
        Z=[],
        eq_solutions={},
        nodes={},
        req_tfs=[],
        transistors={},
        tfs_sol=[],
        its_final=False,
        outputs=[],
        is_primitive=False,
        ext_mask=None,
    ):
        self.name = name
        self.netlist = netlist
        self.electrical_parameters = electrical_parameters
        self.specifications = specifications
        self.electrical_variables = electrical_variables
        self.macromodel_parameters = macromodel_parameters
        self.submacromodels = submacromodels
        self.primitives = primitives
        self.A, self.X, self.Z = A, X, Z
        self.eq_solutions = eq_solutions
        self.nodes = nodes
        self.req_tfs = req_tfs
        self.transistors = transistors
        self.tfs_sol = tfs_sol
        self.its_final = its_final
        self.outputs = outputs
        self.is_primitive = is_primitive
        self.ext_mask = ext_mask

    def hasPrimitive(self):
        if len(self.primitives) == 0:
            return False
        else:
            return True

    def addSubmacromodels(self, submacromodel):
        self.submacromodels.append(submacromodel)

    def update(self, macro_results):
        results_df = macro_results[3]

        for param in self.macromodel_parameters:
            for spec in self.specifications:
                print("param: ", param)
                print(spec.target_param)
                if param == spec.target_param:
                    self.macromodel_parameters[param] = results_df[spec.name].values
                elif spec.target_param != "":
                    shape = np.asarray(results_df["area"].values).shape
                    print(shape)
                    if shape[0] > self.macromodel_parameters[param].shape[0]:
                        self.macromodel_parameters[param] = np.resize(
                            np.asarray(self.macromodel_parameters[param]), shape
                        )
                    else:
                        self.macromodel_parameters[param] = self.macromodel_parameters[
                            param
                        ][: len(results_df["area"])]

        self.output_results = {}
        for output in self.outputs:
            self.output_results[output] = results_df[output].values

        self.is_primitive = True
        print("outputs results: ", self.output_results)
        print("macromodel parameters updated: ", self.macromodel_parameters)


class Test:
    def __init__(self, composed=0, parametros={}, lamb=None, target_param=""):
        self.composed = composed
        self.parametros = parametros
        self.lamd = lamb
        self.target_param = target_param

    def eval(funct):
        return {"eval": funct}
