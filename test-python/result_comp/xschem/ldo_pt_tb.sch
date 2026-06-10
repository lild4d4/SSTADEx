v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 60 -200 60 -170 {
lab=#net1}
N 180 -100 180 -50 {
lab=vout}
N 180 10 180 40 {
lab=vfb}
N 180 100 180 120 {
lab=vss}
N 180 -330 180 -200 {
lab=vdd}
N -60 -200 40 -200 {
lab=#net1}
N -220 -200 -170 -200 {
lab=#net1}
N -170 -200 -140 -200 {
lab=#net1}
N -140 -200 -110 -200 {
lab=#net1}
N -220 -130 -220 -120 {
lab=vss}
N -180 -120 -180 -100 {
lab=vss}
N -220 -120 -100 -120 {
lab=vss}
N -100 -130 -100 -120 {
lab=vss}
N -100 -200 -100 -190 {
lab=#net1}
N 140 20 180 20 {
lab=vfb}
N 180 -70 350 -70 {
lab=vout}
N -220 -200 -220 -190 {
lab=#net1}
N -110 -200 -60 -200 {
lab=#net1}
N 350 -70 360 -70 {
lab=vout}
N 40 -200 60 -200 {
lab=#net1}
N 280 -360 280 -330 {
lab=vdd}
N 180 -360 280 -360 {
lab=vdd}
N 180 -360 180 -340 {
lab=vdd}
N 180 -340 180 -330 {
lab=vdd}
N 280 -320 280 -310 {
lab=vdd}
N 280 -330 280 -320 {
lab=vdd}
N -140 -180 -140 -140 {
lab=vref}
N -180 -180 -140 -180 {
lab=vref}
N -140 -140 -140 -90 {
lab=vref}
N 180 -140 180 -100 {
lab=vout}
N 60 -170 130 -170 {
lab=#net1}
N 130 -170 140 -170 {
lab=#net1}
N 180 -170 200 -170 {
lab=vdd}
N 200 -210 200 -170 {
lab=vdd}
N 180 -210 200 -210 {
lab=vdd}
C {devices/res.sym} 180 -20 0 0 {name=R2
value=100000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 180 70 0 0 {name=R3
value=300000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -220 -160 2 0 {name=Gma value=0.01}
C {devices/res.sym} -100 -160 2 0 {name=Ra
value=10000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} -180 -140 2 0 {name=p4 sig_type=std_logic lab=vfb
value=vref}
C {devices/lab_pin.sym} 140 20 0 0 {name=p5 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} -180 -100 3 0 {name=p8 sig_type=std_logic lab=vss
value=vref}
C {devices/lab_pin.sym} 180 120 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 360 -70 2 0 {name=p2 sig_type=std_logic lab=vout}
C {devices/code_shown.sym} -270 60 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/sky130A/libs.tech/ngspice/sky130.lib.spice tt

Vref vref 0 0.9
Vdd vdd 0 1.8 ac 1
Vss vss 0 0

.control
ac dec 10 1 1G
plot vdb(vout)
.endc

.control
tran 1n 10u 
plot v(vout)
.endc

"}
C {devices/lab_pin.sym} 280 -310 3 0 {name=p9 sig_type=std_logic lab=vdd
}
C {devices/lab_pin.sym} -140 -90 3 0 {name=p3 sig_type=std_logic lab=vref}
C {sky130_fd_pr/pfet_01v8.sym} 160 -170 0 0 {name=M1
W=20
L=1
nf=1
mult=10
ad="'int((nf+1)/2) * W/nf * 0.29'" 
pd="'2*int((nf+1)/2) * (W/nf + 0.29)'"
as="'int((nf+2)/2) * W/nf * 0.29'" 
ps="'2*int((nf+2)/2) * (W/nf + 0.29)'"
nrd="'0.29 / W'" nrs="'0.29 / W'"
sa=0 sb=0 sd=0
model=pfet_01v8
spiceprefix=X
}
