from .transistor import Transistor


class Primitive:
    def __init__(self, name="", parameters={}):
        self.name = name
        self.parameters = parameters


class simplediffpair(Primitive):
    def __init__(self, name="", netlist="", inputs=[], outputs=[], type=""):
        super().__init__()
        self.name = name
        self.netlst = netlist
        self.inputs = inputs
        self.outputs = outputs
        self.type = type

    def build(self):
        pt_transistor = Transistor(
            "../../LUTs/sky130A_LUT_lvt.npy",
            self.type,
            0,
            self.inputs["vds_lut"],
            self.inputs["vgs_lut"],
            self.inputs["length"],
            self.inputs["2d_var"],
            [
                self.inputs[self.inputs["2d_var"][0]],
                self.inputs[self.inputs["2d_var"][1]],
            ],
        )

        self.W = self.inputs["il"] / pt_transistor.jd
        self.Ro = 20e-6 / (self.W * pt_transistor.gds)
        self.cgg = (self.W * pt_transistor.cgg) / 20e-6
        self.gm = pt_transistor.gmid * self.inputs["il"]


class cs_pmos(Primitive):
    def __init__(self, netlist="", inputs=[], outputs=[], type=""):
        self.netlst = netlist
        self.inputs = inputs
        self.outputs = outputs
        self.type = type

    def build(self):
        pt_transistor = Transistor(
            "../../LUTs/sky130A_LUT_lvt.npy",
            self.type,
            0,
            self.inputs["vds_lut"],
            self.inputs["vgs_lut"],
            self.inputs["length"],
            self.inputs["2d_var"],
            [
                self.inputs[self.inputs["2d_var"][0]],
                self.inputs[self.inputs["2d_var"][1]],
            ],
        )

        self.W = self.inputs["il"] / pt_transistor.jd
        self.Ro = 20e-6 / (self.W * pt_transistor.gds)
        self.cgg = (self.W * pt_transistor.cgg) / 20e-6
        self.gm = pt_transistor.gmid * self.inputs["il"]


class cm_pmos(Primitive):
    def __init__(self, name="", netlist="", inputs=[], outputs=[], type=""):
        super().__init__()
        self.name = name
        self.netlst = netlist
        self.inputs = inputs
        self.outputs = outputs
        self.type = type

    def build(self):
        pt_transistor = Transistor(
            "../../LUTs/sky130A_LUT_lvt.npy",
            self.type,
            0,
            self.inputs["vds_lut"],
            self.inputs["vgs_lut"],
            self.inputs["length"],
            self.inputs["2d_var"],
            [
                self.inputs[self.inputs["2d_var"][0]],
                self.inputs[self.inputs["2d_var"][1]],
            ],
        )

        self.W = self.inputs["il"] / pt_transistor.jd
        self.Ro = 20e-6 / (self.W * pt_transistor.gds)
        self.cgg = (self.W * pt_transistor.cgg) / 20e-6
        self.gm = pt_transistor.gmid * self.inputs["il"]
