v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -40 -60 -40 -40 {
lab=vout}
N -40 -60 80 -60 {
lab=vout}
N 80 -60 80 -40 {
lab=vout}
N -40 20 -40 40 {
lab=vss}
N -40 40 80 40 {
lab=vss}
N 80 20 80 40 {
lab=vss}
N 150 -60 150 -40 {
lab=vout}
N 80 -60 150 -60 {
lab=vout}
N 80 40 150 40 {
lab=vss}
N 150 20 150 40 {
lab=vss}
N 80 -80 80 -60 {
lab=vout}
N -180 -60 -180 -40 {
lab=vin}
N -180 -60 -140 -60 {
lab=vin}
N -80 -60 -40 -60 {
lab=vout}
N -180 20 -180 40 {
lab=vss}
N -80 40 -40 40 {
lab=vss}
N -260 -60 -260 -40 {
lab=vin}
N -260 -60 -180 -60 {
lab=vin}
N -260 20 -260 40 {
lab=vss}
N -260 40 -180 40 {
lab=vss}
N -40 40 -40 80 {
lab=vss}
N 80 -240 80 -140 {
lab=vdd}
N 80 -240 160 -240 {
lab=vdd}
N 160 -240 160 -220 {
lab=vdd}
N -80 10 -80 40 {
lab=vss}
N -180 40 -80 40 {
lab=vss}
C {res.sym} 80 -10 0 0 {name=R1
value=1
footprint=1206
device=resistor
m=1}
C {vccs.sym} -40 -10 0 0 {name=G1 value=1e-6}
C {capa.sym} 150 -10 0 0 {name=C1
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {isource.sym} 80 -110 0 0 {name=I0 value=1}
C {capa.sym} -180 -10 0 0 {name=C2
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {capa.sym} -110 -60 3 0 {name=C3
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {vsource.sym} -260 -10 0 0 {name=V1 value=3 savecurrent=false}
C {lab_pin.sym} 80 -160 0 0 {name=p1 sig_type=std_logic lab=vdd}
C {lab_pin.sym} -40 80 0 0 {name=p2 sig_type=std_logic lab=vss}
C {lab_pin.sym} 150 -60 2 0 {name=p3 sig_type=std_logic lab=vout}
C {devices/vsource.sym} 160 -190 0 0 {name=V2 value=1.8 savecurrent=false}
C {devices/lab_pin.sym} 160 -160 3 0 {name=p6 sig_type=std_logic lab=vss}
C {lab_pin.sym} -260 -60 0 0 {name=p4 sig_type=std_logic lab=vin}
C {lab_pin.sym} -80 -30 0 0 {name=p5 sig_type=std_logic lab=vin}
