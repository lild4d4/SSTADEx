v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 470 -70 500 -70 {
lab=vg}
N 540 -70 590 -70 {
lab=vs}
N 590 -70 590 -10 {
lab=vs}
N 540 -10 590 -10 {
lab=vs}
N 540 -40 540 -10 {
lab=vs}
N 540 -140 540 -100 {
lab=vout}
N 540 -240 540 -230 {
lab=vdd}
N 690 -140 690 -80 {
lab=vout}
N 690 -20 690 110 {
lab=vss}
N 560 110 690 110 {
lab=vss}
N 540 -200 560 -200 {
lab=vdd}
N 560 -240 560 -200 {
lab=vdd}
N 540 -240 560 -240 {
lab=vdd}
N 540 -270 540 -240 {
lab=vdd}
N 310 -40 310 -10 {
lab=vs}
N 360 -10 540 -10 {
lab=vs}
N 310 -70 360 -70 {
lab=vs}
N 360 -70 360 -10 {
lab=vs}
N 310 -10 360 -10 {
lab=vs}
N 400 -200 500 -200 {
lab=#net1}
N 310 -150 310 -100 {
lab=#net1}
N 400 -200 400 -150 {
lab=#net1}
N 350 -200 400 -200 {
lab=#net1}
N 310 -150 400 -150 {
lab=#net1}
N 310 -170 310 -150 {
lab=#net1}
N 310 -250 310 -230 {
lab=vdd}
N 310 -270 540 -270 {
lab=vdd}
N 540 -140 690 -140 {
lab=vout}
N 540 -170 540 -140 {
lab=vout}
N 540 -10 540 20 {
lab=vs}
N 540 80 540 110 {
lab=vss}
N 280 -200 310 -200 {
lab=vdd}
N 280 -250 280 -200 {
lab=vdd}
N 280 -250 310 -250 {
lab=vdd}
N 310 -270 310 -250 {
lab=vdd}
N 540 50 560 50 {
lab=vss}
N 560 50 560 110 {
lab=vss}
N 540 110 560 110 {
lab=vss}
N 190 50 500 50 {
lab=#net2}
N 130 80 130 110 {
lab=vss}
N 130 110 540 110 {
lab=vss}
N 130 10 130 20 {
lab=#net2}
N 130 10 190 10 {
lab=#net2}
N 130 -40 130 10 {
lab=#net2}
N 190 10 190 50 {
lab=#net2}
N 170 50 190 50 {
lab=#net2}
N 130 -270 130 -100 {
lab=vdd}
N 130 -270 310 -270 {
lab=vdd}
N 100 50 130 50 {
lab=vss}
N 100 50 100 110 {
lab=vss}
N 100 110 130 110 {
lab=vss}
C {devices/code_shown.sym} -750 -370 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt

Vdd vdd 0 3.3
Vgs vg 0 1.48 ac 1
Vss vss 0 0
Vref vref 0 1.48
.control
ac dec 10 1 1G
plot vdb(vout) 
op 
print vout
print vs
let gm_diff = @n.xm1.nsg13_hv_nmos[gm]
let id_diff = @n.xm1.nsg13_hv_nmos[ids]
let gds_diff = @n.xm1.nsg13_hv_nmos[gds]
let gm_al = @n.xm2.nsg13_hv_pmos[gm]
let id_al = @n.xm2.nsg13_hv_pmos[ids]
let gds_al = @n.xm2.nsg13_hv_pmos[gds]
let gm_cm = @n.xm5.nsg13_hv_nmos[gm]
let id_cm = @n.xm5.nsg13_hv_nmos[ids]
let gds_cm = @n.xm5.nsg13_hv_nmos[gds]
print gm_diff
print gds_diff
print id_diff
print gm_diff/id_diff
print gm_al
print gds_al
print id_al
print gm_al/id_al
print gm_cm
print gds_cm
print id_cm
print gm_cm/id_cm
.endc

"}
C {devices/lab_pin.sym} 470 -70 0 0 {name=p3 sig_type=std_logic lab=vg}
C {devices/lab_pin.sym} 540 110 3 0 {name=p6 sig_type=std_logic lab=vss}
C {sg13g2_pr/sg13_hv_nmos.sym} 520 -70 2 1 {name=M1
l=0.8u
w=39.375u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {devices/lab_pin.sym} 540 -270 2 0 {name=p2 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 690 -140 2 0 {name=p1 sig_type=std_logic lab=vout}
C {capa.sym} 690 -50 0 0 {name=C1
m=1
value=1p
footprint=1206
device="ceramic capacitor"}
C {sg13g2_pr/sg13_hv_pmos.sym} 520 -200 0 0 {name=M2
l=0.8u
w=2.416u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 290 -70 2 1 {name=M3
l=0.8u
w=39.375u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 330 -200 0 1 {name=M4
l=0.8u
w=2.416u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {devices/lab_pin.sym} 270 -70 0 0 {name=p4 sig_type=std_logic lab=vref}
C {devices/lab_pin.sym} 430 -10 3 0 {name=p5 sig_type=std_logic lab=vs}
C {sg13g2_pr/sg13_hv_nmos.sym} 520 50 2 1 {name=M5
l=0.4u
w=5.18u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 150 50 2 0 {name=M6
l=0.4u
w=5.18u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {isource.sym} 130 -70 0 0 {name=I0 value=40e-6}
