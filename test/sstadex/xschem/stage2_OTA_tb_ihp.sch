v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 440 -130 600 -130 {
lab=vs}
N 420 -130 420 -80 {
lab=vs}
N 600 -310 800 -310 {
lab=vout_s1}
N 110 -250 200 -250 {
lab=#net1}
N 660 -250 660 -160 {
lab=vn}
N 420 -20 420 70 {
lab=vss}
N 240 -130 420 -130 {
lab=vs}
N 240 -470 600 -470 {
lab=vdd}
N 240 -350 240 -280 {
lab=#net2}
N 240 -470 240 -440 {
lab=vdd}
N 600 -380 600 -310 {
lab=vout_s1}
N 600 -470 600 -440 {
lab=vdd}
N 600 -310 600 -280 {
lab=vout_s1}
N 600 -220 600 -130 {
lab=vs}
N 240 -220 240 -130 {
lab=vs}
N 640 -250 660 -250 {
lab=vn}
N 440 -250 600 -250 {
lab=vs}
N 440 -250 440 -130 {
lab=vs}
N 420 -410 560 -410 {
lab=#net2}
N 420 -410 420 -350 {
lab=#net2}
N 240 -350 420 -350 {
lab=#net2}
N 190 -410 240 -410 {
lab=vdd}
N 190 -470 190 -410 {
lab=vdd}
N 190 -470 240 -470 {
lab=vdd}
N 600 -410 650 -410 {
lab=vdd}
N 650 -470 650 -410 {
lab=vdd}
N 600 -470 650 -470 {
lab=vdd}
N 840 -360 840 -340 {
lab=vdd}
N 650 -470 840 -470 {
lab=vdd}
N 840 -310 870 -310 {
lab=vdd}
N 870 -360 870 -310 {
lab=vdd}
N 840 -360 870 -360 {
lab=vdd}
N 840 -250 970 -250 {
lab=vout}
N 840 -280 840 -250 {
lab=vout}
N 840 -250 840 -180 {
lab=vout}
N 420 70 840 70 {
lab=vss}
N 840 -120 840 70 {
lab=vss}
N 970 -250 1150 -250 {
lab=vout}
N 970 -250 970 -140 {
lab=vout}
N 840 70 970 70 {
lab=vss}
N 240 -250 440 -250 {
lab=vs}
N 420 -130 440 -130 {
lab=vs}
N 280 -410 420 -410 {
lab=#net2}
N 240 -380 240 -350 {
lab=#net2}
N 840 -470 840 -360 {
lab=vdd}
N 970 -30 970 70 {
lab=vss}
N 1260 -250 1260 -190 {
lab=vfb}
N 1260 -130 1260 -30 {
lab=vss}
N 1210 -250 1260 -250 {
lab=vfb}
N 970 -30 1260 -30 {
lab=vss}
N 970 -80 970 -30 {
lab=vss}
N 1260 -280 1260 -250 {
lab=vfb}
N 1260 -360 1260 -340 {
lab=vss}
N 110 -250 110 -230 {
lab=#net1}
N 110 -170 110 -140 {
lab=vfb}
C {devices/isource.sym} 420 -50 0 0 {name=I2 value=40e-6}
C {devices/lab_pin.sym} 420 -130 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 1020 -250 1 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 400 -470 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 660 -160 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} 420 70 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/isource.sym} 840 -150 0 0 {name=I1 value=20e-6}
C {devices/lab_pin.sym} 690 -310 1 0 {name=p21 sig_type=std_logic lab=vout_s1}
C {devices/code_shown.sym} -890 -400 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt

Vref vn 0 0.9
Vdd vdd 0 1.8
Vss vss 0 0

.param vcc=1.8

.control
ac dec 10 1 1G
plot vdb(vout)
plot phase(vout)
op 
print vout vs vout_s1 vfb
.endc

"}
C {capa.sym} 970 -110 0 0 {name=C1
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {sg13g2_pr/sg13_hv_nmos.sym} 620 -250 2 0 {name=M1
l=0.8u
w=219.27u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 220 -250 2 1 {name=M2
l=0.8u
w=219.27u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 580 -410 0 0 {name=M3
l=0.4u
w=10.48u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 260 -410 0 1 {name=M4
l=0.4u
w=10.48u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 820 -310 0 0 {name=M5
l=0.8u
w=31.32u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {res.sym} 1180 -250 1 0 {name=R1
value=100000000
footprint=1206
device=resistor
m=1}
C {capa.sym} 1260 -160 0 0 {name=C2
m=1
value=10
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 1260 -250 2 0 {name=p5 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 110 -140 2 0 {name=p2 sig_type=std_logic lab=vfb}
C {res.sym} 1260 -310 2 0 {name=R2
value=450000000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 1260 -360 1 0 {name=p3 sig_type=std_logic lab=vss}
C {vsource.sym} 110 -200 0 0 {name=V1 value="dc 0 ac 1" savecurrent=false}
