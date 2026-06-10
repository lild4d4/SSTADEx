v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 170 -190 170 -170 {
lab=vdd}
N 170 -110 170 -90 {
lab=vout}
N 300 -190 300 -170 {
lab=vdd}
N 170 -190 300 -190 {
lab=vdd}
N 300 -110 300 -90 {
lab=vout}
N 170 -90 300 -90 {
lab=vout}
N 130 -190 170 -190 {
lab=vdd}
N 50 -190 50 -120 {
lab=#net1}
N -320 -190 -200 -190 {
lab=#net1}
N -320 -120 -320 -110 {
lab=vss}
N -200 -120 -200 -110 {
lab=vss}
N -200 -190 -200 -180 {
lab=#net1}
N -320 -190 -320 -180 {
lab=#net1}
N 130 -190 130 -160 {
lab=vdd}
N 270 -350 270 -300 {
lab=vdd}
N 170 -350 270 -350 {
lab=vdd}
N -280 -170 -240 -170 {
lab=vss}
N 50 -120 130 -120 {
lab=#net1}
N 170 -90 170 -60 {
lab=vout}
N 120 -190 130 -190 {
lab=vdd}
N 50 -190 60 -190 {
lab=#net1}
N 120 -90 170 -90 {
lab=vout}
N 50 -90 60 -90 {
lab=#net1}
N 50 -120 50 -90 {
lab=#net1}
N -130 -190 -130 -180 {
lab=#net1}
N -130 -190 50 -190 {
lab=#net1}
N -200 -190 -130 -190 {
lab=#net1}
N -200 -110 -130 -110 {
lab=vss}
N -130 -120 -130 -110 {
lab=vss}
N 170 -60 430 -60 {
lab=vout}
N -240 -110 -200 -110 {
lab=vss}
N 170 -350 170 -190 {
lab=vdd}
N -240 -170 -240 -110 {
lab=vss}
N -320 -110 -240 -110 {
lab=vss}
N -240 -110 -240 -50 {
lab=vss}
N -280 -130 -280 -80 {
lab=vpos}
N 430 -60 430 -10 {
lab=vout}
N 430 50 430 110 {
lab=vr}
N 430 170 430 190 {
lab=vss}
N 110 70 110 80 {
lab=vss}
N 230 70 230 80 {
lab=vss}
N 230 0 230 10 {
lab=vout}
N 110 0 110 10 {
lab=vout}
N 150 20 190 20 {
lab=vss}
N 190 80 230 80 {
lab=vss}
N 190 20 190 80 {
lab=vss}
N 170 0 230 0 {
lab=vout}
N 170 80 190 80 {
lab=vss}
N 150 60 150 80 {
lab=vss}
N 110 80 150 80 {
lab=vss}
N 170 -60 170 0 {
lab=vout}
N 110 0 170 0 {
lab=vout}
N 170 80 170 120 {
lab=vss}
N 150 80 170 80 {
lab=vss}
C {devices/res.sym} 300 -140 0 0 {name=Ro_2stage
value=100
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 170 -140 2 1 {name=Gm_2stage value=1}
C {devices/vccs.sym} -320 -150 0 1 {name=Gma_1stage value=1}
C {devices/res.sym} -200 -150 2 0 {name=Ra_1stage
value=100
footprint=1206
device=resistor
m=1}
C {devices/vsource.sym} 270 -270 0 0 {name=V1 value=1 savecurrent=false}
C {devices/lab_pin.sym} 270 -240 3 0 {name=p1 sig_type=std_logic lab=vss
value=1}
C {devices/lab_pin.sym} -240 -50 3 0 {name=p3 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 430 -60 2 0 {name=p2 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 270 -350 2 0 {name=p7 sig_type=std_logic lab=vdd}
C {devices/vsource.sym} -280 -50 0 0 {name=V2 value=1 savecurrent=false}
C {devices/lab_pin.sym} -280 -20 3 0 {name=p8 sig_type=std_logic lab=vss
value=1}
C {devices/lab_pin.sym} 170 120 3 0 {name=p4 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -280 -100 0 0 {name=p5 sig_type=std_logic lab=vpos}
C {devices/vsource.sym} 430 140 0 0 {name=Vr value=0.9 savecurrent=false}
C {devices/res.sym} 430 20 0 0 {name=Rr
value=1000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 430 80 2 0 {name=p10 sig_type=std_logic lab=vr}
C {devices/lab_pin.sym} 430 190 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/vccs.sym} 110 40 0 1 {name=Gcs_2stage value=1}
C {devices/res.sym} 230 40 2 0 {name=Rcs_2stage
value=100
footprint=1206
device=resistor
m=1}
