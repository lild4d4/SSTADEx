v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 340 -160 390 -160 {
lab=vss}
N 390 -160 390 -100 {
lab=vss}
N 340 -100 390 -100 {
lab=vss}
N 340 -130 340 -100 {
lab=vss}
N 340 -230 340 -190 {
lab=vout}
N 490 -230 490 -170 {
lab=vout}
N 340 -10 490 -10 {
lab=vss}
N 490 -110 490 -10 {
lab=vss}
N 340 -270 340 -230 {
lab=vout}
N 340 -350 340 -330 {
lab=vss}
N 340 -230 490 -230 {
lab=vout}
N 340 -40 340 -10 {
lab=vss}
N 20 -100 20 -40 {
lab=vss}
N 20 -40 340 -40 {
lab=vss}
N 340 -100 340 -40 {
lab=vss}
N 660 -230 660 -170 {
lab=vfb}
N 170 -160 300 -160 {
lab=vg1}
N 20 -160 110 -160 {
lab=#net1}
N 660 -110 660 -10 {
lab=vss}
N 490 -10 660 -10 {
lab=vss}
N 490 -230 550 -230 {
lab=vout}
N 610 -230 660 -230 {
lab=vfb}
C {sg13g2_pr/sg13_hv_nmos.sym} 320 -160 2 1 {name=M1
l=0.8u
w=60.84u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {devices/lab_pin.sym} 490 -230 1 0 {name=p1 sig_type=std_logic lab=vout}
C {capa.sym} 490 -140 0 0 {name=C1
m=1
value=1p
footprint=1206
device="ceramic capacitor"}
C {devices/code_shown.sym} -830 -470 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt
.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOSlv.lib mos_tt

Vdd vdd 0 3.3
Vss vss 0 0
Vref vref 0 1.2

.param vcc=3.3

.control
ac dec 1000 1 1G
plot vdb(vout)
meas ac gain find vdb(vout) at=1
let low_gain = gain-3
meas ac fc WHEN vdb(vout)=0
wrdata sd_example4.csv vdb(vout)
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
C {devices/lab_pin.sym} 340 -10 3 0 {name=p2 sig_type=std_logic lab=vss}
C {isource.sym} 340 -300 0 0 {name=I1 value=20e-6}
C {devices/lab_pin.sym} 340 -350 1 0 {name=p4 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 260 -160 3 0 {name=p3 sig_type=std_logic lab=vg1}
C {bsource.sym} 20 -130 0 0 {name=B1 VAR=V FUNC="vcc/2*(1+tanh(1000*(v(vfb)-v(vref))))"}
C {vsource.sym} 140 -160 1 0 {name=V1 value="dc 0 ac 1" savecurrent=false}
C {res.sym} 580 -230 1 0 {name=R1
value=100000000000
footprint=1206
device=resistor
m=1}
C {capa.sym} 660 -140 0 0 {name=C2
m=1
value=10
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 660 -230 2 0 {name=p5 sig_type=std_logic lab=vfb}
