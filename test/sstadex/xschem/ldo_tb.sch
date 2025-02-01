v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 70 -310 70 -290 {
lab=vdd}
N 70 -230 70 -210 {
lab=vout}
N 200 -310 200 -300 {
lab=vdd}
N 200 -300 200 -290 {
lab=vdd}
N 200 -230 200 -210 {
lab=vout}
N 70 -210 200 -210 {
lab=vout}
N 70 -140 70 -90 {
lab=vout}
N 70 -30 70 0 {
lab=vfb}
N 70 60 70 80 {
lab=vss}
N 30 -20 70 -20 {
lab=vfb}
N 70 80 210 80 {
lab=vss}
N 70 -310 200 -310 {
lab=vdd}
N 80 -140 210 -140 {
lab=vout}
N 70 -140 80 -140 {
lab=vout}
N 70 -210 70 -140 {
lab=vout}
N 210 -10 210 80 {
lab=vss}
N 210 -140 210 -70 {
lab=vout}
N -70 -240 30 -240 {
lab=#net1}
N -230 -240 -180 -240 {
lab=#net1}
N -180 -240 -150 -240 {
lab=#net1}
N -150 -240 -120 -240 {
lab=#net1}
N -230 -170 -230 -160 {
lab=vdd}
N -230 -160 -230 -140 {
lab=vdd}
N -230 -160 -110 -160 {
lab=vdd}
N -110 -170 -110 -160 {
lab=vdd}
N -110 -240 -110 -230 {
lab=#net1}
N -230 -240 -230 -230 {
lab=#net1}
N -120 -240 -70 -240 {
lab=#net1}
N 30 -310 30 -280 {
lab=vdd}
N 30 -310 70 -310 {
lab=vdd}
N 130 -370 130 -310 {
lab=vdd}
N -190 -220 -130 -220 {
lab=vss}
N -290 -160 -230 -160 {
lab=vdd}
N -290 -290 -290 -160 {
lab=vdd}
N -290 -290 30 -290 {
lab=vdd}
C {devices/res.sym} 200 -260 0 0 {name=Ro_pt
value=22
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 70 -260 2 1 {name=Gm_pt value=0.503}
C {devices/res.sym} 70 -60 0 0 {name=R1
value=100000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 70 30 0 0 {name=R2
value=300000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 70 80 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 210 -140 2 0 {name=p2 sig_type=std_logic lab=vout}
C {devices/code_shown.sym} -420 180 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/sky130A/libs.tech/ngspice/sky130.lib.spice tt

Vdd vdd 0 0 ac 1
Vss vss 0 0

.control
ac dec 10 1 10G
plot vdb(vout)
.endc

"}
C {devices/res.sym} 210 -40 0 0 {name=R3
value=12
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -230 -200 2 0 {name=Gma value=0.00046415}
C {devices/res.sym} -110 -200 2 0 {name=Ra
value=1291549
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 130 -370 1 0 {name=p15 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} -130 -220 3 0 {name=p16 sig_type=std_logic lab=vss
value=vref}
C {devices/lab_pin.sym} 30 -20 0 0 {name=p1 sig_type=std_logic lab=vfb
value=vfb}
C {devices/lab_pin.sym} -190 -180 2 0 {name=p3 sig_type=std_logic lab=vfb
value=vfb}
