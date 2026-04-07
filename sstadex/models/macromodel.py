from unicodedata import name
from pathlib import Path
import subprocess
import re

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
    netlist_params: dict[str, Any] | None = None
class Macromodel:
    def __init__(
        self,
        name="",
        netlist=None,
        model=None,
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
        self.model = model
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

    def add_instance(
        self,
        name: str,
        block,
        net_map: dict[str, str],
        index: int | None = None,
        netlist_params: dict[str, Any] | None = None,
    ) -> None:
        self.instances.append(
            NetlistInstance(
                name=name,
                block=block,
                net_map=net_map,
                index=index,
                netlist_params=netlist_params or {},
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

    def render_small_signal_instance(self, instance_name: str, net_map: dict[str, str]) -> str:
        if self.model is None:
            raise ValueError(
                f"Macromodel '{self.name}' has no simplified model defined."
            )
        print('DEBUG MESSAGE')
        print(net_map)
        missing = [pin for pin in self.ports if pin not in net_map]
        if missing:
            raise KeyError(
                f"Macromodel '{self.name}' missing nets for ports: {missing}"
            )

        tokens = {"INSTANCE": instance_name}
        for port in self.ports:
            tokens[port] = net_map[port]

        return self.model.format(**tokens)

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

    def _resolve_instance_netlist_params(
        self,
        inst: NetlistInstance,
        point: dict | None,
    ) -> dict[str, Any]:
        if point is None:
            return dict(inst.netlist_params or {})

        resolved = {}
        for key, value in (inst.netlist_params or {}).items():
            resolved[key] = point[value] if value in point else value
        return resolved

    def render_subckt(self) -> str:
        header = f".subckt {self.subckt_name} {' '.join(self.ports)}"

        body_lines = []
        for inst in self.instances:
            print("inst.net_map: ", inst.net_map)
            body_lines.append(
                inst.block.render_instance(
                    instance_name=inst.name,
                    net_map=inst.net_map,
                )
            )

        footer = f".ends {self.subckt_name}"

        return "\n".join([header, *body_lines, footer])

    def _coerce_spice_block(self, block) -> str:
        if block is None:
            return ""
        if isinstance(block, list):
            return "\n".join(str(line) for line in block if str(line).strip()).strip()
        return str(block).strip()

    def _normalize_extra_spice(self, extra_spice=None) -> dict[str, str]:
        if extra_spice is None:
            return {"pre": "", "body": "", "post": ""}

        if isinstance(extra_spice, str):
            return {"pre": "", "body": extra_spice.strip(), "post": ""}

        return {
            "pre": self._coerce_spice_block(extra_spice.get("pre", "")),
            "body": self._coerce_spice_block(extra_spice.get("body", "")),
            "post": self._coerce_spice_block(extra_spice.get("post", "")),
        }

    def _assemble_netlist(self, pre: str, core: str, post: str) -> str:
        parts = []

        if pre:
            parts.append(pre)
        if core:
            parts.append(core)
        if post:
            parts.append(post)

        if not parts:
            return ""

        return "\n\n".join(parts) + "\n"

    def _gen_physical_netlist(self, extra_spice=None) -> str:
        extra = self._normalize_extra_spice(extra_spice)
        subckts = self._collect_subckts()
        top = self.render_subckt()

        if extra["body"]:
            top = top.replace(
                f".ends {self.subckt_name}",
                f"{extra['body']}\n.ends {self.subckt_name}",
            )

        core_parts = [*subckts, top]
        core = "\n\n".join(part for part in core_parts if part)
        return self._assemble_netlist(extra["pre"], core, extra["post"])

    def gen_netlist_for_params(self, point: dict, extra_spice=None) -> str:
        extra = self._normalize_extra_spice(extra_spice)
        emitted = set()
        subckts = []

        for inst in self.instances:
            block = inst.block
            key = getattr(block, "subckt_name", getattr(block, "name", inst.name))

            if key in emitted or not hasattr(block, "render_subckt"):
                continue

            resolved_params = self._resolve_instance_netlist_params(inst, point)
            subckts.append(
                block.render_subckt(
                    index=inst.index,
                    netlist_params=resolved_params,
                    use_defaults=False,
                )
            )
            emitted.add(key)

        top = self.render_subckt()
        if extra["body"]:
            top = top.replace(
                f".ends {self.subckt_name}",
                f"{extra['body']}\n.ends {self.subckt_name}",
            )

        core = "\n\n".join([*subckts, top])
        self.netlist = self._assemble_netlist(extra["pre"], core, extra["post"])
        return self.netlist

    def gen_netlist(self, view: str = "physical", extra_spice=None) -> str:
        if view == "physical":
            self.netlist = self._gen_physical_netlist(extra_spice=extra_spice)
            return self.netlist

        if view == "small_signal":
            self.netlist = self._gen_small_signal_netlist(extra_spice=extra_spice)
            return self.netlist

        raise ValueError(f"Unknown view '{view}'. Expected 'physical' or 'small_signal'.")

    def _gen_small_signal_netlist(self, extra_spice=None) -> str:
        extra = self._normalize_extra_spice(extra_spice)
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

        if extra["body"]:
            lines.append(extra["body"])

        core = "\n".join(lines)
        return self._assemble_netlist(extra["pre"], core, extra["post"])

    def _normalize_sim_points(self, params) -> list[dict]:
        if isinstance(params, dict):
            list_lengths = {}
            scalar_values = {}

            for key, value in params.items():
                if isinstance(value, np.ndarray):
                    if value.ndim == 0:
                        scalar_values[key] = value.item()
                    else:
                        list_lengths[key] = value.tolist()
                elif isinstance(value, (list, tuple)):
                    list_lengths[key] = list(value)
                else:
                    scalar_values[key] = value

            if not list_lengths:
                return [params]

            lengths = {len(values) for values in list_lengths.values()}
            if len(lengths) != 1:
                raise ValueError(
                    "All iterable values in params must have the same length."
                )

            num_points = lengths.pop()
            points = []
            for idx in range(num_points):
                point = {}
                for key, values in list_lengths.items():
                    point[key] = values[idx]
                for key, value in scalar_values.items():
                    point[key] = value
                points.append(point)

            return points
        if isinstance(params, list):
            return params
        if hasattr(params, "to_dict"):
            return params.to_dict(orient="records")
        raise TypeError("params must be a dict, list[dict], or pandas.DataFrame.")

    def _run_ngspice(
        self,
        netlist_text: str,
        run_name: str,
        workdir: str = "./simulations",
    ) -> dict:
        workdir_path = Path(workdir)
        workdir_path.mkdir(parents=True, exist_ok=True)

        spice_path = workdir_path / f"{run_name}.spice"
        log_path = workdir_path / f"{run_name}.log"

        spice_path.write_text(netlist_text)

        proc = subprocess.run(
            ["ngspice", "-b", "-o", str(log_path), str(spice_path)],
            capture_output=True,
            text=True,
        )

        return {
            "run_name": run_name,
            "spice_path": spice_path,
            "log_path": log_path,
            "returncode": proc.returncode,
            "stdout": proc.stdout,
            "stderr": proc.stderr,
        }

    def _extract_log_variables(
        self,
        log_path: Path,
        variables: list[str] | None,
    ) -> dict[str, float | None]:
        if not variables:
            return {}

        if not log_path.exists():
            return {variable: None for variable in variables}

        log_text = log_path.read_text()
        extracted = {}

        for variable in variables:
            patterns = [
                rf"v\(\s*{re.escape(variable)}\s*\)\s*=\s*([^\s]+)",
                rf"\b{re.escape(variable)}\b\s*=\s*([^\s]+)",
            ]

            value = None
            for pattern in patterns:
                matches = re.findall(pattern, log_text, flags=re.IGNORECASE)
                if matches:
                    raw_value = matches[-1]
                    try:
                        value = float(raw_value)
                    except ValueError:
                        value = raw_value
                    break

            extracted[variable] = value

        return extracted

    def ngspice_sim(
        self,
        params,
        extra_spice=None,
        workdir: str = "./simulations",
        variables: list[str] | None = None,
    ) -> list[dict]:
        points = self._normalize_sim_points(params)
        results = []

        for idx, point in enumerate(points):
            run_name = f"{self.name}_{idx}"
            print("run_name: ", run_name)
            netlist_text = self.gen_netlist_for_params(
                point=point,
                extra_spice=extra_spice,
            )
            print(f"Running ngspice simulation for point {idx}: {point}")
            print(f"Generated netlist:\n{netlist_text}")
            sim_result = self._run_ngspice(
                netlist_text=netlist_text,
                run_name=run_name,
                workdir=workdir,
            )
            sim_result["params"] = point
            sim_result["variables"] = self._extract_log_variables(
                log_path=sim_result["log_path"],
                variables=variables,
            )
            results.append(sim_result)

        return results


class Test:
    def __init__(
        self,
        name="",
        tf=None,
        netlist="",
        parametros=None,
        variables=None,
        out_def=None,
        composed=0,
        lamb=None,
        target_param="",
        only_up=False,
        opt_goal="max",
        conditions=None,
        testbench=None,
    ):
        self.name = name
        self.tf = tf
        self.netlist = netlist
        self.composed = composed
        self.parametros = parametros or {}
        self.variables = variables or {}
        self.out_def = out_def or {}
        self.lamd = lamb
        self.target_param = target_param
        self.only_up = only_up
        self.opt_goal = opt_goal
        self.conditions = conditions or {}
        self.testbench = testbench

    def eval(funct):
        return {"eval": funct}
