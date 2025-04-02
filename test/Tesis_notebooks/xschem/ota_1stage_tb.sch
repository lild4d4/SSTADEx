v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 360 -80 520 -80 {
lab=vs}
N 340 -80 340 -30 {
lab=vs}
N 30 -200 30 -90 {
lab=vn}
N 30 -200 120 -200 {
lab=vn}
N 580 -200 580 -110 {
lab=#net1}
N 340 30 340 120 {
lab=vss}
N 160 -80 340 -80 {
lab=vs}
N 160 -420 520 -420 {
lab=vdd}
N 160 -300 160 -230 {
lab=#net2}
N 160 -420 160 -390 {
lab=vdd}
N 520 -260 520 -230 {
lab=vout}
N 520 -420 520 -390 {
lab=vdd}
N 520 -170 520 -80 {
lab=vs}
N 160 -170 160 -80 {
lab=vs}
N 560 -200 580 -200 {
lab=#net1}
N 360 -200 520 -200 {
lab=vs}
N 360 -200 360 -80 {
lab=vs}
N 340 -360 480 -360 {
lab=#net2}
N 340 -360 340 -300 {
lab=#net2}
N 160 -300 340 -300 {
lab=#net2}
N 110 -360 160 -360 {
lab=vdd}
N 110 -420 110 -360 {
lab=vdd}
N 110 -420 160 -420 {
lab=vdd}
N 520 -360 570 -360 {
lab=vdd}
N 570 -420 570 -360 {
lab=vdd}
N 520 -420 570 -420 {
lab=vdd}
N 160 -200 360 -200 {
lab=vs}
N 340 -80 360 -80 {
lab=vs}
N 200 -360 340 -360 {
lab=#net2}
N 160 -330 160 -300 {
lab=#net2}
N 930 -260 930 -200 {
lab=vfb}
N 880 -260 930 -260 {
lab=vfb}
N 930 -290 930 -260 {
lab=vfb}
N 930 -370 930 -350 {
lab=vss}
N 520 -260 820 -260 {
lab=vout}
N 520 -330 520 -260 {
lab=vout}
N 380 120 930 120 {
lab=vss}
N 930 -140 930 120 {
lab=vss}
N 340 0 380 0 {
lab=vss}
N 380 0 380 120 {
lab=vss}
N 340 120 380 120 {
lab=vss}
N -40 0 300 0 {
lab=#net3}
N -120 120 340 120 {
lab=vss}
N -120 30 -120 120 {
lab=vss}
N -120 -60 -120 -30 {
lab=#net3}
N -40 -60 -40 0 {
lab=#net3}
N -80 0 -40 0 {
lab=#net3}
N -120 -60 -40 -60 {
lab=#net3}
N -120 -260 -120 -60 {
lab=#net3}
N -170 -0 -120 0 {
lab=vss}
N -170 -0 -170 120 {
lab=vss}
N -170 120 -120 120 {
lab=vss}
N -120 -420 -120 -320 {
lab=vdd}
N -120 -420 110 -420 {
lab=vdd}
N 580 -50 580 -30 {
lab=vfb}
C {devices/lab_pin.sym} 340 -80 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 720 -260 1 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 320 -420 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 30 -90 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} 340 120 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/code_shown.sym} -650 -410 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt

Vref vn 0 0.9
Vdd vdd 0 1.8
Vss vss 0 0

.control
ac dec 10 1 1G
plot vdb(vout)
plot phase(vout)
op 
print vout vs vfb
.endc

"}
C {sg13g2_pr/sg13_hv_nmos.sym} 540 -200 0 1 {name=M1
l=0.45u
w=100.4u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 140 -200 0 0 {name=M2
l=0.45u
w=100.4u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 500 -360 0 0 {name=M3
l=0.4u
w=10.48u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 180 -360 0 1 {name=M4
l=0.4u
w=10.48u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {res.sym} 850 -260 1 0 {name=R1
value=10000000000
footprint=1206
device=resistor
m=1}
C {capa.sym} 930 -170 0 0 {name=C2
m=1
value=10
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 930 -260 2 0 {name=p5 sig_type=std_logic lab=vfb}
C {res.sym} 930 -320 2 0 {name=R2
value=90000000000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 930 -370 1 0 {name=p10 sig_type=std_logic lab=vss}
C {sg13g2_pr/sg13_hv_nmos.sym} 320 0 0 0 {name=M5
l=0.45u
w=34.32u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} -100 0 0 1 {name=M6
l=0.45u
w=34.32u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {isource.sym} -120 -290 0 0 {name=I2 value=40e-6}
C {vsource.sym} 580 -80 0 0 {name=V1 value="dc 0 ac 1" savecurrent=false}
C {devices/lab_pin.sym} 580 -30 2 0 {name=p2 sig_type=std_logic lab=vfb}
