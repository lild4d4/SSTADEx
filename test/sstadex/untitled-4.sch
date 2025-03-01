v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 0 -150 0 -130 {
lab=vout}
N 0 -70 0 -50 {
lab=vcp}
N 130 -150 130 -130 {
lab=vout}
N 0 -150 130 -150 {
lab=vout}
N 130 -70 130 -50 {
lab=vcp}
N 0 -50 130 -50 {
lab=vcp}
N 0 -50 0 60 {
lab=vcp}
N -40 -80 -40 -50 {
lab=vcp}
N -40 -50 0 -50 {
lab=vcp}
N -40 -150 0 -150 {
lab=vout}
N -120 -150 -100 -150 {
lab=vss}
N -120 -150 -120 -120 {
lab=vss}
N -120 -120 -40 -120 {
lab=vss}
N -120 -50 -40 -50 {
lab=vcp}
N -120 -120 -120 -110 {
lab=vss}
N -160 -120 -120 -120 {
lab=vss}
N 0 60 0 80 {
lab=vcp}
N 0 140 0 160 {
lab=vss}
N 130 60 130 80 {
lab=vcp}
N 0 60 130 60 {
lab=vcp}
N 130 140 130 160 {
lab=vss}
N 0 160 130 160 {
lab=vss}
N -40 130 -40 160 {
lab=vss}
N -40 160 0 160 {
lab=vss}
N -40 60 0 60 {
lab=vcp}
N -120 60 -100 60 {
lab=vin}
N -120 60 -120 90 {
lab=vin}
N -120 90 -40 90 {
lab=vin}
N -120 160 -40 160 {
lab=vss}
N -120 90 -120 100 {
lab=vin}
N -160 90 -120 90 {
lab=vin}
N 0 -180 0 -150 {
lab=vout}
N 0 -180 110 -180 {
lab=vout}
N 240 -150 240 -130 {
lab=vout}
N 130 -150 240 -150 {
lab=vout}
N 240 -70 240 -50 {
lab=vcp}
N 130 -50 240 -50 {
lab=vcp}
N 240 60 240 90 {
lab=vcp}
N 130 60 240 60 {
lab=vcp}
N 240 150 240 160 {
lab=vss}
N 130 160 240 160 {
lab=vss}
N 0 -260 0 -240 {
lab=vss}
C {devices/res.sym} 130 -100 0 0 {name=Ro_m2
value=71761.75
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 0 -100 0 0 {name=Gm_m2 value=0.000224742}
C {capa.sym} -120 -80 2 0 {name=Cgs_m2
m=1
value=4.44e-15
footprint=1206
device="ceramic capacitor"}
C {devices/res.sym} 130 110 0 0 {name=Ro_m1
value=1495752.06
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 0 110 0 0 {name=Gm_m1 value=0.000110133}
C {devices/lab_pin.sym} -160 -120 0 0 {name=p11 sig_type=std_logic lab=vss
value=vref}
C {devices/lab_pin.sym} 110 -180 2 0 {name=p1 sig_type=std_logic lab=vout
value=vref}
C {devices/lab_pin.sym} -160 90 0 0 {name=Vin2 sig_type=std_logic lab=vin
value=vref}
C {capa.sym} 240 120 0 0 {name=Cds_m1
m=1
value=1.28e-14
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 0 0 2 0 {name=Vin3 sig_type=std_logic lab=vcp
value=vref}
C {devices/lab_pin.sym} 0 160 3 0 {name=p3 sig_type=std_logic lab=vss
value=vref}
C {devices/code_shown.sym} -870 -200 0 0 {name=s1 only_toplevel=false value="

Vin vin 0 1 ac 1
Vss vss 0 0

.control
ac dec 10 1 1G
plot vdb(vout)
plot phase(vout)*180/PI
meas ac gain find vdb(vout) at=1
let low_gain = gain-3

meas ac fc WHEN vdb(vout)=low_gain
print fc/(2*PI)
.endc

"}
C {devices/isource.sym} 0 -210 0 0 {name=I2 value=0}
C {devices/lab_pin.sym} 0 -260 1 0 {name=p2 sig_type=std_logic lab=vss
value=vref}
