v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -240 -100 -240 -40 {
lab=vpos}
N -290 -100 -240 -100 {
lab=vpos}
N -380 -100 -350 -100 {
lab=vfb}
N -240 20 -240 40 {
lab=vss}
N -60 -380 -60 -360 {
lab=vss}
N -60 -300 -60 -280 {
lab=vout}
N 70 -380 70 -360 {
lab=vss}
N -60 -380 70 -380 {
lab=vss}
N 70 -300 70 -280 {
lab=vout}
N -60 -280 70 -280 {
lab=vout}
N -100 -380 -60 -380 {
lab=vss}
N -180 -380 -180 -310 {
lab=#net1}
N -60 -250 -60 -230 {
lab=vout}
N -60 -160 -60 -140 {
lab=vfb}
N -60 -70 -60 -60 {
lab=vss}
N -550 -380 -430 -380 {
lab=#net1}
N -550 -310 -550 -300 {
lab=vss}
N -430 -310 -430 -300 {
lab=vss}
N -430 -380 -430 -370 {
lab=#net1}
N -100 -160 -60 -160 {
lab=vfb}
N -550 -380 -550 -370 {
lab=#net1}
N -100 -380 -100 -350 {
lab=vss}
N -60 -540 40 -540 {
lab=vss}
N -510 -360 -470 -360 {
lab=vss}
N -180 -310 -100 -310 {
lab=#net1}
N 160 -250 200 -250 {
lab=vout}
N 160 -250 160 -180 {
lab=vout}
N 60 -70 160 -70 {
lab=vss}
N 160 -120 160 -70 {
lab=vss}
N -60 -170 -60 -160 {
lab=vfb}
N -60 -280 -60 -250 {
lab=vout}
N 60 -250 160 -250 {
lab=vout}
N -60 -80 -60 -70 {
lab=vss}
N -60 -540 -60 -380 {
lab=vss}
N -550 -300 -430 -300 {
lab=vss}
N -110 -380 -100 -380 {
lab=vss}
N -180 -380 -170 -380 {
lab=#net1}
N -110 -280 -60 -280 {
lab=vout}
N -180 -280 -170 -280 {
lab=#net1}
N -180 -310 -180 -280 {
lab=#net1}
N -360 -380 -360 -370 {
lab=#net1}
N -360 -380 -180 -380 {
lab=#net1}
N -430 -380 -360 -380 {
lab=#net1}
N -430 -300 -360 -300 {
lab=vss}
N -360 -310 -360 -300 {
lab=vss}
N 60 -250 60 -180 {
lab=vout}
N -60 -250 60 -250 {
lab=vout}
N 60 -120 60 -70 {
lab=vss}
N -60 -70 60 -70 {
lab=vss}
N -470 -360 -470 -230 {
lab=vss}
N -510 -210 -510 -190 {
lab=vpos}
N -510 -320 -510 -270 {
lab=vol}
C {devices/code_shown.sym} -120 120 0 0 {name=s1 only_toplevel=false value="

Vss vss 0 0

.control

ac dec 10 1 1G
plot vdb(vout)
plot phase(vout)*180/PI


.endc

"}
C {res.sym} -320 -100 1 0 {name=R3
value=100000000
footprint=1206
device=resistor
m=1}
C {capa.sym} -240 -10 0 0 {name=C1
m=1
value=10
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} -380 -100 0 0 {name=p13 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} -240 -100 2 0 {name=p8 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} -240 40 3 0 {name=p14 sig_type=std_logic lab=vss}
C {devices/res.sym} 70 -330 0 0 {name=Ro_pt
value=1314
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -60 -330 2 1 {name=Gm_pt value=0.225}
C {devices/res.sym} -60 -200 0 0 {name=R1
value=328
footprint=1206
device=resistor
m=1}
C {devices/res.sym} -60 -110 0 0 {name=R2
value=984
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -550 -340 0 1 {name=Gma value=0.01}
C {devices/res.sym} -430 -340 2 0 {name=Ra
value=10000000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} -100 -160 0 0 {name=p10 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 40 -540 3 0 {name=p11 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -550 -300 0 0 {name=p12 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -60 -60 3 0 {name=p15 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 200 -250 2 0 {name=p16 sig_type=std_logic lab=vout}
C {devices/res.sym} 160 -150 0 0 {name=Rl
value=1311
footprint=1206
device=resistor
m=1}
C {capa.sym} -140 -380 3 0 {name=Cgg_pt
m=1
value=1.86e-11
footprint=1206
device="ceramic capacitor"}
C {capa.sym} -360 -340 0 0 {name=Ca
m=1
value=1e-17
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 60 -150 0 0 {name=Cl
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} -470 -230 3 0 {name=p18 sig_type=std_logic lab=vss}
C {devices/vsource.sym} -510 -240 0 0 {name=V3 value="dc 0 ac 1" savecurrent=false}
C {devices/lab_pin.sym} -510 -190 3 0 {name=p19 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} -510 -280 0 0 {name=p20 sig_type=std_logic lab=vol}
C {capa.sym} -140 -280 3 0 {name=Cgg_pt1
m=1
value=1.86e-11
footprint=1206
device="ceramic capacitor"}
