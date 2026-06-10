v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -10 -10 -10 50 {
lab=vpos}
N -60 -10 -10 -10 {
lab=vpos}
N -150 -10 -120 -10 {
lab=vfb}
N -10 110 -10 130 {
lab=vss}
N 170 -290 170 -270 {
lab=vss}
N 170 -210 170 -190 {
lab=vout}
N 300 -290 300 -270 {
lab=vss}
N 170 -290 300 -290 {
lab=vss}
N 300 -210 300 -190 {
lab=vout}
N 170 -190 300 -190 {
lab=vout}
N 130 -290 170 -290 {
lab=vss}
N 50 -290 50 -220 {
lab=#net1}
N 170 -160 170 -140 {
lab=vout}
N 170 -70 170 -50 {
lab=vfb}
N 170 20 170 30 {
lab=vss}
N -320 -290 -200 -290 {
lab=#net1}
N -320 -220 -320 -210 {
lab=vss}
N -200 -220 -200 -210 {
lab=vss}
N -200 -290 -200 -280 {
lab=#net1}
N 130 -70 170 -70 {
lab=vfb}
N -320 -290 -320 -280 {
lab=#net1}
N 130 -290 130 -260 {
lab=vss}
N 170 -450 270 -450 {
lab=vss}
N -280 -270 -240 -270 {
lab=vss}
N 50 -220 130 -220 {
lab=#net1}
N 390 -160 430 -160 {
lab=vout}
N 390 -160 390 -90 {
lab=vout}
N 290 20 390 20 {
lab=vss}
N 390 -30 390 20 {
lab=vss}
N 170 -80 170 -70 {
lab=vfb}
N 170 -190 170 -160 {
lab=vout}
N 290 -160 390 -160 {
lab=vout}
N 170 10 170 20 {
lab=vss}
N 170 -450 170 -290 {
lab=vss}
N -320 -210 -200 -210 {
lab=vss}
N 120 -290 130 -290 {
lab=vss}
N 50 -290 60 -290 {
lab=#net1}
N 120 -190 170 -190 {
lab=vout}
N 50 -190 60 -190 {
lab=#net1}
N 50 -220 50 -190 {
lab=#net1}
N -130 -290 -130 -280 {
lab=#net1}
N -130 -290 50 -290 {
lab=#net1}
N -200 -290 -130 -290 {
lab=#net1}
N -200 -210 -130 -210 {
lab=vss}
N -130 -220 -130 -210 {
lab=vss}
N 290 -160 290 -90 {
lab=vout}
N 170 -160 290 -160 {
lab=vout}
N 290 -30 290 20 {
lab=vss}
N 170 20 290 20 {
lab=vss}
N -240 -270 -240 -140 {
lab=vss}
N -280 -120 -280 -100 {
lab=vpos}
N -280 -230 -280 -180 {
lab=vol}
C {res.sym} -90 -10 1 0 {name=R3
value=10000000000
footprint=1206
device=resistor
m=1}
C {capa.sym} -10 80 0 0 {name=C1
m=1
value=10
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} -150 -10 0 0 {name=p13 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} -10 -10 2 0 {name=p8 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} -10 130 3 0 {name=p14 sig_type=std_logic lab=vss}
C {devices/res.sym} 300 -240 0 0 {name=Ro_pt
value=201
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 170 -240 2 1 {name=Gm_pt value=0.1845}
C {devices/res.sym} 170 -110 0 0 {name=R1
value=22500
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 170 -20 0 0 {name=R2
value=67500
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -320 -250 0 1 {name=Gma value=0.0061445}
C {devices/res.sym} -200 -250 2 0 {name=Ra
value=1991879
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 130 -70 0 0 {name=p10 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 270 -450 3 0 {name=p11 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -320 -210 0 0 {name=p12 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 170 30 3 0 {name=p15 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 430 -160 2 0 {name=p16 sig_type=std_logic lab=vout}
C {devices/res.sym} 390 -60 0 0 {name=Rl
value=18
footprint=1206
device=resistor
m=1}
C {capa.sym} 90 -190 3 0 {name=Cgd_pt
m=1
value=4.19e-14
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 290 -60 0 0 {name=Cl
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} -240 -140 3 0 {name=p18 sig_type=std_logic lab=vss}
C {devices/vsource.sym} -280 -150 0 0 {name=V3 value="dc 0 ac 1" savecurrent=false}
C {devices/lab_pin.sym} -280 -100 3 0 {name=p19 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} -280 -190 0 0 {name=p20 sig_type=std_logic lab=vol}
C {devices/code_shown.sym} -1240 -440 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt

Vref vn 0 1.35
Vdd vdd 0 3.3 ac 1
Vdd_1 vdd_1 0 3.3
Vss vss 0 0

.control
ac dec 10 1 100G
plot vdb(vout) 
plot phase(vout)
op
print vout vmid vout_s1 vs
.endc

"}
C {capa.sym} 90 -290 3 0 {name=Cgd_pt1
m=1
value=3.13e-12
footprint=1206
device="ceramic capacitor"}
C {capa.sym} -130 -250 0 0 {name=Cgd_pt2
m=1
value=8e-11
footprint=1206
device="ceramic capacitor"}
