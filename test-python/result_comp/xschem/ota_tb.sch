v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -230 -20 -230 0 {
lab=#net1}
N -230 60 -230 80 {
lab=vs}
N -100 -20 -100 0 {
lab=#net1}
N -160 -20 -100 -20 {
lab=#net1}
N -100 60 -100 80 {
lab=vs}
N -160 80 -100 80 {
lab=vs}
N 130 -20 130 0 {
lab=vout}
N 130 60 130 80 {
lab=vs}
N 260 -20 260 0 {
lab=vout}
N 200 -20 260 -20 {
lab=vout}
N 260 60 260 80 {
lab=vs}
N 200 80 260 80 {
lab=vs}
N -160 80 -160 140 {
lab=vs}
N 20 140 200 140 {
lab=vs}
N 200 80 200 140 {
lab=vs}
N 20 140 20 190 {
lab=vs}
N 160 -200 200 -200 {
lab=vdd}
N -160 -70 -160 -20 {
lab=#net1}
N 200 -40 200 -20 {
lab=vout}
N 200 -40 400 -40 {
lab=vout}
N -360 10 -360 40 {
lab=vn}
N -360 100 -360 120 {
lab=vn}
N -360 10 -270 10 {
lab=vn}
N -270 50 -270 80 {
lab=vs}
N -270 80 -230 80 {
lab=vs}
N 90 50 90 80 {
lab=vs}
N 90 80 130 80 {
lab=vs}
N -10 80 -10 100 {
lab=vp}
N -10 10 -10 20 {
lab=vp}
N 20 250 20 280 {
lab=vss}
N -10 10 90 10 {
lab=vp}
N -230 80 -160 80 {
lab=vs}
N 130 80 200 80 {
lab=vs}
N -160 140 20 140 {
lab=vs}
N -230 -20 -160 -20 {
lab=#net1}
N 130 -20 200 -20 {
lab=vout}
N -160 -200 -60 -200 {
lab=vdd}
N 200 -70 200 -40 {
lab=vout}
N -60 -200 160 -200 {
lab=vdd}
N -360 40 -360 100 {
lab=vn}
N -10 20 -10 80 {
lab=vp}
N 200 -100 200 -70 {
lab=vout}
N -160 -110 -160 -70 {
lab=#net1}
N -160 -200 -160 -170 {
lab=vdd}
N -160 -90 -20 -90 {
lab=#net1}
N 200 -110 200 -100 {
lab=vout}
N 200 -200 200 -170 {
lab=vdd}
N 70 -200 70 -170 {
lab=vdd}
N 70 -110 70 -90 {
lab=vout}
N 30 -200 30 -160 {
lab=vdd}
N 70 -90 200 -90 {
lab=vout}
N -20 -90 30 -90 {
lab=#net1}
N 30 -120 30 -90 {
lab=#net1}
C {devices/res.sym} -100 30 0 0 {name=Rdif_2
value=1747537
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -230 30 0 0 {name=Gdif_2 value=0.000084}
C {devices/res.sym} 260 30 0 0 {name=Rdif_1
value=1747537
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 130 30 0 0 {name=Gdif_1 value=0.000084}
C {devices/isource.sym} 20 220 0 0 {name=I2 value=40e-6}
C {devices/lab_pin.sym} 20 140 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 400 -40 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 0 -200 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} -360 120 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} -10 100 3 0 {name=p8 sig_type=std_logic lab=vp}
C {devices/lab_pin.sym} 20 280 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/code_shown.sym} -350 260 0 0 {name=s1 only_toplevel=false value="

Vref vn 0 0
Vdd vdd 0 0
Vpos vp 0 0 ac 1
Vss vss 0 0

.control

ac dec 10 1 1G
plot vdb(vout)


.endc

"}
C {devices/res.sym} 200 -140 2 0 {name=Raload_3
value=100000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} -160 -140 2 0 {name=Raload_1
value=100000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 70 -140 2 1 {name=Gdif_3 value=0.001}
