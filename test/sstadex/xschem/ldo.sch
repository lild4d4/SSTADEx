v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 80 -70 80 -50 {
lab=#net1}
N 80 10 80 30 {
lab=vout}
N 210 -70 210 -60 {
lab=#net1}
N 80 -70 210 -70 {
lab=#net1}
N 210 -60 210 -50 {
lab=#net1}
N 210 10 210 30 {
lab=vout}
N 80 30 210 30 {
lab=vout}
N 50 -70 80 -70 {
lab=#net1}
N 40 -70 50 -70 {
lab=#net1}
N -40 -70 -40 -40 {
lab=#net2}
N 80 30 80 80 {
lab=vout}
N 80 140 80 170 {
lab=vfb}
N 80 230 80 250 {
lab=vss}
N 80 -200 80 -70 {
lab=#net1}
N -160 -70 -60 -70 {
lab=#net2}
N -320 -70 -270 -70 {
lab=#net2}
N -270 -70 -240 -70 {
lab=#net2}
N -240 -70 -210 -70 {
lab=#net2}
N -320 0 -320 10 {
lab=vss}
N -280 10 -280 30 {
lab=vss}
N -320 10 -200 10 {
lab=vss}
N -200 0 -200 10 {
lab=vss}
N -200 -70 -200 -60 {
lab=#net2}
N 40 150 80 150 {
lab=vfb}
N 80 60 250 60 {
lab=vout}
N -320 -70 -320 -60 {
lab=#net2}
N -210 -70 -160 -70 {
lab=#net2}
N 250 60 260 60 {
lab=vout}
N -40 0 40 0 {
lab=#net2}
N -40 -40 -40 0 {
lab=#net2}
N 40 -70 40 -40 {
lab=#net1}
N -60 -70 -40 -70 {
lab=#net2}
N 180 -230 180 -200 {
lab=#net1}
N 80 -230 180 -230 {
lab=#net1}
N 80 -230 80 -210 {
lab=#net1}
N 80 -210 80 -200 {
lab=#net1}
N 180 -190 180 -180 {
lab=#net1}
N 180 -200 180 -190 {
lab=#net1}
N -240 -50 -240 -10 {
lab=#net3}
N -280 -50 -240 -50 {
lab=#net3}
N -240 -10 -240 40 {
lab=#net3}
C {devices/res.sym} 210 -20 0 0 {name=Ro_pt
value=473596.7
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 80 -20 2 1 {name=Gm_pt value=2.7200000000000004e-05}
C {devices/res.sym} 80 110 0 0 {name=R2
value=100000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 80 200 0 0 {name=R3
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
C {devices/lab_pin.sym} -280 30 3 0 {name=p8 sig_type=std_logic lab=vss
value=vref}
C {devices/vsource.sym} 180 -150 0 0 {name=V1 value=3 savecurrent=false}
C {devices/lab_pin.sym} 180 -120 3 0 {name=p1 sig_type=std_logic lab=vss}
C {devices/vsource.sym} -240 70 0 0 {name=V2 value=3 savecurrent=false}
C {devices/lab_pin.sym} -240 100 3 0 {name=p3 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 80 250 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 260 60 2 0 {name=p2 sig_type=std_logic lab=vout}
