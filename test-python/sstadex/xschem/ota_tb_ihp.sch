v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 370 -60 530 -60 {
lab=vs}
N 350 -60 350 -10 {
lab=vs}
N 700 -240 730 -240 {
lab=vout}
N 40 -180 40 -70 {
lab=vn}
N 40 -180 130 -180 {
lab=vn}
N 590 -180 590 -90 {
lab=vp1}
N 350 50 350 100 {
lab=vss}
N 170 -60 350 -60 {
lab=vs}
N 170 -400 530 -400 {
lab=vdd}
N 170 -280 170 -210 {
lab=#net1}
N 170 -400 170 -370 {
lab=vdd}
N 530 -310 530 -240 {
lab=vout}
N 530 -400 530 -370 {
lab=vdd}
N 530 -240 530 -210 {
lab=vout}
N 530 -150 530 -60 {
lab=vs}
N 170 -150 170 -60 {
lab=vs}
N 570 -180 590 -180 {
lab=vp1}
N 370 -180 530 -180 {
lab=vs}
N 370 -180 370 -60 {
lab=vs}
N 350 100 350 140 {
lab=vss}
N 350 -340 490 -340 {
lab=#net1}
N 350 -340 350 -280 {
lab=#net1}
N 170 -280 350 -280 {
lab=#net1}
N 120 -340 170 -340 {
lab=vdd}
N 120 -400 120 -340 {
lab=vdd}
N 120 -400 170 -400 {
lab=vdd}
N 530 -340 580 -340 {
lab=vdd}
N 580 -400 580 -340 {
lab=vdd}
N 530 -400 580 -400 {
lab=vdd}
N 700 -240 700 -180 {
lab=vout}
N 350 100 700 100 {
lab=vss}
N 700 -120 700 100 {
lab=vss}
N 170 -180 370 -180 {
lab=vs}
N 350 -60 370 -60 {
lab=vs}
N 210 -340 350 -340 {
lab=#net1}
N 170 -310 170 -280 {
lab=#net1}
N 530 -240 700 -240 {
lab=vout}
C {devices/isource.sym} 350 20 0 0 {name=I2 value=40e-6}
C {devices/lab_pin.sym} 350 -60 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 730 -240 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 330 -400 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 40 -70 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} 590 -90 3 0 {name=p8 sig_type=std_logic lab=vp1}
C {devices/lab_pin.sym} 350 140 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/code_shown.sym} -640 -390 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt

Vref vn 0 0.9
Vdd vdd 0 1.8 ac 1
Vpos vp1 0 0.9
Vss vss 0 0

.control
ac dec 10 1 1G
plot vdb(vout)
plot phase(vout)
op 
print vout vs
.endc

"}
C {sg13g2_pr/sg13_hv_nmos.sym} 150 -180 0 0 {name=M1
l=0.45u
w=1.0u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 550 -180 0 1 {name=M2
l=0.45u
w=1.0u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 510 -340 0 0 {name=M3
l=0.45u
w=1.0u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 190 -340 0 1 {name=M4
l=0.45u
w=1.0u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
