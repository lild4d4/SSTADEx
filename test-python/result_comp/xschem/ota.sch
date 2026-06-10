v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -70 -50 -70 -30 {
lab=#net1}
N -70 30 -70 50 {
lab=vs}
N 60 -50 60 -30 {
lab=#net1}
N -0 -50 60 -50 {
lab=#net1}
N 60 30 60 50 {
lab=vs}
N -0 50 60 50 {
lab=vs}
N 290 -50 290 -30 {
lab=vout}
N 290 30 290 50 {
lab=vs}
N 420 -50 420 -30 {
lab=vout}
N 360 -50 420 -50 {
lab=vout}
N 420 30 420 50 {
lab=vs}
N 360 50 420 50 {
lab=vs}
N -0 50 0 110 {
lab=vs}
N 180 110 360 110 {
lab=vs}
N 360 50 360 110 {
lab=vs}
N 180 110 180 160 {
lab=vs}
N 360 -230 360 -180 {
lab=vdd}
N 320 -230 360 -230 {
lab=vdd}
N 0 -100 -0 -50 {
lab=#net1}
N 360 -70 360 -50 {
lab=vout}
N 0 -100 100 -100 {
lab=#net1}
N 460 -230 460 -180 {
lab=vdd}
N 460 -120 460 -100 {
lab=vout}
N 360 -100 460 -100 {
lab=vout}
N 360 -230 460 -230 {
lab=vdd}
N 360 -70 560 -70 {
lab=vout}
N -200 -20 -200 10 {
lab=vneg}
N -200 70 -200 90 {
lab=vss}
N -200 -20 -110 -20 {
lab=vneg}
N -110 20 -110 50 {
lab=vs}
N -110 50 -70 50 {
lab=vs}
N 250 20 250 50 {
lab=vs}
N 250 50 290 50 {
lab=vs}
N 150 50 150 70 {
lab=vss}
N 150 -20 150 -10 {
lab=vpos}
N 180 220 180 250 {
lab=vss}
N 150 -20 250 -20 {
lab=vpos}
N 320 -230 320 -170 {
lab=vdd}
N 100 -100 320 -100 {
lab=#net1}
N 320 -130 320 -100 {
lab=#net1}
N -70 50 -0 50 {
lab=vs}
N 290 50 360 50 {
lab=vs}
N 0 110 180 110 {
lab=vs}
N -70 -50 -0 -50 {
lab=#net1}
N 290 -50 360 -50 {
lab=vout}
N 0 -230 100 -230 {
lab=vdd}
N 0 -120 0 -100 {
lab=#net1}
N 360 -120 360 -100 {
lab=vout}
N 360 -100 360 -70 {
lab=vout}
N 100 -230 320 -230 {
lab=vdd}
N 0 -230 0 -180 {
lab=vdd}
C {devices/res.sym} 60 0 0 0 {name=Rdif_2
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -70 0 0 0 {name=Gdif_2 value=10}
C {devices/res.sym} 420 0 0 0 {name=Rdif_1
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 290 0 0 0 {name=Gdif_1 value=10}
C {devices/isource.sym} 180 190 0 0 {name=I2 value=1}
C {devices/vsource.sym} 230 -200 0 0 {name=V1 value=1.8 savecurrent=false}
C {devices/vsource.sym} -200 40 0 0 {name=V_n value=0.9 savecurrent=false}
C {devices/lab_pin.sym} 180 110 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} -200 -20 1 0 {name=p2 sig_type=std_logic lab=vneg}
C {devices/vsource.sym} 150 20 0 0 {name=V_p value=0.9 savecurrent=false}
C {devices/lab_pin.sym} 150 -20 1 0 {name=p3 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} 560 -70 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 170 -230 1 0 {name=p5 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 230 -170 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -200 90 3 0 {name=p7 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 150 70 3 0 {name=p8 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 180 250 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/res.sym} 460 -150 0 0 {name=Raload_1
value=1000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 0 -150 0 0 {name=Raload_2
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 360 -150 2 1 {name=Gaload value=0.0002236}
