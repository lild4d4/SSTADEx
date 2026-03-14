from unicodedata import name

import numpy as np
from sympy import Symbol
from dataclasses import dataclass
from typing import Any

@dataclass
class NetlistInstance:
    name: str
    block: Any
    net_map: dict[str, str]
    index: int | None = None
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
        run_pareto=True,
        subs_expr=None,
        use_subs=False,
        ports=None,
        instances=None,
        subckt_name=None,
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
        self.run_pareto = run_pareto
        self.subs_expr = subs_expr
        self.use_subs = use_subs

        self.ports = ports or []
        self.instances = instances or []
        self.subckt_name = subckt_name or name

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

            if param == Symbol("Cin_2stage"):
                self.macromodel_parameters[param] = results_df[param].values
            elif param == Symbol("Cgd_2stage"):
                self.macromodel_parameters[param] = results_df[param].values

        self.output_results = {}
        for output in self.outputs:
            self.output_results[output] = results_df[output].values

        self.is_primitive = True
        print("outputs results: ", self.output_results)
        print("macromodel parameters updated: ", self.macromodel_parameters)

    def add_instance(self, name: str, block, net_map: dict[str, str], index: int | None = None) -> None:
        self.instances.append(
            NetlistInstance(
                name=name,
                block=block,
                net_map=net_map,
                index=index,
            )
        )

    def render_instance(self, instance_name: str, net_map: dict[str, str]) -> str:
        missing = [pin for pin in self.ports if pin not in net_map]
        if missing:
            raise KeyError(
                f"Macromodel '{self.name}' missing nets for ports: {missing}"
            )

        nets = [net_map[pin] for pin in self.ports]
        return f"{instance_name} {' '.join(nets)} {self.subckt_name}"

    def _collect_subckts(self, emitted=None) -> list[str]:
        if emitted is None:
            emitted = set()

        blocks = []
        for inst in self.instances:
            block = inst.block
            key = getattr(block, "subckt_name", getattr(block, "name", inst.name))

            if hasattr(block, "_collect_subckts"):
                nested = block._collect_subckts(emitted=emitted)
                for item in nested:
                    if item not in blocks:
                        blocks.append(item)

            if key not in emitted:
                if hasattr(block, "render_subckt"):
                    blocks.append(block.render_subckt(index=inst.index))
                    emitted.add(key)

        return blocks

    def render_subckt(self) -> str:
        header = f".subckt {self.subckt_name} {' '.join(self.ports)}"

        body_lines = []
        for inst in self.instances:
            body_lines.append(
                inst.block.render_instance(
                    instance_name=inst.name,
                    net_map=inst.net_map,
                )
            )

        footer = f".ends {self.subckt_name}"

        return "\n".join([header, *body_lines, footer])

    def gen_netlist(self, view: str = "physical") -> str:
        if view == "physical":
            self.netlist = self._gen_physical_netlist()
            return self.netlist

        if view == "small_signal":
            self.netlist = self._gen_small_signal_netlist()
            return self.netlist

        raise ValueError(f"Unknown view '{view}'. Expected 'physical' or 'small_signal'.")

    def _gen_small_signal_netlist(self) -> str:
        lines = []
    
        if getattr(self, "ports", None):
            header = f"* Small-signal netlist for {self.name}"
            lines.append(header)
    
        for inst in self.instances:
            block = inst.block
    
            if hasattr(block, "render_small_signal_instance"):
                lines.append(f"* instance {inst.name} ({getattr(block, 'name', 'unknown')})")
                lines.append(block.render_small_signal_instance(inst.name, inst.net_map))
                continue
            
            if hasattr(block, "_gen_small_signal_netlist"):
                lines.append(f"* nested macro instance {inst.name}")
                nested_text = block._gen_small_signal_netlist()
                lines.append(nested_text)
                continue
            
            raise TypeError(
                f"Instance '{inst.name}' block does not support small-signal rendering."
            )
    
        return "\n".join(lines)


class Test:
    def __init__(
        self, composed=0, parametros={}, lamb=None, target_param="", only_up=False
    ):
        self.composed = composed
        self.parametros = parametros
        self.lamd = lamb
        self.target_param = target_param
        self.only_up = only_up

    def eval(funct):
        return {"eval": funct}
