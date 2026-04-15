v {xschem version=3.4.6 file_version=1.2}
G {}
K {}
V {}
S {}
E {}
N -300 60 -300 80 {lab=vs}
N -200 80 -100 80 {lab=vs}
N -100 60 -100 80 {lab=vs}
N -300 -20 -300 0 {lab=#net1}
N -200 -70 -140 -70 {lab=#net1}
N -300 -20 -200 -20 {lab=#net1}
N -200 -70 -200 -20 {lab=#net1}
N -260 -70 -200 -70 {lab=#net1}
N -100 -20 -100 0 {lab=vout}
N -300 -120 -300 -100 {lab=vdd}
N -200 -120 -100 -120 {lab=vdd}
N -100 -120 -100 -100 {lab=vdd}
N -320 -70 -300 -70 {lab=vdd}
N -100 -70 -80 -70 {lab=vdd}
N -200 30 -100 30 {lab=vs}
N -200 30 -200 80 {lab=vs}
N -300 30 -200 30 {lab=vs}
N -300 80 -200 80 {lab=vs}
N -200 80 -200 100 {lab=vs}
N -200 160 -200 180 {lab=GND}
N -200 -140 -200 -120 {lab=vdd}
N -300 -120 -200 -120 {lab=vdd}
N 400 70 400 90 {lab=vdd}
N 400 150 400 170 {lab=GND}
N -60 30 -40 30 {lab=#net2}
N 480 70 480 90 {lab=vref}
N 480 150 480 170 {lab=GND}
N 170 -20 180 -20 {lab=vout}
N -300 -40 -300 -20 {lab=#net1}
N -100 -40 -100 -20 {lab=vout}
N -80 -120 -80 -70 {lab=vdd}
N -100 -120 -80 -120 {lab=vdd}
N -320 -120 -320 -70 {lab=vdd}
N -320 -120 -300 -120 {lab=vdd}
N -400 30 -340 30 {lab=vref}
N -40 30 -40 60 {lab=#net2}
N -40 260 -40 280 {lab=GND}
N 170 210 170 230 {lab=GND}
N 170 110 170 150 {lab=vfb}
N 170 -20 170 50 {lab=vout}
N -100 -20 170 -20 {lab=vout}
N -40 120 -40 200 {lab=vfb}
C {sg13g2_pr/sg13_lv_nmos.sym} -80 30 0 1 {name=M1
l=0.4u
w=29.1u
ng=3
m=1
model=sg13_lv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_lv_nmos.sym} -320 30 0 0 {name=M2
l=0.4u
w=29.1u
ng=3
m=1
model=sg13_lv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_lv_pmos.sym} -120 -70 0 0 {name=M3
l=0.4u
w=4.9u
ng=1
m=1
model=sg13_lv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_lv_pmos.sym} -280 -70 0 1 {name=M4
l=0.4u
w=4.9u
ng=1
m=1
model=sg13_lv_pmos
spiceprefix=X
}
C {isource.sym} -200 130 0 0 {name=I0 value=20e-6}
C {gnd.sym} -200 180 0 0 {name=l1 lab=GND}
C {vsource.sym} 400 120 0 0 {name=V1 value=1.5 savecurrent=false}
C {lab_pin.sym} 400 70 2 0 {name=p1 sig_type=std_logic lab=vdd}
C {gnd.sym} 400 170 0 0 {name=l2 lab=GND}
C {lab_pin.sym} -200 -140 2 0 {name=p2 sig_type=std_logic lab=vdd}
C {vsource.sym} 480 120 0 0 {name=V3 value=0.9 savecurrent=false}
C {lab_pin.sym} 480 70 2 0 {name=p5 sig_type=std_logic lab=vref}
C {gnd.sym} 480 170 0 0 {name=l4 lab=GND}
C {lab_pin.sym} -400 30 0 0 {name=p6 sig_type=std_logic lab=vref}
C {lab_pin.sym} -40 150 2 0 {name=p3 sig_type=std_logic lab=vfb}
C {lab_pin.sym} 180 -20 2 0 {name=p7 sig_type=std_logic lab=vout}
C {code_shown.sym} 300 -330 0 0 {name=NGSPICE only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOSlv.lib mos_tt

.control
ac dec 10 1 1G
plot vdb(vout)
plot phase(vout)
op
print vout vfb vs
.endc

"}
C {vsource.sym} -40 90 0 0 {name=V4 value="dc 0 ac 1" savecurrent=false}
C {lab_pin.sym} -120 80 3 0 {name=p8 sig_type=std_logic lab=vs}
C {capa.sym} -40 230 0 0 {name=C1
m=1
value=10
footprint=1206
device="ceramic capacitor"}
C {res.sym} 170 80 0 0 {name=R1
value=100000000
footprint=1206
device=resistor
m=1}
C {res.sym} 170 180 0 0 {name=R2
value=900000000
footprint=1206
device=resistor
m=1}
C {gnd.sym} 170 230 0 0 {name=l3 lab=GND}
C {lab_pin.sym} 170 130 0 0 {name=p4 sig_type=std_logic lab=vfb}
C {gnd.sym} -40 280 0 0 {name=l5 lab=GND}
