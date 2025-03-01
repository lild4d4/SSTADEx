v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 210 140 370 140 {
lab=vs}
N 190 140 190 190 {
lab=vs}
N -120 20 -120 130 {
lab=vn}
N -120 20 -30 20 {
lab=vn}
N 430 20 430 110 {
lab=vp1}
N 190 300 190 340 {
lab=vss}
N 10 140 190 140 {
lab=vs}
N 10 -400 370 -400 {
lab=vdd}
N 10 -400 10 -370 {
lab=vdd}
N 370 -260 370 -240 {
lab=vcp_cm}
N 370 -400 370 -370 {
lab=vdd}
N 370 -20 370 -10 {
lab=vcp}
N 370 50 370 140 {
lab=vs}
N 10 50 10 140 {
lab=vs}
N 410 20 430 20 {
lab=vp1}
N 210 20 370 20 {
lab=vs}
N 210 20 210 140 {
lab=vs}
N 190 -340 330 -340 {
lab=vout_2}
N 190 -340 190 -140 {
lab=vout_2}
N -40 -340 10 -340 {
lab=vdd}
N -40 -400 -40 -340 {
lab=vdd}
N -40 -400 10 -400 {
lab=vdd}
N 370 -340 420 -340 {
lab=vdd}
N 420 -400 420 -340 {
lab=vdd}
N 370 -400 420 -400 {
lab=vdd}
N 10 20 210 20 {
lab=vs}
N 190 140 210 140 {
lab=vs}
N 50 -340 190 -340 {
lab=vout_2}
N 370 -140 750 -140 {
lab=vout}
N 10 -20 10 -10 {
lab=vcp_2}
N 50 -70 330 -70 {
lab=vbias_dp}
N 370 -140 370 -100 {
lab=vout}
N 10 -140 10 -100 {
lab=vout_2}
N -30 -70 10 -70 {
lab=vcp_2}
N -30 -70 -30 -20 {
lab=vcp_2}
N -30 -20 10 -20 {
lab=vcp_2}
N 10 -40 10 -20 {
lab=vcp_2}
N 370 -70 410 -70 {
lab=vcp}
N 410 -70 410 -20 {
lab=vcp}
N 370 -20 410 -20 {
lab=vcp}
N 370 -40 370 -20 {
lab=vcp}
N 860 -140 860 -80 {
lab=vfb}
N 860 -20 860 300 {
lab=vss}
N 810 -140 860 -140 {
lab=vfb}
N 860 -170 860 -140 {
lab=vfb}
N 860 -250 860 -230 {
lab=vss}
N 190 250 190 300 {
lab=vss}
N 430 170 430 200 {
lab=vfb}
N 50 -210 330 -210 {
lab=vbias_cm}
N 370 -180 370 -140 {
lab=vout}
N 10 -140 190 -140 {
lab=vout_2}
N 10 -180 10 -140 {
lab=vout_2}
N 10 -260 10 -240 {
lab=vcp_cm_2}
N -20 -210 10 -210 {
lab=vcp_cm_2}
N -20 -260 -20 -210 {
lab=vcp_cm_2}
N -20 -260 10 -260 {
lab=vcp_cm_2}
N 10 -310 10 -260 {
lab=vcp_cm_2}
N 370 -210 400 -210 {
lab=vcp_cm}
N 400 -260 400 -210 {
lab=vcp_cm}
N 370 -260 400 -260 {
lab=vcp_cm}
N 370 -310 370 -260 {
lab=vcp_cm}
N 190 300 860 300 {
lab=vss}
C {devices/isource.sym} 190 220 0 0 {name=I2 value=20e-6}
C {devices/lab_pin.sym} 190 140 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 570 -140 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 170 -400 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} -120 130 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} 430 110 2 0 {name=p8 sig_type=std_logic lab=vp1}
C {devices/lab_pin.sym} 190 340 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/code_shown.sym} -780 -290 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt

Vref vn 0 1.35
Vdd vdd 0 3.3
Vbias_dp vbias_dp 0 2.2
Vbias_cm vbias_cm vdd -0.88
Vss vss 0 0

.control
ac dec 10 1 1G



plot vdb(vout)
plot ph(vout)*180/PI
meas ac gain find vdb(vout) at=1
let low_gain = gain-3
meas ac fc WHEN vdb(vout)=low_gain

let phas = ph(vout)*180/PI

meas ac fu WHEN vdb(vout)=0
meas ac pmar find phas at=fu

op 

let gm_m1 = @n.xm1.nsg13_hv_nmos[gm]
let gm_m2 = @n.xm2.nsg13_hv_nmos[gm]
let gm_m3 = @n.xm3.nsg13_hv_nmos[gm]
let gm_m4 = @n.xm4.nsg13_hv_nmos[gm]
let gm_m5 = @n.xm5.nsg13_hv_pmos[gm]
let gm_m6 = @n.xm6.nsg13_hv_pmos[gm]
let gm_m7 = @n.xm7.nsg13_hv_pmos[gm]
let gm_m8 = @n.xm8.nsg13_hv_pmos[gm]

