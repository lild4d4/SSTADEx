v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 30 -180 190 -180 {
lab=vs}
N 190 -360 390 -360 {
lab=vout_s1}
N -300 -300 -300 -190 {
lab=vn}
N -300 -300 -210 -300 {
lab=vn}
N 250 -300 250 -210 {
lab=vfb}
N -170 -180 30 -180 {
lab=vs}
N -170 -520 190 -520 {
lab=vdd_1}
N -170 -400 -170 -330 {
lab=#net1}
N -170 -520 -170 -490 {
lab=vdd_1}
N 190 -430 190 -360 {
lab=vout_s1}
N 190 -520 190 -490 {
lab=vdd_1}
N 190 -360 190 -330 {
lab=vout_s1}
N 190 -270 190 -180 {
lab=vs}
N -170 -270 -170 -180 {
lab=vs}
N 230 -300 250 -300 {
lab=vfb}
N 30 -300 190 -300 {
lab=vs}
N 30 -300 30 -180 {
lab=vs}
N 10 -460 150 -460 {
lab=#net1}
N 10 -460 10 -400 {
lab=#net1}
N -170 -400 10 -400 {
lab=#net1}
N -220 -460 -170 -460 {
lab=vdd_1}
N -220 -520 -220 -460 {
lab=vdd_1}
N -220 -520 -170 -520 {
lab=vdd_1}
N 190 -460 240 -460 {
lab=vdd_1}
N 240 -520 240 -460 {
lab=vdd_1}
N 190 -520 240 -520 {
lab=vdd_1}
N 430 -410 430 -390 {
lab=vdd_1}
N 430 -360 460 -360 {
lab=vdd_1}
N 460 -410 460 -360 {
lab=vdd_1}
N 430 -410 460 -410 {
lab=vdd_1}
N 430 -300 610 -300 {
lab=vmid}
N 430 -330 430 -300 {
lab=vmid}
N 650 -240 650 -220 {
lab=vout}
N 650 -150 650 -130 {
lab=vfb}
N 650 -60 650 20 {
lab=vss}
N 610 -150 650 -150 {
lab=vfb}
N 870 -240 870 -170 {
lab=vout}
N 770 -240 770 -170 {
lab=vout}
N 770 -110 770 -60 {
lab=vss}
N 650 -60 770 -60 {
lab=vss}
N 770 -60 870 -60 {
lab=vss}
N 870 -110 870 -60 {
lab=vss}
N 870 -240 950 -240 {
lab=vout}
N 650 -300 690 -300 {
lab=vdd}
N 690 -350 690 -300 {
lab=vdd}
N 650 -350 690 -350 {
lab=vdd}
N 650 -350 650 -330 {
lab=vdd}
N 650 -520 650 -350 {
lab=vdd}
N -170 -300 30 -300 {
lab=vs}
N -130 -460 10 -460 {
lab=#net1}
N -170 -430 -170 -400 {
lab=#net1}
N 430 -520 430 -410 {
lab=vdd_1}
N 650 -160 650 -150 {
lab=vfb}
N 770 -240 870 -240 {
lab=vout}
N 650 -240 770 -240 {
lab=vout}
N 650 -70 650 -60 {
lab=vss}
N 240 -520 430 -520 {
lab=vdd_1}
N 650 -270 650 -240 {
lab=vout}
N 430 -300 430 -130 {
lab=vmid}
N 30 -180 30 -130 {
lab=vs}
N 30 -70 30 20 {
lab=vss}
N 30 -100 70 -100 {
lab=vss}
N 70 -100 70 20 {
lab=vss}
N 30 20 70 20 {
lab=vss}
N -430 20 30 20 {
lab=vss}
N -430 -70 -430 20 {
lab=vss}
N -430 -160 -430 -130 {
lab=#net2}
N -350 -160 -350 -100 {
lab=#net2}
N -390 -100 -350 -100 {
lab=#net2}
N -430 -160 -350 -160 {
lab=#net2}
N -430 -360 -430 -160 {
lab=#net2}
N -480 -100 -430 -100 {
lab=vss}
N -480 -100 -480 20 {
lab=vss}
N -480 20 -430 20 {
lab=vss}
N 430 -70 430 20 {
lab=vss}
N 430 -100 470 -100 {
lab=vss}
N 470 -100 470 20 {
lab=vss}
N 430 20 470 20 {
lab=vss}
N 70 20 430 20 {
lab=vss}
N -40 -100 -10 -100 {
lab=#net2}
N -40 -150 -40 -100 {
lab=#net2}
N -350 -100 -40 -100 {
lab=#net2}
N -40 -150 330 -150 {
lab=#net2}
N 330 -150 330 -100 {
lab=#net2}
N 330 -100 390 -100 {
lab=#net2}
N 470 20 650 20 {
lab=vss}
N -430 -520 -430 -420 {
lab=vdd_1}
N -430 -520 -220 -520 {
lab=vdd_1}
C {devices/lab_pin.sym} 10 -180 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 950 -240 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 650 -520 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} -300 -190 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/res.sym} 650 -190 0 0 {name=R1
value=22500
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 650 -100 0 0 {name=R2
value=67500
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 610 -150 0 0 {name=p5 sig_type=std_logic lab=vfb}
C {capa.sym} 770 -140 0 0 {name=Cl
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 250 -210 3 0 {name=p3 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 550 -300 3 0 {name=p8 sig_type=std_logic lab=vmid}
C {devices/lab_pin.sym} 280 -360 1 0 {name=p10 sig_type=std_logic lab=vout_s1}
C {devices/code_shown.sym} -990 -470 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/ihp-sg13g2/libs.tech/ngspice/models/cornerMOShv.lib mos_tt

Vref vn 0 1.35
Vdd vdd 0 3.3 ac 1
Vdd_1 vdd_1 0 3.3
Vss vss 0 0

.control
ac dec 10 1 100G
plot vdb(vout) 
plot phase(vout)
op
print vout vmid vout_s1 vs

let gm_m1 = @n.xm1.nsg13_hv_nmos[gm]
let gm_m2 = @n.xm2.nsg13_hv_nmos[gm]
let gm_m3 = @n.xm3.nsg13_hv_pmos[gm]
let gm_m4 = @n.xm4.nsg13_hv_pmos[gm]
let gm_m5 = @n.xm5.nsg13_hv_pmos[gm]
let gm_m6 = @n.xm6.nsg13_hv_pmos[gm]

let gds_m1 = @n.xm1.nsg13_hv_nmos[gds]
let gds_m2 = @n.xm2.nsg13_hv_nmos[gds]
let gds_m3 = @n.xm3.nsg13_hv_pmos[gds]
let gds_m4 = @n.xm4.nsg13_hv_pmos[gds]
let gds_m5 = @n.xm5.nsg13_hv_pmos[gds]
let gds_m6 = @n.xm6.nsg13_hv_pmos[gds]

print gm_m1
print gm_m2
print gm_m3
print gm_m4
print gm_m5
print gm_m6

print 1/gds_m1
print 1/gds_m2
print 1/gds_m3
print 1/gds_m4
print 1/gds_m5
print 1/gds_m6

.endc

"}
C {sg13g2_pr/sg13_hv_nmos.sym} 210 -300 2 0 {name=M1
l=0.8u
w=5.33u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} -190 -300 2 1 {name=M2
l=0.8u
w=5.33u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 170 -460 0 0 {name=M3
l=6.4u
w=9.02u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} -150 -460 0 1 {name=M4
l=6.4u
w=9.02u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 410 -360 0 0 {name=M5
l=3.2u
w=8.8u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_pmos.sym} 630 -300 0 0 {name=M9
l=0.4u
w=2488u
ng=1
m=1
model=sg13_hv_pmos
spiceprefix=X
}
C {devices/lab_pin.sym} 20 -520 1 0 {name=p11 sig_type=std_logic lab=vdd_1}
C {devices/res.sym} 870 -140 0 0 {name=R3
value=18
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 30 20 3 0 {name=p9 sig_type=std_logic lab=vss}
C {sg13g2_pr/sg13_hv_nmos.sym} 10 -100 0 0 {name=M6
l=3.2u
w=5.4u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} -410 -100 0 1 {name=M7
l=3.2u
w=5.4u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {sg13g2_pr/sg13_hv_nmos.sym} 410 -100 0 0 {name=M8
l=3.2u
w=5.4u
ng=1
m=1
model=sg13_hv_nmos
spiceprefix=X
}
C {isource.sym} -430 -390 0 0 {name=I2 value=40e-6}
