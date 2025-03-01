v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 400 -180 400 -160 {
lab=vout}
N 400 -80 400 -70 {
lab=vcp}
N 400 -130 440 -130 {
lab=vcp}
N 440 -130 440 -80 {
lab=vcp}
N 400 -80 440 -80 {
lab=vcp}
N 400 -100 400 -80 {
lab=vcp}
N 290 -130 360 -130 {
lab=vbias}
N 90 20 90 80 {
lab=vss}
N 90 -40 180 -40 {
lab=#net1}
N 400 -200 660 -200 {
lab=vout}
N 400 -270 400 -200 {
lab=vout}
N 400 10 400 80 {
lab=vss}
N 330 80 400 80 {
lab=vss}
N 330 -40 360 -40 {
lab=#net2}
N 400 -40 430 -40 {
lab=vss}
N 430 -40 430 10 {
lab=vss}
N 400 10 430 10 {
lab=vss}
N 400 -10 400 10 {
lab=vss}
N 770 -200 770 -140 {
lab=vfb}
N 770 -80 770 80 {
lab=vss}
N 720 -200 770 -200 {
lab=vfb}
N 400 80 770 80 {
lab=vss}
N 330 -40 330 -10 {
lab=#net2}
N 330 50 330 80 {
lab=vss}
N 90 80 330 80 {
lab=vss}
N 350 -80 400 -80 {
lab=vcp}
N 240 -40 330 -40 {
lab=#net2}
N 290 -130 290 -80 {
lab=vbias}
N 280 -130 290 -130 {
lab=vbias}
N 370 -180 400 -180 {
lab=vout}
N 400 -200 400 -180 {
lab=vout}
N 280 -180 310 -180 {
lab=vbias}
N 280 -180 280 -130 {
lab=vbias}
N 270 -130 280 -130 {
lab=vbias}
N 560 -180 560 -160 {
lab=vout}
N 400 -180 560 -180 {
lab=vout}
N 560 -100 560 -80 {
lab=vcp}
N 440 -80 560 -80 {
lab=vcp}
C {devices/lab_pin.sym} 600 -200 1 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 400 -330 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 400 80 3 0 {name=p9 sig_type=std_logic lab=vss}
C {sg13g2_pr/sg13_hv_nmos.sym} 380 -40 2 1 {name=M1
l=6.4u
w=5.87u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 380 -130 2 1 {name=M2
l=0.4u
w=5.11u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {devices/lab_pin.sym} 270 -130 0 0 {name=p2 sig_type=std_logic lab=vbias}
C {devices/lab_pin.sym} 440 -80 2 0 {name=p3 sig_type=std_logic lab=vcp}
C {bsource.sym} 90 -10 0 0 {name=B1 VAR=V FUNC="vcc/2*(1+tanh(10000*(v(vfb)-v(vref))))"}
C {vsource.sym} 210 -40 1 0 {name=V1 value="dc 0 ac 1" savecurrent=false}
C {res.sym} 690 -200 1 0 {name=R1
value=10000000000
footprint=1206
device=resistor
m=1}
C {capa.sym} 770 -110 0 0 {name=C2
m=1
value=100
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 770 -200 2 0 {name=p5 sig_type=std_logic lab=vfb}
C {isource.sym} 400 -300 0 0 {name=I1 value=20e-6}
C {devices/code_shown.sym} -800 -350 0 0 {name=s2 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt
.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOSlv.lib mos_tt

Vdd vdd 0 3.3
Vss vss 0 0
Vref vref 0 1.6
Vbias vbias 0 2.1

.param vcc=3.3

.control
ac dec 10 1 1G
plot vdb(vout)
plot phase(vout)*180/PI

meas ac gain find vdb(vout) at=1
let low_gain = gain-3

meas ac fc WHEN vdb(vout)=low_gain
op 
print vout
print vfb
print vg1
print vg2
let gm_diff = @n.xm1.nsg13_hv_nmos[gm]
let id_diff = @n.xm1.nsg13_hv_nmos[ids]
let gds_diff = @n.xm1.nsg13_hv_nmos[gds]
let cgg_diff = @n.xm1.nsg13_hv_nmos[cgg]

let gm_m2 = @n.xm2.nsg13_hv_nmos[gm]
let gds_m2 = @n.xm2.nsg13_hv_nmos[gds]
print gm_diff
print gds_diff
print id_diff
print cgg_diff
print gm_diff/id_diff

print gm_m2
print gds_m2

print gm_m2/gds_m2

print (gm_diff/gds_diff)*(gm_m2/gds_m2)

.endc

"}
