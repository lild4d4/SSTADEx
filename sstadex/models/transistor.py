# from mosplot import load_lookup_table, LoadMosfet
import sys

sys.path.insert(0, "../../..")
from gmid.mosplot import load_lookup_table, LoadMosfet
import numpy as np


class Transistor:
    def __init__(
        self, lookup_table_file, mos_type, vsb, vds, vgs, lengths, dof, dof_values
    ):
        self.lookup_table_file = lookup_table_file
        self.mos_type = mos_type
        self.vsb = vsb
        self.vds = vds
        self.vgs = vgs
        self.lengths = lengths
        self.dof = dof
        self.dof_values = dof_values

        self.lookup_table = load_lookup_table(lookup_table_file)

        mode = 0

        self.eof = []
        for d in self.dof:
            print(d[:6])
            if d[:6] == "length":
                self.eof.append(0)
            elif d[:3] == "vgs":
                self.eof.append(1)
            elif d[:3] == "vds":
                self.eof.append(2)
                try:
                    vgs_length = len(vgs)
                    if len(lengths) > 1 and vgs_length > 1:
                        mode = 1
                except:
                    mode = 0

        print(mode)

        if mode == 0:
            (
                self.jd,
                self.gmid,
                self.gds,
                self.cgg,
                self.cgd,
                self.cgs,
                self.vth,
                self.id,
                self.cds,
            ) = self.get_parameters(self.lengths)
        elif mode == 1:
            self.jd = []
            self.gmid = []
            self.gds = []
            self.cgg = []
            self.cgd = []
            self.cgs = []
            self.vth = []
            self.id = []
            self.cds = []
            for idx, length in enumerate(lengths):
                parameters = self.get_parameters(length)
                diagonals = [np.diag(x) for x in parameters]
                for param, diag in zip(
                    [
                        self.jd,
                        self.gmid,
                        self.gds,
                        self.cgg,
                        self.cgd,
                        self.cgs,
                        self.vth,
                        self.id,
                        self.cds,
                    ],
                    diagonals,
                ):
                    param.append(diag)

            self.jd = np.asarray(self.jd)
            self.gmid = np.asarray(self.gmid)
            self.gds = np.asarray(self.gds)
            self.cgg = np.asarray(self.cgg)
            self.cgd = np.asarray(self.cgd)
            self.cgs = np.asarray(self.cgs)
            self.vth = np.asarray(self.vth)
            self.id = np.asarray(self.id)
            self.cds = np.asarray(self.cds)

    def get_parameters(self, lengths_m):
        self.pt_lutable = LoadMosfet(
            lookup_table=self.lookup_table,
            mos=self.mos_type,
            vsb=self.vsb,
            vds=self.vds,
            vgs=self.vgs,
            lengths=lengths_m,
        )

        expressions = [
            self.pt_lutable.lengths_expression,
            self.pt_lutable.vgs_expression,
            self.pt_lutable.vds_expression,
        ]

        jd = self.pt_lutable.interpolate(
            x_expression=expressions[self.eof[0]],
            x_value=self.dof_values[0],
            y_expression=expressions[self.eof[1]],
            y_value=self.dof_values[1],
            z_expression=self.pt_lutable.current_density_expression,
        )
        gmid = self.pt_lutable.interpolate(
            x_expression=expressions[self.eof[0]],
            x_value=self.dof_values[0],
            y_expression=expressions[self.eof[1]],
            y_value=self.dof_values[1],
            z_expression=self.pt_lutable.gmid_expression,
        )
        gds = self.pt_lutable.interpolate(
            x_expression=expressions[self.eof[0]],
            x_value=self.dof_values[0],
            y_expression=expressions[self.eof[1]],
            y_value=self.dof_values[1],
            z_expression=self.pt_lutable.gds_expression,
        )
        cgg = self.pt_lutable.interpolate(
            x_expression=expressions[self.eof[0]],
            x_value=self.dof_values[0],
            y_expression=expressions[self.eof[1]],
            y_value=self.dof_values[1],
            z_expression=self.pt_lutable.cgg_expression,
        )

        cgd = self.pt_lutable.interpolate(
            x_expression=expressions[self.eof[0]],
            x_value=self.dof_values[0],
            y_expression=expressions[self.eof[1]],
            y_value=self.dof_values[1],
            z_expression=self.pt_lutable.cgd_expression,
        )

        cgs = self.pt_lutable.interpolate(
            x_expression=expressions[self.eof[0]],
            x_value=self.dof_values[0],
            y_expression=expressions[self.eof[1]],
            y_value=self.dof_values[1],
            z_expression=self.pt_lutable.cgs_expression,
        )

        cds = self.pt_lutable.interpolate(
            x_expression=expressions[self.eof[0]],
            x_value=self.dof_values[0],
            y_expression=expressions[self.eof[1]],
            y_value=self.dof_values[1],
            z_expression=self.pt_lutable.cds_expression,
        )

        vth = self.pt_lutable.interpolate(
            x_expression=expressions[self.eof[0]],
            x_value=self.dof_values[0],
            y_expression=expressions[self.eof[1]],
            y_value=self.dof_values[1],
            z_expression=self.pt_lutable.vth_expression,
        )

        id = self.pt_lutable.interpolate(
            x_expression=expressions[self.eof[0]],
            x_value=self.dof_values[0],
            y_expression=expressions[self.eof[1]],
            y_value=self.dof_values[1],
            z_expression=self.pt_lutable.id_expression,
        )

        return jd, gmid, gds, cgg, cgd, cgs, vth, id, cds
