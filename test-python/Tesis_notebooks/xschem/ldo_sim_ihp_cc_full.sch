v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 450 -110 450 -90 {
lab=vout}
N 450 -20 450 0 {
lab=vfb}
N 450 70 450 80 {
lab=vss}
N 410 -20 450 -20 {
lab=vfb}
N 670 -110 670 -40 {
lab=vout}
N 570 -110 570 -40 {
lab=vout}
N 570 20 570 70 {
lab=vss}
N 450 70 570 70 {
lab=vss}
N 570 70 670 70 {
lab=vss}
N 670 20 670 70 {
lab=vss}
N 670 -110 750 -110 {
lab=vout}
N 450 -170 490 -170 {
lab=vdd}
N 490 -220 490 -170 {
lab=vdd}
N 450 -220 490 -220 {
lab=vdd}
N 450 -220 450 -200 {
lab=vdd}
N 450 -30 450 -20 {
lab=vfb}
N 570 -110 670 -110 {
lab=vout}
N 450 -110 570 -110 {
lab=vout}
N 450 60 450 70 {
lab=vss}
N 450 -140 450 -110 {
lab=vout}
N 160 -170 410 -170 {
lab=vmid}
N 0 110 160 110 {
lab=vs}
N -20 110 -20 160 {
lab=vs}
N -330 -10 -330 100 {
lab=vfb}
N -330 -10 -240 -10 {
lab=vfb}
N -200 110 -20 110 {
lab=vs}
N -200 -430 160 -430 {
lab=vdd_1}
N -200 -430 -200 -400 {
lab=vdd_1}
N 160 -290 160 -270 {
lab=vcp_cm}
N 160 -430 160 -400 {
lab=vdd_1}
N 160 -50 160 -40 {
lab=vcp}
N 160 20 160 110 {
lab=vs}
N -200 20 -200 110 {
lab=vs}
N 200 -10 220 -10 {
lab=vn}
N 0 -10 160 -10 {
lab=vs}
N 0 -10 0 110 {
lab=vs}
N -20 -370 120 -370 {
lab=#net1}
N -20 -370 -20 -170 {
lab=#net1}
N -250 -370 -200 -370 {
lab=vdd_1}
N -250 -430 -250 -370 {
lab=vdd_1}
N -250 -430 -200 -430 {
lab=vdd_1}
N 160 -370 210 -370 {
lab=vdd_1}
N 210 -430 210 -370 {
lab=vdd_1}
N 160 -430 210 -430 {
lab=vdd_1}
N -200 -10 0 -10 {
lab=vs}
N -20 110 0 110 {
lab=vs}
N -160 -370 -20 -370 {
lab=#net1}
N -200 -50 -200 -40 {
lab=#net2}
N -160 -100 120 -100 {
lab=vbias_dp}
N 160 -170 160 -130 {
lab=vmid}
N -200 -170 -200 -130 {
lab=#net1}
N -240 -100 -200 -100 {
lab=#net2}
N -240 -100 -240 -50 {
lab=#net2}
N -240 -50 -200 -50 {
lab=#net2}
N -200 -70 -200 -50 {
lab=#net2}
N 160 -100 200 -100 {
lab=vcp}
N 200 -100 200 -50 {
lab=vcp}
N 160 -50 200 -50 {
lab=vcp}
N 160 -70 160 -50 {
lab=vcp}
N -20 220 -20 310 {
lab=vss}
N 220 -10 220 20 {
lab=vn}
N -160 -240 120 -240 {
lab=vbias_cm}
N 160 -210 160 -170 {
lab=vmid}
N -200 -170 -20 -170 {
lab=#net1}
N -200 -210 -200 -170 {
lab=#net1}
N -200 -290 -200 -270 {
lab=#net3}
N -230 -240 -200 -240 {
lab=#net3}
N -230 -290 -230 -240 {
lab=#net3}
N -230 -290 -200 -290 {
lab=#net3}
N -200 -340 -200 -290 {
lab=#net3}
N 160 -240 190 -240 {
lab=vcp_cm}
N 190 -290 190 -240 {
lab=vcp_cm}
N 160 -290 190 -290 {
lab=vcp_cm}
N 160 -340 160 -290 {
lab=vcp_cm}
N 450 -430 450 -220 {
lab=vdd}
C {devices/lab_pin.sym} 750 -110 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/res.sym} 450 -60 0 0 {name=R1
value=5000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 450 30 0 0 {name=R2
value=15000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 410 -20 0 0 {name=p5 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 450 80 3 0 {name=p2 sig_type=std_logic lab=vss}
C {capa.sym} 570 -10 0 0 {name=Cl
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 350 -170 3 0 {name=p8 sig_type=std_logic lab=vmid}
C {devices/code_shown.sym} -1020 -410 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt

Vref vn 0 1.35
Vdd_1 vdd_1 0 3.3 
Vdd vdd 0 3.3 ac 1
Vss vss 0 0
Vbias_dp vbias_dp 0 2.2
Vbias_cm vbias_cm vdd_1 -0.881

.control
ac dec 10 1 100G
plot vdb(vout) 
plot phase(vout)
op
print vout vmid vcp vfb vs

let gm_m1 = @n.xm1.nsg13_hv_nmos[gm]
let gm_m3 = @n.xm3.nsg13_hv_nmos[gm]
let gm_m6 = @n.xm6.nsg13_hv_pmos[gm]
let gm_m8 = @n.xm8.nsg13_hv_pmos[gm]
let gm_m9 = @n.xm9.nsg13_hv_pmos[gm]

let gds_m1 = @n.xm1.nsg13_hv_nmos[gds]
let gds_m3 = @n.xm3.nsg13_hv_nmos[gds]
let gds_m6 = @n.xm6.nsg13_hv_pmos[gds]
let gds_m8 = @n.xm8.nsg13_hv_pmos[gds]
let gds_m9 = @n.xm9.nsg13_hv_pmos[gds]

print gm_m1
print gm_m3
print gm_m6
print gm_m8
print gm_m9

print 1/gds_m1
print 1/gds_m3
print 1/gds_m6
print 1/gds_m8
print 1/gds_m9

let ro_dp = gm_m3/(gds_m3*gds_m1)
let ro_cm = gm_m8/(gds_m8*gds_m6)
print ro_dp
print ro_cm

print ro_dp*ro_cm/(ro_dp+ro_cm)
.endc

"}
C {sg13g2_pr/sg13_hv_pmos.sym} -180 -240 0 1 {name=M7
l=0.8u
w=2636u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {devices/isource.sym} -20 190 0 0 {name=I2 value=20e-6}
C {devices/lab_pin.sym} -20 110 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 220 20 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} -20 310 3 0 {name=p9 sig_type=std_logic lab=vss}
C {sg13g2_pr/sg13_hv_nmos.sym} 180 -10 2 0 {name=M1
l=1.6u
w=31.17u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} -220 -10 2 1 {name=M2
l=1.6u
w=31.17u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 140 -100 2 1 {name=M3
l=3.2u
w=9.76u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} -180 -100 2 0 {name=M4
l=3.2u
w=9.76u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} -180 -370 0 1 {name=M5
l=6.4u
w=6.657u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 140 -370 0 0 {name=M6
l=6.4u
w=6.657u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {devices/lab_pin.sym} -30 -100 3 0 {name=p10 sig_type=std_logic lab=vbias_dp}
C {devices/lab_pin.sym} 200 -50 2 0 {name=p11 sig_type=std_logic lab=vcp}
C {devices/lab_pin.sym} -330 100 2 0 {name=p12 sig_type=std_logic lab=vfb}
C {sg13g2_pr/sg13_hv_pmos.sym} 140 -240 0 0 {name=M8
l=0.8u
w=2636u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 430 -170 0 0 {name=M9
l=0.4u
w=2489u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {devices/lab_pin.sym} 40 -240 3 0 {name=p13 sig_type=std_logic lab=vbias_cm}
C {devices/lab_pin.sym} 160 -320 2 0 {name=p14 sig_type=std_logic lab=vcp_cm}
C {devices/lab_pin.sym} 450 -430 1 0 {name=p3 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 0 -430 1 0 {name=p6 sig_type=std_logic lab=vdd_1}
C {devices/res.sym} 670 -10 0 0 {name=R3
value=18
footprint=1206
device=resistor
m=1}