let gds_m1 = @n.xm1.nsg13_hv_nmos[gds]
let gds_m2 = @n.xm2.nsg13_hv_nmos[gds]
let gds_m3 = @n.xm3.nsg13_hv_nmos[gds]
let gds_m4 = @n.xm4.nsg13_hv_nmos[gds]
let gds_m5 = @n.xm5.nsg13_hv_pmos[gds]
let gds_m6 = @n.xm6.nsg13_hv_pmos[gds]
let gds_m7 = @n.xm7.nsg13_hv_pmos[gds]
let gds_m8 = @n.xm8.nsg13_hv_pmos[gds]

let cds_m1 = abs(@n.xm1.nsg13_hv_nmos[cds])+abs(@n.xm1.nsg13_hv_nmos[cdb])
let cds_m4 = abs(@n.xm4.nsg13_hv_nmos[cds])+abs(@n.xm4.nsg13_hv_nmos[cdb])
let cds_m6 = abs(@n.xm6.nsg13_hv_pmos[cds])+abs(@n.xm6.nsg13_hv_pmos[cdb])
let cds_m8 = abs(@n.xm8.nsg13_hv_pmos[cds])+abs(@n.xm8.nsg13_hv_pmos[cdb])

let cgd_m1 = @n.xm1.nsg13_hv_nmos[cgd]
let cgd_m4 = @n.xm4.nsg13_hv_nmos[cgd]
let cgd_m6 = @n.xm6.nsg13_hv_pmos[cgd]
let cgd_m8 = @n.xm8.nsg13_hv_pmos[cgd]

let cgs_m1 = @n.xm1.nsg13_hv_nmos[cgg]
let cgs_m4 = @n.xm4.nsg13_hv_nmos[cgg]
let cgs_m6 = @n.xm6.nsg13_hv_pmos[cgg]
let cgs_m8 = @n.xm8.nsg13_hv_pmos[cgg]

let gain_cc1 = (gm_m3/gds_m3)
let gain_cc2 = (gm_m4/gds_m4)

let gain_cc3 = gm_m8/gds_m8
let gain_cc4 = gm_m7/gds_m7

let rout_diff1 = gain_cc1/gds_m1
let rout_diff2 = gain_cc2/gds_m2

let rout_cm1 = gain_cc3/gds_m6
let rout_cm2 = gain_cc4/gds_m5

print vout vs vcp vcp_2 vcp_cm vcp_cm_2 vout_2

print gm_m1
print gm_m2
print gm_m3
print gm_m4
print gm_m5
print gm_m6
print gm_m7
print gm_m8

print 1/gds_m1
print 1/gds_m2
print 1/gds_m3
print 1/gds_m4
print 1/gds_m5
print 1/gds_m6
print 1/gds_m7
print 1/gds_m8

print rout_diff1
print rout_diff2

print rout_cm1
print rout_cm2

print cds_m1
print cds_m4
print cds_m6
print cds_m8

print cgd_m1
print cgd_m4
print cgd_m6
print cgd_m8

print cgs_m1
print cgs_m4
print cgs_m6
print cgs_m8

.endc

"}
C {sg13g2_pr/sg13_hv_nmos.sym} 390 20 2 0 {name=M1
l=1.6u
w=31.17u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} -10 20 2 1 {name=M2
l=1.6u
w=31.17u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 30 -70 2 0 {name=M3
l=3.2u
w=9.76u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 350 -70 2 1 {name=M4
l=3.2u
w=9.76u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 30 -340 0 1 {name=M5
l=6.4u
w=6.657u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 350 -340 0 0 {name=M6
l=6.4u
w=6.657u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {devices/lab_pin.sym} 180 -70 3 0 {name=p2 sig_type=std_logic lab=vbias_dp}
C {devices/lab_pin.sym} 410 -20 2 0 {name=p3 sig_type=std_logic lab=vcp}
C {res.sym} 780 -140 1 0 {name=R1
value=10000000000
footprint=1206
device=resistor
m=1}
C {capa.sym} 860 -50 0 0 {name=C2
m=1
value=10
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 860 -140 2 0 {name=p5 sig_type=std_logic lab=vfb}
C {res.sym} 860 -200 2 0 {name=R2
value=38571428571
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 860 -250 1 0 {name=p10 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 430 200 2 0 {name=p11 sig_type=std_logic lab=vfb}
C {vsource.sym} 430 140 0 0 {name=V1 value="dc 0 ac 1" savecurrent=false}
C {sg13g2_pr/sg13_hv_pmos.sym} 30 -210 0 1 {name=M7
l=0.8u
w=2636u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 350 -210 0 0 {name=M8
l=0.8u
w=2636u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {devices/lab_pin.sym} 250 -210 3 0 {name=p12 sig_type=std_logic lab=vbias_cm}
C {devices/lab_pin.sym} 370 -290 2 0 {name=p13 sig_type=std_logic lab=vcp_cm}
C {devices/lab_pin.sym} -30 -20 0 0 {name=p14 sig_type=std_logic lab=vcp_2}
C {devices/lab_pin.sym} 10 -280 0 0 {name=p15 sig_type=std_logic lab=vcp_cm_2}
C {devices/lab_pin.sym} 10 -140 0 0 {name=p16 sig_type=std_logic lab=vout_2}
