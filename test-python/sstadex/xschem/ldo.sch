v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 80 -70 80 -50 {
lab=vdd}
N 80 10 80 30 {
lab=vout}
N 210 -70 210 -50 {
lab=vdd}
N 80 -70 210 -70 {
lab=vdd}
N 210 10 210 30 {
lab=vout}
N 80 30 210 30 {
lab=vout}
N 40 -70 80 -70 {
lab=vdd}
N -40 -70 -40 0 {
lab=#net1}
N 80 60 80 80 {
lab=vout}
N 80 150 80 170 {
lab=vfb}
N 80 240 80 250 {
lab=vss}
N -120 -70 -40 -70 {
lab=#net1}
N -320 -70 -200 -70 {
lab=#net1}
N -320 0 -320 10 {
lab=vdd}
N -200 0 -200 10 {
lab=vdd}
N -200 -70 -200 -60 {
lab=#net1}
N 40 150 80 150 {
lab=vfb}
N 200 60 300 60 {
lab=vout}
N -320 -70 -320 -60 {
lab=#net1}
N -200 -70 -120 -70 {
lab=#net1}
N 40 -70 40 -40 {
lab=vdd}
N 180 -230 180 -180 {
lab=vdd}
N 80 -230 180 -230 {
lab=vdd}
N -280 -50 -240 -50 {
lab=#net2}
N -40 0 40 -0 {
lab=#net1}
N -120 -70 -120 -60 {
lab=#net1}
N -120 -0 -120 10 {
lab=vdd}
N -200 10 -120 10 {
lab=vdd}
N 300 60 340 60 {
lab=vout}
N 300 60 300 130 {
lab=vout}
N 200 60 200 130 {
lab=vout}
N 200 190 200 240 {
lab=vss}
N 80 240 200 240 {
lab=vss}
N 200 240 300 240 {
lab=vss}
N 300 190 300 240 {
lab=vss}
N -40 60 80 60 {
lab=vout}
N -320 10 -200 10 {
lab=vdd}
N 80 140 80 150 {
lab=vfb}
N 80 30 80 60 {
lab=vout}
N 80 60 200 60 {
lab=vout}
N 80 230 80 240 {
lab=vss}
N -240 -50 -240 40 {
lab=#net2}
N -390 10 -320 10 {
lab=vdd}
N -390 -110 -390 10 {
lab=vdd}
N 80 -110 80 -70 {
lab=vdd}
N -390 -110 80 -110 {
lab=vdd}
N 80 -230 80 -110 {
lab=vdd}
C {devices/res.sym} 210 -20 0 0 {name=Ro_pt
value=473596.7
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 80 -20 2 1 {name=Gm_pt value=2.7200000000000004e-05}
C {devices/res.sym} 80 110 0 0 {name=R1
value=100000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 80 200 0 0 {name=R2
value=300000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -320 -30 2 0 {name=Gma value=10}
C {devices/res.sym} -200 -30 2 0 {name=Ra
value=50
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} -280 -10 2 0 {name=p4 sig_type=std_logic lab=vfb
value=vref}
C {devices/lab_pin.sym} 40 150 0 0 {name=p5 sig_type=std_logic lab=vfb}
C {devices/vsource.sym} 180 -150 0 0 {name=V1 value=3 savecurrent=false}
C {devices/lab_pin.sym} 180 -120 3 0 {name=p1 sig_type=std_logic lab=vss}
C {devices/vsource.sym} -240 70 0 0 {name=V2 value=3 savecurrent=false}
C {devices/lab_pin.sym} -240 100 3 0 {name=p3 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 80 250 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 340 60 2 0 {name=p2 sig_type=std_logic lab=vout}
C {capa.sym} 200 160 0 0 {name=Cl
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/res.sym} 300 160 0 0 {name=Rl
value=100000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 180 -230 2 0 {name=p7 sig_type=std_logic lab=vdd}
C {capa.sym} -120 -30 0 0 {name=Ca
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {capa.sym} -40 30 0 0 {name=Cc
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
