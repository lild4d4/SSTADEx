v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 230 -350 390 -350 {
lab=vs}
N 210 -350 210 -300 {
lab=vs}
N 390 -530 590 -530 {
lab=vout_s1}
N -100 -470 -100 -360 {
lab=vn}
N -100 -470 -10 -470 {
lab=vn}
N 450 -470 450 -380 {
lab=vfb}
N 210 -240 210 -150 {
lab=vss}
N 30 -350 210 -350 {
lab=vs}
N 30 -690 390 -690 {
lab=vdd}
N 30 -570 30 -500 {
lab=#net1}
N 30 -690 30 -660 {
lab=vdd}
N 390 -600 390 -530 {
lab=vout_s1}
N 390 -690 390 -660 {
lab=vdd}
N 390 -530 390 -500 {
lab=vout_s1}
N 390 -440 390 -350 {
lab=vs}
N 30 -440 30 -350 {
lab=vs}
N 430 -470 450 -470 {
lab=vfb}
N 230 -470 390 -470 {
lab=vs}
N 230 -470 230 -350 {
lab=vs}
N 210 -630 350 -630 {
lab=#net1}
N 210 -630 210 -570 {
lab=#net1}
N 30 -570 210 -570 {
lab=#net1}
N -20 -630 30 -630 {
lab=vdd}
N -20 -690 -20 -630 {
lab=vdd}
N -20 -690 30 -690 {
lab=vdd}
N 390 -630 440 -630 {
lab=vdd}
N 440 -690 440 -630 {
lab=vdd}
N 390 -690 440 -690 {
lab=vdd}
N 630 -580 630 -560 {
lab=vdd}
N 630 -530 660 -530 {
lab=vdd}
N 660 -580 660 -530 {
lab=vdd}
N 630 -580 660 -580 {
lab=vdd}
N 630 -470 810 -470 {
lab=vmid}
N 630 -500 630 -470 {
lab=vmid}
N 630 -470 630 -400 {
lab=vmid}
N 210 -150 630 -150 {
lab=vss}
N 630 -340 630 -150 {
lab=vss}
N 540 -470 630 -470 {
lab=vmid}
N 630 -690 850 -690 {
lab=vdd}
N 850 -410 850 -390 {
lab=vout}
N 850 -320 850 -300 {
lab=vfb}
N 850 -230 850 -220 {
lab=vss}
N 810 -320 850 -320 {
lab=vfb}
N 1070 -410 1070 -340 {
lab=vout}
N 970 -410 970 -340 {
lab=vout}
N 970 -280 970 -230 {
lab=vss}
N 850 -230 970 -230 {
lab=vss}
N 970 -230 1070 -230 {
lab=vss}
N 1070 -280 1070 -230 {
lab=vss}
N 1070 -410 1150 -410 {
lab=vout}
N 850 -470 890 -470 {
lab=vdd}
N 890 -520 890 -470 {
lab=vdd}
N 850 -520 890 -520 {
lab=vdd}
N 850 -520 850 -500 {
lab=vdd}
N 850 -690 850 -520 {
lab=vdd}
N 30 -470 230 -470 {
lab=vs}
N 210 -350 230 -350 {
lab=vs}
N 70 -630 210 -630 {
lab=#net1}
N 30 -600 30 -570 {
lab=#net1}
N 630 -690 630 -580 {
lab=vdd}
N 850 -330 850 -320 {
lab=vfb}
N 970 -410 1070 -410 {
lab=vout}
N 850 -410 970 -410 {
lab=vout}
N 850 -240 850 -230 {
lab=vss}
N 440 -690 630 -690 {
lab=vdd}
N 850 -440 850 -410 {
lab=vout}
C {devices/isource.sym} 210 -270 0 0 {name=I2 value=40e-6}
C {devices/lab_pin.sym} 210 -350 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 1150 -410 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 190 -690 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} -100 -360 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} 210 -150 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/isource.sym} 630 -370 0 0 {name=I1 value=20e-6}
C {capa.sym} 540 -500 0 0 {name=C8
m=1
value=3e-12
footprint=1206
device="ceramic capacitor"}
C {devices/res.sym} 850 -360 0 0 {name=R1
value=100000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 850 -270 0 0 {name=R2
value=300000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 810 -320 0 0 {name=p5 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 850 -220 3 0 {name=p2 sig_type=std_logic lab=vss}
C {capa.sym} 970 -310 0 0 {name=Cl
m=1
value=1e-6
footprint=1206
device="ceramic capacitor"}
C {devices/res.sym} 1070 -310 0 0 {name=R3
value=18
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 450 -380 3 0 {name=p3 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 750 -470 3 0 {name=p8 sig_type=std_logic lab=vmid}
C {devices/lab_pin.sym} 480 -530 1 0 {name=p10 sig_type=std_logic lab=vout_s1}
C {devices/code_shown.sym} -790 -640 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt

Vref vn 0 1.35
Vdd vdd 0 3.3 ac 1
Vss vss 0 0

.control
ac dec 10 1 100G
plot vdb(vout) 
plot phase(vout)
op
print vout vmid vout_s1 vs
.endc

"}
C {sg13g2_pr/sg13_hv_nmos.sym} 410 -470 2 0 {name=M1
l=3.2u
w=10.24u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 10 -470 2 1 {name=M2
l=3.2u
w=10.24u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 370 -630 0 0 {name=M3
l=6.4u
w=9.02u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 50 -630 0 1 {name=M4
l=6.4u
w=9.02u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 610 -530 0 0 {name=M5
l=6.4u
w=9.02u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 830 -470 0 0 {name=M6
l=0.4u
w=2487u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
