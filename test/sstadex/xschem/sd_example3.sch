v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 460 -30 510 -30 {
lab=vss}
N 510 -30 510 30 {
lab=vss}
N 460 30 510 30 {
lab=vss}
N 460 0 460 30 {
lab=vss}
N 460 -100 460 -60 {
lab=vout}
N 610 -100 610 -40 {
lab=vout}
N 460 120 610 120 {
lab=vss}
N 610 20 610 120 {
lab=vss}
N 460 -140 460 -100 {
lab=vout}
N 460 -220 460 -200 {
lab=vss}
N 460 -100 610 -100 {
lab=vout}
N 460 90 460 120 {
lab=vss}
N 140 30 140 90 {
lab=vss}
N 140 90 460 90 {
lab=vss}
N 460 30 460 90 {
lab=vss}
N 780 -100 780 -40 {
lab=vfb}
N 290 -30 420 -30 {
lab=vg1}
N 140 -30 230 -30 {
lab=#net1}
N 780 20 780 120 {
lab=vss}
N 610 120 780 120 {
lab=vss}
N 610 -100 670 -100 {
lab=vout}
N 730 -100 780 -100 {
lab=vfb}
C {sg13g2_pr/sg13_hv_nmos.sym} 440 -30 2 1 {name=M1
l=3.2u
w=10.91u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {devices/lab_pin.sym} 610 -100 1 0 {name=p1 sig_type=std_logic lab=vout}
C {capa.sym} 610 -10 0 0 {name=C1
m=1
value=1p
footprint=1206
device="ceramic capacitor"}
C {devices/code_shown.sym} -710 -340 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt
.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOSlv.lib mos_tt

Vdd vdd 0 3.3
Vss vss 0 0
Vref vref 0 1.2

.param vcc=3.3

.control
ac dec 10 1 1G
plot vdb(vout)
wrdata sd_example3.csv vdb(vout)
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
C {devices/lab_pin.sym} 460 120 3 0 {name=p2 sig_type=std_logic lab=vss}
C {isource.sym} 460 -170 0 0 {name=I1 value=8.2e-6}
C {devices/lab_pin.sym} 460 -220 1 0 {name=p4 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 380 -30 3 0 {name=p3 sig_type=std_logic lab=vg1}
C {bsource.sym} 140 0 0 0 {name=B1 VAR=V FUNC="vcc/2*(1+tanh(1000*(v(vfb)-v(vref))))"}
C {vsource.sym} 260 -30 1 0 {name=V1 value="dc 0 ac 1" savecurrent=false}
C {res.sym} 700 -100 1 0 {name=R1
value=100000000
footprint=1206
device=resistor
m=1}
C {capa.sym} 780 -10 0 0 {name=C2
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 780 -100 2 0 {name=p5 sig_type=std_logic lab=vfb}
