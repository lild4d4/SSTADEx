v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 70 -310 70 -290 {
lab=vdd}
N 70 -230 70 -210 {
lab=vout}
N 200 -310 200 -290 {
lab=vdd}
N 200 -230 200 -210 {
lab=vout}
N 70 -210 200 -210 {
lab=vout}
N 70 -140 70 -90 {
lab=vout}
N 70 -20 70 0 {
lab=vfb}
N 70 60 70 80 {
lab=vss}
N 70 80 210 80 {
lab=vss}
N 130 -310 200 -310 {
lab=vdd}
N 70 -140 210 -140 {
lab=vout}
N 70 -210 70 -140 {
lab=vout}
N 210 -10 210 80 {
lab=vss}
N 210 -140 210 -70 {
lab=vout}
N -230 -240 -110 -240 {
lab=vmid}
N -230 -170 -230 -160 {
lab=vss}
N -230 -160 -230 -140 {
lab=vss}
N -110 -170 -110 -160 {
lab=vss}
N -110 -240 -110 -230 {
lab=vmid}
N -230 -240 -230 -230 {
lab=vmid}
N -110 -240 30 -240 {
lab=vmid}
N 30 -310 70 -310 {
lab=vdd}
N 130 -370 130 -310 {
lab=vdd}
N -290 -160 -230 -160 {
lab=vss}
N 70 -30 70 -20 {
lab=vfb}
N 70 -310 130 -310 {
lab=vdd}
N -130 -20 70 -20 {
lab=vfb}
N -130 -20 -130 0 {
lab=vfb}
N 310 -140 310 -70 {
lab=vout}
N 210 -140 310 -140 {
lab=vout}
N 210 80 310 80 {
lab=vss}
N 310 -10 310 80 {
lab=vss}
N -170 -160 -110 -160 {
lab=vss}
N 30 -310 30 -280 {
lab=vdd}
N -170 -160 -170 -140 {
lab=vss}
N -230 -160 -170 -160 {
lab=vss}
N -170 -140 -140 -140 {
lab=vss}
C {devices/res.sym} 200 -260 0 0 {name=Ro_pt
value=201
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 70 -260 2 1 {name=Gm_pt value=0.184}
C {devices/res.sym} 70 -60 0 0 {name=R1
value=5000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 70 30 0 0 {name=R2
value=15000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 70 80 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 210 -140 2 0 {name=p2 sig_type=std_logic lab=vout}
C {devices/code_shown.sym} -580 -330 0 0 {name=s1 only_toplevel=false value="

Vdd vdd 0 3.3 ac 1
Vref vref 0 0
Vss vss 0 0
Vss_1 vss_1 0 0.5

.control
ac dec 10 1 10G
plot vdb(vout)
op
print vout vmid
.endc

"}
C {devices/vccs.sym} -230 -200 2 0 {name=Gma value=0.0001414}
C {devices/res.sym} -110 -200 2 0 {name=Ra
value=32253942
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 130 -370 1 0 {name=p15 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} -140 -140 3 0 {name=p16 sig_type=std_logic lab=vss
value=vref}
C {devices/lab_pin.sym} -130 -20 0 0 {name=p1 sig_type=std_logic lab=vfb
value=vfb}
C {devices/lab_pin.sym} -190 -220 2 0 {name=p3 sig_type=std_logic lab=vfb
value=vfb}
C {capa.sym} 310 -40 0 0 {name=Cl
m=1
value=1e-6
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} -190 -180 2 0 {name=p4 sig_type=std_logic lab=vref
value=vref}
C {devices/lab_pin.sym} -30 -240 3 0 {name=p5 sig_type=std_logic lab=vmid
value=vre}
C {devices/res.sym} 210 -40 0 0 {name=R3
value=18
footprint=1206
device=resistor
m=1}
