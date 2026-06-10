v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 440 90 440 110 {
lab=vout}
N 440 180 440 200 {
lab=vfb}
N 440 270 440 280 {
lab=vss}
N 400 180 440 180 {
lab=vfb}
N 660 90 660 160 {
lab=vout}
N 560 90 560 160 {
lab=vout}
N 560 220 560 270 {
lab=vss}
N 440 270 560 270 {
lab=vss}
N 560 270 660 270 {
lab=vss}
N 660 220 660 270 {
lab=vss}
N 660 90 740 90 {
lab=vout}
N 440 30 480 30 {
lab=vdd}
N 480 -20 480 30 {
lab=vdd}
N 440 -20 480 -20 {
lab=vdd}
N 440 -20 440 0 {
lab=vdd}
N 440 170 440 180 {
lab=vfb}
N 560 90 660 90 {
lab=vout}
N 440 90 560 90 {
lab=vout}
N 440 260 440 270 {
lab=vss}
N 440 60 440 90 {
lab=vout}
N -10 320 150 320 {
lab=vs}
N -30 320 -30 370 {
lab=vs}
N -340 200 -340 310 {
lab=vfb}
N -340 200 -250 200 {
lab=vfb}
N 210 200 210 320 {
lab=vn}
N -210 320 -30 320 {
lab=vs}
N -210 -120 150 -120 {
lab=vdd}
N -210 -120 -210 -90 {
lab=vdd}
N 150 30 150 80 {
lab=vmid}
N 150 -120 150 -90 {
lab=vdd}
N 150 160 150 170 {
lab=vcp}
N 150 230 150 320 {
lab=vs}
N -210 230 -210 320 {
lab=vs}
N 190 200 210 200 {
lab=vn}
N -10 200 150 200 {
lab=vs}
N -10 200 -10 320 {
lab=vs}
N -30 -60 110 -60 {
lab=#net1}
N -30 -60 -30 0 {
lab=#net1}
N -210 0 -30 0 {
lab=#net1}
N -260 -60 -210 -60 {
lab=vdd}
N -260 -120 -260 -60 {
lab=vdd}
N -260 -120 -210 -120 {
lab=vdd}
N 150 -60 200 -60 {
lab=vdd}
N 200 -120 200 -60 {
lab=vdd}
N 150 -120 200 -120 {
lab=vdd}
N -210 200 -10 200 {
lab=vs}
N -30 320 -10 320 {
lab=vs}
N -170 -60 -30 -60 {
lab=#net1}
N -210 -30 -210 0 {
lab=#net1}
N -210 160 -210 170 {
lab=#net2}
N -170 110 110 110 {
lab=vbias}
N -210 0 -210 80 {
lab=#net1}
N -250 110 -210 110 {
lab=#net2}
N -250 110 -250 160 {
lab=#net2}
N -250 160 -210 160 {
lab=#net2}
N -210 140 -210 160 {
lab=#net2}
N 150 110 190 110 {
lab=vcp}
N 190 110 190 160 {
lab=vcp}
N 150 160 190 160 {
lab=vcp}
N 150 140 150 160 {
lab=vcp}
N -30 430 -30 520 {
lab=vss}
N 150 30 400 30 {
lab=vmid}
N 150 -30 150 30 {
lab=vmid}
N 200 -120 440 -120 {
lab=vdd}
N 440 -120 440 -20 {
lab=vdd}
C {devices/lab_pin.sym} 740 90 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/res.sym} 440 140 0 0 {name=R1
value=100000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 440 230 0 0 {name=R2
value=300000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 400 180 0 0 {name=p5 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 440 280 3 0 {name=p2 sig_type=std_logic lab=vss}
C {capa.sym} 560 190 0 0 {name=Cl
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/res.sym} 660 190 0 0 {name=R3
value=18
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 340 30 3 0 {name=p8 sig_type=std_logic lab=vmid}
C {devices/code_shown.sym} -1200 -140 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt

Vref vn 0 1.35
Vdd vdd 0 3.3 ac 1
Vss vss 0 0
Vbias vbias 0 2.2

.control
ac dec 10 1 100G
plot vdb(vout) 
plot phase(vout)
op
print vout vmid vcp vfb vs
.endc

"}
C {sg13g2_pr/sg13_hv_pmos.sym} 130 -60 0 0 {name=M6
l=3.2u
w=8.02u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {devices/isource.sym} -30 400 0 0 {name=I2 value=60e-6}
C {devices/lab_pin.sym} -30 320 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} -50 -120 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 210 320 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} -30 520 3 0 {name=p9 sig_type=std_logic lab=vss}
C {sg13g2_pr/sg13_hv_nmos.sym} 170 200 2 0 {name=M1
l=1.6u
w=7.58u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} -230 200 2 1 {name=M2
l=1.6u
w=7.58u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 130 110 2 1 {name=M3
l=0.4u
w=5.14u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} -190 110 2 0 {name=M4
l=0.4u
w=5.14u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} -190 -60 0 1 {name=M5
l=3.2u
w=8.02u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 420 30 0 0 {name=M7
l=0.4u
w=2487u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {devices/lab_pin.sym} -40 110 3 0 {name=p10 sig_type=std_logic lab=vbias}
C {devices/lab_pin.sym} 190 160 2 0 {name=p11 sig_type=std_logic lab=vcp}
C {devices/lab_pin.sym} -340 310 2 0 {name=p12 sig_type=std_logic lab=vfb}
