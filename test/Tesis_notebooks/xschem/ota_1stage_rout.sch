v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -280 -50 -280 -30 {
lab=#net1}
N -280 30 -280 50 {
lab=vs}
N -150 -50 -150 -30 {
lab=#net1}
N -210 -50 -150 -50 {
lab=#net1}
N -150 30 -150 50 {
lab=vs}
N -210 50 -150 50 {
lab=vs}
N 80 -50 80 -30 {
lab=vout}
N 80 30 80 50 {
lab=vs}
N 210 -50 210 -30 {
lab=vout}
N 150 -50 210 -50 {
lab=vout}
N 210 30 210 50 {
lab=vs}
N 150 50 210 50 {
lab=vs}
N -210 50 -210 110 {
lab=vs}
N -30 110 150 110 {
lab=vs}
N 150 50 150 110 {
lab=vs}
N -30 110 -30 160 {
lab=vs}
N 150 -230 150 -180 {
lab=vdd}
N 110 -230 150 -230 {
lab=vdd}
N -210 -230 -210 -180 {
lab=vdd}
N -210 -100 -210 -50 {
lab=#net1}
N 150 -70 150 -50 {
lab=vout}
N -110 -230 -110 -180 {
lab=vdd}
N -110 -120 -110 -100 {
lab=#net1}
N -210 -100 -110 -100 {
lab=#net1}
N 250 -230 250 -180 {
lab=vdd}
N 250 -120 250 -100 {
lab=vout}
N 150 -100 250 -100 {
lab=vout}
N 150 -230 250 -230 {
lab=vdd}
N 150 -70 350 -70 {
lab=vout}
N -410 -20 -410 10 {
lab=vneg}
N -410 70 -410 90 {
lab=vss}
N -410 -20 -320 -20 {
lab=vneg}
N -320 20 -320 50 {
lab=vs}
N -320 50 -280 50 {
lab=vs}
N 40 20 40 50 {
lab=vs}
N 40 50 80 50 {
lab=vs}
N -60 50 -60 70 {
lab=vss}
N -60 -20 -60 -10 {
lab=vpos}
N -30 240 -30 250 {
lab=vss}
N -60 -20 40 -20 {
lab=vpos}
N -250 -230 -250 -170 {
lab=vdd}
N -250 -230 -210 -230 {
lab=vdd}
N 110 -230 110 -170 {
lab=vdd}
N -250 -100 -210 -100 {
lab=#net1}
N -250 -130 -250 -100 {
lab=#net1}
N -110 -100 110 -100 {
lab=#net1}
N 110 -130 110 -100 {
lab=#net1}
N -280 50 -210 50 {
lab=vs}
N 80 50 150 50 {
lab=vs}
N -210 110 -30 110 {
lab=vs}
N -280 -50 -210 -50 {
lab=#net1}
N 80 -50 150 -50 {
lab=vout}
N -210 -230 -110 -230 {
lab=vdd}
N -210 -120 -210 -100 {
lab=#net1}
N 150 -120 150 -100 {
lab=vout}
N 150 -100 150 -70 {
lab=vout}
N -110 -230 110 -230 {
lab=vdd}
N 350 -70 350 -20 {
lab=vout}
N 350 160 350 240 {
lab=vss}
N 350 40 350 100 {
lab=vr}
N -30 240 350 240 {
lab=vss}
N -30 220 -30 240 {
lab=vss}
C {devices/res.sym} -150 0 0 0 {name=Rdiff_2
value=1
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -280 0 0 0 {name=Gdiff_2 value=1}
C {devices/res.sym} 210 0 0 0 {name=Rdiff_1
value=1
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 80 0 0 0 {name=Gdiff_1 value=1}
C {devices/isource.sym} -30 190 0 0 {name=I2 value=1}
C {devices/vsource.sym} 20 -200 0 0 {name=V1 value=1.8 savecurrent=false}
C {devices/vsource.sym} -410 40 0 0 {name=V_n value=1 savecurrent=false}
C {devices/lab_pin.sym} -30 110 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} -410 -20 1 0 {name=p2 sig_type=std_logic lab=vneg}
C {devices/vsource.sym} -60 20 0 0 {name=V_p value=1 savecurrent=false}
C {devices/lab_pin.sym} -60 -20 1 0 {name=p3 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} 350 -70 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} -40 -230 1 0 {name=p5 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 20 -170 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -410 90 3 0 {name=p7 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -60 70 3 0 {name=p8 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -30 250 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/res.sym} 250 -150 0 0 {name=Raload_1
value=1
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 150 -150 2 1 {name=Gaload_1 value=1}
C {devices/vccs.sym} -210 -150 2 1 {name=Gaload_2 value=1}
C {devices/res.sym} -110 -150 0 0 {name=Raload_2
value=1
footprint=1206
device=resistor
m=1}
C {devices/vsource.sym} 350 130 0 0 {name=Vr value=0.9 savecurrent=false}
C {devices/res.sym} 350 10 0 0 {name=Rr
value=1000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 350 70 2 0 {name=p10 sig_type=std_logic lab=vr}
