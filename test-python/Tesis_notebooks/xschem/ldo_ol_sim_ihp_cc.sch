v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 800 -90 800 -70 {
lab=vout}
N 800 0 800 20 {
lab=vfb}
N 800 90 800 100 {
lab=vss}
N 760 0 800 0 {
lab=vfb}
N 1020 -90 1020 -20 {
lab=vout}
N 920 -90 920 -20 {
lab=vout}
N 920 40 920 90 {
lab=vss}
N 800 90 920 90 {
lab=vss}
N 920 90 1020 90 {
lab=vss}
N 1020 40 1020 90 {
lab=vss}
N 1020 -90 1100 -90 {
lab=vout}
N 800 -150 840 -150 {
lab=vdd}
N 840 -200 840 -150 {
lab=vdd}
N 800 -200 840 -200 {
lab=vdd}
N 800 -200 800 -180 {
lab=vdd}
N 800 -10 800 0 {
lab=vfb}
N 920 -90 1020 -90 {
lab=vout}
N 800 -90 920 -90 {
lab=vout}
N 800 80 800 90 {
lab=vss}
N 800 -120 800 -90 {
lab=vout}
N 350 140 510 140 {
lab=vs}
N 330 140 330 190 {
lab=vs}
N 20 20 20 130 {
lab=#net1}
N 20 20 110 20 {
lab=#net1}
N 570 20 570 140 {
lab=vn}
N 150 140 330 140 {
lab=vs}
N 150 -300 510 -300 {
lab=vdd}
N 150 -300 150 -270 {
lab=vdd}
N 510 -150 510 -100 {
lab=vmid}
N 510 -300 510 -270 {
lab=vdd}
N 510 -20 510 -10 {
lab=vcp}
N 510 50 510 140 {
lab=vs}
N 150 50 150 140 {
lab=vs}
N 550 20 570 20 {
lab=vn}
N 350 20 510 20 {
lab=vs}
N 350 20 350 140 {
lab=vs}
N 330 -240 470 -240 {
lab=#net2}
N 330 -240 330 -180 {
lab=#net2}
N 150 -180 330 -180 {
lab=#net2}
N 100 -240 150 -240 {
lab=vdd}
N 100 -300 100 -240 {
lab=vdd}
N 100 -300 150 -300 {
lab=vdd}
N 510 -240 560 -240 {
lab=vdd}
N 560 -300 560 -240 {
lab=vdd}
N 510 -300 560 -300 {
lab=vdd}
N 150 20 350 20 {
lab=vs}
N 330 140 350 140 {
lab=vs}
N 190 -240 330 -240 {
lab=#net2}
N 150 -210 150 -180 {
lab=#net2}
N 150 -20 150 -10 {
lab=#net3}
N 190 -70 470 -70 {
lab=vbias}
N 150 -180 150 -100 {
lab=#net2}
N 110 -70 150 -70 {
lab=#net3}
N 110 -70 110 -20 {
lab=#net3}
N 110 -20 150 -20 {
lab=#net3}
N 150 -40 150 -20 {
lab=#net3}
N 510 -70 550 -70 {
lab=vcp}
N 550 -70 550 -20 {
lab=vcp}
N 510 -20 550 -20 {
lab=vcp}
N 510 -40 510 -20 {
lab=vcp}
N 330 250 330 340 {
lab=vss}
N 510 -150 760 -150 {
lab=vmid}
N 510 -210 510 -150 {
lab=vmid}
N 560 -300 800 -300 {
lab=vdd}
N 800 -300 800 -200 {
lab=vdd}
N 760 230 760 290 {
lab=vpos}
N 710 230 760 230 {
lab=vpos}
N 620 230 650 230 {
lab=vfb}
N 760 350 760 370 {
lab=vss}
N 20 180 20 210 {
lab=vpos}
N 730 -90 800 -90 {
lab=vout}
C {devices/lab_pin.sym} 1100 -90 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/res.sym} 800 -40 0 0 {name=R1
value=100000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 800 50 0 0 {name=R2
value=300000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 760 0 0 0 {name=p5 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 800 100 3 0 {name=p2 sig_type=std_logic lab=vss}
C {capa.sym} 920 10 0 0 {name=Cl
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/res.sym} 1020 10 0 0 {name=R3
value=60000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 700 -150 3 0 {name=p8 sig_type=std_logic lab=vmid}
C {devices/code_shown.sym} -840 -320 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt

Vref vn 0 1.35
Vdd vdd 0 3.3
Vss vss 0 0
Vbias vbias 0 2.2

.control
ac dec 10 1 100G
plot vdb(vout) 
plot phase(vout)*180/PI
op
print vout vmid vcp vfb vs
.endc

"}
C {sg13g2_pr/sg13_hv_pmos.sym} 490 -240 0 0 {name=M6
l=6.4u
w=10.97u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {devices/isource.sym} 330 220 0 0 {name=I2 value=40e-6}
C {devices/lab_pin.sym} 330 140 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 310 -300 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 570 140 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} 330 340 3 0 {name=p9 sig_type=std_logic lab=vss}
C {sg13g2_pr/sg13_hv_nmos.sym} 530 20 2 0 {name=M1
l=6.4u
w=5.87u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 130 20 2 1 {name=M2
l=6.4u
w=5.87u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 490 -70 2 1 {name=M3
l=0.8u
w=6.42u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 170 -70 2 0 {name=M4
l=0.8u
w=6.42u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 170 -240 0 1 {name=M5
l=6.4u
w=10.97u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 780 -150 0 0 {name=M7
l=0.4u
w=2487u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {devices/lab_pin.sym} 320 -70 3 0 {name=p10 sig_type=std_logic lab=vbias}
C {devices/lab_pin.sym} 550 -20 2 0 {name=p11 sig_type=std_logic lab=vcp}
C {devices/lab_pin.sym} 20 210 2 0 {name=p12 sig_type=std_logic lab=vpos}
C {res.sym} 680 230 1 0 {name=R4
value=100000000
footprint=1206
device=resistor
m=1}
C {capa.sym} 760 320 0 0 {name=C2
m=1
value=10
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 620 230 0 0 {name=p13 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 760 230 2 0 {name=p3 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} 760 370 3 0 {name=p14 sig_type=std_logic lab=vss}
C {vsource.sym} 20 150 0 0 {name=V1 value="dc 0 ac 1" savecurrent=false}
C {capa.sym} 730 -120 0 0 {name=Cl1
m=1
value=3e-12
footprint=1206
device="ceramic capacitor"}
