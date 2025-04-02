v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 110 -130 110 -110 {
lab=vout}
N 110 -50 110 -30 {
lab=vss}
N 240 -130 240 -110 {
lab=vout}
N 180 -130 240 -130 {
lab=vout}
N 240 -50 240 -30 {
lab=vss}
N 180 -30 240 -30 {
lab=vss}
N 180 -30 180 30 {
lab=vss}
N 180 -310 180 -260 {
lab=vdd}
N 140 -310 180 -310 {
lab=vdd}
N 180 -150 180 -130 {
lab=vout}
N 280 -310 280 -260 {
lab=vdd}
N 280 -200 280 -180 {
lab=vout}
N 180 -180 280 -180 {
lab=vout}
N 180 -310 280 -310 {
lab=vdd}
N 180 -150 380 -150 {
lab=vout}
N 70 -30 110 -30 {
lab=vss}
N 140 -310 140 -250 {
lab=vdd}
N 110 -30 180 -30 {
lab=vss}
N 110 -130 180 -130 {
lab=vout}
N 180 -200 180 -180 {
lab=vout}
N 180 -180 180 -150 {
lab=vout}
N 60 -310 140 -310 {
lab=vdd}
N 130 -180 180 -180 {
lab=vout}
N 60 -180 70 -180 {
lab=vss}
N 60 -310 60 -290 {
lab=vdd}
N 60 -230 60 -180 {
lab=vss}
N 30 -210 30 -180 {
lab=vss}
N 30 -210 140 -210 {
lab=vss}
N -80 -310 60 -310 {
lab=vdd}
N 30 -180 60 -180 {
lab=vss}
N -80 -180 30 -180 {
lab=vss}
N 70 -60 70 -30 {
lab=vss}
C {devices/res.sym} 240 -80 0 0 {name=Rdif_1
value=530644
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 110 -80 0 0 {name=Gdif_1 value=0.0001557}
C {devices/lab_pin.sym} 380 -150 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} -10 -310 1 0 {name=p5 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 180 30 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/res.sym} 280 -230 0 0 {name=Raload_1
value=23640662
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 180 -230 2 1 {name=Gaload value=0.00003668}
C {devices/code_shown.sym} -410 270 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/sky130A/libs.tech/ngspice/sky130.lib.spice tt

Vref vn 0 0.9
Vdd vdd 0 0
Vpos vp 0 0.9 ac 1
Vss vss 0 0

.control
ac dec 10 1 1G
plot vdb(vout) 
.endc

.control
tran 10n 1u
plot i(V1)
.endc

"}
C {devices/lab_pin.sym} 70 -100 0 0 {name=p6 sig_type=std_logic lab=vp}
C {devices/lab_pin.sym} -80 -180 3 0 {name=p2 sig_type=std_logic lab=vss}
