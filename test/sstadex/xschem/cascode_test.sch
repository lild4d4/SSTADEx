v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 140 -220 140 -200 {
lab=vout}
N 140 -140 140 -120 {
lab=vcp}
N 270 -220 270 -200 {
lab=vout}
N 140 -220 270 -220 {
lab=vout}
N 270 -140 270 -120 {
lab=vcp}
N 140 -120 270 -120 {
lab=vcp}
N 140 -120 140 -10 {
lab=vcp}
N 100 -150 100 -120 {
lab=vcp}
N 100 -120 140 -120 {
lab=vcp}
N 100 -220 140 -220 {
lab=vout}
N 20 -220 40 -220 {
lab=vss}
N 20 -220 20 -190 {
lab=vss}
N 20 -190 100 -190 {
lab=vss}
N 20 -120 100 -120 {
lab=vcp}
N 20 -190 20 -180 {
lab=vss}
N -20 -190 20 -190 {
lab=vss}
N 140 -10 140 10 {
lab=vcp}
N 140 70 140 90 {
lab=vss}
N 270 -10 270 10 {
lab=vcp}
N 140 -10 270 -10 {
lab=vcp}
N 270 70 270 90 {
lab=vss}
N 140 90 270 90 {
lab=vss}
N 100 60 100 90 {
lab=vss}
N 100 90 140 90 {
lab=vss}
N 100 -10 140 -10 {
lab=vcp}
N 20 -10 40 -10 {
lab=vin}
N 20 -10 20 20 {
lab=vin}
N 20 20 100 20 {
lab=vin}
N 20 90 100 90 {
lab=vss}
N 20 20 20 30 {
lab=vin}
N -20 20 20 20 {
lab=vin}
N 140 -250 140 -220 {
lab=vout}
N 140 -250 250 -250 {
lab=vout}
N 140 -280 140 -250 {
lab=vout}
N 140 -360 140 -340 {
lab=vss}
N -120 20 -80 20 {
lab=vss}
N 380 -220 380 -200 {
lab=vout}
N 270 -220 380 -220 {
lab=vout}
N 380 -140 380 -120 {
lab=vcp}
N 270 -120 380 -120 {
lab=vcp}
N 380 -10 380 20 {
lab=vcp}
N 270 -10 380 -10 {
lab=vcp}
N 380 80 380 90 {
lab=vss}
N 270 90 380 90 {
lab=vss}
C {devices/res.sym} 270 -170 0 0 {name=Ro_m2
value=4.91151e10
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 140 -170 0 0 {name=Gm_m2 value=3.3e-7}
C {capa.sym} 70 -220 3 0 {name=Cgd_m2
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 20 -150 0 0 {name=Cgs_m1
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/res.sym} 270 40 0 0 {name=Ro_m1
value=4.91151e10
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 140 40 0 0 {name=Gm_m1 value=3.3e-7}
C {capa.sym} 70 -10 3 0 {name=Cgd_m1
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 20 60 0 0 {name=Cgs_m2
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} -20 -190 0 0 {name=p11 sig_type=std_logic lab=vss
value=vref}
C {devices/isource.sym} 140 -310 0 0 {name=I2 value=0}
C {devices/lab_pin.sym} 140 -360 1 0 {name=p2 sig_type=std_logic lab=vss
value=vref}
C {devices/lab_pin.sym} 250 -250 2 0 {name=p1 sig_type=std_logic lab=vout
value=vref}
C {vsource.sym} -50 20 1 0 {name=V1 value=3 savecurrent=false}
C {devices/lab_pin.sym} -120 20 0 0 {name=Vin1 sig_type=std_logic lab=vss
value=vref}
C {devices/lab_pin.sym} 20 -10 0 0 {name=Vin2 sig_type=std_logic lab=vin
value=vref}
C {capa.sym} 380 -170 0 0 {name=Cds_m2
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 380 50 0 0 {name=Cds_m1
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 140 -70 2 0 {name=Vin3 sig_type=std_logic lab=vcp
value=vref}
C {devices/lab_pin.sym} 140 90 3 0 {name=p3 sig_type=std_logic lab=vss
value=vref}
