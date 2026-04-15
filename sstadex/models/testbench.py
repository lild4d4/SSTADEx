from dataclasses import dataclass, field
from typing import Any


@dataclass
class BenchElement:
    name: str

    def to_spice(self) -> str:
        raise NotImplementedError


@dataclass
class VoltageSource(BenchElement):
    nplus: str
    nminus: str
    value: Any

    def to_spice(self) -> str:
        return f"{self.name} {self.nplus} {self.nminus} {self.value}"


@dataclass
class CurrentSource(BenchElement):
    nplus: str
    nminus: str
    value: Any

    def to_spice(self) -> str:
        return f"{self.name} {self.nplus} {self.nminus} {self.value}"


@dataclass
class Resistor(BenchElement):
    n1: str
    n2: str
    value: Any

    def to_spice(self) -> str:
        return f"{self.name} {self.n1} {self.n2} {self.value}"


@dataclass
class Capacitor(BenchElement):
    n1: str
    n2: str
    value: Any

    def to_spice(self) -> str:
        return f"{self.name} {self.n1} {self.n2} {self.value}"

@dataclass
class Testbench:
    name: str
    dut: Any
    view: str = "small_signal"
    elements: list[BenchElement] = field(default_factory=list)
    tf: tuple[str, str] | list[str] | None = None
    parameter_map: dict = field(default_factory=dict)
    variables: dict = field(default_factory=dict)
    extra_spice: dict[str, str] | None = None

    def body_lines(self) -> list[str]:
        return [element.to_spice() for element in self.elements]

    def gen_netlist(self) -> str:
        extra_spice = dict(self.extra_spice or {})
        body = extra_spice.get("body", "")

        bench_body = "\n".join(self.body_lines()).strip()
        if body.strip() and bench_body:
            extra_spice["body"] = bench_body + "\n" + body
        elif bench_body:
            extra_spice["body"] = bench_body

        return self.dut.gen_netlist(view=self.view, extra_spice=extra_spice)

    def make_test(
        self,
        *,
        name: str,
        opt_goal: str,
        conditions: dict,
        out_def: dict | None = None,
        composed: int = 0,
        lamd=None,
        target_param="",
        only_up=False,
    ):
        from sstadex.models.macromodel import Test

        test = Test(
            composed=composed,
            parametros=self.parameter_map.copy(),
            lamb=lamd,
            target_param=target_param,
            only_up=only_up,
        )
        test.name = name
        test.tf = self.tf
        test.netlist = self.name
        test.opt_goal = opt_goal
        test.conditions = conditions
        test.variables = self.variables.copy()
        test.out_def = out_def if out_def is not None else {"eval": self.tf}
        test.testbench = self
        return test
