v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 270 60 430 60 {
lab=vs}
N 250 60 250 110 {
lab=vs}
N -60 -60 -60 50 {
lab=vn}
N -60 -60 30 -60 {
lab=vn}
N 490 -60 490 30 {
lab=vp1}
N 250 220 250 260 {
lab=vss}
N 70 60 250 60 {
lab=vs}
N 70 -380 430 -380 {
lab=vdd}
N 70 -380 70 -350 {
lab=vdd}
N 430 -290 430 -220 {
lab=vout}
N 430 -380 430 -350 {
lab=vdd}
N 430 -100 430 -90 {
lab=vcp}
N 430 -30 430 60 {
lab=vs}
N 70 -30 70 60 {
lab=vs}
N 470 -60 490 -60 {
lab=vp1}
N 270 -60 430 -60 {
lab=vs}
N 270 -60 270 60 {
lab=vs}
N 250 -320 390 -320 {
lab=#net1}
N 250 -320 250 -260 {
lab=#net1}
N 70 -260 250 -260 {
lab=#net1}
N 20 -320 70 -320 {
lab=vdd}
N 20 -380 20 -320 {
lab=vdd}
N 20 -380 70 -380 {
lab=vdd}
N 430 -320 480 -320 {
lab=vdd}
N 480 -380 480 -320 {
lab=vdd}
N 430 -380 480 -380 {
lab=vdd}
N 70 -60 270 -60 {
lab=vs}
N 250 60 270 60 {
lab=vs}
N 110 -320 250 -320 {
lab=#net1}
N 70 -290 70 -260 {
lab=#net1}
N 430 -220 810 -220 {
lab=vout}
N 70 -100 70 -90 {
lab=#net2}
N 110 -150 390 -150 {
lab=vbias}
N 430 -220 430 -180 {
lab=vout}
N 70 -260 70 -180 {
lab=#net1}
N 30 -150 70 -150 {
lab=#net2}
N 30 -150 30 -100 {
lab=#net2}
N 30 -100 70 -100 {
lab=#net2}
N 70 -120 70 -100 {
lab=#net2}
N 430 -150 470 -150 {
lab=vcp}
N 470 -150 470 -100 {
lab=vcp}
N 430 -100 470 -100 {
lab=vcp}
N 430 -120 430 -100 {
lab=vcp}
N 920 -220 920 -160 {
lab=vfb}
N 920 -100 920 220 {
lab=vss}
N 870 -220 920 -220 {
lab=vfb}
N 920 -250 920 -220 {
lab=vfb}
N 920 -330 920 -310 {
lab=vss}
N 250 220 920 220 {
lab=vss}
N 250 170 250 220 {
lab=vss}
N 490 90 490 120 {
lab=vfb}
C {devices/isource.sym} 250 140 0 0 {name=I2 value=40e-6}
C {devices/lab_pin.sym} 250 60 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 630 -220 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 230 -380 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} -60 50 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} 490 30 2 0 {name=p8 sig_type=std_logic lab=vp1}
C {devices/lab_pin.sym} 250 260 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/code_shown.sym} -730 -370 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt

Vref vn 0 1.35
Vdd vdd 0 3.3
*Vpos vp1 0 1.35 ac 1
Vbias vbias 0 2.1
Vss vss 0 0

.control
ac dec 10 1 1G
plot vdb(vout)
plot phase(vout)*180/PI
meas ac gain find vdb(vout) at=1
let low_gain = gain-3
meas ac fc WHEN vdb(vout)=low_gain
op 
print vout vs vcp vbias-vcp vp1
.endc

"}
C {sg13g2_pr/sg13_hv_nmos.sym} 450 -60 2 0 {name=M1
l=6.4u
w=5.87u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 50 -60 2 1 {name=M2
l=6.4u
w=5.87u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 410 -150 2 1 {name=M3
l=0.4u
w=5.11u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 90 -150 2 0 {name=M4
l=0.4u
w=5.11u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 90 -320 0 1 {name=M5
l=6.4u
w=11.27u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 410 -320 0 0 {name=M6
l=6.4u
w=11.27u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {devices/lab_pin.sym} 240 -150 3 0 {name=p2 sig_type=std_logic lab=vbias}
C {devices/lab_pin.sym} 470 -100 2 0 {name=p3 sig_type=std_logic lab=vcp}
C {res.sym} 840 -220 1 0 {name=R1
value=100000000
footprint=1206
device=resistor
m=1}
C {capa.sym} 920 -130 0 0 {name=C2
m=1
value=10
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 920 -220 2 0 {name=p5 sig_type=std_logic lab=vfb}
C {res.sym} 920 -280 2 0 {name=R2
value=540000000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 920 -330 1 0 {name=p10 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 490 120 2 0 {name=p11 sig_type=std_logic lab=vfb}
C {vsource.sym} 490 60 0 0 {name=V1 value="dc 0 ac 1" savecurrent=false}
