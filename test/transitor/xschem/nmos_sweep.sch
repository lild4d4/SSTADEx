v {xschem version=3.4.6 file_version=1.2}
G {}
K {}
V {}
S {}
E {}
N -300 60 -300 80 {lab=vs}
N -300 30 -270 30 {lab=vs}
N -270 30 -270 80 {lab=vs}
N -300 80 -270 80 {lab=vs}
N -300 90 -300 100 {lab=vs}
N -300 -20 -300 0 {lab=vd}
N -190 -20 -190 -0 {lab=vd}
N -300 -20 -190 -20 {lab=vd}
N -300 -40 -300 -20 {lab=vd}
N -300 90 -190 90 {lab=vs}
N -300 80 -300 90 {lab=vs}
N -190 60 -190 90 {lab=vs}
N -380 30 -380 40 {lab=vg}
N -380 30 -340 30 {lab=vg}
N -380 100 -300 100 {lab=vs}
C {sg13g2_pr/sg13_lv_nmos.sym} -320 30 0 0 {name=M2
l=6.4u
w=5u
ng=1
m=1
model=sg13_lv_nmos
spiceprefix=X
}
C {vsource.sym} -190 30 0 0 {name=V1 value=1.1 savecurrent=false}
C {vsource.sym} -380 70 0 0 {name=V3 value=1.1 savecurrent=false}
C {code_shown.sym} -120 -300 0 0 {name=NGSPICE only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOSlv.lib mos_tt

.control
op

save @n.xm2.nsg13_lv_nmos[gm]
save @n.xm2.nsg13_lv_nmos[gds]
save @n.xm2.nsg13_lv_nmos[ids]

let gm = @n.xm2.nsg13_lv_nmos[gm]
let gds = @n.xm2.nsg13_lv_nmos[gds]
let ids = @n.xm2.nsg13_lv_nmos[ids]
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
C {lab_pin.sym} -360 30 0 0 {name=p4 sig_type=std_logic lab=vg}
C {lab_pin.sym} -300 -40 0 0 {name=p2 sig_type=std_logic lab=vd}
C {lab_pin.sym} -300 100 0 0 {name=p6 sig_type=std_logic lab=vs}
