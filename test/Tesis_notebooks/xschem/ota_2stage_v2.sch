v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 140 -130 140 -110 {
lab=vdd}
N 140 -50 140 -30 {
lab=vout}
N 270 -130 270 -110 {
lab=vdd}
N 140 -130 270 -130 {
lab=vdd}
N 270 -50 270 -30 {
lab=vout}
N 140 -30 270 -30 {
lab=vout}
N 100 -130 140 -130 {
lab=vdd}
N 20 -130 20 -60 {
lab=vpos}
N 100 -130 100 -100 {
lab=vdd}
N 240 -290 240 -240 {
lab=vdd}
N 140 -290 240 -290 {
lab=vdd}
N 20 -60 100 -60 {
lab=vpos}
N 140 -30 140 0 {
lab=vout}
N 90 -130 100 -130 {
lab=vdd}
N 20 -130 30 -130 {
lab=vpos}
N -230 -130 20 -130 {
lab=vpos}
N 370 0 400 0 {
lab=vout}
N 140 -290 140 -130 {
lab=vdd}
N 140 40 220 40 {
lab=vout}
N 100 110 100 120 {
lab=vss}
N 220 110 220 120 {
lab=vss}
N 220 40 220 50 {
lab=vout}
N 100 40 100 50 {
lab=vout}
N 140 60 180 60 {
lab=vss}
N 180 120 220 120 {
lab=vss}
N 180 60 180 120 {
lab=vss}
N 140 0 140 40 {
lab=vout}
N 100 40 140 40 {
lab=vout}
N 140 120 180 120 {
lab=vss}
N 140 100 140 120 {
lab=vss}
N 100 120 140 120 {
lab=vss}
N 140 120 140 180 {
lab=vss}
N 370 0 370 60 {
lab=vout}
N 140 0 370 0 {
lab=vout}
N 140 180 370 180 {
lab=vss}
N 370 120 370 180 {
lab=vss}
N -230 -130 -230 -90 {
lab=vpos}
C {devices/res.sym} 270 -80 0 0 {name=Ro_2stage
value=100
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 140 -80 2 1 {name=Gm_2stage value=1}
C {devices/vsource.sym} 240 -210 0 0 {name=V1 value=1 savecurrent=false}
C {devices/lab_pin.sym} 240 -180 3 0 {name=p1 sig_type=std_logic lab=vss
value=1}
C {devices/lab_pin.sym} 400 0 2 0 {name=p2 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 240 -290 2 0 {name=p7 sig_type=std_logic lab=vdd}
C {capa.sym} 60 -130 3 0 {name=Cin_2stage
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/vsource.sym} -230 -60 0 0 {name=V2 value=1 savecurrent=false}
C {devices/lab_pin.sym} -230 -30 3 0 {name=p8 sig_type=std_logic lab=vss
value=1}
C {devices/lab_pin.sym} 140 180 3 0 {name=p4 sig_type=std_logic lab=vss}
C {devices/vccs.sym} 100 80 0 1 {name=Gcs_2stage value=1}
C {devices/res.sym} 220 80 2 0 {name=Rcs_2stage
value=100
footprint=1206
device=resistor
m=1}
C {capa.sym} 370 90 0 0 {name=Cin_pt
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} -230 -130 0 0 {name=p5 sig_type=std_logic lab=vin}
