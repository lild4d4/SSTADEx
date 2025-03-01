v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 250 40 410 40 {
lab=vs}
N 230 40 230 90 {
lab=vs}
N -80 -80 -80 30 {
lab=vn}
N -80 -80 10 -80 {
lab=vn}
N 470 -80 470 10 {
lab=vp1}
N 230 200 230 240 {
lab=vss}
N 50 40 230 40 {
lab=vs}
N 50 -500 410 -500 {
lab=vdd}
N 50 -500 50 -470 {
lab=vdd}
N 410 -360 410 -340 {
lab=vcp_cm}
N 410 -500 410 -470 {
lab=vdd}
N 410 -120 410 -110 {
lab=vcp}
N 410 -50 410 40 {
lab=vs}
N 50 -50 50 40 {
lab=vs}
N 450 -80 470 -80 {
lab=vp1}
N 250 -80 410 -80 {
lab=vs}
N 250 -80 250 40 {
lab=vs}
N 230 -440 370 -440 {
lab=vout_2}
N 230 -440 230 -240 {
lab=vout_2}
N 0 -440 50 -440 {
lab=vdd}
N 0 -500 0 -440 {
lab=vdd}
N 0 -500 50 -500 {
lab=vdd}
N 410 -440 460 -440 {
lab=vdd}
N 460 -500 460 -440 {
lab=vdd}
N 410 -500 460 -500 {
lab=vdd}
N 50 -80 250 -80 {
lab=vs}
N 230 40 250 40 {
lab=vs}
N 90 -440 230 -440 {
lab=vout_2}
N 410 -240 790 -240 {
lab=vout}
N 50 -120 50 -110 {
lab=vcp_2}
N 90 -170 370 -170 {
lab=vbias_dp}
N 410 -240 410 -200 {
lab=vout}
N 50 -240 50 -200 {
lab=vout_2}
N 10 -170 50 -170 {
lab=vcp_2}
N 10 -170 10 -120 {
lab=vcp_2}
N 10 -120 50 -120 {
lab=vcp_2}
N 50 -140 50 -120 {
lab=vcp_2}
N 410 -170 450 -170 {
lab=vcp}
N 450 -170 450 -120 {
lab=vcp}
N 410 -120 450 -120 {
lab=vcp}
N 410 -140 410 -120 {
lab=vcp}
N 900 -240 900 -180 {
lab=vfb}
N 900 -120 900 200 {
lab=vss}
N 850 -240 900 -240 {
lab=vfb}
N 900 -270 900 -240 {
lab=vfb}
N 900 -350 900 -330 {
lab=vss}
N 230 200 900 200 {
lab=vss}
N 230 150 230 200 {
lab=vss}
N 470 70 470 100 {
lab=vfb}
N 90 -310 370 -310 {
lab=vbias_cm}
N 410 -280 410 -240 {
lab=vout}
N 50 -240 230 -240 {
lab=vout_2}
N 50 -280 50 -240 {
lab=vout_2}
N 50 -360 50 -340 {
lab=vcp_cm_2}
N 20 -310 50 -310 {
lab=vcp_cm_2}
N 20 -360 20 -310 {
lab=vcp_cm_2}
N 20 -360 50 -360 {
lab=vcp_cm_2}
N 50 -410 50 -360 {
lab=vcp_cm_2}
N 410 -310 440 -310 {
lab=vcp_cm}
N 440 -360 440 -310 {
lab=vcp_cm}
N 410 -360 440 -360 {
lab=vcp_cm}
N 410 -410 410 -360 {
lab=vcp_cm}
C {devices/isource.sym} 230 120 0 0 {name=I2 value=20e-6}
C {devices/lab_pin.sym} 230 40 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 610 -240 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 210 -500 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} -80 30 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} 470 10 2 0 {name=p8 sig_type=std_logic lab=vp1}
C {devices/lab_pin.sym} 230 240 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/code_shown.sym} -750 -410 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt

Vref vn 0 1.35
Vdd vdd 0 3.3
Vbias_dp vbias_dp 0 2.2
Vbias_cm vbias_cm vdd -0.881
Vss vss 0 0

.control
ac dec 10 1 1G
plot vdb(vout)
plot phase(vout)*180/PI
meas ac gain find vdb(vout) at=1
let low_gain = gain-3
meas ac fc WHEN vdb(vout)=low_gain
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

.endc

"}
C {sg13g2_pr/sg13_hv_nmos.sym} 430 -80 2 0 {name=M1
l=1.6u
w=31.17u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 30 -80 2 1 {name=M2
l=1.6u
w=31.17u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 390 -170 2 1 {name=M3
l=3.2u
w=9.76u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 70 -170 2 0 {name=M4
l=3.2u
w=9.76u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 70 -440 0 1 {name=M5
l=6.4u
w=6.657u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 390 -440 0 0 {name=M6
l=6.4u
w=6.657u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {devices/lab_pin.sym} 220 -170 3 0 {name=p2 sig_type=std_logic lab=vbias_dp}
C {devices/lab_pin.sym} 450 -120 2 0 {name=p3 sig_type=std_logic lab=vcp}
C {res.sym} 820 -240 1 0 {name=R1
value=10000000000
footprint=1206
device=resistor
m=1}
C {capa.sym} 900 -150 0 0 {name=C2
m=1
value=10
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 900 -240 2 0 {name=p5 sig_type=std_logic lab=vfb}
C {res.sym} 900 -300 2 0 {name=R2
value=9507042253
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 900 -350 1 0 {name=p10 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 470 100 2 0 {name=p11 sig_type=std_logic lab=vfb}
C {vsource.sym} 470 40 0 0 {name=V1 value="dc 0 ac 1" savecurrent=false}
C {sg13g2_pr/sg13_hv_pmos.sym} 70 -310 0 1 {name=M7
l=0.8u
w=2636u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 390 -310 0 0 {name=M8
l=0.8u
w=2636u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {devices/lab_pin.sym} 290 -310 3 0 {name=p12 sig_type=std_logic lab=vbias_cm}
C {devices/lab_pin.sym} 410 -390 2 0 {name=p13 sig_type=std_logic lab=vcp_cm}
C {devices/lab_pin.sym} 10 -120 0 0 {name=p14 sig_type=std_logic lab=vcp_2}
C {devices/lab_pin.sym} 50 -390 0 0 {name=p15 sig_type=std_logic lab=vcp_cm_2}
C {devices/lab_pin.sym} 50 -240 0 0 {name=p16 sig_type=std_logic lab=vout_2}
