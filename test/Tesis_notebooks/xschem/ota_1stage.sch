v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -230 -100 -230 -80 {
lab=#net1}
N -230 -20 -230 0 {
lab=vs}
N -100 -100 -100 -80 {
lab=#net1}
N -160 -100 -100 -100 {
lab=#net1}
N -100 -20 -100 0 {
lab=vs}
N -160 0 -100 0 {
lab=vs}
N 130 -100 130 -80 {
lab=vout}
N 130 -20 130 0 {
lab=vs}
N 260 -100 260 -80 {
lab=vout}
N 200 -100 260 -100 {
lab=vout}
N 260 -20 260 0 {
lab=vs}
N 200 0 260 0 {
lab=vs}
N -160 0 -160 60 {
lab=vs}
N 20 60 200 60 {
lab=vs}
N 200 0 200 60 {
lab=vs}
N 20 60 20 130 {
lab=vs}
N 200 -280 200 -230 {
lab=vdd}
N 160 -280 200 -280 {
lab=vdd}
N -160 -280 -160 -230 {
lab=vdd}
N -160 -150 -160 -100 {
lab=#net1}
N 200 -120 200 -100 {
lab=vout}
N -60 -280 -60 -230 {
lab=vdd}
N -60 -170 -60 -150 {
lab=#net1}
N -160 -150 -60 -150 {
lab=#net1}
N 300 -280 300 -230 {
lab=vdd}
N 300 -170 300 -150 {
lab=vout}
N 200 -150 300 -150 {
lab=vout}
N 200 -280 300 -280 {
lab=vdd}
N 200 -120 400 -120 {
lab=vout}
N -360 -70 -360 -40 {
lab=vneg}
N -360 20 -360 40 {
lab=vss}
N -360 -70 -270 -70 {
lab=vneg}
N -270 -30 -270 0 {
lab=vs}
N -270 0 -230 0 {
lab=vs}
N 90 -30 90 0 {
lab=vs}
N 90 0 130 0 {
lab=vs}
N -10 0 -10 20 {
lab=vss}
N -10 -70 -10 -60 {
lab=vpos}
N -10 -70 90 -70 {
lab=vpos}
N -200 -280 -200 -220 {
lab=vdd}
N -200 -280 -160 -280 {
lab=vdd}
N 160 -280 160 -220 {
lab=vdd}
N -200 -150 -160 -150 {
lab=#net1}
N -200 -180 -200 -150 {
lab=#net1}
N -60 -150 160 -150 {
lab=#net1}
N 160 -180 160 -150 {
lab=#net1}
N -230 0 -160 0 {
lab=vs}
N 130 0 200 0 {
lab=vs}
N -160 60 20 60 {
lab=vs}
N -230 -100 -160 -100 {
lab=#net1}
N 130 -100 200 -100 {
lab=vout}
N -160 -280 -60 -280 {
lab=vdd}
N -160 -170 -160 -150 {
lab=#net1}
N 200 -170 200 -150 {
lab=vout}
N 200 -150 200 -120 {
lab=vout}
N -60 -280 160 -280 {
lab=vdd}
N -50 130 -50 150 {
lab=vs}
N -50 210 -50 230 {
lab=vss}
N 80 130 80 150 {
lab=vs}
N 20 130 80 130 {
lab=vs}
N 80 210 80 230 {
lab=vss}
N -90 200 -90 230 {
lab=vss}
N -90 230 -50 230 {
lab=vss}
N 20 230 80 230 {
lab=vss}
N -50 130 20 130 {
lab=vs}
N 20 230 20 270 {
lab=vss}
N -50 230 20 230 {
lab=vss}
N -120 160 -90 160 {
lab=vss}
C {devices/res.sym} -100 -50 0 0 {name=Rdiff_2
value=1
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -230 -50 0 0 {name=Gdiff_2 value=1}
C {devices/res.sym} 260 -50 0 0 {name=Rdiff_1
value=1
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 130 -50 0 0 {name=Gdiff_1 value=1}
C {devices/vsource.sym} 70 -250 0 0 {name=V1 value=1.8 savecurrent=false}
C {devices/vsource.sym} -360 -10 0 0 {name=V_n value=1 savecurrent=false}
C {devices/lab_pin.sym} 20 60 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} -360 -70 1 0 {name=p2 sig_type=std_logic lab=vneg}
C {devices/vsource.sym} -10 -30 0 0 {name=V_p value=1 savecurrent=false}
C {devices/lab_pin.sym} -10 -70 1 0 {name=p3 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} 400 -120 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 10 -280 1 0 {name=p5 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 70 -220 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -360 40 3 0 {name=p7 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -10 20 3 0 {name=p8 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 20 270 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/res.sym} 300 -200 0 0 {name=Raload_1
value=1
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 200 -200 2 1 {name=Gaload_1 value=1}
C {devices/vccs.sym} -160 -200 2 1 {name=Gaload_2 value=1}
C {devices/res.sym} -60 -200 0 0 {name=Raload_2
value=1
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 80 180 0 0 {name=Rcs
value=1
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -50 180 0 0 {name=Gcs value=1}
C {devices/lab_pin.sym} -120 160 0 0 {name=p10 sig_type=std_logic lab=vss}
