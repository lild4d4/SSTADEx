v {xschem version=3.4.6 file_version=1.2}
G {}
K {}
V {}
S {}
E {}
N -190 130 -190 160 {lab=vd}
N -190 100 -160 100 {lab=vs}
N -190 160 -190 170 {lab=vd}
N -190 50 -190 70 {lab=vs}
N -80 50 -80 70 {lab=vs}
N -160 50 -80 50 {lab=vs}
N -190 30 -190 50 {lab=vs}
N -190 160 -80 160 {lab=vd}
N -80 130 -80 160 {lab=vd}
N -270 100 -230 100 {lab=vg}
N -270 90 -270 100 {lab=vg}
N -270 30 -190 30 {lab=vs}
N -160 50 -160 100 {lab=vs}
N -190 50 -160 50 {lab=vs}
C {vsource.sym} -80 100 2 0 {name=V1 value=-0.4 savecurrent=false}
C {vsource.sym} -270 60 2 0 {name=V3 value=-0.567 savecurrent=false
}
C {code_shown.sym} -10 -230 0 0 {name=NGSPICE only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOSlv.lib mos_tt

.control
op

save @n.xm2.nsg13_lv_nmos[gm]
save @n.xm2.nsg13_lv_nmos[gds]
save @n.xm2.nsg13_lv_nmos[ids]

let gm = @n.xm1.nsg13_lv_pmos[gm]
let gds = @n.xm1.nsg13_lv_pmos[gds]
let ids = @n.xm1.nsg13_lv_pmos[ids]
let i_vds = i(V1)

let w = 5e-6
let i = 20e-6

let jd = ids/w
let gmid = gm/ids

print jd gmid ids i_vds

print i/jd
print gmid*i


.endc

"}
C {lab_pin.sym} -270 100 0 0 {name=p4 sig_type=std_logic lab=vg}
C {lab_pin.sym} -190 170 0 0 {name=p2 sig_type=std_logic lab=vd}
C {lab_pin.sym} -190 30 0 0 {name=p6 sig_type=std_logic lab=vs}
C {sg13g2_pr/sg13_lv_pmos.sym} -210 100 0 0 {name=M1
l=3.2u
w=5u
ng=1
m=1
model=sg13_lv_pmos
spiceprefix=X
}
