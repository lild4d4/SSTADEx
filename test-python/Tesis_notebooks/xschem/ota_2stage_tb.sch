v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 550 -120 710 -120 {
lab=vs}
N 220 -240 310 -240 {
lab=#net1}
N 770 -240 770 -150 {
lab=vn}
N 350 -120 550 -120 {
lab=vs}
N 350 -460 710 -460 {
lab=vdd}
N 350 -340 350 -270 {
lab=#net2}
N 350 -460 350 -430 {
lab=vdd}
N 710 -370 710 -300 {
lab=vout_s1}
N 710 -460 710 -430 {
lab=vdd}
N 710 -300 710 -270 {
lab=vout_s1}
N 710 -210 710 -120 {
lab=vs}
N 350 -210 350 -120 {
lab=vs}
N 750 -240 770 -240 {
lab=vn}
N 550 -240 710 -240 {
lab=vs}
N 550 -240 550 -120 {
lab=vs}
N 530 -400 670 -400 {
lab=#net2}
N 530 -400 530 -340 {
lab=#net2}
N 350 -340 530 -340 {
lab=#net2}
N 300 -400 350 -400 {
lab=vdd}
N 300 -460 300 -400 {
lab=vdd}
N 300 -460 350 -460 {
lab=vdd}
N 710 -400 760 -400 {
lab=vdd}
N 760 -460 760 -400 {
lab=vdd}
N 710 -460 760 -460 {
lab=vdd}
N 950 -350 950 -330 {
lab=vdd}
N 760 -460 950 -460 {
lab=vdd}
N 950 -300 980 -300 {
lab=vdd}
N 980 -350 980 -300 {
lab=vdd}
N 950 -350 980 -350 {
lab=vdd}
N 1080 -240 1260 -240 {
lab=vout}
N 950 -270 950 -240 {
lab=vout}
N 950 -240 950 -70 {
lab=vout}
N 1080 80 1370 80 {
lab=vss}
N 350 -240 550 -240 {
lab=vs}
N 390 -400 530 -400 {
lab=#net2}
N 350 -370 350 -340 {
lab=#net2}
N 950 -460 950 -350 {
lab=vdd}
N 1370 -240 1370 -180 {
lab=vfb}
N 1370 -120 1370 80 {
lab=vss}
N 1320 -240 1370 -240 {
lab=vfb}
N 1370 -270 1370 -240 {
lab=vfb}
N 1370 -350 1370 -330 {
lab=vss}
N 220 -240 220 -220 {
lab=#net1}
N 220 -160 220 -130 {
lab=vfb}
N 1080 -240 1080 -180 {
lab=vout}
N 950 -240 1080 -240 {
lab=vout}
N 1080 -120 1080 80 {
lab=vss}
N 550 -120 550 -70 {
lab=vs}
N 550 -10 550 80 {
lab=vss}
N 550 -40 590 -40 {
lab=vss}
N 590 -40 590 80 {
lab=vss}
N 550 80 590 80 {
lab=vss}
N 90 80 550 80 {
lab=vss}
N 90 -10 90 80 {
lab=vss}
N 90 -100 90 -70 {
lab=#net3}
N 170 -100 170 -40 {
lab=#net3}
N 130 -40 170 -40 {
lab=#net3}
N 90 -100 170 -100 {
lab=#net3}
N 90 -300 90 -100 {
lab=#net3}
N 40 -40 90 -40 {
lab=vss}
N 40 -40 40 80 {
lab=vss}
N 40 80 90 80 {
lab=vss}
N 90 -460 90 -360 {
lab=vdd}
N 90 -460 300 -460 {
lab=vdd}
N 950 -10 950 80 {
lab=vss}
N 950 -40 990 -40 {
lab=vss}
N 990 -40 990 80 {
lab=vss}
N 950 80 990 80 {
lab=vss}
N 590 80 950 80 {
lab=vss}
N 480 -40 510 -40 {
lab=#net3}
N 480 -90 480 -40 {
lab=#net3}
N 170 -40 480 -40 {
lab=#net3}
N 480 -90 850 -90 {
lab=#net3}
N 850 -90 850 -40 {
lab=#net3}
N 850 -40 910 -40 {
lab=#net3}
N 990 80 1080 80 {
lab=vss}
N 710 -300 910 -300 {
lab=vout_s1}
N 910 -240 950 -240 {
lab=vout}
N 820 -240 850 -240 {
lab=#net4}
C {devices/lab_pin.sym} 550 -150 0 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 1130 -240 1 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 510 -460 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 770 -150 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} 800 -300 1 0 {name=p21 sig_type=std_logic lab=vout_s1}
C {devices/code_shown.sym} -670 -470 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt

Vref vn 0 0.9
Vdd vdd 0 1.8
Vss vss 0 0

.control
ac dec 10 1 1G
plot vdb(vout)
plot phase(vout)*180/PI
meas pm find phase(vout)*180/PI when vdb(vout)=0
op 
print vout vs vout_s1 vfb
.endc

"}
C {sg13g2_pr/sg13_hv_nmos.sym} 730 -240 2 0 {name=M1
l=0.45u
w=117u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 330 -240 2 1 {name=M2
l=0.45u
w=117u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 690 -400 0 0 {name=M3
l=0.4u
w=1.92u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 370 -400 0 1 {name=M4
l=0.4u
w=1.92u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 930 -300 0 0 {name=M5
l=6.4u
w=96.35u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {res.sym} 1290 -240 1 0 {name=R1
value=10000000000
footprint=1206
device=resistor
m=1}
C {capa.sym} 1370 -150 0 0 {name=C2
m=1
value=10
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 1370 -240 2 0 {name=p5 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 220 -130 2 0 {name=p2 sig_type=std_logic lab=vfb}
C {res.sym} 1370 -300 2 0 {name=R2
value=27600000000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 1370 -350 1 0 {name=p3 sig_type=std_logic lab=vss}
C {vsource.sym} 220 -190 0 0 {name=V1 value="dc 0 ac 1" savecurrent=false}
C {capa.sym} 1080 -150 0 0 {name=C3
m=1
value=2.516e-11
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 550 80 3 0 {name=p9 sig_type=std_logic lab=vss}
C {sg13g2_pr/sg13_hv_nmos.sym} 530 -40 0 0 {name=M6
l=3.2u
w=28.4u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 110 -40 0 1 {name=M7
l=3.2u
w=28.4u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {isource.sym} 90 -330 0 0 {name=I2 value=40e-6}
C {sg13g2_pr/sg13_hv_nmos.sym} 930 -40 0 0 {name=M8
l=3.2u
w=28.4u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {capa.sym} 820 -270 2 0 {name=Cc
m=1
value=1e-10
footprint=1206
device="ceramic capacitor"}
C {res.sym} 880 -240 1 0 {name=Rc
value=10000
footprint=1206
device=resistor
m=1}
