v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -280 -380 -280 -360 {
lab=vdd}
N -280 -300 -280 -280 {
lab=vout}
N -150 -380 -150 -370 {
lab=vdd}
N -280 -380 -150 -380 {
lab=vdd}
N -150 -370 -150 -360 {
lab=vdd}
N -150 -300 -150 -280 {
lab=vout}
N -280 -280 -150 -280 {
lab=vout}
N -10 -380 -10 -330 {
lab=vout}
N -10 -270 -10 -240 {
lab=vfb}
N -10 -180 -10 -160 {
lab=vss}
N -50 -260 -10 -260 {
lab=vfb}
N -560 -380 -560 -360 {
lab=vdd}
N -560 -300 -560 -280 {
lab=#net1}
N -430 -380 -430 -370 {
lab=vdd}
N -560 -380 -430 -380 {
lab=vdd}
N -430 -370 -430 -360 {
lab=vdd}
N -430 -300 -430 -280 {
lab=#net1}
N -560 -280 -430 -280 {
lab=#net1}
N -320 -310 -320 -280 {
lab=#net1}
N -320 -380 -320 -350 {
lab=vdd}
N -90 -380 -10 -380 {
lab=vout}
N -150 -280 -90 -280 {
lab=vout}
N -90 -380 -90 -280 {
lab=vout}
N -430 -280 -320 -280 {
lab=#net1}
N -430 -380 -320 -380 {
lab=vdd}
N -10 -380 130 -380 {
lab=vout}
N 130 -320 130 -160 {
lab=vss}
N -10 -160 130 -160 {
lab=vss}
N -220 -400 -220 -380 {
lab=vdd}
N -320 -380 -280 -380 {
lab=vdd}
C {devices/code_shown.sym} -620 60 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/sky130A/libs.tech/ngspice/sky130.lib.spice tt

Vdd vdd 0 0 ac 1
Vss vss 0 0

.control
ac dec 10 1 10G
plot vdb(vout)
.endc

"}
C {devices/res.sym} -150 -330 0 0 {name=Ro_pt
value=21.93
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -280 -330 2 1 {name=Gm_pt value=0.509}
C {devices/res.sym} -10 -300 0 0 {name=R1
value=100000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} -10 -210 0 0 {name=R2
value=300000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} -50 -260 0 0 {name=p5 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} -10 -160 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -10 -380 2 0 {name=p2 sig_type=std_logic lab=vout}
C {devices/res.sym} -430 -330 0 0 {name=Ro_pt1
value=8403361
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} -220 -400 1 0 {name=p3 sig_type=std_logic lab=vdd}
C {devices/res.sym} 130 -350 0 0 {name=R3
value=12
footprint=1206
device=resistor
m=1}
