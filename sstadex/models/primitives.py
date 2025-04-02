from .transistor import Transistor
import numpy as np


class Primitive:
    def __init__(self, name="", parameters={}):
        self.name = name
        self.parameters = parameters


class common_source(Primitive):
    def __init__(self, netlist="", inputs=[], outputs=[], type="", conf=0):
        self.netlst = netlist
        self.inputs = inputs
        self.outputs = outputs
        self.type = type
        self.conf = conf

    def build(self):
        pt_transistor = Transistor(
            "../../LUTs/IHP_LUT_lv_20w.npy",
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
            conf=self.conf,
        )

        self.mesh = np.meshgrid(
            self.inputs[self.inputs["2d_var"][0]], self.inputs[self.inputs["2d_var"][1]]
        )

        pos = 0
        input_has_l = False
        for in_ in self.inputs["2d_var"]:
            if in_ == "length":
                self.L = np.repeat(
                    self.inputs["length"], np.asarray(self.mesh[0]).shape[pos]
                )
                input_has_l = True
            else:
                pos += 1

        if not input_has_l:
            if self.conf == 1:
                self.L = np.repeat(
                    self.inputs["length"],
                    len(self.inputs[self.inputs["2d_var"][0]])
                    * len(self.inputs[self.inputs["2d_var"][1]]),
                )
            else:
                self.L = np.repeat(
                    self.inputs["length"], len(self.inputs[self.inputs["2d_var"][0]])
                )

        self.W = self.inputs["il"] / pt_transistor.jd
        self.gdsid = pt_transistor.gds / pt_transistor.id
        self.gds = self.gdsid * self.inputs["il"]
        self.Ro = 1 / self.gds
        self.cgg = (self.W * pt_transistor.cgg) / 20e-6
        self.gm = pt_transistor.gmid * self.inputs["il"]
        self.cgs = (self.W * pt_transistor.cgs) / 20e-6
        self.cgd = (self.W * pt_transistor.cgd) / 20e-6


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
            "../../LUTs/IHP_LUT_hv_20w.npy",
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

        self.mesh = np.meshgrid(
            self.inputs[self.inputs["2d_var"][0]], self.inputs[self.inputs["2d_var"][1]]
        )

        pos = 0
        input_has_l = False
        for in_ in self.inputs["2d_var"]:
            if in_ == "length":
                self.L = np.repeat(self.inputs["length"], self.mesh[0].shape[pos])
                input_has_l = True
            else:
                pos += 1

        if not input_has_l:
            self.L = np.repeat(
                self.inputs["length"], len(self.inputs[self.inputs["2d_var"][0]])
            )

        self.W = self.inputs["il"] / pt_transistor.jd
        self.gdsid = pt_transistor.gds / pt_transistor.id
        self.gds = self.gdsid * self.inputs["il"]
        self.Ro = 1 / self.gds
        self.cgg = (self.W * pt_transistor.cgg) / 20e-6
        self.gm = pt_transistor.gmid * self.inputs["il"]
        self.cgs = (self.W * pt_transistor.cgs) / 20e-6
        self.cgd = (self.W * pt_transistor.cgd) / 20e-6


