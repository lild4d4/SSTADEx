v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 160 -90 160 -70 {
lab=vdd}
N 160 -10 160 10 {
lab=vout}
N 290 -90 290 -80 {
lab=vdd}
N 160 -90 290 -90 {
lab=vdd}
N 290 -80 290 -70 {
lab=vdd}
N 290 -10 290 10 {
lab=vout}
N 160 10 290 10 {
lab=vout}
N 130 -90 160 -90 {
lab=vdd}
N 120 -90 130 -90 {
lab=vdd}
N 40 -90 40 -60 {
lab=#net1}
N 160 10 160 60 {
lab=vout}
N 160 120 160 150 {
lab=vfb}
N 160 210 160 230 {
lab=vss}
N 160 -220 160 -90 {
lab=vdd}
N -80 -90 20 -90 {
lab=#net1}
N -240 -90 -190 -90 {
lab=#net1}
N -190 -90 -160 -90 {
lab=#net1}
N -160 -90 -130 -90 {
lab=#net1}
N -240 -20 -240 -10 {
lab=vss}
N -200 -10 -200 10 {
lab=vss}
N -240 -10 -120 -10 {
lab=vss}
N -120 -20 -120 -10 {
lab=vss}
N -120 -90 -120 -80 {
lab=#net1}
N 120 130 160 130 {
lab=vfb}
N 160 40 330 40 {
lab=vout}
N -240 -90 -240 -80 {
lab=#net1}
N -130 -90 -80 -90 {
lab=#net1}
N 330 40 340 40 {
lab=vout}
N 40 -20 120 -20 {
lab=#net1}
N 40 -60 40 -20 {
lab=#net1}
N 120 -90 120 -60 {
lab=vdd}
N 20 -90 40 -90 {
lab=#net1}
N 260 -250 260 -220 {
lab=vdd}
N 160 -250 260 -250 {
lab=vdd}
N 160 -250 160 -230 {
lab=vdd}
N 160 -230 160 -220 {
lab=vdd}
N 260 -210 260 -200 {
lab=vdd}
N 260 -220 260 -210 {
lab=vdd}
N -160 -70 -160 -30 {
lab=vref}
N -200 -70 -160 -70 {
lab=vref}
N -160 -30 -160 20 {
lab=vref}
C {devices/res.sym} 290 -40 0 0 {name=Ro_pt
value=100
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 160 -40 2 1 {name=Gm_pt value=100}
C {devices/res.sym} 160 90 0 0 {name=R2
value=100000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 160 180 0 0 {name=R3
value=300000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -240 -50 2 0 {name=Gma value=0.001}
C {devices/res.sym} -120 -50 2 0 {name=Ra
value=100
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} -200 -30 2 0 {name=p4 sig_type=std_logic lab=vfb
value=vref}
C {devices/lab_pin.sym} 120 130 0 0 {name=p5 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} -200 10 3 0 {name=p8 sig_type=std_logic lab=vss
value=vref}
C {devices/lab_pin.sym} 160 230 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 340 40 2 0 {name=p2 sig_type=std_logic lab=vout}
C {devices/code_shown.sym} -290 170 0 0 {name=s1 only_toplevel=false value="

Vref vref 0 1
Vdd vdd 0 1 ac 1
Vss vss 0 0

.control

ac dec 10 1 1G
plot vdb(vout)


.endc

"}
C {devices/lab_pin.sym} 260 -200 3 0 {name=p9 sig_type=std_logic lab=vdd
}
C {devices/lab_pin.sym} -160 20 3 0 {name=p3 sig_type=std_logic lab=vref}
