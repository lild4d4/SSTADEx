v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 180 -140 180 -120 {
lab=vdd}
N 180 -60 180 -40 {
lab=vout}
N 310 -140 310 -130 {
lab=vdd}
N 180 -140 310 -140 {
lab=vdd}
N 310 -130 310 -120 {
lab=vdd}
N 310 -60 310 -40 {
lab=vout}
N 180 -40 310 -40 {
lab=vout}
N 150 -140 180 -140 {
lab=vdd}
N 140 -140 150 -140 {
lab=vdd}
N 60 -140 60 -110 {
lab=#net1}
N 180 -40 180 10 {
lab=vout}
N 180 90 180 110 {
lab=vss}
N 180 -270 180 -140 {
lab=vdd}
N -60 -140 40 -140 {
lab=#net1}
N -220 -140 -170 -140 {
lab=#net1}
N -170 -140 -140 -140 {
lab=#net1}
N -140 -140 -110 -140 {
lab=#net1}
N -220 -70 -220 -60 {
lab=vdd}
N -220 -60 -220 -40 {
lab=vdd}
N -220 -60 -100 -60 {
lab=vdd}
N -100 -70 -100 -60 {
lab=vdd}
N -100 -140 -100 -130 {
lab=#net1}
N 180 -10 350 -10 {
lab=vout}
N -220 -140 -220 -130 {
lab=#net1}
N -110 -140 -60 -140 {
lab=#net1}
N 350 -10 360 -10 {
lab=vout}
N 60 -70 140 -70 {
lab=#net1}
N 60 -110 60 -70 {
lab=#net1}
N 140 -140 140 -110 {
lab=vdd}
N 40 -140 60 -140 {
lab=#net1}
N 280 -300 280 -270 {
lab=vdd}
N 180 -300 280 -300 {
lab=vdd}
N 180 -300 180 -280 {
lab=vdd}
N 180 -280 180 -270 {
lab=vdd}
N 280 -260 280 -250 {
lab=vdd}
N 280 -270 280 -260 {
lab=vdd}
N -180 -120 -140 -120 {
lab=vdd}
N 180 10 180 20 {
lab=vout}
N 180 80 180 90 {
lab=vss}
N -20 -140 -20 -130 {
lab=#net1}
N -20 -130 -20 -120 {
lab=#net1}
N -70 -60 -20 -60 {
lab=vdd}
N -100 -60 -70 -60 {
lab=vdd}
N 90 -10 180 -10 {
lab=vout}
N -340 -60 -220 -60 {
lab=vdd}
N -340 -200 -340 -60 {
lab=vdd}
N -340 -200 180 -200 {
lab=vdd}
N -140 -120 -140 -60 {
lab=vdd}
C {devices/res.sym} 310 -90 0 0 {name=Ro_pt
value=765253
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 180 -90 2 1 {name=Gm_pt value=0.0001244}
C {devices/vccs.sym} -220 -100 2 0 {name=Gma value=0.0003086}
C {devices/res.sym} -100 -100 2 0 {name=Ra
value=139196
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} -180 -80 2 0 {name=p4 sig_type=std_logic lab=vp}
C {devices/lab_pin.sym} 180 110 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/isource.sym} 180 50 0 0 {name=I2 value=20e-6}
C {devices/code_shown.sym} -330 270 0 0 {name=s1 only_toplevel=false value="

Vref vn 0 0
Vdd vdd 0 0
Vpos vp 0 0 ac 1
Vss vss 0 0

.control

ac dec 10 1 1G
plot vdb(vout)


.endc

"}
C {devices/lab_pin.sym} 280 -250 3 0 {name=p1 sig_type=std_logic lab=vdd}
C {capa.sym} 90 -40 0 0 {name=C2
m=1
value=1.48908e-12
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 360 -10 2 0 {name=p2 sig_type=std_logic lab=vout}