class cs_pmos(Primitive):
    def __init__(self, netlist="", inputs=[], outputs=[], type=""):
        self.netlst = netlist
        self.inputs = inputs
        self.outputs = outputs
        self.type = type

    def build(self):
        pt_transistor = Transistor(
            "../../LUTs/IHP_LUT_hv_20w.npy",
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

        self.mesh = np.meshgrid(
            self.inputs[self.inputs["2d_var"][0]], self.inputs[self.inputs["2d_var"][1]]
        )

        pos = 0
        input_has_l = False
        for in_ in self.inputs["2d_var"]:
            if in_ == "length":
                self.L = np.repeat(
                    self.inputs["length"], np.asarray(self.mesh[0]).shape[pos]
                )
                input_has_l = True
            else:
                pos += 1

        if not input_has_l:
            self.L = np.repeat(
                self.inputs["length"], len(self.inputs[self.inputs["2d_var"][0]])
            )

        self.W = self.inputs["il"] / pt_transistor.jd
        self.gdsid = pt_transistor.gds / pt_transistor.id
        self.gds = self.gdsid * self.inputs["il"]
        self.Ro = 1 / self.gds
        self.cgg = (self.W * pt_transistor.cgg) / 20e-6
        self.gm = pt_transistor.gmid * self.inputs["il"]
        self.cgs = (self.W * pt_transistor.cgs) / 20e-6
        self.cgd = (self.W * pt_transistor.cgd) / 20e-6


class cs_nmos(Primitive):
    def __init__(self, netlist="", inputs=[], outputs=[], type=""):
        self.netlst = netlist
        self.inputs = inputs
        self.outputs = outputs
        self.type = type

    def build(self):
        pt_transistor = Transistor(
            "../../LUTs/ihp_lut_hv_20w.npy",
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
        self.gdsid = pt_transistor.gds / pt_transistor.id
        self.gds = self.gdsid * self.inputs["il"]
        self.Ro = 1 / self.gds
        self.cgg = (self.W * pt_transistor.cgg) / 20e-6
        self.gm = pt_transistor.gmid * self.inputs["il"]
        self.cgs = (self.W * pt_transistor.cgs) / 20e-6
        self.cgd = (self.W * pt_transistor.cgd) / 20e-6


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
            "../../LUTs/IHP_LUT_hv_20w.npy",
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

        self.mesh = np.meshgrid(
            self.inputs[self.inputs["2d_var"][0]], self.inputs[self.inputs["2d_var"][1]]
        )

        pos = 0
        input_has_l = False
        for in_ in self.inputs["2d_var"]:
            if in_ == "length":
                self.L = np.repeat(
                    self.inputs["length"], np.asarray(self.mesh[0]).shape[pos]
                )
                input_has_l = True
            else:
                pos += 1

        if not input_has_l:
            self.L = np.repeat(
                self.inputs["length"], len(self.inputs[self.inputs["2d_var"][0]])
            )

        self.W = self.inputs["il"] / pt_transistor.jd
        self.gdsid = pt_transistor.gds / pt_transistor.id
        self.gds = self.gdsid * self.inputs["il"]
        self.Ro = 1 / self.gds
        self.cgg = (self.W * pt_transistor.cgg) / 20e-6
        self.gm = pt_transistor.gmid * self.inputs["il"]
        self.cgs = (self.W * pt_transistor.cgs) / 20e-6
        self.cgd = (self.W * pt_transistor.cgd) / 20e-6


class diffpair_cc(Primitive):
    def __init__(self, vcp, vs, vin, vout, il, sweep_size, lengths, vbias_end):
        self.vcp = vcp
        self.vs = vs
        self.vin = vin
        self.vout = vout
        self.il = il
        self.sweep_size = sweep_size
        self.lengths = lengths
        self.vbias_end = vbias_end

    def build(self):
        aux_mesh = np.meshgrid(self.vs, self.vcp)
        vds_m1 = aux_mesh[1] - aux_mesh[0]
        vds_m1 = vds_m1.flatten()
        vgs_m1 = np.tile(self.vin - self.vs, self.sweep_size)

        M1 = Transistor(
            "../../LUTs/IHP_LUT_hv_20w.npy",
            "nmos",
            0,
            (0.1, 2, 0.01),
            (0.1, 2.1, 0.01),
            [0.4e-06, 0.8e-06, 1.6e-06, 3.2e-06, 6.4e-6],
            ["vds", "vgs"],
            [
                vds_m1,
                vgs_m1,
            ],
        )

        print("M1 jd: ", M1.jd)
        print("M1 gmid: ", M1.gmid)

        vcp = np.tile(aux_mesh[1].flatten(), 5)
        vbias = []
        vdsm2 = []
        for i in vcp:
            vdsm2.append(np.repeat(self.vout - i, self.sweep_size))
            vbias.append(np.linspace(i, self.vbias_end, self.sweep_size))

        vdsm2 = np.asarray(vdsm2).flatten()
        vbias = np.asarray(vbias).flatten()
        vgsm2 = vbias - np.repeat(vcp, self.sweep_size)

        M2 = Transistor(
            "../../LUTs/IHP_LUT_hv_20w.npy",
            "nmos",
            0,
            (0.1, 2, 0.01),
            (0.1, 2.1, 0.01),
            [0.4e-06, 0.8e-06, 1.6e-06, 3.2e-06, 6.4e-6],
            ["vds", "vgs"],
            [
                vdsm2,
                vgsm2,
            ],
        )

        print("M2 jd: ", M2.jd)
        print("M2 gmid: ", M2.gmid)

        id = self.il

        self.vcp = np.tile(np.repeat(vcp, self.sweep_size), 5)
        # vs_m1 = np.tile(np.repeat(np.tile(aux_mesh[1].flatten(), 5), sweep_size), 5)

        gmid_m1 = np.tile(np.repeat(M1.gmid.flatten(), self.sweep_size), 5)
        gmid_m2 = M2.gmid.flatten()

        gdsid_m1 = np.tile(np.repeat(M1.gds.flatten(), self.sweep_size), 5) / np.tile(
            np.repeat(M1.id.flatten(), self.sweep_size), 5
        )
        gdsid_m2 = M2.gds.flatten() / M2.id.flatten()

        jds_m1 = np.tile(np.repeat(M1.jd, self.sweep_size), 5)
        jds_m2 = M2.jd.flatten()

        cgd_m1 = np.tile(np.repeat(M1.cgd.flatten(), self.sweep_size), 5)
        cgs_m1 = np.tile(np.repeat(M1.cgs.flatten(), self.sweep_size), 5)
        cds_m1 = np.tile(np.repeat(M1.cds.flatten(), self.sweep_size), 5)

        cgd_m2 = M2.cgd.flatten()
        cgs_m2 = M2.cgs.flatten()
        cds_m2 = M2.cds.flatten()

        cgg_m2 = M2.cgg.flatten()

        W_m1 = id / jds_m1
        W_m2 = id / jds_m2
        # W = W_m1+W_m2

        L_m1 = np.tile(
            np.repeat(
                np.repeat(self.lengths, self.sweep_size * self.sweep_size),
                self.sweep_size,
            ),
            5,
        )
        L_m2 = np.repeat(self.lengths, len(vdsm2))
        # L = L_m1+L_m2

        # WL = W+L

        gm = gmid_m1 * id
        gds = (gdsid_m1 * id * gdsid_m2 * id) / (gmid_m2 * id)
        # gain = 20*np.log10(gm/gds)

        self.W_m1 = W_m1
        self.W_m2 = W_m2
        self.L_m1 = L_m1
        self.L_m2 = L_m2

        self.W = self.W_m1 + self.W_m2
        self.L = self.L_m1 + self.L_m2

        self.gm = gm
        self.gds = gds
        self.Ro = 1 / self.gds

        self.gm_m1 = gmid_m1 * id
        self.gm_m2 = gmid_m2 * id

        self.Ro_m1 = 1 / (gdsid_m1 * id)
        self.Ro_m2 = 1 / (gdsid_m2 * id)

        self.cgd_m1 = cgd_m1 * W_m1 / 20e-6
        self.cgs_m1 = cgs_m1 * W_m1 / 20e-6
        self.cds_m1 = cds_m1 * W_m1 / 20e-6

        self.cgd_m2 = cgd_m2 * W_m2 / 20e-6
        self.cgs_m2 = cgs_m2 * W_m2 / 20e-6
        self.cds_m2 = cds_m2 * W_m2 / 20e-6

        self.cout = cgd_m2
        self.ccp = cgg_m2

        self.vbias = np.tile(vbias, 5)


class current_mirror_cc:
    def __init__(self, vcp, vout, il, sweep_size, lengths, vbias_end):
        self.vcp = vcp
        self.vout = vout
        self.il = il
        self.sweep_size = sweep_size
        self.lengths = lengths
        self.vbias_end = vbias_end

    def build(self):
        vds_m1 = self.vcp

        M1 = Transistor(
            "../../LUTs/IHP_LUT_hv_20w.npy",
            "pmos",
            0,
            (-2, -0.1, 0.01),
            self.vout,
            [0.4e-06, 0.8e-06, 1.6e-06, 3.2e-06, 6.4e-6],
            ["length", "vds"],
            [
                self.lengths,
                vds_m1,
            ],
        )

        print("M1 jd: ", M1.jd)
        print("M1 gmid: ", M1.gmid)

        vcp = np.tile(self.vcp, 5)
        vbias = []
        vdsm2 = []
        for i in vcp:
            vdsm2.append(np.repeat(self.vout - i, self.sweep_size))
            vbias.append(np.linspace(i - 0.1, self.vbias_end, self.sweep_size))

        vdsm2 = np.asarray(vdsm2).flatten()
        vbias = np.asarray(vbias).flatten()
        vgsm2 = vbias - np.repeat(vcp, self.sweep_size)

        M2 = Transistor(
            "../../LUTs/IHP_LUT_hv_20w.npy",
            "pmos",
            0,
            (-2, -0.1, 0.01),
            (-2.1, -0.1, 0.01),
            [0.4e-06, 0.8e-06, 1.6e-06, 3.2e-06, 6.4e-6],
            ["vds", "vgs"],
            [
                vdsm2,
                vgsm2,
            ],
        )

        print("M2 jd: ", M2.jd)
        print("M2 gmid: ", M2.gmid)

        id = self.il

        self.vcp = np.tile(np.repeat(vcp, self.sweep_size), 5)
        # vs_m1 = np.tile(np.repeat(np.tile(aux_mesh[1].flatten(), 5), sweep_size), 5)

        gmid_m1 = np.tile(np.repeat(M1.gmid.flatten(), self.sweep_size), 5)
        gmid_m2 = M2.gmid.flatten()

        gdsid_m1 = np.tile(np.repeat(M1.gds.flatten(), self.sweep_size), 5) / np.tile(
            np.repeat(M1.id.flatten(), self.sweep_size), 5
        )
        gdsid_m2 = M2.gds.flatten() / M2.id.flatten()

        jds_m1 = np.tile(np.repeat(M1.jd, self.sweep_size), 5)
        jds_m2 = M2.jd.flatten()

        W_m1 = id / jds_m1
        W_m2 = id / jds_m2

        L_m1 = np.tile(
            np.repeat(
                np.repeat(self.lengths, self.sweep_size),
                self.sweep_size,
            ),
            5,
        )
        L_m2 = np.repeat(self.lengths, len(vdsm2))
        # L = L_m1+L_m2

        # WL = W+L

        cgd_m1 = np.tile(np.repeat(M1.cgd.flatten(), self.sweep_size), 5)
        cgd_m2 = M2.cgd.flatten()
        cgg_m2 = M2.cgg.flatten()

        gm = gmid_m1 * id
        gds = (gdsid_m1 * id * gdsid_m2 * id) / (gmid_m2 * id)
        # gain = 20*np.log10(gm/gds)

        self.W_m1 = W_m1
        self.W_m2 = W_m2
        self.L_m1 = L_m1
        self.L_m2 = L_m2

        self.W = self.W_m1 + self.W_m2
        self.L = self.L_m1 + self.L_m2

        self.gm = gm
        self.gds = gds
        self.Ro = 1 / self.gds

        self.gm_m1 = gmid_m1 * id
        self.gm_m2 = gmid_m2 * id

        self.Ro_m1 = 1 / (gdsid_m1 * id)
        self.Ro_m2 = 1 / (gdsid_m2 * id)

        self.vbias = np.tile(vbias, 5)

        self.cout = cgd_m2
        self.ccp_m1 = cgd_m1
        self.ccp_m2 = cgg_m2


def simple_active_load():
    def __init__(
        vout,
        vin,
    ):
        pass

    def build(self):
        pass
