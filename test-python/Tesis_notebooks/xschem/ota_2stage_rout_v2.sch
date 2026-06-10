v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 330 0 590 0 {
lab=vout}
N 590 0 590 50 {
lab=vout}
N 590 110 590 170 {
lab=vr}
N 590 230 590 250 {
lab=vss}
N 70 -130 70 -110 {
lab=vdd}
N 70 -50 70 -30 {
lab=vout}
N 200 -130 200 -110 {
lab=vdd}
N 70 -130 200 -130 {
lab=vdd}
N 200 -50 200 -30 {
lab=vout}
N 70 -30 200 -30 {
lab=vout}
N 30 -130 70 -130 {
lab=vdd}
N -50 -130 -50 -60 {
lab=vin}
N 30 -130 30 -100 {
lab=vdd}
N 170 -290 170 -240 {
lab=vdd}
N 70 -290 170 -290 {
lab=vdd}
N -50 -60 30 -60 {
lab=vin}
N 70 -30 70 0 {
lab=vout}
N 20 -130 30 -130 {
lab=vdd}
N -50 -130 -40 -130 {
lab=vin}
N -300 -130 -50 -130 {
lab=vin}
N 300 0 330 0 {
lab=vout}
N 70 -290 70 -130 {
lab=vdd}
N 70 40 150 40 {
lab=vout}
N 30 110 30 120 {
lab=vss}
N 150 110 150 120 {
lab=vss}
N 150 40 150 50 {
lab=vout}
N 30 40 30 50 {
lab=vout}
N 70 60 110 60 {
lab=vss}
N 110 120 150 120 {
lab=vss}
N 110 60 110 120 {
lab=vss}
N 70 0 70 40 {
lab=vout}
N 30 40 70 40 {
lab=vout}
N 70 120 110 120 {
lab=vss}
N 70 100 70 120 {
lab=vss}
N 30 120 70 120 {
lab=vss}
N 70 120 70 180 {
lab=vss}
N 300 0 300 60 {
lab=vout}
N 70 0 300 0 {
lab=vout}
N 70 180 300 180 {
lab=vss}
N 300 120 300 180 {
lab=vss}
N -300 -130 -300 -90 {
lab=vin}
C {devices/lab_pin.sym} 590 0 2 0 {name=p2 sig_type=std_logic lab=vout}
C {devices/vsource.sym} 590 200 0 0 {name=Vr value=0.9 savecurrent=false}
C {devices/res.sym} 590 80 0 0 {name=Rr
value=1000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 590 140 2 0 {name=p10 sig_type=std_logic lab=vr}
C {devices/lab_pin.sym} 590 250 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/res.sym} 200 -80 0 0 {name=Ro_2stage
value=100
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 70 -80 2 1 {name=Gm_2stage value=1}
C {devices/vsource.sym} 170 -210 0 0 {name=V1 value=1 savecurrent=false}
C {devices/lab_pin.sym} 170 -180 3 0 {name=p1 sig_type=std_logic lab=vss
value=1}
C {devices/lab_pin.sym} 170 -290 2 0 {name=p7 sig_type=std_logic lab=vdd}
C {capa.sym} -10 -130 3 0 {name=Cin_2stage
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/vsource.sym} -300 -60 0 0 {name=V2 value=1 savecurrent=false}
C {devices/lab_pin.sym} -300 -30 3 0 {name=p8 sig_type=std_logic lab=vss
value=1}
C {devices/lab_pin.sym} 70 180 3 0 {name=p4 sig_type=std_logic lab=vss}
C {devices/vccs.sym} 30 80 0 1 {name=Gcs_2stage value=1}
C {devices/res.sym} 150 80 2 0 {name=Rcs_2stage
value=100
footprint=1206
device=resistor
m=1}
C {capa.sym} 300 90 0 0 {name=Cin_pt
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} -300 -130 0 0 {name=p5 sig_type=std_logic lab=vin}
