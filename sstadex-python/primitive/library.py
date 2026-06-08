"""
analoglib.core.library
======================
Central registry for analog primitives.

Usage
-----
    from analoglib.core.library import Library
    from analoglib.primitives.simplediffpair import SimpleDiffPair
    from analoglib.primitives.cs_pmos import CommonSourcePMOS

    # --- build a project library ---
    lib = Library("my_ldo_project")
    lib.register(SimpleDiffPair,     lut_file="luts/ihp_nmos.npy", lut_w=10e-6)
    lib.register(CommonSourcePMOS,   lut_file="luts/ihp_pmos.npy", lut_w=10e-6)

    # --- instantiate and bias a primitive ---
    dp = lib.instantiate("simplediffpair", il=50e-6)
    dp.set_port_voltages({
        "VINP":  0.9,
        "VINN":  0.9,
        "VOUTP": 1.5,
        "VOUTN": 1.5,
        "VTAIL": 0.2,
        "VDD":   1.8,
        "VSS":   0.0,
    })
    df = dp.build()

    # --- render a SPICE snippet ---
    netlist = dp.render_netlist({"VINP": "net_in_p", "VINN": "net_in_n", ...})
"""

from __future__ import annotations
from typing import Type
from .primitive import Primitive


class Library:
    """
    Registry of Primitive classes with their default constructor kwargs.

    Parameters
    ----------
    name : str
        Project / technology library name (e.g. 'ihp_sg13g2').
    """

    def __init__(self, name: str = "default"):
        self.name = name
        self._registry: dict[str, tuple[Type[Primitive], dict]] = {}

    # ------------------------------------------------------------------
    # Registration
    # ------------------------------------------------------------------

    def register(
        self,
        primitive_class: Type[Primitive],
        **default_kwargs,
    ) -> None:
        """
        Register a Primitive class with default constructor arguments.

        Parameters
        ----------
        primitive_class : Type[Primitive]
            The class to register (not an instance).
        **default_kwargs
            Default keyword arguments passed to the constructor
            (e.g. lut_file, lut_w). Can be overridden at instantiation.
        """
        key = primitive_class.name
        if not key:
            raise ValueError(
                f"{primitive_class.__name__} must define a non-empty class-level `name`."
            )
        self._registry[key] = (primitive_class, default_kwargs)
        print(f"[Library:{self.name}] registered '{key}'")

    # ------------------------------------------------------------------
    # Instantiation
    # ------------------------------------------------------------------

    def instantiate(self, primitive_name: str, **override_kwargs) -> Primitive:
        """
        Create an instance of a registered primitive.

        Parameters
        ----------
        primitive_name : str
            Name as registered (e.g. 'simplediffpair').
        **override_kwargs
            Override / extend the default constructor kwargs.

        Returns
        -------
        Primitive
            A fresh, un-built primitive instance.
        """
        if primitive_name not in self._registry:
            available = list(self._registry.keys())
            raise KeyError(
                f"Primitive '{primitive_name}' not in library '{self.name}'. "
                f"Available: {available}"
            )
        cls, defaults = self._registry[primitive_name]
        kwargs = {**defaults, **override_kwargs}
        instance = cls(**kwargs)
        return instance

    # ------------------------------------------------------------------
    # Introspection
    # ------------------------------------------------------------------

    def list(self) -> list[str]:
        """Return names of all registered primitives."""
        return list(self._registry.keys())

    def describe(self, primitive_name: str) -> str:
        """Print the summary of a registered primitive (with default kwargs)."""
        p = self.instantiate(primitive_name)
        return p.summary()

    def __repr__(self) -> str:
        return f"<Library '{self.name}': {self.list()}>"