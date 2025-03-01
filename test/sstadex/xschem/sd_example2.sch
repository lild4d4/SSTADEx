v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 350 -20 400 -20 {
lab=vss}
N 400 -20 400 40 {
lab=vss}
N 350 40 400 40 {
lab=vss}
N 350 10 350 40 {
lab=vss}
N 350 -90 350 -50 {
lab=vout}
N 500 -90 500 -30 {
lab=vout}
N 350 130 500 130 {
lab=vss}
N 500 30 500 130 {
lab=vss}
N 350 -130 350 -90 {
lab=vout}
N 350 -210 350 -190 {
lab=vss}
N 350 -90 500 -90 {
lab=vout}
N 350 100 350 130 {
lab=vss}
N 30 40 30 100 {
lab=vss}
N 30 100 350 100 {
lab=vss}
N 350 40 350 100 {
lab=vss}
N 670 -90 670 -30 {
lab=vfb}
N 180 -20 310 -20 {
lab=vg1}
N 30 -20 120 -20 {
lab=#net1}
N 670 30 670 130 {
lab=vss}
N 500 130 670 130 {
lab=vss}
N 500 -90 560 -90 {
lab=vout}
N 620 -90 670 -90 {
lab=vfb}
C {sg13g2_pr/sg13_hv_nmos.sym} 330 -20 2 1 {name=M1
l=0.8u
w=68.09u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {devices/lab_pin.sym} 500 -90 1 0 {name=p1 sig_type=std_logic lab=vout}
C {capa.sym} 500 0 0 0 {name=C1
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/code_shown.sym} -820 -330 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt
.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOSlv.lib mos_tt

Vdd vdd 0 3.3
Vss vss 0 0
Vref vref 0 1.2

.param vcc=3.3

.control
ac dec 10 1 1G
plot vdb(vout)
wrdata sd_example2.csv vdb(vout)
op 
print vout
print vfb
print vg1
print vg2
let gm_diff = @n.xm1.nsg13_hv_nmos[gm]
let id_diff = @n.xm1.nsg13_hv_nmos[ids]
let gds_diff = @n.xm1.nsg13_hv_nmos[gds]
print gm_diff
print gds_diff
print id_diff
print gm_diff/id_diff
.endc

"}
C {devices/lab_pin.sym} 350 130 3 0 {name=p2 sig_type=std_logic lab=vss}
C {isource.sym} 350 -160 0 0 {name=I1 value=2.31e-6}
C {devices/lab_pin.sym} 350 -210 1 0 {name=p4 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 270 -20 3 0 {name=p3 sig_type=std_logic lab=vg1}
C {bsource.sym} 30 10 0 0 {name=B1 VAR=V FUNC="vcc/2*(1+tanh(1000*(v(vfb)-v(vref))))"}
C {vsource.sym} 150 -20 1 0 {name=V1 value="dc 0 ac 1" savecurrent=false}
C {res.sym} 590 -90 1 0 {name=R1
value=100000000
footprint=1206
device=resistor
m=1}
C {capa.sym} 670 0 0 0 {name=C2
m=1
value=10
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 670 -90 2 0 {name=p5 sig_type=std_logic lab=vfb}
