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
N -200 -190 50 -190 {
lab=#net1}
N 400 -60 430 -60 {
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
N 170 -20 250 -20 {
lab=vout}
N 130 50 130 60 {
lab=vss}
N 250 50 250 60 {
lab=vss}
N 250 -20 250 -10 {
lab=vout}
N 130 -20 130 -10 {
lab=vout}
N 170 0 210 0 {
lab=vss}
N 210 60 250 60 {
lab=vss}
N 210 0 210 60 {
lab=vss}
N 170 -60 170 -20 {
lab=vout}
N 130 -20 170 -20 {
lab=vout}
N 170 60 210 60 {
lab=vss}
N 170 40 170 60 {
lab=vss}
N 130 60 170 60 {
lab=vss}
N 170 60 170 120 {
lab=vss}
N 400 -60 400 -0 {
lab=vout}
N 170 -60 400 -60 {
lab=vout}
N 170 120 400 120 {
lab=vss}
N 400 60 400 120 {
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
C {capa.sym} 90 -190 3 0 {name=Cin_2stage
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/vsource.sym} -280 -50 0 0 {name=V2 value=1 savecurrent=false}
C {devices/lab_pin.sym} -280 -20 3 0 {name=p8 sig_type=std_logic lab=vss
value=1}
C {devices/lab_pin.sym} 170 120 3 0 {name=p4 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -280 -100 0 0 {name=p5 sig_type=std_logic lab=vpos}
C {devices/vccs.sym} 130 20 0 1 {name=Gcs_2stage value=1}
C {devices/res.sym} 250 20 2 0 {name=Rcs_2stage
value=100
footprint=1206
device=resistor
m=1}
C {capa.sym} 400 30 0 0 {name=Cin_pt
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
