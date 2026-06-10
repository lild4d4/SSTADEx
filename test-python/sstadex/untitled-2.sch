v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 40 -50 40 -30 {
lab=vdd}
N 40 -60 40 -50 {
lab=vdd}
N 40 30 40 70 {
lab=vout}
N -30 0 0 0 {
lab=vg}
N 40 -0 90 -0 {
lab=vdd}
N 90 -40 90 -0 {
lab=vdd}
N 40 -40 90 -40 {
lab=vdd}
N 40 140 40 160 {
lab=vss}
N 40 130 40 140 {
lab=vss}
N 40 50 70 50 {
lab=vout}
C {sky130_fd_pr/pfet_01v8_lvt.sym} 20 0 0 0 {name=M5
W=10.86
L=1.6
nf=1
mult=1
ad="'int((nf+1)/2) * W/nf * 0.29'" 
pd="'2*int((nf+1)/2) * (W/nf + 0.29)'"
as="'int((nf+2)/2) * W/nf * 0.29'" 
ps="'2*int((nf+2)/2) * (W/nf + 0.29)'"
nrd="'0.29 / W'" nrs="'0.29 / W'"
sa=0 sb=0 sd=0
model=pfet_01v8_lvt
spiceprefix=X
}
C {devices/code_shown.sym} -710 -150 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/sky130A/libs.tech/ngspice/sky130.lib.spice tt

Vdd vdd 0 1.8
Vgs vg 0 1
Vss vss 0 0

.control
op 
print vout
.endc

"}
C {devices/lab_pin.sym} 40 -60 1 0 {name=p1 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} -30 0 0 0 {name=p3 sig_type=std_logic lab=vg}
C {devices/lab_pin.sym} 40 160 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 70 50 2 0 {name=p2 sig_type=std_logic lab=vout}
C {devices/res.sym} 40 100 0 0 {name=Ro_pt
value=55k
footprint=1206
device=resistor
m=1}
