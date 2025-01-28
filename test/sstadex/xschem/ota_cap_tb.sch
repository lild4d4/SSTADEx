v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -250 -130 -250 -110 {
lab=#net1}
N -250 -50 -250 -30 {
lab=vs}
N -120 -130 -120 -110 {
lab=#net1}
N -180 -130 -120 -130 {
lab=#net1}
N -120 -50 -120 -30 {
lab=vs}
N -180 -30 -120 -30 {
lab=vs}
N 110 -130 110 -110 {
lab=vout}
N 110 -50 110 -30 {
lab=vs}
N 240 -130 240 -110 {
lab=vout}
N 180 -130 240 -130 {
lab=vout}
N 240 -50 240 -30 {
lab=vs}
N 180 -30 240 -30 {
lab=vs}
N -180 -30 -180 30 {
lab=vs}
N 0 30 180 30 {
lab=vs}
N 180 -30 180 30 {
lab=vs}
N 0 30 0 80 {
lab=vs}
N 180 -310 180 -260 {
lab=vdd}
N 140 -310 180 -310 {
lab=vdd}
N -180 -310 -180 -260 {
lab=vdd}
N -180 -180 -180 -130 {
lab=#net1}
N 180 -150 180 -130 {
lab=vout}
N -80 -310 -80 -260 {
lab=vdd}
N -80 -200 -80 -180 {
lab=#net1}
N -180 -180 -80 -180 {
lab=#net1}
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
N -290 -60 -290 -30 {
lab=vs}
N -290 -30 -250 -30 {
lab=vs}
N 70 -60 70 -30 {
lab=vs}
N 70 -30 110 -30 {
lab=vs}
N 0 140 0 170 {
lab=vss}
N -220 -310 -220 -250 {
lab=vdd}
N -220 -310 -180 -310 {
lab=vdd}
N 140 -310 140 -250 {
lab=vdd}
N -220 -180 -180 -180 {
lab=#net1}
N -220 -210 -220 -180 {
lab=#net1}
N -250 -30 -180 -30 {
lab=vs}
N 110 -30 180 -30 {
lab=vs}
N -180 30 0 30 {
lab=vs}
N -250 -130 -180 -130 {
lab=#net1}
N 110 -130 180 -130 {
lab=vout}
N -180 -310 -80 -310 {
lab=vdd}
N -180 -200 -180 -180 {
lab=#net1}
N 180 -200 180 -180 {
lab=vout}
N 180 -180 180 -150 {
lab=vout}
N -80 -310 140 -310 {
lab=vdd}
N -10 -130 10 -130 {
lab=vp}
N 70 -130 110 -130 {
lab=vout}
N 0 -130 0 -110 {
lab=vp}
N -0 -50 0 -30 {
lab=vs}
N 0 -30 70 -30 {
lab=vs}
N -370 -130 -350 -130 {
lab=vn}
N -360 -130 -360 -110 {
lab=vn}
N -360 -50 -360 -30 {
lab=vs}
N -290 -130 -250 -130 {
lab=#net1}
N -360 -30 -290 -30 {
lab=vs}
N 320 -130 320 -110 {
lab=vout}
N 240 -130 320 -130 {
lab=vout}
N 320 -50 320 -30 {
lab=vs}
N 240 -30 320 -30 {
lab=vs}
N -70 -130 -70 -110 {
lab=#net1}
N -120 -130 -70 -130 {
lab=#net1}
N -70 -50 -70 -30 {
lab=vs}
N -120 -30 -70 -30 {
lab=vs}
N 130 -180 140 -180 {
lab=vout}
N -80 -180 70 -180 {
lab=#net1}
N 60 -310 60 -290 {
lab=vdd}
N 60 -230 60 -180 {
lab=#net1}
N 140 -180 180 -180 {
lab=vout}
N 30 -210 30 -180 {
lab=#net1}
N 30 -210 140 -210 {
lab=#net1}
N 380 -310 380 -270 {
lab=vdd}
N 280 -310 380 -310 {
lab=vdd}
N 380 -210 380 -180 {
lab=vout}
N 280 -180 380 -180 {
lab=vout}
N -40 -130 -40 -110 {
lab=vp}
N -40 -130 -10 -130 {
lab=vp}
N -40 -50 -40 -30 {
lab=vs}
N -40 -30 0 -30 {
lab=vs}
N 400 -130 400 -110 {
lab=vout}
N 320 -130 400 -130 {
lab=vout}
N 400 -40 400 -30 {
lab=vs}
N 320 -30 400 -30 {
lab=vs}
N 400 -50 400 -40 {
lab=vs}
N 500 -310 500 -270 {
lab=vdd}
N 380 -310 500 -310 {
lab=vdd}
N 500 -210 500 -180 {
lab=vout}
N 380 -180 500 -180 {
lab=vout}
N -460 -130 -460 -110 {
lab=vn}
N -460 -130 -370 -130 {
lab=vn}
N -460 -50 -460 -30 {
lab=vs}
N -460 -30 -360 -30 {
lab=vs}
C {devices/res.sym} -120 -80 0 0 {name=Rdif_2
value=481556.4
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -250 -80 0 0 {name=Gdif_2 value=0.000405}
C {devices/res.sym} 240 -80 0 0 {name=Rdif_1
value=481556.4
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 110 -80 0 0 {name=Gdif_1 value=0.000405}
C {devices/isource.sym} 0 110 0 0 {name=I2 value=0}
C {devices/lab_pin.sym} 0 30 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} -370 -130 0 0 {name=p2 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} -10 -130 0 0 {name=p3 sig_type=std_logic lab=vp}
C {devices/lab_pin.sym} 380 -150 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} -10 -310 1 0 {name=p5 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 0 170 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/res.sym} 280 -230 0 0 {name=Raload_1
value=1851851
footprint=1206
device=resistor
m=1}
C {devices/res.sym} -80 -230 0 0 {name=Raload_2
value=4476
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 180 -230 2 1 {name=Gaload value=0.0002234}
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
C {devices/lab_pin.sym} -290 -100 0 0 {name=p7 sig_type=std_logic lab=vn}
C {capa.sym} 320 -80 0 0 {name=C5
m=1
value=6.6e-13
footprint=1206
device="ceramic capacitor"}
