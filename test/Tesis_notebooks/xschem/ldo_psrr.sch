v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 60 -130 60 -110 {
lab=vdd}
N 60 -50 60 -30 {
lab=vout}
N 190 -130 190 -110 {
lab=vdd}
N 60 -130 190 -130 {
lab=vdd}
N 190 -50 190 -30 {
lab=vout}
N 60 -30 190 -30 {
lab=vout}
N 20 -130 60 -130 {
lab=vdd}
N -60 -130 -60 -60 {
lab=#net1}
N 60 0 60 20 {
lab=vout}
N 60 90 60 110 {
lab=vfb}
N 60 180 60 190 {
lab=vss}
N -340 -130 -220 -130 {
lab=#net1}
N -340 -60 -340 -50 {
lab=vss}
N -220 -60 -220 -50 {
lab=vss}
N -220 -130 -220 -120 {
lab=#net1}
N 20 90 60 90 {
lab=vfb}
N -340 -130 -340 -120 {
lab=#net1}
N -220 -130 -60 -130 {
lab=#net1}
N 20 -130 20 -100 {
lab=vdd}
N 160 -290 160 -240 {
lab=vdd}
N 60 -290 160 -290 {
lab=vdd}
N -300 -110 -260 -110 {
lab=vss}
N -60 -60 20 -60 {
lab=#net1}
N 280 0 320 0 {
lab=vout}
N 280 0 280 70 {
lab=vout}
N 60 180 280 180 {
lab=vss}
N 280 130 280 180 {
lab=vss}
N -260 -50 -220 -50 {
lab=vss}
N 60 80 60 90 {
lab=vfb}
N 60 -30 60 0 {
lab=vout}
N 60 0 280 0 {
lab=vout}
N 60 170 60 180 {
lab=vss}
N 60 -290 60 -130 {
lab=vdd}
N -340 -50 -260 -50 {
lab=vss}
N -260 -110 -260 -50 {
lab=vss}
C {devices/res.sym} 190 -80 0 0 {name=Ro_pt
value=1
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 60 -80 2 1 {name=Gm_pt value=1}
C {devices/res.sym} 60 50 0 0 {name=R1
value=1
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 60 140 0 0 {name=R2
value=1
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -340 -90 0 1 {name=Gma value=1}
C {devices/res.sym} -220 -90 2 0 {name=Ra
value=1
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} -300 -70 2 0 {name=p4 sig_type=std_logic lab=vfb
value=vref}
C {devices/lab_pin.sym} 20 90 0 0 {name=p5 sig_type=std_logic lab=vfb}
C {devices/vsource.sym} 160 -210 0 0 {name=V1 value=3 savecurrent=false}
C {devices/lab_pin.sym} 160 -180 3 0 {name=p1 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -260 -50 3 0 {name=p3 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 60 190 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 320 0 2 0 {name=p2 sig_type=std_logic lab=vout}
C {devices/res.sym} 280 100 0 0 {name=Rl
value=1
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 160 -290 2 0 {name=p7 sig_type=std_logic lab=vdd}
