v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 360 40 360 100 {
lab=vpos}
N 310 40 360 40 {
lab=vpos}
N 220 40 250 40 {
lab=vfb}
N 360 160 360 180 {
lab=vss}
N 540 -240 540 -220 {
lab=vss}
N 540 -160 540 -140 {
lab=vout}
N 670 -240 670 -220 {
lab=vss}
N 540 -240 670 -240 {
lab=vss}
N 670 -160 670 -140 {
lab=vout}
N 540 -140 670 -140 {
lab=vout}
N 500 -240 540 -240 {
lab=vss}
N 420 -240 420 -170 {
lab=#net1}
N 540 -110 540 -90 {
lab=vout}
N 540 -20 540 0 {
lab=vfb}
N 540 70 540 80 {
lab=vss}
N 50 -240 170 -240 {
lab=#net1}
N 50 -170 50 -160 {
lab=vss}
N 170 -170 170 -160 {
lab=vss}
N 170 -240 170 -230 {
lab=#net1}
N 500 -20 540 -20 {
lab=vfb}
N 50 -240 50 -230 {
lab=#net1}
N 500 -240 500 -210 {
lab=vss}
N 540 -400 640 -400 {
lab=vss}
N 420 -170 500 -170 {
lab=#net1}
N 760 -110 800 -110 {
lab=vout}
N 760 -110 760 -40 {
lab=vout}
N 660 70 760 70 {
lab=vss}
N 760 20 760 70 {
lab=vss}
N 540 -30 540 -20 {
lab=vfb}
N 540 -140 540 -110 {
lab=vout}
N 660 -110 760 -110 {
lab=vout}
N 540 60 540 70 {
lab=vss}
N 540 -400 540 -240 {
lab=vss}
N 50 -160 170 -160 {
lab=vss}
N 490 -240 500 -240 {
lab=vss}
N 420 -240 430 -240 {
lab=#net1}
N 490 -140 540 -140 {
lab=vout}
N 420 -140 430 -140 {
lab=#net1}
N 420 -170 420 -140 {
lab=#net1}
N 170 -240 420 -240 {
lab=#net1}
N 660 -110 660 -40 {
lab=vout}
N 540 -110 660 -110 {
lab=vout}
N 660 20 660 70 {
lab=vss}
N 540 70 660 70 {
lab=vss}
N -300 -240 -180 -240 {
lab=#net2}
N -300 -170 -300 -160 {
lab=vss}
N -180 -170 -180 -160 {
lab=vss}
N -180 -240 -180 -230 {
lab=#net2}
N -300 -240 -300 -230 {
lab=#net2}
N -260 -220 -220 -220 {
lab=vol}
N -260 -160 -180 -160 {
lab=vss}
N -110 -240 -110 -230 {
lab=#net2}
N -180 -240 -110 -240 {
lab=#net2}
N -180 -160 -110 -160 {
lab=vss}
N -110 -170 -110 -160 {
lab=vss}
N -220 -20 -220 0 {
lab=vpos}
N -110 -240 -0 -240 {
lab=#net2}
N -0 -240 -0 -220 {
lab=#net2}
N -0 -220 10 -220 {
lab=#net2}
N -110 -160 0 -160 {
lab=vss}
N 0 -180 -0 -160 {
lab=vss}
N -0 -180 10 -180 {
lab=vss}
N -220 -220 -220 -80 {
lab=vol}
N -260 -180 -260 -160 {
lab=vss}
N -300 -160 -260 -160 {
lab=vss}
C {res.sym} 280 40 1 0 {name=R3
value=10000000000
footprint=1206
device=resistor
m=1}
C {capa.sym} 360 130 0 0 {name=C1
m=1
value=10
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 220 40 0 0 {name=p13 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 360 40 2 0 {name=p8 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} 360 180 3 0 {name=p14 sig_type=std_logic lab=vss}
C {devices/res.sym} 670 -190 0 0 {name=Ro_pt
value=201
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 540 -190 2 1 {name=Gm_pt value=0.1845}
C {devices/res.sym} 540 -60 0 0 {name=R1
value=22500
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 540 30 0 0 {name=R2
value=67500
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 50 -200 0 0 {name=Gma value=0.000073814}
C {devices/res.sym} 170 -200 2 0 {name=Ra
value=1991878
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 500 -20 0 0 {name=p10 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 640 -400 3 0 {name=p11 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 50 -160 0 0 {name=p12 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 540 80 3 0 {name=p15 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 800 -110 2 0 {name=p16 sig_type=std_logic lab=vout}
C {devices/res.sym} 760 -10 0 0 {name=Rl
value=18
footprint=1206
device=resistor
m=1}
C {capa.sym} 660 -10 0 0 {name=Cl
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/code_shown.sym} -870 -390 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt

Vref vn 0 1.35
Vdd vdd 0 3.3 ac 1
Vdd_1 vdd_1 0 3.3
Vss vss 0 0

.control
ac dec 10 1 10G
plot vdb(vout) 
plot phase(vout)
op
print vout vmid vout_s1 vs
.endc

"}
C {capa.sym} 460 -240 3 0 {name=Cgd_pt1
m=1
value=3.13e-12
footprint=1206
device="ceramic capacitor"}
C {devices/vccs.sym} -300 -200 0 1 {name=Gma_1stage value=0.0001536}
C {devices/res.sym} -180 -200 2 0 {name=Ra_1stage
value=541957
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} -300 -160 0 0 {name=p1 sig_type=std_logic lab=vss}
C {devices/vsource.sym} -220 -50 0 0 {name=V1 value="dc 0 ac 1" savecurrent=false}
C {devices/lab_pin.sym} -220 0 3 0 {name=p3 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} -220 -90 0 0 {name=p4 sig_type=std_logic lab=vol}
C {capa.sym} -110 -200 0 0 {name=Cin_2stage
m=1
value=1e-14
footprint=1206
device="ceramic capacitor"}
